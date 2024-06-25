
<a id="0x1_randomness_config_seqnum"></a>

# Module `0x1::randomness_config_seqnum`

Randomness stall recovery utils.

When randomness generation is stuck due to a bug, the chain is also stuck. Below is the recovery procedure.
1. Ensure more than 2/3 stakes are stuck at the same version.
1. Every validator restarts with `randomness_override_seq_num` set to `X+1` in the node config file,
where `X` is the current `RandomnessConfigSeqNum` on chain.
1. The chain should then be unblocked.
1. Once the bug is fixed and the binary &#43; framework have been patched,
a governance proposal is needed to set `RandomnessConfigSeqNum` to be `X+2`.


-  [Resource `RandomnessConfigSeqNum`](#0x1_randomness_config_seqnum_RandomnessConfigSeqNum)
-  [Function `set_for_next_epoch`](#0x1_randomness_config_seqnum_set_for_next_epoch)
-  [Function `initialize`](#0x1_randomness_config_seqnum_initialize)
-  [Function `on_new_epoch`](#0x1_randomness_config_seqnum_on_new_epoch)
-  [Specification](#@Specification_0)
    -  [Function `on_new_epoch`](#@Specification_0_on_new_epoch)


```move
module 0x1::randomness_config_seqnum {
    use 0x1::config_buffer;
    use 0x1::system_addresses;
}
```


<a id="0x1_randomness_config_seqnum_RandomnessConfigSeqNum"></a>

## Resource `RandomnessConfigSeqNum`

If this seqnum is smaller than a validator local override, the on&#45;chain `RandomnessConfig` will be ignored.
Useful in a chain recovery from randomness stall.


```move
module 0x1::randomness_config_seqnum {
    struct RandomnessConfigSeqNum has drop, store, key
}
```


##### Fields


<dl>
<dt>
`seq_num: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_randomness_config_seqnum_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

Update `RandomnessConfigSeqNum`.
Used when re&#45;enable randomness after an emergency randomness disable via local override.


```move
module 0x1::randomness_config_seqnum {
    public fun set_for_next_epoch(framework: &signer, seq_num: u64)
}
```


##### Implementation


```move
module 0x1::randomness_config_seqnum {
    public fun set_for_next_epoch(framework: &signer, seq_num: u64) {
        system_addresses::assert_aptos_framework(framework);
        config_buffer::upsert(RandomnessConfigSeqNum { seq_num });
    }
}
```


<a id="0x1_randomness_config_seqnum_initialize"></a>

## Function `initialize`

Initialize the configuration. Used in genesis or governance.


```move
module 0x1::randomness_config_seqnum {
    public fun initialize(framework: &signer)
}
```


##### Implementation


```move
module 0x1::randomness_config_seqnum {
    public fun initialize(framework: &signer) {
        system_addresses::assert_aptos_framework(framework);
        if (!exists<RandomnessConfigSeqNum>(@aptos_framework)) {
            move_to(framework, RandomnessConfigSeqNum { seq_num: 0 })
        }
    }
}
```


<a id="0x1_randomness_config_seqnum_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `RandomnessConfig`, if there is any.


```move
module 0x1::randomness_config_seqnum {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::randomness_config_seqnum {
    public(friend) fun on_new_epoch(framework: &signer) acquires RandomnessConfigSeqNum {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<RandomnessConfigSeqNum>()) {
            let new_config = config_buffer::extract<RandomnessConfigSeqNum>();
            if (exists<RandomnessConfigSeqNum>(@aptos_framework)) {
                *borrow_global_mut<RandomnessConfigSeqNum>(@aptos_framework) = new_config;
            } else {
                move_to(framework, new_config);
            }
        }
    }
}
```


<a id="@Specification_0"></a>

## Specification


<a id="@Specification_0_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::randomness_config_seqnum {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::randomness_config_seqnum {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<RandomnessConfigSeqNum>;
    aborts_if false;
}
```
