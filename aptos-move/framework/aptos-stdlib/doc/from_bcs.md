
<a id="0x1_from_bcs"></a>

# Module `0x1::from_bcs`

This module provides a number of functions to convert _primitive_ types from their representation in `std::bcs`
to values. This is the opposite of `bcs::to_bytes`. Note that it is not safe to define a generic public `from_bytes`
function because this can violate implicit struct invariants, therefore only primitive types are offerred. If
a general conversion back&#45;and&#45;force is needed, consider the `aptos_std::Any` type which preserves invariants.

Example:
```
use std::bcs;
use aptos_std::from_bcs;

assert!(from_bcs::to_address(bcs::to_bytes(&amp;@0xabcdef)) &#61;&#61; @0xabcdef, 0);
```


-  [Constants](#@Constants_0)
-  [Function `to_bool`](#0x1_from_bcs_to_bool)
-  [Function `to_u8`](#0x1_from_bcs_to_u8)
-  [Function `to_u16`](#0x1_from_bcs_to_u16)
-  [Function `to_u32`](#0x1_from_bcs_to_u32)
-  [Function `to_u64`](#0x1_from_bcs_to_u64)
-  [Function `to_u128`](#0x1_from_bcs_to_u128)
-  [Function `to_u256`](#0x1_from_bcs_to_u256)
-  [Function `to_address`](#0x1_from_bcs_to_address)
-  [Function `to_bytes`](#0x1_from_bcs_to_bytes)
-  [Function `to_string`](#0x1_from_bcs_to_string)
-  [Function `from_bytes`](#0x1_from_bcs_from_bytes)
-  [Specification](#@Specification_1)
    -  [Function `from_bytes`](#@Specification_1_from_bytes)


```move
module 0x1::from_bcs {
    use 0x1::string;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_from_bcs_EINVALID_UTF8"></a>

UTF8 check failed in conversion from bytes to string


```move
module 0x1::from_bcs {
    const EINVALID_UTF8: u64 = 1;
}
```


<a id="0x1_from_bcs_to_bool"></a>

## Function `to_bool`



```move
module 0x1::from_bcs {
    public fun to_bool(v: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_bool(v: vector<u8>): bool {
        from_bytes<bool>(v)
    }
}
```


<a id="0x1_from_bcs_to_u8"></a>

## Function `to_u8`



```move
module 0x1::from_bcs {
    public fun to_u8(v: vector<u8>): u8
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u8(v: vector<u8>): u8 {
        from_bytes<u8>(v)
    }
}
```


<a id="0x1_from_bcs_to_u16"></a>

## Function `to_u16`



```move
module 0x1::from_bcs {
    public fun to_u16(v: vector<u8>): u16
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u16(v: vector<u8>): u16 {
        from_bytes<u16>(v)
    }
}
```


<a id="0x1_from_bcs_to_u32"></a>

## Function `to_u32`



```move
module 0x1::from_bcs {
    public fun to_u32(v: vector<u8>): u32
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u32(v: vector<u8>): u32 {
        from_bytes<u32>(v)
    }
}
```


<a id="0x1_from_bcs_to_u64"></a>

## Function `to_u64`



```move
module 0x1::from_bcs {
    public fun to_u64(v: vector<u8>): u64
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u64(v: vector<u8>): u64 {
        from_bytes<u64>(v)
    }
}
```


<a id="0x1_from_bcs_to_u128"></a>

## Function `to_u128`



```move
module 0x1::from_bcs {
    public fun to_u128(v: vector<u8>): u128
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u128(v: vector<u8>): u128 {
        from_bytes<u128>(v)
    }
}
```


<a id="0x1_from_bcs_to_u256"></a>

## Function `to_u256`



```move
module 0x1::from_bcs {
    public fun to_u256(v: vector<u8>): u256
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_u256(v: vector<u8>): u256 {
        from_bytes<u256>(v)
    }
}
```


<a id="0x1_from_bcs_to_address"></a>

## Function `to_address`



```move
module 0x1::from_bcs {
    public fun to_address(v: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_address(v: vector<u8>): address {
        from_bytes<address>(v)
    }
}
```


<a id="0x1_from_bcs_to_bytes"></a>

## Function `to_bytes`



```move
module 0x1::from_bcs {
    public fun to_bytes(v: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_bytes(v: vector<u8>): vector<u8> {
        from_bytes<vector<u8>>(v)
    }
}
```


<a id="0x1_from_bcs_to_string"></a>

## Function `to_string`



```move
module 0x1::from_bcs {
    public fun to_string(v: vector<u8>): string::String
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public fun to_string(v: vector<u8>): String {
        // To make this safe, we need to evaluate the utf8 invariant.
        let s = from_bytes<String>(v);
        assert!(string::internal_check_utf8(string::bytes(&s)), EINVALID_UTF8);
        s
    }
}
```


<a id="0x1_from_bcs_from_bytes"></a>

## Function `from_bytes`

Package private native function to deserialize a type T.

Note that this function does not put any constraint on `T`. If code uses this function to
deserialize a linear value, its their responsibility that the data they deserialize is
owned.


```move
module 0x1::from_bcs {
    public(friend) fun from_bytes<T>(bytes: vector<u8>): T
}
```


##### Implementation


```move
module 0x1::from_bcs {
    public(friend) native fun from_bytes<T>(bytes: vector<u8>): T;
}
```


<a id="@Specification_1"></a>

## Specification



<a id="0x1_from_bcs_deserialize"></a>


```move
module 0x1::from_bcs {
    fun deserialize<T>(bytes: vector<u8>): T;
<a id="0x1_from_bcs_deserializable"></a>
    fun deserializable<T>(bytes: vector<u8>): bool;
    axiom<T> forall b1: vector<u8>, b2: vector<u8>:
        ( b1 == b2 ==> deserializable<T>(b1) == deserializable<T>(b2) );
    axiom<T> forall b1: vector<u8>, b2: vector<u8>:
        ( b1 == b2 ==> deserialize<T>(b1) == deserialize<T>(b2) );
}
```


<a id="@Specification_1_from_bytes"></a>

### Function `from_bytes`


```move
module 0x1::from_bcs {
    public(friend) fun from_bytes<T>(bytes: vector<u8>): T
}
```



```move
module 0x1::from_bcs {
    pragma opaque;
    aborts_if !deserializable<T>(bytes);
    ensures result == deserialize<T>(bytes);
}
```
