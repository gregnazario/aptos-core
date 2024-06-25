
<a id="0x1_timestamp"></a>

# Module `0x1::timestamp`

This module keeps a global wall clock that stores the current Unix time in microseconds.
It interacts with the other modules in the following ways:
&#42; genesis: to initialize the timestamp
&#42; block: to reach consensus on the global wall clock time


-  [Resource `CurrentTimeMicroseconds`](#0x1_timestamp_CurrentTimeMicroseconds)
-  [Constants](#@Constants_0)
-  [Function `set_time_has_started`](#0x1_timestamp_set_time_has_started)
-  [Function `update_global_time`](#0x1_timestamp_update_global_time)
-  [Function `now_microseconds`](#0x1_timestamp_now_microseconds)
-  [Function `now_seconds`](#0x1_timestamp_now_seconds)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `update_global_time`](#@Specification_1_update_global_time)


```move
module 0x1::timestamp {
    use 0x1::error;
    use 0x1::system_addresses;
}
```


<a id="0x1_timestamp_CurrentTimeMicroseconds"></a>

## Resource `CurrentTimeMicroseconds`

A singleton resource holding the current Unix time in microseconds


```move
module 0x1::timestamp {
    struct CurrentTimeMicroseconds has key
}
```


##### Fields


<dl>
<dt>
`microseconds: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_timestamp_ENOT_OPERATING"></a>

The blockchain is not in an operating state yet


```move
module 0x1::timestamp {
    const ENOT_OPERATING: u64 = 1;
}
```


<a id="0x1_timestamp_EINVALID_TIMESTAMP"></a>

An invalid timestamp was provided


```move
module 0x1::timestamp {
    const EINVALID_TIMESTAMP: u64 = 2;
}
```


<a id="0x1_timestamp_MICRO_CONVERSION_FACTOR"></a>

Conversion factor between seconds and microseconds


```move
module 0x1::timestamp {
    const MICRO_CONVERSION_FACTOR: u64 = 1000000;
}
```


<a id="0x1_timestamp_set_time_has_started"></a>

## Function `set_time_has_started`

Marks that time has started. This can only be called from genesis and with the aptos framework account.


```move
module 0x1::timestamp {
    public(friend) fun set_time_has_started(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::timestamp {
    public(friend) fun set_time_has_started(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        let timer = CurrentTimeMicroseconds { microseconds: 0 };
        move_to(aptos_framework, timer);
    }
}
```


<a id="0x1_timestamp_update_global_time"></a>

## Function `update_global_time`

Updates the wall clock time by consensus. Requires VM privilege and will be invoked during block prologue.


```move
module 0x1::timestamp {
    public fun update_global_time(account: &signer, proposer: address, timestamp: u64)
}
```


##### Implementation


```move
module 0x1::timestamp {
    public fun update_global_time(
        account: &signer,
        proposer: address,
        timestamp: u64
    ) acquires CurrentTimeMicroseconds {
        // Can only be invoked by AptosVM signer.
        system_addresses::assert_vm(account);

        let global_timer = borrow_global_mut<CurrentTimeMicroseconds>(@aptos_framework);
        let now = global_timer.microseconds;
        if (proposer == @vm_reserved) {
            // NIL block with null address as proposer. Timestamp must be equal.
            assert!(now == timestamp, error::invalid_argument(EINVALID_TIMESTAMP));
        } else {
            // Normal block. Time must advance
            assert!(now < timestamp, error::invalid_argument(EINVALID_TIMESTAMP));
            global_timer.microseconds = timestamp;
        };
    }
}
```


<a id="0x1_timestamp_now_microseconds"></a>

## Function `now_microseconds`

Gets the current time in microseconds.


```move
module 0x1::timestamp {
    #[view]
    public fun now_microseconds(): u64
}
```


##### Implementation


```move
module 0x1::timestamp {
    public fun now_microseconds(): u64 acquires CurrentTimeMicroseconds {
        borrow_global<CurrentTimeMicroseconds>(@aptos_framework).microseconds
    }
}
```


<a id="0x1_timestamp_now_seconds"></a>

## Function `now_seconds`

Gets the current time in seconds.


```move
module 0x1::timestamp {
    #[view]
    public fun now_seconds(): u64
}
```


##### Implementation


```move
module 0x1::timestamp {
    public fun now_seconds(): u64 acquires CurrentTimeMicroseconds {
        now_microseconds() / MICRO_CONVERSION_FACTOR
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
<td>There should only exist one global wall clock and it should be created during genesis.</td>
<td>High</td>
<td>The function set_time_has_started is only called by genesis::initialize and ensures that no other resources of this type exist by only assigning it to a predefined account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](module).</td>
</tr>

<tr>
<td>2</td>
<td>The global wall clock resource should only be owned by the Aptos framework.</td>
<td>High</td>
<td>The function set_time_has_started ensures that only the aptos_framework account can possess the CurrentTimeMicroseconds resource using the assert_aptos_framework function.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](module).</td>
</tr>

<tr>
<td>3</td>
<td>The clock time should only be updated by the VM account.</td>
<td>High</td>
<td>The update_global_time function asserts that the transaction signer is the vm_reserved account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](UpdateGlobalTimeAbortsIf).</td>
</tr>

<tr>
<td>4</td>
<td>The clock time should increase with every update as agreed through consensus and proposed by the current epoch&apos;s validator.</td>
<td>High</td>
<td>The update_global_time function asserts that the new timestamp is greater than the current timestamp.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](UpdateGlobalTimeAbortsIf).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::timestamp {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1) and ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    invariant [suspendable] chain_status::is_operating() ==> exists<CurrentTimeMicroseconds>(@aptos_framework);
}
```


<a id="@Specification_1_update_global_time"></a>

### Function `update_global_time`


```move
module 0x1::timestamp {
    public fun update_global_time(account: &signer, proposer: address, timestamp: u64)
}
```



```move
module 0x1::timestamp {
    requires chain_status::is_operating();
    include UpdateGlobalTimeAbortsIf;
    ensures (proposer != @vm_reserved) ==> (spec_now_microseconds() == timestamp);
}
```



<a id="0x1_timestamp_UpdateGlobalTimeAbortsIf"></a>


```move
module 0x1::timestamp {
    schema UpdateGlobalTimeAbortsIf {
        account: signer;
        proposer: address;
        timestamp: u64;
    // This enforces ### high&#45;level&#45;req&#45;3
    [#high&#45;level&#45;req](high&#45;level requirement 3):
        aborts_if !system_addresses::is_vm(account);
    // This enforces ### high&#45;level&#45;req&#45;4
    [#high&#45;level&#45;req](high&#45;level requirement 4):
        aborts_if (proposer == @vm_reserved) && (spec_now_microseconds() != timestamp);
        aborts_if (proposer != @vm_reserved) && (spec_now_microseconds() >= timestamp);
    }
}
```



<a id="0x1_timestamp_spec_now_microseconds"></a>


```move
module 0x1::timestamp {
    fun spec_now_microseconds(): u64 {
       global<CurrentTimeMicroseconds>(@aptos_framework).microseconds
    }
}
```



<a id="0x1_timestamp_spec_now_seconds"></a>


```move
module 0x1::timestamp {
    fun spec_now_seconds(): u64 {
       spec_now_microseconds() / MICRO_CONVERSION_FACTOR
    }
}
```
