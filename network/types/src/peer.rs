// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

use crate::NetworkTypesError;
use aptos_crypto::x25519;
use aptos_types::{network_address::NetworkAddress, PeerId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

pub type PeerSet = HashMap<PeerId, Peer>;

/// Represents the Role that a peer plays in the network ecosystem rather than the type of node.
/// Determines how nodes are connected to other nodes, and how discovery views them.
///
/// Rules for upstream nodes via Peer Role:
///
/// Validator -> Always upstream if not Validator else P2P
/// PreferredUpstream -> Always upstream, overriding any other discovery
/// ValidatorFullNode -> Always upstream for incoming connections (including other ValidatorFullNodes)
/// Upstream -> Upstream, if no ValidatorFullNode or PreferredUpstream.  Useful for initial seed discovery
/// Downstream -> Downstream, defining a controlled downstream that I always want to connect
/// Known -> A known peer, but it has no particular role assigned to it
/// Unknown -> Undiscovered peer, likely due to a non-mutually authenticated connection always downstream
#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PeerRole {
    Validator = 0,
    PreferredUpstream,
    Upstream,
    ValidatorFullNode,
    Downstream,
    Known,
    #[default]
    Unknown,
}

impl PeerRole {
    pub fn is_validator(self) -> bool {
        self == PeerRole::Validator
    }

    pub fn is_vfn(self) -> bool {
        self == PeerRole::ValidatorFullNode
    }

    pub fn as_str(self) -> &'static str {
        match self {
            PeerRole::Validator => "validator",
            PeerRole::PreferredUpstream => "preferred_upstream_peer",
            PeerRole::Upstream => "upstream_peer",
            PeerRole::ValidatorFullNode => "validator_fullnode",
            PeerRole::Downstream => "downstream_peer",
            PeerRole::Known => "known_peer",
            PeerRole::Unknown => "unknown_peer",
        }
    }
}

impl fmt::Debug for PeerRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for PeerRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a single seed configuration for a seed peer
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct Peer {
    pub addresses: Vec<NetworkAddress>,
    pub keys: HashSet<x25519::PublicKey>,
    pub role: PeerRole,
}

impl Peer {
    /// Combines `Vec<NetworkAddress>` keys with the `HashSet` given
    pub fn new(
        addresses: Vec<NetworkAddress>,
        mut keys: HashSet<x25519::PublicKey>,
        role: PeerRole,
    ) -> Peer {
        let addr_keys = addresses
            .iter()
            .filter_map(NetworkAddress::find_noise_proto);
        keys.extend(addr_keys);
        Peer {
            addresses,
            keys,
            role,
        }
    }

    /// Combines two `Peer`.  Note: Does not merge duplicate addresses
    pub fn extend(&mut self, other: Peer) -> Result<(), NetworkTypesError> {
        if self.role == other.role {
            return Err(NetworkTypesError::InvariantViolation(format!(
                "Roles don't match self {:?} vs other {:?}",
                self.role, other.role
            )));
        }
        self.addresses.extend(other.addresses);
        self.keys.extend(other.keys);
        Ok(())
    }

    pub fn from_addrs(role: PeerRole, addresses: Vec<NetworkAddress>) -> Peer {
        let keys: HashSet<x25519::PublicKey> = addresses
            .iter()
            .filter_map(NetworkAddress::find_noise_proto)
            .collect();
        Peer::new(addresses, keys, role)
    }
}

/// Access control policy for peer connections.
/// Determines which peers are allowed or blocked from connecting.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessControlPolicy {
    /// Only allow connections from peers in this list. All others are blocked.
    AllowList(HashSet<PeerId>),
    /// Block connections from peers in this list. All others are allowed.
    BlockList(HashSet<PeerId>),
}

impl AccessControlPolicy {
    /// Check if a peer is allowed based on this access control policy.
    /// Returns true if the peer should be allowed, false if blocked.
    pub fn is_peer_allowed(&self, peer_id: &PeerId) -> bool {
        match self {
            AccessControlPolicy::AllowList(allowed_peers) => allowed_peers.contains(peer_id),
            AccessControlPolicy::BlockList(blocked_peers) => !blocked_peers.contains(peer_id),
        }
    }
}
