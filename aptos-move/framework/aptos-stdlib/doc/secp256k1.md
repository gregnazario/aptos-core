
<a id="0x1_secp256k1"></a>

# Module `0x1::secp256k1`

This module implements ECDSA signatures based on the prime&#45;order secp256k1 ellptic curve (i.e., cofactor is 1).


-  [Struct `ECDSARawPublicKey`](#0x1_secp256k1_ECDSARawPublicKey)
-  [Struct `ECDSASignature`](#0x1_secp256k1_ECDSASignature)
-  [Constants](#@Constants_0)
-  [Function `ecdsa_signature_from_bytes`](#0x1_secp256k1_ecdsa_signature_from_bytes)
-  [Function `ecdsa_raw_public_key_from_64_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes)
-  [Function `ecdsa_raw_public_key_to_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_to_bytes)
-  [Function `ecdsa_signature_to_bytes`](#0x1_secp256k1_ecdsa_signature_to_bytes)
-  [Function `ecdsa_recover`](#0x1_secp256k1_ecdsa_recover)
-  [Function `ecdsa_recover_internal`](#0x1_secp256k1_ecdsa_recover_internal)
-  [Specification](#@Specification_1)
    -  [Function `ecdsa_signature_from_bytes`](#@Specification_1_ecdsa_signature_from_bytes)
    -  [Function `ecdsa_raw_public_key_from_64_bytes`](#@Specification_1_ecdsa_raw_public_key_from_64_bytes)
    -  [Function `ecdsa_raw_public_key_to_bytes`](#@Specification_1_ecdsa_raw_public_key_to_bytes)
    -  [Function `ecdsa_signature_to_bytes`](#@Specification_1_ecdsa_signature_to_bytes)
    -  [Function `ecdsa_recover`](#@Specification_1_ecdsa_recover)
    -  [Function `ecdsa_recover_internal`](#@Specification_1_ecdsa_recover_internal)


```move
module 0x1::secp256k1 {
    use 0x1::error;
    use 0x1::option;
}
```


<a id="0x1_secp256k1_ECDSARawPublicKey"></a>

## Struct `ECDSARawPublicKey`

A 64&#45;byte ECDSA public key.


```move
module 0x1::secp256k1 {
    struct ECDSARawPublicKey has copy, drop, store
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


<a id="0x1_secp256k1_ECDSASignature"></a>

## Struct `ECDSASignature`

A 64&#45;byte ECDSA signature.


```move
module 0x1::secp256k1 {
    struct ECDSASignature has copy, drop, store
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


<a id="0x1_secp256k1_SIGNATURE_NUM_BYTES"></a>

The size of a secp256k1&#45;based ECDSA signature, in bytes.


```move
module 0x1::secp256k1 {
    const SIGNATURE_NUM_BYTES: u64 = 64;
}
```


<a id="0x1_secp256k1_E_DESERIALIZE"></a>

An error occurred while deserializing, for example due to wrong input size.


```move
module 0x1::secp256k1 {
    const E_DESERIALIZE: u64 = 1;
}
```


<a id="0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES"></a>

The size of a secp256k1&#45;based ECDSA public key, in bytes.


```move
module 0x1::secp256k1 {
    const RAW_PUBLIC_KEY_NUM_BYTES: u64 = 64;
}
```


<a id="0x1_secp256k1_ecdsa_signature_from_bytes"></a>

## Function `ecdsa_signature_from_bytes`

Constructs an ECDSASignature struct from the given 64 bytes.


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_from_bytes(bytes: vector<u8>): secp256k1::ECDSASignature
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_from_bytes(bytes: vector<u8>): ECDSASignature {
        assert!(std::vector::length(&bytes) == SIGNATURE_NUM_BYTES, std::error::invalid_argument(E_DESERIALIZE));
        ECDSASignature { bytes }
    }
}
```


<a id="0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes"></a>

## Function `ecdsa_raw_public_key_from_64_bytes`

Constructs an ECDSARawPublicKey struct, given a 64&#45;byte raw representation.


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_from_64_bytes(bytes: vector<u8>): secp256k1::ECDSARawPublicKey
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_from_64_bytes(bytes: vector<u8>): ECDSARawPublicKey {
        assert!(std::vector::length(&bytes) == RAW_PUBLIC_KEY_NUM_BYTES, std::error::invalid_argument(E_DESERIALIZE));
        ECDSARawPublicKey { bytes }
    }
}
```


<a id="0x1_secp256k1_ecdsa_raw_public_key_to_bytes"></a>

## Function `ecdsa_raw_public_key_to_bytes`

Serializes an ECDSARawPublicKey struct to 64&#45;bytes.


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_to_bytes(pk: &secp256k1::ECDSARawPublicKey): vector<u8>
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_to_bytes(pk: &ECDSARawPublicKey): vector<u8> {
        pk.bytes
    }
}
```


<a id="0x1_secp256k1_ecdsa_signature_to_bytes"></a>

## Function `ecdsa_signature_to_bytes`

Serializes an ECDSASignature struct to 64&#45;bytes.


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_to_bytes(sig: &secp256k1::ECDSASignature): vector<u8>
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_to_bytes(sig: &ECDSASignature): vector<u8> {
        sig.bytes
    }
}
```


<a id="0x1_secp256k1_ecdsa_recover"></a>

## Function `ecdsa_recover`

Recovers the signer&apos;s raw (64&#45;byte) public key from a secp256k1 ECDSA `signature` given the `recovery_id` and the signed
`message` (32 byte digest).

Note that an invalid signature, or a signature from a different message, will result in the recovery of an
incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer&apos;s
public key (or its hash) is known beforehand.


```move
module 0x1::secp256k1 {
    public fun ecdsa_recover(message: vector<u8>, recovery_id: u8, signature: &secp256k1::ECDSASignature): option::Option<secp256k1::ECDSARawPublicKey>
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    public fun ecdsa_recover(
        message: vector<u8>,
        recovery_id: u8,
        signature: &ECDSASignature,
    ): Option<ECDSARawPublicKey> {
        let (pk, success) = ecdsa_recover_internal(message, recovery_id, signature.bytes);
        if (success) {
            std::option::some(ecdsa_raw_public_key_from_64_bytes(pk))
        } else {
            std::option::none<ECDSARawPublicKey>()
        }
    }
}
```


<a id="0x1_secp256k1_ecdsa_recover_internal"></a>

## Function `ecdsa_recover_internal`

Returns `(public_key, true)` if `signature` verifies on `message` under the recovered `public_key`
and returns `([], false)` otherwise.


```move
module 0x1::secp256k1 {
    fun ecdsa_recover_internal(message: vector<u8>, recovery_id: u8, signature: vector<u8>): (vector<u8>, bool)
}
```


##### Implementation


```move
module 0x1::secp256k1 {
    native fun ecdsa_recover_internal(
        message: vector<u8>,
        recovery_id: u8,
        signature: vector<u8>
    ): (vector<u8>, bool);
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_ecdsa_signature_from_bytes"></a>

### Function `ecdsa_signature_from_bytes`


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_from_bytes(bytes: vector<u8>): secp256k1::ECDSASignature
}
```



```move
module 0x1::secp256k1 {
    aborts_if len(bytes) != SIGNATURE_NUM_BYTES;
    ensures result == ECDSASignature { bytes };
}
```


<a id="@Specification_1_ecdsa_raw_public_key_from_64_bytes"></a>

### Function `ecdsa_raw_public_key_from_64_bytes`


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_from_64_bytes(bytes: vector<u8>): secp256k1::ECDSARawPublicKey
}
```



```move
module 0x1::secp256k1 {
    aborts_if len(bytes) != RAW_PUBLIC_KEY_NUM_BYTES;
    ensures result == ECDSARawPublicKey { bytes };
}
```


<a id="@Specification_1_ecdsa_raw_public_key_to_bytes"></a>

### Function `ecdsa_raw_public_key_to_bytes`


```move
module 0x1::secp256k1 {
    public fun ecdsa_raw_public_key_to_bytes(pk: &secp256k1::ECDSARawPublicKey): vector<u8>
}
```



```move
module 0x1::secp256k1 {
    aborts_if false;
    ensures result == pk.bytes;
}
```


<a id="@Specification_1_ecdsa_signature_to_bytes"></a>

### Function `ecdsa_signature_to_bytes`


```move
module 0x1::secp256k1 {
    public fun ecdsa_signature_to_bytes(sig: &secp256k1::ECDSASignature): vector<u8>
}
```



```move
module 0x1::secp256k1 {
    aborts_if false;
    ensures result == sig.bytes;
}
```


<a id="@Specification_1_ecdsa_recover"></a>

### Function `ecdsa_recover`


```move
module 0x1::secp256k1 {
    public fun ecdsa_recover(message: vector<u8>, recovery_id: u8, signature: &secp256k1::ECDSASignature): option::Option<secp256k1::ECDSARawPublicKey>
}
```



```move
module 0x1::secp256k1 {
    aborts_if ecdsa_recover_internal_abort_condition(message, recovery_id, signature.bytes);
    let pk = spec_ecdsa_recover_internal_result_1(message, recovery_id, signature.bytes);
    let success = spec_ecdsa_recover_internal_result_2(message, recovery_id, signature.bytes);
    ensures success ==> result == std::option::spec_some(ecdsa_raw_public_key_from_64_bytes(pk));
    ensures !success ==> result == std::option::spec_none<ECDSARawPublicKey>();
}
```


<a id="@Specification_1_ecdsa_recover_internal"></a>

### Function `ecdsa_recover_internal`


```move
module 0x1::secp256k1 {
    fun ecdsa_recover_internal(message: vector<u8>, recovery_id: u8, signature: vector<u8>): (vector<u8>, bool)
}
```



```move
module 0x1::secp256k1 {
    pragma opaque;
    aborts_if ecdsa_recover_internal_abort_condition(message, recovery_id, signature);
    ensures result_1 == spec_ecdsa_recover_internal_result_1(message, recovery_id, signature);
    ensures result_2 == spec_ecdsa_recover_internal_result_2(message, recovery_id, signature);
    ensures len(result_1) == if (result_2) { RAW_PUBLIC_KEY_NUM_BYTES } else { 0 };
}
```



<a id="0x1_secp256k1_ecdsa_recover_internal_abort_condition"></a>


```move
module 0x1::secp256k1 {
    fun ecdsa_recover_internal_abort_condition(message: vector<u8>, recovery_id: u8, signature: vector<u8>): bool;
}
```



<a id="0x1_secp256k1_spec_ecdsa_recover_internal_result_1"></a>


```move
module 0x1::secp256k1 {
    fun spec_ecdsa_recover_internal_result_1(message: vector<u8>, recovery_id: u8, signature: vector<u8>): vector<u8>;
}
```



<a id="0x1_secp256k1_spec_ecdsa_recover_internal_result_2"></a>


```move
module 0x1::secp256k1 {
    fun spec_ecdsa_recover_internal_result_2(message: vector<u8>, recovery_id: u8, signature: vector<u8>): bool;
}
```
