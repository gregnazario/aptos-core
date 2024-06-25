
<a id="0x1_state_storage"></a>

# Module `0x1::state_storage`



-  [Struct `Usage`](#0x1_state_storage_Usage)
-  [Resource `StateStorageUsage`](#0x1_state_storage_StateStorageUsage)
-  [Resource `GasParameter`](#0x1_state_storage_GasParameter)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_state_storage_initialize)
-  [Function `on_new_block`](#0x1_state_storage_on_new_block)
-  [Function `current_items_and_bytes`](#0x1_state_storage_current_items_and_bytes)
-  [Function `get_state_storage_usage_only_at_epoch_beginning`](#0x1_state_storage_get_state_storage_usage_only_at_epoch_beginning)
-  [Function `on_reconfig`](#0x1_state_storage_on_reconfig)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `on_new_block`](#@Specification_1_on_new_block)
    -  [Function `current_items_and_bytes`](#@Specification_1_current_items_and_bytes)
    -  [Function `get_state_storage_usage_only_at_epoch_beginning`](#@Specification_1_get_state_storage_usage_only_at_epoch_beginning)
    -  [Function `on_reconfig`](#@Specification_1_on_reconfig)


```move
module 0x1::state_storage {
    use 0x1::error;
    use 0x1::system_addresses;
}
```


<a id="0x1_state_storage_Usage"></a>

## Struct `Usage`



```move
module 0x1::state_storage {
    struct Usage has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`items: u64`
</dt>
<dd>

</dd>
<dt>
`bytes: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_state_storage_StateStorageUsage"></a>

## Resource `StateStorageUsage`

This is updated at the beginning of each epoch, reflecting the storage
usage after the last txn of the previous epoch is committed.


```move
module 0x1::state_storage {
    struct StateStorageUsage has store, key
}
```


##### Fields


<dl>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
<dt>
`usage: state_storage::Usage`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_state_storage_GasParameter"></a>

## Resource `GasParameter`



```move
module 0x1::state_storage {
    struct GasParameter has store, key
}
```


##### Fields


<dl>
<dt>
`usage: state_storage::Usage`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_state_storage_ESTATE_STORAGE_USAGE"></a>



```move
module 0x1::state_storage {
    const ESTATE_STORAGE_USAGE: u64 = 0;
}
```


<a id="0x1_state_storage_initialize"></a>

## Function `initialize`



```move
module 0x1::state_storage {
    public(friend) fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::state_storage {
    public(friend) fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            !exists<StateStorageUsage>(@aptos_framework),
            error::already_exists(ESTATE_STORAGE_USAGE)
        );
        move_to(aptos_framework, StateStorageUsage {
            epoch: 0,
            usage: Usage {
                items: 0,
                bytes: 0,
            }
        });
    }
}
```


<a id="0x1_state_storage_on_new_block"></a>

## Function `on_new_block`



```move
module 0x1::state_storage {
    public(friend) fun on_new_block(epoch: u64)
}
```


##### Implementation


```move
module 0x1::state_storage {
    public(friend) fun on_new_block(epoch: u64) acquires StateStorageUsage {
        assert!(
            exists<StateStorageUsage>(@aptos_framework),
            error::not_found(ESTATE_STORAGE_USAGE)
        );
        let usage = borrow_global_mut<StateStorageUsage>(@aptos_framework);
        if (epoch != usage.epoch) {
            usage.epoch = epoch;
            usage.usage = get_state_storage_usage_only_at_epoch_beginning();
        }
    }
}
```


<a id="0x1_state_storage_current_items_and_bytes"></a>

## Function `current_items_and_bytes`



```move
module 0x1::state_storage {
    public(friend) fun current_items_and_bytes(): (u64, u64)
}
```


##### Implementation


```move
module 0x1::state_storage {
    public(friend) fun current_items_and_bytes(): (u64, u64) acquires StateStorageUsage {
        assert!(
            exists<StateStorageUsage>(@aptos_framework),
            error::not_found(ESTATE_STORAGE_USAGE)
        );
        let usage = borrow_global<StateStorageUsage>(@aptos_framework);
        (usage.usage.items, usage.usage.bytes)
    }
}
```


<a id="0x1_state_storage_get_state_storage_usage_only_at_epoch_beginning"></a>

## Function `get_state_storage_usage_only_at_epoch_beginning`

Warning: the result returned is based on the base state view held by the
VM for the entire block or chunk of transactions, it&apos;s only deterministic
if called from the first transaction of the block because the execution layer
guarantees a fresh state view then.


```move
module 0x1::state_storage {
    fun get_state_storage_usage_only_at_epoch_beginning(): state_storage::Usage
}
```


##### Implementation


```move
module 0x1::state_storage {
    native fun get_state_storage_usage_only_at_epoch_beginning(): Usage;
}
```


<a id="0x1_state_storage_on_reconfig"></a>

## Function `on_reconfig`



```move
module 0x1::state_storage {
    public(friend) fun on_reconfig()
}
```


##### Implementation


```move
module 0x1::state_storage {
    public(friend) fun on_reconfig() {
        abort 0
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
<td>Given the blockchain is in an operating state, the resources for tracking state storage usage and gas parameters must exist for the Aptos framework address.</td>
<td>Critical</td>
<td>The initialize function ensures only the Aptos framework address can call it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](module).</td>
</tr>

<tr>
<td>2</td>
<td>During the initialization of the module, it is guaranteed that the resource for tracking state storage usage will be moved under the Aptos framework account with default initial values.</td>
<td>Medium</td>
<td>The resource for tracking state storage usage may only be initialized with specific values and published under the aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](initialize).</td>
</tr>

<tr>
<td>3</td>
<td>The initialization function is only called once, during genesis.</td>
<td>Medium</td>
<td>The initialize function ensures StateStorageUsage does not already exist.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](initialize).</td>
</tr>

<tr>
<td>4</td>
<td>During the initialization of the module, it is guaranteed that the resource for tracking state storage usage will be moved under the Aptos framework account with default initial values.</td>
<td>Medium</td>
<td>The resource for tracking state storage usage may only be initialized with specific values and published under the aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](initialize).</td>
</tr>

<tr>
<td>5</td>
<td>The structure for tracking state storage usage should exist for it to be updated at the beginning of each new block and for retrieving the values of structure members.</td>
<td>Medium</td>
<td>The functions on_new_block and current_items_and_bytes verify that the StateStorageUsage structure exists before performing any further operations.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;5.1](current_items_and_bytes), [#high&#45;level&#45;req&#45;5.2](on_new_block), and the [#high&#45;level&#45;req&#45;5.3](global invariant).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::state_storage {
    pragma verify = true;
    pragma aborts_if_is_strict;
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1) and ### high&#45;level&#45;req&#45;5.3
[#high&#45;level&#45;req](high&#45;level requirement 5):
    invariant [suspendable] chain_status::is_operating() ==> exists<StateStorageUsage>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==> exists<GasParameter>(@aptos_framework);
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::state_storage {
    public(friend) fun initialize(aptos_framework: &signer)
}
```

ensure caller is admin.
aborts if StateStorageUsage already exists.


```move
module 0x1::state_storage {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if !system_addresses::is_aptos_framework_address(addr);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if exists<StateStorageUsage>(@aptos_framework);
    ensures exists<StateStorageUsage>(@aptos_framework);
    let post state_usage = global<StateStorageUsage>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    ensures state_usage.epoch == 0 && state_usage.usage.bytes == 0 && state_usage.usage.items == 0;
}
```


<a id="@Specification_1_on_new_block"></a>

### Function `on_new_block`


```move
module 0x1::state_storage {
    public(friend) fun on_new_block(epoch: u64)
}
```



```move
module 0x1::state_storage {
// This enforces ### high&#45;level&#45;req&#45;5.2
[#high&#45;level&#45;req](high&#45;level requirement 5):
    requires chain_status::is_operating();
    aborts_if false;
    ensures epoch == global<StateStorageUsage>(@aptos_framework).epoch;
}
```


<a id="@Specification_1_current_items_and_bytes"></a>

### Function `current_items_and_bytes`


```move
module 0x1::state_storage {
    public(friend) fun current_items_and_bytes(): (u64, u64)
}
```



```move
module 0x1::state_storage {
// This enforces ### high&#45;level&#45;req&#45;5.1
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if !exists<StateStorageUsage>(@aptos_framework);
}
```


<a id="@Specification_1_get_state_storage_usage_only_at_epoch_beginning"></a>

### Function `get_state_storage_usage_only_at_epoch_beginning`


```move
module 0x1::state_storage {
    fun get_state_storage_usage_only_at_epoch_beginning(): state_storage::Usage
}
```



```move
module 0x1::state_storage {
    pragma opaque;
}
```


<a id="@Specification_1_on_reconfig"></a>

### Function `on_reconfig`


```move
module 0x1::state_storage {
    public(friend) fun on_reconfig()
}
```



```move
module 0x1::state_storage {
    aborts_if true;
}
```
