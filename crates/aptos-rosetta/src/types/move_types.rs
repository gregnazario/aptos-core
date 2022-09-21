// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! Types and identifiers for parsing Move structs and types
use crate::AccountAddress;
use serde::Deserialize;
use std::collections::BTreeMap;

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

#[derive(Clone, Debug, Deserialize)]
pub struct Capability {
    pub address: AccountAddress,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pool {
    pub shareholders_limit: u64,
    pub total_coins: u64,
    pub total_shares: u64,
    pub shares: BTreeMap<AccountAddress, u64>,
    pub shareholders: Vec<AccountAddress>,
    pub scaling_factor: u64,
}

pub mod staking {
    use crate::AccountAddress;
    use serde::Deserialize;

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct RegisterValidatorCandidateEvent {
        pub pool_address: AccountAddress,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct SetOperatorEvent {
        pub pool_address: AccountAddress,
        pub old_operator: AccountAddress,
        pub new_operator: AccountAddress,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct AddStakeEvent {
        pub pool_address: AccountAddress,
        pub amount_added: u64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct ReactivateStakeEvent {
        pub pool_address: AccountAddress,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct RotateConsensusKeyEvent {
        pub pool_address: AccountAddress,
        pub old_consensus_pubkey: Vec<u8>,
        pub new_consensus_pubkey: Vec<u8>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UpdateNetworkAndFullnodeAddressesEvent {
        pub pool_address: AccountAddress,
        pub old_network_addresses: Vec<u8>,
        pub new_network_addresses: Vec<u8>,
        pub old_fullnode_addresses: Vec<u8>,
        pub new_fullnode_addresses: Vec<u8>,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct IncreaseLockupEvent {
        pub pool_address: AccountAddress,
        pub old_locked_until_secs: u64,
        pub new_locked_until_secs: u64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct JoinValidatorSetEvent {
        pub pool_address: AccountAddress,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct DistributeRewardsEvent {
        pub pool_address: AccountAddress,
        pub rewards_amount: u64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct UnlockStakeEvent {
        pub pool_address: AccountAddress,
        pub amount_unlocked: u64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct WithdrawStakeEvent {
        pub pool_address: AccountAddress,
        pub amount_withdrawn: u64,
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    pub struct LeaveValidatorSetEvent {
        pub pool_address: AccountAddress,
    }
}

pub mod staking_contract {
    use crate::types::{Capability, Pool};
    use crate::AccountAddress;
    use aptos_types::event::EventHandle;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Deserialize)]
    pub struct StakingContract {
        pub principal: u64,
        pub pool_address: AccountAddress,
        pub owner_cap: Capability,
        pub commission_percentage: u64,
        pub distribution_pool: Pool,
        pub signer_cap: Capability,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Store {
        pub staking_contracts: BTreeMap<AccountAddress, StakingContract>,
        pub create_staking_contract_events: EventHandle,
        pub update_voter_events: EventHandle,
        pub reset_lockup_events: EventHandle,
        pub add_stake_events: EventHandle,
        pub request_commission_events: EventHandle,
        pub unlock_stake_events: EventHandle,
        pub switch_operator_events: EventHandle,
        pub add_distribution_events: EventHandle,
        pub distribute_events: EventHandle,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct CreateStakingContractEvent {
        pub operator: AccountAddress,
        pub voter: AccountAddress,
        pub pool_address: AccountAddress,
        pub principal: u64,
        pub commission_percentage: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UpdateVoterEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub old_voter: AccountAddress,
        pub new_voter: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ResetLockupEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AddStakeEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct RequestCommissionEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub accumulated_rewards: u64,
        pub commission_amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UnlockStakeEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub amount: u64,
        pub commission_paid: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct SwitchOperatorEvent {
        pub old_operator: AccountAddress,
        pub new_operator: AccountAddress,
        pub pool_address: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AddDistributionEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DistributeEvent {
        pub operator: AccountAddress,
        pub pool_address: AccountAddress,
        pub recipient: AccountAddress,
        pub amount: u64,
    }
}

mod vesting {
    use crate::types::{Capability, Pool};
    use crate::AccountAddress;
    use aptos_types::event::EventHandle;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Deserialize)]
    pub struct FixedPoint32 {
        pub value: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct VestingSchedule {
        pub schedule: Vec<FixedPoint32>,
        pub start_timestamp_secs: u64,
        pub period_duration: u64,
        pub last_vested_period: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct StakingInfo {
        pub pool_address: AccountAddress,
        pub operator: AccountAddress,
        pub voter: AccountAddress,
        pub commission_percentage: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct VestingContract {
        pub state: u64,
        pub admin: AccountAddress,
        pub grant_pool: Pool,
        pub beneficiaries: BTreeMap<AccountAddress, AccountAddress>,
        pub vesting_schedule: VestingSchedule,
        pub withdrawal_address: AccountAddress,
        pub staking: StakingInfo,
        pub remaining_grant: u64,
        pub signer_cap: Capability,
        pub update_operator_events: EventHandle,
        pub update_voter_events: EventHandle,
        pub reset_lockup_events: EventHandle,
        pub set_beneficiary_events: EventHandle,
        pub unlock_rewards_events: EventHandle,
        pub vest_events: EventHandle,
        pub distribute_events: EventHandle,
        pub terminate_events: EventHandle,
        pub admin_withdraw_events: EventHandle,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct VestingAccountManagement {
        pub roles: BTreeMap<String, AccountAddress>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AdminStore {
        pub vesting_contracts: Vec<AccountAddress>,
        pub nonce: u64,
        pub create_events: EventHandle,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct CreateVestingContractEvent {
        pub operator: AccountAddress,
        pub voter: AccountAddress,
        pub grant_amount: u64,
        pub withdrawal_address: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub commission_percentage: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UpdateOperatorEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub old_operator: AccountAddress,
        pub new_operator: AccountAddress,
        pub commission_percentage: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UpdateVoterEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub old_voter: AccountAddress,
        pub new_voter: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ResetLockupEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub new_lockup_expiration_secs: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct SetBeneficiaryEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub shareholder: AccountAddress,
        pub old_beneficiary: AccountAddress,
        pub new_beneficiary: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UnlockRewardsEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct VestEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub staking_pool_address: AccountAddress,
        pub period_vested: u64,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DistributeEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub amount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct TerminateEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct AdminWithdrawEvent {
        pub admin: AccountAddress,
        pub vesting_contract_address: AccountAddress,
        pub amount: u64,
    }
}
