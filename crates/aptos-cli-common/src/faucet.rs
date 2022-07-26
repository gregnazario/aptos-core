// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_cli_base::types::{CliError, CliTypedResult};
use aptos_cli_config::config::CliConfig;
use aptos_types::account_address::AccountAddress;
use clap::Parser;

pub const DEFAULT_FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";

#[derive(Debug, Default, Parser)]
pub struct FaucetOptions {
    /// URL for the faucet
    #[clap(long)]
    faucet_url: Option<reqwest::Url>,
}

impl FaucetOptions {
    pub fn new(faucet_url: Option<reqwest::Url>) -> Self {
        FaucetOptions { faucet_url }
    }

    pub fn faucet_url(&self, profile: &str) -> CliTypedResult<reqwest::Url> {
        if let Some(ref faucet_url) = self.faucet_url {
            Ok(faucet_url.clone())
        } else if let Some(Some(url)) =
            CliConfig::load_profile(profile)?.map(|profile| profile.faucet_url)
        {
            reqwest::Url::parse(&url)
                .map_err(|err| CliError::UnableToParse("config faucet_url", err.to_string()))
        } else {
            reqwest::Url::parse(DEFAULT_FAUCET_URL).map_err(|err| {
                CliError::UnexpectedError(format!("Failed to parse default faucet URL {}", err))
            })
        }
    }
}

pub async fn fund_account(
    faucet_url: reqwest::Url,
    num_coins: u64,
    address: AccountAddress,
) -> CliTypedResult<()> {
    let response = reqwest::Client::new()
        .post(format!(
            "{}mint?amount={}&auth_key={}",
            faucet_url, num_coins, address
        ))
        .send()
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?;
    if response.status() == 200 {
        Ok(())
    } else {
        Err(CliError::ApiError(format!(
            "Faucet issue: {}",
            response.status()
        )))
    }
}
