
<a id="0x1_aptos_hash"></a>

# Module `0x1::aptos_hash`

Cryptographic hashes:
&#45; Keccak&#45;256: see https://keccak.team/keccak.html

In addition, SHA2&#45;256 and SHA3&#45;256 are available in `std::hash`. Note that SHA3&#45;256 is a variant of Keccak: it is
NOT the same as Keccak&#45;256.

Non&#45;cryptograhic hashes:
&#45; SipHash: an add&#45;rotate&#45;xor (ARX) based family of pseudorandom functions created by Jean&#45;Philippe Aumasson and Daniel J. Bernstein in 2012


-  [Constants](#@Constants_0)
-  [Function `sip_hash`](#0x1_aptos_hash_sip_hash)
-  [Function `sip_hash_from_value`](#0x1_aptos_hash_sip_hash_from_value)
-  [Function `keccak256`](#0x1_aptos_hash_keccak256)
-  [Function `sha2_512`](#0x1_aptos_hash_sha2_512)
-  [Function `sha3_512`](#0x1_aptos_hash_sha3_512)
-  [Function `ripemd160`](#0x1_aptos_hash_ripemd160)
-  [Function `blake2b_256`](#0x1_aptos_hash_blake2b_256)
-  [Function `sha2_512_internal`](#0x1_aptos_hash_sha2_512_internal)
-  [Function `sha3_512_internal`](#0x1_aptos_hash_sha3_512_internal)
-  [Function `ripemd160_internal`](#0x1_aptos_hash_ripemd160_internal)
-  [Function `blake2b_256_internal`](#0x1_aptos_hash_blake2b_256_internal)
-  [Specification](#@Specification_1)
    -  [Function `sip_hash`](#@Specification_1_sip_hash)
    -  [Function `sip_hash_from_value`](#@Specification_1_sip_hash_from_value)
    -  [Function `keccak256`](#@Specification_1_keccak256)
    -  [Function `sha2_512`](#@Specification_1_sha2_512)
    -  [Function `sha3_512`](#@Specification_1_sha3_512)
    -  [Function `ripemd160`](#@Specification_1_ripemd160)
    -  [Function `blake2b_256`](#@Specification_1_blake2b_256)
    -  [Function `sha2_512_internal`](#@Specification_1_sha2_512_internal)
    -  [Function `sha3_512_internal`](#@Specification_1_sha3_512_internal)
    -  [Function `ripemd160_internal`](#@Specification_1_ripemd160_internal)
    -  [Function `blake2b_256_internal`](#@Specification_1_blake2b_256_internal)


```move
module 0x1::aptos_hash {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::features;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aptos_hash_E_NATIVE_FUN_NOT_AVAILABLE"></a>

A newly&#45;added native function is not yet enabled.


```move
module 0x1::aptos_hash {
    const E_NATIVE_FUN_NOT_AVAILABLE: u64 = 1;
}
```


<a id="0x1_aptos_hash_sip_hash"></a>

## Function `sip_hash`

Returns the (non&#45;cryptographic) SipHash of `bytes`. See https://en.wikipedia.org/wiki/SipHash


```move
module 0x1::aptos_hash {
    public fun sip_hash(bytes: vector<u8>): u64
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native public fun sip_hash(bytes: vector<u8>): u64;
}
```


<a id="0x1_aptos_hash_sip_hash_from_value"></a>

## Function `sip_hash_from_value`

Returns the (non&#45;cryptographic) SipHash of the BCS serialization of `v`. See https://en.wikipedia.org/wiki/SipHash


```move
module 0x1::aptos_hash {
    public fun sip_hash_from_value<MoveValue>(v: &MoveValue): u64
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    public fun sip_hash_from_value<MoveValue>(v: &MoveValue): u64 {
        let bytes = bcs::to_bytes(v);

        sip_hash(bytes)
    }
}
```


<a id="0x1_aptos_hash_keccak256"></a>

## Function `keccak256`

Returns the Keccak&#45;256 hash of `bytes`.


```move
module 0x1::aptos_hash {
    public fun keccak256(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native public fun keccak256(bytes: vector<u8>): vector<u8>;
}
```


<a id="0x1_aptos_hash_sha2_512"></a>

## Function `sha2_512`

Returns the SHA2&#45;512 hash of `bytes`.


```move
module 0x1::aptos_hash {
    public fun sha2_512(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    public fun sha2_512(bytes: vector<u8>): vector<u8> {
        if(!features::sha_512_and_ripemd_160_enabled()) {
            abort(std::error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        sha2_512_internal(bytes)
    }
}
```


<a id="0x1_aptos_hash_sha3_512"></a>

## Function `sha3_512`

Returns the SHA3&#45;512 hash of `bytes`.


```move
module 0x1::aptos_hash {
    public fun sha3_512(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    public fun sha3_512(bytes: vector<u8>): vector<u8> {
        if(!features::sha_512_and_ripemd_160_enabled()) {
            abort(std::error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        sha3_512_internal(bytes)
    }
}
```


<a id="0x1_aptos_hash_ripemd160"></a>

## Function `ripemd160`

Returns the RIPEMD&#45;160 hash of `bytes`.

WARNING: Only 80&#45;bit security is provided by this function. This means an adversary who can compute roughly 2^80
hashes will, with high probability, find a collision x_1 !&#61; x_2 such that RIPEMD&#45;160(x_1) &#61; RIPEMD&#45;160(x_2).


```move
module 0x1::aptos_hash {
    public fun ripemd160(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    public fun ripemd160(bytes: vector<u8>): vector<u8> {
        if(!features::sha_512_and_ripemd_160_enabled()) {
            abort(std::error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        ripemd160_internal(bytes)
    }
}
```


<a id="0x1_aptos_hash_blake2b_256"></a>

## Function `blake2b_256`

Returns the BLAKE2B&#45;256 hash of `bytes`.


```move
module 0x1::aptos_hash {
    public fun blake2b_256(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    public fun blake2b_256(bytes: vector<u8>): vector<u8> {
        if(!features::blake2b_256_enabled()) {
            abort(std::error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        blake2b_256_internal(bytes)
    }
}
```


<a id="0x1_aptos_hash_sha2_512_internal"></a>

## Function `sha2_512_internal`

Returns the SHA2&#45;512 hash of `bytes`.


```move
module 0x1::aptos_hash {
    fun sha2_512_internal(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native fun sha2_512_internal(bytes: vector<u8>): vector<u8>;
}
```


<a id="0x1_aptos_hash_sha3_512_internal"></a>

## Function `sha3_512_internal`

Returns the SHA3&#45;512 hash of `bytes`.


```move
module 0x1::aptos_hash {
    fun sha3_512_internal(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native fun sha3_512_internal(bytes: vector<u8>): vector<u8>;
}
```


<a id="0x1_aptos_hash_ripemd160_internal"></a>

## Function `ripemd160_internal`

Returns the RIPEMD&#45;160 hash of `bytes`.

WARNING: Only 80&#45;bit security is provided by this function. This means an adversary who can compute roughly 2^80
hashes will, with high probability, find a collision x_1 !&#61; x_2 such that RIPEMD&#45;160(x_1) &#61; RIPEMD&#45;160(x_2).


```move
module 0x1::aptos_hash {
    fun ripemd160_internal(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native fun ripemd160_internal(bytes: vector<u8>): vector<u8>;
}
```


<a id="0x1_aptos_hash_blake2b_256_internal"></a>

## Function `blake2b_256_internal`

Returns the BLAKE2B&#45;256 hash of `bytes`.


```move
module 0x1::aptos_hash {
    fun blake2b_256_internal(bytes: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::aptos_hash {
    native fun blake2b_256_internal(bytes: vector<u8>): vector<u8>;
}
```


<a id="@Specification_1"></a>

## Specification


`spec_sip_hash` is not assumed to be injective.


<a id="0x1_aptos_hash_spec_sip_hash"></a>


```move
module 0x1::aptos_hash {
    fun spec_sip_hash(bytes: vector<u8>): u64;
}
```

`spec_keccak256` is an injective function.


<a id="0x1_aptos_hash_spec_keccak256"></a>


```move
module 0x1::aptos_hash {
    fun spec_keccak256(bytes: vector<u8>): vector<u8>;
    axiom forall b1: vector<u8>, b2: vector<u8>:
        (spec_keccak256(b1) == spec_keccak256(b2) ==> b1 == b2);
}
```

`spec_sha2_512_internal` is an injective function.


<a id="0x1_aptos_hash_spec_sha2_512_internal"></a>


```move
module 0x1::aptos_hash {
    fun spec_sha2_512_internal(bytes: vector<u8>): vector<u8>;
    axiom forall b1: vector<u8>, b2: vector<u8>:
        (spec_sha2_512_internal(b1) == spec_sha2_512_internal(b2) ==> b1 == b2);
}
```

`spec_sha3_512_internal` is an injective function.


<a id="0x1_aptos_hash_spec_sha3_512_internal"></a>


```move
module 0x1::aptos_hash {
    fun spec_sha3_512_internal(bytes: vector<u8>): vector<u8>;
    axiom forall b1: vector<u8>, b2: vector<u8>:
        (spec_sha3_512_internal(b1) == spec_sha3_512_internal(b2) ==> b1 == b2);
}
```

`spec_ripemd160_internal` is an injective function.


<a id="0x1_aptos_hash_spec_ripemd160_internal"></a>


```move
module 0x1::aptos_hash {
    fun spec_ripemd160_internal(bytes: vector<u8>): vector<u8>;
    axiom forall b1: vector<u8>, b2: vector<u8>:
        (spec_ripemd160_internal(b1) == spec_ripemd160_internal(b2) ==> b1 == b2);
}
```

`spec_blake2b_256_internal` is an injective function.


<a id="0x1_aptos_hash_spec_blake2b_256_internal"></a>


```move
module 0x1::aptos_hash {
    fun spec_blake2b_256_internal(bytes: vector<u8>): vector<u8>;
    axiom forall b1: vector<u8>, b2: vector<u8>:
        (spec_blake2b_256_internal(b1) == spec_blake2b_256_internal(b2) ==> b1 == b2);
}
```


<a id="@Specification_1_sip_hash"></a>

### Function `sip_hash`


```move
module 0x1::aptos_hash {
    public fun sip_hash(bytes: vector<u8>): u64
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_sip_hash(bytes);
}
```


<a id="@Specification_1_sip_hash_from_value"></a>

### Function `sip_hash_from_value`


```move
module 0x1::aptos_hash {
    public fun sip_hash_from_value<MoveValue>(v: &MoveValue): u64
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    ensures result == spec_sip_hash(bcs::serialize(v));
}
```


<a id="@Specification_1_keccak256"></a>

### Function `keccak256`


```move
module 0x1::aptos_hash {
    public fun keccak256(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_keccak256(bytes);
}
```


<a id="@Specification_1_sha2_512"></a>

### Function `sha2_512`


```move
module 0x1::aptos_hash {
    public fun sha2_512(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if !features::spec_is_enabled(features::SHA_512_AND_RIPEMD_160_NATIVES);
    ensures result == spec_sha2_512_internal(bytes);
}
```


<a id="@Specification_1_sha3_512"></a>

### Function `sha3_512`


```move
module 0x1::aptos_hash {
    public fun sha3_512(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if !features::spec_is_enabled(features::SHA_512_AND_RIPEMD_160_NATIVES);
    ensures result == spec_sha3_512_internal(bytes);
}
```


<a id="@Specification_1_ripemd160"></a>

### Function `ripemd160`


```move
module 0x1::aptos_hash {
    public fun ripemd160(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if !features::spec_is_enabled(features::SHA_512_AND_RIPEMD_160_NATIVES);
    ensures result == spec_ripemd160_internal(bytes);
}
```


<a id="@Specification_1_blake2b_256"></a>

### Function `blake2b_256`


```move
module 0x1::aptos_hash {
    public fun blake2b_256(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if !features::spec_is_enabled(features::BLAKE2B_256_NATIVE);
    ensures result == spec_blake2b_256_internal(bytes);
}
```


<a id="@Specification_1_sha2_512_internal"></a>

### Function `sha2_512_internal`


```move
module 0x1::aptos_hash {
    fun sha2_512_internal(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_sha2_512_internal(bytes);
}
```


<a id="@Specification_1_sha3_512_internal"></a>

### Function `sha3_512_internal`


```move
module 0x1::aptos_hash {
    fun sha3_512_internal(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_sha3_512_internal(bytes);
}
```


<a id="@Specification_1_ripemd160_internal"></a>

### Function `ripemd160_internal`


```move
module 0x1::aptos_hash {
    fun ripemd160_internal(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_ripemd160_internal(bytes);
}
```


<a id="@Specification_1_blake2b_256_internal"></a>

### Function `blake2b_256_internal`


```move
module 0x1::aptos_hash {
    fun blake2b_256_internal(bytes: vector<u8>): vector<u8>
}
```



```move
module 0x1::aptos_hash {
    pragma opaque;
    aborts_if false;
    ensures result == spec_blake2b_256_internal(bytes);
}
```
