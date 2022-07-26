// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_cli_base::types::{CliError, CliResult, CliTypedResult};
use aptos_cli_base::utils::start_logger;
use aptos_telemetry::collect_build_information;
use async_trait::async_trait;
use clap::CommandFactory;
use clap::Parser;
use clap_complete::{generate, Shell};
use serde::Serialize;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::Instant;

/// A common trait for all CLI commands to have consistent outputs
#[async_trait]
pub trait CliCommand<T: Serialize + Send>: Send + Sized {
    /// Returns a name for logging purposes
    fn command_name(&self) -> &'static str;

    /// Executes the command, returning a command specific type
    async fn execute(self) -> CliTypedResult<T>;

    /// Executes the command, and serializes it to the common JSON output type
    async fn execute_serialized(self) -> CliResult {
        start_logger();
        self.execute_serialized_without_logger().await
    }

    /// Executes the command, and throws away Ok(result) for the string Success
    async fn execute_serialized_success(self) -> CliResult {
        self.execute_serialized()
            .await
            .map(|_| "Success".to_string())
    }

    /// Same as execute serialized without setting up logging
    async fn execute_serialized_without_logger(self) -> CliResult {
        let start_time = Instant::now();
        let command_name = self.command_name();
        let result = self.execute().await;
        let is_err = result.is_err();

        let latency = start_time.elapsed();
        let error = if let Err(ref error) = result {
            Some(error.to_string())
        } else {
            None
        };

        // Send the event
        // TODO: Need to fix the build information to come from the CLI tool
        aptos_telemetry::cli_metrics::send_cli_telemetry_event(
            collect_build_information!(),
            command_name.into(),
            latency,
            !is_err,
            error,
        )
        .await;

        let result: ResultWrapper<T> = result.into();
        let string = serde_json::to_string_pretty(&result).unwrap();
        if is_err {
            Err(string)
        } else {
            Ok(string)
        }
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
pub enum ResultWrapper<T> {
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

/// Generates shell completion files
///
/// First generate the completion file, then follow the shell specific directions on how
/// to install the completion file.
#[derive(Parser)]
pub struct GenerateShellCompletions<Tool> {
    /// Shell to generate completions for one of [bash, elvish, powershell, zsh]
    #[clap(long)]
    shell: Shell,
    /// File to output shell completions to
    #[clap(long, parse(from_os_str))]
    output_file: PathBuf,
    #[clap(skip)]
    _tool: PhantomData<Tool>,
}

#[async_trait]
impl<Tool: CommandFactory + Send> CliCommand<()> for GenerateShellCompletions<Tool> {
    fn command_name(&self) -> &'static str {
        "GenerateShellCompletions"
    }

    async fn execute(self) -> CliTypedResult<()> {
        let mut command = Tool::command();
        let mut file = std::fs::File::create(self.output_file.as_path())
            .map_err(|err| CliError::IO(self.output_file.display().to_string(), err))?;
        generate(self.shell, &mut command, "aptos".to_string(), &mut file);
        Ok(())
    }
}
