// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

use crate::{
    config::{
        identity_config::{Identity, IdentityFromStorage},
        Error, IdentityBlob,
    },
    network_id::NetworkId,
    utils,
};
use aptos_crypto::{x25519, Uniform};
pub use aptos_network_types::{
    AccessControlPolicy, Peer, PeerRole, PeerSet, CONNECTION_BACKOFF_BASE,
    CONNECTIVITY_CHECK_INTERVAL_MS, HANDSHAKE_VERSION, MAX_APPLICATION_MESSAGE_SIZE,
    MAX_CONNECTION_DELAY_MS, MAX_FRAME_SIZE, MAX_FULLNODE_OUTBOUND_CONNECTIONS,
    MAX_INBOUND_CONNECTIONS, MAX_MESSAGE_METADATA_SIZE, MAX_MESSAGE_SIZE, MESSAGE_PADDING_SIZE,
    NETWORK_CHANNEL_SIZE, PING_FAILURES_TOLERATED, PING_INTERVAL_MS, PING_TIMEOUT_MS,
};
use aptos_secure_storage::{CryptoStorage, KVStorage, Storage};
use aptos_short_hex_str::AsShortHexStr;
use aptos_types::{
    account_address::from_identity_public_key, network_address::NetworkAddress,
    transaction::authenticator::AuthenticationKey, PeerId,
};
use rand::{
    rngs::{OsRng, StdRng},
    Rng, SeedableRng,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    path::PathBuf,
    string::ToString,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NetworkConfig {
    /// Maximum backoff delay for connecting outbound to peers
    pub max_connection_delay_ms: u64,
    /// Base for outbound connection backoff
    pub connection_backoff_base: u64,
    /// Rate to check connectivity to connected peers
    pub connectivity_check_interval_ms: u64,
    /// Size of all network channels
    pub network_channel_size: usize,
    /// Choose a protocol to discover and dial out to other peers on this network.
    /// `DiscoveryMethod::None` disables discovery and dialing out (unless you have
    /// seed peers configured).
    pub discovery_method: DiscoveryMethod,
    /// Same as `discovery_method` but allows for multiple
    pub discovery_methods: Vec<DiscoveryMethod>,
    /// Identity of this network
    pub identity: Identity,
    // TODO: Add support for multiple listen/advertised addresses in config.
    /// The address that this node is listening on for new connections.
    pub listen_address: NetworkAddress,
    /// Select this to enforce that both peers should authenticate each other, otherwise
    /// authentication only occurs for outgoing connections.
    pub mutual_authentication: bool,
    /// ID of the network to differentiate between networks
    pub network_id: NetworkId,
    /// Number of threads to run for networking
    pub runtime_threads: Option<usize>,
    /// Overrides for the size of the inbound and outbound buffers for each peer.
    /// NOTE: The defaults are None, so socket options are not called. Change to Some values with
    /// caution. Experiments have shown that relying on Linux's default tcp auto-tuning can perform
    /// better than setting these. In particular, for larger values to take effect, the
    /// `net.core.rmem_max` and `net.core.wmem_max` sysctl values may need to be increased. On a
    /// vanilla GCP machine, these are set to 212992. Without increasing the sysctl values and
    /// setting a value will constrain the buffer size to the sysctl value. (In contrast, default
    /// auto-tuning can increase beyond these values.)
    pub inbound_rx_buffer_size_bytes: Option<u32>,
    pub inbound_tx_buffer_size_bytes: Option<u32>,
    pub outbound_rx_buffer_size_bytes: Option<u32>,
    pub outbound_tx_buffer_size_bytes: Option<u32>,
    /// Addresses of initial peers to connect to. In a mutual_authentication network,
    /// we will extract the public keys from these addresses to set our initial
    /// trusted peers set.  TODO: Replace usage in configs with `seeds` this is for backwards compatibility
    pub seed_addrs: HashMap<PeerId, Vec<NetworkAddress>>,
    /// The initial peers to connect to prior to onchain discovery
    pub seeds: PeerSet,
    /// The maximum size of an inbound or outbound request frame
    pub max_frame_size: usize,
    /// Enables proxy protocol on incoming connections to get original source addresses
    pub enable_proxy_protocol: bool,
    /// Interval to send healthcheck pings to peers
    pub ping_interval_ms: u64,
    /// Timeout until a healthcheck ping is rejected
    pub ping_timeout_ms: u64,
    /// Number of failed healthcheck pings until a peer is marked unhealthy
    pub ping_failures_tolerated: u64,
    /// Maximum number of outbound connections, limited by ConnectivityManager
    pub max_outbound_connections: usize,
    /// Maximum number of outbound connections, limited by PeerManager
    pub max_inbound_connections: usize,
    /// The maximum size of an inbound or outbound message (it may be divided into multiple frame)
    pub max_message_size: usize,
    /// The maximum number of parallel message deserialization tasks that can run (per application)
    pub max_parallel_deserialization_tasks: Option<usize>,
    /// Whether or not to enable latency aware peer dialing
    pub enable_latency_aware_dialing: bool,
    /// Access control policy for peer connections. If not specified, all
    /// peers are allowed. Otherwise, the specified policy is enforced.
    pub access_control_policy: Option<AccessControlPolicy>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig::network_with_id(NetworkId::default())
    }
}

impl NetworkConfig {
    pub fn network_with_id(network_id: NetworkId) -> NetworkConfig {
        let mutual_authentication = network_id.is_validator_network();
        let mut config = Self {
            discovery_method: DiscoveryMethod::None,
            discovery_methods: Vec::new(),
            identity: Identity::None,
            listen_address: "/ip4/0.0.0.0/tcp/6180".parse().unwrap(),
            mutual_authentication,
            network_id,
            runtime_threads: None,
            seed_addrs: HashMap::new(),
            seeds: PeerSet::default(),
            max_frame_size: MAX_FRAME_SIZE,
            enable_proxy_protocol: false,
            max_connection_delay_ms: MAX_CONNECTION_DELAY_MS,
            connectivity_check_interval_ms: CONNECTIVITY_CHECK_INTERVAL_MS,
            network_channel_size: NETWORK_CHANNEL_SIZE,
            connection_backoff_base: CONNECTION_BACKOFF_BASE,
            ping_interval_ms: PING_INTERVAL_MS,
            ping_timeout_ms: PING_TIMEOUT_MS,
            ping_failures_tolerated: PING_FAILURES_TOLERATED,
            max_outbound_connections: MAX_FULLNODE_OUTBOUND_CONNECTIONS,
            max_inbound_connections: MAX_INBOUND_CONNECTIONS,
            max_message_size: MAX_MESSAGE_SIZE,
            inbound_rx_buffer_size_bytes: None,
            inbound_tx_buffer_size_bytes: None,
            outbound_rx_buffer_size_bytes: None,
            outbound_tx_buffer_size_bytes: None,
            max_parallel_deserialization_tasks: None,
            enable_latency_aware_dialing: true,
            access_control_policy: None,
        };

        // Configure the number of parallel deserialization tasks
        config.configure_num_deserialization_tasks();

        // Prepare the identity based on the identity format
        config.prepare_identity();

        config
    }

    /// Configures the number of parallel deserialization tasks
    /// based on the number of CPU cores of the machine. This is
    /// only done if the config does not specify a value.
    fn configure_num_deserialization_tasks(&mut self) {
        if self.max_parallel_deserialization_tasks.is_none() {
            self.max_parallel_deserialization_tasks = Some(num_cpus::get());
        }
    }

    pub fn identity_key(&self) -> x25519::PrivateKey {
        let key = match &self.identity {
            Identity::FromConfig(config) => Some(config.key.private_key()),
            Identity::FromStorage(config) => {
                let storage: Storage = (&config.backend).into();
                let key = storage
                    .export_private_key(&config.key_name)
                    .expect("Unable to read key");
                let key = x25519::PrivateKey::from_ed25519_private_bytes(&key.to_bytes())
                    .expect("Unable to convert key");
                Some(key)
            },
            Identity::FromFile(config) => {
                let identity_blob: IdentityBlob = IdentityBlob::from_file(&config.path).unwrap();
                Some(identity_blob.network_private_key)
            },
            Identity::None => None,
        };
        key.expect("identity key should be present")
    }

    pub fn identity_from_storage(&self) -> IdentityFromStorage {
        if let Identity::FromStorage(identity) = self.identity.clone() {
            identity
        } else {
            panic!("Invalid identity found, expected a storage identity.");
        }
    }

    pub fn discovery_methods(&self) -> Vec<&DiscoveryMethod> {
        // TODO: This is a backwards compatibility feature.  Deprecate discovery_method
        if self.discovery_method != DiscoveryMethod::None && !self.discovery_methods.is_empty() {
            panic!("Can't specify discovery_method and discovery_methods")
        } else if self.discovery_method != DiscoveryMethod::None {
            vec![&self.discovery_method]
        } else {
            self.discovery_methods
                .iter()
                .filter(|method| &&DiscoveryMethod::None != method)
                .collect()
        }
    }

    pub fn set_listen_address_and_prepare_identity(&mut self) -> Result<(), Error> {
        // Set the listen address to the local IP if it is not specified
        if self.listen_address.to_string().is_empty() {
            self.listen_address = utils::get_local_ip().ok_or_else(|| {
                Error::InvariantViolation("Failed to get the Local IP".to_string())
            })?;
        }

        // Prepare the identity
        self.prepare_identity();

        Ok(())
    }

    pub fn peer_id(&self) -> PeerId {
        match &self.identity {
            Identity::FromConfig(config) => Some(config.peer_id),
            Identity::FromStorage(config) => {
                let storage: Storage = (&config.backend).into();
                let peer_id = storage
                    .get::<PeerId>(&config.peer_id_name)
                    .expect("Unable to read peer id")
                    .value;
                Some(peer_id)
            },
            Identity::FromFile(config) => {
                let identity_blob: IdentityBlob = IdentityBlob::from_file(&config.path).unwrap();

                // If account is not specified, generate peer id from public key
                if let Some(address) = identity_blob.account_address {
                    Some(address)
                } else {
                    Some(from_identity_public_key(
                        identity_blob.network_private_key.public_key(),
                    ))
                }
            },
            Identity::None => None,
        }
        .expect("peer id should be present")
    }

    fn prepare_identity(&mut self) {
        match &mut self.identity {
            Identity::FromStorage(_) => (),
            Identity::None => {
                let mut rng = StdRng::from_seed(OsRng.r#gen());
                let key = x25519::PrivateKey::generate(&mut rng);
                let peer_id = from_identity_public_key(key.public_key());
                self.identity = Identity::from_config_auto_generated(key, peer_id);
            },
            Identity::FromConfig(config) => {
                if config.peer_id == PeerId::ZERO {
                    config.peer_id = from_identity_public_key(config.key.public_key());
                }
            },
            Identity::FromFile(_) => (),
        };
    }

    pub fn random(&mut self, rng: &mut StdRng) {
        self.random_with_peer_id(rng, None);
    }

    pub fn random_with_peer_id(&mut self, rng: &mut StdRng, peer_id: Option<PeerId>) {
        let identity_key = x25519::PrivateKey::generate(rng);
        let peer_id = if let Some(peer_id) = peer_id {
            peer_id
        } else {
            AuthenticationKey::try_from(identity_key.public_key().as_slice())
                .unwrap()
                .account_address()
        };
        self.identity = Identity::from_config(identity_key, peer_id);
    }

    fn verify_address(peer_id: &PeerId, addr: &NetworkAddress) -> Result<(), Error> {
        if !addr.is_aptosnet_addr() {
            return Err(Error::InvariantViolation(format!(
                "Unexpected seed peer address format: peer_id: {}, addr: '{}'",
                peer_id.short_str(),
                addr,
            )));
        }

        Ok(())
    }

    // Verifies both the `seed_addrs` and `seeds` before they're merged
    pub fn verify_seeds(&self) -> Result<(), Error> {
        for (peer_id, addrs) in self.seed_addrs.iter() {
            for addr in addrs {
                Self::verify_address(peer_id, addr)?;
            }
        }

        for (peer_id, seed) in self.seeds.iter() {
            for addr in seed.addresses.iter() {
                Self::verify_address(peer_id, addr)?;
            }

            // Require there to be a pubkey somewhere, either in the address (assumed by `is_aptosnet_addr`)
            if seed.keys.is_empty() && seed.addresses.is_empty() {
                return Err(Error::InvariantViolation(format!(
                    "Seed peer {} has no pubkeys",
                    peer_id.short_str(),
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryMethod {
    Onchain,
    File(FileDiscovery),
    Rest(RestDiscovery),
    None,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDiscovery {
    pub path: PathBuf,
    pub interval_secs: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RestDiscovery {
    pub url: url::Url,
    pub interval_secs: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_num_parallel_deserialization_tasks() {
        let network_config = NetworkConfig::default();
        assert_eq!(
            network_config.max_parallel_deserialization_tasks,
            Some(num_cpus::get())
        );

        let mut network_config = NetworkConfig {
            max_parallel_deserialization_tasks: Some(1),
            ..NetworkConfig::default()
        };

        network_config.configure_num_deserialization_tasks();
        assert_eq!(network_config.max_parallel_deserialization_tasks, Some(1));
    }

    #[test]
    fn test_access_control_policy_default() {
        let network_config = NetworkConfig::default();
        assert!(network_config.access_control_policy.is_none());
    }

    #[test]
    fn test_access_control_policy_allow_list_serialization() {
        let peer_1 = PeerId::random();
        let peer_2 = PeerId::random();

        let access_control_policy_yaml = format!(
            r#"
            allow_list:
              - "{}"
              - "{}"
            "#,
            peer_1, peer_2
        );
        let access_control_policy: AccessControlPolicy =
            serde_yaml::from_str(&access_control_policy_yaml).unwrap();

        match access_control_policy {
            AccessControlPolicy::AllowList(peers) => {
                assert_eq!(peers.len(), 2);
                assert!(peers.contains(&peer_1));
                assert!(peers.contains(&peer_2));
            },
            _ => panic!("Expected allow list policy!"),
        }
    }

    #[test]
    fn test_access_control_policy_block_list_serialization() {
        let peer_1 = PeerId::random();
        let peer_2 = PeerId::random();
        let peer_3 = PeerId::random();

        let access_control_policy_yaml = format!(
            r#"
            block_list:
              - "{}"
              - "{}"
              - "{}"
            "#,
            peer_1, peer_2, peer_3
        );
        let access_control_policy: AccessControlPolicy =
            serde_yaml::from_str(&access_control_policy_yaml).unwrap();

        match access_control_policy {
            AccessControlPolicy::BlockList(peers) => {
                assert_eq!(peers.len(), 3);
                assert!(peers.contains(&peer_1));
                assert!(peers.contains(&peer_2));
                assert!(peers.contains(&peer_3));
            },
            _ => panic!("Expected block list policy!"),
        }
    }

    #[test]
    fn test_access_control_policy_allow_list() {
        let peer_1 = PeerId::random();
        let peer_2 = PeerId::random();
        let peer_3 = PeerId::random();
        let allow_list = HashSet::from([peer_1, peer_2]);
        let access_control_policy = AccessControlPolicy::AllowList(allow_list);

        assert!(access_control_policy.is_peer_allowed(&peer_1));
        assert!(access_control_policy.is_peer_allowed(&peer_2));
        assert!(!access_control_policy.is_peer_allowed(&peer_3));
    }

    #[test]
    fn test_access_control_policy_block_list() {
        let peer_1 = PeerId::random();
        let peer_2 = PeerId::random();
        let peer_3 = PeerId::random();
        let block_list = HashSet::from([peer_3]);
        let access_control_policy = AccessControlPolicy::BlockList(block_list);

        assert!(access_control_policy.is_peer_allowed(&peer_1));
        assert!(access_control_policy.is_peer_allowed(&peer_2));
        assert!(!access_control_policy.is_peer_allowed(&peer_3));
    }
}
