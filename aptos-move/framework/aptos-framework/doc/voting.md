
<a id="0x1_voting"></a>

# Module `0x1::voting`


This is the general Voting module that can be used as part of a DAO Governance. Voting is designed to be used by
standalone governance modules, who has full control over the voting flow and is responsible for voting power
calculation and including proper capabilities when creating the proposal so resolution can go through.
On&#45;chain governance of the Aptos network also uses Voting.

The voting flow:
1. The Voting module can be deployed at a known address (e.g. 0x1 for Aptos on&#45;chain governance)
2. The governance module, e.g. AptosGovernance, can be deployed later and define a GovernanceProposal resource type
that can also contain other information such as Capability resource for authorization.
3. The governance module&apos;s owner can then register the ProposalType with Voting. This also hosts the proposal list
(forum) on the calling account.
4. A proposer, through the governance module, can call Voting::create_proposal to create a proposal. create_proposal
cannot be called directly not through the governance module. A script hash of the resolution script that can later
be called to execute the proposal is required.
5. A voter, through the governance module, can call Voting::vote on a proposal. vote requires passing a &amp;ProposalType
and thus only the governance module that registers ProposalType can call vote.
6. Once the proposal&apos;s expiration time has passed and more than the defined threshold has voted yes on the proposal,
anyone can call resolve which returns the content of the proposal (of type ProposalType) that can be used to execute.
7. Only the resolution script with the same script hash specified in the proposal can call Voting::resolve as part of
the resolution process.


-  [Struct `Proposal`](#0x1_voting_Proposal)
-  [Resource `VotingForum`](#0x1_voting_VotingForum)
-  [Struct `VotingEvents`](#0x1_voting_VotingEvents)
-  [Struct `CreateProposal`](#0x1_voting_CreateProposal)
-  [Struct `RegisterForum`](#0x1_voting_RegisterForum)
-  [Struct `Vote`](#0x1_voting_Vote)
-  [Struct `ResolveProposal`](#0x1_voting_ResolveProposal)
-  [Struct `CreateProposalEvent`](#0x1_voting_CreateProposalEvent)
-  [Struct `RegisterForumEvent`](#0x1_voting_RegisterForumEvent)
-  [Struct `VoteEvent`](#0x1_voting_VoteEvent)
-  [Constants](#@Constants_0)
-  [Function `register`](#0x1_voting_register)
-  [Function `create_proposal`](#0x1_voting_create_proposal)
-  [Function `create_proposal_v2`](#0x1_voting_create_proposal_v2)
-  [Function `vote`](#0x1_voting_vote)
-  [Function `is_proposal_resolvable`](#0x1_voting_is_proposal_resolvable)
-  [Function `resolve`](#0x1_voting_resolve)
-  [Function `resolve_proposal_v2`](#0x1_voting_resolve_proposal_v2)
-  [Function `next_proposal_id`](#0x1_voting_next_proposal_id)
-  [Function `get_proposer`](#0x1_voting_get_proposer)
-  [Function `is_voting_closed`](#0x1_voting_is_voting_closed)
-  [Function `can_be_resolved_early`](#0x1_voting_can_be_resolved_early)
-  [Function `get_proposal_metadata`](#0x1_voting_get_proposal_metadata)
-  [Function `get_proposal_metadata_value`](#0x1_voting_get_proposal_metadata_value)
-  [Function `get_proposal_state`](#0x1_voting_get_proposal_state)
-  [Function `get_proposal_creation_secs`](#0x1_voting_get_proposal_creation_secs)
-  [Function `get_proposal_expiration_secs`](#0x1_voting_get_proposal_expiration_secs)
-  [Function `get_execution_hash`](#0x1_voting_get_execution_hash)
-  [Function `get_min_vote_threshold`](#0x1_voting_get_min_vote_threshold)
-  [Function `get_early_resolution_vote_threshold`](#0x1_voting_get_early_resolution_vote_threshold)
-  [Function `get_votes`](#0x1_voting_get_votes)
-  [Function `is_resolved`](#0x1_voting_is_resolved)
-  [Function `get_resolution_time_secs`](#0x1_voting_get_resolution_time_secs)
-  [Function `is_multi_step_proposal_in_execution`](#0x1_voting_is_multi_step_proposal_in_execution)
-  [Function `is_voting_period_over`](#0x1_voting_is_voting_period_over)
-  [Function `get_proposal`](#0x1_voting_get_proposal)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `register`](#@Specification_1_register)
    -  [Function `create_proposal`](#@Specification_1_create_proposal)
    -  [Function `create_proposal_v2`](#@Specification_1_create_proposal_v2)
    -  [Function `vote`](#@Specification_1_vote)
    -  [Function `is_proposal_resolvable`](#@Specification_1_is_proposal_resolvable)
    -  [Function `resolve`](#@Specification_1_resolve)
    -  [Function `resolve_proposal_v2`](#@Specification_1_resolve_proposal_v2)
    -  [Function `next_proposal_id`](#@Specification_1_next_proposal_id)
    -  [Function `get_proposer`](#@Specification_1_get_proposer)
    -  [Function `is_voting_closed`](#@Specification_1_is_voting_closed)
    -  [Function `can_be_resolved_early`](#@Specification_1_can_be_resolved_early)
    -  [Function `get_proposal_metadata`](#@Specification_1_get_proposal_metadata)
    -  [Function `get_proposal_metadata_value`](#@Specification_1_get_proposal_metadata_value)
    -  [Function `get_proposal_state`](#@Specification_1_get_proposal_state)
    -  [Function `get_proposal_creation_secs`](#@Specification_1_get_proposal_creation_secs)
    -  [Function `get_proposal_expiration_secs`](#@Specification_1_get_proposal_expiration_secs)
    -  [Function `get_execution_hash`](#@Specification_1_get_execution_hash)
    -  [Function `get_min_vote_threshold`](#@Specification_1_get_min_vote_threshold)
    -  [Function `get_early_resolution_vote_threshold`](#@Specification_1_get_early_resolution_vote_threshold)
    -  [Function `get_votes`](#@Specification_1_get_votes)
    -  [Function `is_resolved`](#@Specification_1_is_resolved)
    -  [Function `get_resolution_time_secs`](#@Specification_1_get_resolution_time_secs)
    -  [Function `is_multi_step_proposal_in_execution`](#@Specification_1_is_multi_step_proposal_in_execution)
    -  [Function `is_voting_period_over`](#@Specification_1_is_voting_period_over)


```move
module 0x1::voting {
    use 0x1::account;
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::from_bcs;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::simple_map;
    use 0x1::string;
    use 0x1::table;
    use 0x1::timestamp;
    use 0x1::transaction_context;
    use 0x1::type_info;
}
```


<a id="0x1_voting_Proposal"></a>

## Struct `Proposal`

Extra metadata (e.g. description, code url) can be part of the ProposalType struct.


```move
module 0x1::voting {
    struct Proposal<ProposalType: store> has store
}
```


##### Fields


<dl>
<dt>
`proposer: address`
</dt>
<dd>
 Required. The address of the proposer.
</dd>
<dt>
`execution_content: option::Option<ProposalType>`
</dt>
<dd>
 Required. Should contain enough information to execute later, for example the required capability.
 This is stored as an option so we can return it to governance when the proposal is resolved.
</dd>
<dt>
`metadata: simple_map::SimpleMap<string::String, vector<u8>>`
</dt>
<dd>
 Optional. Value is serialized value of an attribute.
 Currently, we have three attributes that are used by the voting flow.
 1. RESOLVABLE_TIME_METADATA_KEY: this is uesed to record the resolvable time to ensure that resolution has to be done non&#45;atomically.
 2. IS_MULTI_STEP_PROPOSAL_KEY: this is used to track if a proposal is single&#45;step or multi&#45;step.
 3. IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY: this attribute only applies to multi&#45;step proposals. A single&#45;step proposal will not have
 this field in its metadata map. The value is used to indicate if a multi&#45;step proposal is in execution. If yes, we will disable further
 voting for this multi&#45;step proposal.
</dd>
<dt>
`creation_time_secs: u64`
</dt>
<dd>
 Timestamp when the proposal was created.
</dd>
<dt>
`execution_hash: vector<u8>`
</dt>
<dd>
 Required. The hash for the execution script module. Only the same exact script module can resolve this
 proposal.
</dd>
<dt>
`min_vote_threshold: u128`
</dt>
<dd>
 A proposal is only resolved if expiration has passed and the number of votes is above threshold.
</dd>
<dt>
`expiration_secs: u64`
</dt>
<dd>

</dd>
<dt>
`early_resolution_vote_threshold: option::Option<u128>`
</dt>
<dd>
 Optional. Early resolution threshold. If specified, the proposal can be resolved early if the total
 number of yes or no votes passes this threshold.
 For example, this can be set to 50% of the total supply of the voting token, so if &gt; 50% vote yes or no,
 the proposal can be resolved before expiration.
</dd>
<dt>
`yes_votes: u128`
</dt>
<dd>
 Number of votes for each outcome.
 u128 since the voting power is already u64 and can add up to more than u64 can hold.
</dd>
<dt>
`no_votes: u128`
</dt>
<dd>

</dd>
<dt>
`is_resolved: bool`
</dt>
<dd>
 Whether the proposal has been resolved.
</dd>
<dt>
`resolution_time_secs: u64`
</dt>
<dd>
 Resolution timestamp if the proposal has been resolved. 0 otherwise.
</dd>
</dl>


<a id="0x1_voting_VotingForum"></a>

## Resource `VotingForum`



```move
module 0x1::voting {
    struct VotingForum<ProposalType: store> has key
}
```


##### Fields


<dl>
<dt>
`proposals: table::Table<u64, voting::Proposal<ProposalType>>`
</dt>
<dd>
 Use Table for execution optimization instead of Vector for gas cost since Vector is read entirely into memory
 during execution while only relevant Table entries are.
</dd>
<dt>
`events: voting::VotingEvents`
</dt>
<dd>

</dd>
<dt>
`next_proposal_id: u64`
</dt>
<dd>
 Unique identifier for a proposal. This allows for 2 &#42; 10&#42;&#42;19 proposals.
</dd>
</dl>


<a id="0x1_voting_VotingEvents"></a>

## Struct `VotingEvents`



```move
module 0x1::voting {
    struct VotingEvents has store
}
```


##### Fields


<dl>
<dt>
`create_proposal_events: event::EventHandle<voting::CreateProposalEvent>`
</dt>
<dd>

</dd>
<dt>
`register_forum_events: event::EventHandle<voting::RegisterForumEvent>`
</dt>
<dd>

</dd>
<dt>
`resolve_proposal_events: event::EventHandle<voting::ResolveProposal>`
</dt>
<dd>

</dd>
<dt>
`vote_events: event::EventHandle<voting::VoteEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_CreateProposal"></a>

## Struct `CreateProposal`



```move
module 0x1::voting {
    #[event]
    struct CreateProposal has drop, store
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
`early_resolution_vote_threshold: option::Option<u128>`
</dt>
<dd>

</dd>
<dt>
`execution_hash: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`expiration_secs: u64`
</dt>
<dd>

</dd>
<dt>
`metadata: simple_map::SimpleMap<string::String, vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`min_vote_threshold: u128`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_RegisterForum"></a>

## Struct `RegisterForum`



```move
module 0x1::voting {
    #[event]
    struct RegisterForum has drop, store
}
```


##### Fields


<dl>
<dt>
`hosting_account: address`
</dt>
<dd>

</dd>
<dt>
`proposal_type_info: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_Vote"></a>

## Struct `Vote`



```move
module 0x1::voting {
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
`num_votes: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_ResolveProposal"></a>

## Struct `ResolveProposal`



```move
module 0x1::voting {
    #[event]
    struct ResolveProposal has drop, store
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
`yes_votes: u128`
</dt>
<dd>

</dd>
<dt>
`no_votes: u128`
</dt>
<dd>

</dd>
<dt>
`resolved_early: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_CreateProposalEvent"></a>

## Struct `CreateProposalEvent`



```move
module 0x1::voting {
    struct CreateProposalEvent has drop, store
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
`early_resolution_vote_threshold: option::Option<u128>`
</dt>
<dd>

</dd>
<dt>
`execution_hash: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`expiration_secs: u64`
</dt>
<dd>

</dd>
<dt>
`metadata: simple_map::SimpleMap<string::String, vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`min_vote_threshold: u128`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_RegisterForumEvent"></a>

## Struct `RegisterForumEvent`



```move
module 0x1::voting {
    struct RegisterForumEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`hosting_account: address`
</dt>
<dd>

</dd>
<dt>
`proposal_type_info: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_voting_VoteEvent"></a>

## Struct `VoteEvent`



```move
module 0x1::voting {
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
`num_votes: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_voting_EINVALID_MIN_VOTE_THRESHOLD"></a>

Minimum vote threshold cannot be higher than early resolution threshold.


```move
module 0x1::voting {
    const EINVALID_MIN_VOTE_THRESHOLD: u64 = 7;
}
```


<a id="0x1_voting_EMULTI_STEP_PROPOSAL_CANNOT_USE_SINGLE_STEP_RESOLVE_FUNCTION"></a>

If a proposal is multi&#45;step, we need to use `resolve_proposal_v2()` to resolve it.
If we use `resolve()` to resolve a multi&#45;step proposal, it will fail with EMULTI_STEP_PROPOSAL_CANNOT_USE_SINGLE_STEP_RESOLVE_FUNCTION.


```move
module 0x1::voting {
    const EMULTI_STEP_PROPOSAL_CANNOT_USE_SINGLE_STEP_RESOLVE_FUNCTION: u64 = 10;
}
```


<a id="0x1_voting_EMULTI_STEP_PROPOSAL_IN_EXECUTION"></a>

Cannot vote if the specified multi&#45;step proposal is in execution.


```move
module 0x1::voting {
    const EMULTI_STEP_PROPOSAL_IN_EXECUTION: u64 = 9;
}
```


<a id="0x1_voting_EPROPOSAL_ALREADY_RESOLVED"></a>

Proposal cannot be resolved more than once


```move
module 0x1::voting {
    const EPROPOSAL_ALREADY_RESOLVED: u64 = 3;
}
```


<a id="0x1_voting_EPROPOSAL_CANNOT_BE_RESOLVED"></a>

Proposal cannot be resolved. Either voting duration has not passed, not enough votes, or fewer yes than no votes


```move
module 0x1::voting {
    const EPROPOSAL_CANNOT_BE_RESOLVED: u64 = 2;
}
```


<a id="0x1_voting_EPROPOSAL_EMPTY_EXECUTION_HASH"></a>

Proposal cannot contain an empty execution script hash


```move
module 0x1::voting {
    const EPROPOSAL_EMPTY_EXECUTION_HASH: u64 = 4;
}
```


<a id="0x1_voting_EPROPOSAL_EXECUTION_HASH_NOT_MATCHING"></a>

Current script&apos;s execution hash does not match the specified proposal&apos;s


```move
module 0x1::voting {
    const EPROPOSAL_EXECUTION_HASH_NOT_MATCHING: u64 = 1;
}
```


<a id="0x1_voting_EPROPOSAL_IS_SINGLE_STEP"></a>

Cannot call `is_multi_step_proposal_in_execution()` on single&#45;step proposals.


```move
module 0x1::voting {
    const EPROPOSAL_IS_SINGLE_STEP: u64 = 12;
}
```


<a id="0x1_voting_EPROPOSAL_VOTING_ALREADY_ENDED"></a>

Proposal&apos;s voting period has already ended.


```move
module 0x1::voting {
    const EPROPOSAL_VOTING_ALREADY_ENDED: u64 = 5;
}
```


<a id="0x1_voting_ERESOLUTION_CANNOT_BE_ATOMIC"></a>

Resolution of a proposal cannot happen atomically in the same transaction as the last vote.


```move
module 0x1::voting {
    const ERESOLUTION_CANNOT_BE_ATOMIC: u64 = 8;
}
```


<a id="0x1_voting_ESINGLE_STEP_PROPOSAL_CANNOT_HAVE_NEXT_EXECUTION_HASH"></a>

If we call `resolve_proposal_v2()` to resolve a single&#45;step proposal, the `next_execution_hash` parameter should be an empty vector.


```move
module 0x1::voting {
    const ESINGLE_STEP_PROPOSAL_CANNOT_HAVE_NEXT_EXECUTION_HASH: u64 = 11;
}
```


<a id="0x1_voting_EVOTING_FORUM_ALREADY_REGISTERED"></a>

Voting forum has already been registered.


```move
module 0x1::voting {
    const EVOTING_FORUM_ALREADY_REGISTERED: u64 = 6;
}
```


<a id="0x1_voting_IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY"></a>

Key used to track if the multi&#45;step proposal is in execution / resolving in progress.


```move
module 0x1::voting {
    const IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY: vector<u8> = [73, 83, 95, 77, 85, 76, 84, 73, 95, 83, 84, 69, 80, 95, 80, 82, 79, 80, 79, 83, 65, 76, 95, 73, 78, 95, 69, 88, 69, 67, 85, 84, 73, 79, 78];
}
```


<a id="0x1_voting_IS_MULTI_STEP_PROPOSAL_KEY"></a>

Key used to track if the proposal is multi&#45;step


```move
module 0x1::voting {
    const IS_MULTI_STEP_PROPOSAL_KEY: vector<u8> = [73, 83, 95, 77, 85, 76, 84, 73, 95, 83, 84, 69, 80, 95, 80, 82, 79, 80, 79, 83, 65, 76, 95, 75, 69, 89];
}
```


<a id="0x1_voting_PROPOSAL_STATE_FAILED"></a>

Proposal has failed because either the min vote threshold is not met or majority voted no.


```move
module 0x1::voting {
    const PROPOSAL_STATE_FAILED: u64 = 3;
}
```


<a id="0x1_voting_PROPOSAL_STATE_PENDING"></a>

ProposalStateEnum representing proposal state.


```move
module 0x1::voting {
    const PROPOSAL_STATE_PENDING: u64 = 0;
}
```


<a id="0x1_voting_PROPOSAL_STATE_SUCCEEDED"></a>



```move
module 0x1::voting {
    const PROPOSAL_STATE_SUCCEEDED: u64 = 1;
}
```


<a id="0x1_voting_RESOLVABLE_TIME_METADATA_KEY"></a>

Key used to track the resolvable time in the proposal&apos;s metadata.


```move
module 0x1::voting {
    const RESOLVABLE_TIME_METADATA_KEY: vector<u8> = [82, 69, 83, 79, 76, 86, 65, 66, 76, 69, 95, 84, 73, 77, 69, 95, 77, 69, 84, 65, 68, 65, 84, 65, 95, 75, 69, 89];
}
```


<a id="0x1_voting_register"></a>

## Function `register`



```move
module 0x1::voting {
    public fun register<ProposalType: store>(account: &signer)
}
```


##### Implementation


```move
module 0x1::voting {
    public fun register<ProposalType: store>(account: &signer) {
        let addr = signer::address_of(account);
        assert!(!exists<VotingForum<ProposalType>>(addr), error::already_exists(EVOTING_FORUM_ALREADY_REGISTERED));

        let voting_forum = VotingForum<ProposalType> {
            next_proposal_id: 0,
            proposals: table::new<u64, Proposal<ProposalType>>(),
            events: VotingEvents {
                create_proposal_events: account::new_event_handle<CreateProposalEvent>(account),
                register_forum_events: account::new_event_handle<RegisterForumEvent>(account),
                resolve_proposal_events: account::new_event_handle<ResolveProposal>(account),
                vote_events: account::new_event_handle<VoteEvent>(account),
            }
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                RegisterForum {
                    hosting_account: addr,
                    proposal_type_info: type_info::type_of<ProposalType>(),
                },
            );
        };
        event::emit_event<RegisterForumEvent>(
            &mut voting_forum.events.register_forum_events,
            RegisterForumEvent {
                hosting_account: addr,
                proposal_type_info: type_info::type_of<ProposalType>(),
            },
        );

        move_to(account, voting_forum);
    }
}
```


<a id="0x1_voting_create_proposal"></a>

## Function `create_proposal`

Create a single&#45;step proposal with the given parameters

@param voting_forum_address The forum&apos;s address where the proposal will be stored.
@param execution_content The execution content that will be given back at resolution time. This can contain
data such as a capability resource used to scope the execution.
@param execution_hash The hash for the execution script module. Only the same exact script module can resolve
this proposal.
@param min_vote_threshold The minimum number of votes needed to consider this proposal successful.
@param expiration_secs The time in seconds at which the proposal expires and can potentially be resolved.
@param early_resolution_vote_threshold The vote threshold for early resolution of this proposal.
@param metadata A simple_map that stores information about this proposal.
@return The proposal id.


```move
module 0x1::voting {
    public fun create_proposal<ProposalType: store>(proposer: address, voting_forum_address: address, execution_content: ProposalType, execution_hash: vector<u8>, min_vote_threshold: u128, expiration_secs: u64, early_resolution_vote_threshold: option::Option<u128>, metadata: simple_map::SimpleMap<string::String, vector<u8>>): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun create_proposal<ProposalType: store>(
        proposer: address,
        voting_forum_address: address,
        execution_content: ProposalType,
        execution_hash: vector<u8>,
        min_vote_threshold: u128,
        expiration_secs: u64,
        early_resolution_vote_threshold: Option<u128>,
        metadata: SimpleMap<String, vector<u8>>,
    ): u64 acquires VotingForum {
        create_proposal_v2(
            proposer,
            voting_forum_address,
            execution_content,
            execution_hash,
            min_vote_threshold,
            expiration_secs,
            early_resolution_vote_threshold,
            metadata,
            false
        )
    }
}
```


<a id="0x1_voting_create_proposal_v2"></a>

## Function `create_proposal_v2`

Create a single&#45;step or a multi&#45;step proposal with the given parameters

@param voting_forum_address The forum&apos;s address where the proposal will be stored.
@param execution_content The execution content that will be given back at resolution time. This can contain
data such as a capability resource used to scope the execution.
@param execution_hash The sha&#45;256 hash for the execution script module. Only the same exact script module can
resolve this proposal.
@param min_vote_threshold The minimum number of votes needed to consider this proposal successful.
@param expiration_secs The time in seconds at which the proposal expires and can potentially be resolved.
@param early_resolution_vote_threshold The vote threshold for early resolution of this proposal.
@param metadata A simple_map that stores information about this proposal.
@param is_multi_step_proposal A bool value that indicates if the proposal is single&#45;step or multi&#45;step.
@return The proposal id.


```move
module 0x1::voting {
    public fun create_proposal_v2<ProposalType: store>(proposer: address, voting_forum_address: address, execution_content: ProposalType, execution_hash: vector<u8>, min_vote_threshold: u128, expiration_secs: u64, early_resolution_vote_threshold: option::Option<u128>, metadata: simple_map::SimpleMap<string::String, vector<u8>>, is_multi_step_proposal: bool): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun create_proposal_v2<ProposalType: store>(
        proposer: address,
        voting_forum_address: address,
        execution_content: ProposalType,
        execution_hash: vector<u8>,
        min_vote_threshold: u128,
        expiration_secs: u64,
        early_resolution_vote_threshold: Option<u128>,
        metadata: SimpleMap<String, vector<u8>>,
        is_multi_step_proposal: bool,
    ): u64 acquires VotingForum {
        if (option::is_some(&early_resolution_vote_threshold)) {
            assert!(
                min_vote_threshold <= *option::borrow(&early_resolution_vote_threshold),
                error::invalid_argument(EINVALID_MIN_VOTE_THRESHOLD),
            );
        };
        // Make sure the execution script's hash is not empty.
        assert!(vector::length(&execution_hash) > 0, error::invalid_argument(EPROPOSAL_EMPTY_EXECUTION_HASH));

        let voting_forum = borrow_global_mut<VotingForum<ProposalType>>(voting_forum_address);
        let proposal_id = voting_forum.next_proposal_id;
        voting_forum.next_proposal_id = voting_forum.next_proposal_id + 1;

        // Add a flag to indicate if this proposal is single-step or multi-step.
        simple_map::add(&mut metadata, utf8(IS_MULTI_STEP_PROPOSAL_KEY), to_bytes(&is_multi_step_proposal));

        let is_multi_step_in_execution_key = utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        if (is_multi_step_proposal) {
            // If the given proposal is a multi-step proposal, we will add a flag to indicate if this multi-step proposal is in execution.
            // This value is by default false. We turn this value to true when we start executing the multi-step proposal. This value
            // will be used to disable further voting after we started executing the multi-step proposal.
            simple_map::add(&mut metadata, is_multi_step_in_execution_key, to_bytes(&false));
            // If the proposal is a single-step proposal, we check if the metadata passed by the client has the IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY key.
            // If they have the key, we will remove it, because a single-step proposal that doesn't need this key.
        } else if (simple_map::contains_key(&mut metadata, &is_multi_step_in_execution_key)) {
            simple_map::remove(&mut metadata, &is_multi_step_in_execution_key);
        };

        table::add(&mut voting_forum.proposals, proposal_id, Proposal {
            proposer,
            creation_time_secs: timestamp::now_seconds(),
            execution_content: option::some<ProposalType>(execution_content),
            execution_hash,
            metadata,
            min_vote_threshold,
            expiration_secs,
            early_resolution_vote_threshold,
            yes_votes: 0,
            no_votes: 0,
            is_resolved: false,
            resolution_time_secs: 0,
        });

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CreateProposal {
                    proposal_id,
                    early_resolution_vote_threshold,
                    execution_hash,
                    expiration_secs,
                    metadata,
                    min_vote_threshold,
                },
            );
        };
        event::emit_event<CreateProposalEvent>(
            &mut voting_forum.events.create_proposal_events,
            CreateProposalEvent {
                proposal_id,
                early_resolution_vote_threshold,
                execution_hash,
                expiration_secs,
                metadata,
                min_vote_threshold,
            },
        );

        proposal_id
    }
}
```


<a id="0x1_voting_vote"></a>

## Function `vote`

Vote on the given proposal.

@param _proof Required so only the governance module that defines ProposalType can initiate voting.
This guarantees that voting eligibility and voting power are controlled by the right governance.
@param voting_forum_address The address of the forum where the proposals are stored.
@param proposal_id The proposal id.
@param num_votes Number of votes. Voting power should be calculated by governance.
@param should_pass Whether the votes are for yes or no.


```move
module 0x1::voting {
    public fun vote<ProposalType: store>(_proof: &ProposalType, voting_forum_address: address, proposal_id: u64, num_votes: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::voting {
    public fun vote<ProposalType: store>(
        _proof: &ProposalType,
        voting_forum_address: address,
        proposal_id: u64,
        num_votes: u64,
        should_pass: bool,
    ) acquires VotingForum {
        let voting_forum = borrow_global_mut<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::borrow_mut(&mut voting_forum.proposals, proposal_id);
        // Voting might still be possible after the proposal has enough yes votes to be resolved early. This would only
        // lead to possible proposal resolution failure if the resolve early threshold is not definitive (e.g. < 50% + 1
        // of the total voting token's supply). In this case, more voting might actually still be desirable.
        // Governance mechanisms built on this voting module can apply additional rules on when voting is closed as
        // appropriate.
        assert!(!is_voting_period_over(proposal), error::invalid_state(EPROPOSAL_VOTING_ALREADY_ENDED));
        assert!(!proposal.is_resolved, error::invalid_state(EPROPOSAL_ALREADY_RESOLVED));
        // Assert this proposal is single-step, or if the proposal is multi-step, it is not in execution yet.
        assert!(!simple_map::contains_key(&proposal.metadata, &utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY))
            || *simple_map::borrow(&proposal.metadata, &utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY)) == to_bytes(
            &false
        ),
            error::invalid_state(EMULTI_STEP_PROPOSAL_IN_EXECUTION));

        if (should_pass) {
            proposal.yes_votes = proposal.yes_votes + (num_votes as u128);
        } else {
            proposal.no_votes = proposal.no_votes + (num_votes as u128);
        };

        // Record the resolvable time to ensure that resolution has to be done non-atomically.
        let timestamp_secs_bytes = to_bytes(&timestamp::now_seconds());
        let key = utf8(RESOLVABLE_TIME_METADATA_KEY);
        if (simple_map::contains_key(&proposal.metadata, &key)) {
            *simple_map::borrow_mut(&mut proposal.metadata, &key) = timestamp_secs_bytes;
        } else {
            simple_map::add(&mut proposal.metadata, key, timestamp_secs_bytes);
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(Vote { proposal_id, num_votes });
        };
        event::emit_event<VoteEvent>(
            &mut voting_forum.events.vote_events,
            VoteEvent { proposal_id, num_votes },
        );
    }
}
```


<a id="0x1_voting_is_proposal_resolvable"></a>

## Function `is_proposal_resolvable`

Common checks on if a proposal is resolvable, regardless if the proposal is single&#45;step or multi&#45;step.


```move
module 0x1::voting {
    fun is_proposal_resolvable<ProposalType: store>(voting_forum_address: address, proposal_id: u64)
}
```


##### Implementation


```move
module 0x1::voting {
    fun is_proposal_resolvable<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ) acquires VotingForum {
        let proposal_state = get_proposal_state<ProposalType>(voting_forum_address, proposal_id);
        assert!(proposal_state == PROPOSAL_STATE_SUCCEEDED, error::invalid_state(EPROPOSAL_CANNOT_BE_RESOLVED));

        let voting_forum = borrow_global_mut<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::borrow_mut(&mut voting_forum.proposals, proposal_id);
        assert!(!proposal.is_resolved, error::invalid_state(EPROPOSAL_ALREADY_RESOLVED));

        // We need to make sure that the resolution is happening in
        // a separate transaction from the last vote to guard against any potential flashloan attacks.
        let resolvable_time = to_u64(*simple_map::borrow(&proposal.metadata, &utf8(RESOLVABLE_TIME_METADATA_KEY)));
        assert!(timestamp::now_seconds() > resolvable_time, error::invalid_state(ERESOLUTION_CANNOT_BE_ATOMIC));

        assert!(
            transaction_context::get_script_hash() == proposal.execution_hash,
            error::invalid_argument(EPROPOSAL_EXECUTION_HASH_NOT_MATCHING),
        );
    }
}
```


<a id="0x1_voting_resolve"></a>

## Function `resolve`

Resolve a single&#45;step proposal with given id. Can only be done if there are at least as many votes as min required and
there are more yes votes than no. If either of these conditions is not met, this will revert.

@param voting_forum_address The address of the forum where the proposals are stored.
@param proposal_id The proposal id.


```move
module 0x1::voting {
    public fun resolve<ProposalType: store>(voting_forum_address: address, proposal_id: u64): ProposalType
}
```


##### Implementation


```move
module 0x1::voting {
    public fun resolve<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): ProposalType acquires VotingForum {
        is_proposal_resolvable<ProposalType>(voting_forum_address, proposal_id);

        let voting_forum = borrow_global_mut<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::borrow_mut(&mut voting_forum.proposals, proposal_id);

        // Assert that the specified proposal is not a multi-step proposal.
        let multi_step_key = utf8(IS_MULTI_STEP_PROPOSAL_KEY);
        let has_multi_step_key = simple_map::contains_key(&proposal.metadata, &multi_step_key);
        if (has_multi_step_key) {
            let is_multi_step_proposal = from_bcs::to_bool(*simple_map::borrow(&proposal.metadata, &multi_step_key));
            assert!(
                !is_multi_step_proposal,
                error::permission_denied(EMULTI_STEP_PROPOSAL_CANNOT_USE_SINGLE_STEP_RESOLVE_FUNCTION)
            );
        };

        let resolved_early = can_be_resolved_early(proposal);
        proposal.is_resolved = true;
        proposal.resolution_time_secs = timestamp::now_seconds();

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                ResolveProposal {
                    proposal_id,
                    yes_votes: proposal.yes_votes,
                    no_votes: proposal.no_votes,
                    resolved_early,
                },
            );
        };
        event::emit_event<ResolveProposal>(
            &mut voting_forum.events.resolve_proposal_events,
            ResolveProposal {
                proposal_id,
                yes_votes: proposal.yes_votes,
                no_votes: proposal.no_votes,
                resolved_early,
            },
        );

        option::extract(&mut proposal.execution_content)
    }
}
```


<a id="0x1_voting_resolve_proposal_v2"></a>

## Function `resolve_proposal_v2`

Resolve a single&#45;step or a multi&#45;step proposal with the given id.
Can only be done if there are at least as many votes as min required and
there are more yes votes than no. If either of these conditions is not met, this will revert.


@param voting_forum_address The address of the forum where the proposals are stored.
@param proposal_id The proposal id.
@param next_execution_hash The next execution hash if the given proposal is multi&#45;step.


```move
module 0x1::voting {
    public fun resolve_proposal_v2<ProposalType: store>(voting_forum_address: address, proposal_id: u64, next_execution_hash: vector<u8>)
}
```


##### Implementation


```move
module 0x1::voting {
    public fun resolve_proposal_v2<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
        next_execution_hash: vector<u8>,
    ) acquires VotingForum {
        is_proposal_resolvable<ProposalType>(voting_forum_address, proposal_id);

        let voting_forum = borrow_global_mut<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::borrow_mut(&mut voting_forum.proposals, proposal_id);

        // Update the IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY key to indicate that the multi-step proposal is in execution.
        let multi_step_in_execution_key = utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        if (simple_map::contains_key(&proposal.metadata, &multi_step_in_execution_key)) {
            let is_multi_step_proposal_in_execution_value = simple_map::borrow_mut(
                &mut proposal.metadata,
                &multi_step_in_execution_key
            );
            *is_multi_step_proposal_in_execution_value = to_bytes(&true);
        };

        let multi_step_key = utf8(IS_MULTI_STEP_PROPOSAL_KEY);
        let is_multi_step = simple_map::contains_key(&proposal.metadata, &multi_step_key) && from_bcs::to_bool(
            *simple_map::borrow(&proposal.metadata, &multi_step_key)
        );
        let next_execution_hash_is_empty = vector::length(&next_execution_hash) == 0;

        // Assert that if this proposal is single-step, the `next_execution_hash` parameter is empty.
        assert!(
            is_multi_step || next_execution_hash_is_empty,
            error::invalid_argument(ESINGLE_STEP_PROPOSAL_CANNOT_HAVE_NEXT_EXECUTION_HASH)
        );

        // If the `next_execution_hash` parameter is empty, it means that either
        // - this proposal is a single-step proposal, or
        // - this proposal is multi-step and we're currently resolving the last step in the multi-step proposal.
        // We can mark that this proposal is resolved.
        if (next_execution_hash_is_empty) {
            proposal.is_resolved = true;
            proposal.resolution_time_secs = timestamp::now_seconds();

            // Set the `IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY` value to false upon successful resolution of the last step of a multi-step proposal.
            if (is_multi_step) {
                let is_multi_step_proposal_in_execution_value = simple_map::borrow_mut(
                    &mut proposal.metadata,
                    &multi_step_in_execution_key
                );
                *is_multi_step_proposal_in_execution_value = to_bytes(&false);
            };
        } else {
            // If the current step is not the last step,
            // update the proposal's execution hash on-chain to the execution hash of the next step.
            proposal.execution_hash = next_execution_hash;
        };

        // For single-step proposals, we emit one `ResolveProposal` event per proposal.
        // For multi-step proposals, we emit one `ResolveProposal` event per step in the multi-step proposal. This means
        // that we emit multiple `ResolveProposal` events for the same multi-step proposal.
        let resolved_early = can_be_resolved_early(proposal);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                ResolveProposal {
                    proposal_id,
                    yes_votes: proposal.yes_votes,
                    no_votes: proposal.no_votes,
                    resolved_early,
                },
            );
        };
        event::emit_event(
            &mut voting_forum.events.resolve_proposal_events,
            ResolveProposal {
                proposal_id,
                yes_votes: proposal.yes_votes,
                no_votes: proposal.no_votes,
                resolved_early,
            },
        );

    }
}
```


<a id="0x1_voting_next_proposal_id"></a>

## Function `next_proposal_id`

Return the next unassigned proposal id


```move
module 0x1::voting {
    #[view]
    public fun next_proposal_id<ProposalType: store>(voting_forum_address: address): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun next_proposal_id<ProposalType: store>(voting_forum_address: address, ): u64 acquires VotingForum {
        let voting_forum = borrow_global<VotingForum<ProposalType>>(voting_forum_address);
        voting_forum.next_proposal_id
    }
}
```


<a id="0x1_voting_get_proposer"></a>

## Function `get_proposer`



```move
module 0x1::voting {
    #[view]
    public fun get_proposer<ProposalType: store>(voting_forum_address: address, proposal_id: u64): address
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposer<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64
    ): address acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.proposer
    }
}
```


<a id="0x1_voting_is_voting_closed"></a>

## Function `is_voting_closed`



```move
module 0x1::voting {
    #[view]
    public fun is_voting_closed<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```


##### Implementation


```move
module 0x1::voting {
    public fun is_voting_closed<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64
    ): bool acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        can_be_resolved_early(proposal) || is_voting_period_over(proposal)
    }
}
```


<a id="0x1_voting_can_be_resolved_early"></a>

## Function `can_be_resolved_early`

Return true if the proposal has reached early resolution threshold (if specified).


```move
module 0x1::voting {
    public fun can_be_resolved_early<ProposalType: store>(proposal: &voting::Proposal<ProposalType>): bool
}
```


##### Implementation


```move
module 0x1::voting {
    public fun can_be_resolved_early<ProposalType: store>(proposal: &Proposal<ProposalType>): bool {
        if (option::is_some(&proposal.early_resolution_vote_threshold)) {
            let early_resolution_threshold = *option::borrow(&proposal.early_resolution_vote_threshold);
            if (proposal.yes_votes >= early_resolution_threshold || proposal.no_votes >= early_resolution_threshold) {
                return true
            };
        };
        false
    }
}
```


<a id="0x1_voting_get_proposal_metadata"></a>

## Function `get_proposal_metadata`



```move
module 0x1::voting {
    #[view]
    public fun get_proposal_metadata<ProposalType: store>(voting_forum_address: address, proposal_id: u64): simple_map::SimpleMap<string::String, vector<u8>>
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposal_metadata<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): SimpleMap<String, vector<u8>> acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.metadata
    }
}
```


<a id="0x1_voting_get_proposal_metadata_value"></a>

## Function `get_proposal_metadata_value`



```move
module 0x1::voting {
    #[view]
    public fun get_proposal_metadata_value<ProposalType: store>(voting_forum_address: address, proposal_id: u64, metadata_key: string::String): vector<u8>
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposal_metadata_value<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
        metadata_key: String,
    ): vector<u8> acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        *simple_map::borrow(&proposal.metadata, &metadata_key)
    }
}
```


<a id="0x1_voting_get_proposal_state"></a>

## Function `get_proposal_state`

Return the state of the proposal with given id.

@param voting_forum_address The address of the forum where the proposals are stored.
@param proposal_id The proposal id.
@return Proposal state as an enum value.


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_state<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposal_state<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): u64 acquires VotingForum {
        if (is_voting_closed<ProposalType>(voting_forum_address, proposal_id)) {
            let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
            let yes_votes = proposal.yes_votes;
            let no_votes = proposal.no_votes;

            if (yes_votes > no_votes && yes_votes + no_votes >= proposal.min_vote_threshold) {
                PROPOSAL_STATE_SUCCEEDED
            } else {
                PROPOSAL_STATE_FAILED
            }
        } else {
            PROPOSAL_STATE_PENDING
        }
    }
}
```


<a id="0x1_voting_get_proposal_creation_secs"></a>

## Function `get_proposal_creation_secs`

Return the proposal&apos;s creation time.


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_creation_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposal_creation_secs<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): u64 acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.creation_time_secs
    }
}
```


<a id="0x1_voting_get_proposal_expiration_secs"></a>

## Function `get_proposal_expiration_secs`

Return the proposal&apos;s expiration time.


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_expiration_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_proposal_expiration_secs<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): u64 acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.expiration_secs
    }
}
```


<a id="0x1_voting_get_execution_hash"></a>

## Function `get_execution_hash`

Return the proposal&apos;s execution hash.


```move
module 0x1::voting {
    #[view]
    public fun get_execution_hash<ProposalType: store>(voting_forum_address: address, proposal_id: u64): vector<u8>
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_execution_hash<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): vector<u8> acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.execution_hash
    }
}
```


<a id="0x1_voting_get_min_vote_threshold"></a>

## Function `get_min_vote_threshold`

Return the proposal&apos;s minimum vote threshold


```move
module 0x1::voting {
    #[view]
    public fun get_min_vote_threshold<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u128
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_min_vote_threshold<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): u128 acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.min_vote_threshold
    }
}
```


<a id="0x1_voting_get_early_resolution_vote_threshold"></a>

## Function `get_early_resolution_vote_threshold`

Return the proposal&apos;s early resolution minimum vote threshold (optionally set)


```move
module 0x1::voting {
    #[view]
    public fun get_early_resolution_vote_threshold<ProposalType: store>(voting_forum_address: address, proposal_id: u64): option::Option<u128>
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_early_resolution_vote_threshold<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): Option<u128> acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.early_resolution_vote_threshold
    }
}
```


<a id="0x1_voting_get_votes"></a>

## Function `get_votes`

Return the proposal&apos;s current vote count (yes_votes, no_votes)


```move
module 0x1::voting {
    #[view]
    public fun get_votes<ProposalType: store>(voting_forum_address: address, proposal_id: u64): (u128, u128)
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_votes<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): (u128, u128) acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        (proposal.yes_votes, proposal.no_votes)
    }
}
```


<a id="0x1_voting_is_resolved"></a>

## Function `is_resolved`

Return true if the governance proposal has already been resolved.


```move
module 0x1::voting {
    #[view]
    public fun is_resolved<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```


##### Implementation


```move
module 0x1::voting {
    public fun is_resolved<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): bool acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.is_resolved
    }
}
```


<a id="0x1_voting_get_resolution_time_secs"></a>

## Function `get_resolution_time_secs`



```move
module 0x1::voting {
    #[view]
    public fun get_resolution_time_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::voting {
    public fun get_resolution_time_secs<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): u64 acquires VotingForum {
        let proposal = get_proposal<ProposalType>(voting_forum_address, proposal_id);
        proposal.resolution_time_secs
    }
}
```


<a id="0x1_voting_is_multi_step_proposal_in_execution"></a>

## Function `is_multi_step_proposal_in_execution`

Return true if the multi&#45;step governance proposal is in execution.


```move
module 0x1::voting {
    #[view]
    public fun is_multi_step_proposal_in_execution<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```


##### Implementation


```move
module 0x1::voting {
    public fun is_multi_step_proposal_in_execution<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): bool acquires VotingForum {
        let voting_forum = borrow_global<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::borrow(&voting_forum.proposals, proposal_id);
        let is_multi_step_in_execution_key = utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        assert!(
            simple_map::contains_key(&proposal.metadata, &is_multi_step_in_execution_key),
            error::invalid_argument(EPROPOSAL_IS_SINGLE_STEP)
        );
        from_bcs::to_bool(*simple_map::borrow(&proposal.metadata, &is_multi_step_in_execution_key))
    }
}
```


<a id="0x1_voting_is_voting_period_over"></a>

## Function `is_voting_period_over`

Return true if the voting period of the given proposal has already ended.


```move
module 0x1::voting {
    fun is_voting_period_over<ProposalType: store>(proposal: &voting::Proposal<ProposalType>): bool
}
```


##### Implementation


```move
module 0x1::voting {
    fun is_voting_period_over<ProposalType: store>(proposal: &Proposal<ProposalType>): bool {
        timestamp::now_seconds() > proposal.expiration_secs
    }
}
```


<a id="0x1_voting_get_proposal"></a>

## Function `get_proposal`



```move
module 0x1::voting {
    fun get_proposal<ProposalType: store>(voting_forum_address: address, proposal_id: u64): &voting::Proposal<ProposalType>
}
```


##### Implementation


```move
module 0x1::voting {
    inline fun get_proposal<ProposalType: store>(
        voting_forum_address: address,
        proposal_id: u64,
    ): &Proposal<ProposalType> acquires VotingForum {
        let voting_forum = borrow_global<VotingForum<ProposalType>>(voting_forum_address);
        table::borrow(&voting_forum.proposals, proposal_id)
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
<td>The proposal ID in a voting forum is unique and always increases monotonically with each new proposal created for that voting forum.</td>
<td>High</td>
<td>The create_proposal and create_proposal_v2 create a new proposal with a unique ID derived from the voting_forum&apos;s next_proposal_id incrementally.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](create_proposal).</td>
</tr>

<tr>
<td>2</td>
<td>While voting, it ensures that only the governance module that defines ProposalType may initiate voting and that the proposal under vote exists in the specified voting forum.</td>
<td>Critical</td>
<td>The vote function verifies the eligibility and validity of a proposal before allowing voting. It ensures that only the correct governance module initiates voting. The function checks if the proposal is currently eligible for voting by confirming it has not resolved and the voting period has not ended.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](vote).</td>
</tr>

<tr>
<td>3</td>
<td>After resolving a single&#45;step proposal, the corresponding proposal is guaranteed to be marked as successfully resolved.</td>
<td>High</td>
<td>Upon invoking the resolve function on a proposal, it undergoes a series of checks to ensure its validity. These include verifying if the proposal exists, is a single&#45;step proposal, and meets the criteria for resolution. If the checks pass, the proposal&apos;s is_resolved flag becomes true, indicating a successful resolution.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](resolve).</td>
</tr>

<tr>
<td>4</td>
<td>In the context of v2 proposal resolving, both single&#45;step and multi&#45;step proposals are accurately handled. It ensures that for single&#45;step proposals, the next execution hash is empty and resolves the proposal, while for multi&#45;step proposals, it guarantees that the next execution hash corresponds to the hash of the next step, maintaining the integrity of the proposal execution sequence.</td>
<td>Medium</td>
<td>The function resolve_proposal_v2 correctly handles both single&#45;step and multi&#45;step proposals. For single&#45;step proposals, it ensures that the next_execution_hash parameter is empty and resolves the proposal. For multi&#45;step proposals, it ensures that the next_execution_hash parameter contains the hash of the next step.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](resolve_proposal_v2).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::voting {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_register"></a>

### Function `register`


```move
module 0x1::voting {
    public fun register<ProposalType: store>(account: &signer)
}
```



```move
module 0x1::voting {
    let addr = signer::address_of(account);
    aborts_if exists<VotingForum<ProposalType>>(addr);
    aborts_if !exists<account::Account>(addr);
    let register_account = global<account::Account>(addr);
    aborts_if register_account.guid_creation_num + 4 >= account::MAX_GUID_CREATION_NUM;
    aborts_if register_account.guid_creation_num + 4 > MAX_U64;
    aborts_if !type_info::spec_is_struct<ProposalType>();
    ensures exists<VotingForum<ProposalType>>(addr);
}
```


<a id="@Specification_1_create_proposal"></a>

### Function `create_proposal`


```move
module 0x1::voting {
    public fun create_proposal<ProposalType: store>(proposer: address, voting_forum_address: address, execution_content: ProposalType, execution_hash: vector<u8>, min_vote_threshold: u128, expiration_secs: u64, early_resolution_vote_threshold: option::Option<u128>, metadata: simple_map::SimpleMap<string::String, vector<u8>>): u64
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    include CreateProposalAbortsIfAndEnsures<ProposalType>{is_multi_step_proposal: false};
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures result == old(global<VotingForum<ProposalType>>(voting_forum_address)).next_proposal_id;
}
```


<a id="@Specification_1_create_proposal_v2"></a>

### Function `create_proposal_v2`


```move
module 0x1::voting {
    public fun create_proposal_v2<ProposalType: store>(proposer: address, voting_forum_address: address, execution_content: ProposalType, execution_hash: vector<u8>, min_vote_threshold: u128, expiration_secs: u64, early_resolution_vote_threshold: option::Option<u128>, metadata: simple_map::SimpleMap<string::String, vector<u8>>, is_multi_step_proposal: bool): u64
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    include CreateProposalAbortsIfAndEnsures<ProposalType>;
    ensures result == old(global<VotingForum<ProposalType>>(voting_forum_address)).next_proposal_id;
}
```



<a id="0x1_voting_CreateProposalAbortsIfAndEnsures"></a>


```move
module 0x1::voting {
    schema CreateProposalAbortsIfAndEnsures<ProposalType> {
        voting_forum_address: address;
        execution_hash: vector<u8>;
        min_vote_threshold: u128;
        early_resolution_vote_threshold: Option<u128>;
        metadata: SimpleMap<String, vector<u8>>;
        is_multi_step_proposal: bool;
        let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
        let proposal_id = voting_forum.next_proposal_id;
        aborts_if !exists<VotingForum<ProposalType>>(voting_forum_address);
        aborts_if table::spec_contains(voting_forum.proposals,proposal_id);
        aborts_if len(early_resolution_vote_threshold.vec) != 0 && min_vote_threshold > early_resolution_vote_threshold.vec[0];
        aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
        aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        aborts_if len(execution_hash) == 0;
        let execution_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
        aborts_if simple_map::spec_contains_key(metadata, execution_key);
        aborts_if voting_forum.next_proposal_id + 1 > MAX_U64;
        let is_multi_step_in_execution_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
        aborts_if is_multi_step_proposal && simple_map::spec_contains_key(metadata, is_multi_step_in_execution_key);
        let post post_voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
        let post post_metadata = table::spec_get(post_voting_forum.proposals, proposal_id).metadata;
        ensures post_voting_forum.next_proposal_id == voting_forum.next_proposal_id + 1;
        ensures table::spec_contains(post_voting_forum.proposals, proposal_id);
        ensures if (is_multi_step_proposal) {
            simple_map::spec_get(post_metadata, is_multi_step_in_execution_key) == std::bcs::serialize(false)
        } else {
            !simple_map::spec_contains_key(post_metadata, is_multi_step_in_execution_key)
        };
    }
}
```


<a id="@Specification_1_vote"></a>

### Function `vote`


```move
module 0x1::voting {
    public fun vote<ProposalType: store>(_proof: &ProposalType, voting_forum_address: address, proposal_id: u64, num_votes: u64, should_pass: bool)
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if !exists<VotingForum<ProposalType>>(voting_forum_address);
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
    aborts_if is_voting_period_over(proposal);
    aborts_if proposal.is_resolved;
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    let execution_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    aborts_if simple_map::spec_contains_key(proposal.metadata, execution_key) &&
              simple_map::spec_get(proposal.metadata, execution_key) != std::bcs::serialize(false);
    aborts_if if (should_pass) { proposal.yes_votes + num_votes > MAX_U128 } else { proposal.no_votes + num_votes > MAX_U128 };
    aborts_if !std::string::spec_internal_check_utf8(RESOLVABLE_TIME_METADATA_KEY);
    let post post_voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
    ensures if (should_pass) {
        post_proposal.yes_votes == proposal.yes_votes + num_votes
    } else {
        post_proposal.no_votes == proposal.no_votes + num_votes
    };
    let timestamp_secs_bytes = std::bcs::serialize(timestamp::spec_now_seconds());
    let key = std::string::spec_utf8(RESOLVABLE_TIME_METADATA_KEY);
    ensures simple_map::spec_get(post_proposal.metadata, key) == timestamp_secs_bytes;
}
```


<a id="@Specification_1_is_proposal_resolvable"></a>

### Function `is_proposal_resolvable`


```move
module 0x1::voting {
    fun is_proposal_resolvable<ProposalType: store>(voting_forum_address: address, proposal_id: u64)
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    include IsProposalResolvableAbortsIf<ProposalType>;
}
```



<a id="0x1_voting_IsProposalResolvableAbortsIf"></a>


```move
module 0x1::voting {
    schema IsProposalResolvableAbortsIf<ProposalType> {
        voting_forum_address: address;
        proposal_id: u64;
        include AbortsIfNotContainProposalID<ProposalType>;
        let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
        let proposal = table::spec_get(voting_forum.proposals, proposal_id);
        let voting_closed = spec_is_voting_closed<ProposalType>(voting_forum_address, proposal_id);
        aborts_if voting_closed && (proposal.yes_votes <= proposal.no_votes || proposal.yes_votes + proposal.no_votes < proposal.min_vote_threshold);
        aborts_if !voting_closed;
        aborts_if proposal.is_resolved;
        aborts_if !std::string::spec_internal_check_utf8(RESOLVABLE_TIME_METADATA_KEY);
        aborts_if !simple_map::spec_contains_key(proposal.metadata, std::string::spec_utf8(RESOLVABLE_TIME_METADATA_KEY));
        aborts_if !from_bcs::deserializable<u64>(simple_map::spec_get(proposal.metadata, std::string::spec_utf8(RESOLVABLE_TIME_METADATA_KEY)));
        aborts_if timestamp::spec_now_seconds() <= from_bcs::deserialize<u64>(simple_map::spec_get(proposal.metadata, std::string::spec_utf8(RESOLVABLE_TIME_METADATA_KEY)));
        aborts_if transaction_context::spec_get_script_hash() != proposal.execution_hash;
    }
}
```


<a id="@Specification_1_resolve"></a>

### Function `resolve`


```move
module 0x1::voting {
    public fun resolve<ProposalType: store>(voting_forum_address: address, proposal_id: u64): ProposalType
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    include IsProposalResolvableAbortsIf<ProposalType>;
    aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    let multi_step_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
    let has_multi_step_key = simple_map::spec_contains_key(proposal.metadata, multi_step_key);
    aborts_if has_multi_step_key && !from_bcs::deserializable<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    aborts_if has_multi_step_key && from_bcs::deserialize<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    let post post_voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures post_proposal.is_resolved == true;
    ensures post_proposal.resolution_time_secs == timestamp::spec_now_seconds();
    aborts_if option::spec_is_none(proposal.execution_content);
    ensures result == option::spec_borrow(proposal.execution_content);
    ensures option::spec_is_none(post_proposal.execution_content);
}
```


<a id="@Specification_1_resolve_proposal_v2"></a>

### Function `resolve_proposal_v2`


```move
module 0x1::voting {
    public fun resolve_proposal_v2<ProposalType: store>(voting_forum_address: address, proposal_id: u64, next_execution_hash: vector<u8>)
}
```



```move
module 0x1::voting {
    pragma verify_duration_estimate = 300;
    requires chain_status::is_operating();
    include IsProposalResolvableAbortsIf<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    let post post_voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let post post_proposal = table::spec_get(post_voting_forum.proposals, proposal_id);
    let multi_step_in_execution_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
    ensures (simple_map::spec_contains_key(proposal.metadata, multi_step_in_execution_key) && len(next_execution_hash) != 0) ==>
        simple_map::spec_get(post_proposal.metadata, multi_step_in_execution_key) == std::bcs::serialize(true);
    ensures (simple_map::spec_contains_key(proposal.metadata, multi_step_in_execution_key) &&
        (len(next_execution_hash) == 0 && !is_multi_step)) ==>
        simple_map::spec_get(post_proposal.metadata, multi_step_in_execution_key) == std::bcs::serialize(true);
    let multi_step_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_KEY);
    aborts_if simple_map::spec_contains_key(proposal.metadata, multi_step_key) &&
        !from_bcs::deserializable<bool>(simple_map::spec_get(proposal.metadata, multi_step_key));
    let is_multi_step = simple_map::spec_contains_key(proposal.metadata, multi_step_key) &&
        from_bcs::deserialize(simple_map::spec_get(proposal.metadata, multi_step_key));
    aborts_if !is_multi_step && len(next_execution_hash) != 0;
    aborts_if len(next_execution_hash) == 0 && !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if len(next_execution_hash) == 0 && is_multi_step && !simple_map::spec_contains_key(proposal.metadata, multi_step_in_execution_key);
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    ensures len(next_execution_hash) == 0 ==> post_proposal.resolution_time_secs == timestamp::spec_now_seconds();
    ensures len(next_execution_hash) == 0 ==> post_proposal.is_resolved == true;
    ensures (len(next_execution_hash) == 0 && is_multi_step) ==> simple_map::spec_get(post_proposal.metadata, multi_step_in_execution_key) == std::bcs::serialize(false);
    ensures len(next_execution_hash) != 0 ==> post_proposal.execution_hash == next_execution_hash;
}
```


<a id="@Specification_1_next_proposal_id"></a>

### Function `next_proposal_id`


```move
module 0x1::voting {
    #[view]
    public fun next_proposal_id<ProposalType: store>(voting_forum_address: address): u64
}
```



```move
module 0x1::voting {
    aborts_if !exists<VotingForum<ProposalType>>(voting_forum_address);
    ensures result == global<VotingForum<ProposalType>>(voting_forum_address).next_proposal_id;
}
```


<a id="@Specification_1_get_proposer"></a>

### Function `get_proposer`


```move
module 0x1::voting {
    #[view]
    public fun get_proposer<ProposalType: store>(voting_forum_address: address, proposal_id: u64): address
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.proposer;
}
```


<a id="@Specification_1_is_voting_closed"></a>

### Function `is_voting_closed`


```move
module 0x1::voting {
    #[view]
    public fun is_voting_closed<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    include AbortsIfNotContainProposalID<ProposalType>;
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    ensures result == spec_is_voting_closed<ProposalType>(voting_forum_address, proposal_id);
}
```



<a id="0x1_voting_spec_is_voting_closed"></a>


```move
module 0x1::voting {
    fun spec_is_voting_closed<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool {
       let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
       let proposal = table::spec_get(voting_forum.proposals, proposal_id);
       spec_can_be_resolved_early<ProposalType>(proposal) || is_voting_period_over(proposal)
    }
}
```


<a id="@Specification_1_can_be_resolved_early"></a>

### Function `can_be_resolved_early`


```move
module 0x1::voting {
    public fun can_be_resolved_early<ProposalType: store>(proposal: &voting::Proposal<ProposalType>): bool
}
```



```move
module 0x1::voting {
    aborts_if false;
    ensures result == spec_can_be_resolved_early<ProposalType>(proposal);
}
```



<a id="0x1_voting_spec_can_be_resolved_early"></a>


```move
module 0x1::voting {
    fun spec_can_be_resolved_early<ProposalType: store>(proposal: Proposal<ProposalType>): bool {
       if (option::spec_is_some(proposal.early_resolution_vote_threshold)) {
           let early_resolution_threshold = option::spec_borrow(proposal.early_resolution_vote_threshold);
           if (proposal.yes_votes >= early_resolution_threshold || proposal.no_votes >= early_resolution_threshold) {
               true
           } else{
               false
           }
       } else {
           false
       }
    }
}
```



<a id="0x1_voting_spec_get_proposal_state"></a>


```move
module 0x1::voting {
    fun spec_get_proposal_state<ProposalType>(
       voting_forum_address: address,
       proposal_id: u64,
       voting_forum: VotingForum<ProposalType>
    ): u64 {
       let proposal = table::spec_get(voting_forum.proposals, proposal_id);
       let voting_closed = spec_is_voting_closed<ProposalType>(voting_forum_address, proposal_id);
       let proposal_vote_cond = (proposal.yes_votes > proposal.no_votes && proposal.yes_votes + proposal.no_votes >= proposal.min_vote_threshold);
       if (voting_closed && proposal_vote_cond) {
           PROPOSAL_STATE_SUCCEEDED
       } else if (voting_closed && !proposal_vote_cond) {
           PROPOSAL_STATE_FAILED
       } else {
           PROPOSAL_STATE_PENDING
       }
    }
}
```



<a id="0x1_voting_spec_get_proposal_expiration_secs"></a>


```move
module 0x1::voting {
    fun spec_get_proposal_expiration_secs<ProposalType: store>(
       voting_forum_address: address,
       proposal_id: u64,
    ): u64 {
       let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
       let proposal = table::spec_get(voting_forum.proposals, proposal_id);
       proposal.expiration_secs
    }
}
```


<a id="@Specification_1_get_proposal_metadata"></a>

### Function `get_proposal_metadata`


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_metadata<ProposalType: store>(voting_forum_address: address, proposal_id: u64): simple_map::SimpleMap<string::String, vector<u8>>
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.metadata;
}
```


<a id="@Specification_1_get_proposal_metadata_value"></a>

### Function `get_proposal_metadata_value`


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_metadata_value<ProposalType: store>(voting_forum_address: address, proposal_id: u64, metadata_key: string::String): vector<u8>
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    aborts_if !simple_map::spec_contains_key(proposal.metadata, metadata_key);
    ensures result == simple_map::spec_get(proposal.metadata, metadata_key);
}
```


<a id="@Specification_1_get_proposal_state"></a>

### Function `get_proposal_state`


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_state<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```



```move
module 0x1::voting {
    pragma addition_overflow_unchecked;
    requires chain_status::is_operating();
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    ensures result == spec_get_proposal_state(voting_forum_address, proposal_id, voting_forum);
}
```


<a id="@Specification_1_get_proposal_creation_secs"></a>

### Function `get_proposal_creation_secs`


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_creation_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.creation_time_secs;
}
```


<a id="@Specification_1_get_proposal_expiration_secs"></a>

### Function `get_proposal_expiration_secs`


```move
module 0x1::voting {
    #[view]
    public fun get_proposal_expiration_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    ensures result == spec_get_proposal_expiration_secs<ProposalType>(voting_forum_address, proposal_id);
}
```


<a id="@Specification_1_get_execution_hash"></a>

### Function `get_execution_hash`


```move
module 0x1::voting {
    #[view]
    public fun get_execution_hash<ProposalType: store>(voting_forum_address: address, proposal_id: u64): vector<u8>
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.execution_hash;
}
```


<a id="@Specification_1_get_min_vote_threshold"></a>

### Function `get_min_vote_threshold`


```move
module 0x1::voting {
    #[view]
    public fun get_min_vote_threshold<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u128
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.min_vote_threshold;
}
```


<a id="@Specification_1_get_early_resolution_vote_threshold"></a>

### Function `get_early_resolution_vote_threshold`


```move
module 0x1::voting {
    #[view]
    public fun get_early_resolution_vote_threshold<ProposalType: store>(voting_forum_address: address, proposal_id: u64): option::Option<u128>
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.early_resolution_vote_threshold;
}
```


<a id="@Specification_1_get_votes"></a>

### Function `get_votes`


```move
module 0x1::voting {
    #[view]
    public fun get_votes<ProposalType: store>(voting_forum_address: address, proposal_id: u64): (u128, u128)
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result_1 == proposal.yes_votes;
    ensures result_2 == proposal.no_votes;
}
```


<a id="@Specification_1_is_resolved"></a>

### Function `is_resolved`


```move
module 0x1::voting {
    #[view]
    public fun is_resolved<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.is_resolved;
}
```



<a id="0x1_voting_AbortsIfNotContainProposalID"></a>


```move
module 0x1::voting {
    schema AbortsIfNotContainProposalID<ProposalType> {
        proposal_id: u64;
        voting_forum_address: address;
        let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
        aborts_if !table::spec_contains(voting_forum.proposals, proposal_id);
        aborts_if !exists<VotingForum<ProposalType>>(voting_forum_address);
    }
}
```


<a id="@Specification_1_get_resolution_time_secs"></a>

### Function `get_resolution_time_secs`


```move
module 0x1::voting {
    #[view]
    public fun get_resolution_time_secs<ProposalType: store>(voting_forum_address: address, proposal_id: u64): u64
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals, proposal_id);
    ensures result == proposal.resolution_time_secs;
}
```


<a id="@Specification_1_is_multi_step_proposal_in_execution"></a>

### Function `is_multi_step_proposal_in_execution`


```move
module 0x1::voting {
    #[view]
    public fun is_multi_step_proposal_in_execution<ProposalType: store>(voting_forum_address: address, proposal_id: u64): bool
}
```



```move
module 0x1::voting {
    include AbortsIfNotContainProposalID<ProposalType>;
    let voting_forum = global<VotingForum<ProposalType>>(voting_forum_address);
    let proposal = table::spec_get(voting_forum.proposals,proposal_id);
    aborts_if !std::string::spec_internal_check_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    let execution_key = std::string::spec_utf8(IS_MULTI_STEP_PROPOSAL_IN_EXECUTION_KEY);
    aborts_if !simple_map::spec_contains_key(proposal.metadata,execution_key);
    let is_multi_step_in_execution_key = simple_map::spec_get(proposal.metadata,execution_key);
    aborts_if !from_bcs::deserializable<bool>(is_multi_step_in_execution_key);
    ensures result == from_bcs::deserialize<bool>(is_multi_step_in_execution_key);
}
```


<a id="@Specification_1_is_voting_period_over"></a>

### Function `is_voting_period_over`


```move
module 0x1::voting {
    fun is_voting_period_over<ProposalType: store>(proposal: &voting::Proposal<ProposalType>): bool
}
```



```move
module 0x1::voting {
    requires chain_status::is_operating();
    aborts_if false;
    ensures result == (timestamp::spec_now_seconds() > proposal.expiration_secs);
}
```
