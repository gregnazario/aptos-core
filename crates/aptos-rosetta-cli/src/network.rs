// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::{NetworkArgs, UrlArgs};
use aptos_cli_common::command::CliCommand;
use aptos_cli_common::types::{CliResult, CliTypedResult};
use aptos_rosetta::types::{NetworkListResponse, NetworkOptionsResponse, NetworkStatusResponse};
use async_trait::async_trait;
use clap::{Parser, Subcommand};

/// Network APIs
///
/// Used to get status of the current network and what is supported on the API
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html)
#[derive(Debug, Subcommand)]
pub enum NetworkCommand {
    List(NetworkListCommand),
    Options(NetworkOptionsCommand),
    Status(NetworkStatusCommand),
}

impl NetworkCommand {
    pub async fn execute(self) -> CliResult {
        match self {
            NetworkCommand::List(inner) => inner.execute_serialized().await,
            NetworkCommand::Options(inner) => inner.execute_serialized().await,
            NetworkCommand::Status(inner) => inner.execute_serialized().await,
        }
    }
}

/// Get list of available networks
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networklist)
#[derive(Debug, Parser)]
pub struct NetworkListCommand {
    #[clap(flatten)]
    url_args: UrlArgs,
}

#[async_trait]
impl CliCommand<NetworkListResponse> for NetworkListCommand {
    fn command_name(&self) -> &'static str {
        "RosettaListNetworks"
    }

    async fn execute(self) -> CliTypedResult<NetworkListResponse> {
        Ok(self.url_args.client().network_list().await?)
    }
}

/// Get network options
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networkoptions)
#[derive(Debug, Parser)]
pub struct NetworkOptionsCommand {
    #[clap(flatten)]
    network_args: NetworkArgs,
    #[clap(flatten)]
    url_args: UrlArgs,
}

#[async_trait]
impl CliCommand<NetworkOptionsResponse> for NetworkOptionsCommand {
    fn command_name(&self) -> &'static str {
        "RosettaListNetworkOptions"
    }

    async fn execute(self) -> CliTypedResult<NetworkOptionsResponse> {
        let request = self.network_args.network_request();
        Ok(self.url_args.client().network_options(&request).await?)
    }
}

/// Get network status
///
/// [API Spec](https://www.rosetta-api.org/docs/NetworkApi.html#networkstatus)
#[derive(Debug, Parser)]
pub struct NetworkStatusCommand {
    #[clap(flatten)]
    network_args: NetworkArgs,
    #[clap(flatten)]
    url_args: UrlArgs,
}

#[async_trait]
impl CliCommand<NetworkStatusResponse> for NetworkStatusCommand {
    fn command_name(&self) -> &'static str {
        "RosettaListStatus"
    }

    async fn execute(self) -> CliTypedResult<NetworkStatusResponse> {
        let request = self.network_args.network_request();
        Ok(self.url_args.client().network_status(&request).await?)
    }
}
