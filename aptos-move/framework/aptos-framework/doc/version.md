
<a id="0x1_version"></a>

# Module `0x1::version`

Maintains the version number for the blockchain.


-  [Resource `Version`](#0x1_version_Version)
-  [Resource `SetVersionCapability`](#0x1_version_SetVersionCapability)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_version_initialize)
-  [Function `set_version`](#0x1_version_set_version)
-  [Function `set_for_next_epoch`](#0x1_version_set_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_version_on_new_epoch)
-  [Function `initialize_for_test`](#0x1_version_initialize_for_test)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `set_version`](#@Specification_1_set_version)
    -  [Function `set_for_next_epoch`](#@Specification_1_set_for_next_epoch)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)
    -  [Function `initialize_for_test`](#@Specification_1_initialize_for_test)


```move
module 0x1::version {
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::error;
    use 0x1::reconfiguration;
    use 0x1::signer;
    use 0x1::system_addresses;
}
```


<a id="0x1_version_Version"></a>

## Resource `Version`



```move
module 0x1::version {
    struct Version has drop, store, key
}
```


##### Fields


<dl>
<dt>
`major: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_version_SetVersionCapability"></a>

## Resource `SetVersionCapability`



```move
module 0x1::version {
    struct SetVersionCapability has key
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


<a id="0x1_version_EINVALID_MAJOR_VERSION_NUMBER"></a>

Specified major version number must be greater than current version number.


```move
module 0x1::version {
    const EINVALID_MAJOR_VERSION_NUMBER: u64 = 1;
}
```


<a id="0x1_version_ENOT_AUTHORIZED"></a>

Account is not authorized to make this change.


```move
module 0x1::version {
    const ENOT_AUTHORIZED: u64 = 2;
}
```


<a id="0x1_version_initialize"></a>

## Function `initialize`

Only called during genesis.
Publishes the Version config.


```move
module 0x1::version {
    public(friend) fun initialize(aptos_framework: &signer, initial_version: u64)
}
```


##### Implementation


```move
module 0x1::version {
    public(friend) fun initialize(aptos_framework: &signer, initial_version: u64) {
        system_addresses::assert_aptos_framework(aptos_framework);

        move_to(aptos_framework, Version { major: initial_version });
        // Give aptos framework account capability to call set version. This allows on chain governance to do it through
        // control of the aptos framework account.
        move_to(aptos_framework, SetVersionCapability {});
    }
}
```


<a id="0x1_version_set_version"></a>

## Function `set_version`

Deprecated by `set_for_next_epoch()`.

WARNING: calling this while randomness is enabled will trigger a new epoch without randomness!

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::version {
    public entry fun set_version(account: &signer, major: u64)
}
```


##### Implementation


```move
module 0x1::version {
    public entry fun set_version(account: &signer, major: u64) acquires Version {
        assert!(exists<SetVersionCapability>(signer::address_of(account)), error::permission_denied(ENOT_AUTHORIZED));
        chain_status::assert_genesis();

        let old_major = borrow_global<Version>(@aptos_framework).major;
        assert!(old_major < major, error::invalid_argument(EINVALID_MAJOR_VERSION_NUMBER));

        let config = borrow_global_mut<Version>(@aptos_framework);
        config.major = major;

        // Need to trigger reconfiguration so validator nodes can sync on the updated version.
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_version_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

Used in on&#45;chain governances to update the major version for the next epoch.
Example usage:
&#45; `aptos_framework::version::set_for_next_epoch(&framework_signer, new_version);`
&#45; `aptos_framework::aptos_governance::reconfigure(&framework_signer);`


```move
module 0x1::version {
    public entry fun set_for_next_epoch(account: &signer, major: u64)
}
```


##### Implementation


```move
module 0x1::version {
    public entry fun set_for_next_epoch(account: &signer, major: u64) acquires Version {
        assert!(exists<SetVersionCapability>(signer::address_of(account)), error::permission_denied(ENOT_AUTHORIZED));
        let old_major = borrow_global<Version>(@aptos_framework).major;
        assert!(old_major < major, error::invalid_argument(EINVALID_MAJOR_VERSION_NUMBER));
        config_buffer::upsert(Version {major});
    }
}
```


<a id="0x1_version_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `Version`, if there is any.


```move
module 0x1::version {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::version {
    public(friend) fun on_new_epoch(framework: &signer) acquires Version {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<Version>()) {
            let new_value = config_buffer::extract<Version>();
            if (exists<Version>(@aptos_framework)) {
                *borrow_global_mut<Version>(@aptos_framework) = new_value;
            } else {
                move_to(framework, new_value);
            }
        }
    }
}
```


<a id="0x1_version_initialize_for_test"></a>

## Function `initialize_for_test`

Only called in tests and testnets. This allows the core resources account, which only exists in tests/testnets,
to update the version.


```move
module 0x1::version {
    fun initialize_for_test(core_resources: &signer)
}
```


##### Implementation


```move
module 0x1::version {
    fun initialize_for_test(core_resources: &signer) {
        system_addresses::assert_core_resource(core_resources);
        move_to(core_resources, SetVersionCapability {});
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
<td>During genesis, the Version resource should be initialized with the initial version and stored along with its capability under the aptos framework account.</td>
<td>Medium</td>
<td>The initialize function ensures that the signer is the aptos framework account and stores the Version and SetVersionCapability resources in it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>The version should be updateable after initialization, but only by the Aptos framework account and with an increasing version number.</td>
<td>Medium</td>
<td>The version number for the blockchain should be updatable whenever necessary. This functionality is provided by the set_version function which ensures that the new version is greater than the previous one.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](set_version).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::version {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::version {
    public(friend) fun initialize(aptos_framework: &signer, initial_version: u64)
}
```

Abort if resource already exists in `@aptos_framwork` when initializing.


```move
module 0x1::version {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if signer::address_of(aptos_framework) != @aptos_framework;
    aborts_if exists<Version>(@aptos_framework);
    aborts_if exists<SetVersionCapability>(@aptos_framework);
    ensures exists<Version>(@aptos_framework);
    ensures exists<SetVersionCapability>(@aptos_framework);
    ensures global<Version>(@aptos_framework) == Version { major: initial_version };
    ensures global<SetVersionCapability>(@aptos_framework) == SetVersionCapability {};
}
```


<a id="@Specification_1_set_version"></a>

### Function `set_version`


```move
module 0x1::version {
    public entry fun set_version(account: &signer, major: u64)
}
```



```move
module 0x1::version {
    pragma verify_duration_estimate = 120;
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    include staking_config::StakingRewardsConfigRequirement;
    requires chain_status::is_genesis();
    requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    aborts_if !exists<SetVersionCapability>(signer::address_of(account));
    aborts_if !exists<Version>(@aptos_framework);
    let old_major = global<Version>(@aptos_framework).major;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if !(old_major < major);
    ensures global<Version>(@aptos_framework).major == major;
}
```


<a id="@Specification_1_set_for_next_epoch"></a>

### Function `set_for_next_epoch`


```move
module 0x1::version {
    public entry fun set_for_next_epoch(account: &signer, major: u64)
}
```



```move
module 0x1::version {
    aborts_if !exists<SetVersionCapability>(signer::address_of(account));
    aborts_if !exists<Version>(@aptos_framework);
    aborts_if global<Version>(@aptos_framework).major >= major;
    aborts_if !exists<config_buffer::PendingConfigs>(@aptos_framework);
}
```


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::version {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::version {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<Version>;
    aborts_if false;
}
```


<a id="@Specification_1_initialize_for_test"></a>

### Function `initialize_for_test`


```move
module 0x1::version {
    fun initialize_for_test(core_resources: &signer)
}
```

This module turns on `aborts_if_is_strict`, so need to add spec for test function `initialize_for_test`.


```move
module 0x1::version {
    pragma verify = false;
}
```
