// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

//! HTTP/3 (QUIC) transport for the v2 API server.
//!
//! When compiled with the `api-v2-http3` feature and HTTP/3 is enabled at runtime,
//! a UDP-based QUIC listener is bound alongside the existing TCP listener.
//! TLS is mandatory for QUIC (built into the protocol); the same PEM cert/key
//! files used by the TCP TLS path are loaded separately via quinn's rustls 0.23
//! integration (the workspace uses rustls 0.21 for TCP).
//!
//! Instead of using h3-axum (which requires axum 0.8), this module bridges h3
//! requests to the axum 0.7 Router by converting between h3 and http types
//! and invoking the Router as a tower::Service.

use anyhow::{Context, Result};
use aptos_logger::{info, warn};
use axum::Router;
use bytes::{Buf, Bytes};
use http_body_util::BodyExt;
use std::{
    io::BufReader,
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::watch;
use tower::Service;

/// Build a Quinn QUIC server endpoint with TLS from PEM cert/key files.
///
/// Uses quinn's bundled rustls 0.23 (separate from the workspace rustls 0.21),
/// so the cert/key types come from `quinn::rustls::pki_types`.
pub fn build_quic_endpoint(
    addr: SocketAddr,
    cert_path: &str,
    key_path: &str,
) -> Result<quinn::Endpoint> {
    let cert_data = std::fs::read(cert_path)
        .with_context(|| format!("Failed to read TLS cert: {}", cert_path))?;
    let key_data =
        std::fs::read(key_path).with_context(|| format!("Failed to read TLS key: {}", key_path))?;

    let certs: Vec<quinn::rustls::pki_types::CertificateDer<'static>> =
        rustls_pemfile_v2::certs(&mut BufReader::new(cert_data.as_slice()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .with_context(|| format!("Failed to parse PEM certs from: {}", cert_path))?;

    if certs.is_empty() {
        anyhow::bail!("No certificates found in {}", cert_path);
    }

    let key = rustls_pemfile_v2::private_key(&mut BufReader::new(key_data.as_slice()))
        .with_context(|| format!("Failed to parse private key from: {}", key_path))?
        .ok_or_else(|| anyhow::anyhow!("No private key found in {}", key_path))?;

    let mut tls_config = quinn::rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("Failed to build QUIC TLS ServerConfig")?;

    tls_config.alpn_protocols = vec![b"h3".to_vec()];
    tls_config.max_early_data_size = u32::MAX;

    let quic_server_config = quinn::ServerConfig::with_crypto(Arc::new(
        quinn::crypto::rustls::QuicServerConfig::try_from(tls_config)
            .context("Failed to build QuicServerConfig")?,
    ));

    let endpoint = quinn::Endpoint::server(quic_server_config, addr)
        .with_context(|| format!("Failed to bind QUIC endpoint on {}", addr))?;

    info!("QUIC/HTTP3 endpoint bound on {}", addr);
    Ok(endpoint)
}

/// Serve an Axum router over HTTP/3 using the given Quinn endpoint.
///
/// For each incoming QUIC connection, spawns a task that builds an h3
/// connection and dispatches requests through the Axum router's tower::Service.
///
/// Supports graceful shutdown: when `shutdown_rx` receives `true`, the endpoint
/// stops accepting new connections and waits up to `drain_timeout_ms` for
/// in-flight connections to complete.
pub async fn serve_h3(
    endpoint: quinn::Endpoint,
    app: Router,
    shutdown_rx: watch::Receiver<bool>,
    drain_timeout_ms: u64,
) {
    let local_addr = endpoint
        .local_addr()
        .expect("Failed to get QUIC local addr");
    info!("HTTP/3 server listening on {} (UDP)", local_addr);

    let active_connections = Arc::new(AtomicUsize::new(0));
    let mut accept_shutdown_rx = shutdown_rx.clone();

    loop {
        tokio::select! {
            incoming = endpoint.accept() => {
                let Some(incoming) = incoming else {
                    info!("[h3] QUIC endpoint closed");
                    break;
                };

                let app = app.clone();
                let active = active_connections.clone();
                active.fetch_add(1, Ordering::Relaxed);

                tokio::spawn(async move {
                    if let Err(e) = handle_h3_connection(incoming, app).await {
                        warn!("[h3] Connection error: {}", e);
                    }
                    active.fetch_sub(1, Ordering::Relaxed);
                });
            }
            _ = accept_shutdown_rx.wait_for(|&v| v) => {
                info!("[h3] Shutdown signal received, stopping QUIC accept loop");
                break;
            }
        }
    }

    endpoint.close(quinn::VarInt::from_u32(0), b"server shutting down");

    let remaining = active_connections.load(Ordering::Relaxed);
    if remaining > 0 {
        info!(
            "[h3] Draining {} active connections (timeout: {}ms)...",
            remaining, drain_timeout_ms
        );

        if drain_timeout_ms > 0 {
            let deadline = tokio::time::Instant::now() + Duration::from_millis(drain_timeout_ms);
            loop {
                let count = active_connections.load(Ordering::Relaxed);
                if count == 0 {
                    info!("[h3] All connections drained");
                    break;
                }
                if tokio::time::Instant::now() >= deadline {
                    warn!(
                        "[h3] Drain timeout ({}ms) exceeded, {} connections remaining",
                        drain_timeout_ms, count
                    );
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    } else {
        info!("[h3] Server shut down (no active connections)");
    }
}

async fn handle_h3_connection(
    incoming: quinn::Incoming,
    app: Router,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = incoming.await?;
    let remote_addr = conn.remote_address();

    let mut h3_conn = h3::server::builder()
        .build(h3_quinn::Connection::new(conn))
        .await?;

    loop {
        match h3_conn.accept().await {
            Ok(Some(resolver)) => {
                let app = app.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_h3_request(resolver, app).await {
                        warn!("[h3] Request error from {}: {}", remote_addr, e);
                    }
                });
            },
            Ok(None) => break,
            Err(conn_err) => {
                if !conn_err.is_h3_no_error() {
                    warn!("[h3] Connection error from {}: {}", remote_addr, conn_err);
                }
                break;
            },
        }
    }

    Ok(())
}

/// Bridge an h3 request to the Axum router and send the response back over h3.
async fn handle_h3_request(
    resolver: h3::server::RequestResolver<h3_quinn::Connection, Bytes>,
    mut app: Router,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (request, mut stream) = resolver.resolve_request().await?;
    let (parts, _) = request.into_parts();

    let mut body_bytes = Vec::new();
    while let Some(chunk) = stream.recv_data().await? {
        body_bytes.extend_from_slice(chunk.chunk());
    }

    let axum_request =
        axum::http::Request::from_parts(parts, axum::body::Body::from(Bytes::from(body_bytes)));

    let response: axum::http::Response<axum::body::Body> = app.call(axum_request).await?;
    let (resp_parts, resp_body) = response.into_parts();

    let h3_response = axum::http::Response::from_parts(resp_parts, ());
    stream.send_response(h3_response).await?;

    let collected = resp_body.collect().await?;
    let resp_bytes = collected.to_bytes();
    if !resp_bytes.is_empty() {
        stream.send_data(resp_bytes).await?;
    }

    stream.finish().await?;
    Ok(())
}
