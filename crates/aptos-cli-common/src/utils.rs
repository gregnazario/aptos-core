// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::types::{CliError, CliTypedResult};
use aptos_logger::Level;
use aptos_types::account_address::AccountAddress;
use aptos_types::chain_id::ChainId;
use reqwest::Url;

/// Retrieves the chain id from the rest client
pub async fn chain_id(rest_client: &aptos_rest_client::Client) -> CliTypedResult<ChainId> {
    let state = rest_client
        .get_ledger_information()
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?
        .into_inner();
    Ok(ChainId::new(state.chain_id))
}

/// Fund account (and possibly create it) from a faucet
pub async fn fund_account(
    faucet_url: Url,
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

pub fn start_logger() {
    let mut logger = aptos_logger::Logger::new();
    logger
        .channel_size(1000)
        .is_async(false)
        .level(Level::Warn)
        .read_env();
    logger.build();
}
