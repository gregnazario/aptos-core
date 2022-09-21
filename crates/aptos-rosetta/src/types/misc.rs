// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::error::ApiError;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Errors that can be returned by the API
///
/// [API Spec](https://www.rosetta-api.org/docs/models/Error.html)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Error {
    /// Error code
    pub code: u32,
    /// Message that always matches the error code
    pub message: String,
    /// Whether a call can retry on the error
    pub retriable: bool,
    /// Specific details of the error e.g. stack trace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

/// Error details that are specific to the instance
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorDetails {
    /// Related error details
    pub details: String,
}

/// Status of an operation
///
/// [API Spec](https://www.rosetta-api.org/docs/models/OperationStatus.html)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}

/// Represents a Peer, used for discovery
///
/// [API Spec](https://www.rosetta-api.org/docs/models/Peer.html)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Peer {
    peer_id: String,
}

/// [API Spec](https://www.rosetta-api.org/docs/models/SyncStatus.html)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    current_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<String>,
    synced: bool,
}

/// Version information for the current deployment to handle software version matching
///
/// [API Spec](https://www.rosetta-api.org/docs/models/Version.html)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Version {
    /// Rosetta version, this should be hardcoded
    pub rosetta_version: String,
    /// Node version, this should come from the node
    pub node_version: String,
    /// Middleware version, this should be the version of this software
    pub middleware_version: String,
}

/// An internal enum to support Operation typing
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OperationType {
    // Create must always be first for ordering
    CreateAccount,
    // Withdraw must come before deposit
    Withdraw,
    Deposit,
    // Staking operations
    InitializeStakePool,
    AddStake,
    ReactivateStake,
    RotateConsensusKey,
    UpdateNetworkAddresses,
    IncreaseLockup,
    JoinValidatorSet,
    DistributeRewards,
    UnlockStake,
    WithdrawStake,
    LeaveValidatorSet,
    SetOperator,
    SetVoter,
    // Fee must always be last for ordering
    Fee,
}

impl OperationType {
    const CREATE_ACCOUNT: &'static str = "create_account";
    const WITHDRAW: &'static str = "withdraw";
    const DEPOSIT: &'static str = "deposit";
    const REGISTER_VALIDATOR_CANDIDATE: &'static str = "register_validator_candidate";
    const ADD_STAKE: &'static str = "add_stake";
    const REACTIVATE_STAKE: &'static str = "reactivate_stake";
    const ROTATE_CONSENSUS_KEY: &'static str = "rotate_consensus_key";
    const UPDATE_NETWORK_AND_FULLNODE_ADDRESSES: &'static str =
        "update_network_and_fullnode_addresses";
    const INCREASE_LOCKUP: &'static str = "increase_lockup";
    const JOIN_VALIDATOR_SET: &'static str = "join_validator_set";
    const DISTRIBUTE_REWARDS: &'static str = "distribute_rewards";
    const UNLOCK_STAKE: &'static str = "unlock_stake";
    const WITHDRAW_STAKE: &'static str = "withdraw_stake";
    const LEAVE_VALIDATOR_SET: &'static str = "leave_validator_set";
    const SET_OPERATOR: &'static str = "set_operator";
    const FEE: &'static str = "fee";

    pub const ALL: &'static [&'static str] = &[
        Self::CREATE_ACCOUNT,
        Self::WITHDRAW,
        Self::DEPOSIT,
        Self::REGISTER_VALIDATOR_CANDIDATE,
        Self::ADD_STAKE,
        Self::REACTIVATE_STAKE,
        Self::ROTATE_CONSENSUS_KEY,
        Self::UPDATE_NETWORK_AND_FULLNODE_ADDRESSES,
        Self::INCREASE_LOCKUP,
        Self::JOIN_VALIDATOR_SET,
        Self::DISTRIBUTE_REWARDS,
        Self::UNLOCK_STAKE,
        Self::WITHDRAW_STAKE,
        Self::LEAVE_VALIDATOR_SET,
        Self::SET_OPERATOR,
        Self::FEE,
    ];
}

impl FromStr for OperationType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OperationType::*;
        match s.to_lowercase().trim() {
            Self::CREATE_ACCOUNT => Ok(CreateAccount),
            Self::WITHDRAW => Ok(Withdraw),
            Self::DEPOSIT => Ok(Deposit),
            Self::REGISTER_VALIDATOR_CANDIDATE => Ok(InitializeStakePool),
            Self::ADD_STAKE => Ok(AddStake),
            Self::REACTIVATE_STAKE => Ok(ReactivateStake),
            Self::ROTATE_CONSENSUS_KEY => Ok(RotateConsensusKey),
            Self::UPDATE_NETWORK_AND_FULLNODE_ADDRESSES => Ok(UpdateNetworkAddresses),
            Self::INCREASE_LOCKUP => Ok(IncreaseLockup),
            Self::JOIN_VALIDATOR_SET => Ok(JoinValidatorSet),
            Self::DISTRIBUTE_REWARDS => Ok(DistributeRewards),
            Self::UNLOCK_STAKE => Ok(UnlockStake),
            Self::WITHDRAW_STAKE => Ok(WithdrawStake),
            Self::LEAVE_VALIDATOR_SET => Ok(LeaveValidatorSet),
            Self::SET_OPERATOR => Ok(SetOperator),
            Self::FEE => Ok(Fee),
            _ => Err(ApiError::DeserializationFailed(Some(format!(
                "Invalid OperationType: {}",
                s
            )))),
        }
    }
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use OperationType::*;
        f.write_str(match self {
            CreateAccount => Self::CREATE_ACCOUNT,
            Withdraw => Self::WITHDRAW,
            Deposit => Self::DEPOSIT,
            InitializeStakePool => Self::REGISTER_VALIDATOR_CANDIDATE,
            AddStake => Self::ADD_STAKE,
            ReactivateStake => Self::REACTIVATE_STAKE,
            RotateConsensusKey => Self::ROTATE_CONSENSUS_KEY,
            UpdateNetworkAddresses => Self::UPDATE_NETWORK_AND_FULLNODE_ADDRESSES,
            IncreaseLockup => Self::INCREASE_LOCKUP,
            JoinValidatorSet => Self::JOIN_VALIDATOR_SET,
            DistributeRewards => Self::DISTRIBUTE_REWARDS,
            UnlockStake => Self::UNLOCK_STAKE,
            WithdrawStake => Self::WITHDRAW_STAKE,
            LeaveValidatorSet => Self::LEAVE_VALIDATOR_SET,
            SetOperator => Self::SET_OPERATOR,
            Fee => Self::FEE,
        })
    }
}

/// An internal type to support typing of Operation statuses
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationStatusType {
    /// Operation was part of a successfully committed transaction
    Success,
    /// Operation was not part of a successfully committed transaction
    Failure,
}

impl OperationStatusType {
    const SUCCESS: &'static str = "success";
    const FAILURE: &'static str = "failure";

    pub fn all() -> Vec<OperationStatusType> {
        vec![OperationStatusType::Success, OperationStatusType::Failure]
    }
}

impl From<OperationStatusType> for OperationStatus {
    fn from(status: OperationStatusType) -> Self {
        let successful = match status {
            OperationStatusType::Success => true,
            OperationStatusType::Failure => false,
        };

        OperationStatus {
            status: status.to_string(),
            successful,
        }
    }
}

impl TryFrom<OperationStatus> for OperationStatusType {
    type Error = ApiError;

    fn try_from(status: OperationStatus) -> Result<Self, Self::Error> {
        OperationStatusType::from_str(&status.status)
    }
}

impl FromStr for OperationStatusType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            Self::SUCCESS => Ok(OperationStatusType::Success),
            Self::FAILURE => Ok(OperationStatusType::Failure),
            _ => Err(ApiError::DeserializationFailed(Some(format!(
                "Invalid OperationStatusType: {}",
                s
            )))),
        }
    }
}

impl Display for OperationStatusType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OperationStatusType::Success => Self::SUCCESS,
            OperationStatusType::Failure => Self::FAILURE,
        })
    }
}
