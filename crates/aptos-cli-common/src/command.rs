// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::types::{to_common_result, to_common_success_result, CliResult, CliTypedResult};
use crate::utils::start_logger;
use async_trait::async_trait;
use serde::Serialize;
use std::time::Instant;

/// A common trait for all CLI commands to have consistent outputs
#[async_trait]
pub trait CliCommand<T: Serialize + Send>: Sized + Send {
    /// Returns a name for logging purposes
    fn command_name(&self) -> &'static str;

    /// Executes the command, returning a command specific type
    async fn execute(self) -> CliTypedResult<T>;

    /// Executes the command, and serializes it to the common JSON output type
    async fn execute_serialized(self) -> CliResult {
        let command_name = self.command_name();
        start_logger();
        let start_time = Instant::now();
        to_common_result(command_name, start_time, self.execute().await).await
    }

    /// Same as execute serialized without setting up logging
    async fn execute_serialized_without_logger(self) -> CliResult {
        let command_name = self.command_name();
        let start_time = Instant::now();
        to_common_result(command_name, start_time, self.execute().await).await
    }

    /// Executes the command, and throws away Ok(result) for the string Success
    async fn execute_serialized_success(self) -> CliResult {
        start_logger();
        let command_name = self.command_name();
        let start_time = Instant::now();
        to_common_success_result(command_name, start_time, self.execute().await).await
    }
}
