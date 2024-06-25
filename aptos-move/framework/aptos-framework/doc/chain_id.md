
<a id="0x1_chain_id"></a>

# Module `0x1::chain_id`

The chain id distinguishes between different chains (e.g., testnet and the main network).
One important role is to prevent transactions intended for one chain from being executed on another.
This code provides a container for storing a chain id and functions to initialize and get it.


-  [Resource `ChainId`](#0x1_chain_id_ChainId)
-  [Function `initialize`](#0x1_chain_id_initialize)
-  [Function `get`](#0x1_chain_id_get)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_0_initialize)
    -  [Function `get`](#@Specification_0_get)


```move
module 0x1::chain_id {
    use 0x1::system_addresses;
}
```


<a id="0x1_chain_id_ChainId"></a>

## Resource `ChainId`



```move
module 0x1::chain_id {
    struct ChainId has key
}
```


##### Fields


<dl>
<dt>
`id: u8`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_chain_id_initialize"></a>

## Function `initialize`

Only called during genesis.
Publish the chain ID `id` of this instance under the SystemAddresses address


```move
module 0x1::chain_id {
    public(friend) fun initialize(aptos_framework: &signer, id: u8)
}
```


##### Implementation


```move
module 0x1::chain_id {
    public(friend) fun initialize(aptos_framework: &signer, id: u8) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, ChainId { id })
    }
}
```


<a id="0x1_chain_id_get"></a>

## Function `get`

Return the chain ID of this instance.


```move
module 0x1::chain_id {
    #[view]
    public fun get(): u8
}
```


##### Implementation


```move
module 0x1::chain_id {
    public fun get(): u8 acquires ChainId {
        borrow_global<ChainId>(@aptos_framework).id
    }
}
```


<a id="@Specification_0"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>During genesis, the ChainId resource should be created and moved under the Aptos framework account with the specified chain id.</td>
<td>Medium</td>
<td>The chain_id::initialize function is responsible for generating the ChainId resource and then storing it under the aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>The chain id can only be fetched if the chain id resource exists under the Aptos framework account.</td>
<td>Low</td>
<td>The chain_id::get function fetches the chain id by borrowing the ChainId resource from the aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](get).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::chain_id {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_0_initialize"></a>

### Function `initialize`


```move
module 0x1::chain_id {
    public(friend) fun initialize(aptos_framework: &signer, id: u8)
}
```



```move
module 0x1::chain_id {
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if exists<ChainId>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<ChainId>(addr);
    ensures global<ChainId>(addr).id == id;
}
```


<a id="@Specification_0_get"></a>

### Function `get`


```move
module 0x1::chain_id {
    #[view]
    public fun get(): u8
}
```



```move
module 0x1::chain_id {
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if !exists<ChainId>(@aptos_framework);
}
```
