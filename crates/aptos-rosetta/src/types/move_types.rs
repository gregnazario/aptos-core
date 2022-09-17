// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! Types and identifiers for parsing Move structs and types

use crate::AccountAddress;
use serde::Deserialize;

pub const ACCOUNT_MODULE: &str = "account";
pub const APTOS_ACCOUNT_MODULE: &str = "aptos_account";
pub const APTOS_COIN_MODULE: &str = "aptos_coin";
pub const COIN_MODULE: &str = "coin";
pub const STAKE_MODULE: &str = "stake";

pub const ACCOUNT_RESOURCE: &str = "Account";
pub const APTOS_COIN_RESOURCE: &str = "AptosCoin";
pub const COIN_INFO_RESOURCE: &str = "CoinInfo";
pub const COIN_STORE_RESOURCE: &str = "CoinStore";
pub const STAKE_POOL_RESOURCE: &str = "StakePool";

pub const CREATE_ACCOUNT_FUNCTION: &str = "create_account";
pub const TRANSFER_FUNCTION: &str = "transfer";
pub const SET_OPERATOR_FUNCTION: &str = "set_operator";

pub const DECIMALS_FIELD: &str = "decimal";
pub const DEPOSIT_EVENTS_FIELD: &str = "deposit_events";
pub const WITHDRAW_EVENTS_FIELD: &str = "withdraw_events";
pub const SET_OPERATOR_EVENTS_FIELD: &str = "set_operator_events";
pub const SEQUENCE_NUMBER_FIELD: &str = "sequence_number";
pub const SYMBOL_FIELD: &str = "symbol";

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RegisterValidatorCandidateEvent {
    pool_address: AccountAddress,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct SetOperatorEvent {
    pool_address: AccountAddress,
    old_operator: AccountAddress,
    new_operator: AccountAddress,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct AddStakeEvent {
    pool_address: AccountAddress,
    amount_added: u64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct ReactivateStakeEvent {
    pool_address: AccountAddress,
    amount: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct RotateConsensusKeyEvent {
    pool_address: AccountAddress,
    old_consensus_pubkey: Vec<u8>,
    new_consensus_pubkey: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize)]
struct UpdateNetworkAndFullnodeAddressesEvent {
    pool_address: AccountAddress,
    old_network_addresses: Vec<u8>,
    new_network_addresses: Vec<u8>,
    old_fullnode_addresses: Vec<u8>,
    new_fullnode_addresses: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct IncreaseLockupEvent {
    pool_address: AccountAddress,
    old_locked_until_secs: u64,
    new_locked_until_secs: u64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct JoinValidatorSetEvent {
    pool_address: AccountAddress,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct DistributeRewardsEvent {
    pool_address: AccountAddress,
    rewards_amount: u64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct UnlockStakeEvent {
    pool_address: AccountAddress,
    amount_unlocked: u64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct WithdrawStakeEvent {
    pool_address: AccountAddress,
    amount_withdrawn: u64,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct LeaveValidatorSetEvent {
    pool_address: AccountAddress,
}
