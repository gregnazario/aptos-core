// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::metrics::COUNTER;
use aptos_crypto::{hash::CryptoHash, HashValue};
use aptos_metrics_core::IntCounterHelper;
use aptos_types::{
    state_store::{
        state_key::StateKey,
        state_value::{DbStateValue, StateValue},
    },
    transaction::Version,
};

#[derive(Clone, Debug)]
pub struct DbStateUpdate {
    /// The version where the key got updated (incl. deletion).
    pub version: Version,
    /// `None` indicates deletion.
    pub value: Option<DbStateValue>,
}

impl DbStateUpdate {
    pub fn to_jmt_update_opt(
        &self,
        key: StateKey,
        min_version: Version,
    ) -> Option<(HashValue, Option<(HashValue, StateKey)>)> {
        // Items from < min_version are cached old items.
        if self.version < min_version {
            return None;
        }

        match &self.value {
            None => {
                // HotNonExistent is not explicitly evicted for now, so this must be a real delete
                // on the jmt
                Some((CryptoHash::hash(&key), None))
            },
            Some(db_val) => {
                if db_val.is_hot_non_existent() {
                    // not persisting HotNoneExistent for now
                    None
                } else {
                    Some((
                        CryptoHash::hash(&key),
                        Some((CryptoHash::hash(db_val.expect_state_value()), key)),
                    ))
                }
            },
        }
    }

    pub fn expect_non_delete(&self) -> &DbStateValue {
        self.value.as_ref().expect("Unexpected deletion.")
    }
}

#[derive(Clone, Debug)]
pub struct StateUpdateRef<'kv> {
    /// The version where the key got updated (incl. deletion).
    pub version: Version,
    /// `None` indicates deletion.
    /// TODO(aldenhu): Maybe upgrade to make DbStateValue here already,  but for now this is raw
    ///                VM output that doesn't involve hot state manipulation.
    pub value: Option<&'kv StateValue>,
}

impl<'kv> StateUpdateRef<'kv> {
    pub fn to_db_state_update(&self, access_time_secs: u32) -> DbStateUpdate {
        DbStateUpdate {
            version: self.version,
            value: self
                .value
                .cloned()
                .map(|val| val.into_db_state_value(access_time_secs)),
        }
    }

    pub fn value_hash_opt(&self) -> Option<HashValue> {
        self.value.as_ref().map(|val| val.hash())
    }
}

#[derive(Clone, Debug)]
pub enum MemorizedStateRead {
    /// Underlying storage doesn't have an entry for this state key.
    NonExistent,
    /// A creation or modification at a known version.
    Value {
        version: Version,
        value: DbStateValue,
    },
}

impl MemorizedStateRead {
    pub fn from_db_get(tuple_opt: Option<(Version, StateValue)>) -> Self {
        match tuple_opt {
            None => Self::NonExistent,
            Some((version, value)) => Self::Value {
                version,
                value: value.into_db_state_value(0),
            },
        }
    }

    pub fn from_speculative_state(db_update: DbStateUpdate) -> Self {
        match db_update.value {
            None => Self::NonExistent,
            Some(value) => Self::Value {
                version: db_update.version,
                value,
            },
        }
    }

    pub fn from_hot_state_hit(db_update: DbStateUpdate) -> Self {
        match db_update.value {
            None => unreachable!("Hot state doesn't have None value."),
            Some(value) => Self::Value {
                version: db_update.version,
                value,
            },
        }
    }

    /// TODO(aldenhu): Remove. Use only in a context where the access time doesn't matter
    pub fn dummy_from_state_update_ref(state_update_ref: &StateUpdateRef) -> Self {
        match state_update_ref.value {
            None => Self::NonExistent,
            Some(value) => Self::Value {
                version: state_update_ref.version,
                value: value.clone().into_db_state_value(0),
            },
        }
    }

    pub fn to_state_value_opt(&self) -> Option<StateValue> {
        self.to_state_value_ref_opt().cloned()
    }

    pub fn to_state_value_ref_opt(&self) -> Option<&StateValue> {
        match self {
            Self::NonExistent => None,
            Self::Value { value, .. } => value.to_state_value_opt(),
        }
    }

    pub fn into_state_value_opt(self) -> Option<StateValue> {
        match self {
            Self::NonExistent => None,
            Self::Value { value, .. } => value.into_state_value_opt(),
        }
    }

    pub fn to_hot_state_refresh(&self, access_time_secs: u32) -> Option<DbStateUpdate> {
        const READ_CACHE_REFRESH_INTERVAL: u32 = 600;

        match self {
            MemorizedStateRead::NonExistent => {
                COUNTER.inc_with(&["memorized_read_new_hot_non_existent"]);
                Some(DbStateUpdate {
                    // Dummy creation version
                    version: 0,
                    value: Some(DbStateValue::new_hot_non_existent(access_time_secs)),
                })
            },
            MemorizedStateRead::Value { version, value } => {
                let old_ts = value.access_time_secs();
                if old_ts + READ_CACHE_REFRESH_INTERVAL < access_time_secs {
                    if old_ts == 0 {
                        COUNTER.inc_with(&["memorized_read_new_hot"]);
                    } else {
                        COUNTER.inc_with(&["memorized_read_refreshed_hot"]);
                    }
                    Some(DbStateUpdate {
                        version: *version,
                        value: Some(value.clone()),
                    })
                } else {
                    COUNTER.inc_with(&["memorized_read_still_hot"]);
                    None
                } // end if-else
            }, // end ::Value
        } // end match
    }
}
