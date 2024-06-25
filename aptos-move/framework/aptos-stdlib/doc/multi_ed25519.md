
<a id="0x1_multi_ed25519"></a>

# Module `0x1::multi_ed25519`

Exports MultiEd25519 multi&#45;signatures in Move.
This module has the exact same interface as the Ed25519 module.


-  [Struct `UnvalidatedPublicKey`](#0x1_multi_ed25519_UnvalidatedPublicKey)
-  [Struct `ValidatedPublicKey`](#0x1_multi_ed25519_ValidatedPublicKey)
-  [Struct `Signature`](#0x1_multi_ed25519_Signature)
-  [Constants](#@Constants_0)
-  [Function `new_unvalidated_public_key_from_bytes`](#0x1_multi_ed25519_new_unvalidated_public_key_from_bytes)
-  [Function `new_validated_public_key_from_bytes`](#0x1_multi_ed25519_new_validated_public_key_from_bytes)
-  [Function `new_validated_public_key_from_bytes_v2`](#0x1_multi_ed25519_new_validated_public_key_from_bytes_v2)
-  [Function `new_signature_from_bytes`](#0x1_multi_ed25519_new_signature_from_bytes)
-  [Function `public_key_to_unvalidated`](#0x1_multi_ed25519_public_key_to_unvalidated)
-  [Function `public_key_into_unvalidated`](#0x1_multi_ed25519_public_key_into_unvalidated)
-  [Function `unvalidated_public_key_to_bytes`](#0x1_multi_ed25519_unvalidated_public_key_to_bytes)
-  [Function `validated_public_key_to_bytes`](#0x1_multi_ed25519_validated_public_key_to_bytes)
-  [Function `signature_to_bytes`](#0x1_multi_ed25519_signature_to_bytes)
-  [Function `public_key_validate`](#0x1_multi_ed25519_public_key_validate)
-  [Function `public_key_validate_v2`](#0x1_multi_ed25519_public_key_validate_v2)
-  [Function `signature_verify_strict`](#0x1_multi_ed25519_signature_verify_strict)
-  [Function `signature_verify_strict_t`](#0x1_multi_ed25519_signature_verify_strict_t)
-  [Function `unvalidated_public_key_to_authentication_key`](#0x1_multi_ed25519_unvalidated_public_key_to_authentication_key)
-  [Function `unvalidated_public_key_num_sub_pks`](#0x1_multi_ed25519_unvalidated_public_key_num_sub_pks)
-  [Function `unvalidated_public_key_threshold`](#0x1_multi_ed25519_unvalidated_public_key_threshold)
-  [Function `validated_public_key_to_authentication_key`](#0x1_multi_ed25519_validated_public_key_to_authentication_key)
-  [Function `validated_public_key_num_sub_pks`](#0x1_multi_ed25519_validated_public_key_num_sub_pks)
-  [Function `validated_public_key_threshold`](#0x1_multi_ed25519_validated_public_key_threshold)
-  [Function `check_and_get_threshold`](#0x1_multi_ed25519_check_and_get_threshold)
-  [Function `public_key_bytes_to_authentication_key`](#0x1_multi_ed25519_public_key_bytes_to_authentication_key)
-  [Function `public_key_validate_internal`](#0x1_multi_ed25519_public_key_validate_internal)
-  [Function `public_key_validate_v2_internal`](#0x1_multi_ed25519_public_key_validate_v2_internal)
-  [Function `signature_verify_strict_internal`](#0x1_multi_ed25519_signature_verify_strict_internal)
-  [Specification](#@Specification_1)
    -  [Function `new_unvalidated_public_key_from_bytes`](#@Specification_1_new_unvalidated_public_key_from_bytes)
    -  [Function `new_validated_public_key_from_bytes`](#@Specification_1_new_validated_public_key_from_bytes)
    -  [Function `new_validated_public_key_from_bytes_v2`](#@Specification_1_new_validated_public_key_from_bytes_v2)
    -  [Function `new_signature_from_bytes`](#@Specification_1_new_signature_from_bytes)
    -  [Function `unvalidated_public_key_num_sub_pks`](#@Specification_1_unvalidated_public_key_num_sub_pks)
    -  [Function `unvalidated_public_key_threshold`](#@Specification_1_unvalidated_public_key_threshold)
    -  [Function `validated_public_key_num_sub_pks`](#@Specification_1_validated_public_key_num_sub_pks)
    -  [Function `validated_public_key_threshold`](#@Specification_1_validated_public_key_threshold)
    -  [Function `check_and_get_threshold`](#@Specification_1_check_and_get_threshold)
    -  [Function `public_key_bytes_to_authentication_key`](#@Specification_1_public_key_bytes_to_authentication_key)
    -  [Function `public_key_validate_internal`](#@Specification_1_public_key_validate_internal)
    -  [Function `public_key_validate_v2_internal`](#@Specification_1_public_key_validate_v2_internal)
    -  [Function `signature_verify_strict_internal`](#@Specification_1_signature_verify_strict_internal)
    -  [Helper functions](#@Helper_functions_2)


```move
module 0x1::multi_ed25519 {
    use 0x1::bcs;
    use 0x1::ed25519;
    use 0x1::error;
    use 0x1::features;
    use 0x1::hash;
    use 0x1::option;
}
```


<a id="0x1_multi_ed25519_UnvalidatedPublicKey"></a>

## Struct `UnvalidatedPublicKey`

An &#42;unvalidated&#42;, k out of n MultiEd25519 public key. The `bytes` field contains (1) several chunks of
`ed25519::PUBLIC_KEY_NUM_BYTES` bytes, each encoding a Ed25519 PK, and (2) a single byte encoding the threshold k.
&#42;Unvalidated&#42; means there is no guarantee that the underlying PKs are valid elliptic curve points of non&#45;small
order.


```move
module 0x1::multi_ed25519 {
    struct UnvalidatedPublicKey has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`bytes: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_multi_ed25519_ValidatedPublicKey"></a>

## Struct `ValidatedPublicKey`

A &#42;validated&#42; k out of n MultiEd25519 public key. &#42;Validated&#42; means that all the underlying PKs will be
elliptic curve points that are NOT of small&#45;order. It does not necessarily mean they will be prime&#45;order points.
This struct encodes the public key exactly as `UnvalidatedPublicKey`.

For now, this struct is not used in any verification functions, but it might be in the future.


```move
module 0x1::multi_ed25519 {
    struct ValidatedPublicKey has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`bytes: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_multi_ed25519_Signature"></a>

## Struct `Signature`

A purported MultiEd25519 multi&#45;signature that can be verified via `signature_verify_strict` or
`signature_verify_strict_t`. The `bytes` field contains (1) several chunks of `ed25519::SIGNATURE_NUM_BYTES`
bytes, each encoding a Ed25519 signature, and (2) a `BITMAP_NUM_OF_BYTES`&#45;byte bitmap encoding the signer
identities.


```move
module 0x1::multi_ed25519 {
    struct Signature has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`bytes: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_multi_ed25519_E_NATIVE_FUN_NOT_AVAILABLE"></a>

The native functions have not been rolled out yet.


```move
module 0x1::multi_ed25519 {
    const E_NATIVE_FUN_NOT_AVAILABLE: u64 = 4;
}
```


<a id="0x1_multi_ed25519_E_WRONG_PUBKEY_SIZE"></a>

Wrong number of bytes were given as input when deserializing an Ed25519 public key.


```move
module 0x1::multi_ed25519 {
    const E_WRONG_PUBKEY_SIZE: u64 = 1;
}
```


<a id="0x1_multi_ed25519_E_WRONG_SIGNATURE_SIZE"></a>

Wrong number of bytes were given as input when deserializing an Ed25519 signature.


```move
module 0x1::multi_ed25519 {
    const E_WRONG_SIGNATURE_SIZE: u64 = 2;
}
```


<a id="0x1_multi_ed25519_SIGNATURE_SCHEME_ID"></a>

The identifier of the MultiEd25519 signature scheme, which is used when deriving Aptos authentication keys by hashing
it together with an MultiEd25519 public key.


```move
module 0x1::multi_ed25519 {
    const SIGNATURE_SCHEME_ID: u8 = 1;
}
```


<a id="0x1_multi_ed25519_BITMAP_NUM_OF_BYTES"></a>

When serializing a MultiEd25519 signature, the bitmap that indicates the signers will be encoded using this many
bytes.


```move
module 0x1::multi_ed25519 {
    const BITMAP_NUM_OF_BYTES: u64 = 4;
}
```


<a id="0x1_multi_ed25519_E_INVALID_THRESHOLD_OR_NUMBER_OF_SIGNERS"></a>

The threshold must be in the range `[1, n]`, where n is the total number of signers.


```move
module 0x1::multi_ed25519 {
    const E_INVALID_THRESHOLD_OR_NUMBER_OF_SIGNERS: u64 = 3;
}
```


<a id="0x1_multi_ed25519_INDIVIDUAL_PUBLIC_KEY_NUM_BYTES"></a>

The size of an individual Ed25519 public key, in bytes.
(A MultiEd25519 public key consists of several of these, plus the threshold.)


```move
module 0x1::multi_ed25519 {
    const INDIVIDUAL_PUBLIC_KEY_NUM_BYTES: u64 = 32;
}
```


<a id="0x1_multi_ed25519_INDIVIDUAL_SIGNATURE_NUM_BYTES"></a>

The size of an individual Ed25519 signature, in bytes.
(A MultiEd25519 signature consists of several of these, plus the signer bitmap.)


```move
module 0x1::multi_ed25519 {
    const INDIVIDUAL_SIGNATURE_NUM_BYTES: u64 = 64;
}
```


<a id="0x1_multi_ed25519_MAX_NUMBER_OF_PUBLIC_KEYS"></a>

Max number of ed25519 public keys allowed in multi&#45;ed25519 keys


```move
module 0x1::multi_ed25519 {
    const MAX_NUMBER_OF_PUBLIC_KEYS: u64 = 32;
}
```


<a id="0x1_multi_ed25519_THRESHOLD_SIZE_BYTES"></a>

When serializing a MultiEd25519 public key, the threshold k will be encoded using this many bytes.


```move
module 0x1::multi_ed25519 {
    const THRESHOLD_SIZE_BYTES: u64 = 1;
}
```


<a id="0x1_multi_ed25519_new_unvalidated_public_key_from_bytes"></a>

## Function `new_unvalidated_public_key_from_bytes`

Parses the input 32 bytes as an &#42;unvalidated&#42; MultiEd25519 public key.

NOTE: This function could have also checked that the # of sub&#45;PKs is &gt; 0, but it did not. However, since such
invalid PKs are rejected during signature verification  (see `bugfix_unvalidated_pk_from_zero_subpks`) they
will not cause problems.

We could fix this API by adding a new one that checks the # of sub&#45;PKs is &gt; 0, but it is likely not a good idea
to reproduce the PK validation logic in Move. We should not have done so in the first place. Instead, we will
leave it as is and continue assuming `UnvalidatedPublicKey` objects could be invalid PKs that will safely be
rejected during signature verification.


```move
module 0x1::multi_ed25519 {
    public fun new_unvalidated_public_key_from_bytes(bytes: vector<u8>): multi_ed25519::UnvalidatedPublicKey
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun new_unvalidated_public_key_from_bytes(bytes: vector<u8>): UnvalidatedPublicKey {
        let len = vector::length(&bytes);
        let num_sub_pks = len / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;

        assert!(num_sub_pks <= MAX_NUMBER_OF_PUBLIC_KEYS, error::invalid_argument(E_WRONG_PUBKEY_SIZE));
        assert!(len % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES == THRESHOLD_SIZE_BYTES, error::invalid_argument(E_WRONG_PUBKEY_SIZE));
        UnvalidatedPublicKey { bytes }
    }
}
```


<a id="0x1_multi_ed25519_new_validated_public_key_from_bytes"></a>

## Function `new_validated_public_key_from_bytes`

DEPRECATED: Use `new_validated_public_key_from_bytes_v2` instead. See `public_key_validate_internal` comments.

(Incorrectly) parses the input bytes as a &#42;validated&#42; MultiEd25519 public key.


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes(bytes: vector<u8>): option::Option<multi_ed25519::ValidatedPublicKey>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes(bytes: vector<u8>): Option<ValidatedPublicKey> {
        // Note that `public_key_validate_internal` will check that `vector::length(&bytes) / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES <= MAX_NUMBER_OF_PUBLIC_KEYS`.
        if (vector::length(&bytes) % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES == THRESHOLD_SIZE_BYTES &&
            public_key_validate_internal(bytes)) {
            option::some(ValidatedPublicKey {
                bytes
            })
        } else {
            option::none<ValidatedPublicKey>()
        }
    }
}
```


<a id="0x1_multi_ed25519_new_validated_public_key_from_bytes_v2"></a>

## Function `new_validated_public_key_from_bytes_v2`

Parses the input bytes as a &#42;validated&#42; MultiEd25519 public key (see `public_key_validate_internal_v2`).


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes_v2(bytes: vector<u8>): option::Option<multi_ed25519::ValidatedPublicKey>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes_v2(bytes: vector<u8>): Option<ValidatedPublicKey> {
        if (!features::multi_ed25519_pk_validate_v2_enabled()) {
            abort(error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        if (public_key_validate_v2_internal(bytes)) {
            option::some(ValidatedPublicKey {
                bytes
            })
        } else {
            option::none<ValidatedPublicKey>()
        }
    }
}
```


<a id="0x1_multi_ed25519_new_signature_from_bytes"></a>

## Function `new_signature_from_bytes`

Parses the input bytes as a purported MultiEd25519 multi&#45;signature.


```move
module 0x1::multi_ed25519 {
    public fun new_signature_from_bytes(bytes: vector<u8>): multi_ed25519::Signature
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun new_signature_from_bytes(bytes: vector<u8>): Signature {
        assert!(vector::length(&bytes) % INDIVIDUAL_SIGNATURE_NUM_BYTES == BITMAP_NUM_OF_BYTES, error::invalid_argument(E_WRONG_SIGNATURE_SIZE));
        Signature { bytes }
    }
}
```


<a id="0x1_multi_ed25519_public_key_to_unvalidated"></a>

## Function `public_key_to_unvalidated`

Converts a ValidatedPublicKey to an UnvalidatedPublicKey, which can be used in the strict verification APIs.


```move
module 0x1::multi_ed25519 {
    public fun public_key_to_unvalidated(pk: &multi_ed25519::ValidatedPublicKey): multi_ed25519::UnvalidatedPublicKey
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun public_key_to_unvalidated(pk: &ValidatedPublicKey): UnvalidatedPublicKey {
        UnvalidatedPublicKey {
            bytes: pk.bytes
        }
    }
}
```


<a id="0x1_multi_ed25519_public_key_into_unvalidated"></a>

## Function `public_key_into_unvalidated`

Moves a ValidatedPublicKey into an UnvalidatedPublicKey, which can be used in the strict verification APIs.


```move
module 0x1::multi_ed25519 {
    public fun public_key_into_unvalidated(pk: multi_ed25519::ValidatedPublicKey): multi_ed25519::UnvalidatedPublicKey
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun public_key_into_unvalidated(pk: ValidatedPublicKey): UnvalidatedPublicKey {
        UnvalidatedPublicKey {
            bytes: pk.bytes
        }
    }
}
```


<a id="0x1_multi_ed25519_unvalidated_public_key_to_bytes"></a>

## Function `unvalidated_public_key_to_bytes`

Serializes an UnvalidatedPublicKey struct to 32&#45;bytes.


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_to_bytes(pk: &multi_ed25519::UnvalidatedPublicKey): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_to_bytes(pk: &UnvalidatedPublicKey): vector<u8> {
        pk.bytes
    }
}
```


<a id="0x1_multi_ed25519_validated_public_key_to_bytes"></a>

## Function `validated_public_key_to_bytes`

Serializes a ValidatedPublicKey struct to 32&#45;bytes.


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_to_bytes(pk: &multi_ed25519::ValidatedPublicKey): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_to_bytes(pk: &ValidatedPublicKey): vector<u8> {
        pk.bytes
    }
}
```


<a id="0x1_multi_ed25519_signature_to_bytes"></a>

## Function `signature_to_bytes`

Serializes a Signature struct to 64&#45;bytes.


```move
module 0x1::multi_ed25519 {
    public fun signature_to_bytes(sig: &multi_ed25519::Signature): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun signature_to_bytes(sig: &Signature): vector<u8> {
        sig.bytes
    }
}
```


<a id="0x1_multi_ed25519_public_key_validate"></a>

## Function `public_key_validate`

DEPRECATED: Use `public_key_validate_v2` instead. See `public_key_validate_internal` comments.

Takes in an &#42;unvalidated&#42; public key and attempts to validate it.
Returns `Some(ValidatedPublicKey)` if successful and `None` otherwise.


```move
module 0x1::multi_ed25519 {
    public fun public_key_validate(pk: &multi_ed25519::UnvalidatedPublicKey): option::Option<multi_ed25519::ValidatedPublicKey>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun public_key_validate(pk: &UnvalidatedPublicKey): Option<ValidatedPublicKey> {
        new_validated_public_key_from_bytes(pk.bytes)
    }
}
```


<a id="0x1_multi_ed25519_public_key_validate_v2"></a>

## Function `public_key_validate_v2`

Takes in an &#42;unvalidated&#42; public key and attempts to validate it.
Returns `Some(ValidatedPublicKey)` if successful and `None` otherwise.


```move
module 0x1::multi_ed25519 {
    public fun public_key_validate_v2(pk: &multi_ed25519::UnvalidatedPublicKey): option::Option<multi_ed25519::ValidatedPublicKey>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun public_key_validate_v2(pk: &UnvalidatedPublicKey): Option<ValidatedPublicKey> {
        new_validated_public_key_from_bytes_v2(pk.bytes)
    }
}
```


<a id="0x1_multi_ed25519_signature_verify_strict"></a>

## Function `signature_verify_strict`

Verifies a purported MultiEd25519 `multisignature` under an &#42;unvalidated&#42; `public_key` on the specified `message`.
This call will validate the public key by checking it is NOT in the small subgroup.


```move
module 0x1::multi_ed25519 {
    public fun signature_verify_strict(multisignature: &multi_ed25519::Signature, public_key: &multi_ed25519::UnvalidatedPublicKey, message: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun signature_verify_strict(
        multisignature: &Signature,
        public_key: &UnvalidatedPublicKey,
        message: vector<u8>
    ): bool {
        signature_verify_strict_internal(multisignature.bytes, public_key.bytes, message)
    }
}
```


<a id="0x1_multi_ed25519_signature_verify_strict_t"></a>

## Function `signature_verify_strict_t`

This function is used to verify a multi&#45;signature on any BCS&#45;serializable type T. For now, it is used to verify the
proof of private key ownership when rotating authentication keys.


```move
module 0x1::multi_ed25519 {
    public fun signature_verify_strict_t<T: drop>(multisignature: &multi_ed25519::Signature, public_key: &multi_ed25519::UnvalidatedPublicKey, data: T): bool
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun signature_verify_strict_t<T: drop>(multisignature: &Signature, public_key: &UnvalidatedPublicKey, data: T): bool {
        let encoded = ed25519::new_signed_message(data);

        signature_verify_strict_internal(multisignature.bytes, public_key.bytes, bcs::to_bytes(&encoded))
    }
}
```


<a id="0x1_multi_ed25519_unvalidated_public_key_to_authentication_key"></a>

## Function `unvalidated_public_key_to_authentication_key`

Derives the Aptos&#45;specific authentication key of the given Ed25519 public key.


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_to_authentication_key(pk: &multi_ed25519::UnvalidatedPublicKey): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_to_authentication_key(pk: &UnvalidatedPublicKey): vector<u8> {
        public_key_bytes_to_authentication_key(pk.bytes)
    }
}
```


<a id="0x1_multi_ed25519_unvalidated_public_key_num_sub_pks"></a>

## Function `unvalidated_public_key_num_sub_pks`

Returns the number n of sub&#45;PKs in an unvalidated t&#45;out&#45;of&#45;n MultiEd25519 PK.
If this `UnvalidatedPublicKey` would pass validation in `public_key_validate`, then the returned # of sub&#45;PKs
can be relied upon as correct.

We provide this API as a cheaper alternative to calling `public_key_validate` and then `validated_public_key_num_sub_pks`
when the input `pk` is known to be valid.


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_num_sub_pks(pk: &multi_ed25519::UnvalidatedPublicKey): u8
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_num_sub_pks(pk: &UnvalidatedPublicKey): u8 {
        let len = vector::length(&pk.bytes);

        ((len / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES) as u8)
    }
}
```


<a id="0x1_multi_ed25519_unvalidated_public_key_threshold"></a>

## Function `unvalidated_public_key_threshold`

Returns the number t of sub&#45;PKs in an unvalidated t&#45;out&#45;of&#45;n MultiEd25519 PK (i.e., the threshold) or `None`
if `bytes` does not correctly encode such a PK.


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_threshold(pk: &multi_ed25519::UnvalidatedPublicKey): option::Option<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_threshold(pk: &UnvalidatedPublicKey): Option<u8> {
        check_and_get_threshold(pk.bytes)
    }
}
```


<a id="0x1_multi_ed25519_validated_public_key_to_authentication_key"></a>

## Function `validated_public_key_to_authentication_key`

Derives the Aptos&#45;specific authentication key of the given Ed25519 public key.


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_to_authentication_key(pk: &multi_ed25519::ValidatedPublicKey): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_to_authentication_key(pk: &ValidatedPublicKey): vector<u8> {
        public_key_bytes_to_authentication_key(pk.bytes)
    }
}
```


<a id="0x1_multi_ed25519_validated_public_key_num_sub_pks"></a>

## Function `validated_public_key_num_sub_pks`

Returns the number n of sub&#45;PKs in a validated t&#45;out&#45;of&#45;n MultiEd25519 PK.
Since the format of this PK has been validated, the returned # of sub&#45;PKs is guaranteed to be correct.


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_num_sub_pks(pk: &multi_ed25519::ValidatedPublicKey): u8
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_num_sub_pks(pk: &ValidatedPublicKey): u8 {
        let len = vector::length(&pk.bytes);

        ((len / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES) as u8)
    }
}
```


<a id="0x1_multi_ed25519_validated_public_key_threshold"></a>

## Function `validated_public_key_threshold`

Returns the number t of sub&#45;PKs in a validated t&#45;out&#45;of&#45;n MultiEd25519 PK (i.e., the threshold).


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_threshold(pk: &multi_ed25519::ValidatedPublicKey): u8
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_threshold(pk: &ValidatedPublicKey): u8 {
        let len = vector::length(&pk.bytes);
        let threshold_byte = *vector::borrow(&pk.bytes, len - 1);

        threshold_byte
    }
}
```


<a id="0x1_multi_ed25519_check_and_get_threshold"></a>

## Function `check_and_get_threshold`

Checks that the serialized format of a t&#45;out&#45;of&#45;n MultiEd25519 PK correctly encodes 1 &lt;&#61; n &lt;&#61; 32 sub&#45;PKs.
(All `ValidatedPublicKey` objects are guaranteed to pass this check.)
Returns the threshold t &lt;&#61; n of the PK.


```move
module 0x1::multi_ed25519 {
    public fun check_and_get_threshold(bytes: vector<u8>): option::Option<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    public fun check_and_get_threshold(bytes: vector<u8>): Option<u8> {
        let len = vector::length(&bytes);
        if (len == 0) {
            return option::none<u8>()
        };

        let threshold_num_of_bytes = len % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;
        let num_of_keys = len / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;
        let threshold_byte = *vector::borrow(&bytes, len - 1);

        if (num_of_keys == 0 || num_of_keys > MAX_NUMBER_OF_PUBLIC_KEYS || threshold_num_of_bytes != 1) {
            return option::none<u8>()
        } else if (threshold_byte == 0 || threshold_byte > (num_of_keys as u8)) {
            return option::none<u8>()
        } else {
            return option::some(threshold_byte)
        }
    }
}
```


<a id="0x1_multi_ed25519_public_key_bytes_to_authentication_key"></a>

## Function `public_key_bytes_to_authentication_key`

Derives the Aptos&#45;specific authentication key of the given Ed25519 public key.


```move
module 0x1::multi_ed25519 {
    fun public_key_bytes_to_authentication_key(pk_bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    fun public_key_bytes_to_authentication_key(pk_bytes: vector<u8>): vector<u8> {
        vector::push_back(&mut pk_bytes, SIGNATURE_SCHEME_ID);
        std::hash::sha3_256(pk_bytes)
    }
}
```


<a id="0x1_multi_ed25519_public_key_validate_internal"></a>

## Function `public_key_validate_internal`

DEPRECATED: Use `public_key_validate_internal_v2` instead. This function was NOT correctly implemented:

1. It does not check that the # of sub public keys is &gt; 0, which leads to invalid `ValidatedPublicKey` objects
against which no signature will verify, since `signature_verify_strict_internal` will reject such invalid PKs.
This is not a security issue, but a correctness issue. See `bugfix_validated_pk_from_zero_subpks`.
2. It charges too much gas: if the first sub&#45;PK is invalid, it will charge for verifying all remaining sub&#45;PKs.

DEPRECATES:
&#45; new_validated_public_key_from_bytes
&#45; public_key_validate

Return `true` if the bytes in `public_key` can be parsed as a valid MultiEd25519 public key: i.e., all underlying
PKs pass point&#45;on&#45;curve and not&#45;in&#45;small&#45;subgroup checks.
Returns `false` otherwise.


```move
module 0x1::multi_ed25519 {
    fun public_key_validate_internal(bytes: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    native fun public_key_validate_internal(bytes: vector<u8>): bool;
}
```


<a id="0x1_multi_ed25519_public_key_validate_v2_internal"></a>

## Function `public_key_validate_v2_internal`

Return `true` if the bytes in `public_key` can be parsed as a valid MultiEd25519 public key: i.e., all underlying
sub&#45;PKs pass point&#45;on&#45;curve and not&#45;in&#45;small&#45;subgroup checks.
Returns `false` otherwise.


```move
module 0x1::multi_ed25519 {
    fun public_key_validate_v2_internal(bytes: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    native fun public_key_validate_v2_internal(bytes: vector<u8>): bool;
}
```


<a id="0x1_multi_ed25519_signature_verify_strict_internal"></a>

## Function `signature_verify_strict_internal`

Return true if the MultiEd25519 `multisignature` on `message` verifies against the MultiEd25519 `public_key`.
Returns `false` if either:
&#45; The PKs in `public_key` do not all pass points&#45;on&#45;curve or not&#45;in&#45;small&#45;subgroup checks,
&#45; The signatures in `multisignature` do not all pass points&#45;on&#45;curve or not&#45;in&#45;small&#45;subgroup checks,
&#45; the `multisignature` on `message` does not verify.


```move
module 0x1::multi_ed25519 {
    fun signature_verify_strict_internal(multisignature: vector<u8>, public_key: vector<u8>, message: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::multi_ed25519 {
    native fun signature_verify_strict_internal(
        multisignature: vector<u8>,
        public_key: vector<u8>,
        message: vector<u8>
    ): bool;
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_new_unvalidated_public_key_from_bytes"></a>

### Function `new_unvalidated_public_key_from_bytes`


```move
module 0x1::multi_ed25519 {
    public fun new_unvalidated_public_key_from_bytes(bytes: vector<u8>): multi_ed25519::UnvalidatedPublicKey
}
```



```move
module 0x1::multi_ed25519 {
    include NewUnvalidatedPublicKeyFromBytesAbortsIf;
    ensures result == UnvalidatedPublicKey { bytes };
}
```



<a id="0x1_multi_ed25519_NewUnvalidatedPublicKeyFromBytesAbortsIf"></a>


```move
module 0x1::multi_ed25519 {
    schema NewUnvalidatedPublicKeyFromBytesAbortsIf {
        bytes: vector<u8>;
        let length = len(bytes);
        aborts_if length / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES > MAX_NUMBER_OF_PUBLIC_KEYS;
        aborts_if length % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES != THRESHOLD_SIZE_BYTES;
    }
}
```


<a id="@Specification_1_new_validated_public_key_from_bytes"></a>

### Function `new_validated_public_key_from_bytes`


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes(bytes: vector<u8>): option::Option<multi_ed25519::ValidatedPublicKey>
}
```



```move
module 0x1::multi_ed25519 {
    aborts_if false;
    let cond = len(bytes) % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES == THRESHOLD_SIZE_BYTES
        && spec_public_key_validate_internal(bytes);
    ensures cond ==> result == option::spec_some(ValidatedPublicKey{bytes});
    ensures !cond ==> result == option::spec_none<ValidatedPublicKey>();
}
```


<a id="@Specification_1_new_validated_public_key_from_bytes_v2"></a>

### Function `new_validated_public_key_from_bytes_v2`


```move
module 0x1::multi_ed25519 {
    public fun new_validated_public_key_from_bytes_v2(bytes: vector<u8>): option::Option<multi_ed25519::ValidatedPublicKey>
}
```



```move
module 0x1::multi_ed25519 {
    let cond = spec_public_key_validate_v2_internal(bytes);
    ensures cond ==> result == option::spec_some(ValidatedPublicKey{bytes});
    ensures !cond ==> result == option::spec_none<ValidatedPublicKey>();
}
```


<a id="@Specification_1_new_signature_from_bytes"></a>

### Function `new_signature_from_bytes`


```move
module 0x1::multi_ed25519 {
    public fun new_signature_from_bytes(bytes: vector<u8>): multi_ed25519::Signature
}
```



```move
module 0x1::multi_ed25519 {
    include NewSignatureFromBytesAbortsIf;
    ensures result == Signature { bytes };
}
```



<a id="0x1_multi_ed25519_NewSignatureFromBytesAbortsIf"></a>


```move
module 0x1::multi_ed25519 {
    schema NewSignatureFromBytesAbortsIf {
        bytes: vector<u8>;
        aborts_if len(bytes) % INDIVIDUAL_SIGNATURE_NUM_BYTES != BITMAP_NUM_OF_BYTES;
    }
}
```


<a id="@Specification_1_unvalidated_public_key_num_sub_pks"></a>

### Function `unvalidated_public_key_num_sub_pks`


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_num_sub_pks(pk: &multi_ed25519::UnvalidatedPublicKey): u8
}
```



```move
module 0x1::multi_ed25519 {
    let bytes = pk.bytes;
    include PkDivision;
}
```


<a id="@Specification_1_unvalidated_public_key_threshold"></a>

### Function `unvalidated_public_key_threshold`


```move
module 0x1::multi_ed25519 {
    public fun unvalidated_public_key_threshold(pk: &multi_ed25519::UnvalidatedPublicKey): option::Option<u8>
}
```



```move
module 0x1::multi_ed25519 {
    aborts_if false;
    ensures result == spec_check_and_get_threshold(pk.bytes);
}
```


<a id="@Specification_1_validated_public_key_num_sub_pks"></a>

### Function `validated_public_key_num_sub_pks`


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_num_sub_pks(pk: &multi_ed25519::ValidatedPublicKey): u8
}
```



```move
module 0x1::multi_ed25519 {
    let bytes = pk.bytes;
    include PkDivision;
}
```


<a id="@Specification_1_validated_public_key_threshold"></a>

### Function `validated_public_key_threshold`


```move
module 0x1::multi_ed25519 {
    public fun validated_public_key_threshold(pk: &multi_ed25519::ValidatedPublicKey): u8
}
```



```move
module 0x1::multi_ed25519 {
    aborts_if len(pk.bytes) == 0;
    ensures result == pk.bytes[len(pk.bytes) - 1];
}
```


<a id="@Specification_1_check_and_get_threshold"></a>

### Function `check_and_get_threshold`


```move
module 0x1::multi_ed25519 {
    public fun check_and_get_threshold(bytes: vector<u8>): option::Option<u8>
}
```



```move
module 0x1::multi_ed25519 {
    aborts_if false;
    ensures result == spec_check_and_get_threshold(bytes);
}
```



<a id="0x1_multi_ed25519_PkDivision"></a>


```move
module 0x1::multi_ed25519 {
    schema PkDivision {
        bytes: vector<u8>;
        result: u8;
        aborts_if len(bytes) / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES > MAX_U8;
        ensures result == len(bytes) / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;
    }
}
```


<a id="@Specification_1_public_key_bytes_to_authentication_key"></a>

### Function `public_key_bytes_to_authentication_key`


```move
module 0x1::multi_ed25519 {
    fun public_key_bytes_to_authentication_key(pk_bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::multi_ed25519 {
    pragma opaque;
    aborts_if false;
    ensures [abstract] result == spec_public_key_bytes_to_authentication_key(pk_bytes);
}
```


<a id="@Specification_1_public_key_validate_internal"></a>

### Function `public_key_validate_internal`


```move
module 0x1::multi_ed25519 {
    fun public_key_validate_internal(bytes: vector<u8>): bool
}
```



```move
module 0x1::multi_ed25519 {
    pragma opaque;
    aborts_if false;
    ensures (len(bytes) / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES > MAX_NUMBER_OF_PUBLIC_KEYS) ==> (result == false);
    ensures result == spec_public_key_validate_internal(bytes);
}
```


<a id="@Specification_1_public_key_validate_v2_internal"></a>

### Function `public_key_validate_v2_internal`


```move
module 0x1::multi_ed25519 {
    fun public_key_validate_v2_internal(bytes: vector<u8>): bool
}
```



```move
module 0x1::multi_ed25519 {
    pragma opaque;
    ensures result == spec_public_key_validate_v2_internal(bytes);
}
```


<a id="@Specification_1_signature_verify_strict_internal"></a>

### Function `signature_verify_strict_internal`


```move
module 0x1::multi_ed25519 {
    fun signature_verify_strict_internal(multisignature: vector<u8>, public_key: vector<u8>, message: vector<u8>): bool
}
```



```move
module 0x1::multi_ed25519 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_signature_verify_strict_internal(multisignature, public_key, message);
}
```


<a id="@Helper_functions_2"></a>

### Helper functions



<a id="0x1_multi_ed25519_spec_check_and_get_threshold"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_check_and_get_threshold(bytes: vector<u8>): Option<u8> {
       let len = len(bytes);
       if (len == 0) {
           option::none<u8>()
       } else {
           let threshold_num_of_bytes = len % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;
           let num_of_keys = len / INDIVIDUAL_PUBLIC_KEY_NUM_BYTES;
           let threshold_byte = bytes[len - 1];
           if (num_of_keys == 0 || num_of_keys > MAX_NUMBER_OF_PUBLIC_KEYS || len % INDIVIDUAL_PUBLIC_KEY_NUM_BYTES != 1) {
               option::none<u8>()
           } else if (threshold_byte == 0 || threshold_byte > (num_of_keys as u8)) {
               option::none<u8>()
           } else {
               option::spec_some(threshold_byte)
           }
       }
    }
}
```



<a id="0x1_multi_ed25519_spec_signature_verify_strict_internal"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_signature_verify_strict_internal(
       multisignature: vector<u8>,
       public_key: vector<u8>,
       message: vector<u8>
    ): bool;
}
```



<a id="0x1_multi_ed25519_spec_public_key_validate_internal"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_public_key_validate_internal(bytes: vector<u8>): bool;
}
```



<a id="0x1_multi_ed25519_spec_public_key_validate_v2_internal"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_public_key_validate_v2_internal(bytes: vector<u8>): bool;
}
```



<a id="0x1_multi_ed25519_spec_public_key_bytes_to_authentication_key"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_public_key_bytes_to_authentication_key(pk_bytes: vector<u8>): vector<u8>;
}
```



<a id="0x1_multi_ed25519_spec_signature_verify_strict_t"></a>


```move
module 0x1::multi_ed25519 {
    fun spec_signature_verify_strict_t<T>(signature: Signature, public_key: UnvalidatedPublicKey, data: T): bool {
       let encoded = ed25519::new_signed_message<T>(data);
       let message = bcs::serialize(encoded);
       spec_signature_verify_strict_internal(signature.bytes, public_key.bytes, message)
    }
}
```
