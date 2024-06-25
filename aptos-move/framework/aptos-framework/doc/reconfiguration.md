
<a id="0x1_reconfiguration"></a>

# Module `0x1::reconfiguration`

Publishes configuration information for validators, and issues reconfiguration events
to synchronize configuration changes for the validators.


-  [Struct `NewEpochEvent`](#0x1_reconfiguration_NewEpochEvent)
-  [Struct `NewEpoch`](#0x1_reconfiguration_NewEpoch)
-  [Resource `Configuration`](#0x1_reconfiguration_Configuration)
-  [Resource `DisableReconfiguration`](#0x1_reconfiguration_DisableReconfiguration)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_reconfiguration_initialize)
-  [Function `disable_reconfiguration`](#0x1_reconfiguration_disable_reconfiguration)
-  [Function `enable_reconfiguration`](#0x1_reconfiguration_enable_reconfiguration)
-  [Function `reconfiguration_enabled`](#0x1_reconfiguration_reconfiguration_enabled)
-  [Function `reconfigure`](#0x1_reconfiguration_reconfigure)
-  [Function `last_reconfiguration_time`](#0x1_reconfiguration_last_reconfiguration_time)
-  [Function `current_epoch`](#0x1_reconfiguration_current_epoch)
-  [Function `emit_genesis_reconfiguration_event`](#0x1_reconfiguration_emit_genesis_reconfiguration_event)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `disable_reconfiguration`](#@Specification_1_disable_reconfiguration)
    -  [Function `enable_reconfiguration`](#@Specification_1_enable_reconfiguration)
    -  [Function `reconfiguration_enabled`](#@Specification_1_reconfiguration_enabled)
    -  [Function `reconfigure`](#@Specification_1_reconfigure)
    -  [Function `last_reconfiguration_time`](#@Specification_1_last_reconfiguration_time)
    -  [Function `current_epoch`](#@Specification_1_current_epoch)
    -  [Function `emit_genesis_reconfiguration_event`](#@Specification_1_emit_genesis_reconfiguration_event)


```move
module 0x1::reconfiguration {
    use 0x1::account;
    use 0x1::chain_status;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::reconfiguration_state;
    use 0x1::signer;
    use 0x1::stake;
    use 0x1::storage_gas;
    use 0x1::system_addresses;
    use 0x1::timestamp;
    use 0x1::transaction_fee;
}
```


<a id="0x1_reconfiguration_NewEpochEvent"></a>

## Struct `NewEpochEvent`

Event that signals consensus to start a new epoch,
with new configuration information. This is also called a
&quot;reconfiguration event&quot;


```move
module 0x1::reconfiguration {
    #[event]
    struct NewEpochEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_reconfiguration_NewEpoch"></a>

## Struct `NewEpoch`

Event that signals consensus to start a new epoch,
with new configuration information. This is also called a
&quot;reconfiguration event&quot;


```move
module 0x1::reconfiguration {
    #[event]
    struct NewEpoch has drop, store
}
```


##### Fields


<dl>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_reconfiguration_Configuration"></a>

## Resource `Configuration`

Holds information about state of reconfiguration


```move
module 0x1::reconfiguration {
    struct Configuration has key
}
```


##### Fields


<dl>
<dt>
`epoch: u64`
</dt>
<dd>
 Epoch number
</dd>
<dt>
`last_reconfiguration_time: u64`
</dt>
<dd>
 Time of last reconfiguration. Only changes on reconfiguration events.
</dd>
<dt>
`events: event::EventHandle<reconfiguration::NewEpochEvent>`
</dt>
<dd>
 Event handle for reconfiguration events
</dd>
</dl>


<a id="0x1_reconfiguration_DisableReconfiguration"></a>

## Resource `DisableReconfiguration`

Reconfiguration will be disabled if this resource is published under the
aptos_framework system address


```move
module 0x1::reconfiguration {
    struct DisableReconfiguration has key
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


<a id="0x1_reconfiguration_ECONFIG"></a>

A `Reconfiguration` resource is in an invalid state


```move
module 0x1::reconfiguration {
    const ECONFIG: u64 = 2;
}
```


<a id="0x1_reconfiguration_ECONFIGURATION"></a>

The `Configuration` resource is in an invalid state


```move
module 0x1::reconfiguration {
    const ECONFIGURATION: u64 = 1;
}
```


<a id="0x1_reconfiguration_EINVALID_BLOCK_TIME"></a>

An invalid block time was encountered.


```move
module 0x1::reconfiguration {
    const EINVALID_BLOCK_TIME: u64 = 4;
}
```


<a id="0x1_reconfiguration_EINVALID_GUID_FOR_EVENT"></a>

An invalid block time was encountered.


```move
module 0x1::reconfiguration {
    const EINVALID_GUID_FOR_EVENT: u64 = 5;
}
```


<a id="0x1_reconfiguration_EMODIFY_CAPABILITY"></a>

A `ModifyConfigCapability` is in a different state than was expected


```move
module 0x1::reconfiguration {
    const EMODIFY_CAPABILITY: u64 = 3;
}
```


<a id="0x1_reconfiguration_initialize"></a>

## Function `initialize`

Only called during genesis.
Publishes `Configuration` resource. Can only be invoked by aptos framework account, and only a single time in Genesis.


```move
module 0x1::reconfiguration {
    public(friend) fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    public(friend) fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);

        // assert it matches `new_epoch_event_key()`, otherwise the event can't be recognized
        assert!(account::get_guid_next_creation_num(signer::address_of(aptos_framework)) == 2, error::invalid_state(EINVALID_GUID_FOR_EVENT));
        move_to<Configuration>(
            aptos_framework,
            Configuration {
                epoch: 0,
                last_reconfiguration_time: 0,
                events: account::new_event_handle<NewEpochEvent>(aptos_framework),
            }
        );
    }
}
```


<a id="0x1_reconfiguration_disable_reconfiguration"></a>

## Function `disable_reconfiguration`

Private function to temporarily halt reconfiguration.
This function should only be used for offline WriteSet generation purpose and should never be invoked on chain.


```move
module 0x1::reconfiguration {
    fun disable_reconfiguration(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    fun disable_reconfiguration(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(reconfiguration_enabled(), error::invalid_state(ECONFIGURATION));
        move_to(aptos_framework, DisableReconfiguration {})
    }
}
```


<a id="0x1_reconfiguration_enable_reconfiguration"></a>

## Function `enable_reconfiguration`

Private function to resume reconfiguration.
This function should only be used for offline WriteSet generation purpose and should never be invoked on chain.


```move
module 0x1::reconfiguration {
    fun enable_reconfiguration(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    fun enable_reconfiguration(aptos_framework: &signer) acquires DisableReconfiguration {
        system_addresses::assert_aptos_framework(aptos_framework);

        assert!(!reconfiguration_enabled(), error::invalid_state(ECONFIGURATION));
        DisableReconfiguration {} = move_from<DisableReconfiguration>(signer::address_of(aptos_framework));
    }
}
```


<a id="0x1_reconfiguration_reconfiguration_enabled"></a>

## Function `reconfiguration_enabled`



```move
module 0x1::reconfiguration {
    fun reconfiguration_enabled(): bool
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    fun reconfiguration_enabled(): bool {
        !exists<DisableReconfiguration>(@aptos_framework)
    }
}
```


<a id="0x1_reconfiguration_reconfigure"></a>

## Function `reconfigure`

Signal validators to start using new configuration. Must be called from friend config modules.


```move
module 0x1::reconfiguration {
    public(friend) fun reconfigure()
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    public(friend) fun reconfigure() acquires Configuration {
        // Do not do anything if genesis has not finished.
        if (chain_status::is_genesis() || timestamp::now_microseconds() == 0 || !reconfiguration_enabled()) {
            return
        };

        let config_ref = borrow_global_mut<Configuration>(@aptos_framework);
        let current_time = timestamp::now_microseconds();

        // Do not do anything if a reconfiguration event is already emitted within this transaction.
        //
        // This is OK because:
        // - The time changes in every non-empty block
        // - A block automatically ends after a transaction that emits a reconfiguration event, which is guaranteed by
        //   VM spec that all transactions comming after a reconfiguration transaction will be returned as Retry
        //   status.
        // - Each transaction must emit at most one reconfiguration event
        //
        // Thus, this check ensures that a transaction that does multiple "reconfiguration required" actions emits only
        // one reconfiguration event.
        //
        if (current_time == config_ref.last_reconfiguration_time) {
            return
        };

        reconfiguration_state::on_reconfig_start();

        // Reconfiguration "forces the block" to end, as mentioned above. Therefore, we must process the collected fees
        // explicitly so that staking can distribute them.
        //
        // This also handles the case when a validator is removed due to the governance proposal. In particular, removing
        // the validator causes a reconfiguration. We explicitly process fees, i.e. we drain aggregatable coin and populate
        // the fees table, prior to calling `on_new_epoch()`. That call, in turn, distributes transaction fees for all active
        // and pending_inactive validators, which include any validator that is to be removed.
        if (features::collect_and_distribute_gas_fees()) {
            // All transactions after reconfiguration are Retry. Therefore, when the next
            // block starts and tries to assign/burn collected fees it will be just 0 and
            // nothing will be assigned.
            transaction_fee::process_collected_fees();
        };

        // Call stake to compute the new validator set and distribute rewards and transaction fees.
        stake::on_new_epoch();
        storage_gas::on_reconfig();

        assert!(current_time > config_ref.last_reconfiguration_time, error::invalid_state(EINVALID_BLOCK_TIME));
        config_ref.last_reconfiguration_time = current_time;
        spec {
            assume config_ref.epoch + 1 <= MAX_U64;
        };
        config_ref.epoch = config_ref.epoch + 1;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                NewEpoch {
                    epoch: config_ref.epoch,
                },
            );
        };
        event::emit_event<NewEpochEvent>(
            &mut config_ref.events,
            NewEpochEvent {
                epoch: config_ref.epoch,
            },
        );

        reconfiguration_state::on_reconfig_finish();
    }
}
```


<a id="0x1_reconfiguration_last_reconfiguration_time"></a>

## Function `last_reconfiguration_time`



```move
module 0x1::reconfiguration {
    public fun last_reconfiguration_time(): u64
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    public fun last_reconfiguration_time(): u64 acquires Configuration {
        borrow_global<Configuration>(@aptos_framework).last_reconfiguration_time
    }
}
```


<a id="0x1_reconfiguration_current_epoch"></a>

## Function `current_epoch`



```move
module 0x1::reconfiguration {
    public fun current_epoch(): u64
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    public fun current_epoch(): u64 acquires Configuration {
        borrow_global<Configuration>(@aptos_framework).epoch
    }
}
```


<a id="0x1_reconfiguration_emit_genesis_reconfiguration_event"></a>

## Function `emit_genesis_reconfiguration_event`

Emit a `NewEpochEvent` event. This function will be invoked by genesis directly to generate the very first
reconfiguration event.


```move
module 0x1::reconfiguration {
    fun emit_genesis_reconfiguration_event()
}
```


##### Implementation


```move
module 0x1::reconfiguration {
    fun emit_genesis_reconfiguration_event() acquires Configuration {
        let config_ref = borrow_global_mut<Configuration>(@aptos_framework);
        assert!(config_ref.epoch == 0 && config_ref.last_reconfiguration_time == 0, error::invalid_state(ECONFIGURATION));
        config_ref.epoch = 1;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                NewEpoch {
                    epoch: config_ref.epoch,
                },
            );
        };
        event::emit_event<NewEpochEvent>(
            &mut config_ref.events,
            NewEpochEvent {
                epoch: config_ref.epoch,
            },
        );
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
<td>The Configuration resource is stored under the Aptos framework account with initial values upon module&apos;s initialization.</td>
<td>Medium</td>
<td>The Configuration resource may only be initialized with specific values and published under the aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>The reconfiguration status may be determined at any time without causing an abort, indicating whether or not the system allows reconfiguration.</td>
<td>Low</td>
<td>The reconfiguration_enabled function will never abort and always returns a boolean value that accurately represents whether the system allows reconfiguration.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](reconfiguration_enabled).</td>
</tr>

<tr>
<td>3</td>
<td>For each reconfiguration, the epoch value (config_ref.epoch) increases by 1, and one &apos;NewEpochEvent&apos; is emitted.</td>
<td>Critical</td>
<td>After reconfiguration, the reconfigure() function increases the epoch value of the configuration by one and increments the counter of the NewEpochEvent&apos;s EventHandle by one.</td>
<td>Audited that these two values remain in sync.</td>
</tr>

<tr>
<td>4</td>
<td>Reconfiguration is possible only if genesis has started and reconfiguration is enabled. Also, the last reconfiguration must not be the current time, returning early without further actions otherwise.</td>
<td>High</td>
<td>The reconfigure() function may only execute to perform successful reconfiguration when genesis has started and when reconfiguration is enabled. Without satisfying both conditions, the function returns early without executing any further actions.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](reconfigure).</td>
</tr>

<tr>
<td>5</td>
<td>Consecutive reconfigurations without the passage of time are not permitted.</td>
<td>High</td>
<td>The reconfigure() function enforces the restriction that reconfiguration may only be performed when the current time is not equal to the last_reconfiguration_time.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](reconfigure).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::reconfiguration {
    pragma verify = true;
    pragma aborts_if_is_strict;
    invariant [suspendable] chain_status::is_operating() ==> exists<Configuration>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==>
        (timestamp::spec_now_microseconds() >= last_reconfiguration_time());
}
```

Make sure the signer address is @aptos_framework.


<a id="0x1_reconfiguration_AbortsIfNotAptosFramework"></a>


```move
module 0x1::reconfiguration {
    schema AbortsIfNotAptosFramework {
        aptos_framework: &signer;
        let addr = signer::address_of(aptos_framework);
        aborts_if !system_addresses::is_aptos_framework_address(addr);
    }
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::reconfiguration {
    public(friend) fun initialize(aptos_framework: &signer)
}
```

Address @aptos_framework must exist resource Account and Configuration.
Already exists in framework account.
Guid_creation_num should be 2 according to logic.


```move
module 0x1::reconfiguration {
    include AbortsIfNotAptosFramework;
    let addr = signer::address_of(aptos_framework);
    let post config = global<Configuration>(@aptos_framework);
    requires exists<Account>(addr);
    aborts_if !(global<Account>(addr).guid_creation_num == 2);
    aborts_if exists<Configuration>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<Configuration>(@aptos_framework);
    ensures config.epoch == 0 && config.last_reconfiguration_time == 0;
    ensures config.events == event::EventHandle<NewEpochEvent> {
        counter: 0,
        guid: guid::GUID {
            id: guid::ID {
                creation_num: 2,
                addr: @aptos_framework
            }
        }
    };
}
```


<a id="@Specification_1_disable_reconfiguration"></a>

### Function `disable_reconfiguration`


```move
module 0x1::reconfiguration {
    fun disable_reconfiguration(aptos_framework: &signer)
}
```



```move
module 0x1::reconfiguration {
    include AbortsIfNotAptosFramework;
    aborts_if exists<DisableReconfiguration>(@aptos_framework);
    ensures exists<DisableReconfiguration>(@aptos_framework);
}
```


<a id="@Specification_1_enable_reconfiguration"></a>

### Function `enable_reconfiguration`


```move
module 0x1::reconfiguration {
    fun enable_reconfiguration(aptos_framework: &signer)
}
```

Make sure the caller is admin and check the resource DisableReconfiguration.


```move
module 0x1::reconfiguration {
    include AbortsIfNotAptosFramework;
    aborts_if !exists<DisableReconfiguration>(@aptos_framework);
    ensures !exists<DisableReconfiguration>(@aptos_framework);
}
```


<a id="@Specification_1_reconfiguration_enabled"></a>

### Function `reconfiguration_enabled`


```move
module 0x1::reconfiguration {
    fun reconfiguration_enabled(): bool
}
```



```move
module 0x1::reconfiguration {
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
    ensures result == !exists<DisableReconfiguration>(@aptos_framework);
}
```


<a id="@Specification_1_reconfigure"></a>

### Function `reconfigure`


```move
module 0x1::reconfiguration {
    public(friend) fun reconfigure()
}
```



```move
module 0x1::reconfiguration {
    pragma verify = true;
    pragma verify_duration_estimate = 600;
    requires exists<stake::ValidatorFees>(@aptos_framework);
    let success = !(chain_status::is_genesis() || timestamp::spec_now_microseconds() == 0 || !reconfiguration_enabled())
        && timestamp::spec_now_microseconds() != global<Configuration>(@aptos_framework).last_reconfiguration_time;
    include features::spec_periodical_reward_rate_decrease_enabled() ==> staking_config::StakingRewardsConfigEnabledRequirement;
    include success ==> aptos_coin::ExistsAptosCoin;
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    aborts_if false;
    ensures success ==> global<Configuration>(@aptos_framework).epoch == old(global<Configuration>(@aptos_framework).epoch) + 1;
    ensures success ==> global<Configuration>(@aptos_framework).last_reconfiguration_time == timestamp::spec_now_microseconds();
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4) and ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    ensures !success ==> global<Configuration>(@aptos_framework).epoch == old(global<Configuration>(@aptos_framework).epoch);
}
```


<a id="@Specification_1_last_reconfiguration_time"></a>

### Function `last_reconfiguration_time`


```move
module 0x1::reconfiguration {
    public fun last_reconfiguration_time(): u64
}
```



```move
module 0x1::reconfiguration {
    aborts_if !exists<Configuration>(@aptos_framework);
    ensures result == global<Configuration>(@aptos_framework).last_reconfiguration_time;
}
```


<a id="@Specification_1_current_epoch"></a>

### Function `current_epoch`


```move
module 0x1::reconfiguration {
    public fun current_epoch(): u64
}
```



```move
module 0x1::reconfiguration {
    aborts_if !exists<Configuration>(@aptos_framework);
    ensures result == global<Configuration>(@aptos_framework).epoch;
}
```


<a id="@Specification_1_emit_genesis_reconfiguration_event"></a>

### Function `emit_genesis_reconfiguration_event`


```move
module 0x1::reconfiguration {
    fun emit_genesis_reconfiguration_event()
}
```

When genesis_event emit the epoch and the `last_reconfiguration_time` .
Should equal to 0


```move
module 0x1::reconfiguration {
    aborts_if !exists<Configuration>(@aptos_framework);
    let config_ref = global<Configuration>(@aptos_framework);
    aborts_if !(config_ref.epoch == 0 && config_ref.last_reconfiguration_time == 0);
    ensures global<Configuration>(@aptos_framework).epoch == 1;
}
```
