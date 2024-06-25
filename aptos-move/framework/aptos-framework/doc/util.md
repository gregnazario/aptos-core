
<a id="0x1_util"></a>

# Module `0x1::util`

Utility functions used by the framework modules.


-  [Function `from_bytes`](#0x1_util_from_bytes)
-  [Function `address_from_bytes`](#0x1_util_address_from_bytes)
-  [Specification](#@Specification_0)
    -  [Function `from_bytes`](#@Specification_0_from_bytes)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `address_from_bytes`](#@Specification_0_address_from_bytes)


```move
module 0x1::util {
}
```


<a id="0x1_util_from_bytes"></a>

## Function `from_bytes`

Native function to deserialize a type T.

Note that this function does not put any constraint on `T`. If code uses this function to
deserialized a linear value, its their responsibility that the data they deserialize is
owned.


```move
module 0x1::util {
    public(friend) fun from_bytes<T>(bytes: vector<u8>): T
}
```


##### Implementation


```move
module 0x1::util {
    public(friend) native fun from_bytes<T>(bytes: vector<u8>): T;
}
```


<a id="0x1_util_address_from_bytes"></a>

## Function `address_from_bytes`



```move
module 0x1::util {
    public fun address_from_bytes(bytes: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::util {
    public fun address_from_bytes(bytes: vector<u8>): address {
        from_bytes(bytes)
    }
}
```


<a id="@Specification_0"></a>

## Specification


<a id="@Specification_0_from_bytes"></a>

### Function `from_bytes`


```move
module 0x1::util {
    public(friend) fun from_bytes<T>(bytes: vector<u8>): T
}
```




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>The address input bytes should be exactly 32 bytes long.</td>
<td>Low</td>
<td>The address_from_bytes function should assert if the length of the input bytes is 32.</td>
<td>Verified via [#high&#45;level&#45;req&#45;1](address_from_bytes).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::util {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_from_bytes<T>(bytes);
}
```



<a id="0x1_util_spec_from_bytes"></a>


```move
module 0x1::util {
    fun spec_from_bytes<T>(bytes: vector<u8>): T;
}
```


<a id="@Specification_0_address_from_bytes"></a>

### Function `address_from_bytes`


```move
module 0x1::util {
    public fun address_from_bytes(bytes: vector<u8>): address
}
```



```move
module 0x1::util {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if [abstract] len(bytes) != 32;
}
```
