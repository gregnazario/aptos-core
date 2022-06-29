// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{
        check_network, get_timestamp, get_transaction, handle_request, with_context,
        with_empty_request, EmptyRequest,
    },
    error::{ApiError, ApiResult},
    types::{
        Allow, NetworkListResponse, NetworkOptionsResponse, NetworkRequest,
        NetworkStatusResponse, OperationStatusType, OperationType, Peer, Version,
    },
    RosettaContext, MIDDLEWARE_VERSION, NODE_VERSION, ROSETTA_VERSION,
};
use aptos_logger::{debug, trace};
use warp::Filter;

pub fn routes(
    server_context: RosettaContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("network" / "list")
                .and(with_empty_request())
                .and(with_context(server_context.clone()))
                .and_then(handle_request(network_list)),
        )
        .or(warp::path!("network" / "options")
            .and(warp::body::json())
            .and(with_context(server_context.clone()))
            .and_then(handle_request(network_options)))
        .or(warp::path!("network" / "status")
            .and(warp::body::json())
            .and(with_context(server_context))
            .and_then(handle_request(network_status)))
}

/// List [`NetworkIdentifier`]s supported by this proxy aka [`ChainId`]s
///
/// This should be able to run without a running full node connection
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networklist)
async fn network_list(
    _empty: EmptyRequest,
    server_context: RosettaContext,
) -> Result<NetworkListResponse, ApiError> {
    debug!("/network/list");
    trace!(
        server_context = ?server_context,
        "network_list",
    );

    let response = NetworkListResponse {
        network_identifiers: vec![server_context.chain_id.into()],
    };

    Ok(response)
}

/// Get Network options
///
/// This lists out all errors, operations, and statuses, along with versioning information.
/// This should be able to run without a running full node connection
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networkoptions)
async fn network_options(
    request: NetworkRequest,
    server_context: RosettaContext,
) -> Result<NetworkOptionsResponse, ApiError> {
    debug!("/network/options");
    trace!(
        request = ?request,
        server_context = ?server_context,
        "network_options",
    );

    check_network(request.network_identifier, &server_context)?;

    let version = Version {
        rosetta_version: ROSETTA_VERSION.to_string(),
        // TODO: Get from node via REST API
        node_version: NODE_VERSION.to_string(),
        // TODO: Get from the binary directly
        middleware_version: MIDDLEWARE_VERSION.to_string(),
    };

    let operation_statuses = OperationStatusType::all()
        .into_iter()
        .map(|status| status.into())
        .collect();
    let operation_types = OperationType::all()
        .into_iter()
        .map(|op| op.to_string())
        .collect();
    let errors = ApiError::all()
        .into_iter()
        .map(|err| err.into_error())
        .collect();

    let allow = Allow {
        operation_statuses,
        operation_types,
        errors,
        historical_balance_lookup: false,
        timestamp_start_index: None,
        call_methods: vec![],
        balance_exemptions: vec![],
        mempool_coins: false,
        block_hash_case: None,
        transaction_hash_case: None,
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}

/// Get network status including the latest state
///
/// This should respond with the latest ledger version, timestamp, and genesis information
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networkoptions)
async fn network_status(
    request: NetworkRequest,
    server_context: RosettaContext,
) -> ApiResult<NetworkStatusResponse> {
    debug!("/network/status");
    trace!(
        request = ?request,
        server_context = ?server_context,
        "network_status",
    );

    check_network(request.network_identifier, &server_context)?;

    let rest_client = server_context.rest_client()?;
    // TODO: Cache the genesis transaction?
    let (genesis_txn, response) = get_transaction(rest_client, 0).await?;
    let state = response.state();

    // If there is an oldest block identifier, retrieve the transaction
    let oldest_txn = if let Some(oldest_version) = state.oldest_ledger_version {
        let (oldest_txn, _) = get_transaction(rest_client, oldest_version).await?;
        Some(oldest_txn)
    } else {
        None
    };

    let (latest_txn, response) = get_transaction(rest_client, state.version).await?;
    let current_block_timestamp = get_timestamp(&response);

    // TODO: add peers
    let peers: Vec<Peer> = vec![];

    let response = NetworkStatusResponse {
        current_block_identifier: latest_txn,
        current_block_timestamp,
        genesis_block_identifier: genesis_txn,
        oldest_block_identifier: oldest_txn,
        sync_status: None,
        peers,
    };

    Ok(response)
}
