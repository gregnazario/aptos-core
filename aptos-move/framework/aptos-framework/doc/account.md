
<a id="0x1_account"></a>

# Module `0x1::account`



-  [Struct `KeyRotation`](#0x1_account_KeyRotation)
-  [Resource `Account`](#0x1_account_Account)
-  [Struct `KeyRotationEvent`](#0x1_account_KeyRotationEvent)
-  [Struct `CoinRegisterEvent`](#0x1_account_CoinRegisterEvent)
-  [Struct `CapabilityOffer`](#0x1_account_CapabilityOffer)
-  [Struct `RotationCapability`](#0x1_account_RotationCapability)
-  [Struct `SignerCapability`](#0x1_account_SignerCapability)
-  [Resource `OriginatingAddress`](#0x1_account_OriginatingAddress)
-  [Struct `RotationProofChallenge`](#0x1_account_RotationProofChallenge)
-  [Struct `RotationCapabilityOfferProofChallenge`](#0x1_account_RotationCapabilityOfferProofChallenge)
-  [Struct `SignerCapabilityOfferProofChallenge`](#0x1_account_SignerCapabilityOfferProofChallenge)
-  [Struct `RotationCapabilityOfferProofChallengeV2`](#0x1_account_RotationCapabilityOfferProofChallengeV2)
-  [Struct `SignerCapabilityOfferProofChallengeV2`](#0x1_account_SignerCapabilityOfferProofChallengeV2)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_account_initialize)
-  [Function `create_account_if_does_not_exist`](#0x1_account_create_account_if_does_not_exist)
-  [Function `create_account`](#0x1_account_create_account)
-  [Function `create_account_unchecked`](#0x1_account_create_account_unchecked)
-  [Function `exists_at`](#0x1_account_exists_at)
-  [Function `get_guid_next_creation_num`](#0x1_account_get_guid_next_creation_num)
-  [Function `get_sequence_number`](#0x1_account_get_sequence_number)
-  [Function `increment_sequence_number`](#0x1_account_increment_sequence_number)
-  [Function `get_authentication_key`](#0x1_account_get_authentication_key)
-  [Function `rotate_authentication_key_internal`](#0x1_account_rotate_authentication_key_internal)
-  [Function `rotate_authentication_key_call`](#0x1_account_rotate_authentication_key_call)
-  [Function `rotate_authentication_key`](#0x1_account_rotate_authentication_key)
-  [Function `rotate_authentication_key_with_rotation_capability`](#0x1_account_rotate_authentication_key_with_rotation_capability)
-  [Function `offer_rotation_capability`](#0x1_account_offer_rotation_capability)
-  [Function `is_rotation_capability_offered`](#0x1_account_is_rotation_capability_offered)
-  [Function `get_rotation_capability_offer_for`](#0x1_account_get_rotation_capability_offer_for)
-  [Function `revoke_rotation_capability`](#0x1_account_revoke_rotation_capability)
-  [Function `revoke_any_rotation_capability`](#0x1_account_revoke_any_rotation_capability)
-  [Function `offer_signer_capability`](#0x1_account_offer_signer_capability)
-  [Function `is_signer_capability_offered`](#0x1_account_is_signer_capability_offered)
-  [Function `get_signer_capability_offer_for`](#0x1_account_get_signer_capability_offer_for)
-  [Function `revoke_signer_capability`](#0x1_account_revoke_signer_capability)
-  [Function `revoke_any_signer_capability`](#0x1_account_revoke_any_signer_capability)
-  [Function `create_authorized_signer`](#0x1_account_create_authorized_signer)
-  [Function `assert_valid_rotation_proof_signature_and_get_auth_key`](#0x1_account_assert_valid_rotation_proof_signature_and_get_auth_key)
-  [Function `update_auth_key_and_originating_address_table`](#0x1_account_update_auth_key_and_originating_address_table)
-  [Function `create_resource_address`](#0x1_account_create_resource_address)
-  [Function `create_resource_account`](#0x1_account_create_resource_account)
-  [Function `create_framework_reserved_account`](#0x1_account_create_framework_reserved_account)
-  [Function `create_guid`](#0x1_account_create_guid)
-  [Function `new_event_handle`](#0x1_account_new_event_handle)
-  [Function `register_coin`](#0x1_account_register_coin)
-  [Function `create_signer_with_capability`](#0x1_account_create_signer_with_capability)
-  [Function `get_signer_capability_address`](#0x1_account_get_signer_capability_address)
-  [Function `verify_signed_message`](#0x1_account_verify_signed_message)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `create_account_if_does_not_exist`](#@Specification_1_create_account_if_does_not_exist)
    -  [Function `create_account`](#@Specification_1_create_account)
    -  [Function `create_account_unchecked`](#@Specification_1_create_account_unchecked)
    -  [Function `exists_at`](#@Specification_1_exists_at)
    -  [Function `get_guid_next_creation_num`](#@Specification_1_get_guid_next_creation_num)
    -  [Function `get_sequence_number`](#@Specification_1_get_sequence_number)
    -  [Function `increment_sequence_number`](#@Specification_1_increment_sequence_number)
    -  [Function `get_authentication_key`](#@Specification_1_get_authentication_key)
    -  [Function `rotate_authentication_key_internal`](#@Specification_1_rotate_authentication_key_internal)
    -  [Function `rotate_authentication_key_call`](#@Specification_1_rotate_authentication_key_call)
    -  [Function `rotate_authentication_key`](#@Specification_1_rotate_authentication_key)
    -  [Function `rotate_authentication_key_with_rotation_capability`](#@Specification_1_rotate_authentication_key_with_rotation_capability)
    -  [Function `offer_rotation_capability`](#@Specification_1_offer_rotation_capability)
    -  [Function `is_rotation_capability_offered`](#@Specification_1_is_rotation_capability_offered)
    -  [Function `get_rotation_capability_offer_for`](#@Specification_1_get_rotation_capability_offer_for)
    -  [Function `revoke_rotation_capability`](#@Specification_1_revoke_rotation_capability)
    -  [Function `revoke_any_rotation_capability`](#@Specification_1_revoke_any_rotation_capability)
    -  [Function `offer_signer_capability`](#@Specification_1_offer_signer_capability)
    -  [Function `is_signer_capability_offered`](#@Specification_1_is_signer_capability_offered)
    -  [Function `get_signer_capability_offer_for`](#@Specification_1_get_signer_capability_offer_for)
    -  [Function `revoke_signer_capability`](#@Specification_1_revoke_signer_capability)
    -  [Function `revoke_any_signer_capability`](#@Specification_1_revoke_any_signer_capability)
    -  [Function `create_authorized_signer`](#@Specification_1_create_authorized_signer)
    -  [Function `assert_valid_rotation_proof_signature_and_get_auth_key`](#@Specification_1_assert_valid_rotation_proof_signature_and_get_auth_key)
    -  [Function `update_auth_key_and_originating_address_table`](#@Specification_1_update_auth_key_and_originating_address_table)
    -  [Function `create_resource_address`](#@Specification_1_create_resource_address)
    -  [Function `create_resource_account`](#@Specification_1_create_resource_account)
    -  [Function `create_framework_reserved_account`](#@Specification_1_create_framework_reserved_account)
    -  [Function `create_guid`](#@Specification_1_create_guid)
    -  [Function `new_event_handle`](#@Specification_1_new_event_handle)
    -  [Function `register_coin`](#@Specification_1_register_coin)
    -  [Function `create_signer_with_capability`](#@Specification_1_create_signer_with_capability)
    -  [Function `verify_signed_message`](#@Specification_1_verify_signed_message)


```move
module 0x1::account {
    use 0x1::bcs;
    use 0x1::chain_id;
    use 0x1::create_signer;
    use 0x1::ed25519;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::from_bcs;
    use 0x1::guid;
    use 0x1::hash;
    use 0x1::multi_ed25519;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::system_addresses;
    use 0x1::table;
    use 0x1::type_info;
    use 0x1::vector;
}
```


<a id="0x1_account_KeyRotation"></a>

## Struct `KeyRotation`



```move
module 0x1::account {
    #[event]
    struct KeyRotation has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`old_authentication_key: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_authentication_key: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_Account"></a>

## Resource `Account`

Resource representing an account.


```move
module 0x1::account {
    struct Account has store, key
}
```


##### Fields


<dl>
<dt>
`authentication_key: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`guid_creation_num: u64`
</dt>
<dd>

</dd>
<dt>
`coin_register_events: event::EventHandle<account::CoinRegisterEvent>`
</dt>
<dd>

</dd>
<dt>
`key_rotation_events: event::EventHandle<account::KeyRotationEvent>`
</dt>
<dd>

</dd>
<dt>
`rotation_capability_offer: account::CapabilityOffer<account::RotationCapability>`
</dt>
<dd>

</dd>
<dt>
`signer_capability_offer: account::CapabilityOffer<account::SignerCapability>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_KeyRotationEvent"></a>

## Struct `KeyRotationEvent`



```move
module 0x1::account {
    struct KeyRotationEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`old_authentication_key: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_authentication_key: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_CoinRegisterEvent"></a>

## Struct `CoinRegisterEvent`



```move
module 0x1::account {
    struct CoinRegisterEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`type_info: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_CapabilityOffer"></a>

## Struct `CapabilityOffer`



```move
module 0x1::account {
    struct CapabilityOffer<T> has store
}
```


##### Fields


<dl>
<dt>
`for: option::Option<address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_RotationCapability"></a>

## Struct `RotationCapability`



```move
module 0x1::account {
    struct RotationCapability has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_SignerCapability"></a>

## Struct `SignerCapability`



```move
module 0x1::account {
    struct SignerCapability has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_OriginatingAddress"></a>

## Resource `OriginatingAddress`

It is easy to fetch the authentication key of an address by simply reading it from the `Account` struct at that address.
The table in this struct makes it possible to do a reverse lookup: it maps an authentication key, to the address of the account which has that authentication key set.

This mapping is needed when recovering wallets for accounts whose authentication key has been rotated.

For example, imagine a freshly&#45;created wallet with address `a` and thus also with authentication key `a`, derived from a PK `pk_a` with corresponding SK `sk_a`.
It is easy to recover such a wallet given just the secret key `sk_a`, since the PK can be derived from the SK, the authentication key can then be derived from the PK, and the address equals the authentication key (since there was no key rotation).

However, if such a wallet rotates its authentication key to `b` derived from a different PK `pk_b` with SK `sk_b`, how would account recovery work?
The recovered address would no longer be &apos;a&apos;; it would be `b`, which is incorrect.
This struct solves this problem by mapping the new authentication key `b` to the original address `a` and thus helping the wallet software during recovery find the correct address.


```move
module 0x1::account {
    struct OriginatingAddress has key
}
```


##### Fields


<dl>
<dt>
`address_map: table::Table<address, address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_RotationProofChallenge"></a>

## Struct `RotationProofChallenge`

This structs stores the challenge message that should be signed during key rotation. First, this struct is
signed by the account owner&apos;s current public key, which proves possession of a capability to rotate the key.
Second, this struct is signed by the new public key that the account owner wants to rotate to, which proves
knowledge of this new public key&apos;s associated secret key. These two signatures cannot be replayed in another
context because they include the TXN&apos;s unique sequence number.


```move
module 0x1::account {
    struct RotationProofChallenge has copy, drop
}
```


##### Fields


<dl>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`originator: address`
</dt>
<dd>

</dd>
<dt>
`current_auth_key: address`
</dt>
<dd>

</dd>
<dt>
`new_public_key: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_RotationCapabilityOfferProofChallenge"></a>

## Struct `RotationCapabilityOfferProofChallenge`

Deprecated struct &#45; newest version is `RotationCapabilityOfferProofChallengeV2`


```move
module 0x1::account {
    struct RotationCapabilityOfferProofChallenge has drop
}
```


##### Fields


<dl>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`recipient_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_SignerCapabilityOfferProofChallenge"></a>

## Struct `SignerCapabilityOfferProofChallenge`

Deprecated struct &#45; newest version is `SignerCapabilityOfferProofChallengeV2`


```move
module 0x1::account {
    struct SignerCapabilityOfferProofChallenge has drop
}
```


##### Fields


<dl>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`recipient_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_RotationCapabilityOfferProofChallengeV2"></a>

## Struct `RotationCapabilityOfferProofChallengeV2`

This struct stores the challenge message that should be signed by the source account, when the source account
is delegating its rotation capability to the `recipient_address`.
This V2 struct adds the `chain_id` and `source_address` to the challenge message, which prevents replaying the challenge message.


```move
module 0x1::account {
    struct RotationCapabilityOfferProofChallengeV2 has drop
}
```


##### Fields


<dl>
<dt>
`chain_id: u8`
</dt>
<dd>

</dd>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`source_address: address`
</dt>
<dd>

</dd>
<dt>
`recipient_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_account_SignerCapabilityOfferProofChallengeV2"></a>

## Struct `SignerCapabilityOfferProofChallengeV2`



```move
module 0x1::account {
    struct SignerCapabilityOfferProofChallengeV2 has copy, drop
}
```


##### Fields


<dl>
<dt>
`sequence_number: u64`
</dt>
<dd>

</dd>
<dt>
`source_address: address`
</dt>
<dd>

</dd>
<dt>
`recipient_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_account_MAX_U64"></a>



```move
module 0x1::account {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_account_DERIVE_RESOURCE_ACCOUNT_SCHEME"></a>

Scheme identifier used when hashing an account&apos;s address together with a seed to derive the address (not the
authentication key) of a resource account. This is an abuse of the notion of a scheme identifier which, for now,
serves to domain separate hashes used to derive resource account addresses from hashes used to derive
authentication keys. Without such separation, an adversary could create (and get a signer for) a resource account
whose address matches an existing address of a MultiEd25519 wallet.


```move
module 0x1::account {
    const DERIVE_RESOURCE_ACCOUNT_SCHEME: u8 = 255;
}
```


<a id="0x1_account_EACCOUNT_ALREADY_EXISTS"></a>

Account already exists


```move
module 0x1::account {
    const EACCOUNT_ALREADY_EXISTS: u64 = 1;
}
```


<a id="0x1_account_EACCOUNT_ALREADY_USED"></a>

An attempt to create a resource account on an account that has a committed transaction


```move
module 0x1::account {
    const EACCOUNT_ALREADY_USED: u64 = 16;
}
```


<a id="0x1_account_EACCOUNT_DOES_NOT_EXIST"></a>

Account does not exist


```move
module 0x1::account {
    const EACCOUNT_DOES_NOT_EXIST: u64 = 2;
}
```


<a id="0x1_account_ECANNOT_RESERVED_ADDRESS"></a>

Cannot create account because address is reserved


```move
module 0x1::account {
    const ECANNOT_RESERVED_ADDRESS: u64 = 5;
}
```


<a id="0x1_account_ED25519_SCHEME"></a>

Scheme identifier for Ed25519 signatures used to derive authentication keys for Ed25519 public keys.


```move
module 0x1::account {
    const ED25519_SCHEME: u8 = 0;
}
```


<a id="0x1_account_EEXCEEDED_MAX_GUID_CREATION_NUM"></a>



```move
module 0x1::account {
    const EEXCEEDED_MAX_GUID_CREATION_NUM: u64 = 20;
}
```


<a id="0x1_account_EINVALID_ACCEPT_ROTATION_CAPABILITY"></a>

The caller does not have a valid rotation capability offer from the other account


```move
module 0x1::account {
    const EINVALID_ACCEPT_ROTATION_CAPABILITY: u64 = 10;
}
```


<a id="0x1_account_EINVALID_ORIGINATING_ADDRESS"></a>

Abort the transaction if the expected originating address is different from the originating address on&#45;chain


```move
module 0x1::account {
    const EINVALID_ORIGINATING_ADDRESS: u64 = 13;
}
```


<a id="0x1_account_EINVALID_PROOF_OF_KNOWLEDGE"></a>

Specified proof of knowledge required to prove ownership of a public key is invalid


```move
module 0x1::account {
    const EINVALID_PROOF_OF_KNOWLEDGE: u64 = 8;
}
```


<a id="0x1_account_EINVALID_SCHEME"></a>

Specified scheme required to proceed with the smart contract operation &#45; can only be ED25519_SCHEME(0) OR MULTI_ED25519_SCHEME(1)


```move
module 0x1::account {
    const EINVALID_SCHEME: u64 = 12;
}
```


<a id="0x1_account_EMALFORMED_AUTHENTICATION_KEY"></a>

The provided authentication key has an invalid length


```move
module 0x1::account {
    const EMALFORMED_AUTHENTICATION_KEY: u64 = 4;
}
```


<a id="0x1_account_ENO_CAPABILITY"></a>

The caller does not have a digital&#45;signature&#45;based capability to call this function


```move
module 0x1::account {
    const ENO_CAPABILITY: u64 = 9;
}
```


<a id="0x1_account_ENO_SIGNER_CAPABILITY_OFFERED"></a>



```move
module 0x1::account {
    const ENO_SIGNER_CAPABILITY_OFFERED: u64 = 19;
}
```


<a id="0x1_account_ENO_SUCH_ROTATION_CAPABILITY_OFFER"></a>

The specified rotation capablity offer does not exist at the specified offerer address


```move
module 0x1::account {
    const ENO_SUCH_ROTATION_CAPABILITY_OFFER: u64 = 18;
}
```


<a id="0x1_account_ENO_SUCH_SIGNER_CAPABILITY"></a>

The signer capability offer doesn&apos;t exist at the given address


```move
module 0x1::account {
    const ENO_SUCH_SIGNER_CAPABILITY: u64 = 14;
}
```


<a id="0x1_account_ENO_VALID_FRAMEWORK_RESERVED_ADDRESS"></a>

Address to create is not a valid reserved address for Aptos framework


```move
module 0x1::account {
    const ENO_VALID_FRAMEWORK_RESERVED_ADDRESS: u64 = 11;
}
```


<a id="0x1_account_EOFFERER_ADDRESS_DOES_NOT_EXIST"></a>

Offerer address doesn&apos;t exist


```move
module 0x1::account {
    const EOFFERER_ADDRESS_DOES_NOT_EXIST: u64 = 17;
}
```


<a id="0x1_account_EOUT_OF_GAS"></a>

Transaction exceeded its allocated max gas


```move
module 0x1::account {
    const EOUT_OF_GAS: u64 = 6;
}
```


<a id="0x1_account_ERESOURCE_ACCCOUNT_EXISTS"></a>

An attempt to create a resource account on a claimed account


```move
module 0x1::account {
    const ERESOURCE_ACCCOUNT_EXISTS: u64 = 15;
}
```


<a id="0x1_account_ESEQUENCE_NUMBER_TOO_BIG"></a>

Sequence number exceeds the maximum value for a u64


```move
module 0x1::account {
    const ESEQUENCE_NUMBER_TOO_BIG: u64 = 3;
}
```


<a id="0x1_account_EWRONG_CURRENT_PUBLIC_KEY"></a>

Specified current public key is not correct


```move
module 0x1::account {
    const EWRONG_CURRENT_PUBLIC_KEY: u64 = 7;
}
```


<a id="0x1_account_MAX_GUID_CREATION_NUM"></a>

Explicitly separate the GUID space between Object and Account to prevent accidental overlap.


```move
module 0x1::account {
    const MAX_GUID_CREATION_NUM: u64 = 1125899906842624;
}
```


<a id="0x1_account_MULTI_ED25519_SCHEME"></a>

Scheme identifier for MultiEd25519 signatures used to derive authentication keys for MultiEd25519 public keys.


```move
module 0x1::account {
    const MULTI_ED25519_SCHEME: u8 = 1;
}
```


<a id="0x1_account_ZERO_AUTH_KEY"></a>



```move
module 0x1::account {
    const ZERO_AUTH_KEY: vector<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
}
```


<a id="0x1_account_initialize"></a>

## Function `initialize`

Only called during genesis to initialize system resources for this module.


```move
module 0x1::account {
    public(friend) fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, OriginatingAddress {
            address_map: table::new(),
        });
    }
}
```


<a id="0x1_account_create_account_if_does_not_exist"></a>

## Function `create_account_if_does_not_exist`



```move
module 0x1::account {
    public fun create_account_if_does_not_exist(account_address: address)
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_account_if_does_not_exist(account_address: address) {
        if (!exists<Account>(account_address)) {
            create_account(account_address);
        }
    }
}
```


<a id="0x1_account_create_account"></a>

## Function `create_account`

Publishes a new `Account` resource under `new_address`. A signer representing `new_address`
is returned. This way, the caller of this function can publish additional resources under
`new_address`.


```move
module 0x1::account {
    public(friend) fun create_account(new_address: address): signer
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun create_account(new_address: address): signer {
        // there cannot be an Account resource under new_addr already.
        assert!(!exists<Account>(new_address), error::already_exists(EACCOUNT_ALREADY_EXISTS));

        // NOTE: @core_resources gets created via a `create_account` call, so we do not include it below.
        assert!(
            new_address != @vm_reserved && new_address != @aptos_framework && new_address != @aptos_token,
            error::invalid_argument(ECANNOT_RESERVED_ADDRESS)
        );

        create_account_unchecked(new_address)
    }
}
```


<a id="0x1_account_create_account_unchecked"></a>

## Function `create_account_unchecked`



```move
module 0x1::account {
    fun create_account_unchecked(new_address: address): signer
}
```


##### Implementation


```move
module 0x1::account {
    fun create_account_unchecked(new_address: address): signer {
        let new_account = create_signer(new_address);
        let authentication_key = bcs::to_bytes(&new_address);
        assert!(
            vector::length(&authentication_key) == 32,
            error::invalid_argument(EMALFORMED_AUTHENTICATION_KEY)
        );

        let guid_creation_num = 0;

        let guid_for_coin = guid::create(new_address, &mut guid_creation_num);
        let coin_register_events = event::new_event_handle<CoinRegisterEvent>(guid_for_coin);

        let guid_for_rotation = guid::create(new_address, &mut guid_creation_num);
        let key_rotation_events = event::new_event_handle<KeyRotationEvent>(guid_for_rotation);

        move_to(
            &new_account,
            Account {
                authentication_key,
                sequence_number: 0,
                guid_creation_num,
                coin_register_events,
                key_rotation_events,
                rotation_capability_offer: CapabilityOffer { for: option::none() },
                signer_capability_offer: CapabilityOffer { for: option::none() },
            }
        );

        new_account
    }
}
```


<a id="0x1_account_exists_at"></a>

## Function `exists_at`



```move
module 0x1::account {
    #[view]
    public fun exists_at(addr: address): bool
}
```


##### Implementation


```move
module 0x1::account {
    public fun exists_at(addr: address): bool {
        exists<Account>(addr)
    }
}
```


<a id="0x1_account_get_guid_next_creation_num"></a>

## Function `get_guid_next_creation_num`



```move
module 0x1::account {
    #[view]
    public fun get_guid_next_creation_num(addr: address): u64
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_guid_next_creation_num(addr: address): u64 acquires Account {
        borrow_global<Account>(addr).guid_creation_num
    }
}
```


<a id="0x1_account_get_sequence_number"></a>

## Function `get_sequence_number`



```move
module 0x1::account {
    #[view]
    public fun get_sequence_number(addr: address): u64
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_sequence_number(addr: address): u64 acquires Account {
        borrow_global<Account>(addr).sequence_number
    }
}
```


<a id="0x1_account_increment_sequence_number"></a>

## Function `increment_sequence_number`



```move
module 0x1::account {
    public(friend) fun increment_sequence_number(addr: address)
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun increment_sequence_number(addr: address) acquires Account {
        let sequence_number = &mut borrow_global_mut<Account>(addr).sequence_number;

        assert!(
            (*sequence_number as u128) < MAX_U64,
            error::out_of_range(ESEQUENCE_NUMBER_TOO_BIG)
        );

        *sequence_number = *sequence_number + 1;
    }
}
```


<a id="0x1_account_get_authentication_key"></a>

## Function `get_authentication_key`



```move
module 0x1::account {
    #[view]
    public fun get_authentication_key(addr: address): vector<u8>
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_authentication_key(addr: address): vector<u8> acquires Account {
        borrow_global<Account>(addr).authentication_key
    }
}
```


<a id="0x1_account_rotate_authentication_key_internal"></a>

## Function `rotate_authentication_key_internal`

This function is used to rotate a resource account&apos;s authentication key to `new_auth_key`. This is done in
many contexts:
1. During normal key rotation via `rotate_authentication_key` or `rotate_authentication_key_call`
2. During resource account initialization so that no private key can control the resource account
3. During multisig_v2 account creation


```move
module 0x1::account {
    public(friend) fun rotate_authentication_key_internal(account: &signer, new_auth_key: vector<u8>)
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun rotate_authentication_key_internal(account: &signer, new_auth_key: vector<u8>) acquires Account {
        let addr = signer::address_of(account);
        assert!(exists_at(addr), error::not_found(EACCOUNT_DOES_NOT_EXIST));
        assert!(
            vector::length(&new_auth_key) == 32,
            error::invalid_argument(EMALFORMED_AUTHENTICATION_KEY)
        );
        let account_resource = borrow_global_mut<Account>(addr);
        account_resource.authentication_key = new_auth_key;
    }
}
```


<a id="0x1_account_rotate_authentication_key_call"></a>

## Function `rotate_authentication_key_call`

Private entry function for key rotation that allows the signer to update their authentication key.
Note that this does not update the `OriginatingAddress` table because the `new_auth_key` is not &quot;verified&quot;: it
does not come with a proof&#45;of&#45;knowledge of the underlying SK. Nonetheless, we need this functionality due to
the introduction of non&#45;standard key algorithms, such as passkeys, which cannot produce proofs&#45;of&#45;knowledge in
the format expected in `rotate_authentication_key`.


```move
module 0x1::account {
    entry fun rotate_authentication_key_call(account: &signer, new_auth_key: vector<u8>)
}
```


##### Implementation


```move
module 0x1::account {
    entry fun rotate_authentication_key_call(account: &signer, new_auth_key: vector<u8>) acquires Account {
        rotate_authentication_key_internal(account, new_auth_key);
    }
}
```


<a id="0x1_account_rotate_authentication_key"></a>

## Function `rotate_authentication_key`

Generic authentication key rotation function that allows the user to rotate their authentication key from any scheme to any scheme.
To authorize the rotation, we need two signatures:
&#45; the first signature `cap_rotate_key` refers to the signature by the account owner&apos;s current key on a valid `RotationProofChallenge`,
demonstrating that the user intends to and has the capability to rotate the authentication key of this account;
&#45; the second signature `cap_update_table` refers to the signature by the new key (that the account owner wants to rotate to) on a
valid `RotationProofChallenge`, demonstrating that the user owns the new private key, and has the authority to update the
`OriginatingAddress` map with the new address mapping `<new_address, originating_address>`.
To verify these two signatures, we need their corresponding public key and public key scheme: we use `from_scheme` and `from_public_key_bytes`
to verify `cap_rotate_key`, and `to_scheme` and `to_public_key_bytes` to verify `cap_update_table`.
A scheme of 0 refers to an Ed25519 key and a scheme of 1 refers to Multi&#45;Ed25519 keys.
`originating address` refers to an account&apos;s original/first address.

Here is an example attack if we don&apos;t ask for the second signature `cap_update_table`:
Alice has rotated her account `addr_a` to `new_addr_a`. As a result, the following entry is created, to help Alice when recovering her wallet:
`OriginatingAddress[new_addr_a]` &#45;&gt; `addr_a`
Alice has had bad day: her laptop blew up and she needs to reset her account on a new one.
(Fortunately, she still has her secret key `new_sk_a` associated with her new address `new_addr_a`, so she can do this.)

But Bob likes to mess with Alice.
Bob creates an account `addr_b` and maliciously rotates it to Alice&apos;s new address `new_addr_a`. Since we are no longer checking a PoK,
Bob can easily do this.

Now, the table will be updated to make Alice&apos;s new address point to Bob&apos;s address: `OriginatingAddress[new_addr_a]` &#45;&gt; `addr_b`.
When Alice recovers her account, her wallet will display the attacker&apos;s address (Bob&apos;s) `addr_b` as her address.
Now Alice will give `addr_b` to everyone to pay her, but the money will go to Bob.

Because we ask for a valid `cap_update_table`, this kind of attack is not possible. Bob would not have the secret key of Alice&apos;s address
to rotate his address to Alice&apos;s address in the first place.


```move
module 0x1::account {
    public entry fun rotate_authentication_key(account: &signer, from_scheme: u8, from_public_key_bytes: vector<u8>, to_scheme: u8, to_public_key_bytes: vector<u8>, cap_rotate_key: vector<u8>, cap_update_table: vector<u8>)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun rotate_authentication_key(
        account: &signer,
        from_scheme: u8,
        from_public_key_bytes: vector<u8>,
        to_scheme: u8,
        to_public_key_bytes: vector<u8>,
        cap_rotate_key: vector<u8>,
        cap_update_table: vector<u8>,
    ) acquires Account, OriginatingAddress {
        let addr = signer::address_of(account);
        assert!(exists_at(addr), error::not_found(EACCOUNT_DOES_NOT_EXIST));
        let account_resource = borrow_global_mut<Account>(addr);

        // Verify the given `from_public_key_bytes` matches this account's current authentication key.
        if (from_scheme == ED25519_SCHEME) {
            let from_pk = ed25519::new_unvalidated_public_key_from_bytes(from_public_key_bytes);
            let from_auth_key = ed25519::unvalidated_public_key_to_authentication_key(&from_pk);
            assert!(
                account_resource.authentication_key == from_auth_key,
                error::unauthenticated(EWRONG_CURRENT_PUBLIC_KEY)
            );
        } else if (from_scheme == MULTI_ED25519_SCHEME) {
            let from_pk = multi_ed25519::new_unvalidated_public_key_from_bytes(from_public_key_bytes);
            let from_auth_key = multi_ed25519::unvalidated_public_key_to_authentication_key(&from_pk);
            assert!(
                account_resource.authentication_key == from_auth_key,
                error::unauthenticated(EWRONG_CURRENT_PUBLIC_KEY)
            );
        } else {
            abort error::invalid_argument(EINVALID_SCHEME)
        };

        // Construct a valid `RotationProofChallenge` that `cap_rotate_key` and `cap_update_table` will validate against.
        let curr_auth_key_as_address = from_bcs::to_address(account_resource.authentication_key);
        let challenge = RotationProofChallenge {
            sequence_number: account_resource.sequence_number,
            originator: addr,
            current_auth_key: curr_auth_key_as_address,
            new_public_key: to_public_key_bytes,
        };

        // Assert the challenges signed by the current and new keys are valid
        assert_valid_rotation_proof_signature_and_get_auth_key(
            from_scheme,
            from_public_key_bytes,
            cap_rotate_key,
            &challenge
        );
        let new_auth_key = assert_valid_rotation_proof_signature_and_get_auth_key(
            to_scheme,
            to_public_key_bytes,
            cap_update_table,
            &challenge
        );

        // Update the `OriginatingAddress` table.
        update_auth_key_and_originating_address_table(addr, account_resource, new_auth_key);
    }
}
```


<a id="0x1_account_rotate_authentication_key_with_rotation_capability"></a>

## Function `rotate_authentication_key_with_rotation_capability`



```move
module 0x1::account {
    public entry fun rotate_authentication_key_with_rotation_capability(delegate_signer: &signer, rotation_cap_offerer_address: address, new_scheme: u8, new_public_key_bytes: vector<u8>, cap_update_table: vector<u8>)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun rotate_authentication_key_with_rotation_capability(
        delegate_signer: &signer,
        rotation_cap_offerer_address: address,
        new_scheme: u8,
        new_public_key_bytes: vector<u8>,
        cap_update_table: vector<u8>
    ) acquires Account, OriginatingAddress {
        assert!(exists_at(rotation_cap_offerer_address), error::not_found(EOFFERER_ADDRESS_DOES_NOT_EXIST));

        // Check that there exists a rotation capability offer at the offerer's account resource for the delegate.
        let delegate_address = signer::address_of(delegate_signer);
        let offerer_account_resource = borrow_global<Account>(rotation_cap_offerer_address);
        assert!(
            option::contains(&offerer_account_resource.rotation_capability_offer.for, &delegate_address),
            error::not_found(ENO_SUCH_ROTATION_CAPABILITY_OFFER)
        );

        let curr_auth_key = from_bcs::to_address(offerer_account_resource.authentication_key);
        let challenge = RotationProofChallenge {
            sequence_number: get_sequence_number(delegate_address),
            originator: rotation_cap_offerer_address,
            current_auth_key: curr_auth_key,
            new_public_key: new_public_key_bytes,
        };

        // Verifies that the `RotationProofChallenge` from above is signed under the new public key that we are rotating to.        l
        let new_auth_key = assert_valid_rotation_proof_signature_and_get_auth_key(
            new_scheme,
            new_public_key_bytes,
            cap_update_table,
            &challenge
        );

        // Update the `OriginatingAddress` table, so we can find the originating address using the new address.
        let offerer_account_resource = borrow_global_mut<Account>(rotation_cap_offerer_address);
        update_auth_key_and_originating_address_table(
            rotation_cap_offerer_address,
            offerer_account_resource,
            new_auth_key
        );
    }
}
```


<a id="0x1_account_offer_rotation_capability"></a>

## Function `offer_rotation_capability`

Offers rotation capability on behalf of `account` to the account at address `recipient_address`.
An account can delegate its rotation capability to only one other address at one time. If the account
has an existing rotation capability offer, calling this function will update the rotation capability offer with
the new `recipient_address`.
Here, `rotation_capability_sig_bytes` signature indicates that this key rotation is authorized by the account owner,
and prevents the classic &quot;time&#45;of&#45;check time&#45;of&#45;use&quot; attack.
For example, users usually rely on what the wallet displays to them as the transaction&apos;s outcome. Consider a contract that with 50% probability
(based on the current timestamp in Move), rotates somebody&apos;s key. The wallet might be unlucky and get an outcome where nothing is rotated,
incorrectly telling the user nothing bad will happen. But when the transaction actually gets executed, the attacker gets lucky and
the execution path triggers the account key rotation.
We prevent such attacks by asking for this extra signature authorizing the key rotation.

@param rotation_capability_sig_bytes is the signature by the account owner&apos;s key on `RotationCapabilityOfferProofChallengeV2`.
@param account_scheme is the scheme of the account (ed25519 or multi_ed25519).
@param account_public_key_bytes is the public key of the account owner.
@param recipient_address is the address of the recipient of the rotation capability &#45; note that if there&apos;s an existing rotation capability
offer, calling this function will replace the previous `recipient_address` upon successful verification.


```move
module 0x1::account {
    public entry fun offer_rotation_capability(account: &signer, rotation_capability_sig_bytes: vector<u8>, account_scheme: u8, account_public_key_bytes: vector<u8>, recipient_address: address)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun offer_rotation_capability(
        account: &signer,
        rotation_capability_sig_bytes: vector<u8>,
        account_scheme: u8,
        account_public_key_bytes: vector<u8>,
        recipient_address: address,
    ) acquires Account {
        let addr = signer::address_of(account);
        assert!(exists_at(recipient_address), error::not_found(EACCOUNT_DOES_NOT_EXIST));

        // proof that this account intends to delegate its rotation capability to another account
        let account_resource = borrow_global_mut<Account>(addr);
        let proof_challenge = RotationCapabilityOfferProofChallengeV2 {
            chain_id: chain_id::get(),
            sequence_number: account_resource.sequence_number,
            source_address: addr,
            recipient_address,
        };

        // verify the signature on `RotationCapabilityOfferProofChallengeV2` by the account owner
        if (account_scheme == ED25519_SCHEME) {
            let pubkey = ed25519::new_unvalidated_public_key_from_bytes(account_public_key_bytes);
            let expected_auth_key = ed25519::unvalidated_public_key_to_authentication_key(&pubkey);
            assert!(
                account_resource.authentication_key == expected_auth_key,
                error::invalid_argument(EWRONG_CURRENT_PUBLIC_KEY)
            );

            let rotation_capability_sig = ed25519::new_signature_from_bytes(rotation_capability_sig_bytes);
            assert!(
                ed25519::signature_verify_strict_t(&rotation_capability_sig, &pubkey, proof_challenge),
                error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE)
            );
        } else if (account_scheme == MULTI_ED25519_SCHEME) {
            let pubkey = multi_ed25519::new_unvalidated_public_key_from_bytes(account_public_key_bytes);
            let expected_auth_key = multi_ed25519::unvalidated_public_key_to_authentication_key(&pubkey);
            assert!(
                account_resource.authentication_key == expected_auth_key,
                error::invalid_argument(EWRONG_CURRENT_PUBLIC_KEY)
            );

            let rotation_capability_sig = multi_ed25519::new_signature_from_bytes(rotation_capability_sig_bytes);
            assert!(
                multi_ed25519::signature_verify_strict_t(&rotation_capability_sig, &pubkey, proof_challenge),
                error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE)
            );
        } else {
            abort error::invalid_argument(EINVALID_SCHEME)
        };

        // update the existing rotation capability offer or put in a new rotation capability offer for the current account
        option::swap_or_fill(&mut account_resource.rotation_capability_offer.for, recipient_address);
    }
}
```


<a id="0x1_account_is_rotation_capability_offered"></a>

## Function `is_rotation_capability_offered`

Returns true if the account at `account_addr` has a rotation capability offer.


```move
module 0x1::account {
    #[view]
    public fun is_rotation_capability_offered(account_addr: address): bool
}
```


##### Implementation


```move
module 0x1::account {
    public fun is_rotation_capability_offered(account_addr: address): bool acquires Account {
        let account_resource = borrow_global<Account>(account_addr);
        option::is_some(&account_resource.rotation_capability_offer.for)
    }
}
```


<a id="0x1_account_get_rotation_capability_offer_for"></a>

## Function `get_rotation_capability_offer_for`

Returns the address of the account that has a rotation capability offer from the account at `account_addr`.


```move
module 0x1::account {
    #[view]
    public fun get_rotation_capability_offer_for(account_addr: address): address
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_rotation_capability_offer_for(account_addr: address): address acquires Account {
        let account_resource = borrow_global<Account>(account_addr);
        assert!(
            option::is_some(&account_resource.rotation_capability_offer.for),
            error::not_found(ENO_SIGNER_CAPABILITY_OFFERED),
        );
        *option::borrow(&account_resource.rotation_capability_offer.for)
    }
}
```


<a id="0x1_account_revoke_rotation_capability"></a>

## Function `revoke_rotation_capability`

Revoke the rotation capability offer given to `to_be_revoked_recipient_address` from `account`


```move
module 0x1::account {
    public entry fun revoke_rotation_capability(account: &signer, to_be_revoked_address: address)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun revoke_rotation_capability(account: &signer, to_be_revoked_address: address) acquires Account {
        assert!(exists_at(to_be_revoked_address), error::not_found(EACCOUNT_DOES_NOT_EXIST));
        let addr = signer::address_of(account);
        let account_resource = borrow_global_mut<Account>(addr);
        assert!(
            option::contains(&account_resource.rotation_capability_offer.for, &to_be_revoked_address),
            error::not_found(ENO_SUCH_ROTATION_CAPABILITY_OFFER)
        );
        revoke_any_rotation_capability(account);
    }
}
```


<a id="0x1_account_revoke_any_rotation_capability"></a>

## Function `revoke_any_rotation_capability`

Revoke any rotation capability offer in the specified account.


```move
module 0x1::account {
    public entry fun revoke_any_rotation_capability(account: &signer)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun revoke_any_rotation_capability(account: &signer) acquires Account {
        let account_resource = borrow_global_mut<Account>(signer::address_of(account));
        option::extract(&mut account_resource.rotation_capability_offer.for);
    }
}
```


<a id="0x1_account_offer_signer_capability"></a>

## Function `offer_signer_capability`

Offers signer capability on behalf of `account` to the account at address `recipient_address`.
An account can delegate its signer capability to only one other address at one time.
`signer_capability_key_bytes` is the `SignerCapabilityOfferProofChallengeV2` signed by the account owner&apos;s key
`account_scheme` is the scheme of the account (ed25519 or multi_ed25519).
`account_public_key_bytes` is the public key of the account owner.
`recipient_address` is the address of the recipient of the signer capability &#45; note that if there&apos;s an existing
`recipient_address` in the account owner&apos;s `SignerCapabilityOffer`, this will replace the
previous `recipient_address` upon successful verification (the previous recipient will no longer have access
to the account owner&apos;s signer capability).


```move
module 0x1::account {
    public entry fun offer_signer_capability(account: &signer, signer_capability_sig_bytes: vector<u8>, account_scheme: u8, account_public_key_bytes: vector<u8>, recipient_address: address)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun offer_signer_capability(
        account: &signer,
        signer_capability_sig_bytes: vector<u8>,
        account_scheme: u8,
        account_public_key_bytes: vector<u8>,
        recipient_address: address
    ) acquires Account {
        let source_address = signer::address_of(account);
        assert!(exists_at(recipient_address), error::not_found(EACCOUNT_DOES_NOT_EXIST));

        // Proof that this account intends to delegate its signer capability to another account.
        let proof_challenge = SignerCapabilityOfferProofChallengeV2 {
            sequence_number: get_sequence_number(source_address),
            source_address,
            recipient_address,
        };
        verify_signed_message(
            source_address, account_scheme, account_public_key_bytes, signer_capability_sig_bytes, proof_challenge);

        // Update the existing signer capability offer or put in a new signer capability offer for the recipient.
        let account_resource = borrow_global_mut<Account>(source_address);
        option::swap_or_fill(&mut account_resource.signer_capability_offer.for, recipient_address);
    }
}
```


<a id="0x1_account_is_signer_capability_offered"></a>

## Function `is_signer_capability_offered`

Returns true if the account at `account_addr` has a signer capability offer.


```move
module 0x1::account {
    #[view]
    public fun is_signer_capability_offered(account_addr: address): bool
}
```


##### Implementation


```move
module 0x1::account {
    public fun is_signer_capability_offered(account_addr: address): bool acquires Account {
        let account_resource = borrow_global<Account>(account_addr);
        option::is_some(&account_resource.signer_capability_offer.for)
    }
}
```


<a id="0x1_account_get_signer_capability_offer_for"></a>

## Function `get_signer_capability_offer_for`

Returns the address of the account that has a signer capability offer from the account at `account_addr`.


```move
module 0x1::account {
    #[view]
    public fun get_signer_capability_offer_for(account_addr: address): address
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_signer_capability_offer_for(account_addr: address): address acquires Account {
        let account_resource = borrow_global<Account>(account_addr);
        assert!(
            option::is_some(&account_resource.signer_capability_offer.for),
            error::not_found(ENO_SIGNER_CAPABILITY_OFFERED),
        );
        *option::borrow(&account_resource.signer_capability_offer.for)
    }
}
```


<a id="0x1_account_revoke_signer_capability"></a>

## Function `revoke_signer_capability`

Revoke the account owner&apos;s signer capability offer for `to_be_revoked_address` (i.e., the address that
has a signer capability offer from `account` but will be revoked in this function).


```move
module 0x1::account {
    public entry fun revoke_signer_capability(account: &signer, to_be_revoked_address: address)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun revoke_signer_capability(account: &signer, to_be_revoked_address: address) acquires Account {
        assert!(exists_at(to_be_revoked_address), error::not_found(EACCOUNT_DOES_NOT_EXIST));
        let addr = signer::address_of(account);
        let account_resource = borrow_global_mut<Account>(addr);
        assert!(
            option::contains(&account_resource.signer_capability_offer.for, &to_be_revoked_address),
            error::not_found(ENO_SUCH_SIGNER_CAPABILITY)
        );
        revoke_any_signer_capability(account);
    }
}
```


<a id="0x1_account_revoke_any_signer_capability"></a>

## Function `revoke_any_signer_capability`

Revoke any signer capability offer in the specified account.


```move
module 0x1::account {
    public entry fun revoke_any_signer_capability(account: &signer)
}
```


##### Implementation


```move
module 0x1::account {
    public entry fun revoke_any_signer_capability(account: &signer) acquires Account {
        let account_resource = borrow_global_mut<Account>(signer::address_of(account));
        option::extract(&mut account_resource.signer_capability_offer.for);
    }
}
```


<a id="0x1_account_create_authorized_signer"></a>

## Function `create_authorized_signer`

Return an authorized signer of the offerer, if there&apos;s an existing signer capability offer for `account`
at the offerer&apos;s address.


```move
module 0x1::account {
    public fun create_authorized_signer(account: &signer, offerer_address: address): signer
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_authorized_signer(account: &signer, offerer_address: address): signer acquires Account {
        assert!(exists_at(offerer_address), error::not_found(EOFFERER_ADDRESS_DOES_NOT_EXIST));

        // Check if there's an existing signer capability offer from the offerer.
        let account_resource = borrow_global<Account>(offerer_address);
        let addr = signer::address_of(account);
        assert!(
            option::contains(&account_resource.signer_capability_offer.for, &addr),
            error::not_found(ENO_SUCH_SIGNER_CAPABILITY)
        );

        create_signer(offerer_address)
    }
}
```


<a id="0x1_account_assert_valid_rotation_proof_signature_and_get_auth_key"></a>

## Function `assert_valid_rotation_proof_signature_and_get_auth_key`

Helper functions for authentication key rotation.


```move
module 0x1::account {
    fun assert_valid_rotation_proof_signature_and_get_auth_key(scheme: u8, public_key_bytes: vector<u8>, signature: vector<u8>, challenge: &account::RotationProofChallenge): vector<u8>
}
```


##### Implementation


```move
module 0x1::account {
    fun assert_valid_rotation_proof_signature_and_get_auth_key(
        scheme: u8,
        public_key_bytes: vector<u8>,
        signature: vector<u8>,
        challenge: &RotationProofChallenge
    ): vector<u8> {
        if (scheme == ED25519_SCHEME) {
            let pk = ed25519::new_unvalidated_public_key_from_bytes(public_key_bytes);
            let sig = ed25519::new_signature_from_bytes(signature);
            assert!(
                ed25519::signature_verify_strict_t(&sig, &pk, *challenge),
                std::error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE)
            );
            ed25519::unvalidated_public_key_to_authentication_key(&pk)
        } else if (scheme == MULTI_ED25519_SCHEME) {
            let pk = multi_ed25519::new_unvalidated_public_key_from_bytes(public_key_bytes);
            let sig = multi_ed25519::new_signature_from_bytes(signature);
            assert!(
                multi_ed25519::signature_verify_strict_t(&sig, &pk, *challenge),
                std::error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE)
            );
            multi_ed25519::unvalidated_public_key_to_authentication_key(&pk)
        } else {
            abort error::invalid_argument(EINVALID_SCHEME)
        }
    }
}
```


<a id="0x1_account_update_auth_key_and_originating_address_table"></a>

## Function `update_auth_key_and_originating_address_table`

Update the `OriginatingAddress` table, so that we can find the originating address using the latest address
in the event of key recovery.


```move
module 0x1::account {
    fun update_auth_key_and_originating_address_table(originating_addr: address, account_resource: &mut account::Account, new_auth_key_vector: vector<u8>)
}
```


##### Implementation


```move
module 0x1::account {
    fun update_auth_key_and_originating_address_table(
        originating_addr: address,
        account_resource: &mut Account,
        new_auth_key_vector: vector<u8>,
    ) acquires OriginatingAddress {
        let address_map = &mut borrow_global_mut<OriginatingAddress>(@aptos_framework).address_map;
        let curr_auth_key = from_bcs::to_address(account_resource.authentication_key);

        // Checks `OriginatingAddress[curr_auth_key]` is either unmapped, or mapped to `originating_address`.
        // If it's mapped to the originating address, removes that mapping.
        // Otherwise, abort if it's mapped to a different address.
        if (table::contains(address_map, curr_auth_key)) {
            // If account_a with address_a is rotating its keypair from keypair_a to keypair_b, we expect
            // the address of the account to stay the same, while its keypair updates to keypair_b.
            // Here, by asserting that we're calling from the account with the originating address, we enforce
            // the standard of keeping the same address and updating the keypair at the contract level.
            // Without this assertion, the dapps could also update the account's address to address_b (the address that
            // is programmatically related to keypaier_b) and update the keypair to keypair_b. This causes problems
            // for interoperability because different dapps can implement this in different ways.
            // If the account with address b calls this function with two valid signatures, it will abort at this step,
            // because address b is not the account's originating address.
            assert!(
                originating_addr == table::remove(address_map, curr_auth_key),
                error::not_found(EINVALID_ORIGINATING_ADDRESS)
            );
        };

        // Set `OriginatingAddress[new_auth_key] = originating_address`.
        let new_auth_key = from_bcs::to_address(new_auth_key_vector);
        table::add(address_map, new_auth_key, originating_addr);

        if (std::features::module_event_migration_enabled()) {
            event::emit(KeyRotation {
                account: originating_addr,
                old_authentication_key: account_resource.authentication_key,
                new_authentication_key: new_auth_key_vector,
            });
        };
        event::emit_event<KeyRotationEvent>(
            &mut account_resource.key_rotation_events,
            KeyRotationEvent {
                old_authentication_key: account_resource.authentication_key,
                new_authentication_key: new_auth_key_vector,
            }
        );

        // Update the account resource's authentication key.
        account_resource.authentication_key = new_auth_key_vector;
    }
}
```


<a id="0x1_account_create_resource_address"></a>

## Function `create_resource_address`

Basic account creation methods.
This is a helper function to compute resource addresses. Computation of the address
involves the use of a cryptographic hash operation and should be use thoughtfully.


```move
module 0x1::account {
    public fun create_resource_address(source: &address, seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_resource_address(source: &address, seed: vector<u8>): address {
        let bytes = bcs::to_bytes(source);
        vector::append(&mut bytes, seed);
        vector::push_back(&mut bytes, DERIVE_RESOURCE_ACCOUNT_SCHEME);
        from_bcs::to_address(hash::sha3_256(bytes))
    }
}
```


<a id="0x1_account_create_resource_account"></a>

## Function `create_resource_account`

A resource account is used to manage resources independent of an account managed by a user.
In Aptos a resource account is created based upon the sha3 256 of the source&apos;s address and additional seed data.
A resource account can only be created once, this is designated by setting the
`Account::signer_capability_offer::for` to the address of the resource account. While an entity may call
`create_account` to attempt to claim an account ahead of the creation of a resource account, if found Aptos will
transition ownership of the account over to the resource account. This is done by validating that the account has
yet to execute any transactions and that the `Account::signer_capability_offer::for` is none. The probability of a
collision where someone has legitimately produced a private key that maps to a resource account address is less
than `(1/2)^(256)`.


```move
module 0x1::account {
    public fun create_resource_account(source: &signer, seed: vector<u8>): (signer, account::SignerCapability)
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_resource_account(source: &signer, seed: vector<u8>): (signer, SignerCapability) acquires Account {
        let resource_addr = create_resource_address(&signer::address_of(source), seed);
        let resource = if (exists_at(resource_addr)) {
            let account = borrow_global<Account>(resource_addr);
            assert!(
                option::is_none(&account.signer_capability_offer.for),
                error::already_exists(ERESOURCE_ACCCOUNT_EXISTS),
            );
            assert!(
                account.sequence_number == 0,
                error::invalid_state(EACCOUNT_ALREADY_USED),
            );
            create_signer(resource_addr)
        } else {
            create_account_unchecked(resource_addr)
        };

        // By default, only the SignerCapability should have control over the resource account and not the auth key.
        // If the source account wants direct control via auth key, they would need to explicitly rotate the auth key
        // of the resource account using the SignerCapability.
        rotate_authentication_key_internal(&resource, ZERO_AUTH_KEY);

        let account = borrow_global_mut<Account>(resource_addr);
        account.signer_capability_offer.for = option::some(resource_addr);
        let signer_cap = SignerCapability { account: resource_addr };
        (resource, signer_cap)
    }
}
```


<a id="0x1_account_create_framework_reserved_account"></a>

## Function `create_framework_reserved_account`

create the account for system reserved addresses


```move
module 0x1::account {
    public(friend) fun create_framework_reserved_account(addr: address): (signer, account::SignerCapability)
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun create_framework_reserved_account(addr: address): (signer, SignerCapability) {
        assert!(
            addr == @0x1 ||
                addr == @0x2 ||
                addr == @0x3 ||
                addr == @0x4 ||
                addr == @0x5 ||
                addr == @0x6 ||
                addr == @0x7 ||
                addr == @0x8 ||
                addr == @0x9 ||
                addr == @0xa,
            error::permission_denied(ENO_VALID_FRAMEWORK_RESERVED_ADDRESS),
        );
        let signer = create_account_unchecked(addr);
        let signer_cap = SignerCapability { account: addr };
        (signer, signer_cap)
    }
}
```


<a id="0x1_account_create_guid"></a>

## Function `create_guid`

GUID management methods.


```move
module 0x1::account {
    public fun create_guid(account_signer: &signer): guid::GUID
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_guid(account_signer: &signer): guid::GUID acquires Account {
        let addr = signer::address_of(account_signer);
        let account = borrow_global_mut<Account>(addr);
        let guid = guid::create(addr, &mut account.guid_creation_num);
        assert!(
            account.guid_creation_num < MAX_GUID_CREATION_NUM,
            error::out_of_range(EEXCEEDED_MAX_GUID_CREATION_NUM),
        );
        guid
    }
}
```


<a id="0x1_account_new_event_handle"></a>

## Function `new_event_handle`

GUID management methods.


```move
module 0x1::account {
    public fun new_event_handle<T: drop, store>(account: &signer): event::EventHandle<T>
}
```


##### Implementation


```move
module 0x1::account {
    public fun new_event_handle<T: drop + store>(account: &signer): EventHandle<T> acquires Account {
        event::new_event_handle(create_guid(account))
    }
}
```


<a id="0x1_account_register_coin"></a>

## Function `register_coin`

Coin management methods.


```move
module 0x1::account {
    public(friend) fun register_coin<CoinType>(account_addr: address)
}
```


##### Implementation


```move
module 0x1::account {
    public(friend) fun register_coin<CoinType>(account_addr: address) acquires Account {
        let account = borrow_global_mut<Account>(account_addr);
        event::emit_event<CoinRegisterEvent>(
            &mut account.coin_register_events,
            CoinRegisterEvent {
                type_info: type_info::type_of<CoinType>(),
            },
        );
    }
}
```


<a id="0x1_account_create_signer_with_capability"></a>

## Function `create_signer_with_capability`

Capability based functions for efficient use.


```move
module 0x1::account {
    public fun create_signer_with_capability(capability: &account::SignerCapability): signer
}
```


##### Implementation


```move
module 0x1::account {
    public fun create_signer_with_capability(capability: &SignerCapability): signer {
        let addr = &capability.account;
        create_signer(*addr)
    }
}
```


<a id="0x1_account_get_signer_capability_address"></a>

## Function `get_signer_capability_address`



```move
module 0x1::account {
    public fun get_signer_capability_address(capability: &account::SignerCapability): address
}
```


##### Implementation


```move
module 0x1::account {
    public fun get_signer_capability_address(capability: &SignerCapability): address {
        capability.account
    }
}
```


<a id="0x1_account_verify_signed_message"></a>

## Function `verify_signed_message`



```move
module 0x1::account {
    public fun verify_signed_message<T: drop>(account: address, account_scheme: u8, account_public_key: vector<u8>, signed_message_bytes: vector<u8>, message: T)
}
```


##### Implementation


```move
module 0x1::account {
    public fun verify_signed_message<T: drop>(
        account: address,
        account_scheme: u8,
        account_public_key: vector<u8>,
        signed_message_bytes: vector<u8>,
        message: T,
    ) acquires Account {
        let account_resource = borrow_global_mut<Account>(account);
        // Verify that the `SignerCapabilityOfferProofChallengeV2` has the right information and is signed by the account owner's key
        if (account_scheme == ED25519_SCHEME) {
            let pubkey = ed25519::new_unvalidated_public_key_from_bytes(account_public_key);
            let expected_auth_key = ed25519::unvalidated_public_key_to_authentication_key(&pubkey);
            assert!(
                account_resource.authentication_key == expected_auth_key,
                error::invalid_argument(EWRONG_CURRENT_PUBLIC_KEY),
            );

            let signer_capability_sig = ed25519::new_signature_from_bytes(signed_message_bytes);
            assert!(
                ed25519::signature_verify_strict_t(&signer_capability_sig, &pubkey, message),
                error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE),
            );
        } else if (account_scheme == MULTI_ED25519_SCHEME) {
            let pubkey = multi_ed25519::new_unvalidated_public_key_from_bytes(account_public_key);
            let expected_auth_key = multi_ed25519::unvalidated_public_key_to_authentication_key(&pubkey);
            assert!(
                account_resource.authentication_key == expected_auth_key,
                error::invalid_argument(EWRONG_CURRENT_PUBLIC_KEY),
            );

            let signer_capability_sig = multi_ed25519::new_signature_from_bytes(signed_message_bytes);
            assert!(
                multi_ed25519::signature_verify_strict_t(&signer_capability_sig, &pubkey, message),
                error::invalid_argument(EINVALID_PROOF_OF_KNOWLEDGE),
            );
        } else {
            abort error::invalid_argument(EINVALID_SCHEME)
        };
    }
}
```


<a id="@Specification_1"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>The initialization of the account module should result in the proper system initialization with valid and consistent resources.</td>
<td>High</td>
<td>Initialization of the account module creates a valid address_map table and moves the resources to the OriginatingAddress under the aptos_framework account.</td>
<td>Audited that the address_map table is created and populated correctly with the expected initial values.</td>
</tr>

<tr>
<td>2</td>
<td>After successfully creating an account, the account resources should initialize with the default data, ensuring the proper initialization of the account state.</td>
<td>High</td>
<td>Creating an account via the create_account function validates the state and moves a new account resource under new_address.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](create_account).</td>
</tr>

<tr>
<td>3</td>
<td>Checking the existence of an account under a given address never results in an abort.</td>
<td>Low</td>
<td>The exists_at function returns a boolean value indicating the existence of an account under the given address.</td>
<td>Formally verified by the [#high&#45;level&#45;req&#45;3](aborts_if) condition.</td>
</tr>

<tr>
<td>4</td>
<td>The account module maintains bounded sequence numbers for all accounts, guaranteeing they remain within the specified limit.</td>
<td>Medium</td>
<td>The sequence number of an account may only increase up to MAX_U64 in a succeeding manner.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](increment_sequence_number) that it remains within the defined boundary of MAX_U64.</td>
</tr>

<tr>
<td>5</td>
<td>Only the ed25519 and multied25519 signature schemes are permissible.</td>
<td>Low</td>
<td>Exclusively perform key rotation using either the ed25519 or multied25519 signature schemes. Currently restricts the offering of rotation/signing capabilities to the ed25519 or multied25519 schemes.</td>
<td>Formally Verified: [#high&#45;level&#45;req&#45;5.1](rotate_authentication_key), [#high&#45;level&#45;req&#45;5.2](offer_rotation_capability), and [#high&#45;level&#45;req&#45;5.3](offer_signer_capability). Verified that it aborts if the account_scheme is not ED25519_SCHEME and not MULTI_ED25519_SCHEME. Audited that the scheme enums correspond correctly to signature logic.</td>
</tr>

<tr>
<td>6</td>
<td>Exclusively permit the rotation of the authentication key of an account for the account owner or any user who possesses rotation capabilities associated with that account.</td>
<td>Critical</td>
<td>In the rotate_authentication_key function, the authentication key derived from the from_public_key_bytes should match the signer&apos;s current authentication key. Only the delegate_signer granted the rotation capabilities may invoke the rotate_authentication_key_with_rotation_capability function.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;6.1](rotate_authentication_key) and [#high&#45;level&#45;req&#45;6.2](rotate_authentication_key_with_rotation_capability).</td>
</tr>

<tr>
<td>7</td>
<td>Only the owner of an account may offer or revoke the following capabilities: (1) offer_rotation_capability, (2) offer_signer_capability, (3) revoke_rotation_capability, and (4) revoke_signer_capability.</td>
<td>Critical</td>
<td>An account resource may only be modified by the owner of the account utilizing: rotation_capability_offer, signer_capability_offer.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;7.1](offer_rotation_capability), [#high&#45;level&#45;req&#45;7.2](offer_signer_capability), and [#high&#45;level&#45;req&#45;7.3](revoke_rotation_capability). and [#high&#45;level&#45;req&#45;7.4](revoke_signer_capability).</td>
</tr>

<tr>
<td>8</td>
<td>The capability to create a signer for the account is exclusively reserved for either the account owner or the account that has been granted the signing capabilities.</td>
<td>Critical</td>
<td>Signer creation for the account may only be successfully executed by explicitly granting the signing capabilities with the create_authorized_signer function.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;8](create_authorized_signer).</td>
</tr>

<tr>
<td>9</td>
<td>Rotating the authentication key requires two valid signatures. With the private key of the current authentication key. With the private key of the new authentication key.</td>
<td>Critical</td>
<td>The rotate_authentication_key verifies two signatures (current and new) before rotating to the new key. The first signature ensures the user has the intended capability, and the second signature ensures that the user owns the new key.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;9.1](rotate_authentication_key) and [#high&#45;level&#45;req&#45;9.2](rotate_authentication_key_with_rotation_capability).</td>
</tr>

<tr>
<td>10</td>
<td>The rotation of the authentication key updates the account&apos;s authentication key with the newly supplied one.</td>
<td>High</td>
<td>The auth_key may only update to the provided new_auth_key after verifying the signature.</td>
<td>Formally Verified in [#high&#45;level&#45;req&#45;10](rotate_authentication_key_internal) that the authentication key of an account is modified to the provided authentication key if the signature verification was successful.</td>
</tr>

<tr>
<td>11</td>
<td>The creation number is monotonically increasing.</td>
<td>Low</td>
<td>The guid_creation_num in the Account structure is monotonically increasing.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;11](guid_creation_num).</td>
</tr>

<tr>
<td>12</td>
<td>The Account resource is persistent.</td>
<td>Low</td>
<td>The Account structure assigned to the address should be persistent.</td>
<td>Audited that the Account structure is persistent.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::account {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::account {
    public(friend) fun initialize(aptos_framework: &signer)
}
```

Only the address `@aptos_framework` can call.
OriginatingAddress does not exist under `@aptos_framework` before the call.


```move
module 0x1::account {
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if exists<OriginatingAddress>(aptos_addr);
    ensures exists<OriginatingAddress>(aptos_addr);
}
```


<a id="@Specification_1_create_account_if_does_not_exist"></a>

### Function `create_account_if_does_not_exist`


```move
module 0x1::account {
    public fun create_account_if_does_not_exist(account_address: address)
}
```

Ensure that the account exists at the end of the call.


```move
module 0x1::account {
    let authentication_key = bcs::to_bytes(account_address);
    aborts_if !exists<Account>(account_address) && (
        account_address == @vm_reserved
        || account_address == @aptos_framework
        || account_address == @aptos_token
        || !(len(authentication_key) == 32)
    );
    ensures exists<Account>(account_address);
}
```


<a id="@Specification_1_create_account"></a>

### Function `create_account`


```move
module 0x1::account {
    public(friend) fun create_account(new_address: address): signer
}
```

Check if the bytes of the new address is 32.
The Account does not exist under the new address before creating the account.
Limit the new account address is not @vm_reserved / @aptos_framework / @aptos_toke.


```move
module 0x1::account {
    include CreateAccountAbortsIf {addr: new_address};
    aborts_if new_address == @vm_reserved || new_address == @aptos_framework || new_address == @aptos_token;
    ensures signer::address_of(result) == new_address;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    ensures exists<Account>(new_address);
}
```


<a id="@Specification_1_create_account_unchecked"></a>

### Function `create_account_unchecked`


```move
module 0x1::account {
    fun create_account_unchecked(new_address: address): signer
}
```

Check if the bytes of the new address is 32.
The Account does not exist under the new address before creating the account.


```move
module 0x1::account {
    include CreateAccountAbortsIf {addr: new_address};
    ensures signer::address_of(result) == new_address;
    ensures exists<Account>(new_address);
}
```


<a id="@Specification_1_exists_at"></a>

### Function `exists_at`


```move
module 0x1::account {
    #[view]
    public fun exists_at(addr: address): bool
}
```



```move
module 0x1::account {
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if false;
}
```



<a id="0x1_account_CreateAccountAbortsIf"></a>


```move
module 0x1::account {
    schema CreateAccountAbortsIf {
        addr: address;
        let authentication_key = bcs::to_bytes(addr);
        aborts_if len(authentication_key) != 32;
        aborts_if exists<Account>(addr);
        ensures len(authentication_key) == 32;
    }
}
```


<a id="@Specification_1_get_guid_next_creation_num"></a>

### Function `get_guid_next_creation_num`


```move
module 0x1::account {
    #[view]
    public fun get_guid_next_creation_num(addr: address): u64
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(addr);
    ensures result == global<Account>(addr).guid_creation_num;
}
```


<a id="@Specification_1_get_sequence_number"></a>

### Function `get_sequence_number`


```move
module 0x1::account {
    #[view]
    public fun get_sequence_number(addr: address): u64
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(addr);
    ensures result == global<Account>(addr).sequence_number;
}
```


<a id="@Specification_1_increment_sequence_number"></a>

### Function `increment_sequence_number`


```move
module 0x1::account {
    public(friend) fun increment_sequence_number(addr: address)
}
```

The Account existed under the address.
The sequence_number of the Account is up to MAX_U64.


```move
module 0x1::account {
    let sequence_number = global<Account>(addr).sequence_number;
    aborts_if !exists<Account>(addr);
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if sequence_number == MAX_U64;
    modifies global<Account>(addr);
    let post post_sequence_number = global<Account>(addr).sequence_number;
    ensures post_sequence_number == sequence_number + 1;
}
```


<a id="@Specification_1_get_authentication_key"></a>

### Function `get_authentication_key`


```move
module 0x1::account {
    #[view]
    public fun get_authentication_key(addr: address): vector<u8>
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(addr);
    ensures result == global<Account>(addr).authentication_key;
}
```


<a id="@Specification_1_rotate_authentication_key_internal"></a>

### Function `rotate_authentication_key_internal`


```move
module 0x1::account {
    public(friend) fun rotate_authentication_key_internal(account: &signer, new_auth_key: vector<u8>)
}
```

The Account existed under the signer before the call.
The length of new_auth_key is 32.


```move
module 0x1::account {
    let addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;10
[#high&#45;level&#45;req](high&#45;level requirement 10):
    let post account_resource = global<Account>(addr);
    aborts_if !exists<Account>(addr);
    aborts_if vector::length(new_auth_key) != 32;
    modifies global<Account>(addr);
    ensures account_resource.authentication_key == new_auth_key;
}
```


<a id="@Specification_1_rotate_authentication_key_call"></a>

### Function `rotate_authentication_key_call`


```move
module 0x1::account {
    entry fun rotate_authentication_key_call(account: &signer, new_auth_key: vector<u8>)
}
```



```move
module 0x1::account {
    let addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;10
[#high&#45;level&#45;req](high&#45;level requirement 10):
    let post account_resource = global<Account>(addr);
    aborts_if !exists<Account>(addr);
    aborts_if vector::length(new_auth_key) != 32;
    modifies global<Account>(addr);
    ensures account_resource.authentication_key == new_auth_key;
}
```



<a id="0x1_account_spec_assert_valid_rotation_proof_signature_and_get_auth_key"></a>


```move
module 0x1::account {
    fun spec_assert_valid_rotation_proof_signature_and_get_auth_key(scheme: u8, public_key_bytes: vector<u8>, signature: vector<u8>, challenge: RotationProofChallenge): vector<u8>;
}
```


<a id="@Specification_1_rotate_authentication_key"></a>

### Function `rotate_authentication_key`


```move
module 0x1::account {
    public entry fun rotate_authentication_key(account: &signer, from_scheme: u8, from_public_key_bytes: vector<u8>, to_scheme: u8, to_public_key_bytes: vector<u8>, cap_rotate_key: vector<u8>, cap_update_table: vector<u8>)
}
```

The Account existed under the signer
The authentication scheme is ED25519_SCHEME and MULTI_ED25519_SCHEME


```move
module 0x1::account {
    let addr = signer::address_of(account);
    let account_resource = global<Account>(addr);
    aborts_if !exists<Account>(addr);
// This enforces ### high&#45;level&#45;req&#45;6.1
[#high&#45;level&#45;req](high&#45;level requirement 6):
    include from_scheme == ED25519_SCHEME ==> ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: from_public_key_bytes };
    aborts_if from_scheme == ED25519_SCHEME && ({
        let expected_auth_key = ed25519::spec_public_key_bytes_to_authentication_key(from_public_key_bytes);
        account_resource.authentication_key != expected_auth_key
    });
    include from_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: from_public_key_bytes };
    aborts_if from_scheme == MULTI_ED25519_SCHEME && ({
        let from_auth_key = multi_ed25519::spec_public_key_bytes_to_authentication_key(from_public_key_bytes);
        account_resource.authentication_key != from_auth_key
    });
// This enforces ### high&#45;level&#45;req&#45;5.1
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if from_scheme != ED25519_SCHEME && from_scheme != MULTI_ED25519_SCHEME;
    let curr_auth_key = from_bcs::deserialize<address>(account_resource.authentication_key);
    aborts_if !from_bcs::deserializable<address>(account_resource.authentication_key);
    let challenge = RotationProofChallenge {
        sequence_number: account_resource.sequence_number,
        originator: addr,
        current_auth_key: curr_auth_key,
        new_public_key: to_public_key_bytes,
    };
// This enforces ### high&#45;level&#45;req&#45;9.1
[#high&#45;level&#45;req](high&#45;level requirement 9):
    include AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf {
        scheme: from_scheme,
        public_key_bytes: from_public_key_bytes,
        signature: cap_rotate_key,
        challenge,
    };
    include AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf {
        scheme: to_scheme,
        public_key_bytes: to_public_key_bytes,
        signature: cap_update_table,
        challenge,
    };
    let originating_addr = addr;
    let new_auth_key_vector = spec_assert_valid_rotation_proof_signature_and_get_auth_key(to_scheme, to_public_key_bytes, cap_update_table, challenge);
    let address_map = global<OriginatingAddress>(@aptos_framework).address_map;
    let new_auth_key = from_bcs::deserialize<address>(new_auth_key_vector);
    aborts_if !exists<OriginatingAddress>(@aptos_framework);
    aborts_if !from_bcs::deserializable<address>(account_resource.authentication_key);
    aborts_if table::spec_contains(address_map, curr_auth_key) &&
        table::spec_get(address_map, curr_auth_key) != originating_addr;
    aborts_if !from_bcs::deserializable<address>(new_auth_key_vector);
    aborts_if curr_auth_key != new_auth_key && table::spec_contains(address_map, new_auth_key);
    include UpdateAuthKeyAndOriginatingAddressTableAbortsIf {
        originating_addr: addr,
    };
    let post auth_key = global<Account>(addr).authentication_key;
    ensures auth_key == new_auth_key_vector;
}
```


<a id="@Specification_1_rotate_authentication_key_with_rotation_capability"></a>

### Function `rotate_authentication_key_with_rotation_capability`


```move
module 0x1::account {
    public entry fun rotate_authentication_key_with_rotation_capability(delegate_signer: &signer, rotation_cap_offerer_address: address, new_scheme: u8, new_public_key_bytes: vector<u8>, cap_update_table: vector<u8>)
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(rotation_cap_offerer_address);
    let delegate_address = signer::address_of(delegate_signer);
    let offerer_account_resource = global<Account>(rotation_cap_offerer_address);
    aborts_if !from_bcs::deserializable<address>(offerer_account_resource.authentication_key);
    let curr_auth_key = from_bcs::deserialize<address>(offerer_account_resource.authentication_key);
    aborts_if !exists<Account>(delegate_address);
    let challenge = RotationProofChallenge {
        sequence_number: global<Account>(delegate_address).sequence_number,
        originator: rotation_cap_offerer_address,
        current_auth_key: curr_auth_key,
        new_public_key: new_public_key_bytes,
    };
// This enforces ### high&#45;level&#45;req&#45;6.2
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !option::spec_contains(offerer_account_resource.rotation_capability_offer.for, delegate_address);
// This enforces ### high&#45;level&#45;req&#45;9.1
[#high&#45;level&#45;req](high&#45;level requirement 9):
    include AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf {
        scheme: new_scheme,
        public_key_bytes: new_public_key_bytes,
        signature: cap_update_table,
        challenge,
    };
    let new_auth_key_vector = spec_assert_valid_rotation_proof_signature_and_get_auth_key(new_scheme, new_public_key_bytes, cap_update_table, challenge);
    let address_map = global<OriginatingAddress>(@aptos_framework).address_map;
    aborts_if !exists<OriginatingAddress>(@aptos_framework);
    aborts_if !from_bcs::deserializable<address>(offerer_account_resource.authentication_key);
    aborts_if table::spec_contains(address_map, curr_auth_key) &&
        table::spec_get(address_map, curr_auth_key) != rotation_cap_offerer_address;
    aborts_if !from_bcs::deserializable<address>(new_auth_key_vector);
    let new_auth_key = from_bcs::deserialize<address>(new_auth_key_vector);
    aborts_if curr_auth_key != new_auth_key && table::spec_contains(address_map, new_auth_key);
    include UpdateAuthKeyAndOriginatingAddressTableAbortsIf {
        originating_addr: rotation_cap_offerer_address,
        account_resource: offerer_account_resource,
    };
    let post auth_key = global<Account>(rotation_cap_offerer_address).authentication_key;
    ensures auth_key == new_auth_key_vector;
}
```


<a id="@Specification_1_offer_rotation_capability"></a>

### Function `offer_rotation_capability`


```move
module 0x1::account {
    public entry fun offer_rotation_capability(account: &signer, rotation_capability_sig_bytes: vector<u8>, account_scheme: u8, account_public_key_bytes: vector<u8>, recipient_address: address)
}
```



```move
module 0x1::account {
    let source_address = signer::address_of(account);
    let account_resource = global<Account>(source_address);
    let proof_challenge = RotationCapabilityOfferProofChallengeV2 {
        chain_id: global<chain_id::ChainId>(@aptos_framework).id,
        sequence_number: account_resource.sequence_number,
        source_address,
        recipient_address,
    };
    aborts_if !exists<chain_id::ChainId>(@aptos_framework);
    aborts_if !exists<Account>(recipient_address);
    aborts_if !exists<Account>(source_address);
    include account_scheme == ED25519_SCHEME ==> ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key_bytes };
    aborts_if account_scheme == ED25519_SCHEME && ({
        let expected_auth_key = ed25519::spec_public_key_bytes_to_authentication_key(account_public_key_bytes);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == ED25519_SCHEME ==> ed25519::NewSignatureFromBytesAbortsIf { bytes: rotation_capability_sig_bytes };
    aborts_if account_scheme == ED25519_SCHEME && !ed25519::spec_signature_verify_strict_t(
        ed25519::Signature { bytes: rotation_capability_sig_bytes },
        ed25519::UnvalidatedPublicKey { bytes: account_public_key_bytes },
        proof_challenge
    );
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key_bytes };
    aborts_if account_scheme == MULTI_ED25519_SCHEME && ({
        let expected_auth_key = multi_ed25519::spec_public_key_bytes_to_authentication_key(account_public_key_bytes);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewSignatureFromBytesAbortsIf { bytes: rotation_capability_sig_bytes };
    aborts_if account_scheme == MULTI_ED25519_SCHEME && !multi_ed25519::spec_signature_verify_strict_t(
        multi_ed25519::Signature { bytes: rotation_capability_sig_bytes },
        multi_ed25519::UnvalidatedPublicKey { bytes: account_public_key_bytes },
        proof_challenge
    );
// This enforces ### high&#45;level&#45;req&#45;5.2
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if account_scheme != ED25519_SCHEME && account_scheme != MULTI_ED25519_SCHEME;
// This enforces ### high&#45;level&#45;req&#45;7.1
[#high&#45;level&#45;req](high&#45;level requirement 7):
    modifies global<Account>(source_address);
    let post offer_for = global<Account>(source_address).rotation_capability_offer.for;
    ensures option::spec_borrow(offer_for) == recipient_address;
}
```


<a id="@Specification_1_is_rotation_capability_offered"></a>

### Function `is_rotation_capability_offered`


```move
module 0x1::account {
    #[view]
    public fun is_rotation_capability_offered(account_addr: address): bool
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(account_addr);
}
```


<a id="@Specification_1_get_rotation_capability_offer_for"></a>

### Function `get_rotation_capability_offer_for`


```move
module 0x1::account {
    #[view]
    public fun get_rotation_capability_offer_for(account_addr: address): address
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(account_addr);
    let account_resource = global<Account>(account_addr);
    aborts_if len(account_resource.rotation_capability_offer.for.vec) == 0;
}
```


<a id="@Specification_1_revoke_rotation_capability"></a>

### Function `revoke_rotation_capability`


```move
module 0x1::account {
    public entry fun revoke_rotation_capability(account: &signer, to_be_revoked_address: address)
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(to_be_revoked_address);
    let addr = signer::address_of(account);
    let account_resource = global<Account>(addr);
    aborts_if !exists<Account>(addr);
    aborts_if !option::spec_contains(account_resource.rotation_capability_offer.for,to_be_revoked_address);
    modifies global<Account>(addr);
    ensures exists<Account>(to_be_revoked_address);
    let post offer_for = global<Account>(addr).rotation_capability_offer.for;
    ensures !option::spec_is_some(offer_for);
}
```


<a id="@Specification_1_revoke_any_rotation_capability"></a>

### Function `revoke_any_rotation_capability`


```move
module 0x1::account {
    public entry fun revoke_any_rotation_capability(account: &signer)
}
```



```move
module 0x1::account {
    let addr = signer::address_of(account);
    modifies global<Account>(addr);
    aborts_if !exists<Account>(addr);
    let account_resource = global<Account>(addr);
// This enforces ### high&#45;level&#45;req&#45;7.3
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if !option::is_some(account_resource.rotation_capability_offer.for);
    let post offer_for = global<Account>(addr).rotation_capability_offer.for;
    ensures !option::spec_is_some(offer_for);
}
```


<a id="@Specification_1_offer_signer_capability"></a>

### Function `offer_signer_capability`


```move
module 0x1::account {
    public entry fun offer_signer_capability(account: &signer, signer_capability_sig_bytes: vector<u8>, account_scheme: u8, account_public_key_bytes: vector<u8>, recipient_address: address)
}
```

The Account existed under the signer.
The authentication scheme is ED25519_SCHEME and MULTI_ED25519_SCHEME.


```move
module 0x1::account {
    let source_address = signer::address_of(account);
    let account_resource = global<Account>(source_address);
    let proof_challenge = SignerCapabilityOfferProofChallengeV2 {
        sequence_number: account_resource.sequence_number,
        source_address,
        recipient_address,
    };
    aborts_if !exists<Account>(recipient_address);
    aborts_if !exists<Account>(source_address);
    include account_scheme == ED25519_SCHEME ==> ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key_bytes };
    aborts_if account_scheme == ED25519_SCHEME && ({
        let expected_auth_key = ed25519::spec_public_key_bytes_to_authentication_key(account_public_key_bytes);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == ED25519_SCHEME ==> ed25519::NewSignatureFromBytesAbortsIf { bytes: signer_capability_sig_bytes };
    aborts_if account_scheme == ED25519_SCHEME && !ed25519::spec_signature_verify_strict_t(
        ed25519::Signature { bytes: signer_capability_sig_bytes },
        ed25519::UnvalidatedPublicKey { bytes: account_public_key_bytes },
        proof_challenge
    );
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key_bytes };
    aborts_if account_scheme == MULTI_ED25519_SCHEME && ({
        let expected_auth_key = multi_ed25519::spec_public_key_bytes_to_authentication_key(account_public_key_bytes);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewSignatureFromBytesAbortsIf { bytes: signer_capability_sig_bytes };
    aborts_if account_scheme == MULTI_ED25519_SCHEME && !multi_ed25519::spec_signature_verify_strict_t(
        multi_ed25519::Signature { bytes: signer_capability_sig_bytes },
        multi_ed25519::UnvalidatedPublicKey { bytes: account_public_key_bytes },
        proof_challenge
    );
// This enforces ### high&#45;level&#45;req&#45;5.3
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if account_scheme != ED25519_SCHEME && account_scheme != MULTI_ED25519_SCHEME;
// This enforces ### high&#45;level&#45;req&#45;7.2
[#high&#45;level&#45;req](high&#45;level requirement 7):
    modifies global<Account>(source_address);
    let post offer_for = global<Account>(source_address).signer_capability_offer.for;
    ensures option::spec_borrow(offer_for) == recipient_address;
}
```


<a id="@Specification_1_is_signer_capability_offered"></a>

### Function `is_signer_capability_offered`


```move
module 0x1::account {
    #[view]
    public fun is_signer_capability_offered(account_addr: address): bool
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(account_addr);
}
```


<a id="@Specification_1_get_signer_capability_offer_for"></a>

### Function `get_signer_capability_offer_for`


```move
module 0x1::account {
    #[view]
    public fun get_signer_capability_offer_for(account_addr: address): address
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(account_addr);
    let account_resource = global<Account>(account_addr);
    aborts_if len(account_resource.signer_capability_offer.for.vec) == 0;
}
```


<a id="@Specification_1_revoke_signer_capability"></a>

### Function `revoke_signer_capability`


```move
module 0x1::account {
    public entry fun revoke_signer_capability(account: &signer, to_be_revoked_address: address)
}
```

The Account existed under the signer.
The value of signer_capability_offer.for of Account resource under the signer is to_be_revoked_address.


```move
module 0x1::account {
    aborts_if !exists<Account>(to_be_revoked_address);
    let addr = signer::address_of(account);
    let account_resource = global<Account>(addr);
    aborts_if !exists<Account>(addr);
    aborts_if !option::spec_contains(account_resource.signer_capability_offer.for,to_be_revoked_address);
    modifies global<Account>(addr);
    ensures exists<Account>(to_be_revoked_address);
}
```


<a id="@Specification_1_revoke_any_signer_capability"></a>

### Function `revoke_any_signer_capability`


```move
module 0x1::account {
    public entry fun revoke_any_signer_capability(account: &signer)
}
```



```move
module 0x1::account {
    modifies global<Account>(signer::address_of(account));
// This enforces ### high&#45;level&#45;req&#45;7.4
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if !exists<Account>(signer::address_of(account));
    let account_resource = global<Account>(signer::address_of(account));
    aborts_if !option::is_some(account_resource.signer_capability_offer.for);
}
```


<a id="@Specification_1_create_authorized_signer"></a>

### Function `create_authorized_signer`


```move
module 0x1::account {
    public fun create_authorized_signer(account: &signer, offerer_address: address): signer
}
```

The Account existed under the signer.
The value of signer_capability_offer.for of Account resource under the signer is offerer_address.


```move
module 0x1::account {
// This enforces ### high&#45;level&#45;req&#45;8
[#high&#45;level&#45;req](high&#45;level requirement 8):
    include AccountContainsAddr{
        account,
        address: offerer_address,
    };
    modifies global<Account>(offerer_address);
    ensures exists<Account>(offerer_address);
    ensures signer::address_of(result) == offerer_address;
}
```



<a id="0x1_account_AccountContainsAddr"></a>


```move
module 0x1::account {
    schema AccountContainsAddr {
        account: signer;
        address: address;
        let addr = signer::address_of(account);
        let account_resource = global<Account>(address);
        aborts_if !exists<Account>(address);
    // This enforces ### high&#45;level&#45;spec&#45;3
    [create_signer.md#high&#45;level&#45;req](high&#45;level requirement 3) of the [create_signer.md](create_signer) module:
        aborts_if !option::spec_contains(account_resource.signer_capability_offer.for,addr);
    }
}
```


<a id="@Specification_1_assert_valid_rotation_proof_signature_and_get_auth_key"></a>

### Function `assert_valid_rotation_proof_signature_and_get_auth_key`


```move
module 0x1::account {
    fun assert_valid_rotation_proof_signature_and_get_auth_key(scheme: u8, public_key_bytes: vector<u8>, signature: vector<u8>, challenge: &account::RotationProofChallenge): vector<u8>
}
```



```move
module 0x1::account {
    pragma opaque;
    include AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf;
    ensures [abstract] result == spec_assert_valid_rotation_proof_signature_and_get_auth_key(scheme, public_key_bytes, signature, challenge);
}
```



<a id="0x1_account_AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf"></a>


```move
module 0x1::account {
    schema AssertValidRotationProofSignatureAndGetAuthKeyAbortsIf {
        scheme: u8;
        public_key_bytes: vector<u8>;
        signature: vector<u8>;
        challenge: RotationProofChallenge;
        include scheme == ED25519_SCHEME ==> ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: public_key_bytes };
        include scheme == ED25519_SCHEME ==> ed25519::NewSignatureFromBytesAbortsIf { bytes: signature };
        aborts_if scheme == ED25519_SCHEME && !ed25519::spec_signature_verify_strict_t(
            ed25519::Signature { bytes: signature },
            ed25519::UnvalidatedPublicKey { bytes: public_key_bytes },
            challenge
        );
        include scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: public_key_bytes };
        include scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewSignatureFromBytesAbortsIf { bytes: signature };
        aborts_if scheme == MULTI_ED25519_SCHEME && !multi_ed25519::spec_signature_verify_strict_t(
            multi_ed25519::Signature { bytes: signature },
            multi_ed25519::UnvalidatedPublicKey { bytes: public_key_bytes },
            challenge
        );
        aborts_if scheme != ED25519_SCHEME && scheme != MULTI_ED25519_SCHEME;
    }
}
```


<a id="@Specification_1_update_auth_key_and_originating_address_table"></a>

### Function `update_auth_key_and_originating_address_table`


```move
module 0x1::account {
    fun update_auth_key_and_originating_address_table(originating_addr: address, account_resource: &mut account::Account, new_auth_key_vector: vector<u8>)
}
```



```move
module 0x1::account {
    modifies global<OriginatingAddress>(@aptos_framework);
    include UpdateAuthKeyAndOriginatingAddressTableAbortsIf;
}
```



<a id="0x1_account_UpdateAuthKeyAndOriginatingAddressTableAbortsIf"></a>


```move
module 0x1::account {
    schema UpdateAuthKeyAndOriginatingAddressTableAbortsIf {
        originating_addr: address;
        account_resource: Account;
        new_auth_key_vector: vector<u8>;
        let address_map = global<OriginatingAddress>(@aptos_framework).address_map;
        let curr_auth_key = from_bcs::deserialize<address>(account_resource.authentication_key);
        let new_auth_key = from_bcs::deserialize<address>(new_auth_key_vector);
        aborts_if !exists<OriginatingAddress>(@aptos_framework);
        aborts_if !from_bcs::deserializable<address>(account_resource.authentication_key);
        aborts_if table::spec_contains(address_map, curr_auth_key) &&
            table::spec_get(address_map, curr_auth_key) != originating_addr;
        aborts_if !from_bcs::deserializable<address>(new_auth_key_vector);
        aborts_if curr_auth_key != new_auth_key && table::spec_contains(address_map, new_auth_key);
        ensures table::spec_contains(global<OriginatingAddress>(@aptos_framework).address_map, from_bcs::deserialize<address>(new_auth_key_vector));
    }
}
```


<a id="@Specification_1_create_resource_address"></a>

### Function `create_resource_address`


```move
module 0x1::account {
    public fun create_resource_address(source: &address, seed: vector<u8>): address
}
```

The Account existed under the signer
The value of signer_capability_offer.for of Account resource under the signer is to_be_revoked_address


```move
module 0x1::account {
    pragma opaque;
    pragma aborts_if_is_strict = false;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_create_resource_address(source, seed);
}
```



<a id="0x1_account_spec_create_resource_address"></a>


```move
module 0x1::account {
    fun spec_create_resource_address(source: address, seed: vector<u8>): address;
}
```


<a id="@Specification_1_create_resource_account"></a>

### Function `create_resource_account`


```move
module 0x1::account {
    public fun create_resource_account(source: &signer, seed: vector<u8>): (signer, account::SignerCapability)
}
```



```move
module 0x1::account {
    let source_addr = signer::address_of(source);
    let resource_addr = spec_create_resource_address(source_addr, seed);
    aborts_if len(ZERO_AUTH_KEY) != 32;
    include exists_at(resource_addr) ==> CreateResourceAccountAbortsIf;
    include !exists_at(resource_addr) ==> CreateAccountAbortsIf {addr: resource_addr};
    ensures signer::address_of(result_1) == resource_addr;
    let post offer_for = global<Account>(resource_addr).signer_capability_offer.for;
    ensures option::spec_borrow(offer_for) == resource_addr;
    ensures result_2 == SignerCapability { account: resource_addr };
}
```


<a id="@Specification_1_create_framework_reserved_account"></a>

### Function `create_framework_reserved_account`


```move
module 0x1::account {
    public(friend) fun create_framework_reserved_account(addr: address): (signer, account::SignerCapability)
}
```

Check if the bytes of the new address is 32.
The Account does not exist under the new address before creating the account.
The system reserved addresses is @0x1 / @0x2 / @0x3 / @0x4 / @0x5  / @0x6 / @0x7 / @0x8 / @0x9 / @0xa.


```move
module 0x1::account {
    aborts_if spec_is_framework_address(addr);
    include CreateAccountAbortsIf {addr};
    ensures signer::address_of(result_1) == addr;
    ensures result_2 == SignerCapability { account: addr };
}
```



<a id="0x1_account_spec_is_framework_address"></a>


```move
module 0x1::account {
    fun spec_is_framework_address(addr: address): bool{
       addr != @0x1 &&
       addr != @0x2 &&
       addr != @0x3 &&
       addr != @0x4 &&
       addr != @0x5 &&
       addr != @0x6 &&
       addr != @0x7 &&
       addr != @0x8 &&
       addr != @0x9 &&
       addr != @0xa
    }
}
```


<a id="@Specification_1_create_guid"></a>

### Function `create_guid`


```move
module 0x1::account {
    public fun create_guid(account_signer: &signer): guid::GUID
}
```

The Account existed under the signer.
The guid_creation_num of the ccount resource is up to MAX_U64.


```move
module 0x1::account {
    let addr = signer::address_of(account_signer);
    include NewEventHandleAbortsIf {
        account: account_signer,
    };
    modifies global<Account>(addr);
// This enforces ### high&#45;level&#45;req&#45;11
[#high&#45;level&#45;req](high&#45;level requirement 11):
    ensures global<Account>(addr).guid_creation_num == old(global<Account>(addr).guid_creation_num) + 1;
}
```


<a id="@Specification_1_new_event_handle"></a>

### Function `new_event_handle`


```move
module 0x1::account {
    public fun new_event_handle<T: drop, store>(account: &signer): event::EventHandle<T>
}
```

The Account existed under the signer.
The guid_creation_num of the Account is up to MAX_U64.


```move
module 0x1::account {
    include NewEventHandleAbortsIf;
}
```



<a id="0x1_account_NewEventHandleAbortsIf"></a>


```move
module 0x1::account {
    schema NewEventHandleAbortsIf {
        account: &signer;
        let addr = signer::address_of(account);
        let account = global<Account>(addr);
        aborts_if !exists<Account>(addr);
        aborts_if account.guid_creation_num + 1 > MAX_U64;
        aborts_if account.guid_creation_num + 1 >= MAX_GUID_CREATION_NUM;
    }
}
```


<a id="@Specification_1_register_coin"></a>

### Function `register_coin`


```move
module 0x1::account {
    public(friend) fun register_coin<CoinType>(account_addr: address)
}
```



```move
module 0x1::account {
    aborts_if !exists<Account>(account_addr);
    aborts_if !type_info::spec_is_struct<CoinType>();
    modifies global<Account>(account_addr);
}
```


<a id="@Specification_1_create_signer_with_capability"></a>

### Function `create_signer_with_capability`


```move
module 0x1::account {
    public fun create_signer_with_capability(capability: &account::SignerCapability): signer
}
```



```move
module 0x1::account {
    let addr = capability.account;
    ensures signer::address_of(result) == addr;
}
```



<a id="0x1_account_CreateResourceAccountAbortsIf"></a>


```move
module 0x1::account {
    schema CreateResourceAccountAbortsIf {
        resource_addr: address;
        let account = global<Account>(resource_addr);
        aborts_if len(account.signer_capability_offer.for.vec) != 0;
        aborts_if account.sequence_number != 0;
    }
}
```


<a id="@Specification_1_verify_signed_message"></a>

### Function `verify_signed_message`


```move
module 0x1::account {
    public fun verify_signed_message<T: drop>(account: address, account_scheme: u8, account_public_key: vector<u8>, signed_message_bytes: vector<u8>, message: T)
}
```



```move
module 0x1::account {
    pragma aborts_if_is_partial;
    modifies global<Account>(account);
    let account_resource = global<Account>(account);
    aborts_if !exists<Account>(account);
    include account_scheme == ED25519_SCHEME ==> ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key };
    aborts_if account_scheme == ED25519_SCHEME && ({
        let expected_auth_key = ed25519::spec_public_key_bytes_to_authentication_key(account_public_key);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewUnvalidatedPublicKeyFromBytesAbortsIf { bytes: account_public_key };
    aborts_if account_scheme == MULTI_ED25519_SCHEME && ({
        let expected_auth_key = multi_ed25519::spec_public_key_bytes_to_authentication_key(account_public_key);
        account_resource.authentication_key != expected_auth_key
    });
    include account_scheme == ED25519_SCHEME ==> ed25519::NewSignatureFromBytesAbortsIf { bytes: signed_message_bytes };
    include account_scheme == MULTI_ED25519_SCHEME ==> multi_ed25519::NewSignatureFromBytesAbortsIf { bytes: signed_message_bytes };
    aborts_if account_scheme != ED25519_SCHEME && account_scheme != MULTI_ED25519_SCHEME;
}
```
