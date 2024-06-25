
<a id="0x1_keyless_account"></a>

# Module `0x1::keyless_account`

This module is responsible for configuring keyless blockchain accounts which were introduced in
[AIP&#45;61](https://github.com/aptos&#45;foundation/AIPs/blob/main/aips/aip&#45;61.md).


-  [Struct `Group`](#0x1_keyless_account_Group)
-  [Resource `Groth16VerificationKey`](#0x1_keyless_account_Groth16VerificationKey)
-  [Resource `Configuration`](#0x1_keyless_account_Configuration)
-  [Constants](#@Constants_0)
-  [Function `new_groth16_verification_key`](#0x1_keyless_account_new_groth16_verification_key)
-  [Function `new_configuration`](#0x1_keyless_account_new_configuration)
-  [Function `validate_groth16_vk`](#0x1_keyless_account_validate_groth16_vk)
-  [Function `update_groth16_verification_key`](#0x1_keyless_account_update_groth16_verification_key)
-  [Function `update_configuration`](#0x1_keyless_account_update_configuration)
-  [Function `update_training_wheels`](#0x1_keyless_account_update_training_wheels)
-  [Function `update_max_exp_horizon`](#0x1_keyless_account_update_max_exp_horizon)
-  [Function `remove_all_override_auds`](#0x1_keyless_account_remove_all_override_auds)
-  [Function `add_override_aud`](#0x1_keyless_account_add_override_aud)
-  [Function `set_groth16_verification_key_for_next_epoch`](#0x1_keyless_account_set_groth16_verification_key_for_next_epoch)
-  [Function `set_configuration_for_next_epoch`](#0x1_keyless_account_set_configuration_for_next_epoch)
-  [Function `update_training_wheels_for_next_epoch`](#0x1_keyless_account_update_training_wheels_for_next_epoch)
-  [Function `update_max_exp_horizon_for_next_epoch`](#0x1_keyless_account_update_max_exp_horizon_for_next_epoch)
-  [Function `remove_all_override_auds_for_next_epoch`](#0x1_keyless_account_remove_all_override_auds_for_next_epoch)
-  [Function `add_override_aud_for_next_epoch`](#0x1_keyless_account_add_override_aud_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_keyless_account_on_new_epoch)
-  [Specification](#@Specification_1)


```move
module 0x1::keyless_account {
    use 0x1::bn254_algebra;
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::crypto_algebra;
    use 0x1::ed25519;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::system_addresses;
}
```


<a id="0x1_keyless_account_Group"></a>

## Struct `Group`



```move
module 0x1::keyless_account {
    #[resource_group(#[scope = global])]
    struct Group
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_keyless_account_Groth16VerificationKey"></a>

## Resource `Groth16VerificationKey`

The 288&#45;byte Groth16 verification key (VK) for the ZK relation that implements keyless accounts


```move
module 0x1::keyless_account {
    #[resource_group_member(#[group = 0x1::keyless_account::Group])]
    struct Groth16VerificationKey has drop, store, key
}
```


##### Fields


<dl>
<dt>
`alpha_g1: vector<u8>`
</dt>
<dd>
 32&#45;byte serialization of `alpha * G`, where `G` is the generator of `G1`.
</dd>
<dt>
`beta_g2: vector<u8>`
</dt>
<dd>
 64&#45;byte serialization of `alpha * H`, where `H` is the generator of `G2`.
</dd>
<dt>
`gamma_g2: vector<u8>`
</dt>
<dd>
 64&#45;byte serialization of `gamma * H`, where `H` is the generator of `G2`.
</dd>
<dt>
`delta_g2: vector<u8>`
</dt>
<dd>
 64&#45;byte serialization of `delta * H`, where `H` is the generator of `G2`.
</dd>
<dt>
`gamma_abc_g1: vector<vector<u8>>`
</dt>
<dd>
 `\forall i \in {0, ..., \ell}, 64-byte serialization of gamma^{-1} * (beta * a_i + alpha * b_i + c_i) * H`, where
 `H` is the generator of `G1` and `\ell` is 1 for the ZK relation.
</dd>
</dl>


<a id="0x1_keyless_account_Configuration"></a>

## Resource `Configuration`



```move
module 0x1::keyless_account {
    #[resource_group_member(#[group = 0x1::keyless_account::Group])]
    struct Configuration has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`override_aud_vals: vector<string::String>`
</dt>
<dd>
 An override `aud` for the identity of a recovery service, which will help users recover their keyless accounts
 associated with dapps or wallets that have disappeared.
 IMPORTANT: This recovery service &#42;&#42;cannot&#42;&#42; on its own take over user accounts; a user must first sign in
 via OAuth in the recovery service in order to allow it to rotate any of that user&apos;s keyless accounts.
</dd>
<dt>
`max_signatures_per_txn: u16`
</dt>
<dd>
 No transaction can have more than this many keyless signatures.
</dd>
<dt>
`max_exp_horizon_secs: u64`
</dt>
<dd>
 How far in the future from the JWT issued at time the EPK expiry can be set.
</dd>
<dt>
`training_wheels_pubkey: option::Option<vector<u8>>`
</dt>
<dd>
 The training wheels PK, if training wheels are on
</dd>
<dt>
`max_commited_epk_bytes: u16`
</dt>
<dd>
 The max length of an ephemeral public key supported in our circuit (93 bytes)
</dd>
<dt>
`max_iss_val_bytes: u16`
</dt>
<dd>
 The max length of the value of the JWT&apos;s `iss` field supported in our circuit (e.g., `"https://accounts.google.com"`)
</dd>
<dt>
`max_extra_field_bytes: u16`
</dt>
<dd>
 The max length of the JWT field name and value (e.g., `"max_age":"18"`) supported in our circuit
</dd>
<dt>
`max_jwt_header_b64_bytes: u32`
</dt>
<dd>
 The max length of the base64url&#45;encoded JWT header in bytes supported in our circuit
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_keyless_account_E_INVALID_BN254_G1_SERIALIZATION"></a>

A serialized BN254 G1 point is invalid.


```move
module 0x1::keyless_account {
    const E_INVALID_BN254_G1_SERIALIZATION: u64 = 2;
}
```


<a id="0x1_keyless_account_E_INVALID_BN254_G2_SERIALIZATION"></a>

A serialized BN254 G2 point is invalid.


```move
module 0x1::keyless_account {
    const E_INVALID_BN254_G2_SERIALIZATION: u64 = 3;
}
```


<a id="0x1_keyless_account_E_TRAINING_WHEELS_PK_WRONG_SIZE"></a>

The training wheels PK needs to be 32 bytes long.


```move
module 0x1::keyless_account {
    const E_TRAINING_WHEELS_PK_WRONG_SIZE: u64 = 1;
}
```


<a id="0x1_keyless_account_new_groth16_verification_key"></a>

## Function `new_groth16_verification_key`



```move
module 0x1::keyless_account {
    public fun new_groth16_verification_key(alpha_g1: vector<u8>, beta_g2: vector<u8>, gamma_g2: vector<u8>, delta_g2: vector<u8>, gamma_abc_g1: vector<vector<u8>>): keyless_account::Groth16VerificationKey
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun new_groth16_verification_key(alpha_g1: vector<u8>,
                                            beta_g2: vector<u8>,
                                            gamma_g2: vector<u8>,
                                            delta_g2: vector<u8>,
                                            gamma_abc_g1: vector<vector<u8>>
    ): Groth16VerificationKey {
        Groth16VerificationKey {
            alpha_g1,
            beta_g2,
            gamma_g2,
            delta_g2,
            gamma_abc_g1,
        }
    }
}
```


<a id="0x1_keyless_account_new_configuration"></a>

## Function `new_configuration`



```move
module 0x1::keyless_account {
    public fun new_configuration(override_aud_val: vector<string::String>, max_signatures_per_txn: u16, max_exp_horizon_secs: u64, training_wheels_pubkey: option::Option<vector<u8>>, max_commited_epk_bytes: u16, max_iss_val_bytes: u16, max_extra_field_bytes: u16, max_jwt_header_b64_bytes: u32): keyless_account::Configuration
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun new_configuration(
        override_aud_val: vector<String>,
        max_signatures_per_txn: u16,
        max_exp_horizon_secs: u64,
        training_wheels_pubkey: Option<vector<u8>>,
        max_commited_epk_bytes: u16,
        max_iss_val_bytes: u16,
        max_extra_field_bytes: u16,
        max_jwt_header_b64_bytes: u32
    ): Configuration {
        Configuration {
            override_aud_vals: override_aud_val,
            max_signatures_per_txn,
            max_exp_horizon_secs,
            training_wheels_pubkey,
            max_commited_epk_bytes,
            max_iss_val_bytes,
            max_extra_field_bytes,
            max_jwt_header_b64_bytes,
        }
    }
}
```


<a id="0x1_keyless_account_validate_groth16_vk"></a>

## Function `validate_groth16_vk`

Pre&#45;validate the VK to actively&#45;prevent incorrect VKs from being set on&#45;chain.


```move
module 0x1::keyless_account {
    fun validate_groth16_vk(vk: &keyless_account::Groth16VerificationKey)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    fun validate_groth16_vk(vk: &Groth16VerificationKey) {
        // Could be leveraged to speed up the VM deserialization of the VK by 2x, since it can assume the points are valid.
        assert!(option::is_some(&crypto_algebra::deserialize<bn254_algebra::G1, bn254_algebra::FormatG1Compr>(&vk.alpha_g1)), E_INVALID_BN254_G1_SERIALIZATION);
        assert!(option::is_some(&crypto_algebra::deserialize<bn254_algebra::G2, bn254_algebra::FormatG2Compr>(&vk.beta_g2)), E_INVALID_BN254_G2_SERIALIZATION);
        assert!(option::is_some(&crypto_algebra::deserialize<bn254_algebra::G2, bn254_algebra::FormatG2Compr>(&vk.gamma_g2)), E_INVALID_BN254_G2_SERIALIZATION);
        assert!(option::is_some(&crypto_algebra::deserialize<bn254_algebra::G2, bn254_algebra::FormatG2Compr>(&vk.delta_g2)), E_INVALID_BN254_G2_SERIALIZATION);
        for (i in 0..vector::length(&vk.gamma_abc_g1)) {
            assert!(option::is_some(&crypto_algebra::deserialize<bn254_algebra::G1, bn254_algebra::FormatG1Compr>(vector::borrow(&vk.gamma_abc_g1, i))), E_INVALID_BN254_G1_SERIALIZATION);
        };
    }
}
```


<a id="0x1_keyless_account_update_groth16_verification_key"></a>

## Function `update_groth16_verification_key`

Sets the Groth16 verification key, only callable during genesis. To call during governance proposals, use
`set_groth16_verification_key_for_next_epoch`.

WARNING: See `set_groth16_verification_key_for_next_epoch` for caveats.


```move
module 0x1::keyless_account {
    public fun update_groth16_verification_key(fx: &signer, vk: keyless_account::Groth16VerificationKey)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_groth16_verification_key(fx: &signer, vk: Groth16VerificationKey) {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();
        // There should not be a previous resource set here.
        move_to(fx, vk);
    }
}
```


<a id="0x1_keyless_account_update_configuration"></a>

## Function `update_configuration`

Sets the keyless configuration, only callable during genesis. To call during governance proposals, use
`set_configuration_for_next_epoch`.

WARNING: See `set_configuration_for_next_epoch` for caveats.


```move
module 0x1::keyless_account {
    public fun update_configuration(fx: &signer, config: keyless_account::Configuration)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_configuration(fx: &signer, config: Configuration) {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();
        // There should not be a previous resource set here.
        move_to(fx, config);
    }
}
```


<a id="0x1_keyless_account_update_training_wheels"></a>

## Function `update_training_wheels`



```move
module 0x1::keyless_account {
    #[deprecated]
    public fun update_training_wheels(fx: &signer, pk: option::Option<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_training_wheels(fx: &signer, pk: Option<vector<u8>>) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        if (option::is_some(&pk)) {
            assert!(vector::length(option::borrow(&pk)) == 32, E_TRAINING_WHEELS_PK_WRONG_SIZE)
        };

        let config = borrow_global_mut<Configuration>(signer::address_of(fx));
        config.training_wheels_pubkey = pk;
    }
}
```


<a id="0x1_keyless_account_update_max_exp_horizon"></a>

## Function `update_max_exp_horizon`



```move
module 0x1::keyless_account {
    #[deprecated]
    public fun update_max_exp_horizon(fx: &signer, max_exp_horizon_secs: u64)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_max_exp_horizon(fx: &signer, max_exp_horizon_secs: u64) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        let config = borrow_global_mut<Configuration>(signer::address_of(fx));
        config.max_exp_horizon_secs = max_exp_horizon_secs;
    }
}
```


<a id="0x1_keyless_account_remove_all_override_auds"></a>

## Function `remove_all_override_auds`



```move
module 0x1::keyless_account {
    #[deprecated]
    public fun remove_all_override_auds(fx: &signer)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun remove_all_override_auds(fx: &signer) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        let config = borrow_global_mut<Configuration>(signer::address_of(fx));
        config.override_aud_vals = vector[];
    }
}
```


<a id="0x1_keyless_account_add_override_aud"></a>

## Function `add_override_aud`



```move
module 0x1::keyless_account {
    #[deprecated]
    public fun add_override_aud(fx: &signer, aud: string::String)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun add_override_aud(fx: &signer, aud: String) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        let config = borrow_global_mut<Configuration>(signer::address_of(fx));
        vector::push_back(&mut config.override_aud_vals, aud);
    }
}
```


<a id="0x1_keyless_account_set_groth16_verification_key_for_next_epoch"></a>

## Function `set_groth16_verification_key_for_next_epoch`

Queues up a change to the Groth16 verification key. The change will only be effective after reconfiguration.
Only callable via governance proposal.

WARNING: To mitigate against DoS attacks, a VK change should be done together with a training wheels PK change,
so that old ZKPs for the old VK cannot be replayed as potentially&#45;valid ZKPs.

WARNING: If a malicious key is set, this would lead to stolen funds.


```move
module 0x1::keyless_account {
    public fun set_groth16_verification_key_for_next_epoch(fx: &signer, vk: keyless_account::Groth16VerificationKey)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun set_groth16_verification_key_for_next_epoch(fx: &signer, vk: Groth16VerificationKey) {
        system_addresses::assert_aptos_framework(fx);
        config_buffer::upsert<Groth16VerificationKey>(vk);
    }
}
```


<a id="0x1_keyless_account_set_configuration_for_next_epoch"></a>

## Function `set_configuration_for_next_epoch`

Queues up a change to the keyless configuration. The change will only be effective after reconfiguration. Only
callable via governance proposal.

WARNING: A malicious `Configuration` could lead to DoS attacks, create liveness issues, or enable a malicious
recovery service provider to phish users&apos; accounts.


```move
module 0x1::keyless_account {
    public fun set_configuration_for_next_epoch(fx: &signer, config: keyless_account::Configuration)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun set_configuration_for_next_epoch(fx: &signer, config: Configuration) {
        system_addresses::assert_aptos_framework(fx);
        config_buffer::upsert<Configuration>(config);
    }
}
```


<a id="0x1_keyless_account_update_training_wheels_for_next_epoch"></a>

## Function `update_training_wheels_for_next_epoch`

Convenience method to queue up a change to the training wheels PK. The change will only be effective after
reconfiguration. Only callable via governance proposal.

WARNING: If a malicious key is set, this &#42;could&#42; lead to stolen funds.


```move
module 0x1::keyless_account {
    public fun update_training_wheels_for_next_epoch(fx: &signer, pk: option::Option<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_training_wheels_for_next_epoch(fx: &signer, pk: Option<vector<u8>>) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);

        // If a PK is being set, validate it first.
        if (option::is_some(&pk)) {
            let bytes = *option::borrow(&pk);
            let vpk = ed25519::new_validated_public_key_from_bytes(bytes);
            assert!(option::is_some(&vpk), E_TRAINING_WHEELS_PK_WRONG_SIZE)
        };

        let config = if (config_buffer::does_exist<Configuration>()) {
            config_buffer::extract<Configuration>()
        } else {
            *borrow_global<Configuration>(signer::address_of(fx))
        };

        config.training_wheels_pubkey = pk;

        set_configuration_for_next_epoch(fx, config);
    }
}
```


<a id="0x1_keyless_account_update_max_exp_horizon_for_next_epoch"></a>

## Function `update_max_exp_horizon_for_next_epoch`

Convenience method to queues up a change to the max expiration horizon. The change will only be effective after
reconfiguration. Only callable via governance proposal.


```move
module 0x1::keyless_account {
    public fun update_max_exp_horizon_for_next_epoch(fx: &signer, max_exp_horizon_secs: u64)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun update_max_exp_horizon_for_next_epoch(fx: &signer, max_exp_horizon_secs: u64) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);

        let config = if (config_buffer::does_exist<Configuration>()) {
            config_buffer::extract<Configuration>()
        } else {
            *borrow_global<Configuration>(signer::address_of(fx))
        };

        config.max_exp_horizon_secs = max_exp_horizon_secs;

        set_configuration_for_next_epoch(fx, config);
    }
}
```


<a id="0x1_keyless_account_remove_all_override_auds_for_next_epoch"></a>

## Function `remove_all_override_auds_for_next_epoch`

Convenience method to queue up clearing the set of override `aud`&apos;s. The change will only be effective after
reconfiguration. Only callable via governance proposal.

WARNING: When no override `aud` is set, recovery of keyless accounts associated with applications that disappeared
is no longer possible.


```move
module 0x1::keyless_account {
    public fun remove_all_override_auds_for_next_epoch(fx: &signer)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun remove_all_override_auds_for_next_epoch(fx: &signer) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);

        let config = if (config_buffer::does_exist<Configuration>()) {
            config_buffer::extract<Configuration>()
        } else {
            *borrow_global<Configuration>(signer::address_of(fx))
        };

        config.override_aud_vals = vector[];

        set_configuration_for_next_epoch(fx, config);
    }
}
```


<a id="0x1_keyless_account_add_override_aud_for_next_epoch"></a>

## Function `add_override_aud_for_next_epoch`

Convenience method to queue up an append to to the set of override `aud`&apos;s. The change will only be effective
after reconfiguration. Only callable via governance proposal.

WARNING: If a malicious override `aud` is set, this &#42;could&#42; lead to stolen funds.


```move
module 0x1::keyless_account {
    public fun add_override_aud_for_next_epoch(fx: &signer, aud: string::String)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public fun add_override_aud_for_next_epoch(fx: &signer, aud: String) acquires Configuration {
        system_addresses::assert_aptos_framework(fx);

        let config = if (config_buffer::does_exist<Configuration>()) {
            config_buffer::extract<Configuration>()
        } else {
            *borrow_global<Configuration>(signer::address_of(fx))
        };

        vector::push_back(&mut config.override_aud_vals, aud);

        set_configuration_for_next_epoch(fx, config);
    }
}
```


<a id="0x1_keyless_account_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the queued up configuration changes, if there are any.


```move
module 0x1::keyless_account {
    public(friend) fun on_new_epoch(fx: &signer)
}
```


##### Implementation


```move
module 0x1::keyless_account {
    public(friend) fun on_new_epoch(fx: &signer) acquires Groth16VerificationKey, Configuration {
        system_addresses::assert_aptos_framework(fx);

        if (config_buffer::does_exist<Groth16VerificationKey>()) {
            let vk = config_buffer::extract();
            if (exists<Groth16VerificationKey>(@aptos_framework)) {
                *borrow_global_mut<Groth16VerificationKey>(@aptos_framework) = vk;
            } else {
                move_to(fx, vk);
            }
        };

        if (config_buffer::does_exist<Configuration>()) {
            let config = config_buffer::extract();
            if (exists<Configuration>(@aptos_framework)) {
                *borrow_global_mut<Configuration>(@aptos_framework) = config;
            } else {
                move_to(fx, config);
            }
        };
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::keyless_account {
    pragma verify=false;
}
```
