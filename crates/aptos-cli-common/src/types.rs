// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_telemetry::collect_build_information;
use hex::FromHexError;
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use thiserror::Error;

/// A common result to be returned to users
pub type CliResult = Result<String, String>;

/// A common result to remove need for typing `Result<T, CliError>`
pub type CliTypedResult<T> = Result<T, CliError>;

/// CLI Errors for reporting through telemetry and outputs
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Aborted command")]
    AbortedError,
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Error (de)serializing '{0}': {1}")]
    BCS(&'static str, #[source] bcs::Error),
    #[error("Invalid arguments: {0}")]
    CommandArgumentError(String),
    #[error("Unable to load config: {0} {1}")]
    ConfigLoadError(String, String),
    #[error("Unable to find config {0}, have you run `aptos init`?")]
    ConfigNotFoundError(String),
    #[error("Error accessing '{0}': {1}")]
    IO(String, #[source] std::io::Error),
    #[error("Move compilation failed: {0}")]
    MoveCompilationError(String),
    #[error("Move unit tests failed")]
    MoveTestError,
    #[error("Move Prover failed")]
    MoveProverError,
    #[error("Unable to parse '{0}': error: {1}")]
    UnableToParse(&'static str, String),
    #[error("Unable to read file '{0}', error: {1}")]
    UnableToReadFile(String, String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl CliError {
    pub fn to_str(&self) -> &'static str {
        match self {
            CliError::AbortedError => "AbortedError",
            CliError::ApiError(_) => "ApiError",
            CliError::BCS(_, _) => "BCS",
            CliError::CommandArgumentError(_) => "CommandArgumentError",
            CliError::ConfigLoadError(_, _) => "ConfigLoadError",
            CliError::ConfigNotFoundError(_) => "ConfigNotFoundError",
            CliError::IO(_, _) => "IO",
            CliError::MoveCompilationError(_) => "MoveCompilationError",
            CliError::MoveTestError => "MoveTestError",
            CliError::MoveProverError => "MoveProverError",
            CliError::UnableToParse(_, _) => "UnableToParse",
            CliError::UnableToReadFile(_, _) => "UnableToReadFile",
            CliError::UnexpectedError(_) => "UnexpectedError",
        }
    }
}

impl From<aptos_config::config::Error> for CliError {
    fn from(e: aptos_config::config::Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<aptos_github_client::Error> for CliError {
    fn from(e: aptos_github_client::Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(e: serde_yaml::Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<base64::DecodeError> for CliError {
    fn from(e: base64::DecodeError) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for CliError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<aptos_crypto::CryptoMaterialError> for CliError {
    fn from(e: aptos_crypto::CryptoMaterialError) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<FromHexError> for CliError {
    fn from(e: FromHexError) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<anyhow::Error> for CliError {
    fn from(e: anyhow::Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

impl From<bcs::Error> for CliError {
    fn from(e: bcs::Error) -> Self {
        CliError::UnexpectedError(e.to_string())
    }
}

/// A result wrapper for displaying either a correct execution result or an error.
///
/// The purpose of this is to have a pretty easy to recognize JSON output format e.g.
///
/// {
///   "Result":{
///     "encoded":{ ... }
///   }
/// }
///
/// {
///   "Error":"Failed to run command"
/// }
///
#[derive(Debug, Serialize)]
enum ResultWrapper<T> {
    Result(T),
    Error(String),
}

impl<T> From<CliTypedResult<T>> for ResultWrapper<T> {
    fn from(result: CliTypedResult<T>) -> Self {
        match result {
            Ok(inner) => ResultWrapper::Result(inner),
            Err(inner) => ResultWrapper::Error(inner.to_string()),
        }
    }
}

/// Convert any successful response to Success
pub async fn to_common_success_result<T>(
    command: &str,
    start_time: Instant,
    result: CliTypedResult<T>,
) -> CliResult {
    to_common_result(command, start_time, result.map(|_| "Success")).await
}

/// For pretty printing outputs in JSON
pub async fn to_common_result<T: Serialize>(
    command: &str,
    start_time: Instant,
    result: CliTypedResult<T>,
) -> CliResult {
    let latency = start_time.elapsed();
    let is_err = result.is_err();
    let error = if let Err(ref error) = result {
        Some(error.to_string())
    } else {
        None
    };
    send_telemetry_event(command, latency, !is_err, error).await;

    let result: ResultWrapper<T> = result.into();
    let string = serde_json::to_string_pretty(&result).unwrap();
    if is_err {
        Err(string)
    } else {
        Ok(string)
    }
}

/// Sends a telemetry event about the CLI build, command and result
async fn send_telemetry_event(
    command: &str,
    latency: Duration,
    success: bool,
    error: Option<String>,
) {
    // Collect the build information
    // TODO: Pass in the version directly from CLI
    let build_information = collect_build_information!();

    // Send the event
    aptos_telemetry::cli_metrics::send_cli_telemetry_event(
        build_information,
        command.into(),
        latency,
        success,
        error,
    )
    .await;
}
