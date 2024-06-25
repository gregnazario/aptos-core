
<a id="0x1_dkg"></a>

# Module `0x1::dkg`

DKG on&#45;chain states and helper functions.


-  [Struct `DKGSessionMetadata`](#0x1_dkg_DKGSessionMetadata)
-  [Struct `DKGStartEvent`](#0x1_dkg_DKGStartEvent)
-  [Struct `DKGSessionState`](#0x1_dkg_DKGSessionState)
-  [Resource `DKGState`](#0x1_dkg_DKGState)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_dkg_initialize)
-  [Function `start`](#0x1_dkg_start)
-  [Function `finish`](#0x1_dkg_finish)
-  [Function `try_clear_incomplete_session`](#0x1_dkg_try_clear_incomplete_session)
-  [Function `incomplete_session`](#0x1_dkg_incomplete_session)
-  [Function `session_dealer_epoch`](#0x1_dkg_session_dealer_epoch)
-  [Specification](#@Specification_1)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `start`](#@Specification_1_start)
    -  [Function `finish`](#@Specification_1_finish)
    -  [Function `try_clear_incomplete_session`](#@Specification_1_try_clear_incomplete_session)
    -  [Function `incomplete_session`](#@Specification_1_incomplete_session)


```move
module 0x1::dkg {
    use 0x1::error;
    use 0x1::event;
    use 0x1::option;
    use 0x1::randomness_config;
    use 0x1::system_addresses;
    use 0x1::timestamp;
    use 0x1::validator_consensus_info;
}
```


<a id="0x1_dkg_DKGSessionMetadata"></a>

## Struct `DKGSessionMetadata`

This can be considered as the public input of DKG.


```move
module 0x1::dkg {
    struct DKGSessionMetadata has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`dealer_epoch: u64`
</dt>
<dd>

</dd>
<dt>
`randomness_config: randomness_config::RandomnessConfig`
</dt>
<dd>

</dd>
<dt>
`dealer_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>`
</dt>
<dd>

</dd>
<dt>
`target_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_dkg_DKGStartEvent"></a>

## Struct `DKGStartEvent`



```move
module 0x1::dkg {
    #[event]
    struct DKGStartEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`session_metadata: dkg::DKGSessionMetadata`
</dt>
<dd>

</dd>
<dt>
`start_time_us: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_dkg_DKGSessionState"></a>

## Struct `DKGSessionState`

The input and output of a DKG session.
The validator set of epoch `x` works together for an DKG output for the target validator set of epoch `x+1`.


```move
module 0x1::dkg {
    struct DKGSessionState has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`metadata: dkg::DKGSessionMetadata`
</dt>
<dd>

</dd>
<dt>
`start_time_us: u64`
</dt>
<dd>

</dd>
<dt>
`transcript: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_dkg_DKGState"></a>

## Resource `DKGState`

The completed and in&#45;progress DKG sessions.


```move
module 0x1::dkg {
    struct DKGState has key
}
```


##### Fields


<dl>
<dt>
`last_completed: option::Option<dkg::DKGSessionState>`
</dt>
<dd>

</dd>
<dt>
`in_progress: option::Option<dkg::DKGSessionState>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_dkg_EDKG_IN_PROGRESS"></a>



```move
module 0x1::dkg {
    const EDKG_IN_PROGRESS: u64 = 1;
}
```


<a id="0x1_dkg_EDKG_NOT_IN_PROGRESS"></a>



```move
module 0x1::dkg {
    const EDKG_NOT_IN_PROGRESS: u64 = 2;
}
```


<a id="0x1_dkg_initialize"></a>

## Function `initialize`

Called in genesis to initialize on&#45;chain states.


```move
module 0x1::dkg {
    public fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::dkg {
    public fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        if (!exists<DKGState>(@aptos_framework)) {
            move_to<DKGState>(
                aptos_framework,
                DKGState {
                    last_completed: std::option::none(),
                    in_progress: std::option::none(),
                }
            );
        }
    }
}
```


<a id="0x1_dkg_start"></a>

## Function `start`

Mark on&#45;chain DKG state as in&#45;progress. Notify validators to start DKG.
Abort if a DKG is already in progress.


```move
module 0x1::dkg {
    public(friend) fun start(dealer_epoch: u64, randomness_config: randomness_config::RandomnessConfig, dealer_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>, target_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>)
}
```


##### Implementation


```move
module 0x1::dkg {
    public(friend) fun start(
        dealer_epoch: u64,
        randomness_config: RandomnessConfig,
        dealer_validator_set: vector<ValidatorConsensusInfo>,
        target_validator_set: vector<ValidatorConsensusInfo>,
    ) acquires DKGState {
        let dkg_state = borrow_global_mut<DKGState>(@aptos_framework);
        let new_session_metadata = DKGSessionMetadata {
            dealer_epoch,
            randomness_config,
            dealer_validator_set,
            target_validator_set,
        };
        let start_time_us = timestamp::now_microseconds();
        dkg_state.in_progress = std::option::some(DKGSessionState {
            metadata: new_session_metadata,
            start_time_us,
            transcript: vector[],
        });

        emit(DKGStartEvent {
            start_time_us,
            session_metadata: new_session_metadata,
        });
    }
}
```


<a id="0x1_dkg_finish"></a>

## Function `finish`

Put a transcript into the currently incomplete DKG session, then mark it completed.

Abort if DKG is not in progress.


```move
module 0x1::dkg {
    public(friend) fun finish(transcript: vector<u8>)
}
```


##### Implementation


```move
module 0x1::dkg {
    public(friend) fun finish(transcript: vector<u8>) acquires DKGState {
        let dkg_state = borrow_global_mut<DKGState>(@aptos_framework);
        assert!(option::is_some(&dkg_state.in_progress), error::invalid_state(EDKG_NOT_IN_PROGRESS));
        let session = option::extract(&mut dkg_state.in_progress);
        session.transcript = transcript;
        dkg_state.last_completed = option::some(session);
        dkg_state.in_progress = option::none();
    }
}
```


<a id="0x1_dkg_try_clear_incomplete_session"></a>

## Function `try_clear_incomplete_session`

Delete the currently incomplete session, if it exists.


```move
module 0x1::dkg {
    public fun try_clear_incomplete_session(fx: &signer)
}
```


##### Implementation


```move
module 0x1::dkg {
    public fun try_clear_incomplete_session(fx: &signer) acquires DKGState {
        system_addresses::assert_aptos_framework(fx);
        if (exists<DKGState>(@aptos_framework)) {
            let dkg_state = borrow_global_mut<DKGState>(@aptos_framework);
            dkg_state.in_progress = option::none();
        }
    }
}
```


<a id="0x1_dkg_incomplete_session"></a>

## Function `incomplete_session`

Return the incomplete DKG session state, if it exists.


```move
module 0x1::dkg {
    public fun incomplete_session(): option::Option<dkg::DKGSessionState>
}
```


##### Implementation


```move
module 0x1::dkg {
    public fun incomplete_session(): Option<DKGSessionState> acquires DKGState {
        if (exists<DKGState>(@aptos_framework)) {
            borrow_global<DKGState>(@aptos_framework).in_progress
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_dkg_session_dealer_epoch"></a>

## Function `session_dealer_epoch`

Return the dealer epoch of a `DKGSessionState`.


```move
module 0x1::dkg {
    public fun session_dealer_epoch(session: &dkg::DKGSessionState): u64
}
```


##### Implementation


```move
module 0x1::dkg {
    public fun session_dealer_epoch(session: &DKGSessionState): u64 {
        session.metadata.dealer_epoch
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::dkg {
    invariant [suspendable] chain_status::is_operating() ==> exists<DKGState>(@aptos_framework);
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::dkg {
    public fun initialize(aptos_framework: &signer)
}
```



```move
module 0x1::dkg {
    let aptos_framework_addr = signer::address_of(aptos_framework);
    aborts_if aptos_framework_addr != @aptos_framework;
}
```


<a id="@Specification_1_start"></a>

### Function `start`


```move
module 0x1::dkg {
    public(friend) fun start(dealer_epoch: u64, randomness_config: randomness_config::RandomnessConfig, dealer_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>, target_validator_set: vector<validator_consensus_info::ValidatorConsensusInfo>)
}
```



```move
module 0x1::dkg {
    aborts_if !exists<DKGState>(@aptos_framework);
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
}
```


<a id="@Specification_1_finish"></a>

### Function `finish`


```move
module 0x1::dkg {
    public(friend) fun finish(transcript: vector<u8>)
}
```



```move
module 0x1::dkg {
    requires exists<DKGState>(@aptos_framework);
    requires option::is_some(global<DKGState>(@aptos_framework).in_progress);
    aborts_if false;
}
```



<a id="0x1_dkg_has_incomplete_session"></a>


```move
module 0x1::dkg {
    fun has_incomplete_session(): bool {
       if (exists<DKGState>(@aptos_framework)) {
           option::spec_is_some(global<DKGState>(@aptos_framework).in_progress)
       } else {
           false
       }
    }
}
```


<a id="@Specification_1_try_clear_incomplete_session"></a>

### Function `try_clear_incomplete_session`


```move
module 0x1::dkg {
    public fun try_clear_incomplete_session(fx: &signer)
}
```



```move
module 0x1::dkg {
    let addr = signer::address_of(fx);
    aborts_if addr != @aptos_framework;
}
```


<a id="@Specification_1_incomplete_session"></a>

### Function `incomplete_session`


```move
module 0x1::dkg {
    public fun incomplete_session(): option::Option<dkg::DKGSessionState>
}
```



```move
module 0x1::dkg {
    aborts_if false;
}
```
