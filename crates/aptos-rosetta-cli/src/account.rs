// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::{BlockArgs, NetworkArgs, UrlArgs};
use aptos_cli_common::command::CliCommand;
use aptos_cli_common::types::{CliResult, CliTypedResult};
use aptos_rosetta::{
    common::native_coin,
    types::{AccountBalanceRequest, AccountBalanceResponse},
};
use aptos_types::account_address::AccountAddress;
use async_trait::async_trait;
use clap::{Parser, Subcommand};

/// Account APIs
///
/// Used for pulling state of an account at a point in time
///
/// [API Spec](https://www.rosetta-api.org/docs/AccountApi.html)
#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    Balance(AccountBalanceCommand),
}

impl AccountCommand {
    pub async fn execute(self) -> CliResult {
        match self {
            AccountCommand::Balance(inner) => inner.execute_serialized().await,
        }
    }
}

/// Retrieve the balance for an account
///
/// [API Spec](https://www.rosetta-api.org/docs/AccountApi.html#accountbalance)
#[derive(Debug, Parser)]
pub struct AccountBalanceCommand {
    #[clap(flatten)]
    network_args: NetworkArgs,
    #[clap(flatten)]
    url_args: UrlArgs,
    #[clap(flatten)]
    block_args: BlockArgs,
    /// Whether to filter the currency to the native coin
    #[clap(long)]
    filter_currency: bool,
    /// Account to list the balance
    #[clap(long, parse(try_from_str=aptos_cli_common::account::load_account_arg))]
    account: AccountAddress,
}

#[async_trait]
impl CliCommand<AccountBalanceResponse> for AccountBalanceCommand {
    fn command_name(&self) -> &'static str {
        "RosettaAccountBalance"
    }

    async fn execute(self) -> CliTypedResult<AccountBalanceResponse> {
        let client = self.url_args.client();
        Ok(client
            .account_balance(&AccountBalanceRequest {
                network_identifier: self.network_args.network_identifier(),
                account_identifier: self.account.into(),
                block_identifier: self.block_args.into(),
                currencies: if self.filter_currency {
                    Some(vec![native_coin()])
                } else {
                    None
                },
            })
            .await?)
    }
}
