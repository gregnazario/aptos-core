// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::{BlockArgs, NetworkArgs, UrlArgs};
use aptos_cli_common::command::CliCommand;
use aptos_cli_common::types::{CliResult, CliTypedResult};
use aptos_rosetta::types::{BlockRequest, BlockResponse};
use async_trait::async_trait;
use clap::{Parser, Subcommand};

/// Block APIs
///
/// Used for pulling blocks from the blockchain
///
/// [API Spec](https://www.rosetta-api.org/docs/BlockApi.html)
#[derive(Debug, Subcommand)]
pub enum BlockCommand {
    Get(GetBlockCommand),
}

impl BlockCommand {
    pub async fn execute(self) -> CliResult {
        match self {
            BlockCommand::Get(inner) => inner.execute_serialized().await,
        }
    }
}

/// Get a block by transaction hash or version
///
/// [API Spec](https://www.rosetta-api.org/docs/BlockApi.html#block)
#[derive(Debug, Parser)]
pub struct GetBlockCommand {
    #[clap(flatten)]
    block_args: BlockArgs,
    #[clap(flatten)]
    network_args: NetworkArgs,
    #[clap(flatten)]
    url_args: UrlArgs,
}

#[async_trait]
impl CliCommand<BlockResponse> for GetBlockCommand {
    fn command_name(&self) -> &'static str {
        "RosettaGetBlock"
    }

    async fn execute(self) -> CliTypedResult<BlockResponse> {
        let request = BlockRequest {
            network_identifier: self.network_args.network_identifier(),
            block_identifier: self.block_args.into(),
        };
        Ok(self.url_args.client().block(&request).await?)
    }
}
