
<a id="0x1_aptos_governance"></a>

# Module `0x1::aptos_governance`


AptosGovernance represents the on&#45;chain governance of the Aptos network. Voting power is calculated based on the
current epoch&apos;s voting power of the proposer or voter&apos;s backing stake pool. In addition, for it to count,
the stake pool&apos;s lockup needs to be at least as long as the proposal&apos;s duration.

It provides the following flow:
1. Proposers can create a proposal by calling AptosGovernance::create_proposal. The proposer&apos;s backing stake pool
needs to have the minimum proposer stake required. Off&#45;chain components can subscribe to CreateProposalEvent to
track proposal creation and proposal ids.
2. Voters can vote on a proposal. Their voting power is derived from the backing stake pool. A stake pool can vote
on a proposal multiple times as long as the total voting power of these votes doesn&apos;t exceed its total voting power.


-  [Resource `GovernanceResponsbility`](#0x1_aptos_governance_GovernanceResponsbility)
-  [Resource `GovernanceConfig`](#0x1_aptos_governance_GovernanceConfig)
-  [Struct `RecordKey`](#0x1_aptos_governance_RecordKey)
-  [Resource `VotingRecords`](#0x1_aptos_governance_VotingRecords)
-  [Resource `VotingRecordsV2`](#0x1_aptos_governance_VotingRecordsV2)
-  [Resource `ApprovedExecutionHashes`](#0x1_aptos_governance_ApprovedExecutionHashes)
-  [Resource `GovernanceEvents`](#0x1_aptos_governance_GovernanceEvents)
-  [Struct `CreateProposalEvent`](#0x1_aptos_governance_CreateProposalEvent)
-  [Struct `VoteEvent`](#0x1_aptos_governance_VoteEvent)
-  [Struct `UpdateConfigEvent`](#0x1_aptos_governance_UpdateConfigEvent)
-  [Struct `CreateProposal`](#0x1_aptos_governance_CreateProposal)
-  [Struct `Vote`](#0x1_aptos_governance_Vote)
-  [Struct `UpdateConfig`](#0x1_aptos_governance_UpdateConfig)
-  [Constants](#@Constants_0)
-  [Function `store_signer_cap`](#0x1_aptos_governance_store_signer_cap)
-  [Function `initialize`](#0x1_aptos_governance_initialize)
-  [Function `update_governance_config`](#0x1_aptos_governance_update_governance_config)
-  [Function `initialize_partial_voting`](#0x1_aptos_governance_initialize_partial_voting)
-  [Function `get_voting_duration_secs`](#0x1_aptos_governance_get_voting_duration_secs)
-  [Function `get_min_voting_threshold`](#0x1_aptos_governance_get_min_voting_threshold)
-  [Function `get_required_proposer_stake`](#0x1_aptos_governance_get_required_proposer_stake)
-  [Function `has_entirely_voted`](#0x1_aptos_governance_has_entirely_voted)
-  [Function `get_remaining_voting_power`](#0x1_aptos_governance_get_remaining_voting_power)
-  [Function `create_proposal`](#0x1_aptos_governance_create_proposal)
-  [Function `create_proposal_v2`](#0x1_aptos_governance_create_proposal_v2)
-  [Function `create_proposal_v2_impl`](#0x1_aptos_governance_create_proposal_v2_impl)
-  [Function `batch_vote`](#0x1_aptos_governance_batch_vote)
-  [Function `batch_partial_vote`](#0x1_aptos_governance_batch_partial_vote)
-  [Function `vote`](#0x1_aptos_governance_vote)
-  [Function `partial_vote`](#0x1_aptos_governance_partial_vote)
-  [Function `vote_internal`](#0x1_aptos_governance_vote_internal)
-  [Function `add_approved_script_hash_script`](#0x1_aptos_governance_add_approved_script_hash_script)
-  [Function `add_approved_script_hash`](#0x1_aptos_governance_add_approved_script_hash)
-  [Function `resolve`](#0x1_aptos_governance_resolve)
-  [Function `resolve_multi_step_proposal`](#0x1_aptos_governance_resolve_multi_step_proposal)
-  [Function `remove_approved_hash`](#0x1_aptos_governance_remove_approved_hash)
-  [Function `reconfigure`](#0x1_aptos_governance_reconfigure)
-  [Function `force_end_epoch`](#0x1_aptos_governance_force_end_epoch)
-  [Function `force_end_epoch_test_only`](#0x1_aptos_governance_force_end_epoch_test_only)
-  [Function `toggle_features`](#0x1_aptos_governance_toggle_features)
-  [Function `get_signer_testnet_only`](#0x1_aptos_governance_get_signer_testnet_only)
-  [Function `get_voting_power`](#0x1_aptos_governance_get_voting_power)
-  [Function `get_signer`](#0x1_aptos_governance_get_signer)
-  [Function `create_proposal_metadata`](#0x1_aptos_governance_create_proposal_metadata)
-  [Function `assert_voting_initialization`](#0x1_aptos_governance_assert_voting_initialization)
-  [Function `initialize_for_verification`](#0x1_aptos_governance_initialize_for_verification)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `store_signer_cap`](#@Specification_1_store_signer_cap)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `update_governance_config`](#@Specification_1_update_governance_config)
    -  [Function `initialize_partial_voting`](#@Specification_1_initialize_partial_voting)
    -  [Function `get_voting_duration_secs`](#@Specification_1_get_voting_duration_secs)
    -  [Function `get_min_voting_threshold`](#@Specification_1_get_min_voting_threshold)
    -  [Function `get_required_proposer_stake`](#@Specification_1_get_required_proposer_stake)
    -  [Function `has_entirely_voted`](#@Specification_1_has_entirely_voted)
    -  [Function `get_remaining_voting_power`](#@Specification_1_get_remaining_voting_power)
    -  [Function `create_proposal`](#@Specification_1_create_proposal)
    -  [Function `create_proposal_v2`](#@Specification_1_create_proposal_v2)
    -  [Function `create_proposal_v2_impl`](#@Specification_1_create_proposal_v2_impl)
    -  [Function `batch_vote`](#@Specification_1_batch_vote)
    -  [Function `batch_partial_vote`](#@Specification_1_batch_partial_vote)
    -  [Function `vote`](#@Specification_1_vote)
    -  [Function `partial_vote`](#@Specification_1_partial_vote)
    -  [Function `vote_internal`](#@Specification_1_vote_internal)
    -  [Function `add_approved_script_hash_script`](#@Specification_1_add_approved_script_hash_script)
    -  [Function `add_approved_script_hash`](#@Specification_1_add_approved_script_hash)
    -  [Function `resolve`](#@Specification_1_resolve)
    -  [Function `resolve_multi_step_proposal`](#@Specification_1_resolve_multi_step_proposal)
    -  [Function `remove_approved_hash`](#@Specification_1_remove_approved_hash)
    -  [Function `reconfigure`](#@Specification_1_reconfigure)
    -  [Function `force_end_epoch`](#@Specification_1_force_end_epoch)
    -  [Function `force_end_epoch_test_only`](#@Specification_1_force_end_epoch_test_only)
    -  [Function `toggle_features`](#@Specification_1_toggle_features)
    -  [Function `get_signer_testnet_only`](#@Specification_1_get_signer_testnet_only)
    -  [Function `get_voting_power`](#@Specification_1_get_voting_power)
    -  [Function `get_signer`](#@Specification_1_get_signer)
    -  [Function `create_proposal_metadata`](#@Specification_1_create_proposal_metadata)
    -  [Function `assert_voting_initialization`](#@Specification_1_assert_voting_initialization)
    -  [Function `initialize_for_verification`](#@Specification_1_initialize_for_verification)


```move
module 0x1::aptos_governance {
    use 0x1::account;
    use 0x1::aptos_coin;
    use 0x1::coin;
    use 0x1::consensus_config;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::governance_proposal;
    use 0x1::math64;
    use 0x1::option;
    use 0x1::randomness_config;
    use 0x1::reconfiguration_with_dkg;
    use 0x1::signer;
    use 0x1::simple_map;
    use 0x1::smart_table;
    use 0x1::stake;
    use 0x1::staking_config;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::table;
    use 0x1::timestamp;
    use 0x1::vector;
    use 0x1::voting;
}
```


<a id="0x1_aptos_governance_GovernanceResponsbility"></a>

## Resource `GovernanceResponsbility`

Store the SignerCapabilities of accounts under the on&#45;chain governance&apos;s control.


```move
module 0x1::aptos_governance {
    struct GovernanceResponsbility has key
}
```


##### Fields


<dl>
<dt>
`signer_caps: simple_map::SimpleMap<address, account::SignerCapability>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_GovernanceConfig"></a>

## Resource `GovernanceConfig`

Configurations of the AptosGovernance, set during Genesis and can be updated by the same process offered
by this AptosGovernance module.


```move
module 0x1::aptos_governance {
    struct GovernanceConfig has key
}
```


##### Fields


<dl>
<dt>
`min_voting_threshold: u128`
</dt>
<dd>

</dd>
<dt>
`required_proposer_stake: u64`
</dt>
<dd>

</dd>
<dt>
`voting_duration_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_RecordKey"></a>

## Struct `RecordKey`



```move
module 0x1::aptos_governance {
    struct RecordKey has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`stake_pool: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_VotingRecords"></a>

## Resource `VotingRecords`

Records to track the proposals each stake pool has been used to vote on.


```move
module 0x1::aptos_governance {
    struct VotingRecords has key
}
```


##### Fields


<dl>
<dt>
`votes: table::Table<aptos_governance::RecordKey, bool>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_VotingRecordsV2"></a>

## Resource `VotingRecordsV2`

Records to track the voting power usage of each stake pool on each proposal.


```move
module 0x1::aptos_governance {
    struct VotingRecordsV2 has key
}
```


##### Fields


<dl>
<dt>
`votes: smart_table::SmartTable<aptos_governance::RecordKey, u64>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_ApprovedExecutionHashes"></a>

## Resource `ApprovedExecutionHashes`

Used to track which execution script hashes have been approved by governance.
This is required to bypass cases where the execution scripts exceed the size limit imposed by mempool.


```move
module 0x1::aptos_governance {
    struct ApprovedExecutionHashes has key
}
```


##### Fields


<dl>
<dt>
`hashes: simple_map::SimpleMap<u64, vector<u8>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_GovernanceEvents"></a>

## Resource `GovernanceEvents`

Events generated by interactions with the AptosGovernance module.


```move
module 0x1::aptos_governance {
    struct GovernanceEvents has key
}
```


##### Fields


<dl>
<dt>
`create_proposal_events: event::EventHandle<aptos_governance::CreateProposalEvent>`
</dt>
<dd>

</dd>
<dt>
`update_config_events: event::EventHandle<aptos_governance::UpdateConfigEvent>`
</dt>
<dd>

</dd>
<dt>
`vote_events: event::EventHandle<aptos_governance::VoteEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_CreateProposalEvent"></a>

## Struct `CreateProposalEvent`

Event emitted when a proposal is created.


```move
module 0x1::aptos_governance {
    struct CreateProposalEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`proposer: address`
</dt>
<dd>

</dd>
<dt>
`stake_pool: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`execution_hash: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`proposal_metadata: simple_map::SimpleMap<string::String, vector<u8>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_VoteEvent"></a>

## Struct `VoteEvent`

Event emitted when there&apos;s a vote on a proposa;


```move
module 0x1::aptos_governance {
    struct VoteEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`stake_pool: address`
</dt>
<dd>

</dd>
<dt>
`num_votes: u64`
</dt>
<dd>

</dd>
<dt>
`should_pass: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_UpdateConfigEvent"></a>

## Struct `UpdateConfigEvent`

Event emitted when the governance configs are updated.


```move
module 0x1::aptos_governance {
    struct UpdateConfigEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`min_voting_threshold: u128`
</dt>
<dd>

</dd>
<dt>
`required_proposer_stake: u64`
</dt>
<dd>

</dd>
<dt>
`voting_duration_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_CreateProposal"></a>

## Struct `CreateProposal`

Event emitted when a proposal is created.


```move
module 0x1::aptos_governance {
    #[event]
    struct CreateProposal has drop, store
}
```


##### Fields


<dl>
<dt>
`proposer: address`
</dt>
<dd>

</dd>
<dt>
`stake_pool: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`execution_hash: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`proposal_metadata: simple_map::SimpleMap<string::String, vector<u8>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_Vote"></a>

## Struct `Vote`

Event emitted when there&apos;s a vote on a proposa;


```move
module 0x1::aptos_governance {
    #[event]
    struct Vote has drop, store
}
```


##### Fields


<dl>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`stake_pool: address`
</dt>
<dd>

</dd>
<dt>
`num_votes: u64`
</dt>
<dd>

</dd>
<dt>
`should_pass: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_governance_UpdateConfig"></a>

## Struct `UpdateConfig`

Event emitted when the governance configs are updated.


```move
module 0x1::aptos_governance {
    #[event]
    struct UpdateConfig has drop, store
}
```


##### Fields


<dl>
<dt>
`min_voting_threshold: u128`
</dt>
<dd>

</dd>
<dt>
`required_proposer_stake: u64`
</dt>
<dd>

</dd>
<dt>
`voting_duration_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aptos_governance_MAX_U64"></a>



```move
module 0x1::aptos_governance {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x1_aptos_governance_PROPOSAL_STATE_SUCCEEDED"></a>

This matches the same enum const in voting. We have to duplicate it as Move doesn&apos;t have support for enums yet.


```move
module 0x1::aptos_governance {
    const PROPOSAL_STATE_SUCCEEDED: u64 = 1;
}
```


<a id="0x1_aptos_governance_EALREADY_VOTED"></a>

The specified stake pool has already been used to vote on the same proposal


```move
module 0x1::aptos_governance {
    const EALREADY_VOTED: u64 = 4;
}
```


<a id="0x1_aptos_governance_EINSUFFICIENT_PROPOSER_STAKE"></a>

The specified stake pool does not have sufficient stake to create a proposal


```move
module 0x1::aptos_governance {
    const EINSUFFICIENT_PROPOSER_STAKE: u64 = 1;
}
```


<a id="0x1_aptos_governance_EINSUFFICIENT_STAKE_LOCKUP"></a>

The specified stake pool does not have long enough remaining lockup to create a proposal or vote


```move
module 0x1::aptos_governance {
    const EINSUFFICIENT_STAKE_LOCKUP: u64 = 3;
}
```


<a id="0x1_aptos_governance_EMETADATA_HASH_TOO_LONG"></a>

Metadata hash cannot be longer than 256 chars


```move
module 0x1::aptos_governance {
    const EMETADATA_HASH_TOO_LONG: u64 = 10;
}
```


<a id="0x1_aptos_governance_EMETADATA_LOCATION_TOO_LONG"></a>

Metadata location cannot be longer than 256 chars


```move
module 0x1::aptos_governance {
    const EMETADATA_LOCATION_TOO_LONG: u64 = 9;
}
```


<a id="0x1_aptos_governance_ENOT_DELEGATED_VOTER"></a>

This account is not the designated voter of the specified stake pool


```move
module 0x1::aptos_governance {
    const ENOT_DELEGATED_VOTER: u64 = 2;
}
```


<a id="0x1_aptos_governance_ENOT_PARTIAL_VOTING_PROPOSAL"></a>

The proposal in the argument is not a partial voting proposal.


```move
module 0x1::aptos_governance {
    const ENOT_PARTIAL_VOTING_PROPOSAL: u64 = 14;
}
```


<a id="0x1_aptos_governance_ENO_VOTING_POWER"></a>

The specified stake pool must be part of the validator set


```move
module 0x1::aptos_governance {
    const ENO_VOTING_POWER: u64 = 5;
}
```


<a id="0x1_aptos_governance_EPARTIAL_VOTING_NOT_INITIALIZED"></a>

Partial voting feature hasn&apos;t been properly initialized.


```move
module 0x1::aptos_governance {
    const EPARTIAL_VOTING_NOT_INITIALIZED: u64 = 13;
}
```


<a id="0x1_aptos_governance_EPROPOSAL_NOT_RESOLVABLE_YET"></a>

Proposal is not ready to be resolved. Waiting on time or votes


```move
module 0x1::aptos_governance {
    const EPROPOSAL_NOT_RESOLVABLE_YET: u64 = 6;
}
```


<a id="0x1_aptos_governance_EPROPOSAL_NOT_RESOLVED_YET"></a>

The proposal has not been resolved yet


```move
module 0x1::aptos_governance {
    const EPROPOSAL_NOT_RESOLVED_YET: u64 = 8;
}
```


<a id="0x1_aptos_governance_EUNAUTHORIZED"></a>

Account is not authorized to call this function.


```move
module 0x1::aptos_governance {
    const EUNAUTHORIZED: u64 = 11;
}
```


<a id="0x1_aptos_governance_EVOTING_POWER_OVERFLOW"></a>

The stake pool is using voting power more than it has.


```move
module 0x1::aptos_governance {
    const EVOTING_POWER_OVERFLOW: u64 = 12;
}
```


<a id="0x1_aptos_governance_METADATA_HASH_KEY"></a>



```move
module 0x1::aptos_governance {
    const METADATA_HASH_KEY: vector<u8> = [109, 101, 116, 97, 100, 97, 116, 97, 95, 104, 97, 115, 104];
}
```


<a id="0x1_aptos_governance_METADATA_LOCATION_KEY"></a>

Proposal metadata attribute keys.


```move
module 0x1::aptos_governance {
    const METADATA_LOCATION_KEY: vector<u8> = [109, 101, 116, 97, 100, 97, 116, 97, 95, 108, 111, 99, 97, 116, 105, 111, 110];
}
```


<a id="0x1_aptos_governance_store_signer_cap"></a>

## Function `store_signer_cap`

Can be called during genesis or by the governance itself.
Stores the signer capability for a given address.


```move
module 0x1::aptos_governance {
    public fun store_signer_cap(aptos_framework: &signer, signer_address: address, signer_cap: account::SignerCapability)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun store_signer_cap(
        aptos_framework: &signer,
        signer_address: address,
        signer_cap: SignerCapability,
    ) acquires GovernanceResponsbility {
        system_addresses::assert_aptos_framework(aptos_framework);
        system_addresses::assert_framework_reserved(signer_address);

        if (!exists<GovernanceResponsbility>(@aptos_framework)) {
            move_to(
                aptos_framework,
                GovernanceResponsbility { signer_caps: simple_map::create<address, SignerCapability>() }
            );
        };

        let signer_caps = &mut borrow_global_mut<GovernanceResponsbility>(@aptos_framework).signer_caps;
        simple_map::add(signer_caps, signer_address, signer_cap);
    }
}
```


<a id="0x1_aptos_governance_initialize"></a>

## Function `initialize`

Initializes the state for Aptos Governance. Can only be called during Genesis with a signer
for the aptos_framework (0x1) account.
This function is private because it&apos;s called directly from the vm.


```move
module 0x1::aptos_governance {
    fun initialize(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    fun initialize(
        aptos_framework: &signer,
        min_voting_threshold: u128,
        required_proposer_stake: u64,
        voting_duration_secs: u64,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        voting::register<GovernanceProposal>(aptos_framework);
        move_to(aptos_framework, GovernanceConfig {
            voting_duration_secs,
            min_voting_threshold,
            required_proposer_stake,
        });
        move_to(aptos_framework, GovernanceEvents {
            create_proposal_events: account::new_event_handle<CreateProposalEvent>(aptos_framework),
            update_config_events: account::new_event_handle<UpdateConfigEvent>(aptos_framework),
            vote_events: account::new_event_handle<VoteEvent>(aptos_framework),
        });
        move_to(aptos_framework, VotingRecords {
            votes: table::new(),
        });
        move_to(aptos_framework, ApprovedExecutionHashes {
            hashes: simple_map::create<u64, vector<u8>>(),
        })
    }
}
```


<a id="0x1_aptos_governance_update_governance_config"></a>

## Function `update_governance_config`

Update the governance configurations. This can only be called as part of resolving a proposal in this same
AptosGovernance.


```move
module 0x1::aptos_governance {
    public fun update_governance_config(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun update_governance_config(
        aptos_framework: &signer,
        min_voting_threshold: u128,
        required_proposer_stake: u64,
        voting_duration_secs: u64,
    ) acquires GovernanceConfig, GovernanceEvents {
        system_addresses::assert_aptos_framework(aptos_framework);

        let governance_config = borrow_global_mut<GovernanceConfig>(@aptos_framework);
        governance_config.voting_duration_secs = voting_duration_secs;
        governance_config.min_voting_threshold = min_voting_threshold;
        governance_config.required_proposer_stake = required_proposer_stake;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                UpdateConfig {
                    min_voting_threshold,
                    required_proposer_stake,
                    voting_duration_secs
                },
            )
        };
        let events = borrow_global_mut<GovernanceEvents>(@aptos_framework);
        event::emit_event<UpdateConfigEvent>(
            &mut events.update_config_events,
            UpdateConfigEvent {
                min_voting_threshold,
                required_proposer_stake,
                voting_duration_secs
            },
        );
    }
}
```


<a id="0x1_aptos_governance_initialize_partial_voting"></a>

## Function `initialize_partial_voting`

Initializes the state for Aptos Governance partial voting. Can only be called through Aptos governance
proposals with a signer for the aptos_framework (0x1) account.


```move
module 0x1::aptos_governance {
    public fun initialize_partial_voting(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun initialize_partial_voting(
        aptos_framework: &signer,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        move_to(aptos_framework, VotingRecordsV2 {
            votes: smart_table::new(),
        });
    }
}
```


<a id="0x1_aptos_governance_get_voting_duration_secs"></a>

## Function `get_voting_duration_secs`



```move
module 0x1::aptos_governance {
    #[view]
    public fun get_voting_duration_secs(): u64
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_voting_duration_secs(): u64 acquires GovernanceConfig {
        borrow_global<GovernanceConfig>(@aptos_framework).voting_duration_secs
    }
}
```


<a id="0x1_aptos_governance_get_min_voting_threshold"></a>

## Function `get_min_voting_threshold`



```move
module 0x1::aptos_governance {
    #[view]
    public fun get_min_voting_threshold(): u128
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_min_voting_threshold(): u128 acquires GovernanceConfig {
        borrow_global<GovernanceConfig>(@aptos_framework).min_voting_threshold
    }
}
```


<a id="0x1_aptos_governance_get_required_proposer_stake"></a>

## Function `get_required_proposer_stake`



```move
module 0x1::aptos_governance {
    #[view]
    public fun get_required_proposer_stake(): u64
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_required_proposer_stake(): u64 acquires GovernanceConfig {
        borrow_global<GovernanceConfig>(@aptos_framework).required_proposer_stake
    }
}
```


<a id="0x1_aptos_governance_has_entirely_voted"></a>

## Function `has_entirely_voted`

Return true if a stake pool has already voted on a proposal before partial governance voting is enabled.


```move
module 0x1::aptos_governance {
    #[view]
    public fun has_entirely_voted(stake_pool: address, proposal_id: u64): bool
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun has_entirely_voted(stake_pool: address, proposal_id: u64): bool acquires VotingRecords {
        let record_key = RecordKey {
            stake_pool,
            proposal_id,
        };
        // If a stake pool has already voted on a proposal before partial governance voting is enabled,
        // there is a record in VotingRecords.
        let voting_records = borrow_global<VotingRecords>(@aptos_framework);
        table::contains(&voting_records.votes, record_key)
    }
}
```


<a id="0x1_aptos_governance_get_remaining_voting_power"></a>

## Function `get_remaining_voting_power`

Return remaining voting power of a stake pool on a proposal.
Note: a stake pool&apos;s voting power on a proposal could increase over time(e.g. rewards/new stake).


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_remaining_voting_power(stake_pool: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_remaining_voting_power(
        stake_pool: address,
        proposal_id: u64
    ): u64 acquires VotingRecords, VotingRecordsV2 {
        assert_voting_initialization();

        let proposal_expiration = voting::get_proposal_expiration_secs<GovernanceProposal>(
            @aptos_framework,
            proposal_id
        );
        let lockup_until = stake::get_lockup_secs(stake_pool);
        // The voter's stake needs to be locked up at least as long as the proposal's expiration.
        // Also no one can vote on a expired proposal.
        if (proposal_expiration > lockup_until || timestamp::now_seconds() > proposal_expiration) {
            return 0
        };

        // If a stake pool has already voted on a proposal before partial governance voting is enabled, the stake pool
        // cannot vote on the proposal even after partial governance voting is enabled.
        if (has_entirely_voted(stake_pool, proposal_id)) {
            return 0
        };
        let record_key = RecordKey {
            stake_pool,
            proposal_id,
        };
        let used_voting_power = 0u64;
        if (features::partial_governance_voting_enabled()) {
            let voting_records_v2 = borrow_global<VotingRecordsV2>(@aptos_framework);
            used_voting_power = *smart_table::borrow_with_default(&voting_records_v2.votes, record_key, &0);
        };
        get_voting_power(stake_pool) - used_voting_power
    }
}
```


<a id="0x1_aptos_governance_create_proposal"></a>

## Function `create_proposal`

Create a single&#45;step proposal with the backing `stake_pool`.
@param execution_hash Required. This is the hash of the resolution script. When the proposal is resolved,
only the exact script with matching hash can be successfully executed.


```move
module 0x1::aptos_governance {
    public entry fun create_proposal(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun create_proposal(
        proposer: &signer,
        stake_pool: address,
        execution_hash: vector<u8>,
        metadata_location: vector<u8>,
        metadata_hash: vector<u8>,
    ) acquires GovernanceConfig, GovernanceEvents {
        create_proposal_v2(proposer, stake_pool, execution_hash, metadata_location, metadata_hash, false);
    }
}
```


<a id="0x1_aptos_governance_create_proposal_v2"></a>

## Function `create_proposal_v2`

Create a single&#45;step or multi&#45;step proposal with the backing `stake_pool`.
@param execution_hash Required. This is the hash of the resolution script. When the proposal is resolved,
only the exact script with matching hash can be successfully executed.


```move
module 0x1::aptos_governance {
    public entry fun create_proposal_v2(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>, is_multi_step_proposal: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun create_proposal_v2(
        proposer: &signer,
        stake_pool: address,
        execution_hash: vector<u8>,
        metadata_location: vector<u8>,
        metadata_hash: vector<u8>,
        is_multi_step_proposal: bool,
    ) acquires GovernanceConfig, GovernanceEvents {
        create_proposal_v2_impl(
            proposer,
            stake_pool,
            execution_hash,
            metadata_location,
            metadata_hash,
            is_multi_step_proposal
        );
    }
}
```


<a id="0x1_aptos_governance_create_proposal_v2_impl"></a>

## Function `create_proposal_v2_impl`

Create a single&#45;step or multi&#45;step proposal with the backing `stake_pool`.
@param execution_hash Required. This is the hash of the resolution script. When the proposal is resolved,
only the exact script with matching hash can be successfully executed.
Return proposal_id when a proposal is successfully created.


```move
module 0x1::aptos_governance {
    public fun create_proposal_v2_impl(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>, is_multi_step_proposal: bool): u64
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun create_proposal_v2_impl(
        proposer: &signer,
        stake_pool: address,
        execution_hash: vector<u8>,
        metadata_location: vector<u8>,
        metadata_hash: vector<u8>,
        is_multi_step_proposal: bool,
    ): u64 acquires GovernanceConfig, GovernanceEvents {
        let proposer_address = signer::address_of(proposer);
        assert!(
            stake::get_delegated_voter(stake_pool) == proposer_address,
            error::invalid_argument(ENOT_DELEGATED_VOTER)
        );

        // The proposer's stake needs to be at least the required bond amount.
        let governance_config = borrow_global<GovernanceConfig>(@aptos_framework);
        let stake_balance = get_voting_power(stake_pool);
        assert!(
            stake_balance >= governance_config.required_proposer_stake,
            error::invalid_argument(EINSUFFICIENT_PROPOSER_STAKE),
        );

        // The proposer's stake needs to be locked up at least as long as the proposal's voting period.
        let current_time = timestamp::now_seconds();
        let proposal_expiration = current_time + governance_config.voting_duration_secs;
        assert!(
            stake::get_lockup_secs(stake_pool) >= proposal_expiration,
            error::invalid_argument(EINSUFFICIENT_STAKE_LOCKUP),
        );

        // Create and validate proposal metadata.
        let proposal_metadata = create_proposal_metadata(metadata_location, metadata_hash);

        // We want to allow early resolution of proposals if more than 50% of the total supply of the network coins
        // has voted. This doesn't take into subsequent inflation/deflation (rewards are issued every epoch and gas fees
        // are burnt after every transaction), but inflation/delation is very unlikely to have a major impact on total
        // supply during the voting period.
        let total_voting_token_supply = coin::supply<AptosCoin>();
        let early_resolution_vote_threshold = option::none<u128>();
        if (option::is_some(&total_voting_token_supply)) {
            let total_supply = *option::borrow(&total_voting_token_supply);
            // 50% + 1 to avoid rounding errors.
            early_resolution_vote_threshold = option::some(total_supply / 2 + 1);
        };

        let proposal_id = voting::create_proposal_v2(
            proposer_address,
            @aptos_framework,
            governance_proposal::create_proposal(),
            execution_hash,
            governance_config.min_voting_threshold,
            proposal_expiration,
            early_resolution_vote_threshold,
            proposal_metadata,
            is_multi_step_proposal,
        );

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CreateProposal {
                    proposal_id,
                    proposer: proposer_address,
                    stake_pool,
                    execution_hash,
                    proposal_metadata,
                },
            );
        };
        let events = borrow_global_mut<GovernanceEvents>(@aptos_framework);
        event::emit_event<CreateProposalEvent>(
            &mut events.create_proposal_events,
            CreateProposalEvent {
                proposal_id,
                proposer: proposer_address,
                stake_pool,
                execution_hash,
                proposal_metadata,
            },
        );
        proposal_id
    }
}
```


<a id="0x1_aptos_governance_batch_vote"></a>

## Function `batch_vote`

Vote on proposal with proposal_id and all voting power from multiple stake_pools.


```move
module 0x1::aptos_governance {
    public entry fun batch_vote(voter: &signer, stake_pools: vector<address>, proposal_id: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun batch_vote(
        voter: &signer,
        stake_pools: vector<address>,
        proposal_id: u64,
        should_pass: bool,
    ) acquires ApprovedExecutionHashes, VotingRecords, VotingRecordsV2, GovernanceEvents {
        vector::for_each(stake_pools, |stake_pool| {
            vote_internal(voter, stake_pool, proposal_id, MAX_U64, should_pass);
        });
    }
}
```


<a id="0x1_aptos_governance_batch_partial_vote"></a>

## Function `batch_partial_vote`

Batch vote on proposal with proposal_id and specified voting power from multiple stake_pools.


```move
module 0x1::aptos_governance {
    public entry fun batch_partial_vote(voter: &signer, stake_pools: vector<address>, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun batch_partial_vote(
        voter: &signer,
        stake_pools: vector<address>,
        proposal_id: u64,
        voting_power: u64,
        should_pass: bool,
    ) acquires ApprovedExecutionHashes, VotingRecords, VotingRecordsV2, GovernanceEvents {
        vector::for_each(stake_pools, |stake_pool| {
            vote_internal(voter, stake_pool, proposal_id, voting_power, should_pass);
        });
    }
}
```


<a id="0x1_aptos_governance_vote"></a>

## Function `vote`

Vote on proposal with `proposal_id` and all voting power from `stake_pool`.


```move
module 0x1::aptos_governance {
    public entry fun vote(voter: &signer, stake_pool: address, proposal_id: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun vote(
        voter: &signer,
        stake_pool: address,
        proposal_id: u64,
        should_pass: bool,
    ) acquires ApprovedExecutionHashes, VotingRecords, VotingRecordsV2, GovernanceEvents {
        vote_internal(voter, stake_pool, proposal_id, MAX_U64, should_pass);
    }
}
```


<a id="0x1_aptos_governance_partial_vote"></a>

## Function `partial_vote`

Vote on proposal with `proposal_id` and specified voting power from `stake_pool`.


```move
module 0x1::aptos_governance {
    public entry fun partial_vote(voter: &signer, stake_pool: address, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun partial_vote(
        voter: &signer,
        stake_pool: address,
        proposal_id: u64,
        voting_power: u64,
        should_pass: bool,
    ) acquires ApprovedExecutionHashes, VotingRecords, VotingRecordsV2, GovernanceEvents {
        vote_internal(voter, stake_pool, proposal_id, voting_power, should_pass);
    }
}
```


<a id="0x1_aptos_governance_vote_internal"></a>

## Function `vote_internal`

Vote on proposal with `proposal_id` and specified voting_power from `stake_pool`.
If voting_power is more than all the left voting power of `stake_pool`, use all the left voting power.
If a stake pool has already voted on a proposal before partial governance voting is enabled, the stake pool
cannot vote on the proposal even after partial governance voting is enabled.


```move
module 0x1::aptos_governance {
    fun vote_internal(voter: &signer, stake_pool: address, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    fun vote_internal(
        voter: &signer,
        stake_pool: address,
        proposal_id: u64,
        voting_power: u64,
        should_pass: bool,
    ) acquires ApprovedExecutionHashes, VotingRecords, VotingRecordsV2, GovernanceEvents {
        let voter_address = signer::address_of(voter);
        assert!(stake::get_delegated_voter(stake_pool) == voter_address, error::invalid_argument(ENOT_DELEGATED_VOTER));

        // The voter's stake needs to be locked up at least as long as the proposal's expiration.
        let proposal_expiration = voting::get_proposal_expiration_secs<GovernanceProposal>(
            @aptos_framework,
            proposal_id
        );
        assert!(
            stake::get_lockup_secs(stake_pool) >= proposal_expiration,
            error::invalid_argument(EINSUFFICIENT_STAKE_LOCKUP),
        );

        // If a stake pool has already voted on a proposal before partial governance voting is enabled,
        // `get_remaining_voting_power` returns 0.
        let staking_pool_voting_power = get_remaining_voting_power(stake_pool, proposal_id);
        voting_power = min(voting_power, staking_pool_voting_power);

        // Short-circuit if the voter has no voting power.
        assert!(voting_power > 0, error::invalid_argument(ENO_VOTING_POWER));

        voting::vote<GovernanceProposal>(
            &governance_proposal::create_empty_proposal(),
            @aptos_framework,
            proposal_id,
            voting_power,
            should_pass,
        );

        let record_key = RecordKey {
            stake_pool,
            proposal_id,
        };
        if (features::partial_governance_voting_enabled()) {
            let voting_records_v2 = borrow_global_mut<VotingRecordsV2>(@aptos_framework);
            let used_voting_power = smart_table::borrow_mut_with_default(&mut voting_records_v2.votes, record_key, 0);
            // This calculation should never overflow because the used voting cannot exceed the total voting power of this stake pool.
            *used_voting_power = *used_voting_power + voting_power;
        } else {
            let voting_records = borrow_global_mut<VotingRecords>(@aptos_framework);
            assert!(
                !table::contains(&voting_records.votes, record_key),
                error::invalid_argument(EALREADY_VOTED));
            table::add(&mut voting_records.votes, record_key, true);
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                Vote {
                    proposal_id,
                    voter: voter_address,
                    stake_pool,
                    num_votes: voting_power,
                    should_pass,
                },
            );
        };
        let events = borrow_global_mut<GovernanceEvents>(@aptos_framework);
        event::emit_event<VoteEvent>(
            &mut events.vote_events,
            VoteEvent {
                proposal_id,
                voter: voter_address,
                stake_pool,
                num_votes: voting_power,
                should_pass,
            },
        );

        let proposal_state = voting::get_proposal_state<GovernanceProposal>(@aptos_framework, proposal_id);
        if (proposal_state == PROPOSAL_STATE_SUCCEEDED) {
            add_approved_script_hash(proposal_id);
        }
    }
}
```


<a id="0x1_aptos_governance_add_approved_script_hash_script"></a>

## Function `add_approved_script_hash_script`



```move
module 0x1::aptos_governance {
    public entry fun add_approved_script_hash_script(proposal_id: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun add_approved_script_hash_script(proposal_id: u64) acquires ApprovedExecutionHashes {
        add_approved_script_hash(proposal_id)
    }
}
```


<a id="0x1_aptos_governance_add_approved_script_hash"></a>

## Function `add_approved_script_hash`

Add the execution script hash of a successful governance proposal to the approved list.
This is needed to bypass the mempool transaction size limit for approved governance proposal transactions that
are too large (e.g. module upgrades).


```move
module 0x1::aptos_governance {
    public fun add_approved_script_hash(proposal_id: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun add_approved_script_hash(proposal_id: u64) acquires ApprovedExecutionHashes {
        let approved_hashes = borrow_global_mut<ApprovedExecutionHashes>(@aptos_framework);

        // Ensure the proposal can be resolved.
        let proposal_state = voting::get_proposal_state<GovernanceProposal>(@aptos_framework, proposal_id);
        assert!(proposal_state == PROPOSAL_STATE_SUCCEEDED, error::invalid_argument(EPROPOSAL_NOT_RESOLVABLE_YET));

        let execution_hash = voting::get_execution_hash<GovernanceProposal>(@aptos_framework, proposal_id);

        // If this is a multi-step proposal, the proposal id will already exist in the ApprovedExecutionHashes map.
        // We will update execution hash in ApprovedExecutionHashes to be the next_execution_hash.
        if (simple_map::contains_key(&approved_hashes.hashes, &proposal_id)) {
            let current_execution_hash = simple_map::borrow_mut(&mut approved_hashes.hashes, &proposal_id);
            *current_execution_hash = execution_hash;
        } else {
            simple_map::add(&mut approved_hashes.hashes, proposal_id, execution_hash);
        }
    }
}
```


<a id="0x1_aptos_governance_resolve"></a>

## Function `resolve`

Resolve a successful single&#45;step proposal. This would fail if the proposal is not successful (not enough votes or more no
than yes).


```move
module 0x1::aptos_governance {
    public fun resolve(proposal_id: u64, signer_address: address): signer
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun resolve(
        proposal_id: u64,
        signer_address: address
    ): signer acquires ApprovedExecutionHashes, GovernanceResponsbility {
        voting::resolve<GovernanceProposal>(@aptos_framework, proposal_id);
        remove_approved_hash(proposal_id);
        get_signer(signer_address)
    }
}
```


<a id="0x1_aptos_governance_resolve_multi_step_proposal"></a>

## Function `resolve_multi_step_proposal`

Resolve a successful multi&#45;step proposal. This would fail if the proposal is not successful.


```move
module 0x1::aptos_governance {
    public fun resolve_multi_step_proposal(proposal_id: u64, signer_address: address, next_execution_hash: vector<u8>): signer
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun resolve_multi_step_proposal(
        proposal_id: u64,
        signer_address: address,
        next_execution_hash: vector<u8>
    ): signer acquires GovernanceResponsbility, ApprovedExecutionHashes {
        voting::resolve_proposal_v2<GovernanceProposal>(@aptos_framework, proposal_id, next_execution_hash);
        // If the current step is the last step of this multi-step proposal,
        // we will remove the execution hash from the ApprovedExecutionHashes map.
        if (vector::length(&next_execution_hash) == 0) {
            remove_approved_hash(proposal_id);
        } else {
            // If the current step is not the last step of this proposal,
            // we replace the current execution hash with the next execution hash
            // in the ApprovedExecutionHashes map.
            add_approved_script_hash(proposal_id)
        };
        get_signer(signer_address)
    }
}
```


<a id="0x1_aptos_governance_remove_approved_hash"></a>

## Function `remove_approved_hash`

Remove an approved proposal&apos;s execution script hash.


```move
module 0x1::aptos_governance {
    public fun remove_approved_hash(proposal_id: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun remove_approved_hash(proposal_id: u64) acquires ApprovedExecutionHashes {
        assert!(
            voting::is_resolved<GovernanceProposal>(@aptos_framework, proposal_id),
            error::invalid_argument(EPROPOSAL_NOT_RESOLVED_YET),
        );

        let approved_hashes = &mut borrow_global_mut<ApprovedExecutionHashes>(@aptos_framework).hashes;
        if (simple_map::contains_key(approved_hashes, &proposal_id)) {
            simple_map::remove(approved_hashes, &proposal_id);
        };
    }
}
```


<a id="0x1_aptos_governance_reconfigure"></a>

## Function `reconfigure`

Manually reconfigure. Called at the end of a governance txn that alters on&#45;chain configs.

WARNING: this function always ensures a reconfiguration starts, but when the reconfiguration finishes depends.
&#45; If feature `RECONFIGURE_WITH_DKG` is disabled, it finishes immediately.
&#45; At the end of the calling transaction, we will be in a new epoch.
&#45; If feature `RECONFIGURE_WITH_DKG` is enabled, it starts DKG, and the new epoch will start in a block prologue after DKG finishes.

This behavior affects when an update of an on&#45;chain config (e.g. `ConsensusConfig`, `Features`) takes effect,
since such updates are applied whenever we enter an new epoch.


```move
module 0x1::aptos_governance {
    public entry fun reconfigure(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun reconfigure(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        if (consensus_config::validator_txn_enabled() && randomness_config::enabled()) {
            reconfiguration_with_dkg::try_start();
        } else {
            reconfiguration_with_dkg::finish(aptos_framework);
        }
    }
}
```


<a id="0x1_aptos_governance_force_end_epoch"></a>

## Function `force_end_epoch`

Change epoch immediately.
If `RECONFIGURE_WITH_DKG` is enabled and we are in the middle of a DKG,
stop waiting for DKG and enter the new epoch without randomness.

WARNING: currently only used by tests. In most cases you should use `reconfigure()` instead.
TODO: migrate these tests to be aware of async reconfiguration.


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        reconfiguration_with_dkg::finish(aptos_framework);
    }
}
```


<a id="0x1_aptos_governance_force_end_epoch_test_only"></a>

## Function `force_end_epoch_test_only`

`force_end_epoch()` equivalent but only called in testnet,
where the core resources account exists and has been granted power to mint Aptos coins.


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch_test_only(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch_test_only(aptos_framework: &signer) acquires GovernanceResponsbility {
        let core_signer = get_signer_testnet_only(aptos_framework, @0x1);
        system_addresses::assert_aptos_framework(&core_signer);
        reconfiguration_with_dkg::finish(&core_signer);
    }
}
```


<a id="0x1_aptos_governance_toggle_features"></a>

## Function `toggle_features`

Update feature flags and also trigger reconfiguration.


```move
module 0x1::aptos_governance {
    public fun toggle_features(aptos_framework: &signer, enable: vector<u64>, disable: vector<u64>)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun toggle_features(aptos_framework: &signer, enable: vector<u64>, disable: vector<u64>) {
        system_addresses::assert_aptos_framework(aptos_framework);
        features::change_feature_flags_for_next_epoch(aptos_framework, enable, disable);
        reconfigure(aptos_framework);
    }
}
```


<a id="0x1_aptos_governance_get_signer_testnet_only"></a>

## Function `get_signer_testnet_only`

Only called in testnet where the core resources account exists and has been granted power to mint Aptos coins.


```move
module 0x1::aptos_governance {
    public fun get_signer_testnet_only(core_resources: &signer, signer_address: address): signer
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_signer_testnet_only(
        core_resources: &signer, signer_address: address): signer acquires GovernanceResponsbility {
        system_addresses::assert_core_resource(core_resources);
        // Core resources account only has mint capability in tests/testnets.
        assert!(aptos_coin::has_mint_capability(core_resources), error::unauthenticated(EUNAUTHORIZED));
        get_signer(signer_address)
    }
}
```


<a id="0x1_aptos_governance_get_voting_power"></a>

## Function `get_voting_power`

Return the voting power a stake pool has with respect to governance proposals.


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_voting_power(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun get_voting_power(pool_address: address): u64 {
        let allow_validator_set_change = staking_config::get_allow_validator_set_change(&staking_config::get());
        if (allow_validator_set_change) {
            let (active, _, pending_active, pending_inactive) = stake::get_stake(pool_address);
            // We calculate the voting power as total non-inactive stakes of the pool. Even if the validator is not in the
            // active validator set, as long as they have a lockup (separately checked in create_proposal and voting), their
            // stake would still count in their voting power for governance proposals.
            active + pending_active + pending_inactive
        } else {
            stake::get_current_epoch_voting_power(pool_address)
        }
    }
}
```


<a id="0x1_aptos_governance_get_signer"></a>

## Function `get_signer`

Return a signer for making changes to 0x1 as part of on&#45;chain governance proposal process.


```move
module 0x1::aptos_governance {
    fun get_signer(signer_address: address): signer
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    fun get_signer(signer_address: address): signer acquires GovernanceResponsbility {
        let governance_responsibility = borrow_global<GovernanceResponsbility>(@aptos_framework);
        let signer_cap = simple_map::borrow(&governance_responsibility.signer_caps, &signer_address);
        create_signer_with_capability(signer_cap)
    }
}
```


<a id="0x1_aptos_governance_create_proposal_metadata"></a>

## Function `create_proposal_metadata`



```move
module 0x1::aptos_governance {
    fun create_proposal_metadata(metadata_location: vector<u8>, metadata_hash: vector<u8>): simple_map::SimpleMap<string::String, vector<u8>>
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    fun create_proposal_metadata(
        metadata_location: vector<u8>,
        metadata_hash: vector<u8>
    ): SimpleMap<String, vector<u8>> {
        assert!(string::length(&utf8(metadata_location)) <= 256, error::invalid_argument(EMETADATA_LOCATION_TOO_LONG));
        assert!(string::length(&utf8(metadata_hash)) <= 256, error::invalid_argument(EMETADATA_HASH_TOO_LONG));

        let metadata = simple_map::create<String, vector<u8>>();
        simple_map::add(&mut metadata, utf8(METADATA_LOCATION_KEY), metadata_location);
        simple_map::add(&mut metadata, utf8(METADATA_HASH_KEY), metadata_hash);
        metadata
    }
}
```


<a id="0x1_aptos_governance_assert_voting_initialization"></a>

## Function `assert_voting_initialization`



```move
module 0x1::aptos_governance {
    fun assert_voting_initialization()
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    fun assert_voting_initialization() {
        if (features::partial_governance_voting_enabled()) {
            assert!(exists<VotingRecordsV2>(@aptos_framework), error::invalid_state(EPARTIAL_VOTING_NOT_INITIALIZED));
        };
    }
}
```


<a id="0x1_aptos_governance_initialize_for_verification"></a>

## Function `initialize_for_verification`



```move
module 0x1::aptos_governance {
    #[verify_only]
    public fun initialize_for_verification(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```


##### Implementation


```move
module 0x1::aptos_governance {
    public fun initialize_for_verification(
        aptos_framework: &signer,
        min_voting_threshold: u128,
        required_proposer_stake: u64,
        voting_duration_secs: u64,
    ) {
        initialize(aptos_framework, min_voting_threshold, required_proposer_stake, voting_duration_secs);
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
<td>The create proposal function calls create proposal v2.</td>
<td>Low</td>
<td>The create_proposal function internally calls create_proposal_v2.</td>
<td>This is manually audited to ensure create_proposal_v2 is called in create_proposal.</td>
</tr>

<tr>
<td>2</td>
<td>The proposer must have a stake equal to or greater than the required bond amount.</td>
<td>High</td>
<td>The create_proposal_v2 function verifies that the stake balance equals or exceeds the required proposer stake amount.</td>
<td>Formally verified in [#high&#45;level&#45;req&#45;2](CreateProposalAbortsIf).</td>
</tr>

<tr>
<td>3</td>
<td>The Approved execution hashes resources that exist when the vote function is called.</td>
<td>Low</td>
<td>The Vote function acquires the Approved execution hashes resources.</td>
<td>Formally verified in [#high&#45;level&#45;req&#45;3](VoteAbortIf).</td>
</tr>

<tr>
<td>4</td>
<td>The execution script hash of a successful governance proposal is added to the approved list if the proposal can be resolved.</td>
<td>Medium</td>
<td>The add_approved_script_hash function asserts that proposal_state &#61;&#61; PROPOSAL_STATE_SUCCEEDED.</td>
<td>Formally verified in [#high&#45;level&#45;req&#45;4](AddApprovedScriptHash).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::aptos_governance {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_store_signer_cap"></a>

### Function `store_signer_cap`


```move
module 0x1::aptos_governance {
    public fun store_signer_cap(aptos_framework: &signer, signer_address: address, signer_cap: account::SignerCapability)
}
```



```move
module 0x1::aptos_governance {
    aborts_if !system_addresses::is_aptos_framework_address(signer::address_of(aptos_framework));
    aborts_if !system_addresses::is_framework_reserved_address(signer_address);
    let signer_caps = global<GovernanceResponsbility>(@aptos_framework).signer_caps;
    aborts_if exists<GovernanceResponsbility>(@aptos_framework) &&
        simple_map::spec_contains_key(signer_caps, signer_address);
    ensures exists<GovernanceResponsbility>(@aptos_framework);
    let post post_signer_caps = global<GovernanceResponsbility>(@aptos_framework).signer_caps;
    ensures simple_map::spec_contains_key(post_signer_caps, signer_address);
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::aptos_governance {
    fun initialize(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```

Signer address must be @aptos_framework.
The signer does not allow these resources (GovernanceProposal, GovernanceConfig, GovernanceEvents, VotingRecords, ApprovedExecutionHashes) to exist.
The signer must have an Account.
Limit addition overflow.


```move
module 0x1::aptos_governance {
    let addr = signer::address_of(aptos_framework);
    let register_account = global<account::Account>(addr);
    aborts_if exists<voting::VotingForum<GovernanceProposal>>(addr);
    aborts_if !exists<account::Account>(addr);
    aborts_if register_account.guid_creation_num + 7 > MAX_U64;
    aborts_if register_account.guid_creation_num + 7 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !type_info::spec_is_struct<GovernanceProposal>();
    include InitializeAbortIf;
    ensures exists<voting::VotingForum<governance_proposal::GovernanceProposal>>(addr);
    ensures exists<GovernanceConfig>(addr);
    ensures exists<GovernanceEvents>(addr);
    ensures exists<VotingRecords>(addr);
    ensures exists<ApprovedExecutionHashes>(addr);
}
```


<a id="@Specification_1_update_governance_config"></a>

### Function `update_governance_config`


```move
module 0x1::aptos_governance {
    public fun update_governance_config(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```

Signer address must be @aptos_framework.
Address @aptos_framework must exist GovernanceConfig and GovernanceEvents.


```move
module 0x1::aptos_governance {
    let addr = signer::address_of(aptos_framework);
    let governance_config = global<GovernanceConfig>(@aptos_framework);
    let post new_governance_config = global<GovernanceConfig>(@aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if !exists<GovernanceConfig>(@aptos_framework);
    aborts_if !exists<GovernanceEvents>(@aptos_framework);
    modifies global<GovernanceConfig>(addr);
    ensures new_governance_config.voting_duration_secs == voting_duration_secs;
    ensures new_governance_config.min_voting_threshold == min_voting_threshold;
    ensures new_governance_config.required_proposer_stake == required_proposer_stake;
}
```


<a id="@Specification_1_initialize_partial_voting"></a>

### Function `initialize_partial_voting`


```move
module 0x1::aptos_governance {
    public fun initialize_partial_voting(aptos_framework: &signer)
}
```

Signer address must be @aptos_framework.
Abort if structs have already been created.


```move
module 0x1::aptos_governance {
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if exists<VotingRecordsV2>(@aptos_framework);
    ensures exists<VotingRecordsV2>(@aptos_framework);
}
```



<a id="0x1_aptos_governance_InitializeAbortIf"></a>


```move
module 0x1::aptos_governance {
    schema InitializeAbortIf {
        aptos_framework: &signer;
        min_voting_threshold: u128;
        required_proposer_stake: u64;
        voting_duration_secs: u64;
        let addr = signer::address_of(aptos_framework);
        let account = global<account::Account>(addr);
        aborts_if addr != @aptos_framework;
        aborts_if exists<voting::VotingForum<governance_proposal::GovernanceProposal>>(addr);
        aborts_if exists<GovernanceConfig>(addr);
        aborts_if exists<GovernanceEvents>(addr);
        aborts_if exists<VotingRecords>(addr);
        aborts_if exists<ApprovedExecutionHashes>(addr);
        aborts_if !exists<account::Account>(addr);
    }
}
```


<a id="@Specification_1_get_voting_duration_secs"></a>

### Function `get_voting_duration_secs`


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_voting_duration_secs(): u64
}
```



```move
module 0x1::aptos_governance {
    include AbortsIfNotGovernanceConfig;
}
```


<a id="@Specification_1_get_min_voting_threshold"></a>

### Function `get_min_voting_threshold`


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_min_voting_threshold(): u128
}
```



```move
module 0x1::aptos_governance {
    include AbortsIfNotGovernanceConfig;
}
```


<a id="@Specification_1_get_required_proposer_stake"></a>

### Function `get_required_proposer_stake`


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_required_proposer_stake(): u64
}
```



```move
module 0x1::aptos_governance {
    include AbortsIfNotGovernanceConfig;
}
```



<a id="0x1_aptos_governance_AbortsIfNotGovernanceConfig"></a>


```move
module 0x1::aptos_governance {
    schema AbortsIfNotGovernanceConfig {
        aborts_if !exists<GovernanceConfig>(@aptos_framework);
    }
}
```


<a id="@Specification_1_has_entirely_voted"></a>

### Function `has_entirely_voted`


```move
module 0x1::aptos_governance {
    #[view]
    public fun has_entirely_voted(stake_pool: address, proposal_id: u64): bool
}
```



```move
module 0x1::aptos_governance {
    aborts_if !exists<VotingRecords>(@aptos_framework);
}
```


<a id="@Specification_1_get_remaining_voting_power"></a>

### Function `get_remaining_voting_power`


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_remaining_voting_power(stake_pool: address, proposal_id: u64): u64
}
```



```move
module 0x1::aptos_governance {
    aborts_if features::spec_partial_governance_voting_enabled() && !exists<VotingRecordsV2>(@aptos_framework);
    include voting::AbortsIfNotContainProposalID<GovernanceProposal> {
        voting_forum_address: @aptos_framework
    };
    aborts_if !exists<stake::StakePool>(stake_pool);
    aborts_if spec_proposal_expiration <= locked_until && !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    let spec_proposal_expiration = voting::spec_get_proposal_expiration_secs<GovernanceProposal>(@aptos_framework, proposal_id);
    let locked_until = global<stake::StakePool>(stake_pool).locked_until_secs;
    let remain_zero_1_cond = (spec_proposal_expiration > locked_until || timestamp::spec_now_seconds() > spec_proposal_expiration);
    ensures remain_zero_1_cond ==> result == 0;
    let record_key = RecordKey {
        stake_pool,
        proposal_id,
    };
    let entirely_voted = spec_has_entirely_voted(stake_pool, proposal_id, record_key);
    aborts_if !remain_zero_1_cond && !exists<VotingRecords>(@aptos_framework);
    include !remain_zero_1_cond && !entirely_voted ==> GetVotingPowerAbortsIf {
        pool_address: stake_pool
    };
    let staking_config = global<staking_config::StakingConfig>(@aptos_framework);
    let voting_power = spec_get_voting_power(stake_pool, staking_config);
    let voting_records_v2 = borrow_global<VotingRecordsV2>(@aptos_framework);
    let used_voting_power = if (smart_table::spec_contains(voting_records_v2.votes, record_key)) {
        smart_table::spec_get(voting_records_v2.votes, record_key)
    } else {
        0
    };
    aborts_if !remain_zero_1_cond && !entirely_voted && features::spec_partial_governance_voting_enabled() &&
        used_voting_power > 0 && voting_power < used_voting_power;
    ensures result == spec_get_remaining_voting_power(stake_pool, proposal_id);
}
```



<a id="0x1_aptos_governance_spec_get_remaining_voting_power"></a>


```move
module 0x1::aptos_governance {
    fun spec_get_remaining_voting_power(stake_pool: address, proposal_id: u64): u64 {
       let spec_proposal_expiration = voting::spec_get_proposal_expiration_secs<GovernanceProposal>(@aptos_framework, proposal_id);
       let locked_until = global<stake::StakePool>(stake_pool).locked_until_secs;
       let remain_zero_1_cond = (spec_proposal_expiration > locked_until || timestamp::spec_now_seconds() > spec_proposal_expiration);
       let staking_config = global<staking_config::StakingConfig>(@aptos_framework);
       let voting_records_v2 = borrow_global<VotingRecordsV2>(@aptos_framework);
       let record_key = RecordKey {
           stake_pool,
           proposal_id,
       };
       let entirely_voted = spec_has_entirely_voted(stake_pool, proposal_id, record_key);
       let voting_power = spec_get_voting_power(stake_pool, staking_config);
       let used_voting_power = if (smart_table::spec_contains(voting_records_v2.votes, record_key)) {
           smart_table::spec_get(voting_records_v2.votes, record_key)
       } else {
           0
       };
       if (remain_zero_1_cond) {
           0
       } else if (entirely_voted) {
           0
       } else if (!features::spec_partial_governance_voting_enabled()) {
           voting_power
       } else {
           voting_power - used_voting_power
       }
    }
}
```



<a id="0x1_aptos_governance_spec_has_entirely_voted"></a>


```move
module 0x1::aptos_governance {
    fun spec_has_entirely_voted(stake_pool: address, proposal_id: u64, record_key: RecordKey): bool {
       let voting_records = global<VotingRecords>(@aptos_framework);
       table::spec_contains(voting_records.votes, record_key)
    }
}
```



<a id="0x1_aptos_governance_GetVotingPowerAbortsIf"></a>


```move
module 0x1::aptos_governance {
    schema GetVotingPowerAbortsIf {
        pool_address: address;
        let staking_config = global<staking_config::StakingConfig>(@aptos_framework);
        aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
        let allow_validator_set_change = staking_config.allow_validator_set_change;
        let stake_pool_res = global<stake::StakePool>(pool_address);
        aborts_if allow_validator_set_change && (stake_pool_res.active.value + stake_pool_res.pending_active.value + stake_pool_res.pending_inactive.value) > MAX_U64;
        aborts_if !exists<stake::StakePool>(pool_address);
        aborts_if !allow_validator_set_change && !exists<stake::ValidatorSet>(@aptos_framework);
        aborts_if !allow_validator_set_change && stake::spec_is_current_epoch_validator(pool_address) && stake_pool_res.active.value + stake_pool_res.pending_inactive.value > MAX_U64;
    }
}
```


<a id="@Specification_1_create_proposal"></a>

### Function `create_proposal`


```move
module 0x1::aptos_governance {
    public entry fun create_proposal(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>)
}
```

The same as spec of `create_proposal_v2()`.


```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include CreateProposalAbortsIf;
}
```


<a id="@Specification_1_create_proposal_v2"></a>

### Function `create_proposal_v2`


```move
module 0x1::aptos_governance {
    public entry fun create_proposal_v2(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>, is_multi_step_proposal: bool)
}
```



```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include CreateProposalAbortsIf;
}
```


<a id="@Specification_1_create_proposal_v2_impl"></a>

### Function `create_proposal_v2_impl`


```move
module 0x1::aptos_governance {
    public fun create_proposal_v2_impl(proposer: &signer, stake_pool: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>, is_multi_step_proposal: bool): u64
}
```



```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include CreateProposalAbortsIf;
}
```


<a id="@Specification_1_batch_vote"></a>

### Function `batch_vote`


```move
module 0x1::aptos_governance {
    public entry fun batch_vote(voter: &signer, stake_pools: vector<address>, proposal_id: u64, should_pass: bool)
}
```



```move
module 0x1::aptos_governance {
    pragma verify = false;
}
```


<a id="@Specification_1_batch_partial_vote"></a>

### Function `batch_partial_vote`


```move
module 0x1::aptos_governance {
    public entry fun batch_partial_vote(voter: &signer, stake_pools: vector<address>, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```



```move
module 0x1::aptos_governance {
    pragma verify = false;
}
```


<a id="@Specification_1_vote"></a>

### Function `vote`


```move
module 0x1::aptos_governance {
    public entry fun vote(voter: &signer, stake_pool: address, proposal_id: u64, should_pass: bool)
}
```

stake_pool must exist StakePool.
The delegated voter under the resource StakePool of the stake_pool must be the voter address.
Address @aptos_framework must exist VotingRecords and GovernanceProposal.


```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include VoteAbortIf  {
        voting_power: MAX_U64
    };
}
```


<a id="@Specification_1_partial_vote"></a>

### Function `partial_vote`


```move
module 0x1::aptos_governance {
    public entry fun partial_vote(voter: &signer, stake_pool: address, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```

stake_pool must exist StakePool.
The delegated voter under the resource StakePool of the stake_pool must be the voter address.
Address @aptos_framework must exist VotingRecords and GovernanceProposal.
Address @aptos_framework must exist VotingRecordsV2 if partial_governance_voting flag is enabled.


```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include VoteAbortIf;
}
```


<a id="@Specification_1_vote_internal"></a>

### Function `vote_internal`


```move
module 0x1::aptos_governance {
    fun vote_internal(voter: &signer, stake_pool: address, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```

stake_pool must exist StakePool.
The delegated voter under the resource StakePool of the stake_pool must be the voter address.
Address @aptos_framework must exist VotingRecords and GovernanceProposal.
Address @aptos_framework must exist VotingRecordsV2 if partial_governance_voting flag is enabled.


```move
module 0x1::aptos_governance {
    pragma verify_duration_estimate = 60;
    requires chain_status::is_operating();
    include VoteAbortIf;
}
```



<a id="0x1_aptos_governance_VoteAbortIf"></a>


```move
module 0x1::aptos_governance {
    schema VoteAbortIf {
        voter: &signer;
        stake_pool: address;
        proposal_id: u64;
        should_pass: bool;
        voting_power: u64;
        include VotingGetDelegatedVoterAbortsIf { sign: voter };
        aborts_if spec_proposal_expiration <= locked_until && !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        let spec_proposal_expiration = voting::spec_get_proposal_expiration_secs<GovernanceProposal>(@aptos_framework, proposal_id);
        let locked_until = global<stake::StakePool>(stake_pool).locked_until_secs;
        let remain_zero_1_cond = (spec_proposal_expiration > locked_until || timestamp::spec_now_seconds() > spec_proposal_expiration);
        let record_key = RecordKey {
            stake_pool,
            proposal_id,
        };
        let entirely_voted = spec_has_entirely_voted(stake_pool, proposal_id, record_key);
        aborts_if !remain_zero_1_cond && !exists<VotingRecords>(@aptos_framework);
        include !remain_zero_1_cond && !entirely_voted ==> GetVotingPowerAbortsIf {
            pool_address: stake_pool
        };
        let staking_config = global<staking_config::StakingConfig>(@aptos_framework);
        let spec_voting_power = spec_get_voting_power(stake_pool, staking_config);
        let voting_records_v2 = borrow_global<VotingRecordsV2>(@aptos_framework);
        let used_voting_power = if (smart_table::spec_contains(voting_records_v2.votes, record_key)) {
            smart_table::spec_get(voting_records_v2.votes, record_key)
        } else {
            0
        };
        aborts_if !remain_zero_1_cond && !entirely_voted && features::spec_partial_governance_voting_enabled() &&
            used_voting_power > 0 && spec_voting_power < used_voting_power;
        let remaining_power = spec_get_remaining_voting_power(stake_pool, proposal_id);
        let real_voting_power =  min(voting_power, remaining_power);
        aborts_if !(real_voting_power > 0);
        aborts_if !exists<VotingRecords>(@aptos_framework);
        let voting_records = global<VotingRecords>(@aptos_framework);
        let allow_validator_set_change = global<staking_config::StakingConfig>(@aptos_framework).allow_validator_set_change;
        let stake_pool_res = global<stake::StakePool>(stake_pool);
        aborts_if !exists<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let proposal = table::spec_get(voting_forum.proposals, proposal_id);
        aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
        let proposal_expiration = proposal.expiration_secs;
        let locked_until_secs = global<stake::StakePool>(stake_pool).locked_until_secs;
        aborts_if proposal_expiration > locked_until_secs;
        aborts_if timestamp::now_seconds() > proposal_expiration;
        aborts_if proposal.is_resolved;
        aborts_if !string::spec_internal_check_utf8(voting::IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        let execution_key = utf8(voting::IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        aborts_if simple_map::spec_contains_key(proposal.metadata, execution_key) &&
                  simple_map::spec_get(proposal.metadata, execution_key) != std::bcs::to_bytes(false);
        aborts_if
            if (should_pass) { proposal.yes_votes + real_voting_power > MAX_U128 } else { proposal.no_votes + real_voting_power > MAX_U128 };
        let post post_voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
        aborts_if !string::spec_internal_check_utf8(voting::RESOLVABLE_TIME_METADATA_KEY);
        let key = utf8(voting::RESOLVABLE_TIME_METADATA_KEY);
        ensures simple_map::spec_contains_key(post_proposal.metadata, key);
        ensures simple_map::spec_get(post_proposal.metadata, key) == std::bcs::to_bytes(timestamp::now_seconds());
        aborts_if features::spec_partial_governance_voting_enabled() && used_voting_power + real_voting_power > MAX_U64;
        aborts_if !features::spec_partial_governance_voting_enabled() && table::spec_contains(voting_records.votes, record_key);
        aborts_if !exists<GovernanceEvents>(@aptos_framework);
        let early_resolution_threshold = option::spec_borrow(proposal.early_resolution_vote_threshold);
        let is_voting_period_over = timestamp::spec_now_seconds() > proposal_expiration;
        let new_proposal_yes_votes_0 = proposal.yes_votes + real_voting_power;
        let can_be_resolved_early_0 = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (new_proposal_yes_votes_0 >= early_resolution_threshold ||
                                     proposal.no_votes >= early_resolution_threshold);
        let is_voting_closed_0 = is_voting_period_over || can_be_resolved_early_0;
        let proposal_state_successed_0 = is_voting_closed_0 && new_proposal_yes_votes_0 > proposal.no_votes &&
                                         new_proposal_yes_votes_0 + proposal.no_votes >= proposal.min_vote_threshold;
        let new_proposal_no_votes_0 = proposal.no_votes + real_voting_power;
        let can_be_resolved_early_1 = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (proposal.yes_votes >= early_resolution_threshold ||
                                     new_proposal_no_votes_0 >= early_resolution_threshold);
        let is_voting_closed_1 = is_voting_period_over || can_be_resolved_early_1;
        let proposal_state_successed_1 = is_voting_closed_1 && proposal.yes_votes > new_proposal_no_votes_0 &&
                                         proposal.yes_votes + new_proposal_no_votes_0 >= proposal.min_vote_threshold;
        let new_proposal_yes_votes_1 = proposal.yes_votes + real_voting_power;
        let can_be_resolved_early_2 = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (new_proposal_yes_votes_1 >= early_resolution_threshold ||
                                     proposal.no_votes >= early_resolution_threshold);
        let is_voting_closed_2 = is_voting_period_over || can_be_resolved_early_2;
        let proposal_state_successed_2 = is_voting_closed_2 && new_proposal_yes_votes_1 > proposal.no_votes &&
                                         new_proposal_yes_votes_1 + proposal.no_votes >= proposal.min_vote_threshold;
        let new_proposal_no_votes_1 = proposal.no_votes + real_voting_power;
        let can_be_resolved_early_3 = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (proposal.yes_votes >= early_resolution_threshold ||
                                     new_proposal_no_votes_1 >= early_resolution_threshold);
        let is_voting_closed_3 = is_voting_period_over || can_be_resolved_early_3;
        let proposal_state_successed_3 = is_voting_closed_3 && proposal.yes_votes > new_proposal_no_votes_1 &&
                                         proposal.yes_votes + new_proposal_no_votes_1 >= proposal.min_vote_threshold;
        let post can_be_resolved_early = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (post_proposal.yes_votes >= early_resolution_threshold ||
                                     post_proposal.no_votes >= early_resolution_threshold);
        let post is_voting_closed = is_voting_period_over || can_be_resolved_early;
        let post proposal_state_successed = is_voting_closed && post_proposal.yes_votes > post_proposal.no_votes &&
                                         post_proposal.yes_votes + post_proposal.no_votes >= proposal.min_vote_threshold;
        let execution_hash = proposal.execution_hash;
        let post post_approved_hashes = global<ApprovedExecutionHashes>(@aptos_framework);
    // This enforces ### high&#45;level&#45;req&#45;3
    [#high&#45;level&#45;req](high&#45;level requirement 3):
        aborts_if
            if (should_pass) {
                proposal_state_successed_0 && !exists<ApprovedExecutionHashes>(@aptos_framework)
            } else {
                proposal_state_successed_1 && !exists<ApprovedExecutionHashes>(@aptos_framework)
            };
        aborts_if
            if (should_pass) {
                proposal_state_successed_2 && !exists<ApprovedExecutionHashes>(@aptos_framework)
            } else {
                proposal_state_successed_3 && !exists<ApprovedExecutionHashes>(@aptos_framework)
            };
        ensures proposal_state_successed ==> simple_map::spec_contains_key(post_approved_hashes.hashes, proposal_id) &&
                                             simple_map::spec_get(post_approved_hashes.hashes, proposal_id) == execution_hash;
        aborts_if features::spec_partial_governance_voting_enabled() && !exists<VotingRecordsV2>(@aptos_framework);
    }
}
```


<a id="@Specification_1_add_approved_script_hash_script"></a>

### Function `add_approved_script_hash_script`


```move
module 0x1::aptos_governance {
    public entry fun add_approved_script_hash_script(proposal_id: u64)
}
```



```move
module 0x1::aptos_governance {
    requires chain_status::is_operating();
    include AddApprovedScriptHash;
}
```



<a id="0x1_aptos_governance_AddApprovedScriptHash"></a>


```move
module 0x1::aptos_governance {
    schema AddApprovedScriptHash {
        proposal_id: u64;
        aborts_if !exists<ApprovedExecutionHashes>(@aptos_framework);
        aborts_if !exists<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let proposal = table::spec_get(voting_forum.proposals, proposal_id);
        aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
        let early_resolution_threshold = option::spec_borrow(proposal.early_resolution_vote_threshold);
        aborts_if timestamp::now_seconds() <= proposal.expiration_secs &&
            (option::spec_is_none(proposal.early_resolution_vote_threshold) ||
            proposal.yes_votes < early_resolution_threshold && proposal.no_votes < early_resolution_threshold);
        aborts_if (timestamp::now_seconds() > proposal.expiration_secs ||
            option::spec_is_some(proposal.early_resolution_vote_threshold) && (proposal.yes_votes >= early_resolution_threshold ||
                                                                               proposal.no_votes >= early_resolution_threshold)) &&
            (proposal.yes_votes <= proposal.no_votes || proposal.yes_votes + proposal.no_votes < proposal.min_vote_threshold);
        let post post_approved_hashes = global<ApprovedExecutionHashes>(@aptos_framework);
    // This enforces ### high&#45;level&#45;req&#45;4
    [#high&#45;level&#45;req](high&#45;level requirement 4):
        ensures simple_map::spec_contains_key(post_approved_hashes.hashes, proposal_id) &&
            simple_map::spec_get(post_approved_hashes.hashes, proposal_id) == proposal.execution_hash;
    }
}
```


<a id="@Specification_1_add_approved_script_hash"></a>

### Function `add_approved_script_hash`


```move
module 0x1::aptos_governance {
    public fun add_approved_script_hash(proposal_id: u64)
}
```



```move
module 0x1::aptos_governance {
    requires chain_status::is_operating();
    include AddApprovedScriptHash;
}
```


<a id="@Specification_1_resolve"></a>

### Function `resolve`


```move
module 0x1::aptos_governance {
    public fun resolve(proposal_id: u64, signer_address: address): signer
}
```

Address @aptos_framework must exist ApprovedExecutionHashes and GovernanceProposal and GovernanceResponsbility.


```move
module 0x1::aptos_governance {
    requires chain_status::is_operating();
    include VotingIsProposalResolvableAbortsif;
    let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    let multi_step_key = utf8(voting::IS_MULTI_STEP_PROPOSAL_KEY);
    let has_multi_step_key = simple_map::spec_contains_key(proposal.metadata, multi_step_key);
    let is_multi_step_proposal = aptos_std::from_bcs::deserialize<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    aborts_if has_multi_step_key && !aptos_std::from_bcs::deserializable<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    aborts_if !string::spec_internal_check_utf8(voting::IS_MULTI_STEP_PROPOSAL_KEY);
    aborts_if has_multi_step_key && is_multi_step_proposal;
    let post post_voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
    ensures post_proposal.is_resolved == true && post_proposal.resolution_time_secs == timestamp::now_seconds();
    aborts_if option::spec_is_none(proposal.execution_content);
    aborts_if !exists<ApprovedExecutionHashes>(@aptos_framework);
    let post post_approved_hashes = global<ApprovedExecutionHashes>(@aptos_framework).hashes;
    ensures !simple_map::spec_contains_key(post_approved_hashes, proposal_id);
    include GetSignerAbortsIf;
    let governance_responsibility = global<GovernanceResponsbility>(@aptos_framework);
    let signer_cap = simple_map::spec_get(governance_responsibility.signer_caps, signer_address);
    let addr = signer_cap.account;
    ensures signer::address_of(result) == addr;
}
```


<a id="@Specification_1_resolve_multi_step_proposal"></a>

### Function `resolve_multi_step_proposal`


```move
module 0x1::aptos_governance {
    public fun resolve_multi_step_proposal(proposal_id: u64, signer_address: address, next_execution_hash: vector<u8>): signer
}
```



```move
module 0x1::aptos_governance {
    requires chain_status::is_operating();
    pragma verify_duration_estimate = 120;
    include VotingIsProposalResolvableAbortsif;
    let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    let post post_voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
    aborts_if !string::spec_internal_check_utf8(voting::IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    let multi_step_in_execution_key = utf8(voting::IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    let post is_multi_step_proposal_in_execution_value = simple_map::spec_get(post_proposal.metadata, multi_step_in_execution_key);
    aborts_if !string::spec_internal_check_utf8(voting::IS_MULTI_STEP_PROPOSAL_KEY);
    let multi_step_key = utf8(voting::IS_MULTI_STEP_PROPOSAL_KEY);
    aborts_if simple_map::spec_contains_key(proposal.metadata, multi_step_key) &&
        !aptos_std::from_bcs::deserializable<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    let is_multi_step = simple_map::spec_contains_key(proposal.metadata, multi_step_key) &&
                        aptos_std::from_bcs::deserialize<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    let next_execution_hash_is_empty = len(next_execution_hash) == 0;
    aborts_if !is_multi_step && !next_execution_hash_is_empty;
    aborts_if next_execution_hash_is_empty && is_multi_step && !simple_map::spec_contains_key(proposal.metadata, multi_step_in_execution_key);
    ensures next_execution_hash_is_empty ==> post_proposal.is_resolved == true && post_proposal.resolution_time_secs == timestamp::spec_now_seconds() &&
        if (is_multi_step) {
            is_multi_step_proposal_in_execution_value == std::bcs::serialize(false)
        } else {
            simple_map::spec_contains_key(proposal.metadata, multi_step_in_execution_key) ==>
                is_multi_step_proposal_in_execution_value == std::bcs::serialize(true)
        };
    ensures !next_execution_hash_is_empty ==> post_proposal.execution_hash == next_execution_hash;
    aborts_if !exists<ApprovedExecutionHashes>(@aptos_framework);
    let post post_approved_hashes = global<ApprovedExecutionHashes>(@aptos_framework).hashes;
    ensures next_execution_hash_is_empty ==> !simple_map::spec_contains_key(post_approved_hashes, proposal_id);
    ensures !next_execution_hash_is_empty ==>
        simple_map::spec_get(post_approved_hashes, proposal_id) == next_execution_hash;
    include GetSignerAbortsIf;
    let governance_responsibility = global<GovernanceResponsbility>(@aptos_framework);
    let signer_cap = simple_map::spec_get(governance_responsibility.signer_caps, signer_address);
    let addr = signer_cap.account;
    ensures signer::address_of(result) == addr;
}
```



<a id="0x1_aptos_governance_VotingIsProposalResolvableAbortsif"></a>


```move
module 0x1::aptos_governance {
    schema VotingIsProposalResolvableAbortsif {
        proposal_id: u64;
        aborts_if !exists<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
        let proposal = table::spec_get(voting_forum.proposals, proposal_id);
        aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
        let early_resolution_threshold = option::spec_borrow(proposal.early_resolution_vote_threshold);
        let voting_period_over = timestamp::now_seconds() > proposal.expiration_secs;
        let be_resolved_early = option::spec_is_some(proposal.early_resolution_vote_threshold) &&
                                    (proposal.yes_votes >= early_resolution_threshold ||
                                     proposal.no_votes >= early_resolution_threshold);
        let voting_closed = voting_period_over || be_resolved_early;
        aborts_if voting_closed && (proposal.yes_votes <= proposal.no_votes || proposal.yes_votes + proposal.no_votes < proposal.min_vote_threshold);
        aborts_if !voting_closed;
        aborts_if proposal.is_resolved;
        aborts_if !string::spec_internal_check_utf8(voting::RESOLVABLE_TIME_METADATA_KEY);
        aborts_if !simple_map::spec_contains_key(proposal.metadata, utf8(voting::RESOLVABLE_TIME_METADATA_KEY));
        let resolvable_time = aptos_std::from_bcs::deserialize<u64>(simple_map::spec_get(proposal.metadata, utf8(voting::RESOLVABLE_TIME_METADATA_KEY)));
        aborts_if !aptos_std::from_bcs::deserializable<u64>(simple_map::spec_get(proposal.metadata, utf8(voting::RESOLVABLE_TIME_METADATA_KEY)));
        aborts_if timestamp::now_seconds() <= resolvable_time;
        aborts_if aptos_framework::transaction_context::spec_get_script_hash() != proposal.execution_hash;
    }
}
```


<a id="@Specification_1_remove_approved_hash"></a>

### Function `remove_approved_hash`


```move
module 0x1::aptos_governance {
    public fun remove_approved_hash(proposal_id: u64)
}
```

Address @aptos_framework must exist ApprovedExecutionHashes and GovernanceProposal.


```move
module 0x1::aptos_governance {
    aborts_if !exists<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    aborts_if !exists<ApprovedExecutionHashes>(@aptos_framework);
    let voting_forum = global<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
    aborts_if !exists<voting::VotingForum<GovernanceProposal>>(@aptos_framework);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    aborts_if !proposal.is_resolved;
    let post approved_hashes = global<ApprovedExecutionHashes>(@aptos_framework).hashes;
    ensures !simple_map::spec_contains_key(approved_hashes, proposal_id);
}
```


<a id="@Specification_1_reconfigure"></a>

### Function `reconfigure`


```move
module 0x1::aptos_governance {
    public entry fun reconfigure(aptos_framework: &signer)
}
```



```move
module 0x1::aptos_governance {
    pragma verify = false;
    aborts_if !system_addresses::is_aptos_framework_address(signer::address_of(aptos_framework));
    include reconfiguration_with_dkg::FinishRequirement {
        framework: aptos_framework
    };
    include stake::GetReconfigStartTimeRequirement;
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    requires chain_status::is_operating();
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    requires exists<staking_config::StakingRewardsConfig>(@aptos_framework);
    include staking_config::StakingRewardsConfigRequirement;
}
```


<a id="@Specification_1_force_end_epoch"></a>

### Function `force_end_epoch`


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch(aptos_framework: &signer)
}
```



```move
module 0x1::aptos_governance {
    pragma verify = false;
    let address = signer::address_of(aptos_framework);
    include reconfiguration_with_dkg::FinishRequirement {
        framework: aptos_framework
    };
}
```



<a id="0x1_aptos_governance_VotingInitializationAbortIfs"></a>


```move
module 0x1::aptos_governance {
    schema VotingInitializationAbortIfs {
        aborts_if features::spec_partial_governance_voting_enabled() && !exists<VotingRecordsV2>(@aptos_framework);
    }
}
```


<a id="@Specification_1_force_end_epoch_test_only"></a>

### Function `force_end_epoch_test_only`


```move
module 0x1::aptos_governance {
    public entry fun force_end_epoch_test_only(aptos_framework: &signer)
}
```



```move
module 0x1::aptos_governance {
    pragma verify = false;
}
```


<a id="@Specification_1_toggle_features"></a>

### Function `toggle_features`


```move
module 0x1::aptos_governance {
    public fun toggle_features(aptos_framework: &signer, enable: vector<u64>, disable: vector<u64>)
}
```

Signer address must be @aptos_framework.
Address @aptos_framework must exist GovernanceConfig and GovernanceEvents.


```move
module 0x1::aptos_governance {
    pragma verify = false;
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    include reconfiguration_with_dkg::FinishRequirement {
        framework: aptos_framework
    };
    include stake::GetReconfigStartTimeRequirement;
    include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    requires chain_status::is_operating();
    requires exists<stake::ValidatorFees>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    requires exists<staking_config::StakingRewardsConfig>(@aptos_framework);
    include staking_config::StakingRewardsConfigRequirement;
}
```


<a id="@Specification_1_get_signer_testnet_only"></a>

### Function `get_signer_testnet_only`


```move
module 0x1::aptos_governance {
    public fun get_signer_testnet_only(core_resources: &signer, signer_address: address): signer
}
```

Signer address must be @core_resources.
signer must exist in MintCapStore.
Address @aptos_framework must exist GovernanceResponsbility.


```move
module 0x1::aptos_governance {
    aborts_if signer::address_of(core_resources) != @core_resources;
    aborts_if !exists<aptos_coin::MintCapStore>(signer::address_of(core_resources));
    include GetSignerAbortsIf;
}
```


<a id="@Specification_1_get_voting_power"></a>

### Function `get_voting_power`


```move
module 0x1::aptos_governance {
    #[view]
    public fun get_voting_power(pool_address: address): u64
}
```

Address @aptos_framework must exist StakingConfig.
limit addition overflow.
pool_address must exist in StakePool.


```move
module 0x1::aptos_governance {
    include GetVotingPowerAbortsIf;
    let staking_config = global<staking_config::StakingConfig>(@aptos_framework);
    let allow_validator_set_change = staking_config.allow_validator_set_change;
    let stake_pool_res = global<stake::StakePool>(pool_address);
    ensures allow_validator_set_change ==> result == stake_pool_res.active.value + stake_pool_res.pending_active.value + stake_pool_res.pending_inactive.value;
    ensures !allow_validator_set_change ==> if (stake::spec_is_current_epoch_validator(pool_address)) {
        result == stake_pool_res.active.value + stake_pool_res.pending_inactive.value
    } else {
        result == 0
    };
    ensures result == spec_get_voting_power(pool_address, staking_config);
}
```



<a id="0x1_aptos_governance_spec_get_voting_power"></a>


```move
module 0x1::aptos_governance {
    fun spec_get_voting_power(pool_address: address, staking_config: staking_config::StakingConfig): u64 {
       let allow_validator_set_change = staking_config.allow_validator_set_change;
       let stake_pool_res = global<stake::StakePool>(pool_address);
       if (allow_validator_set_change) {
           stake_pool_res.active.value + stake_pool_res.pending_active.value + stake_pool_res.pending_inactive.value
       } else if (!allow_validator_set_change && (stake::spec_is_current_epoch_validator(pool_address))) {
           stake_pool_res.active.value + stake_pool_res.pending_inactive.value
       } else {
           0
       }
    }
}
```


<a id="@Specification_1_get_signer"></a>

### Function `get_signer`


```move
module 0x1::aptos_governance {
    fun get_signer(signer_address: address): signer
}
```



```move
module 0x1::aptos_governance {
    include GetSignerAbortsIf;
}
```



<a id="0x1_aptos_governance_GetSignerAbortsIf"></a>


```move
module 0x1::aptos_governance {
    schema GetSignerAbortsIf {
        signer_address: address;
        aborts_if !exists<GovernanceResponsbility>(@aptos_framework);
        let cap_map = global<GovernanceResponsbility>(@aptos_framework).signer_caps;
        aborts_if !simple_map::spec_contains_key(cap_map, signer_address);
    }
}
```


<a id="@Specification_1_create_proposal_metadata"></a>

### Function `create_proposal_metadata`


```move
module 0x1::aptos_governance {
    fun create_proposal_metadata(metadata_location: vector<u8>, metadata_hash: vector<u8>): simple_map::SimpleMap<string::String, vector<u8>>
}
```



```move
module 0x1::aptos_governance {
    include CreateProposalMetadataAbortsIf;
}
```



<a id="0x1_aptos_governance_CreateProposalMetadataAbortsIf"></a>


```move
module 0x1::aptos_governance {
    schema CreateProposalMetadataAbortsIf {
        metadata_location: vector<u8>;
        metadata_hash: vector<u8>;
        aborts_if string::length(utf8(metadata_location)) > 256;
        aborts_if string::length(utf8(metadata_hash)) > 256;
        aborts_if !string::spec_internal_check_utf8(metadata_location);
        aborts_if !string::spec_internal_check_utf8(metadata_hash);
        aborts_if !string::spec_internal_check_utf8(METADATA_LOCATION_KEY);
        aborts_if !string::spec_internal_check_utf8(METADATA_HASH_KEY);
    }
}
```


<a id="@Specification_1_assert_voting_initialization"></a>

### Function `assert_voting_initialization`


```move
module 0x1::aptos_governance {
    fun assert_voting_initialization()
}
```



```move
module 0x1::aptos_governance {
    include VotingInitializationAbortIfs;
}
```


<a id="@Specification_1_initialize_for_verification"></a>

### Function `initialize_for_verification`


```move
module 0x1::aptos_governance {
    #[verify_only]
    public fun initialize_for_verification(aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64)
}
```

verify_only


```move
module 0x1::aptos_governance {
    pragma verify = false;
}
```
