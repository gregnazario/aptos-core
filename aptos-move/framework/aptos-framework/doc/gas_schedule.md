
<a id="0x1_gas_schedule"></a>

# Module `0x1::gas_schedule`

This module defines structs and methods to initialize the gas schedule, which dictates how much
it costs to execute Move on the network.


-  [Struct `GasEntry`](#0x1_gas_schedule_GasEntry)
-  [Resource `GasSchedule`](#0x1_gas_schedule_GasSchedule)
-  [Resource `GasScheduleV2`](#0x1_gas_schedule_GasScheduleV2)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_gas_schedule_initialize)
-  [Function `set_gas_schedule`](#0x1_gas_schedule_set_gas_schedule)
-  [Function `set_for_next_epoch`](#0x1_gas_schedule_set_for_next_epoch)
-  [Function `set_for_next_epoch_check_hash`](#0x1_gas_schedule_set_for_next_epoch_check_hash)
-  [Function `on_new_epoch`](#0x1_gas_schedule_on_new_epoch)
-  [Function `set_storage_gas_config`](#0x1_gas_schedule_set_storage_gas_config)
-  [Function `set_storage_gas_config_for_next_epoch`](#0x1_gas_schedule_set_storage_gas_config_for_next_epoch)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `set_gas_schedule`](#@Specification_1_set_gas_schedule)
    -  [Function `set_for_next_epoch`](#@Specification_1_set_for_next_epoch)
    -  [Function `set_for_next_epoch_check_hash`](#@Specification_1_set_for_next_epoch_check_hash)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)
    -  [Function `set_storage_gas_config`](#@Specification_1_set_storage_gas_config)
    -  [Function `set_storage_gas_config_for_next_epoch`](#@Specification_1_set_storage_gas_config_for_next_epoch)


```move
module 0x1::gas_schedule {
    use 0x1::aptos_hash;
    use 0x1::bcs;
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::error;
    use 0x1::reconfiguration;
    use 0x1::storage_gas;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::util;
    use 0x1::vector;
}
```


<a id="0x1_gas_schedule_GasEntry"></a>

## Struct `GasEntry`



```move
module 0x1::gas_schedule {
    struct GasEntry has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`key: string::String`
</dt>
<dd>

</dd>
<dt>
`val: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_gas_schedule_GasSchedule"></a>

## Resource `GasSchedule`



```move
module 0x1::gas_schedule {
    struct GasSchedule has copy, drop, key
}
```


##### Fields


<dl>
<dt>
`entries: vector<gas_schedule::GasEntry>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_gas_schedule_GasScheduleV2"></a>

## Resource `GasScheduleV2`



```move
module 0x1::gas_schedule {
    struct GasScheduleV2 has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`feature_version: u64`
</dt>
<dd>

</dd>
<dt>
`entries: vector<gas_schedule::GasEntry>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_gas_schedule_EINVALID_GAS_FEATURE_VERSION"></a>



```move
module 0x1::gas_schedule {
    const EINVALID_GAS_FEATURE_VERSION: u64 = 2;
}
```


<a id="0x1_gas_schedule_EINVALID_GAS_SCHEDULE"></a>

The provided gas schedule bytes are empty or invalid


```move
module 0x1::gas_schedule {
    const EINVALID_GAS_SCHEDULE: u64 = 1;
}
```


<a id="0x1_gas_schedule_EINVALID_GAS_SCHEDULE_HASH"></a>



```move
module 0x1::gas_schedule {
    const EINVALID_GAS_SCHEDULE_HASH: u64 = 3;
}
```


<a id="0x1_gas_schedule_initialize"></a>

## Function `initialize`

Only called during genesis.


```move
module 0x1::gas_schedule {
    public(friend) fun initialize(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public(friend) fun initialize(aptos_framework: &signer, gas_schedule_blob: vector<u8>) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(!vector::is_empty(&gas_schedule_blob), error::invalid_argument(EINVALID_GAS_SCHEDULE));

        // TODO(Gas): check if gas schedule is consistent
        let gas_schedule: GasScheduleV2 = from_bytes(gas_schedule_blob);
        move_to<GasScheduleV2>(aptos_framework, gas_schedule);
    }
}
```


<a id="0x1_gas_schedule_set_gas_schedule"></a>

## Function `set_gas_schedule`

Deprecated by `set_for_next_epoch()`.

WARNING: calling this while randomness is enabled will trigger a new epoch without randomness!

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::gas_schedule {
    public fun set_gas_schedule(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public fun set_gas_schedule(aptos_framework: &signer, gas_schedule_blob: vector<u8>) acquires GasSchedule, GasScheduleV2 {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(!vector::is_empty(&gas_schedule_blob), error::invalid_argument(EINVALID_GAS_SCHEDULE));
        chain_status::assert_genesis();

        if (exists<GasScheduleV2>(@aptos_framework)) {
            let gas_schedule = borrow_global_mut<GasScheduleV2>(@aptos_framework);
            let new_gas_schedule: GasScheduleV2 = from_bytes(gas_schedule_blob);
            assert!(new_gas_schedule.feature_version >= gas_schedule.feature_version,
                error::invalid_argument(EINVALID_GAS_FEATURE_VERSION));
            // TODO(Gas): check if gas schedule is consistent
            *gas_schedule = new_gas_schedule;
        }
        else {
            if (exists<GasSchedule>(@aptos_framework)) {
                _ = move_from<GasSchedule>(@aptos_framework);
            };
            let new_gas_schedule: GasScheduleV2 = from_bytes(gas_schedule_blob);
            // TODO(Gas): check if gas schedule is consistent
            move_to<GasScheduleV2>(aptos_framework, new_gas_schedule);
        };

        // Need to trigger reconfiguration so validator nodes can sync on the updated gas schedule.
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_gas_schedule_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

Set the gas schedule for the next epoch, typically called by on&#45;chain governance.
Abort if the version of the given schedule is lower than the current version.

Example usage:
```
aptos_framework::gas_schedule::set_for_next_epoch(&amp;framework_signer, some_gas_schedule_blob);
aptos_framework::aptos_governance::reconfigure(&amp;framework_signer);
```


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch(aptos_framework: &signer, gas_schedule_blob: vector<u8>) acquires GasScheduleV2 {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(!vector::is_empty(&gas_schedule_blob), error::invalid_argument(EINVALID_GAS_SCHEDULE));
        let new_gas_schedule: GasScheduleV2 = from_bytes(gas_schedule_blob);
        if (exists<GasScheduleV2>(@aptos_framework)) {
            let cur_gas_schedule = borrow_global<GasScheduleV2>(@aptos_framework);
            assert!(
                new_gas_schedule.feature_version >= cur_gas_schedule.feature_version,
                error::invalid_argument(EINVALID_GAS_FEATURE_VERSION)
            );
        };
        config_buffer::upsert(new_gas_schedule);
    }
}
```


<a id="0x1_gas_schedule_set_for_next_epoch_check_hash"></a>

## Function `set_for_next_epoch_check_hash`

Set the gas schedule for the next epoch, typically called by on&#45;chain governance.
Abort if the version of the given schedule is lower than the current version.
Require a hash of the old gas schedule to be provided and will abort if the hashes mismatch.


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch_check_hash(aptos_framework: &signer, old_gas_schedule_hash: vector<u8>, new_gas_schedule_blob: vector<u8>)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch_check_hash(
        aptos_framework: &signer,
        old_gas_schedule_hash: vector<u8>,
        new_gas_schedule_blob: vector<u8>
    ) acquires GasScheduleV2 {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(!vector::is_empty(&new_gas_schedule_blob), error::invalid_argument(EINVALID_GAS_SCHEDULE));

        let new_gas_schedule: GasScheduleV2 = from_bytes(new_gas_schedule_blob);
        if (exists<GasScheduleV2>(@aptos_framework)) {
            let cur_gas_schedule = borrow_global<GasScheduleV2>(@aptos_framework);
            assert!(
                new_gas_schedule.feature_version >= cur_gas_schedule.feature_version,
                error::invalid_argument(EINVALID_GAS_FEATURE_VERSION)
            );
            let cur_gas_schedule_bytes = bcs::to_bytes(cur_gas_schedule);
            let cur_gas_schedule_hash = aptos_hash::sha3_512(cur_gas_schedule_bytes);
            assert!(
                cur_gas_schedule_hash == old_gas_schedule_hash,
                error::invalid_argument(EINVALID_GAS_SCHEDULE_HASH)
            );
        };

        config_buffer::upsert(new_gas_schedule);
    }
}
```


<a id="0x1_gas_schedule_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `GasScheduleV2`, if there is any.


```move
module 0x1::gas_schedule {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public(friend) fun on_new_epoch(framework: &signer) acquires GasScheduleV2 {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<GasScheduleV2>()) {
            let new_gas_schedule = config_buffer::extract<GasScheduleV2>();
            if (exists<GasScheduleV2>(@aptos_framework)) {
                *borrow_global_mut<GasScheduleV2>(@aptos_framework) = new_gas_schedule;
            } else {
                move_to(framework, new_gas_schedule);
            }
        }
    }
}
```


<a id="0x1_gas_schedule_set_storage_gas_config"></a>

## Function `set_storage_gas_config`



```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config(aptos_framework: &signer, config: StorageGasConfig) {
        storage_gas::set_config(aptos_framework, config);
        // Need to trigger reconfiguration so the VM is guaranteed to load the new gas fee starting from the next
        // transaction.
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_gas_schedule_set_storage_gas_config_for_next_epoch"></a>

## Function `set_storage_gas_config_for_next_epoch`



```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config_for_next_epoch(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```


##### Implementation


```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config_for_next_epoch(aptos_framework: &signer, config: StorageGasConfig) {
        storage_gas::set_config(aptos_framework, config);
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
<td>During genesis, the Aptos framework account should be assigned the gas schedule resource.</td>
<td>Medium</td>
<td>The gas_schedule::initialize function calls the assert_aptos_framework function to ensure that the signer is the aptos_framework and then assigns the GasScheduleV2 resource to it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>Only the Aptos framework account should be allowed to update the gas schedule resource.</td>
<td>Critical</td>
<td>The gas_schedule::set_gas_schedule function calls the assert_aptos_framework function to ensure that the signer is the aptos framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](set_gas_schedule).</td>
</tr>

<tr>
<td>3</td>
<td>Only valid gas schedule should be allowed for initialization and update.</td>
<td>Medium</td>
<td>The initialize and set_gas_schedule functions ensures that the gas_schedule_blob is not empty.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3.3](initialize) and [#high&#45;level&#45;req&#45;3.2](set_gas_schedule).</td>
</tr>

<tr>
<td>4</td>
<td>Only a gas schedule with the feature version greater or equal than the current feature version is allowed to be provided when performing an update operation.</td>
<td>Medium</td>
<td>The set_gas_schedule function validates the feature_version of the new_gas_schedule by ensuring that it is greater or equal than the current gas_schedule.feature_version.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](set_gas_schedule).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::gas_schedule {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::gas_schedule {
    public(friend) fun initialize(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```



```move
module 0x1::gas_schedule {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
// This enforces ### high&#45;level&#45;req&#45;3.3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if len(gas_schedule_blob) == 0;
    aborts_if exists<GasScheduleV2>(addr);
    ensures exists<GasScheduleV2>(addr);
}
```


<a id="@Specification_1_set_gas_schedule"></a>

### Function `set_gas_schedule`


```move
module 0x1::gas_schedule {
    public fun set_gas_schedule(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```



```move
module 0x1::gas_schedule {
    pragma verify_duration_estimate = 600;
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    requires chain_status::is_genesis();
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    include staking_config::StakingRewardsConfigRequirement;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
// This enforces ### high&#45;level&#45;req&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if len(gas_schedule_blob) == 0;
    let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(gas_schedule_blob);
    let gas_schedule = global<GasScheduleV2>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if exists<GasScheduleV2>(@aptos_framework) && new_gas_schedule.feature_version < gas_schedule.feature_version;
    ensures exists<GasScheduleV2>(signer::address_of(aptos_framework));
    ensures global<GasScheduleV2>(@aptos_framework) == new_gas_schedule;
}
```


<a id="@Specification_1_set_for_next_epoch"></a>

### Function `set_for_next_epoch`


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch(aptos_framework: &signer, gas_schedule_blob: vector<u8>)
}
```



```move
module 0x1::gas_schedule {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    include config_buffer::SetForNextEpochAbortsIf {
        account: aptos_framework,
        config: gas_schedule_blob
    };
    let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(gas_schedule_blob);
    let cur_gas_schedule = global<GasScheduleV2>(@aptos_framework);
    aborts_if exists<GasScheduleV2>(@aptos_framework) && new_gas_schedule.feature_version < cur_gas_schedule.feature_version;
}
```


<a id="@Specification_1_set_for_next_epoch_check_hash"></a>

### Function `set_for_next_epoch_check_hash`


```move
module 0x1::gas_schedule {
    public fun set_for_next_epoch_check_hash(aptos_framework: &signer, old_gas_schedule_hash: vector<u8>, new_gas_schedule_blob: vector<u8>)
}
```



```move
module 0x1::gas_schedule {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    include config_buffer::SetForNextEpochAbortsIf {
        account: aptos_framework,
        config: new_gas_schedule_blob
    };
    let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(new_gas_schedule_blob);
    let cur_gas_schedule = global<GasScheduleV2>(@aptos_framework);
    aborts_if exists<GasScheduleV2>(@aptos_framework) && new_gas_schedule.feature_version < cur_gas_schedule.feature_version;
    aborts_if exists<GasScheduleV2>(@aptos_framework) && (!features::spec_sha_512_and_ripemd_160_enabled() || aptos_hash::spec_sha3_512_internal(bcs::serialize(cur_gas_schedule)) != old_gas_schedule_hash);
}
```


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::gas_schedule {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::gas_schedule {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<GasScheduleV2>;
    aborts_if false;
}
```


<a id="@Specification_1_set_storage_gas_config"></a>

### Function `set_storage_gas_config`


```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```



```move
module 0x1::gas_schedule {
    pragma verify_duration_estimate = 600;
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    include staking_config::StakingRewardsConfigRequirement;
    aborts_if !exists<StorageGasConfig>(@aptos_framework);
    ensures global<StorageGasConfig>(@aptos_framework) == config;
}
```



```move
module 0x1::gas_schedule {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    aborts_if !exists<storage_gas::StorageGasConfig>(@aptos_framework);
}
```


<a id="@Specification_1_set_storage_gas_config_for_next_epoch"></a>

### Function `set_storage_gas_config_for_next_epoch`


```move
module 0x1::gas_schedule {
    public fun set_storage_gas_config_for_next_epoch(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```



```move
module 0x1::gas_schedule {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    aborts_if !exists<storage_gas::StorageGasConfig>(@aptos_framework);
}
```
