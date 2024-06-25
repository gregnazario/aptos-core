
<a id="0x1_execution_config"></a>

# Module `0x1::execution_config`

Maintains the execution config for the blockchain. The config is stored in a
Reconfiguration, and may be updated by root.


-  [Resource `ExecutionConfig`](#0x1_execution_config_ExecutionConfig)
-  [Constants](#@Constants_0)
-  [Function `set`](#0x1_execution_config_set)
-  [Function `set_for_next_epoch`](#0x1_execution_config_set_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_execution_config_on_new_epoch)
-  [Specification](#@Specification_1)
    -  [Function `set`](#@Specification_1_set)
    -  [Function `set_for_next_epoch`](#@Specification_1_set_for_next_epoch)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)


```move
module 0x1::execution_config {
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::error;
    use 0x1::reconfiguration;
    use 0x1::system_addresses;
}
```


<a id="0x1_execution_config_ExecutionConfig"></a>

## Resource `ExecutionConfig`



```move
module 0x1::execution_config {
    struct ExecutionConfig has drop, store, key
}
```


##### Fields


<dl>
<dt>
`config: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_execution_config_EINVALID_CONFIG"></a>

The provided on chain config bytes are empty or invalid


```move
module 0x1::execution_config {
    const EINVALID_CONFIG: u64 = 1;
}
```


<a id="0x1_execution_config_set"></a>

## Function `set`

Deprecated by `set_for_next_epoch()`.

WARNING: calling this while randomness is enabled will trigger a new epoch without randomness!

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::execution_config {
    public fun set(account: &signer, config: vector<u8>)
}
```


##### Implementation


```move
module 0x1::execution_config {
    public fun set(account: &signer, config: vector<u8>) acquires ExecutionConfig {
        system_addresses::assert_aptos_framework(account);
        chain_status::assert_genesis();

        assert!(vector::length(&config) > 0, error::invalid_argument(EINVALID_CONFIG));

        if (exists<ExecutionConfig>(@aptos_framework)) {
            let config_ref = &mut borrow_global_mut<ExecutionConfig>(@aptos_framework).config;
            *config_ref = config;
        } else {
            move_to(account, ExecutionConfig { config });
        };
        // Need to trigger reconfiguration so validator nodes can sync on the updated configs.
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_execution_config_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

This can be called by on&#45;chain governance to update on&#45;chain execution configs for the next epoch.
Example usage:
```
aptos_framework::execution_config::set_for_next_epoch(&amp;framework_signer, some_config_bytes);
aptos_framework::aptos_governance::reconfigure(&amp;framework_signer);
```


```move
module 0x1::execution_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>)
}
```


##### Implementation


```move
module 0x1::execution_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>) {
        system_addresses::assert_aptos_framework(account);
        assert!(vector::length(&config) > 0, error::invalid_argument(EINVALID_CONFIG));
        config_buffer::upsert(ExecutionConfig { config });
    }
}
```


<a id="0x1_execution_config_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `ExecutionConfig`, if there is any.


```move
module 0x1::execution_config {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::execution_config {
    public(friend) fun on_new_epoch(framework: &signer) acquires ExecutionConfig {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<ExecutionConfig>()) {
            let config = config_buffer::extract<ExecutionConfig>();
            if (exists<ExecutionConfig>(@aptos_framework)) {
                *borrow_global_mut<ExecutionConfig>(@aptos_framework) = config;
            } else {
                move_to(framework, config);
            };
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::execution_config {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_set"></a>

### Function `set`


```move
module 0x1::execution_config {
    public fun set(account: &signer, config: vector<u8>)
}
```

Ensure the caller is admin
When setting now time must be later than last_reconfiguration_time.


```move
module 0x1::execution_config {
    pragma verify_duration_estimate = 600;
    let addr = signer::address_of(account);
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    requires chain_status::is_genesis();
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<staking_config::StakingRewardsConfig>(@aptos_framework);
    requires len(config) > 0;
    include features::spec_periodical_reward_rate_decrease_enabled() ==> staking_config::StakingRewardsConfigEnabledRequirement;
    include aptos_coin::ExistsAptosCoin;
    requires system_addresses::is_aptos_framework_address(addr);
    requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
    ensures exists<ExecutionConfig>(@aptos_framework);
}
```


<a id="@Specification_1_set_for_next_epoch"></a>

### Function `set_for_next_epoch`


```move
module 0x1::execution_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>)
}
```



```move
module 0x1::execution_config {
    include config_buffer::SetForNextEpochAbortsIf;
}
```


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::execution_config {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::execution_config {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<ExecutionConfig>;
    aborts_if false;
}
```
