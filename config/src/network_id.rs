// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

pub use aptos_network_types::{NetworkContext, NetworkId, PeerNetworkId};

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::RoleType;
    use aptos_types::PeerId;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_ensure_network_id_order() {
        assert!(NetworkId::Validator < NetworkId::Vfn);
        assert!(NetworkId::Vfn < NetworkId::Public);
        assert!(NetworkId::Validator < NetworkId::Public);
    }

    #[test]
    fn test_serialization() {
        for id in [NetworkId::Validator, NetworkId::Vfn, NetworkId::Public] {
            let encoded = serde_yaml::to_string(&id).unwrap();
            let decoded: NetworkId = serde_yaml::from_str(encoded.as_str()).unwrap();
            assert_eq!(id, decoded);
            let encoded = bcs::to_bytes(&id).unwrap();
            let decoded: NetworkId = bcs::from_bytes(&encoded).unwrap();
            assert_eq!(id, decoded);
        }
    }

    #[test]
    fn test_network_context_serialization() {
        let peer_id = PeerId::random();
        let context = NetworkContext::new(RoleType::Validator, NetworkId::Vfn, peer_id);
        let expected = format!(
            "---\nrole: {}\nnetwork_id: {}\npeer_id: {:x}\n",
            RoleType::Validator,
            "vfn",
            peer_id
        );
        assert_eq!(expected, serde_yaml::to_string(&context).unwrap());
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename = "NetworkId", rename_all = "snake_case")]
    enum OldNetworkId {
        Validator,
        Public,
        Private(String),
    }

    #[test]
    fn test_backwards_compatibility() {
        for (old, new) in [
            (OldNetworkId::Validator, NetworkId::Validator),
            (OldNetworkId::Public, NetworkId::Public),
            (
                OldNetworkId::Private("vfn".to_string()),
                NetworkId::Vfn,
            ),
        ] {
            let encoded = serde_yaml::to_string(&old).unwrap();
            let decoded: NetworkId = serde_yaml::from_str(&encoded).unwrap();
            assert_eq!(new, decoded);
            let encoded = bcs::to_bytes(&old).unwrap();
            let decoded: NetworkId = bcs::from_bytes(&encoded).unwrap();
            assert_eq!(new, decoded);

            let encoded = serde_yaml::to_string(&new).unwrap();
            let decoded: OldNetworkId = serde_yaml::from_str(&encoded).unwrap();
            assert_eq!(old, decoded);
            let encoded = bcs::to_bytes(&new).unwrap();
            let decoded: OldNetworkId = bcs::from_bytes(&encoded).unwrap();
            assert_eq!(old, decoded);
        }
    }
}
