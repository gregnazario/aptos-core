
<a id="0x1_block"></a>

# Module `0x1::block`

This module defines a struct storing the metadata of the block and new block events.


-  [Resource `BlockResource`](#0x1_block_BlockResource)
-  [Resource `CommitHistory`](#0x1_block_CommitHistory)
-  [Struct `NewBlockEvent`](#0x1_block_NewBlockEvent)
-  [Struct `UpdateEpochIntervalEvent`](#0x1_block_UpdateEpochIntervalEvent)
-  [Struct `NewBlock`](#0x1_block_NewBlock)
-  [Struct `UpdateEpochInterval`](#0x1_block_UpdateEpochInterval)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_block_initialize)
-  [Function `initialize_commit_history`](#0x1_block_initialize_commit_history)
-  [Function `update_epoch_interval_microsecs`](#0x1_block_update_epoch_interval_microsecs)
-  [Function `get_epoch_interval_secs`](#0x1_block_get_epoch_interval_secs)
-  [Function `block_prologue_common`](#0x1_block_block_prologue_common)
-  [Function `block_prologue`](#0x1_block_block_prologue)
-  [Function `block_prologue_ext`](#0x1_block_block_prologue_ext)
-  [Function `get_current_block_height`](#0x1_block_get_current_block_height)
-  [Function `emit_new_block_event`](#0x1_block_emit_new_block_event)
-  [Function `emit_genesis_block_event`](#0x1_block_emit_genesis_block_event)
-  [Function `emit_writeset_block_event`](#0x1_block_emit_writeset_block_event)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Resource `BlockResource`](#@Specification_1_BlockResource)
    -  [Resource `CommitHistory`](#@Specification_1_CommitHistory)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `update_epoch_interval_microsecs`](#@Specification_1_update_epoch_interval_microsecs)
    -  [Function `get_epoch_interval_secs`](#@Specification_1_get_epoch_interval_secs)
    -  [Function `block_prologue_common`](#@Specification_1_block_prologue_common)
    -  [Function `block_prologue`](#@Specification_1_block_prologue)
    -  [Function `block_prologue_ext`](#@Specification_1_block_prologue_ext)
    -  [Function `get_current_block_height`](#@Specification_1_get_current_block_height)
    -  [Function `emit_new_block_event`](#@Specification_1_emit_new_block_event)
    -  [Function `emit_genesis_block_event`](#@Specification_1_emit_genesis_block_event)
    -  [Function `emit_writeset_block_event`](#@Specification_1_emit_writeset_block_event)


```move
module 0x1::block {
    use 0x1::account;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::option;
    use 0x1::randomness;
    use 0x1::reconfiguration;
    use 0x1::reconfiguration_with_dkg;
    use 0x1::stake;
    use 0x1::state_storage;
    use 0x1::system_addresses;
    use 0x1::table_with_length;
    use 0x1::timestamp;
    use 0x1::transaction_fee;
}
```


<a id="0x1_block_BlockResource"></a>

## Resource `BlockResource`

Should be in&#45;sync with BlockResource rust struct in new_block.rs


```move
module 0x1::block {
    struct BlockResource has key
}
```


##### Fields


<dl>
<dt>
`height: u64`
</dt>
<dd>
 Height of the current block
</dd>
<dt>
`epoch_interval: u64`
</dt>
<dd>
 Time period between epochs.
</dd>
<dt>
`new_block_events: event::EventHandle<block::NewBlockEvent>`
</dt>
<dd>
 Handle where events with the time of new blocks are emitted
</dd>
<dt>
`update_epoch_interval_events: event::EventHandle<block::UpdateEpochIntervalEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_block_CommitHistory"></a>

## Resource `CommitHistory`

Store new block events as a move resource, internally using a circular buffer.


```move
module 0x1::block {
    struct CommitHistory has key
}
```


##### Fields


<dl>
<dt>
`max_capacity: u32`
</dt>
<dd>

</dd>
<dt>
`next_idx: u32`
</dt>
<dd>

</dd>
<dt>
`table: table_with_length::TableWithLength<u32, block::NewBlockEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_block_NewBlockEvent"></a>

## Struct `NewBlockEvent`

Should be in&#45;sync with NewBlockEvent rust struct in new_block.rs


```move
module 0x1::block {
    struct NewBlockEvent has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`hash: address`
</dt>
<dd>

</dd>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
<dt>
`round: u64`
</dt>
<dd>

</dd>
<dt>
`height: u64`
</dt>
<dd>

</dd>
<dt>
`previous_block_votes_bitvec: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`proposer: address`
</dt>
<dd>

</dd>
<dt>
`failed_proposer_indices: vector<u64>`
</dt>
<dd>

</dd>
<dt>
`time_microseconds: u64`
</dt>
<dd>
 On&#45;chain time during the block at the given height
</dd>
</dl>


<a id="0x1_block_UpdateEpochIntervalEvent"></a>

## Struct `UpdateEpochIntervalEvent`

Event emitted when a proposal is created.


```move
module 0x1::block {
    struct UpdateEpochIntervalEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`old_epoch_interval: u64`
</dt>
<dd>

</dd>
<dt>
`new_epoch_interval: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_block_NewBlock"></a>

## Struct `NewBlock`

Should be in&#45;sync with NewBlockEvent rust struct in new_block.rs


```move
module 0x1::block {
    #[event]
    struct NewBlock has drop, store
}
```


##### Fields


<dl>
<dt>
`hash: address`
</dt>
<dd>

</dd>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
<dt>
`round: u64`
</dt>
<dd>

</dd>
<dt>
`height: u64`
</dt>
<dd>

</dd>
<dt>
`previous_block_votes_bitvec: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`proposer: address`
</dt>
<dd>

</dd>
<dt>
`failed_proposer_indices: vector<u64>`
</dt>
<dd>

</dd>
<dt>
`time_microseconds: u64`
</dt>
<dd>
 On&#45;chain time during the block at the given height
</dd>
</dl>


<a id="0x1_block_UpdateEpochInterval"></a>

## Struct `UpdateEpochInterval`

Event emitted when a proposal is created.


```move
module 0x1::block {
    #[event]
    struct UpdateEpochInterval has drop, store
}
```


##### Fields


<dl>
<dt>
`old_epoch_interval: u64`
</dt>
<dd>

</dd>
<dt>
`new_epoch_interval: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_block_MAX_U64"></a>



```move
module 0x1::block {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x1_block_EINVALID_PROPOSER"></a>

An invalid proposer was provided. Expected the proposer to be the VM or an active validator.


```move
module 0x1::block {
    const EINVALID_PROPOSER: u64 = 2;
}
```


<a id="0x1_block_ENUM_NEW_BLOCK_EVENTS_DOES_NOT_MATCH_BLOCK_HEIGHT"></a>

The number of new block events does not equal the current block height.


```move
module 0x1::block {
    const ENUM_NEW_BLOCK_EVENTS_DOES_NOT_MATCH_BLOCK_HEIGHT: u64 = 1;
}
```


<a id="0x1_block_EZERO_EPOCH_INTERVAL"></a>

Epoch interval cannot be 0.


```move
module 0x1::block {
    const EZERO_EPOCH_INTERVAL: u64 = 3;
}
```


<a id="0x1_block_EZERO_MAX_CAPACITY"></a>

The maximum capacity of the commit history cannot be 0.


```move
module 0x1::block {
    const EZERO_MAX_CAPACITY: u64 = 3;
}
```


<a id="0x1_block_initialize"></a>

## Function `initialize`

This can only be called during Genesis.


```move
module 0x1::block {
    public(friend) fun initialize(aptos_framework: &signer, epoch_interval_microsecs: u64)
}
```


##### Implementation


```move
module 0x1::block {
    public(friend) fun initialize(aptos_framework: &signer, epoch_interval_microsecs: u64) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(epoch_interval_microsecs > 0, error::invalid_argument(EZERO_EPOCH_INTERVAL));

        move_to<CommitHistory>(aptos_framework, CommitHistory {
            max_capacity: 2000,
            next_idx: 0,
            table: table_with_length::new(),
        });

        move_to<BlockResource>(
            aptos_framework,
            BlockResource {
                height: 0,
                epoch_interval: epoch_interval_microsecs,
                new_block_events: account::new_event_handle<NewBlockEvent>(aptos_framework),
                update_epoch_interval_events: account::new_event_handle<UpdateEpochIntervalEvent>(aptos_framework),
            }
        );
    }
}
```


<a id="0x1_block_initialize_commit_history"></a>

## Function `initialize_commit_history`

Initialize the commit history resource if it&apos;s not in genesis.


```move
module 0x1::block {
    public fun initialize_commit_history(fx: &signer, max_capacity: u32)
}
```


##### Implementation


```move
module 0x1::block {
    public fun initialize_commit_history(fx: &signer, max_capacity: u32) {
        assert!(max_capacity > 0, error::invalid_argument(EZERO_MAX_CAPACITY));
        move_to<CommitHistory>(fx, CommitHistory {
            max_capacity,
            next_idx: 0,
            table: table_with_length::new(),
        });
    }
}
```


<a id="0x1_block_update_epoch_interval_microsecs"></a>

## Function `update_epoch_interval_microsecs`

Update the epoch interval.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::block {
    public fun update_epoch_interval_microsecs(aptos_framework: &signer, new_epoch_interval: u64)
}
```


##### Implementation


```move
module 0x1::block {
    public fun update_epoch_interval_microsecs(
        aptos_framework: &signer,
        new_epoch_interval: u64,
    ) acquires BlockResource {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(new_epoch_interval > 0, error::invalid_argument(EZERO_EPOCH_INTERVAL));

        let block_resource = borrow_global_mut<BlockResource>(@aptos_framework);
        let old_epoch_interval = block_resource.epoch_interval;
        block_resource.epoch_interval = new_epoch_interval;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                UpdateEpochInterval { old_epoch_interval, new_epoch_interval },
            );
        };
        event::emit_event<UpdateEpochIntervalEvent>(
            &mut block_resource.update_epoch_interval_events,
            UpdateEpochIntervalEvent { old_epoch_interval, new_epoch_interval },
        );
    }
}
```


<a id="0x1_block_get_epoch_interval_secs"></a>

## Function `get_epoch_interval_secs`

Return epoch interval in seconds.


```move
module 0x1::block {
    #[view]
    public fun get_epoch_interval_secs(): u64
}
```


##### Implementation


```move
module 0x1::block {
    public fun get_epoch_interval_secs(): u64 acquires BlockResource {
        borrow_global<BlockResource>(@aptos_framework).epoch_interval / 1000000
    }
}
```


<a id="0x1_block_block_prologue_common"></a>

## Function `block_prologue_common`



```move
module 0x1::block {
    fun block_prologue_common(vm: &signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64): u64
}
```


##### Implementation


```move
module 0x1::block {
    fun block_prologue_common(
        vm: &signer,
        hash: address,
        epoch: u64,
        round: u64,
        proposer: address,
        failed_proposer_indices: vector<u64>,
        previous_block_votes_bitvec: vector<u8>,
        timestamp: u64
    ): u64 acquires BlockResource, CommitHistory {
        // Operational constraint: can only be invoked by the VM.
        system_addresses::assert_vm(vm);

        // Blocks can only be produced by a valid proposer or by the VM itself for Nil blocks (no user txs).
        assert!(
            proposer == @vm_reserved || stake::is_current_epoch_validator(proposer),
            error::permission_denied(EINVALID_PROPOSER),
        );

        let proposer_index = option::none();
        if (proposer != @vm_reserved) {
            proposer_index = option::some(stake::get_validator_index(proposer));
        };

        let block_metadata_ref = borrow_global_mut<BlockResource>(@aptos_framework);
        block_metadata_ref.height = event::counter(&block_metadata_ref.new_block_events);

        // Emit both event v1 and v2 for compatibility. Eventually only module events will be kept.
        let new_block_event = NewBlockEvent {
            hash,
            epoch,
            round,
            height: block_metadata_ref.height,
            previous_block_votes_bitvec,
            proposer,
            failed_proposer_indices,
            time_microseconds: timestamp,
        };
        let new_block_event_v2 = NewBlock {
            hash,
            epoch,
            round,
            height: block_metadata_ref.height,
            previous_block_votes_bitvec,
            proposer,
            failed_proposer_indices,
            time_microseconds: timestamp,
        };
        emit_new_block_event(vm, &mut block_metadata_ref.new_block_events, new_block_event, new_block_event_v2);

        if (features::collect_and_distribute_gas_fees()) {
            // Assign the fees collected from the previous block to the previous block proposer.
            // If for any reason the fees cannot be assigned, this function burns the collected coins.
            transaction_fee::process_collected_fees();
            // Set the proposer of this block as the receiver of the fees, so that the fees for this
            // block are assigned to the right account.
            transaction_fee::register_proposer_for_fee_collection(proposer);
        };

        // Performance scores have to be updated before the epoch transition as the transaction that triggers the
        // transition is the last block in the previous epoch.
        stake::update_performance_statistics(proposer_index, failed_proposer_indices);
        state_storage::on_new_block(reconfiguration::current_epoch());

        block_metadata_ref.epoch_interval
    }
}
```


<a id="0x1_block_block_prologue"></a>

## Function `block_prologue`

Set the metadata for the current block.
The runtime always runs this before executing the transactions in a block.


```move
module 0x1::block {
    fun block_prologue(vm: signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64)
}
```


##### Implementation


```move
module 0x1::block {
    fun block_prologue(
        vm: signer,
        hash: address,
        epoch: u64,
        round: u64,
        proposer: address,
        failed_proposer_indices: vector<u64>,
        previous_block_votes_bitvec: vector<u8>,
        timestamp: u64
    ) acquires BlockResource, CommitHistory {
        let epoch_interval = block_prologue_common(&vm, hash, epoch, round, proposer, failed_proposer_indices, previous_block_votes_bitvec, timestamp);
        randomness::on_new_block(&vm, epoch, round, option::none());
        if (timestamp - reconfiguration::last_reconfiguration_time() >= epoch_interval) {
            reconfiguration::reconfigure();
        };
    }
}
```


<a id="0x1_block_block_prologue_ext"></a>

## Function `block_prologue_ext`

`block_prologue()` but trigger reconfiguration with DKG after epoch timed out.


```move
module 0x1::block {
    fun block_prologue_ext(vm: signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64, randomness_seed: option::Option<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::block {
    fun block_prologue_ext(
        vm: signer,
        hash: address,
        epoch: u64,
        round: u64,
        proposer: address,
        failed_proposer_indices: vector<u64>,
        previous_block_votes_bitvec: vector<u8>,
        timestamp: u64,
        randomness_seed: Option<vector<u8>>,
    ) acquires BlockResource, CommitHistory {
        let epoch_interval = block_prologue_common(
            &vm,
            hash,
            epoch,
            round,
            proposer,
            failed_proposer_indices,
            previous_block_votes_bitvec,
            timestamp
        );
        randomness::on_new_block(&vm, epoch, round, randomness_seed);

        if (timestamp - reconfiguration::last_reconfiguration_time() >= epoch_interval) {
            reconfiguration_with_dkg::try_start();
        };
    }
}
```


<a id="0x1_block_get_current_block_height"></a>

## Function `get_current_block_height`

Get the current block height


```move
module 0x1::block {
    #[view]
    public fun get_current_block_height(): u64
}
```


##### Implementation


```move
module 0x1::block {
    public fun get_current_block_height(): u64 acquires BlockResource {
        borrow_global<BlockResource>(@aptos_framework).height
    }
}
```


<a id="0x1_block_emit_new_block_event"></a>

## Function `emit_new_block_event`

Emit the event and update height and global timestamp


```move
module 0x1::block {
    fun emit_new_block_event(vm: &signer, event_handle: &mut event::EventHandle<block::NewBlockEvent>, new_block_event: block::NewBlockEvent, new_block_event_v2: block::NewBlock)
}
```


##### Implementation


```move
module 0x1::block {
    fun emit_new_block_event(
        vm: &signer,
        event_handle: &mut EventHandle<NewBlockEvent>,
        new_block_event: NewBlockEvent,
        new_block_event_v2: NewBlock
    ) acquires CommitHistory {
        if (exists<CommitHistory>(@aptos_framework)) {
            let commit_history_ref = borrow_global_mut<CommitHistory>(@aptos_framework);
            let idx = commit_history_ref.next_idx;
            if (table_with_length::contains(&commit_history_ref.table, idx)) {
                table_with_length::remove(&mut commit_history_ref.table, idx);
            };
            table_with_length::add(&mut commit_history_ref.table, idx, copy new_block_event);
            spec {
                assume idx + 1 <= MAX_U32;
            };
            commit_history_ref.next_idx = (idx + 1) % commit_history_ref.max_capacity;
        };
        timestamp::update_global_time(vm, new_block_event.proposer, new_block_event.time_microseconds);
        assert!(
            event::counter(event_handle) == new_block_event.height,
            error::invalid_argument(ENUM_NEW_BLOCK_EVENTS_DOES_NOT_MATCH_BLOCK_HEIGHT),
        );
        if (std::features::module_event_migration_enabled()) {
            event::emit(new_block_event_v2);
        };
        event::emit_event<NewBlockEvent>(event_handle, new_block_event);
    }
}
```


<a id="0x1_block_emit_genesis_block_event"></a>

## Function `emit_genesis_block_event`

Emit a `NewBlockEvent` event. This function will be invoked by genesis directly to generate the very first
reconfiguration event.


```move
module 0x1::block {
    fun emit_genesis_block_event(vm: signer)
}
```


##### Implementation


```move
module 0x1::block {
    fun emit_genesis_block_event(vm: signer) acquires BlockResource, CommitHistory {
        let block_metadata_ref = borrow_global_mut<BlockResource>(@aptos_framework);
        let genesis_id = @0x0;
        emit_new_block_event(
            &vm,
            &mut block_metadata_ref.new_block_events,
            NewBlockEvent {
                hash: genesis_id,
                epoch: 0,
                round: 0,
                height: 0,
                previous_block_votes_bitvec: vector::empty(),
                proposer: @vm_reserved,
                failed_proposer_indices: vector::empty(),
                time_microseconds: 0,
            },
            NewBlock {
                hash: genesis_id,
                epoch: 0,
                round: 0,
                height: 0,
                previous_block_votes_bitvec: vector::empty(),
                proposer: @vm_reserved,
                failed_proposer_indices: vector::empty(),
                time_microseconds: 0,
            }
        );
    }
}
```


<a id="0x1_block_emit_writeset_block_event"></a>

## Function `emit_writeset_block_event`

Emit a `NewBlockEvent` event. This function will be invoked by write set script directly to generate the
new block event for WriteSetPayload.


```move
module 0x1::block {
    public fun emit_writeset_block_event(vm_signer: &signer, fake_block_hash: address)
}
```


##### Implementation


```move
module 0x1::block {
    public fun emit_writeset_block_event(vm_signer: &signer, fake_block_hash: address) acquires BlockResource, CommitHistory {
        system_addresses::assert_vm(vm_signer);
        let block_metadata_ref = borrow_global_mut<BlockResource>(@aptos_framework);
        block_metadata_ref.height = event::counter(&block_metadata_ref.new_block_events);

        emit_new_block_event(
            vm_signer,
            &mut block_metadata_ref.new_block_events,
            NewBlockEvent {
                hash: fake_block_hash,
                epoch: reconfiguration::current_epoch(),
                round: MAX_U64,
                height: block_metadata_ref.height,
                previous_block_votes_bitvec: vector::empty(),
                proposer: @vm_reserved,
                failed_proposer_indices: vector::empty(),
                time_microseconds: timestamp::now_microseconds(),
            },
            NewBlock {
                hash: fake_block_hash,
                epoch: reconfiguration::current_epoch(),
                round: MAX_U64,
                height: block_metadata_ref.height,
                previous_block_votes_bitvec: vector::empty(),
                proposer: @vm_reserved,
                failed_proposer_indices: vector::empty(),
                time_microseconds: timestamp::now_microseconds(),
            }
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
<td>During the module&apos;s initialization, it guarantees that the BlockResource resource moves under the Aptos framework account with initial values.</td>
<td>High</td>
<td>The initialize function is responsible for setting up the initial state of the module, ensuring that the following conditions are met (1) the BlockResource resource is created, indicating its existence within the module&apos;s context, and moved under the Aptos framework account, (2) the block height is set to zero during initialization, and (3) the epoch interval is greater than zero.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;1](Initialize).</td>
</tr>

<tr>
<td>2</td>
<td>Only the Aptos framework address may execute the following functionalities: (1) initialize BlockResource, and (2) update the epoch interval.</td>
<td>Critical</td>
<td>The initialize and  update_epoch_interval_microsecs functions ensure that only aptos_framework can call them.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;2.1](Initialize) and [#high&#45;level&#45;req&#45;2.2](update_epoch_interval_microsecs).</td>
</tr>

<tr>
<td>3</td>
<td>When updating the epoch interval, its value must be greater than zero and BlockResource must exist.</td>
<td>High</td>
<td>The update_epoch_interval_microsecs function asserts that new_epoch_interval is greater than zero and updates BlockResource&apos;s state.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3.1](UpdateEpochIntervalMicrosecs) and [#high&#45;level&#45;req&#45;3.2](epoch_interval).</td>
</tr>

<tr>
<td>4</td>
<td>Only a valid proposer or the virtual machine is authorized to produce blocks.</td>
<td>Critical</td>
<td>During the execution of the block_prologue function, the validity of the proposer address is verified when setting the metadata for the current block.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;4](block_prologue).</td>
</tr>

<tr>
<td>5</td>
<td>While emitting a new block event, the number of them is equal to the current block height.</td>
<td>Medium</td>
<td>The emit_new_block_event function asserts that the number of new block events equals the current block height.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;5](emit_new_block_event).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::block {
    invariant [suspendable] chain_status::is_operating() ==> exists<BlockResource>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==> exists<CommitHistory>(@aptos_framework);
}
```


<a id="@Specification_1_BlockResource"></a>

### Resource `BlockResource`


```move
module 0x1::block {
    struct BlockResource has key
}
```


<dl>
<dt>
`height: u64`
</dt>
<dd>
 Height of the current block
</dd>
<dt>
`epoch_interval: u64`
</dt>
<dd>
 Time period between epochs.
</dd>
<dt>
`new_block_events: event::EventHandle<block::NewBlockEvent>`
</dt>
<dd>
 Handle where events with the time of new blocks are emitted
</dd>
<dt>
`update_epoch_interval_events: event::EventHandle<block::UpdateEpochIntervalEvent>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::block {
// This enforces ### high&#45;level&#45;req&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3):
    invariant epoch_interval > 0;
}
```


<a id="@Specification_1_CommitHistory"></a>

### Resource `CommitHistory`


```move
module 0x1::block {
    struct CommitHistory has key
}
```


<dl>
<dt>
`max_capacity: u32`
</dt>
<dd>

</dd>
<dt>
`next_idx: u32`
</dt>
<dd>

</dd>
<dt>
`table: table_with_length::TableWithLength<u32, block::NewBlockEvent>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::block {
    invariant max_capacity > 0;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::block {
    public(friend) fun initialize(aptos_framework: &signer, epoch_interval_microsecs: u64)
}
```

The caller is aptos_framework.
The new_epoch_interval must be greater than 0.
The BlockResource is not under the caller before initializing.
The Account is not under the caller until the BlockResource is created for the caller.
Make sure The BlockResource under the caller existed after initializing.
The number of new events created does not exceed MAX_U64.


```move
module 0x1::block {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    include Initialize;
    include NewEventHandle;
    let addr = signer::address_of(aptos_framework);
    let account = global<account::Account>(addr);
    aborts_if account.guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
}
```


<a id="@Specification_1_update_epoch_interval_microsecs"></a>

### Function `update_epoch_interval_microsecs`


```move
module 0x1::block {
    public fun update_epoch_interval_microsecs(aptos_framework: &signer, new_epoch_interval: u64)
}
```

The caller is @aptos_framework.
The new_epoch_interval must be greater than 0.
The BlockResource existed under the @aptos_framework.


```move
module 0x1::block {
// This enforces ### high&#45;level&#45;req&#45;3.1
[#high&#45;level&#45;req](high&#45;level requirement 3):
    include UpdateEpochIntervalMicrosecs;
}
```



<a id="0x1_block_UpdateEpochIntervalMicrosecs"></a>


```move
module 0x1::block {
    schema UpdateEpochIntervalMicrosecs {
        aptos_framework: signer;
        new_epoch_interval: u64;
        let addr = signer::address_of(aptos_framework);
    // This enforces ### high&#45;level&#45;req&#45;2.2
    [#high&#45;level&#45;req](high&#45;level requirement 2):
        aborts_if addr != @aptos_framework;
        aborts_if new_epoch_interval == 0;
        aborts_if !exists<BlockResource>(addr);
        let post block_resource = global<BlockResource>(addr);
        ensures block_resource.epoch_interval == new_epoch_interval;
    }
}
```


<a id="@Specification_1_get_epoch_interval_secs"></a>

### Function `get_epoch_interval_secs`


```move
module 0x1::block {
    #[view]
    public fun get_epoch_interval_secs(): u64
}
```



```move
module 0x1::block {
    aborts_if !exists<BlockResource>(@aptos_framework);
}
```


<a id="@Specification_1_block_prologue_common"></a>

### Function `block_prologue_common`


```move
module 0x1::block {
    fun block_prologue_common(vm: &signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64): u64
}
```



```move
module 0x1::block {
    pragma verify_duration_estimate = 1000;
    include BlockRequirement;
    aborts_if false;
}
```


<a id="@Specification_1_block_prologue"></a>

### Function `block_prologue`


```move
module 0x1::block {
    fun block_prologue(vm: signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64)
}
```



```move
module 0x1::block {
    pragma verify_duration_estimate = 1000;
    requires timestamp >= reconfiguration::last_reconfiguration_time();
    include BlockRequirement;
    aborts_if false;
}
```


<a id="@Specification_1_block_prologue_ext"></a>

### Function `block_prologue_ext`


```move
module 0x1::block {
    fun block_prologue_ext(vm: signer, hash: address, epoch: u64, round: u64, proposer: address, failed_proposer_indices: vector<u64>, previous_block_votes_bitvec: vector<u8>, timestamp: u64, randomness_seed: option::Option<vector<u8>>)
}
```



```move
module 0x1::block {
    pragma verify_duration_estimate = 1000;
    requires timestamp >= reconfiguration::last_reconfiguration_time();
    include BlockRequirement;
    include stake::ResourceRequirement;
    include stake::GetReconfigStartTimeRequirement;
    aborts_if false;
}
```


<a id="@Specification_1_get_current_block_height"></a>

### Function `get_current_block_height`


```move
module 0x1::block {
    #[view]
    public fun get_current_block_height(): u64
}
```



```move
module 0x1::block {
    aborts_if !exists<BlockResource>(@aptos_framework);
}
```


<a id="@Specification_1_emit_new_block_event"></a>

### Function `emit_new_block_event`


```move
module 0x1::block {
    fun emit_new_block_event(vm: &signer, event_handle: &mut event::EventHandle<block::NewBlockEvent>, new_block_event: block::NewBlockEvent, new_block_event_v2: block::NewBlock)
}
```



```move
module 0x1::block {
    let proposer = new_block_event.proposer;
    let timestamp = new_block_event.time_microseconds;
    requires chain_status::is_operating();
    requires system_addresses::is_vm(vm);
    requires (proposer == @vm_reserved) ==> (timestamp::spec_now_microseconds() == timestamp);
    requires (proposer != @vm_reserved) ==> (timestamp::spec_now_microseconds() < timestamp);
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    requires event::counter(event_handle) == new_block_event.height;
    aborts_if false;
}
```


<a id="@Specification_1_emit_genesis_block_event"></a>

### Function `emit_genesis_block_event`


```move
module 0x1::block {
    fun emit_genesis_block_event(vm: signer)
}
```



```move
module 0x1::block {
    requires chain_status::is_operating();
    requires system_addresses::is_vm(vm);
    requires event::counter(global<BlockResource>(@aptos_framework).new_block_events) == 0;
    requires (timestamp::spec_now_microseconds() == 0);
    aborts_if false;
}
```


<a id="@Specification_1_emit_writeset_block_event"></a>

### Function `emit_writeset_block_event`


```move
module 0x1::block {
    public fun emit_writeset_block_event(vm_signer: &signer, fake_block_hash: address)
}
```

The caller is @vm_reserved.
The BlockResource existed under the @aptos_framework.
The Configuration existed under the @aptos_framework.
The CurrentTimeMicroseconds existed under the @aptos_framework.


```move
module 0x1::block {
    requires chain_status::is_operating();
    include EmitWritesetBlockEvent;
}
```



<a id="0x1_block_EmitWritesetBlockEvent"></a>


```move
module 0x1::block {
    schema EmitWritesetBlockEvent {
        vm_signer: signer;
        let addr = signer::address_of(vm_signer);
        aborts_if addr != @vm_reserved;
        aborts_if !exists<BlockResource>(@aptos_framework);
        aborts_if !exists<reconfiguration::Configuration>(@aptos_framework);
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    }
}
```
