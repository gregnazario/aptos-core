
<a id="0x1_reconfiguration_state"></a>

# Module `0x1::reconfiguration_state`

Reconfiguration meta&#45;state resources and util functions.

WARNING: `reconfiguration_state::initialize()` is required before `RECONFIGURE_WITH_DKG` can be enabled.


-  [Resource `State`](#0x1_reconfiguration_state_State)
-  [Struct `StateInactive`](#0x1_reconfiguration_state_StateInactive)
-  [Struct `StateActive`](#0x1_reconfiguration_state_StateActive)
-  [Constants](#@Constants_0)
-  [Function `is_initialized`](#0x1_reconfiguration_state_is_initialized)
-  [Function `initialize`](#0x1_reconfiguration_state_initialize)
-  [Function `initialize_for_testing`](#0x1_reconfiguration_state_initialize_for_testing)
-  [Function `is_in_progress`](#0x1_reconfiguration_state_is_in_progress)
-  [Function `on_reconfig_start`](#0x1_reconfiguration_state_on_reconfig_start)
-  [Function `start_time_secs`](#0x1_reconfiguration_state_start_time_secs)
-  [Function `on_reconfig_finish`](#0x1_reconfiguration_state_on_reconfig_finish)
-  [Specification](#@Specification_1)
    -  [Resource `State`](#@Specification_1_State)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `initialize_for_testing`](#@Specification_1_initialize_for_testing)
    -  [Function `is_in_progress`](#@Specification_1_is_in_progress)
    -  [Function `on_reconfig_start`](#@Specification_1_on_reconfig_start)
    -  [Function `start_time_secs`](#@Specification_1_start_time_secs)


```move
module 0x1::reconfiguration_state {
    use 0x1::copyable_any;
    use 0x1::error;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::timestamp;
}
```


<a id="0x1_reconfiguration_state_State"></a>

## Resource `State`

Reconfiguration drivers update this resources to notify other modules of some reconfiguration state.


```move
module 0x1::reconfiguration_state {
    struct State has key
}
```


##### Fields


<dl>
<dt>
`variant: copyable_any::Any`
</dt>
<dd>
 The state variant packed as an `Any`.
 Currently the variant type is one of the following.
 &#45; `ReconfigStateInactive`
 &#45; `ReconfigStateActive`
</dd>
</dl>


<a id="0x1_reconfiguration_state_StateInactive"></a>

## Struct `StateInactive`

A state variant indicating no reconfiguration is in progress.


```move
module 0x1::reconfiguration_state {
    struct StateInactive has copy, drop, store
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


<a id="0x1_reconfiguration_state_StateActive"></a>

## Struct `StateActive`

A state variant indicating a reconfiguration is in progress.


```move
module 0x1::reconfiguration_state {
    struct StateActive has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`start_time_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_reconfiguration_state_ERECONFIG_NOT_IN_PROGRESS"></a>



```move
module 0x1::reconfiguration_state {
    const ERECONFIG_NOT_IN_PROGRESS: u64 = 1;
}
```


<a id="0x1_reconfiguration_state_is_initialized"></a>

## Function `is_initialized`



```move
module 0x1::reconfiguration_state {
    public fun is_initialized(): bool
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public fun is_initialized(): bool {
        exists<State>(@aptos_framework)
    }
}
```


<a id="0x1_reconfiguration_state_initialize"></a>

## Function `initialize`



```move
module 0x1::reconfiguration_state {
    public fun initialize(fx: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public fun initialize(fx: &signer) {
        system_addresses::assert_aptos_framework(fx);
        if (!exists<State>(@aptos_framework)) {
            move_to(fx, State {
                variant: copyable_any::pack(StateInactive {})
            })
        }
    }
}
```


<a id="0x1_reconfiguration_state_initialize_for_testing"></a>

## Function `initialize_for_testing`



```move
module 0x1::reconfiguration_state {
    public fun initialize_for_testing(fx: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public fun initialize_for_testing(fx: &signer) {
        initialize(fx)
    }
}
```


<a id="0x1_reconfiguration_state_is_in_progress"></a>

## Function `is_in_progress`

Return whether the reconfiguration state is marked &quot;in progress&quot;.


```move
module 0x1::reconfiguration_state {
    public(friend) fun is_in_progress(): bool
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public(friend) fun is_in_progress(): bool acquires State {
        if (!exists<State>(@aptos_framework)) {
            return false
        };

        let state = borrow_global<State>(@aptos_framework);
        let variant_type_name = *string::bytes(copyable_any::type_name(&state.variant));
        variant_type_name == b"0x1::reconfiguration_state::StateActive"
    }
}
```


<a id="0x1_reconfiguration_state_on_reconfig_start"></a>

## Function `on_reconfig_start`

Called at the beginning of a reconfiguration (either immediate or async)
to mark the reconfiguration state &quot;in progress&quot; if it is currently &quot;stopped&quot;.

Also record the current time as the reconfiguration start time. (Some module, e.g., `stake.move`, needs this info).


```move
module 0x1::reconfiguration_state {
    public(friend) fun on_reconfig_start()
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public(friend) fun on_reconfig_start() acquires State {
        if (exists<State>(@aptos_framework)) {
            let state = borrow_global_mut<State>(@aptos_framework);
            let variant_type_name = *string::bytes(copyable_any::type_name(&state.variant));
            if (variant_type_name == b"0x1::reconfiguration_state::StateInactive") {
                state.variant = copyable_any::pack(StateActive {
                    start_time_secs: timestamp::now_seconds()
                });
            }
        };
    }
}
```


<a id="0x1_reconfiguration_state_start_time_secs"></a>

## Function `start_time_secs`

Get the unix time when the currently in&#45;progress reconfiguration started.
Abort if the reconfiguration state is not &quot;in progress&quot;.


```move
module 0x1::reconfiguration_state {
    public(friend) fun start_time_secs(): u64
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public(friend) fun start_time_secs(): u64 acquires State {
        let state = borrow_global<State>(@aptos_framework);
        let variant_type_name = *string::bytes(copyable_any::type_name(&state.variant));
        if (variant_type_name == b"0x1::reconfiguration_state::StateActive") {
            let active = copyable_any::unpack<StateActive>(state.variant);
            active.start_time_secs
        } else {
            abort(error::invalid_state(ERECONFIG_NOT_IN_PROGRESS))
        }
    }
}
```


<a id="0x1_reconfiguration_state_on_reconfig_finish"></a>

## Function `on_reconfig_finish`

Called at the end of every reconfiguration to mark the state as &quot;stopped&quot;.
Abort if the current state is not &quot;in progress&quot;.


```move
module 0x1::reconfiguration_state {
    public(friend) fun on_reconfig_finish()
}
```


##### Implementation


```move
module 0x1::reconfiguration_state {
    public(friend) fun on_reconfig_finish() acquires State {
        if (exists<State>(@aptos_framework)) {
            let state = borrow_global_mut<State>(@aptos_framework);
            let variant_type_name = *string::bytes(copyable_any::type_name(&state.variant));
            if (variant_type_name == b"0x1::reconfiguration_state::StateActive") {
                state.variant = copyable_any::pack(StateInactive {});
            } else {
                abort(error::invalid_state(ERECONFIG_NOT_IN_PROGRESS))
            }
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::reconfiguration_state {
    invariant [suspendable] chain_status::is_operating() ==> exists<State>(@aptos_framework);
}
```


<a id="@Specification_1_State"></a>

### Resource `State`


```move
module 0x1::reconfiguration_state {
    struct State has key
}
```


<dl>
<dt>
`variant: copyable_any::Any`
</dt>
<dd>
 The state variant packed as an `Any`.
 Currently the variant type is one of the following.
 &#45; `ReconfigStateInactive`
 &#45; `ReconfigStateActive`
</dd>
</dl>



```move
module 0x1::reconfiguration_state {
    invariant copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateActive" ||
        copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateInactive";
    invariant copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateActive"
        ==> from_bcs::deserializable<StateActive>(variant.data);
    invariant copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateInactive"
        ==> from_bcs::deserializable<StateInactive>(variant.data);
    invariant copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateActive" ==>
        type_info::type_name<StateActive>() == variant.type_name;
    invariant copyable_any::type_name(variant).bytes == b"0x1::reconfiguration_state::StateInactive" ==>
        type_info::type_name<StateInactive>() == variant.type_name;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::reconfiguration_state {
    public fun initialize(fx: &signer)
}
```



```move
module 0x1::reconfiguration_state {
    aborts_if signer::address_of(fx) != @aptos_framework;
    let post post_state = global<State>(@aptos_framework);
    ensures exists<State>(@aptos_framework);
    ensures !exists<State>(@aptos_framework) ==> from_bcs::deserializable<StateInactive>(post_state.variant.data);
}
```


<a id="@Specification_1_initialize_for_testing"></a>

### Function `initialize_for_testing`


```move
module 0x1::reconfiguration_state {
    public fun initialize_for_testing(fx: &signer)
}
```



```move
module 0x1::reconfiguration_state {
    aborts_if signer::address_of(fx) != @aptos_framework;
}
```


<a id="@Specification_1_is_in_progress"></a>

### Function `is_in_progress`


```move
module 0x1::reconfiguration_state {
    public(friend) fun is_in_progress(): bool
}
```



```move
module 0x1::reconfiguration_state {
    aborts_if false;
}
```



<a id="0x1_reconfiguration_state_spec_is_in_progress"></a>


```move
module 0x1::reconfiguration_state {
    fun spec_is_in_progress(): bool {
       if (!exists<State>(@aptos_framework)) {
           false
       } else {
           copyable_any::type_name(global<State>(@aptos_framework).variant).bytes == b"0x1::reconfiguration_state::StateActive"
       }
    }
}
```


<a id="@Specification_1_on_reconfig_start"></a>

### Function `on_reconfig_start`


```move
module 0x1::reconfiguration_state {
    public(friend) fun on_reconfig_start()
}
```



```move
module 0x1::reconfiguration_state {
    aborts_if false;
    requires exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    let state = Any {
        type_name: type_info::type_name<StateActive>(),
        data: bcs::serialize(StateActive {
            start_time_secs: timestamp::spec_now_seconds()
        })
    };
    let pre_state = global<State>(@aptos_framework);
    let post post_state = global<State>(@aptos_framework);
    ensures (exists<State>(@aptos_framework) && copyable_any::type_name(pre_state.variant).bytes
        == b"0x1::reconfiguration_state::StateInactive") ==> copyable_any::type_name(post_state.variant).bytes
        == b"0x1::reconfiguration_state::StateActive";
    ensures (exists<State>(@aptos_framework) && copyable_any::type_name(pre_state.variant).bytes
        == b"0x1::reconfiguration_state::StateInactive") ==> post_state.variant == state;
    ensures (exists<State>(@aptos_framework) && copyable_any::type_name(pre_state.variant).bytes
        == b"0x1::reconfiguration_state::StateInactive") ==> from_bcs::deserializable<StateActive>(post_state.variant.data);
}
```


<a id="@Specification_1_start_time_secs"></a>

### Function `start_time_secs`


```move
module 0x1::reconfiguration_state {
    public(friend) fun start_time_secs(): u64
}
```



```move
module 0x1::reconfiguration_state {
    include StartTimeSecsAbortsIf;
}
```



<a id="0x1_reconfiguration_state_spec_start_time_secs"></a>


```move
module 0x1::reconfiguration_state {
    fun spec_start_time_secs(): u64 {
       use aptos_std::from_bcs;
       let state = global<State>(@aptos_framework);
       from_bcs::deserialize<StateActive>(state.variant.data).start_time_secs
    }
}
```



<a id="0x1_reconfiguration_state_StartTimeSecsRequirement"></a>


```move
module 0x1::reconfiguration_state {
    schema StartTimeSecsRequirement {
        requires exists<State>(@aptos_framework);
        requires copyable_any::type_name(global<State>(@aptos_framework).variant).bytes
            == b"0x1::reconfiguration_state::StateActive";
        include UnpackRequiresStateActive {
            x:  global<State>(@aptos_framework).variant
        };
    }
}
```



<a id="0x1_reconfiguration_state_UnpackRequiresStateActive"></a>


```move
module 0x1::reconfiguration_state {
    schema UnpackRequiresStateActive {
        x: Any;
        requires type_info::type_name<StateActive>() == x.type_name && from_bcs::deserializable<StateActive>(x.data);
    }
}
```



<a id="0x1_reconfiguration_state_StartTimeSecsAbortsIf"></a>


```move
module 0x1::reconfiguration_state {
    schema StartTimeSecsAbortsIf {
        aborts_if !exists<State>(@aptos_framework);
        include  copyable_any::type_name(global<State>(@aptos_framework).variant).bytes
            == b"0x1::reconfiguration_state::StateActive" ==>
        copyable_any::UnpackAbortsIf<StateActive> {
            x:  global<State>(@aptos_framework).variant
        };
        aborts_if copyable_any::type_name(global<State>(@aptos_framework).variant).bytes
            != b"0x1::reconfiguration_state::StateActive";
    }
}
```
