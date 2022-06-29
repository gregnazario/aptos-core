// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{check_network, handle_request, strip_hex_prefix, with_context},
    error::{ApiError, ApiResult},
    types::{Block, BlockRequest, BlockResponse, Currency},
    RosettaContext,
};
use aptos_crypto::HashValue;
use aptos_logger::{debug, trace};
use aptos_rest_client::{
    aptos_api_types::{
        MoveStructTag, MoveStructValue, MoveType, MoveValue::Address, TransactionInfo,
        WriteSetChange,
    },
    Transaction,
};
use aptos_sdk::move_types::{
    ident_str,
    identifier::Identifier,
    language_storage::{ModuleId, StructTag, TypeTag},
};
use aptos_types::account_address::AccountAddress;
use std::{collections::HashMap, str::FromStr};
use warp::Filter;
use move_deps::move_resource_viewer::AnnotatedMoveStruct;

pub fn routes(
    server_context: RosettaContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("block")
            .and(warp::body::json())
            .and(with_context(server_context))
            .and_then(handle_request(block)),
    )
}

/// Retrieves a block (in this case a single transaction) given it's identifier.
///
/// Our implementation allows for by `index`, which is the ledger `version` or by
/// transaction `hash`.
///
/// [API Spec](https://www.rosetta-api.org/docs/BlockApi.html#block)
async fn block(request: BlockRequest, server_context: RosettaContext) -> ApiResult<BlockResponse> {
    debug!("/block");
    trace!(
        request = ?request,
        server_context = ?server_context,
        "/block",
    );

    check_network(request.network_identifier, &server_context)?;

    let rest_client = server_context.rest_client()?;

    // Retrieve by block or by hash, both or neither is not allowed
    let (parent_transaction, transaction): (Transaction, _) = match (
        &request.block_identifier.index,
        &request.block_identifier.hash,
    ) {
        (Some(version), None) => {
            // For the genesis block, we populate parent_block_identifier with the
            // same genesis block. Refer to
            // https://www.rosetta-api.org/docs/common_mistakes.html#malformed-genesis-block
            if *version == 0 {
                let response = rest_client.get_transaction_by_version(*version).await?;
                let txn = response.into_inner();
                (txn.clone(), txn)
            } else {
                let response = rest_client
                    .get_transactions(Some(*version - 1), Some(2))
                    .await?;
                let txns = response.into_inner();
                if txns.len() != 2 {
                    return Err(ApiError::TransactionRetrievalFailed(
                        "Failed to get transaction and parent transaction".to_string(),
                    ));
                }
                (
                    txns.first().cloned().unwrap(),
                    txns.last().cloned().unwrap(),
                )
            }
        }
        (None, Some(hash)) => {
            // Allow 0x in front of hash
            let hash = HashValue::from_str(strip_hex_prefix(hash))
                .map_err(|err| ApiError::BadBlockRequest(err.to_string()))?;
            let response = rest_client.get_transaction(hash).await?;
            let txn = response.into_inner();
            let version = txn.version().unwrap();

            // If this is genesis, set parent to genesis txn
            if version == 0 {
                (txn.clone(), txn)
            } else {
                let parent_response = rest_client.get_transaction_by_version(version - 1).await?;
                (parent_response.into_inner(), txn)
            }
        }
        (None, None) => {
            // Get current version
            let response = rest_client.get_transactions(None, Some(2)).await?;
            let txns = response.into_inner();
            if txns.len() != 2 {
                return Err(ApiError::TransactionRetrievalFailed(
                    "Failed to get transaction and parent transaction".to_string(),
                ));
            }
            (
                txns.first().cloned().unwrap(),
                txns.last().cloned().unwrap(),
            )
        }
        (_, _) => {
            return Err(ApiError::BadBlockRequest(
                "Can't specify both transaction hash and version".to_string(),
            ))
        }
    };

    // Build up the transaction, which should contain the `operations` as the change set
    let transaction_info = transaction.transaction_info()?;
    let transactions = vec![transaction_info.into()];

    // note: timestamps are in microseconds, so we convert to milliseconds
    let mut timestamp = transaction.timestamp() / 1000;

    // Rosetta doesn't like timestamps before 2000
    if timestamp == 0 {
        timestamp = 946713600000;
    }
    let block = Block {
        block_identifier: transaction_info.into(),
        parent_block_identifier: parent_transaction.transaction_info()?.into(),
        timestamp,
        transactions,
    };

    let response = BlockResponse {
        block: Some(block),
        other_transactions: None,
    };

    Ok(response)
}

async fn get_transaction(rest_client: &aptos_rest_client::Client, txn: &TransactionInfo) {
    // TODO: get new currencies from updates and store locally
    // Convert operations from txn
    let mut currencies: HashMap<String, Currency> = HashMap::new();
    let mut changes = HashMap::new();
    for change in txn.changes {
        match change {
            WriteSetChange::WriteResource { address, data, .. } => {
                let test_coin = StructTag {
                    address: AccountAddress::ONE,
                    module: Identifier::new("Coin").unwrap(),
                    name: Identifier::new("CoinStore").unwrap(),
                    type_params: vec![TypeTag::Struct(StructTag {
                        address: AccountAddress::ONE,
                        module: Identifier::new("TestCoin").unwrap(),
                        name: Identifier::new("TestCoin").unwrap(),
                        type_params: vec![],
                    })],
                };

                // If it is a coinstore
                if data.typ.address == Address::from(AccountAddress::ONE)
                    && data.typ.module == Identifier::new("Coin").unwrap()
                    && data.typ.name == Identifier::new("CoinStore").unwrap()
                {
                    // Retrieve the coin type
                    if let MoveType::Struct(coin_type) =
                        data.typ.generic_type_params.first().unwrap()
                    {
                        // TODO: If the coin type is a generic, we should handle that, and we aren't here
                        if !coin_type.generic_type_params.is_empty() {
                            continue;
                        }

                        // If we already processed this coin type, let's continue
                        if currencies.contains_key(&coin_type.to_string()) {
                            // do nothing
                        } else if let Some(currency) = get_currency(&rest_client, coin_type)? {
                            currencies.insert(coin_type.to_string(), currency);
                        } else {
                            // This is a malformed currency, and we have to just skip it, otherwise the API will completely fail
                            continue;
                        }

                        // Now get the balance changes
                        let data: AnnotatedMoveStruct = data.data.into();
                        for item in data.value {
                            match item.0.as_str() {
                                "deposit_events" => {
                                    item.1.
                                },
                                "withdraw_events" => {},
                                _ => continue,
                            }
                        }
                        match data.data {
                            MoveStructValue(inner) => inner,
                        }
                    } else {
                        // Continue, but this is a bad coin so we'll skip it
                        continue;
                    }
                }
            }
            // skip non resource changes
            _ => continue,
        }
    }

    Transaction {
        transaction_identifier: txn.into(),
        operations: vec![],
        related_transactions: None,
    }
}

fn get_currency(
    rest_client: &aptos_rest_client::Client,
    coin_type: &MoveStructTag,
) -> ApiResult<Option<Currency>> {
    let response = rest_client
        .get_account_resource(coin_type.address.into(), &coin_type.to_string())
        .await?;

    // TODO: Handle all the unwraps
    if let Some(resource) = response.into_inner() {
        if let Some(data) = resource.data.as_object() {
            let decimals = u64::from_str(data.get("decimals").unwrap().as_str().unwrap()).unwrap();
            let symbol = data.get("symbol").unwrap().as_str().unwrap();

            return Ok(Some(Currency {
                symbol: symbol.to_string(),
                decimals,
            }));
        }
    }
    // TODO: This is a bad coin, but we have to skip it for now
    Ok(None)
}
