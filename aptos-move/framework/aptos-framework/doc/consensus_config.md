
<a id="0x1_consensus_config"></a>

# Module `0x1::consensus_config`

Maintains the consensus config for the blockchain. The config is stored in a
Reconfiguration, and may be updated by root.


-  [Resource `ConsensusConfig`](#0x1_consensus_config_ConsensusConfig)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_consensus_config_initialize)
-  [Function `set`](#0x1_consensus_config_set)
-  [Function `set_for_next_epoch`](#0x1_consensus_config_set_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_consensus_config_on_new_epoch)
-  [Function `validator_txn_enabled`](#0x1_consensus_config_validator_txn_enabled)
-  [Function `validator_txn_enabled_internal`](#0x1_consensus_config_validator_txn_enabled_internal)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `set`](#@Specification_1_set)
    -  [Function `set_for_next_epoch`](#@Specification_1_set_for_next_epoch)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)
    -  [Function `validator_txn_enabled`](#@Specification_1_validator_txn_enabled)
    -  [Function `validator_txn_enabled_internal`](#@Specification_1_validator_txn_enabled_internal)


```move
module 0x1::consensus_config {
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::error;
    use 0x1::reconfiguration;
    use 0x1::system_addresses;
}
```


<a id="0x1_consensus_config_ConsensusConfig"></a>

## Resource `ConsensusConfig`



```move
module 0x1::consensus_config {
    struct ConsensusConfig has drop, store, key
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


<a id="0x1_consensus_config_EINVALID_CONFIG"></a>

The provided on chain config bytes are empty or invalid


```move
module 0x1::consensus_config {
    const EINVALID_CONFIG: u64 = 1;
}
```


<a id="0x1_consensus_config_initialize"></a>

## Function `initialize`

Publishes the ConsensusConfig config.


```move
module 0x1::consensus_config {
    public(friend) fun initialize(aptos_framework: &signer, config: vector<u8>)
}
```


##### Implementation


```move
module 0x1::consensus_config {
    public(friend) fun initialize(aptos_framework: &signer, config: vector<u8>) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(vector::length(&config) > 0, error::invalid_argument(EINVALID_CONFIG));
        move_to(aptos_framework, ConsensusConfig { config });
    }
}
```


<a id="0x1_consensus_config_set"></a>

## Function `set`

Deprecated by `set_for_next_epoch()`.

WARNING: calling this while randomness is enabled will trigger a new epoch without randomness!

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::consensus_config {
    public fun set(account: &signer, config: vector<u8>)
}
```


##### Implementation


```move
module 0x1::consensus_config {
    public fun set(account: &signer, config: vector<u8>) acquires ConsensusConfig {
        system_addresses::assert_aptos_framework(account);
        chain_status::assert_genesis();
        assert!(vector::length(&config) > 0, error::invalid_argument(EINVALID_CONFIG));

        let config_ref = &mut borrow_global_mut<ConsensusConfig>(@aptos_framework).config;
        *config_ref = config;

        // Need to trigger reconfiguration so validator nodes can sync on the updated configs.
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_consensus_config_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

This can be called by on&#45;chain governance to update on&#45;chain consensus configs for the next epoch.
Example usage:
```
aptos_framework::consensus_config::set_for_next_epoch(&amp;framework_signer, some_config_bytes);
aptos_framework::aptos_governance::reconfigure(&amp;framework_signer);
```


```move
module 0x1::consensus_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>)
}
```


##### Implementation


```move
module 0x1::consensus_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>) {
        system_addresses::assert_aptos_framework(account);
        assert!(vector::length(&config) > 0, error::invalid_argument(EINVALID_CONFIG));
        std::config_buffer::upsert<ConsensusConfig>(ConsensusConfig {config});
    }
}
```


<a id="0x1_consensus_config_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `ConsensusConfig`, if there is any.


```move
module 0x1::consensus_config {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::consensus_config {
    public(friend) fun on_new_epoch(framework: &signer) acquires ConsensusConfig {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<ConsensusConfig>()) {
            let new_config = config_buffer::extract<ConsensusConfig>();
            if (exists<ConsensusConfig>(@aptos_framework)) {
                *borrow_global_mut<ConsensusConfig>(@aptos_framework) = new_config;
            } else {
                move_to(framework, new_config);
            };
        }
    }
}
```


<a id="0x1_consensus_config_validator_txn_enabled"></a>

## Function `validator_txn_enabled`



```move
module 0x1::consensus_config {
    public fun validator_txn_enabled(): bool
}
```


##### Implementation


```move
module 0x1::consensus_config {
    public fun validator_txn_enabled(): bool acquires ConsensusConfig {
        let config_bytes = borrow_global<ConsensusConfig>(@aptos_framework).config;
        validator_txn_enabled_internal(config_bytes)
    }
}
```


<a id="0x1_consensus_config_validator_txn_enabled_internal"></a>

## Function `validator_txn_enabled_internal`



```move
module 0x1::consensus_config {
    fun validator_txn_enabled_internal(config_bytes: vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::consensus_config {
    native fun validator_txn_enabled_internal(config_bytes: vector<u8>): bool;
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
<td>During genesis, the Aptos framework account should be assigned the consensus config resource.</td>
<td>Medium</td>
<td>The consensus_config::initialize function calls the assert_aptos_framework function to ensure that the signer is the aptos_framework and then assigns the ConsensusConfig resource to it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>Only aptos framework account is allowed to update the consensus configuration.</td>
<td>Medium</td>
<td>The consensus_config::set function ensures that the signer is aptos_framework.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](set).</td>
</tr>

<tr>
<td>3</td>
<td>Only a valid configuration can be used during initialization and update.</td>
<td>Medium</td>
<td>Both the initialize and set functions validate the config by ensuring its length to be greater than 0.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3.1](initialize) and [#high&#45;level&#45;req&#45;3.2](set).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::consensus_config {
    pragma verify = true;
    pragma aborts_if_is_strict;
    invariant [suspendable] chain_status::is_operating() ==> exists<ConsensusConfig>(@aptos_framework);
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::consensus_config {
    public(friend) fun initialize(aptos_framework: &signer, config: vector<u8>)
}
```

Ensure caller is admin.
Aborts if StateStorageUsage already exists.


```move
module 0x1::consensus_config {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if !system_addresses::is_aptos_framework_address(addr);
    aborts_if exists<ConsensusConfig>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;3.1
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if !(len(config) > 0);
    ensures global<ConsensusConfig>(addr) == ConsensusConfig { config };
}
```


<a id="@Specification_1_set"></a>

### Function `set`


```move
module 0x1::consensus_config {
    public fun set(account: &signer, config: vector<u8>)
}
```

Ensure the caller is admin and `ConsensusConfig` should be existed.
When setting now time must be later than last_reconfiguration_time.


```move
module 0x1::consensus_config {
    pragma verify_duration_estimate = 600;
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    include staking_config::StakingRewardsConfigRequirement;
    let addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if !system_addresses::is_aptos_framework_address(addr);
    aborts_if !exists<ConsensusConfig>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if !(len(config) > 0);
    requires chain_status::is_genesis();
    requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    ensures global<ConsensusConfig>(@aptos_framework).config == config;
}
```


<a id="@Specification_1_set_for_next_epoch"></a>

### Function `set_for_next_epoch`


```move
module 0x1::consensus_config {
    public fun set_for_next_epoch(account: &signer, config: vector<u8>)
}
```



```move
module 0x1::consensus_config {
    include config_buffer::SetForNextEpochAbortsIf;
}
```


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::consensus_config {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::consensus_config {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<ConsensusConfig>;
    aborts_if false;
}
```


<a id="@Specification_1_validator_txn_enabled"></a>

### Function `validator_txn_enabled`


```move
module 0x1::consensus_config {
    public fun validator_txn_enabled(): bool
}
```



```move
module 0x1::consensus_config {
    pragma opaque;
    aborts_if !exists<ConsensusConfig>(@aptos_framework);
    ensures [abstract] result == spec_validator_txn_enabled_internal(global<ConsensusConfig>(@aptos_framework).config);
}
```


<a id="@Specification_1_validator_txn_enabled_internal"></a>

### Function `validator_txn_enabled_internal`


```move
module 0x1::consensus_config {
    fun validator_txn_enabled_internal(config_bytes: vector<u8>): bool
}
```



```move
module 0x1::consensus_config {
    pragma opaque;
    ensures [abstract] result == spec_validator_txn_enabled_internal(config_bytes);
}
```



<a id="0x1_consensus_config_spec_validator_txn_enabled_internal"></a>


```move
module 0x1::consensus_config {
    fun spec_validator_txn_enabled_internal(config_bytes: vector<u8>): bool;
}
```
