
<a id="0x1_chain_status"></a>

# Module `0x1::chain_status`

This module code to assert that it is running in genesis (`Self::assert_genesis`) or after
genesis (`Self::assert_operating`). These are essentially distinct states of the system. Specifically,
if `Self::assert_operating` succeeds, assumptions about invariants over the global state can be made
which reflect that the system has been successfully initialized.


-  [Resource `GenesisEndMarker`](#0x1_chain_status_GenesisEndMarker)
-  [Constants](#@Constants_0)
-  [Function `set_genesis_end`](#0x1_chain_status_set_genesis_end)
-  [Function `is_genesis`](#0x1_chain_status_is_genesis)
-  [Function `is_operating`](#0x1_chain_status_is_operating)
-  [Function `assert_operating`](#0x1_chain_status_assert_operating)
-  [Function `assert_genesis`](#0x1_chain_status_assert_genesis)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `set_genesis_end`](#@Specification_1_set_genesis_end)
    -  [Function `assert_operating`](#@Specification_1_assert_operating)
    -  [Function `assert_genesis`](#@Specification_1_assert_genesis)


```move
module 0x1::chain_status {
    use 0x1::error;
    use 0x1::system_addresses;
}
```


<a id="0x1_chain_status_GenesisEndMarker"></a>

## Resource `GenesisEndMarker`

Marker to publish at the end of genesis.


```move
module 0x1::chain_status {
    struct GenesisEndMarker has key
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


<a id="@Constants_0"></a>

## Constants


<a id="0x1_chain_status_ENOT_GENESIS"></a>

The blockchain is not in the genesis status.


```move
module 0x1::chain_status {
    const ENOT_GENESIS: u64 = 2;
}
```


<a id="0x1_chain_status_ENOT_OPERATING"></a>

The blockchain is not in the operating status.


```move
module 0x1::chain_status {
    const ENOT_OPERATING: u64 = 1;
}
```


<a id="0x1_chain_status_set_genesis_end"></a>

## Function `set_genesis_end`

Marks that genesis has finished.


```move
module 0x1::chain_status {
    public(friend) fun set_genesis_end(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::chain_status {
    public(friend) fun set_genesis_end(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, GenesisEndMarker {});
    }
}
```


<a id="0x1_chain_status_is_genesis"></a>

## Function `is_genesis`

Helper function to determine if Aptos is in genesis state.


```move
module 0x1::chain_status {
    #[view]
    public fun is_genesis(): bool
}
```


##### Implementation


```move
module 0x1::chain_status {
    public fun is_genesis(): bool {
        !exists<GenesisEndMarker>(@aptos_framework)
    }
}
```


<a id="0x1_chain_status_is_operating"></a>

## Function `is_operating`

Helper function to determine if Aptos is operating. This is
the same as `!is_genesis()` and is provided for convenience.
Testing `is_operating()` is more frequent than `is_genesis()`.


```move
module 0x1::chain_status {
    #[view]
    public fun is_operating(): bool
}
```


##### Implementation


```move
module 0x1::chain_status {
    public fun is_operating(): bool {
        exists<GenesisEndMarker>(@aptos_framework)
    }
}
```


<a id="0x1_chain_status_assert_operating"></a>

## Function `assert_operating`

Helper function to assert operating (not genesis) state.


```move
module 0x1::chain_status {
    public fun assert_operating()
}
```


##### Implementation


```move
module 0x1::chain_status {
    public fun assert_operating() {
        assert!(is_operating(), error::invalid_state(ENOT_OPERATING));
    }
}
```


<a id="0x1_chain_status_assert_genesis"></a>

## Function `assert_genesis`

Helper function to assert genesis state.


```move
module 0x1::chain_status {
    public fun assert_genesis()
}
```


##### Implementation


```move
module 0x1::chain_status {
    public fun assert_genesis() {
        assert!(is_genesis(), error::invalid_state(ENOT_OPERATING));
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
<td>The end of genesis mark should persist throughout the entire life of the chain.</td>
<td>Medium</td>
<td>The Aptos framework account should never drop the GenesisEndMarker resource.</td>
<td>Audited that GenesisEndMarker is published at the end of genesis and never removed. Formally verified via [#high&#45;level&#45;req&#45;1](set_genesis_end) that GenesisEndMarker is published.</td>
</tr>

<tr>
<td>2</td>
<td>The status of the chain should never be genesis and operating at the same time.</td>
<td>Low</td>
<td>The status of the chain is determined by the GenesisEndMarker resource.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](global invariant).</td>
</tr>

<tr>
<td>3</td>
<td>The status of the chain should only be changed once, from genesis to operating.</td>
<td>Low</td>
<td>Attempting to assign a resource type more than once will abort.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](set_genesis_end).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::chain_status {
    pragma verify = true;
    pragma aborts_if_is_strict;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    invariant is_genesis() == !is_operating();
}
```


<a id="@Specification_1_set_genesis_end"></a>

### Function `set_genesis_end`


```move
module 0x1::chain_status {
    public(friend) fun set_genesis_end(aptos_framework: &signer)
}
```



```move
module 0x1::chain_status {
    pragma verify = true;
    pragma delegate_invariants_to_caller;
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if exists<GenesisEndMarker>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures global<GenesisEndMarker>(@aptos_framework) == GenesisEndMarker {};
}
```



<a id="0x1_chain_status_RequiresIsOperating"></a>


```move
module 0x1::chain_status {
    schema RequiresIsOperating {
        requires is_operating();
    }
}
```


<a id="@Specification_1_assert_operating"></a>

### Function `assert_operating`


```move
module 0x1::chain_status {
    public fun assert_operating()
}
```



```move
module 0x1::chain_status {
    aborts_if !is_operating();
}
```


<a id="@Specification_1_assert_genesis"></a>

### Function `assert_genesis`


```move
module 0x1::chain_status {
    public fun assert_genesis()
}
```



```move
module 0x1::chain_status {
    aborts_if !is_genesis();
}
```
