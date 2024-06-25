
<a id="0x1_delegation_pool"></a>

# Module `0x1::delegation_pool`


Allow multiple delegators to participate in the same stake pool in order to collect the minimum
stake required to join the validator set. Delegators are rewarded out of the validator rewards
proportionally to their stake and provided the same stake&#45;management API as the stake pool owner.

The main accounting logic in the delegation pool contract handles the following:
1. Tracks how much stake each delegator owns, privately deposited as well as earned.
Accounting individual delegator stakes is achieved through the shares&#45;based pool defined at
<code>aptos_std::pool_u64</code>, hence delegators own shares rather than absolute stakes into the delegation pool.
2. Tracks rewards earned by the stake pool, implicitly by the delegation one, in the meantime
and distribute them accordingly.
3. Tracks lockup cycles on the stake pool in order to separate inactive stake (not earning rewards)
from pending_inactive stake (earning rewards) and allow its delegators to withdraw the former.
4. Tracks how much commission fee has to be paid to the operator out of incoming rewards before
distributing them to the internal pool_u64 pools.

In order to distinguish between stakes in different states and route rewards accordingly,
separate pool_u64 pools are used for individual stake states:
1. one of <code>active</code> &#43; <code>pending_active</code> stake
2. one of <code>inactive</code> stake FOR each past observed lockup cycle (OLC) on the stake pool
3. one of <code>pending_inactive</code> stake scheduled during this ongoing OLC

As stake&#45;state transitions and rewards are computed only at the stake pool level, the delegation pool
gets outdated. To mitigate this, at any interaction with the delegation pool, a process of synchronization
to the underlying stake pool is executed before the requested operation itself.

At synchronization:
&#45; stake deviations between the two pools are actually the rewards produced in the meantime.
&#45; the commission fee is extracted from the rewards, the remaining stake is distributed to the internal
pool_u64 pools and then the commission stake used to buy shares for operator.
&#45; if detecting that the lockup expired on the stake pool, the delegation pool will isolate its
pending_inactive stake (now inactive) and create a new pool_u64 to host future pending_inactive stake
scheduled this newly started lockup.
Detecting a lockup expiration on the stake pool resumes to detecting new inactive stake.

Accounting main invariants:
&#45; each stake&#45;management operation (add/unlock/reactivate/withdraw) and operator change triggers
the synchronization process before executing its own function.
&#45; each OLC maps to one or more real lockups on the stake pool, but not the opposite. Actually, only a real
lockup with &apos;activity&apos; (which inactivated some unlocking stake) triggers the creation of a new OLC.
&#45; unlocking and/or unlocked stake originating from different real lockups are never mixed together into
the same pool_u64. This invalidates the accounting of which rewards belong to whom.
&#45; no delegator can have unlocking and/or unlocked stake (pending withdrawals) in different OLCs. This ensures
delegators do not have to keep track of the OLCs when they unlocked. When creating a new pending withdrawal,
the existing one is executed (withdrawn) if is already inactive.
&#45; <code>add_stake</code> fees are always refunded, but only after the epoch when they have been charged ends.
&#45; withdrawing pending_inactive stake (when validator had gone inactive before its lockup expired)
does not inactivate any stake additional to the requested one to ensure OLC would not advance indefinitely.
&#45; the pending withdrawal exists at an OLC iff delegator owns some shares within the shares pool of that OLC.

Example flow:
<ol>
<li>A node operator creates a delegation pool by calling
<code>initialize_delegation_pool</code> and sets
its commission fee to 0% (for simplicity). A stake pool is created with no initial stake and owned by
a resource account controlled by the delegation pool.</li>
<li>Delegator A adds 100 stake which is converted to 100 shares into the active pool_u64</li>
<li>Operator joins the validator set as the stake pool has now the minimum stake</li>
<li>The stake pool earned rewards and now has 200 active stake. A&apos;s active shares are worth 200 coins as
the commission fee is 0%.</li>
<li></li>
<ol>
<li>A requests <code>unlock</code> for 100 stake</li>
<li>Synchronization detects 200 &#45; 100 active rewards which are entirely (0% commission) added to the active pool.</li>
<li>100 coins &#61; (100 &#42; 100) / 200 &#61; 50 shares are redeemed from the active pool and exchanged for 100 shares
into the pending_inactive one on A&apos;s behalf</li>
</ol>
<li>Delegator B adds 200 stake which is converted to (200 &#42; 50) / 100 &#61; 100 shares into the active pool</li>
<li>The stake pool earned rewards and now has 600 active and 200 pending_inactive stake.</li>
<li></li>
<ol>
<li>A requests <code>reactivate_stake</code> for 100 stake</li>
<li>
Synchronization detects 600 &#45; 300 active and 200 &#45; 100 pending_inactive rewards which are both entirely
distributed to their corresponding pools
</li>
<li>
100 coins &#61; (100 &#42; 100) / 200 &#61; 50 shares are redeemed from the pending_inactive pool and exchanged for
(100 &#42; 150) / 600 &#61; 25 shares into the active one on A&apos;s behalf
</li>
</ol>
<li>The lockup expires on the stake pool, inactivating the entire pending_inactive stake</li>
<li></li>
<ol>
<li>B requests <code>unlock</code> for 100 stake</li>
<li>
Synchronization detects no active or pending_inactive rewards, but 0 &#45;&gt; 100 inactive stake on the stake pool,
so it advances the observed lockup cycle and creates a pool_u64 for the new lockup, hence allowing previous
pending_inactive shares to be redeemed</li>
<li>
100 coins &#61; (100 &#42; 175) / 700 &#61; 25 shares are redeemed from the active pool and exchanged for 100 shares
into the new pending_inactive one on B&apos;s behalf
</li>
</ol>
<li>The stake pool earned rewards and now has some pending_inactive rewards.</li>
<li></li>
<ol>
<li>A requests <code>withdraw</code> for its entire inactive stake</li>
<li>
Synchronization detects no new inactive stake, but some pending_inactive rewards which are distributed
to the (2nd) pending_inactive pool
</li>
<li>
A&apos;s 50 shares &#61; (50 &#42; 100) / 50 &#61; 100 coins are redeemed from the (1st) inactive pool and 100 stake is
transferred to A
</li>
</ol>
</ol>



-  [Resource `DelegationPoolOwnership`](#0x1_delegation_pool_DelegationPoolOwnership)
-  [Struct `ObservedLockupCycle`](#0x1_delegation_pool_ObservedLockupCycle)
-  [Resource `DelegationPool`](#0x1_delegation_pool_DelegationPool)
-  [Struct `VotingRecordKey`](#0x1_delegation_pool_VotingRecordKey)
-  [Struct `VoteDelegation`](#0x1_delegation_pool_VoteDelegation)
-  [Struct `DelegatedVotes`](#0x1_delegation_pool_DelegatedVotes)
-  [Resource `GovernanceRecords`](#0x1_delegation_pool_GovernanceRecords)
-  [Resource `BeneficiaryForOperator`](#0x1_delegation_pool_BeneficiaryForOperator)
-  [Resource `NextCommissionPercentage`](#0x1_delegation_pool_NextCommissionPercentage)
-  [Resource `DelegationPoolAllowlisting`](#0x1_delegation_pool_DelegationPoolAllowlisting)
-  [Struct `AddStake`](#0x1_delegation_pool_AddStake)
-  [Struct `AddStakeEvent`](#0x1_delegation_pool_AddStakeEvent)
-  [Struct `ReactivateStake`](#0x1_delegation_pool_ReactivateStake)
-  [Struct `ReactivateStakeEvent`](#0x1_delegation_pool_ReactivateStakeEvent)
-  [Struct `UnlockStake`](#0x1_delegation_pool_UnlockStake)
-  [Struct `UnlockStakeEvent`](#0x1_delegation_pool_UnlockStakeEvent)
-  [Struct `WithdrawStake`](#0x1_delegation_pool_WithdrawStake)
-  [Struct `WithdrawStakeEvent`](#0x1_delegation_pool_WithdrawStakeEvent)
-  [Struct `DistributeCommissionEvent`](#0x1_delegation_pool_DistributeCommissionEvent)
-  [Struct `DistributeCommission`](#0x1_delegation_pool_DistributeCommission)
-  [Struct `Vote`](#0x1_delegation_pool_Vote)
-  [Struct `VoteEvent`](#0x1_delegation_pool_VoteEvent)
-  [Struct `CreateProposal`](#0x1_delegation_pool_CreateProposal)
-  [Struct `CreateProposalEvent`](#0x1_delegation_pool_CreateProposalEvent)
-  [Struct `DelegateVotingPower`](#0x1_delegation_pool_DelegateVotingPower)
-  [Struct `DelegateVotingPowerEvent`](#0x1_delegation_pool_DelegateVotingPowerEvent)
-  [Struct `SetBeneficiaryForOperator`](#0x1_delegation_pool_SetBeneficiaryForOperator)
-  [Struct `CommissionPercentageChange`](#0x1_delegation_pool_CommissionPercentageChange)
-  [Struct `EnableDelegatorsAllowlisting`](#0x1_delegation_pool_EnableDelegatorsAllowlisting)
-  [Struct `DisableDelegatorsAllowlisting`](#0x1_delegation_pool_DisableDelegatorsAllowlisting)
-  [Struct `AllowlistDelegator`](#0x1_delegation_pool_AllowlistDelegator)
-  [Struct `RemoveDelegatorFromAllowlist`](#0x1_delegation_pool_RemoveDelegatorFromAllowlist)
-  [Struct `EvictDelegator`](#0x1_delegation_pool_EvictDelegator)
-  [Constants](#@Constants_0)
-  [Function `owner_cap_exists`](#0x1_delegation_pool_owner_cap_exists)
-  [Function `get_owned_pool_address`](#0x1_delegation_pool_get_owned_pool_address)
-  [Function `delegation_pool_exists`](#0x1_delegation_pool_delegation_pool_exists)
-  [Function `partial_governance_voting_enabled`](#0x1_delegation_pool_partial_governance_voting_enabled)
-  [Function `observed_lockup_cycle`](#0x1_delegation_pool_observed_lockup_cycle)
-  [Function `is_next_commission_percentage_effective`](#0x1_delegation_pool_is_next_commission_percentage_effective)
-  [Function `operator_commission_percentage`](#0x1_delegation_pool_operator_commission_percentage)
-  [Function `operator_commission_percentage_next_lockup_cycle`](#0x1_delegation_pool_operator_commission_percentage_next_lockup_cycle)
-  [Function `shareholders_count_active_pool`](#0x1_delegation_pool_shareholders_count_active_pool)
-  [Function `get_delegation_pool_stake`](#0x1_delegation_pool_get_delegation_pool_stake)
-  [Function `get_pending_withdrawal`](#0x1_delegation_pool_get_pending_withdrawal)
-  [Function `get_stake`](#0x1_delegation_pool_get_stake)
-  [Function `get_add_stake_fee`](#0x1_delegation_pool_get_add_stake_fee)
-  [Function `can_withdraw_pending_inactive`](#0x1_delegation_pool_can_withdraw_pending_inactive)
-  [Function `calculate_and_update_voter_total_voting_power`](#0x1_delegation_pool_calculate_and_update_voter_total_voting_power)
-  [Function `calculate_and_update_remaining_voting_power`](#0x1_delegation_pool_calculate_and_update_remaining_voting_power)
-  [Function `calculate_and_update_delegator_voter`](#0x1_delegation_pool_calculate_and_update_delegator_voter)
-  [Function `calculate_and_update_voting_delegation`](#0x1_delegation_pool_calculate_and_update_voting_delegation)
-  [Function `get_expected_stake_pool_address`](#0x1_delegation_pool_get_expected_stake_pool_address)
-  [Function `min_remaining_secs_for_commission_change`](#0x1_delegation_pool_min_remaining_secs_for_commission_change)
-  [Function `allowlisting_enabled`](#0x1_delegation_pool_allowlisting_enabled)
-  [Function `delegator_allowlisted`](#0x1_delegation_pool_delegator_allowlisted)
-  [Function `get_delegators_allowlist`](#0x1_delegation_pool_get_delegators_allowlist)
-  [Function `initialize_delegation_pool`](#0x1_delegation_pool_initialize_delegation_pool)
-  [Function `beneficiary_for_operator`](#0x1_delegation_pool_beneficiary_for_operator)
-  [Function `enable_partial_governance_voting`](#0x1_delegation_pool_enable_partial_governance_voting)
-  [Function `vote`](#0x1_delegation_pool_vote)
-  [Function `create_proposal`](#0x1_delegation_pool_create_proposal)
-  [Function `assert_owner_cap_exists`](#0x1_delegation_pool_assert_owner_cap_exists)
-  [Function `assert_delegation_pool_exists`](#0x1_delegation_pool_assert_delegation_pool_exists)
-  [Function `assert_min_active_balance`](#0x1_delegation_pool_assert_min_active_balance)
-  [Function `assert_min_pending_inactive_balance`](#0x1_delegation_pool_assert_min_pending_inactive_balance)
-  [Function `assert_partial_governance_voting_enabled`](#0x1_delegation_pool_assert_partial_governance_voting_enabled)
-  [Function `assert_allowlisting_enabled`](#0x1_delegation_pool_assert_allowlisting_enabled)
-  [Function `assert_delegator_allowlisted`](#0x1_delegation_pool_assert_delegator_allowlisted)
-  [Function `coins_to_redeem_to_ensure_min_stake`](#0x1_delegation_pool_coins_to_redeem_to_ensure_min_stake)
-  [Function `coins_to_transfer_to_ensure_min_stake`](#0x1_delegation_pool_coins_to_transfer_to_ensure_min_stake)
-  [Function `retrieve_stake_pool_owner`](#0x1_delegation_pool_retrieve_stake_pool_owner)
-  [Function `get_pool_address`](#0x1_delegation_pool_get_pool_address)
-  [Function `get_delegator_active_shares`](#0x1_delegation_pool_get_delegator_active_shares)
-  [Function `get_delegator_pending_inactive_shares`](#0x1_delegation_pool_get_delegator_pending_inactive_shares)
-  [Function `get_used_voting_power`](#0x1_delegation_pool_get_used_voting_power)
-  [Function `create_resource_account_seed`](#0x1_delegation_pool_create_resource_account_seed)
-  [Function `borrow_mut_used_voting_power`](#0x1_delegation_pool_borrow_mut_used_voting_power)
-  [Function `update_and_borrow_mut_delegator_vote_delegation`](#0x1_delegation_pool_update_and_borrow_mut_delegator_vote_delegation)
-  [Function `update_and_borrow_mut_delegated_votes`](#0x1_delegation_pool_update_and_borrow_mut_delegated_votes)
-  [Function `olc_with_index`](#0x1_delegation_pool_olc_with_index)
-  [Function `calculate_total_voting_power`](#0x1_delegation_pool_calculate_total_voting_power)
-  [Function `calculate_and_update_delegator_voter_internal`](#0x1_delegation_pool_calculate_and_update_delegator_voter_internal)
-  [Function `calculate_and_update_delegated_votes`](#0x1_delegation_pool_calculate_and_update_delegated_votes)
-  [Function `borrow_mut_delegators_allowlist`](#0x1_delegation_pool_borrow_mut_delegators_allowlist)
-  [Function `set_operator`](#0x1_delegation_pool_set_operator)
-  [Function `set_beneficiary_for_operator`](#0x1_delegation_pool_set_beneficiary_for_operator)
-  [Function `update_commission_percentage`](#0x1_delegation_pool_update_commission_percentage)
-  [Function `set_delegated_voter`](#0x1_delegation_pool_set_delegated_voter)
-  [Function `delegate_voting_power`](#0x1_delegation_pool_delegate_voting_power)
-  [Function `enable_delegators_allowlisting`](#0x1_delegation_pool_enable_delegators_allowlisting)
-  [Function `disable_delegators_allowlisting`](#0x1_delegation_pool_disable_delegators_allowlisting)
-  [Function `allowlist_delegator`](#0x1_delegation_pool_allowlist_delegator)
-  [Function `remove_delegator_from_allowlist`](#0x1_delegation_pool_remove_delegator_from_allowlist)
-  [Function `evict_delegator`](#0x1_delegation_pool_evict_delegator)
-  [Function `add_stake`](#0x1_delegation_pool_add_stake)
-  [Function `unlock`](#0x1_delegation_pool_unlock)
-  [Function `unlock_internal`](#0x1_delegation_pool_unlock_internal)
-  [Function `reactivate_stake`](#0x1_delegation_pool_reactivate_stake)
-  [Function `withdraw`](#0x1_delegation_pool_withdraw)
-  [Function `withdraw_internal`](#0x1_delegation_pool_withdraw_internal)
-  [Function `pending_withdrawal_exists`](#0x1_delegation_pool_pending_withdrawal_exists)
-  [Function `pending_inactive_shares_pool_mut`](#0x1_delegation_pool_pending_inactive_shares_pool_mut)
-  [Function `pending_inactive_shares_pool`](#0x1_delegation_pool_pending_inactive_shares_pool)
-  [Function `execute_pending_withdrawal`](#0x1_delegation_pool_execute_pending_withdrawal)
-  [Function `buy_in_active_shares`](#0x1_delegation_pool_buy_in_active_shares)
-  [Function `buy_in_pending_inactive_shares`](#0x1_delegation_pool_buy_in_pending_inactive_shares)
-  [Function `amount_to_shares_to_redeem`](#0x1_delegation_pool_amount_to_shares_to_redeem)
-  [Function `redeem_active_shares`](#0x1_delegation_pool_redeem_active_shares)
-  [Function `redeem_inactive_shares`](#0x1_delegation_pool_redeem_inactive_shares)
-  [Function `calculate_stake_pool_drift`](#0x1_delegation_pool_calculate_stake_pool_drift)
-  [Function `synchronize_delegation_pool`](#0x1_delegation_pool_synchronize_delegation_pool)
-  [Function `assert_and_update_proposal_used_voting_power`](#0x1_delegation_pool_assert_and_update_proposal_used_voting_power)
-  [Function `update_governance_records_for_buy_in_active_shares`](#0x1_delegation_pool_update_governance_records_for_buy_in_active_shares)
-  [Function `update_governance_records_for_buy_in_pending_inactive_shares`](#0x1_delegation_pool_update_governance_records_for_buy_in_pending_inactive_shares)
-  [Function `update_governanace_records_for_redeem_active_shares`](#0x1_delegation_pool_update_governanace_records_for_redeem_active_shares)
-  [Function `update_governanace_records_for_redeem_pending_inactive_shares`](#0x1_delegation_pool_update_governanace_records_for_redeem_pending_inactive_shares)
-  [Function `multiply_then_divide`](#0x1_delegation_pool_multiply_then_divide)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)


```move
module 0x1::delegation_pool {
    use 0x1::account;
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::aptos_governance;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::pool_u64_unbound;
    use 0x1::signer;
    use 0x1::smart_table;
    use 0x1::stake;
    use 0x1::staking_config;
    use 0x1::table;
    use 0x1::table_with_length;
    use 0x1::timestamp;
    use 0x1::vector;
}
```


<a id="0x1_delegation_pool_DelegationPoolOwnership"></a>

## Resource `DelegationPoolOwnership`

Capability that represents ownership over privileged operations on the underlying stake pool.


```move
module 0x1::delegation_pool {
    struct DelegationPoolOwnership has store, key
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>
 equal to address of the resource account owning the stake pool
</dd>
</dl>


<a id="0x1_delegation_pool_ObservedLockupCycle"></a>

## Struct `ObservedLockupCycle`



```move
module 0x1::delegation_pool {
    struct ObservedLockupCycle has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`index: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DelegationPool"></a>

## Resource `DelegationPool`



```move
module 0x1::delegation_pool {
    struct DelegationPool has key
}
```


##### Fields


<dl>
<dt>
`active_shares: pool_u64_unbound::Pool`
</dt>
<dd>

</dd>
<dt>
`observed_lockup_cycle: delegation_pool::ObservedLockupCycle`
</dt>
<dd>

</dd>
<dt>
`inactive_shares: table::Table<delegation_pool::ObservedLockupCycle, pool_u64_unbound::Pool>`
</dt>
<dd>

</dd>
<dt>
`pending_withdrawals: table::Table<address, delegation_pool::ObservedLockupCycle>`
</dt>
<dd>

</dd>
<dt>
`stake_pool_signer_cap: account::SignerCapability`
</dt>
<dd>

</dd>
<dt>
`total_coins_inactive: u64`
</dt>
<dd>

</dd>
<dt>
`operator_commission_percentage: u64`
</dt>
<dd>

</dd>
<dt>
`add_stake_events: event::EventHandle<delegation_pool::AddStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`reactivate_stake_events: event::EventHandle<delegation_pool::ReactivateStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`unlock_stake_events: event::EventHandle<delegation_pool::UnlockStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`withdraw_stake_events: event::EventHandle<delegation_pool::WithdrawStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`distribute_commission_events: event::EventHandle<delegation_pool::DistributeCommissionEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_VotingRecordKey"></a>

## Struct `VotingRecordKey`



```move
module 0x1::delegation_pool {
    struct VotingRecordKey has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_VoteDelegation"></a>

## Struct `VoteDelegation`

Track delegated voter of each delegator.


```move
module 0x1::delegation_pool {
    struct VoteDelegation has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`pending_voter: address`
</dt>
<dd>

</dd>
<dt>
`last_locked_until_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DelegatedVotes"></a>

## Struct `DelegatedVotes`

Track total voting power of each voter.


```move
module 0x1::delegation_pool {
    struct DelegatedVotes has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`active_shares: u128`
</dt>
<dd>

</dd>
<dt>
`pending_inactive_shares: u128`
</dt>
<dd>

</dd>
<dt>
`active_shares_next_lockup: u128`
</dt>
<dd>

</dd>
<dt>
`last_locked_until_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_GovernanceRecords"></a>

## Resource `GovernanceRecords`

Track governance information of a delegation(e.g. voter delegation/voting power calculation).
This struct should be stored in the delegation pool resource account.


```move
module 0x1::delegation_pool {
    struct GovernanceRecords has key
}
```


##### Fields


<dl>
<dt>
`votes: smart_table::SmartTable<delegation_pool::VotingRecordKey, u64>`
</dt>
<dd>

</dd>
<dt>
`votes_per_proposal: smart_table::SmartTable<u64, u64>`
</dt>
<dd>

</dd>
<dt>
`vote_delegation: smart_table::SmartTable<address, delegation_pool::VoteDelegation>`
</dt>
<dd>

</dd>
<dt>
`delegated_votes: smart_table::SmartTable<address, delegation_pool::DelegatedVotes>`
</dt>
<dd>

</dd>
<dt>
`vote_events: event::EventHandle<delegation_pool::VoteEvent>`
</dt>
<dd>

</dd>
<dt>
`create_proposal_events: event::EventHandle<delegation_pool::CreateProposalEvent>`
</dt>
<dd>

</dd>
<dt>
`delegate_voting_power_events: event::EventHandle<delegation_pool::DelegateVotingPowerEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_BeneficiaryForOperator"></a>

## Resource `BeneficiaryForOperator`



```move
module 0x1::delegation_pool {
    struct BeneficiaryForOperator has key
}
```


##### Fields


<dl>
<dt>
`beneficiary_for_operator: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_NextCommissionPercentage"></a>

## Resource `NextCommissionPercentage`



```move
module 0x1::delegation_pool {
    struct NextCommissionPercentage has key
}
```


##### Fields


<dl>
<dt>
`commission_percentage_next_lockup_cycle: u64`
</dt>
<dd>

</dd>
<dt>
`effective_after_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DelegationPoolAllowlisting"></a>

## Resource `DelegationPoolAllowlisting`

Tracks a delegation pool&apos;s allowlist of delegators.
If allowlisting is enabled, existing delegators are not implicitly allowlisted and they can be individually
evicted later by the pool owner.


```move
module 0x1::delegation_pool {
    struct DelegationPoolAllowlisting has key
}
```


##### Fields


<dl>
<dt>
`allowlist: smart_table::SmartTable<address, bool>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_AddStake"></a>

## Struct `AddStake`



```move
module 0x1::delegation_pool {
    #[event]
    struct AddStake has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_added: u64`
</dt>
<dd>

</dd>
<dt>
`add_stake_fee: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_AddStakeEvent"></a>

## Struct `AddStakeEvent`



```move
module 0x1::delegation_pool {
    struct AddStakeEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_added: u64`
</dt>
<dd>

</dd>
<dt>
`add_stake_fee: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_ReactivateStake"></a>

## Struct `ReactivateStake`



```move
module 0x1::delegation_pool {
    #[event]
    struct ReactivateStake has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_reactivated: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_ReactivateStakeEvent"></a>

## Struct `ReactivateStakeEvent`



```move
module 0x1::delegation_pool {
    struct ReactivateStakeEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_reactivated: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_UnlockStake"></a>

## Struct `UnlockStake`



```move
module 0x1::delegation_pool {
    #[event]
    struct UnlockStake has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_unlocked: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_UnlockStakeEvent"></a>

## Struct `UnlockStakeEvent`



```move
module 0x1::delegation_pool {
    struct UnlockStakeEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_unlocked: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_WithdrawStake"></a>

## Struct `WithdrawStake`



```move
module 0x1::delegation_pool {
    #[event]
    struct WithdrawStake has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_withdrawn: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_WithdrawStakeEvent"></a>

## Struct `WithdrawStakeEvent`



```move
module 0x1::delegation_pool {
    struct WithdrawStakeEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
<dt>
`amount_withdrawn: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DistributeCommissionEvent"></a>

## Struct `DistributeCommissionEvent`



```move
module 0x1::delegation_pool {
    #[event]
    struct DistributeCommissionEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`operator: address`
</dt>
<dd>

</dd>
<dt>
`commission_active: u64`
</dt>
<dd>

</dd>
<dt>
`commission_pending_inactive: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DistributeCommission"></a>

## Struct `DistributeCommission`



```move
module 0x1::delegation_pool {
    #[event]
    struct DistributeCommission has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`operator: address`
</dt>
<dd>

</dd>
<dt>
`beneficiary: address`
</dt>
<dd>

</dd>
<dt>
`commission_active: u64`
</dt>
<dd>

</dd>
<dt>
`commission_pending_inactive: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_Vote"></a>

## Struct `Vote`



```move
module 0x1::delegation_pool {
    #[event]
    struct Vote has drop, store
}
```


##### Fields


<dl>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`delegation_pool: address`
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


<a id="0x1_delegation_pool_VoteEvent"></a>

## Struct `VoteEvent`



```move
module 0x1::delegation_pool {
    struct VoteEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`voter: address`
</dt>
<dd>

</dd>
<dt>
`proposal_id: u64`
</dt>
<dd>

</dd>
<dt>
`delegation_pool: address`
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


<a id="0x1_delegation_pool_CreateProposal"></a>

## Struct `CreateProposal`



```move
module 0x1::delegation_pool {
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
`voter: address`
</dt>
<dd>

</dd>
<dt>
`delegation_pool: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_CreateProposalEvent"></a>

## Struct `CreateProposalEvent`



```move
module 0x1::delegation_pool {
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
`voter: address`
</dt>
<dd>

</dd>
<dt>
`delegation_pool: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DelegateVotingPower"></a>

## Struct `DelegateVotingPower`



```move
module 0x1::delegation_pool {
    #[event]
    struct DelegateVotingPower has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator: address`
</dt>
<dd>

</dd>
<dt>
`voter: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DelegateVotingPowerEvent"></a>

## Struct `DelegateVotingPowerEvent`



```move
module 0x1::delegation_pool {
    struct DelegateVotingPowerEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator: address`
</dt>
<dd>

</dd>
<dt>
`voter: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_SetBeneficiaryForOperator"></a>

## Struct `SetBeneficiaryForOperator`



```move
module 0x1::delegation_pool {
    #[event]
    struct SetBeneficiaryForOperator has drop, store
}
```


##### Fields


<dl>
<dt>
`operator: address`
</dt>
<dd>

</dd>
<dt>
`old_beneficiary: address`
</dt>
<dd>

</dd>
<dt>
`new_beneficiary: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_CommissionPercentageChange"></a>

## Struct `CommissionPercentageChange`



```move
module 0x1::delegation_pool {
    #[event]
    struct CommissionPercentageChange has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`owner: address`
</dt>
<dd>

</dd>
<dt>
`commission_percentage_next_lockup_cycle: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_EnableDelegatorsAllowlisting"></a>

## Struct `EnableDelegatorsAllowlisting`



```move
module 0x1::delegation_pool {
    #[event]
    struct EnableDelegatorsAllowlisting has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_DisableDelegatorsAllowlisting"></a>

## Struct `DisableDelegatorsAllowlisting`



```move
module 0x1::delegation_pool {
    #[event]
    struct DisableDelegatorsAllowlisting has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_AllowlistDelegator"></a>

## Struct `AllowlistDelegator`



```move
module 0x1::delegation_pool {
    #[event]
    struct AllowlistDelegator has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_RemoveDelegatorFromAllowlist"></a>

## Struct `RemoveDelegatorFromAllowlist`



```move
module 0x1::delegation_pool {
    #[event]
    struct RemoveDelegatorFromAllowlist has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_delegation_pool_EvictDelegator"></a>

## Struct `EvictDelegator`



```move
module 0x1::delegation_pool {
    #[event]
    struct EvictDelegator has drop, store
}
```


##### Fields


<dl>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`delegator_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_delegation_pool_MAX_U64"></a>



```move
module 0x1::delegation_pool {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x1_delegation_pool_EDEPRECATED_FUNCTION"></a>

Function is deprecated.


```move
module 0x1::delegation_pool {
    const EDEPRECATED_FUNCTION: u64 = 12;
}
```


<a id="0x1_delegation_pool_EDISABLED_FUNCTION"></a>

The function is disabled or hasn&apos;t been enabled.


```move
module 0x1::delegation_pool {
    const EDISABLED_FUNCTION: u64 = 13;
}
```


<a id="0x1_delegation_pool_ENOT_OPERATOR"></a>

The account is not the operator of the stake pool.


```move
module 0x1::delegation_pool {
    const ENOT_OPERATOR: u64 = 18;
}
```


<a id="0x1_delegation_pool_EOWNER_CAP_ALREADY_EXISTS"></a>

Account is already owning a delegation pool.


```move
module 0x1::delegation_pool {
    const EOWNER_CAP_ALREADY_EXISTS: u64 = 2;
}
```


<a id="0x1_delegation_pool_EOWNER_CAP_NOT_FOUND"></a>

Delegation pool owner capability does not exist at the provided account.


```move
module 0x1::delegation_pool {
    const EOWNER_CAP_NOT_FOUND: u64 = 1;
}
```


<a id="0x1_delegation_pool_VALIDATOR_STATUS_INACTIVE"></a>



```move
module 0x1::delegation_pool {
    const VALIDATOR_STATUS_INACTIVE: u64 = 4;
}
```


<a id="0x1_delegation_pool_EINSUFFICIENT_PROPOSER_STAKE"></a>

The voter does not have sufficient stake to create a proposal.


```move
module 0x1::delegation_pool {
    const EINSUFFICIENT_PROPOSER_STAKE: u64 = 15;
}
```


<a id="0x1_delegation_pool_ENO_VOTING_POWER"></a>

The voter does not have any voting power on this proposal.


```move
module 0x1::delegation_pool {
    const ENO_VOTING_POWER: u64 = 16;
}
```


<a id="0x1_delegation_pool_EALREADY_VOTED_BEFORE_ENABLE_PARTIAL_VOTING"></a>

The stake pool has already voted on the proposal before enabling partial governance voting on this delegation pool.


```move
module 0x1::delegation_pool {
    const EALREADY_VOTED_BEFORE_ENABLE_PARTIAL_VOTING: u64 = 17;
}
```


<a id="0x1_delegation_pool_ECANNOT_EVICT_ALLOWLISTED_DELEGATOR"></a>

Cannot evict an allowlisted delegator, should remove them from the allowlist first.


```move
module 0x1::delegation_pool {
    const ECANNOT_EVICT_ALLOWLISTED_DELEGATOR: u64 = 26;
}
```


<a id="0x1_delegation_pool_ECANNOT_UNLOCK_NULL_SHAREHOLDER"></a>

Cannot unlock the accumulated active stake of NULL_SHAREHOLDER(0x0).


```move
module 0x1::delegation_pool {
    const ECANNOT_UNLOCK_NULL_SHAREHOLDER: u64 = 27;
}
```


<a id="0x1_delegation_pool_ECOMMISSION_RATE_CHANGE_NOT_SUPPORTED"></a>

Changing operator commission rate in delegation pool is not supported.


```move
module 0x1::delegation_pool {
    const ECOMMISSION_RATE_CHANGE_NOT_SUPPORTED: u64 = 22;
}
```


<a id="0x1_delegation_pool_EDELEGATION_POOLS_DISABLED"></a>

Creating delegation pools is not enabled yet.


```move
module 0x1::delegation_pool {
    const EDELEGATION_POOLS_DISABLED: u64 = 10;
}
```


<a id="0x1_delegation_pool_EDELEGATION_POOL_DOES_NOT_EXIST"></a>

Delegation pool does not exist at the provided pool address.


```move
module 0x1::delegation_pool {
    const EDELEGATION_POOL_DOES_NOT_EXIST: u64 = 3;
}
```


<a id="0x1_delegation_pool_EDELEGATORS_ALLOWLISTING_NOT_ENABLED"></a>

Delegators allowlisting should be enabled to perform this operation.


```move
module 0x1::delegation_pool {
    const EDELEGATORS_ALLOWLISTING_NOT_ENABLED: u64 = 24;
}
```


<a id="0x1_delegation_pool_EDELEGATORS_ALLOWLISTING_NOT_SUPPORTED"></a>

Delegators allowlisting is not supported.


```move
module 0x1::delegation_pool {
    const EDELEGATORS_ALLOWLISTING_NOT_SUPPORTED: u64 = 23;
}
```


<a id="0x1_delegation_pool_EDELEGATOR_ACTIVE_BALANCE_TOO_LOW"></a>

Delegator&apos;s active balance cannot be less than `MIN_COINS_ON_SHARES_POOL`.


```move
module 0x1::delegation_pool {
    const EDELEGATOR_ACTIVE_BALANCE_TOO_LOW: u64 = 8;
}
```


<a id="0x1_delegation_pool_EDELEGATOR_NOT_ALLOWLISTED"></a>

Cannot add/reactivate stake unless being allowlisted by the pool owner.


```move
module 0x1::delegation_pool {
    const EDELEGATOR_NOT_ALLOWLISTED: u64 = 25;
}
```


<a id="0x1_delegation_pool_EDELEGATOR_PENDING_INACTIVE_BALANCE_TOO_LOW"></a>

Delegator&apos;s pending_inactive balance cannot be less than `MIN_COINS_ON_SHARES_POOL`.


```move
module 0x1::delegation_pool {
    const EDELEGATOR_PENDING_INACTIVE_BALANCE_TOO_LOW: u64 = 9;
}
```


<a id="0x1_delegation_pool_EINVALID_COMMISSION_PERCENTAGE"></a>

Commission percentage has to be between 0 and `MAX_FEE` &#45; 100%.


```move
module 0x1::delegation_pool {
    const EINVALID_COMMISSION_PERCENTAGE: u64 = 5;
}
```


<a id="0x1_delegation_pool_ENOT_ENOUGH_ACTIVE_STAKE_TO_UNLOCK"></a>

There is not enough `active` stake on the stake pool to `unlock`.


```move
module 0x1::delegation_pool {
    const ENOT_ENOUGH_ACTIVE_STAKE_TO_UNLOCK: u64 = 6;
}
```


<a id="0x1_delegation_pool_EOPERATOR_BENEFICIARY_CHANGE_NOT_SUPPORTED"></a>

Changing beneficiaries for operators is not supported.


```move
module 0x1::delegation_pool {
    const EOPERATOR_BENEFICIARY_CHANGE_NOT_SUPPORTED: u64 = 19;
}
```


<a id="0x1_delegation_pool_EPARTIAL_GOVERNANCE_VOTING_NOT_ENABLED"></a>

Partial governance voting hasn&apos;t been enabled on this delegation pool.


```move
module 0x1::delegation_pool {
    const EPARTIAL_GOVERNANCE_VOTING_NOT_ENABLED: u64 = 14;
}
```


<a id="0x1_delegation_pool_EPENDING_WITHDRAWAL_EXISTS"></a>

There is a pending withdrawal to be executed before `unlock`ing any new stake.


```move
module 0x1::delegation_pool {
    const EPENDING_WITHDRAWAL_EXISTS: u64 = 4;
}
```


<a id="0x1_delegation_pool_ESLASHED_INACTIVE_STAKE_ON_PAST_OLC"></a>

Slashing (if implemented) should not be applied to already `inactive` stake.
Not only it invalidates the accounting of past observed lockup cycles (OLC),
but is also unfair to delegators whose stake has been inactive before validator started misbehaving.
Additionally, the inactive stake does not count on the voting power of validator.


```move
module 0x1::delegation_pool {
    const ESLASHED_INACTIVE_STAKE_ON_PAST_OLC: u64 = 7;
}
```


<a id="0x1_delegation_pool_ETOO_LARGE_COMMISSION_INCREASE"></a>

Commission percentage increase is too large.


```move
module 0x1::delegation_pool {
    const ETOO_LARGE_COMMISSION_INCREASE: u64 = 20;
}
```


<a id="0x1_delegation_pool_ETOO_LATE_COMMISSION_CHANGE"></a>

Commission percentage change is too late in this lockup period, and should be done at least a quarter (1/4) of the lockup duration before the lockup cycle ends.


```move
module 0x1::delegation_pool {
    const ETOO_LATE_COMMISSION_CHANGE: u64 = 21;
}
```


<a id="0x1_delegation_pool_EWITHDRAW_ZERO_STAKE"></a>

Cannot request to withdraw zero stake.


```move
module 0x1::delegation_pool {
    const EWITHDRAW_ZERO_STAKE: u64 = 11;
}
```


<a id="0x1_delegation_pool_MAX_COMMISSION_INCREASE"></a>

Maximum commission percentage increase per lockup cycle. 10% is represented as 1000.


```move
module 0x1::delegation_pool {
    const MAX_COMMISSION_INCREASE: u64 = 1000;
}
```


<a id="0x1_delegation_pool_MAX_FEE"></a>

Maximum operator percentage fee(of double digit precision): 22.85% is represented as 2285


```move
module 0x1::delegation_pool {
    const MAX_FEE: u64 = 10000;
}
```


<a id="0x1_delegation_pool_MIN_COINS_ON_SHARES_POOL"></a>

Minimum coins to exist on a shares pool at all times.
Enforced per delegator for both active and pending_inactive pools.
This constraint ensures the share price cannot overly increase and lead to
substantial losses when buying shares (can lose at most 1 share which may
be worth a lot if current share price is high).
This constraint is not enforced on inactive pools as they only allow redeems
(can lose at most 1 coin regardless of current share price).


```move
module 0x1::delegation_pool {
    const MIN_COINS_ON_SHARES_POOL: u64 = 1000000000;
}
```


<a id="0x1_delegation_pool_MODULE_SALT"></a>



```move
module 0x1::delegation_pool {
    const MODULE_SALT: vector<u8> = [97, 112, 116, 111, 115, 95, 102, 114, 97, 109, 101, 119, 111, 114, 107, 58, 58, 100, 101, 108, 101, 103, 97, 116, 105, 111, 110, 95, 112, 111, 111, 108];
}
```


<a id="0x1_delegation_pool_NULL_SHAREHOLDER"></a>

Special shareholder temporarily owning the `add_stake` fees charged during this epoch.
On each `add_stake` operation any resulted fee is used to buy active shares for this shareholder.
First synchronization after this epoch ends will distribute accumulated fees to the rest of the pool as refunds.


```move
module 0x1::delegation_pool {
    const NULL_SHAREHOLDER: address = 0x0;
}
```


<a id="0x1_delegation_pool_SHARES_SCALING_FACTOR"></a>

Scaling factor of shares pools used within the delegation pool


```move
module 0x1::delegation_pool {
    const SHARES_SCALING_FACTOR: u64 = 10000000000000000;
}
```


<a id="0x1_delegation_pool_owner_cap_exists"></a>

## Function `owner_cap_exists`

Return whether supplied address `addr` is owner of a delegation pool.


```move
module 0x1::delegation_pool {
    #[view]
    public fun owner_cap_exists(addr: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun owner_cap_exists(addr: address): bool {
        exists<DelegationPoolOwnership>(addr)
    }
}
```


<a id="0x1_delegation_pool_get_owned_pool_address"></a>

## Function `get_owned_pool_address`

Return address of the delegation pool owned by `owner` or fail if there is none.


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_owned_pool_address(owner: address): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_owned_pool_address(owner: address): address acquires DelegationPoolOwnership {
        assert_owner_cap_exists(owner);
        borrow_global<DelegationPoolOwnership>(owner).pool_address
    }
}
```


<a id="0x1_delegation_pool_delegation_pool_exists"></a>

## Function `delegation_pool_exists`

Return whether a delegation pool exists at supplied address `addr`.


```move
module 0x1::delegation_pool {
    #[view]
    public fun delegation_pool_exists(addr: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun delegation_pool_exists(addr: address): bool {
        exists<DelegationPool>(addr)
    }
}
```


<a id="0x1_delegation_pool_partial_governance_voting_enabled"></a>

## Function `partial_governance_voting_enabled`

Return whether a delegation pool has already enabled partial governance voting.


```move
module 0x1::delegation_pool {
    #[view]
    public fun partial_governance_voting_enabled(pool_address: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun partial_governance_voting_enabled(pool_address: address): bool {
        exists<GovernanceRecords>(pool_address) && stake::get_delegated_voter(pool_address) == pool_address
    }
}
```


<a id="0x1_delegation_pool_observed_lockup_cycle"></a>

## Function `observed_lockup_cycle`

Return the index of current observed lockup cycle on delegation pool `pool_address`.


```move
module 0x1::delegation_pool {
    #[view]
    public fun observed_lockup_cycle(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun observed_lockup_cycle(pool_address: address): u64 acquires DelegationPool {
        assert_delegation_pool_exists(pool_address);
        borrow_global<DelegationPool>(pool_address).observed_lockup_cycle.index
    }
}
```


<a id="0x1_delegation_pool_is_next_commission_percentage_effective"></a>

## Function `is_next_commission_percentage_effective`

Return whether the commission percentage for the next lockup cycle is effective.


```move
module 0x1::delegation_pool {
    #[view]
    public fun is_next_commission_percentage_effective(pool_address: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun is_next_commission_percentage_effective(pool_address: address): bool acquires NextCommissionPercentage {
        exists<NextCommissionPercentage>(pool_address) &&
            timestamp::now_seconds() >= borrow_global<NextCommissionPercentage>(pool_address).effective_after_secs
    }
}
```


<a id="0x1_delegation_pool_operator_commission_percentage"></a>

## Function `operator_commission_percentage`

Return the operator commission percentage set on the delegation pool `pool_address`.


```move
module 0x1::delegation_pool {
    #[view]
    public fun operator_commission_percentage(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun operator_commission_percentage(
        pool_address: address
    ): u64 acquires DelegationPool, NextCommissionPercentage {
        assert_delegation_pool_exists(pool_address);
        if (is_next_commission_percentage_effective(pool_address)) {
            operator_commission_percentage_next_lockup_cycle(pool_address)
        } else {
            borrow_global<DelegationPool>(pool_address).operator_commission_percentage
        }
    }
}
```


<a id="0x1_delegation_pool_operator_commission_percentage_next_lockup_cycle"></a>

## Function `operator_commission_percentage_next_lockup_cycle`

Return the operator commission percentage for the next lockup cycle.


```move
module 0x1::delegation_pool {
    #[view]
    public fun operator_commission_percentage_next_lockup_cycle(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun operator_commission_percentage_next_lockup_cycle(
        pool_address: address
    ): u64 acquires DelegationPool, NextCommissionPercentage {
        assert_delegation_pool_exists(pool_address);
        if (exists<NextCommissionPercentage>(pool_address)) {
            borrow_global<NextCommissionPercentage>(pool_address).commission_percentage_next_lockup_cycle
        } else {
            borrow_global<DelegationPool>(pool_address).operator_commission_percentage
        }
    }
}
```


<a id="0x1_delegation_pool_shareholders_count_active_pool"></a>

## Function `shareholders_count_active_pool`

Return the number of delegators owning active stake within `pool_address`.


```move
module 0x1::delegation_pool {
    #[view]
    public fun shareholders_count_active_pool(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun shareholders_count_active_pool(pool_address: address): u64 acquires DelegationPool {
        assert_delegation_pool_exists(pool_address);
        pool_u64::shareholders_count(&borrow_global<DelegationPool>(pool_address).active_shares)
    }
}
```


<a id="0x1_delegation_pool_get_delegation_pool_stake"></a>

## Function `get_delegation_pool_stake`

Return the stake amounts on `pool_address` in the different states:
(`active`,`inactive`,`pending_active`,`pending_inactive`)


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_delegation_pool_stake(pool_address: address): (u64, u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_delegation_pool_stake(pool_address: address): (u64, u64, u64, u64) {
        assert_delegation_pool_exists(pool_address);
        stake::get_stake(pool_address)
    }
}
```


<a id="0x1_delegation_pool_get_pending_withdrawal"></a>

## Function `get_pending_withdrawal`

Return whether the given delegator has any withdrawable stake. If they recently requested to unlock
some stake and the stake pool&apos;s lockup cycle has not ended, their coins are not withdrawable yet.


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_pending_withdrawal(pool_address: address, delegator_address: address): (bool, u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_pending_withdrawal(
        pool_address: address,
        delegator_address: address
    ): (bool, u64) acquires DelegationPool {
        assert_delegation_pool_exists(pool_address);
        let pool = borrow_global<DelegationPool>(pool_address);
        let (
            lockup_cycle_ended,
            _,
            pending_inactive,
            _,
            commission_pending_inactive
        ) = calculate_stake_pool_drift(pool);

        let (withdrawal_exists, withdrawal_olc) = pending_withdrawal_exists(pool, delegator_address);
        if (!withdrawal_exists) {
            // if no pending withdrawal, there is neither inactive nor pending_inactive stake
            (false, 0)
        } else {
            // delegator has either inactive or pending_inactive stake due to automatic withdrawals
            let inactive_shares = table::borrow(&pool.inactive_shares, withdrawal_olc);
            if (withdrawal_olc.index < pool.observed_lockup_cycle.index) {
                // if withdrawal's lockup cycle ended on delegation pool then it is inactive
                (true, pool_u64::balance(inactive_shares, delegator_address))
            } else {
                pending_inactive = pool_u64::shares_to_amount_with_total_coins(
                    inactive_shares,
                    pool_u64::shares(inactive_shares, delegator_address),
                    // exclude operator pending_inactive rewards not converted to shares yet
                    pending_inactive - commission_pending_inactive
                );
                // if withdrawal's lockup cycle ended ONLY on stake pool then it is also inactive
                (lockup_cycle_ended, pending_inactive)
            }
        }
    }
}
```


<a id="0x1_delegation_pool_get_stake"></a>

## Function `get_stake`

Return total stake owned by `delegator_address` within delegation pool `pool_address`
in each of its individual states: (`active`,`inactive`,`pending_inactive`)


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_stake(pool_address: address, delegator_address: address): (u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_stake(
        pool_address: address,
        delegator_address: address
    ): (u64, u64, u64) acquires DelegationPool, BeneficiaryForOperator {
        assert_delegation_pool_exists(pool_address);
        let pool = borrow_global<DelegationPool>(pool_address);
        let (
            lockup_cycle_ended,
            active,
            _,
            commission_active,
            commission_pending_inactive
        ) = calculate_stake_pool_drift(pool);

        let total_active_shares = pool_u64::total_shares(&pool.active_shares);
        let delegator_active_shares = pool_u64::shares(&pool.active_shares, delegator_address);

        let (_, _, pending_active, _) = stake::get_stake(pool_address);
        if (pending_active == 0) {
            // zero `pending_active` stake indicates that either there are no `add_stake` fees or
            // previous epoch has ended and should identify shares owning these fees as released
            total_active_shares = total_active_shares - pool_u64::shares(&pool.active_shares, NULL_SHAREHOLDER);
            if (delegator_address == NULL_SHAREHOLDER) {
                delegator_active_shares = 0
            }
        };
        active = pool_u64::shares_to_amount_with_total_stats(
            &pool.active_shares,
            delegator_active_shares,
            // exclude operator active rewards not converted to shares yet
            active - commission_active,
            total_active_shares
        );

        // get state and stake (0 if there is none) of the pending withdrawal
        let (withdrawal_inactive, withdrawal_stake) = get_pending_withdrawal(pool_address, delegator_address);
        // report non-active stakes accordingly to the state of the pending withdrawal
        let (inactive, pending_inactive) = if (withdrawal_inactive) (withdrawal_stake, 0) else (0, withdrawal_stake);

        // should also include commission rewards in case of the operator account
        // operator rewards are actually used to buy shares which is introducing
        // some imprecision (received stake would be slightly less)
        // but adding rewards onto the existing stake is still a good approximation
        if (delegator_address == beneficiary_for_operator(get_operator(pool_address))) {
            active = active + commission_active;
            // in-flight pending_inactive commission can coexist with already inactive withdrawal
            if (lockup_cycle_ended) {
                inactive = inactive + commission_pending_inactive
            } else {
                pending_inactive = pending_inactive + commission_pending_inactive
            }
        };

        (active, inactive, pending_inactive)
    }
}
```


<a id="0x1_delegation_pool_get_add_stake_fee"></a>

## Function `get_add_stake_fee`

Return refundable stake to be extracted from added `amount` at `add_stake` operation on pool `pool_address`.
If the validator produces rewards this epoch, added stake goes directly to `pending_active` and
does not earn rewards. However, all shares within a pool appreciate uniformly and when this epoch ends:
&#45; either added shares are still `pending_active` and steal from rewards of existing `active` stake
&#45; or have moved to `pending_inactive` and get full rewards (they displaced `active` stake at `unlock`)
To mitigate this, some of the added stake is extracted and fed back into the pool as placeholder
for the rewards the remaining stake would have earned if active:
extracted&#45;fee &#61; (amount &#45; extracted&#45;fee) &#42; reward&#45;rate% &#42; (100% &#45; operator&#45;commission%)


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_add_stake_fee(pool_address: address, amount: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_add_stake_fee(
        pool_address: address,
        amount: u64
    ): u64 acquires DelegationPool, NextCommissionPercentage {
        if (stake::is_current_epoch_validator(pool_address)) {
            let (rewards_rate, rewards_rate_denominator) = staking_config::get_reward_rate(&staking_config::get());
            if (rewards_rate_denominator > 0) {
                assert_delegation_pool_exists(pool_address);

                rewards_rate = rewards_rate * (MAX_FEE - operator_commission_percentage(pool_address));
                rewards_rate_denominator = rewards_rate_denominator * MAX_FEE;
                ((((amount as u128) * (rewards_rate as u128)) / ((rewards_rate as u128) + (rewards_rate_denominator as u128))) as u64)
            } else { 0 }
        } else { 0 }
    }
}
```


<a id="0x1_delegation_pool_can_withdraw_pending_inactive"></a>

## Function `can_withdraw_pending_inactive`

Return whether `pending_inactive` stake can be directly withdrawn from
the delegation pool, implicitly its stake pool, in the special case
the validator had gone inactive before its lockup expired.


```move
module 0x1::delegation_pool {
    #[view]
    public fun can_withdraw_pending_inactive(pool_address: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun can_withdraw_pending_inactive(pool_address: address): bool {
        stake::get_validator_state(pool_address) == VALIDATOR_STATUS_INACTIVE &&
            timestamp::now_seconds() >= stake::get_lockup_secs(pool_address)
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_voter_total_voting_power"></a>

## Function `calculate_and_update_voter_total_voting_power`

Return the total voting power of a delegator in a delegation pool. This function syncs DelegationPool to the
latest state.


```move
module 0x1::delegation_pool {
    #[view]
    public fun calculate_and_update_voter_total_voting_power(pool_address: address, voter: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun calculate_and_update_voter_total_voting_power(
        pool_address: address,
        voter: address
    ): u64 acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_partial_governance_voting_enabled(pool_address);
        // Delegation pool need to be synced to explain rewards(which could change the coin amount) and
        // commission(which could cause share transfer).
        synchronize_delegation_pool(pool_address);
        let pool = borrow_global<DelegationPool>(pool_address);
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let latest_delegated_votes = update_and_borrow_mut_delegated_votes(pool, governance_records, voter);
        calculate_total_voting_power(pool, latest_delegated_votes)
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_remaining_voting_power"></a>

## Function `calculate_and_update_remaining_voting_power`

Return the remaining voting power of a delegator in a delegation pool on a proposal. This function syncs DelegationPool to the
latest state.


```move
module 0x1::delegation_pool {
    #[view]
    public fun calculate_and_update_remaining_voting_power(pool_address: address, voter_address: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun calculate_and_update_remaining_voting_power(
        pool_address: address,
        voter_address: address,
        proposal_id: u64
    ): u64 acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_partial_governance_voting_enabled(pool_address);
        // If the whole stake pool has no voting power(e.g. it has already voted before partial
        // governance voting flag is enabled), the delegator also has no voting power.
        if (aptos_governance::get_remaining_voting_power(pool_address, proposal_id) == 0) {
            return 0
        };

        let total_voting_power = calculate_and_update_voter_total_voting_power(pool_address, voter_address);
        let governance_records = borrow_global<GovernanceRecords>(pool_address);
        total_voting_power - get_used_voting_power(governance_records, voter_address, proposal_id)
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_delegator_voter"></a>

## Function `calculate_and_update_delegator_voter`

Return the latest delegated voter of a delegator in a delegation pool. This function syncs DelegationPool to the
latest state.


```move
module 0x1::delegation_pool {
    #[view]
    public fun calculate_and_update_delegator_voter(pool_address: address, delegator_address: address): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun calculate_and_update_delegator_voter(
        pool_address: address,
        delegator_address: address
    ): address acquires DelegationPool, GovernanceRecords {
        assert_partial_governance_voting_enabled(pool_address);
        calculate_and_update_delegator_voter_internal(
            borrow_global<DelegationPool>(pool_address),
            borrow_global_mut<GovernanceRecords>(pool_address),
            delegator_address
        )
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_voting_delegation"></a>

## Function `calculate_and_update_voting_delegation`

Return the current state of a voting delegation of a delegator in a delegation pool.


```move
module 0x1::delegation_pool {
    #[view]
    public fun calculate_and_update_voting_delegation(pool_address: address, delegator_address: address): (address, address, u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun calculate_and_update_voting_delegation(
        pool_address: address,
        delegator_address: address
    ): (address, address, u64) acquires DelegationPool, GovernanceRecords {
        assert_partial_governance_voting_enabled(pool_address);
        let vote_delegation = update_and_borrow_mut_delegator_vote_delegation(
            borrow_global<DelegationPool>(pool_address),
            borrow_global_mut<GovernanceRecords>(pool_address),
            delegator_address
        );

        (vote_delegation.voter, vote_delegation.pending_voter, vote_delegation.last_locked_until_secs)
    }
}
```


<a id="0x1_delegation_pool_get_expected_stake_pool_address"></a>

## Function `get_expected_stake_pool_address`

Return the address of the stake pool to be created with the provided owner, and seed.


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_expected_stake_pool_address(owner: address, delegation_pool_creation_seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_expected_stake_pool_address(owner: address, delegation_pool_creation_seed: vector<u8>
    ): address {
        let seed = create_resource_account_seed(delegation_pool_creation_seed);
        account::create_resource_address(&owner, seed)
    }
}
```


<a id="0x1_delegation_pool_min_remaining_secs_for_commission_change"></a>

## Function `min_remaining_secs_for_commission_change`

Return the minimum remaining time in seconds for commission change, which is one fourth of the lockup duration.


```move
module 0x1::delegation_pool {
    #[view]
    public fun min_remaining_secs_for_commission_change(): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun min_remaining_secs_for_commission_change(): u64 {
        let config = staking_config::get();
        staking_config::get_recurring_lockup_duration(&config) / 4
    }
}
```


<a id="0x1_delegation_pool_allowlisting_enabled"></a>

## Function `allowlisting_enabled`

Return whether allowlisting is enabled for the provided delegation pool.


```move
module 0x1::delegation_pool {
    #[view]
    public fun allowlisting_enabled(pool_address: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun allowlisting_enabled(pool_address: address): bool {
        assert_delegation_pool_exists(pool_address);
        exists<DelegationPoolAllowlisting>(pool_address)
    }
}
```


<a id="0x1_delegation_pool_delegator_allowlisted"></a>

## Function `delegator_allowlisted`

Return whether the provided delegator is allowlisted.
A delegator is allowlisted if:
&#45; allowlisting is disabled on the pool
&#45; delegator is part of the allowlist


```move
module 0x1::delegation_pool {
    #[view]
    public fun delegator_allowlisted(pool_address: address, delegator_address: address): bool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun delegator_allowlisted(
        pool_address: address,
        delegator_address: address,
    ): bool acquires DelegationPoolAllowlisting {
        if (!allowlisting_enabled(pool_address)) { return true };
        smart_table::contains(freeze(borrow_mut_delegators_allowlist(pool_address)), delegator_address)
    }
}
```


<a id="0x1_delegation_pool_get_delegators_allowlist"></a>

## Function `get_delegators_allowlist`

Return allowlist or revert if allowlisting is not enabled for the provided delegation pool.


```move
module 0x1::delegation_pool {
    #[view]
    public fun get_delegators_allowlist(pool_address: address): vector<address>
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun get_delegators_allowlist(
        pool_address: address,
    ): vector<address> acquires DelegationPoolAllowlisting {
        assert_allowlisting_enabled(pool_address);

        let allowlist = vector[];
        smart_table::for_each_ref(freeze(borrow_mut_delegators_allowlist(pool_address)), |delegator, _v| {
            vector::push_back(&mut allowlist, *delegator);
        });
        allowlist
    }
}
```


<a id="0x1_delegation_pool_initialize_delegation_pool"></a>

## Function `initialize_delegation_pool`

Initialize a delegation pool of custom fixed `operator_commission_percentage`.
A resource account is created from `owner` signer and its supplied `delegation_pool_creation_seed`
to host the delegation pool resource and own the underlying stake pool.
Ownership over setting the operator/voter is granted to `owner` who has both roles initially.


```move
module 0x1::delegation_pool {
    public entry fun initialize_delegation_pool(owner: &signer, operator_commission_percentage: u64, delegation_pool_creation_seed: vector<u8>)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun initialize_delegation_pool(
        owner: &signer,
        operator_commission_percentage: u64,
        delegation_pool_creation_seed: vector<u8>,
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert!(features::delegation_pools_enabled(), error::invalid_state(EDELEGATION_POOLS_DISABLED));
        let owner_address = signer::address_of(owner);
        assert!(!owner_cap_exists(owner_address), error::already_exists(EOWNER_CAP_ALREADY_EXISTS));
        assert!(operator_commission_percentage <= MAX_FEE, error::invalid_argument(EINVALID_COMMISSION_PERCENTAGE));

        // generate a seed to be used to create the resource account hosting the delegation pool
        let seed = create_resource_account_seed(delegation_pool_creation_seed);

        let (stake_pool_signer, stake_pool_signer_cap) = account::create_resource_account(owner, seed);
        coin::register<AptosCoin>(&stake_pool_signer);

        // stake_pool_signer will be owner of the stake pool and have its `stake::OwnerCapability`
        let pool_address = signer::address_of(&stake_pool_signer);
        stake::initialize_stake_owner(&stake_pool_signer, 0, owner_address, owner_address);

        let inactive_shares = table::new<ObservedLockupCycle, pool_u64::Pool>();
        table::add(
            &mut inactive_shares,
            olc_with_index(0),
            pool_u64::create_with_scaling_factor(SHARES_SCALING_FACTOR)
        );

        move_to(&stake_pool_signer, DelegationPool {
            active_shares: pool_u64::create_with_scaling_factor(SHARES_SCALING_FACTOR),
            observed_lockup_cycle: olc_with_index(0),
            inactive_shares,
            pending_withdrawals: table::new<address, ObservedLockupCycle>(),
            stake_pool_signer_cap,
            total_coins_inactive: 0,
            operator_commission_percentage,
            add_stake_events: account::new_event_handle<AddStakeEvent>(&stake_pool_signer),
            reactivate_stake_events: account::new_event_handle<ReactivateStakeEvent>(&stake_pool_signer),
            unlock_stake_events: account::new_event_handle<UnlockStakeEvent>(&stake_pool_signer),
            withdraw_stake_events: account::new_event_handle<WithdrawStakeEvent>(&stake_pool_signer),
            distribute_commission_events: account::new_event_handle<DistributeCommissionEvent>(&stake_pool_signer),
        });

        // save delegation pool ownership and resource account address (inner stake pool address) on `owner`
        move_to(owner, DelegationPoolOwnership { pool_address });

        // All delegation pool enable partial governance voting by default once the feature flag is enabled.
        if (features::partial_governance_voting_enabled(
        ) && features::delegation_pool_partial_governance_voting_enabled()) {
            enable_partial_governance_voting(pool_address);
        }
    }
}
```


<a id="0x1_delegation_pool_beneficiary_for_operator"></a>

## Function `beneficiary_for_operator`

Return the beneficiary address of the operator.


```move
module 0x1::delegation_pool {
    #[view]
    public fun beneficiary_for_operator(operator: address): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun beneficiary_for_operator(operator: address): address acquires BeneficiaryForOperator {
        if (exists<BeneficiaryForOperator>(operator)) {
            return borrow_global<BeneficiaryForOperator>(operator).beneficiary_for_operator
        } else {
            operator
        }
    }
}
```


<a id="0x1_delegation_pool_enable_partial_governance_voting"></a>

## Function `enable_partial_governance_voting`

Enable partial governance voting on a stake pool. The voter of this stake pool will be managed by this module.
The existing voter will be replaced. The function is permissionless.


```move
module 0x1::delegation_pool {
    public entry fun enable_partial_governance_voting(pool_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun enable_partial_governance_voting(
        pool_address: address,
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert!(features::partial_governance_voting_enabled(), error::invalid_state(EDISABLED_FUNCTION));
        assert!(
            features::delegation_pool_partial_governance_voting_enabled(),
            error::invalid_state(EDISABLED_FUNCTION)
        );
        assert_delegation_pool_exists(pool_address);
        // synchronize delegation and stake pools before any user operation.
        synchronize_delegation_pool(pool_address);

        let delegation_pool = borrow_global<DelegationPool>(pool_address);
        let stake_pool_signer = retrieve_stake_pool_owner(delegation_pool);
        // delegated_voter is managed by the stake pool itself, which signer capability is managed by DelegationPool.
        // So voting power of this stake pool can only be used through this module.
        stake::set_delegated_voter(&stake_pool_signer, signer::address_of(&stake_pool_signer));

        move_to(&stake_pool_signer, GovernanceRecords {
            votes: smart_table::new(),
            votes_per_proposal: smart_table::new(),
            vote_delegation: smart_table::new(),
            delegated_votes: smart_table::new(),
            vote_events: account::new_event_handle<VoteEvent>(&stake_pool_signer),
            create_proposal_events: account::new_event_handle<CreateProposalEvent>(&stake_pool_signer),
            delegate_voting_power_events: account::new_event_handle<DelegateVotingPowerEvent>(&stake_pool_signer),
        });
    }
}
```


<a id="0x1_delegation_pool_vote"></a>

## Function `vote`

Vote on a proposal with a voter&apos;s voting power. To successfully vote, the following conditions must be met:
1. The voting period of the proposal hasn&apos;t ended.
2. The delegation pool&apos;s lockup period ends after the voting period of the proposal.
3. The voter still has spare voting power on this proposal.
4. The delegation pool never votes on the proposal before enabling partial governance voting.


```move
module 0x1::delegation_pool {
    public entry fun vote(voter: &signer, pool_address: address, proposal_id: u64, voting_power: u64, should_pass: bool)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun vote(
        voter: &signer,
        pool_address: address,
        proposal_id: u64,
        voting_power: u64,
        should_pass: bool
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_partial_governance_voting_enabled(pool_address);
        // synchronize delegation and stake pools before any user operation.
        synchronize_delegation_pool(pool_address);

        let voter_address = signer::address_of(voter);
        let remaining_voting_power = calculate_and_update_remaining_voting_power(
            pool_address,
            voter_address,
            proposal_id
        );
        if (voting_power > remaining_voting_power) {
            voting_power = remaining_voting_power;
        };
        assert!(voting_power > 0, error::invalid_argument(ENO_VOTING_POWER));

        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        // Check a edge case during the transient period of enabling partial governance voting.
        assert_and_update_proposal_used_voting_power(governance_records, pool_address, proposal_id, voting_power);
        let used_voting_power = borrow_mut_used_voting_power(governance_records, voter_address, proposal_id);
        *used_voting_power = *used_voting_power + voting_power;

        let pool_signer = retrieve_stake_pool_owner(borrow_global<DelegationPool>(pool_address));
        aptos_governance::partial_vote(&pool_signer, pool_address, proposal_id, voting_power, should_pass);

        if (features::module_event_migration_enabled()) {
            event::emit(
                Vote {
                    voter: voter_address,
                    proposal_id,
                    delegation_pool: pool_address,
                    num_votes: voting_power,
                    should_pass,
                }
            );
        };

        event::emit_event(
            &mut governance_records.vote_events,
            VoteEvent {
                voter: voter_address,
                proposal_id,
                delegation_pool: pool_address,
                num_votes: voting_power,
                should_pass,
            }
        );
    }
}
```


<a id="0x1_delegation_pool_create_proposal"></a>

## Function `create_proposal`

A voter could create a governance proposal by this function. To successfully create a proposal, the voter&apos;s
voting power in THIS delegation pool must be not less than the minimum required voting power specified in
`aptos_governance.move`.


```move
module 0x1::delegation_pool {
    public entry fun create_proposal(voter: &signer, pool_address: address, execution_hash: vector<u8>, metadata_location: vector<u8>, metadata_hash: vector<u8>, is_multi_step_proposal: bool)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun create_proposal(
        voter: &signer,
        pool_address: address,
        execution_hash: vector<u8>,
        metadata_location: vector<u8>,
        metadata_hash: vector<u8>,
        is_multi_step_proposal: bool,
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_partial_governance_voting_enabled(pool_address);

        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);

        let voter_addr = signer::address_of(voter);
        let pool = borrow_global<DelegationPool>(pool_address);
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let total_voting_power = calculate_and_update_delegated_votes(pool, governance_records, voter_addr);
        assert!(
            total_voting_power >= aptos_governance::get_required_proposer_stake(),
            error::invalid_argument(EINSUFFICIENT_PROPOSER_STAKE));
        let pool_signer = retrieve_stake_pool_owner(borrow_global<DelegationPool>(pool_address));
        let proposal_id = aptos_governance::create_proposal_v2_impl(
            &pool_signer,
            pool_address,
            execution_hash,
            metadata_location,
            metadata_hash,
            is_multi_step_proposal,
        );

        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);

        if (features::module_event_migration_enabled()) {
            event::emit(
                CreateProposal {
                    proposal_id,
                    voter: voter_addr,
                    delegation_pool: pool_address,
                }
            );
        };

        event::emit_event(
            &mut governance_records.create_proposal_events,
            CreateProposalEvent {
                proposal_id,
                voter: voter_addr,
                delegation_pool: pool_address,
            }
        );
    }
}
```


<a id="0x1_delegation_pool_assert_owner_cap_exists"></a>

## Function `assert_owner_cap_exists`



```move
module 0x1::delegation_pool {
    fun assert_owner_cap_exists(owner: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_owner_cap_exists(owner: address) {
        assert!(owner_cap_exists(owner), error::not_found(EOWNER_CAP_NOT_FOUND));
    }
}
```


<a id="0x1_delegation_pool_assert_delegation_pool_exists"></a>

## Function `assert_delegation_pool_exists`



```move
module 0x1::delegation_pool {
    fun assert_delegation_pool_exists(pool_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_delegation_pool_exists(pool_address: address) {
        assert!(delegation_pool_exists(pool_address), error::invalid_argument(EDELEGATION_POOL_DOES_NOT_EXIST));
    }
}
```


<a id="0x1_delegation_pool_assert_min_active_balance"></a>

## Function `assert_min_active_balance`



```move
module 0x1::delegation_pool {
    fun assert_min_active_balance(pool: &delegation_pool::DelegationPool, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_min_active_balance(pool: &DelegationPool, delegator_address: address) {
        let balance = pool_u64::balance(&pool.active_shares, delegator_address);
        assert!(balance >= MIN_COINS_ON_SHARES_POOL, error::invalid_argument(EDELEGATOR_ACTIVE_BALANCE_TOO_LOW));
    }
}
```


<a id="0x1_delegation_pool_assert_min_pending_inactive_balance"></a>

## Function `assert_min_pending_inactive_balance`



```move
module 0x1::delegation_pool {
    fun assert_min_pending_inactive_balance(pool: &delegation_pool::DelegationPool, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_min_pending_inactive_balance(pool: &DelegationPool, delegator_address: address) {
        let balance = pool_u64::balance(pending_inactive_shares_pool(pool), delegator_address);
        assert!(
            balance >= MIN_COINS_ON_SHARES_POOL,
            error::invalid_argument(EDELEGATOR_PENDING_INACTIVE_BALANCE_TOO_LOW)
        );
    }
}
```


<a id="0x1_delegation_pool_assert_partial_governance_voting_enabled"></a>

## Function `assert_partial_governance_voting_enabled`



```move
module 0x1::delegation_pool {
    fun assert_partial_governance_voting_enabled(pool_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_partial_governance_voting_enabled(pool_address: address) {
        assert_delegation_pool_exists(pool_address);
        assert!(
            partial_governance_voting_enabled(pool_address),
            error::invalid_state(EPARTIAL_GOVERNANCE_VOTING_NOT_ENABLED)
        );
    }
}
```


<a id="0x1_delegation_pool_assert_allowlisting_enabled"></a>

## Function `assert_allowlisting_enabled`



```move
module 0x1::delegation_pool {
    fun assert_allowlisting_enabled(pool_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_allowlisting_enabled(pool_address: address) {
        assert!(allowlisting_enabled(pool_address), error::invalid_state(EDELEGATORS_ALLOWLISTING_NOT_ENABLED));
    }
}
```


<a id="0x1_delegation_pool_assert_delegator_allowlisted"></a>

## Function `assert_delegator_allowlisted`



```move
module 0x1::delegation_pool {
    fun assert_delegator_allowlisted(pool_address: address, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun assert_delegator_allowlisted(
        pool_address: address,
        delegator_address: address,
    ) acquires DelegationPoolAllowlisting {
        assert!(
            delegator_allowlisted(pool_address, delegator_address),
            error::permission_denied(EDELEGATOR_NOT_ALLOWLISTED)
        );
    }
}
```


<a id="0x1_delegation_pool_coins_to_redeem_to_ensure_min_stake"></a>

## Function `coins_to_redeem_to_ensure_min_stake`



```move
module 0x1::delegation_pool {
    fun coins_to_redeem_to_ensure_min_stake(src_shares_pool: &pool_u64_unbound::Pool, shareholder: address, amount: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun coins_to_redeem_to_ensure_min_stake(
        src_shares_pool: &pool_u64::Pool,
        shareholder: address,
        amount: u64,
    ): u64 {
        // find how many coins would be redeemed if supplying `amount`
        let redeemed_coins = pool_u64::shares_to_amount(
            src_shares_pool,
            amount_to_shares_to_redeem(src_shares_pool, shareholder, amount)
        );
        // if balance drops under threshold then redeem it entirely
        let src_balance = pool_u64::balance(src_shares_pool, shareholder);
        if (src_balance - redeemed_coins < MIN_COINS_ON_SHARES_POOL) {
            amount = src_balance;
        };
        amount
    }
}
```


<a id="0x1_delegation_pool_coins_to_transfer_to_ensure_min_stake"></a>

## Function `coins_to_transfer_to_ensure_min_stake`



```move
module 0x1::delegation_pool {
    fun coins_to_transfer_to_ensure_min_stake(src_shares_pool: &pool_u64_unbound::Pool, dst_shares_pool: &pool_u64_unbound::Pool, shareholder: address, amount: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun coins_to_transfer_to_ensure_min_stake(
        src_shares_pool: &pool_u64::Pool,
        dst_shares_pool: &pool_u64::Pool,
        shareholder: address,
        amount: u64,
    ): u64 {
        // find how many coins would be redeemed from source if supplying `amount`
        let redeemed_coins = pool_u64::shares_to_amount(
            src_shares_pool,
            amount_to_shares_to_redeem(src_shares_pool, shareholder, amount)
        );
        // if balance on destination would be less than threshold then redeem difference to threshold
        let dst_balance = pool_u64::balance(dst_shares_pool, shareholder);
        if (dst_balance + redeemed_coins < MIN_COINS_ON_SHARES_POOL) {
            // `redeemed_coins` >= `amount` - 1 as redeem can lose at most 1 coin
            amount = MIN_COINS_ON_SHARES_POOL - dst_balance + 1;
        };
        // check if new `amount` drops balance on source under threshold and adjust
        coins_to_redeem_to_ensure_min_stake(src_shares_pool, shareholder, amount)
    }
}
```


<a id="0x1_delegation_pool_retrieve_stake_pool_owner"></a>

## Function `retrieve_stake_pool_owner`

Retrieves the shared resource account owning the stake pool in order
to forward a stake&#45;management operation to this underlying pool.


```move
module 0x1::delegation_pool {
    fun retrieve_stake_pool_owner(pool: &delegation_pool::DelegationPool): signer
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun retrieve_stake_pool_owner(pool: &DelegationPool): signer {
        account::create_signer_with_capability(&pool.stake_pool_signer_cap)
    }
}
```


<a id="0x1_delegation_pool_get_pool_address"></a>

## Function `get_pool_address`

Get the address of delegation pool reference `pool`.


```move
module 0x1::delegation_pool {
    fun get_pool_address(pool: &delegation_pool::DelegationPool): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun get_pool_address(pool: &DelegationPool): address {
        account::get_signer_capability_address(&pool.stake_pool_signer_cap)
    }
}
```


<a id="0x1_delegation_pool_get_delegator_active_shares"></a>

## Function `get_delegator_active_shares`

Get the active share amount of the delegator.


```move
module 0x1::delegation_pool {
    fun get_delegator_active_shares(pool: &delegation_pool::DelegationPool, delegator: address): u128
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun get_delegator_active_shares(pool: &DelegationPool, delegator: address): u128 {
        pool_u64::shares(&pool.active_shares, delegator)
    }
}
```


<a id="0x1_delegation_pool_get_delegator_pending_inactive_shares"></a>

## Function `get_delegator_pending_inactive_shares`

Get the pending inactive share amount of the delegator.


```move
module 0x1::delegation_pool {
    fun get_delegator_pending_inactive_shares(pool: &delegation_pool::DelegationPool, delegator: address): u128
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun get_delegator_pending_inactive_shares(pool: &DelegationPool, delegator: address): u128 {
        pool_u64::shares(pending_inactive_shares_pool(pool), delegator)
    }
}
```


<a id="0x1_delegation_pool_get_used_voting_power"></a>

## Function `get_used_voting_power`

Get the used voting power of a voter on a proposal.


```move
module 0x1::delegation_pool {
    fun get_used_voting_power(governance_records: &delegation_pool::GovernanceRecords, voter: address, proposal_id: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun get_used_voting_power(governance_records: &GovernanceRecords, voter: address, proposal_id: u64): u64 {
        let votes = &governance_records.votes;
        let key = VotingRecordKey {
            voter,
            proposal_id,
        };
        *smart_table::borrow_with_default(votes, key, &0)
    }
}
```


<a id="0x1_delegation_pool_create_resource_account_seed"></a>

## Function `create_resource_account_seed`

Create the seed to derive the resource account address.


```move
module 0x1::delegation_pool {
    fun create_resource_account_seed(delegation_pool_creation_seed: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun create_resource_account_seed(
        delegation_pool_creation_seed: vector<u8>,
    ): vector<u8> {
        let seed = vector::empty<u8>();
        // include module salt (before any subseeds) to avoid conflicts with other modules creating resource accounts
        vector::append(&mut seed, MODULE_SALT);
        // include an additional salt in case the same resource account has already been created
        vector::append(&mut seed, delegation_pool_creation_seed);
        seed
    }
}
```


<a id="0x1_delegation_pool_borrow_mut_used_voting_power"></a>

## Function `borrow_mut_used_voting_power`

Borrow the mutable used voting power of a voter on a proposal.


```move
module 0x1::delegation_pool {
    fun borrow_mut_used_voting_power(governance_records: &mut delegation_pool::GovernanceRecords, voter: address, proposal_id: u64): &mut u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    inline fun borrow_mut_used_voting_power(
        governance_records: &mut GovernanceRecords,
        voter: address,
        proposal_id: u64
    ): &mut u64 {
        let votes = &mut governance_records.votes;
        let key = VotingRecordKey {
            proposal_id,
            voter,
        };
        smart_table::borrow_mut_with_default(votes, key, 0)
    }
}
```


<a id="0x1_delegation_pool_update_and_borrow_mut_delegator_vote_delegation"></a>

## Function `update_and_borrow_mut_delegator_vote_delegation`

Update VoteDelegation of a delegator to up&#45;to&#45;date then borrow_mut it.


```move
module 0x1::delegation_pool {
    fun update_and_borrow_mut_delegator_vote_delegation(pool: &delegation_pool::DelegationPool, governance_records: &mut delegation_pool::GovernanceRecords, delegator: address): &mut delegation_pool::VoteDelegation
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_and_borrow_mut_delegator_vote_delegation(
        pool: &DelegationPool,
        governance_records: &mut GovernanceRecords,
        delegator: address
    ): &mut VoteDelegation {
        let pool_address = get_pool_address(pool);
        let locked_until_secs = stake::get_lockup_secs(pool_address);

        let vote_delegation_table = &mut governance_records.vote_delegation;
        // By default, a delegator's delegated voter is itself.
        // TODO: recycle storage when VoteDelegation equals to default value.
        if (!smart_table::contains(vote_delegation_table, delegator)) {
            return smart_table::borrow_mut_with_default(vote_delegation_table, delegator, VoteDelegation {
                voter: delegator,
                last_locked_until_secs: locked_until_secs,
                pending_voter: delegator,
            })
        };

        let vote_delegation = smart_table::borrow_mut(vote_delegation_table, delegator);
        // A lockup period has passed since last time `vote_delegation` was updated. Pending voter takes effect.
        if (vote_delegation.last_locked_until_secs < locked_until_secs) {
            vote_delegation.voter = vote_delegation.pending_voter;
            vote_delegation.last_locked_until_secs = locked_until_secs;
        };
        vote_delegation
    }
}
```


<a id="0x1_delegation_pool_update_and_borrow_mut_delegated_votes"></a>

## Function `update_and_borrow_mut_delegated_votes`

Update DelegatedVotes of a voter to up&#45;to&#45;date then borrow_mut it.


```move
module 0x1::delegation_pool {
    fun update_and_borrow_mut_delegated_votes(pool: &delegation_pool::DelegationPool, governance_records: &mut delegation_pool::GovernanceRecords, voter: address): &mut delegation_pool::DelegatedVotes
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_and_borrow_mut_delegated_votes(
        pool: &DelegationPool,
        governance_records: &mut GovernanceRecords,
        voter: address
    ): &mut DelegatedVotes {
        let pool_address = get_pool_address(pool);
        let locked_until_secs = stake::get_lockup_secs(pool_address);

        let delegated_votes_per_voter = &mut governance_records.delegated_votes;
        // By default, a delegator's voter is itself.
        // TODO: recycle storage when DelegatedVotes equals to default value.
        if (!smart_table::contains(delegated_votes_per_voter, voter)) {
            let active_shares = get_delegator_active_shares(pool, voter);
            let inactive_shares = get_delegator_pending_inactive_shares(pool, voter);
            return smart_table::borrow_mut_with_default(delegated_votes_per_voter, voter, DelegatedVotes {
                active_shares,
                pending_inactive_shares: inactive_shares,
                active_shares_next_lockup: active_shares,
                last_locked_until_secs: locked_until_secs,
            })
        };

        let delegated_votes = smart_table::borrow_mut(delegated_votes_per_voter, voter);
        // A lockup period has passed since last time `delegated_votes` was updated. Pending voter takes effect.
        if (delegated_votes.last_locked_until_secs < locked_until_secs) {
            delegated_votes.active_shares = delegated_votes.active_shares_next_lockup;
            delegated_votes.pending_inactive_shares = 0;
            delegated_votes.last_locked_until_secs = locked_until_secs;
        };
        delegated_votes
    }
}
```


<a id="0x1_delegation_pool_olc_with_index"></a>

## Function `olc_with_index`



```move
module 0x1::delegation_pool {
    fun olc_with_index(index: u64): delegation_pool::ObservedLockupCycle
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun olc_with_index(index: u64): ObservedLockupCycle {
        ObservedLockupCycle { index }
    }
}
```


<a id="0x1_delegation_pool_calculate_total_voting_power"></a>

## Function `calculate_total_voting_power`

Given the amounts of shares in `active_shares` pool and `inactive_shares` pool, calculate the total voting
power, which equals to the sum of the coin amounts.


```move
module 0x1::delegation_pool {
    fun calculate_total_voting_power(delegation_pool: &delegation_pool::DelegationPool, latest_delegated_votes: &delegation_pool::DelegatedVotes): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun calculate_total_voting_power(delegation_pool: &DelegationPool, latest_delegated_votes: &DelegatedVotes): u64 {
        let active_amount = pool_u64::shares_to_amount(
            &delegation_pool.active_shares,
            latest_delegated_votes.active_shares);
        let pending_inactive_amount = pool_u64::shares_to_amount(
            pending_inactive_shares_pool(delegation_pool),
            latest_delegated_votes.pending_inactive_shares);
        active_amount + pending_inactive_amount
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_delegator_voter_internal"></a>

## Function `calculate_and_update_delegator_voter_internal`

Update VoteDelegation of a delegator to up&#45;to&#45;date then return the latest voter.


```move
module 0x1::delegation_pool {
    fun calculate_and_update_delegator_voter_internal(pool: &delegation_pool::DelegationPool, governance_records: &mut delegation_pool::GovernanceRecords, delegator: address): address
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun calculate_and_update_delegator_voter_internal(
        pool: &DelegationPool,
        governance_records: &mut GovernanceRecords,
        delegator: address
    ): address {
        let vote_delegation = update_and_borrow_mut_delegator_vote_delegation(pool, governance_records, delegator);
        vote_delegation.voter
    }
}
```


<a id="0x1_delegation_pool_calculate_and_update_delegated_votes"></a>

## Function `calculate_and_update_delegated_votes`

Update DelegatedVotes of a voter to up&#45;to&#45;date then return the total voting power of this voter.


```move
module 0x1::delegation_pool {
    fun calculate_and_update_delegated_votes(pool: &delegation_pool::DelegationPool, governance_records: &mut delegation_pool::GovernanceRecords, voter: address): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun calculate_and_update_delegated_votes(
        pool: &DelegationPool,
        governance_records: &mut GovernanceRecords,
        voter: address
    ): u64 {
        let delegated_votes = update_and_borrow_mut_delegated_votes(pool, governance_records, voter);
        calculate_total_voting_power(pool, delegated_votes)
    }
}
```


<a id="0x1_delegation_pool_borrow_mut_delegators_allowlist"></a>

## Function `borrow_mut_delegators_allowlist`



```move
module 0x1::delegation_pool {
    fun borrow_mut_delegators_allowlist(pool_address: address): &mut smart_table::SmartTable<address, bool>
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    inline fun borrow_mut_delegators_allowlist(
        pool_address: address
    ): &mut SmartTable<address, bool> acquires DelegationPoolAllowlisting {
        &mut borrow_global_mut<DelegationPoolAllowlisting>(pool_address).allowlist
    }
}
```


<a id="0x1_delegation_pool_set_operator"></a>

## Function `set_operator`

Allows an owner to change the operator of the underlying stake pool.


```move
module 0x1::delegation_pool {
    public entry fun set_operator(owner: &signer, new_operator: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun set_operator(
        owner: &signer,
        new_operator: address
    ) acquires DelegationPoolOwnership, DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        // synchronize delegation and stake pools before any user operation
        // ensure the old operator is paid its uncommitted commission rewards
        synchronize_delegation_pool(pool_address);
        stake::set_operator(&retrieve_stake_pool_owner(borrow_global<DelegationPool>(pool_address)), new_operator);
    }
}
```


<a id="0x1_delegation_pool_set_beneficiary_for_operator"></a>

## Function `set_beneficiary_for_operator`

Allows an operator to change its beneficiary. Any existing unpaid commission rewards will be paid to the new
beneficiary. To ensure payment to the current beneficiary, one should first call `synchronize_delegation_pool`
before switching the beneficiary. An operator can set one beneficiary for delegation pools, not a separate
one for each pool.


```move
module 0x1::delegation_pool {
    public entry fun set_beneficiary_for_operator(operator: &signer, new_beneficiary: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun set_beneficiary_for_operator(
        operator: &signer,
        new_beneficiary: address
    ) acquires BeneficiaryForOperator {
        assert!(features::operator_beneficiary_change_enabled(), std::error::invalid_state(
            EOPERATOR_BENEFICIARY_CHANGE_NOT_SUPPORTED
        ));
        // The beneficiay address of an operator is stored under the operator's address.
        // So, the operator does not need to be validated with respect to a staking pool.
        let operator_addr = signer::address_of(operator);
        let old_beneficiary = beneficiary_for_operator(operator_addr);
        if (exists<BeneficiaryForOperator>(operator_addr)) {
            borrow_global_mut<BeneficiaryForOperator>(operator_addr).beneficiary_for_operator = new_beneficiary;
        } else {
            move_to(operator, BeneficiaryForOperator { beneficiary_for_operator: new_beneficiary });
        };

        emit(SetBeneficiaryForOperator {
            operator: operator_addr,
            old_beneficiary,
            new_beneficiary,
        });
    }
}
```


<a id="0x1_delegation_pool_update_commission_percentage"></a>

## Function `update_commission_percentage`

Allows an owner to update the commission percentage for the operator of the underlying stake pool.


```move
module 0x1::delegation_pool {
    public entry fun update_commission_percentage(owner: &signer, new_commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun update_commission_percentage(
        owner: &signer,
        new_commission_percentage: u64
    ) acquires DelegationPoolOwnership, DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert!(features::commission_change_delegation_pool_enabled(), error::invalid_state(
            ECOMMISSION_RATE_CHANGE_NOT_SUPPORTED
        ));
        assert!(new_commission_percentage <= MAX_FEE, error::invalid_argument(EINVALID_COMMISSION_PERCENTAGE));
        let owner_address = signer::address_of(owner);
        let pool_address = get_owned_pool_address(owner_address);
        assert!(
            operator_commission_percentage(pool_address) + MAX_COMMISSION_INCREASE >= new_commission_percentage,
            error::invalid_argument(ETOO_LARGE_COMMISSION_INCREASE)
        );
        assert!(
            stake::get_remaining_lockup_secs(pool_address) >= min_remaining_secs_for_commission_change(),
            error::invalid_state(ETOO_LATE_COMMISSION_CHANGE)
        );

        // synchronize delegation and stake pools before any user operation. this ensures:
        // (1) the operator is paid its uncommitted commission rewards with the old commission percentage, and
        // (2) any pending commission percentage change is applied before the new commission percentage is set.
        synchronize_delegation_pool(pool_address);

        if (exists<NextCommissionPercentage>(pool_address)) {
            let commission_percentage = borrow_global_mut<NextCommissionPercentage>(pool_address);
            commission_percentage.commission_percentage_next_lockup_cycle = new_commission_percentage;
            commission_percentage.effective_after_secs = stake::get_lockup_secs(pool_address);
        } else {
            let delegation_pool = borrow_global<DelegationPool>(pool_address);
            let pool_signer = account::create_signer_with_capability(&delegation_pool.stake_pool_signer_cap);
            move_to(&pool_signer, NextCommissionPercentage {
                commission_percentage_next_lockup_cycle: new_commission_percentage,
                effective_after_secs: stake::get_lockup_secs(pool_address),
            });
        };

        event::emit(CommissionPercentageChange {
            pool_address,
            owner: owner_address,
            commission_percentage_next_lockup_cycle: new_commission_percentage,
        });
    }
}
```


<a id="0x1_delegation_pool_set_delegated_voter"></a>

## Function `set_delegated_voter`

Allows an owner to change the delegated voter of the underlying stake pool.


```move
module 0x1::delegation_pool {
    public entry fun set_delegated_voter(owner: &signer, new_voter: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun set_delegated_voter(
        owner: &signer,
        new_voter: address
    ) acquires DelegationPoolOwnership, DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        // No one can change delegated_voter once the partial governance voting feature is enabled.
        assert!(
            !features::delegation_pool_partial_governance_voting_enabled(),
            error::invalid_state(EDEPRECATED_FUNCTION)
        );
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);
        stake::set_delegated_voter(&retrieve_stake_pool_owner(borrow_global<DelegationPool>(pool_address)), new_voter);
    }
}
```


<a id="0x1_delegation_pool_delegate_voting_power"></a>

## Function `delegate_voting_power`

Allows a delegator to delegate its voting power to a voter. If this delegator already has a delegated voter,
this change won&apos;t take effects until the next lockup period.


```move
module 0x1::delegation_pool {
    public entry fun delegate_voting_power(delegator: &signer, pool_address: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun delegate_voting_power(
        delegator: &signer,
        pool_address: address,
        new_voter: address
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_partial_governance_voting_enabled(pool_address);

        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);

        let delegator_address = signer::address_of(delegator);
        let delegation_pool = borrow_global<DelegationPool>(pool_address);
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let delegator_vote_delegation = update_and_borrow_mut_delegator_vote_delegation(
            delegation_pool,
            governance_records,
            delegator_address
        );
        let pending_voter: address = delegator_vote_delegation.pending_voter;

        // No need to update if the voter doesn't really change.
        if (pending_voter != new_voter) {
            delegator_vote_delegation.pending_voter = new_voter;
            let active_shares = get_delegator_active_shares(delegation_pool, delegator_address);
            // <active shares> of <pending voter of shareholder> -= <active_shares>
            // <active shares> of <new voter of shareholder> += <active_shares>
            let pending_delegated_votes = update_and_borrow_mut_delegated_votes(
                delegation_pool,
                governance_records,
                pending_voter
            );
            pending_delegated_votes.active_shares_next_lockup =
                pending_delegated_votes.active_shares_next_lockup - active_shares;

            let new_delegated_votes = update_and_borrow_mut_delegated_votes(
                delegation_pool,
                governance_records,
                new_voter
            );
            new_delegated_votes.active_shares_next_lockup =
                new_delegated_votes.active_shares_next_lockup + active_shares;
        };

        if (features::module_event_migration_enabled()) {
            event::emit(DelegateVotingPower {
                pool_address,
                delegator: delegator_address,
                voter: new_voter,
            })
        };

        event::emit_event(&mut governance_records.delegate_voting_power_events, DelegateVotingPowerEvent {
            pool_address,
            delegator: delegator_address,
            voter: new_voter,
        });
    }
}
```


<a id="0x1_delegation_pool_enable_delegators_allowlisting"></a>

## Function `enable_delegators_allowlisting`

Enable delegators allowlisting as the pool owner.


```move
module 0x1::delegation_pool {
    public entry fun enable_delegators_allowlisting(owner: &signer)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun enable_delegators_allowlisting(
        owner: &signer,
    ) acquires DelegationPoolOwnership, DelegationPool {
        assert!(
            features::delegation_pool_allowlisting_enabled(),
            error::invalid_state(EDELEGATORS_ALLOWLISTING_NOT_SUPPORTED)
        );

        let pool_address = get_owned_pool_address(signer::address_of(owner));
        if (allowlisting_enabled(pool_address)) { return };

        let pool_signer = retrieve_stake_pool_owner(borrow_global<DelegationPool>(pool_address));
        move_to(&pool_signer, DelegationPoolAllowlisting { allowlist: smart_table::new<address, bool>() });

        event::emit(EnableDelegatorsAllowlisting { pool_address });
    }
}
```


<a id="0x1_delegation_pool_disable_delegators_allowlisting"></a>

## Function `disable_delegators_allowlisting`

Disable delegators allowlisting as the pool owner. The existing allowlist will be emptied.


```move
module 0x1::delegation_pool {
    public entry fun disable_delegators_allowlisting(owner: &signer)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun disable_delegators_allowlisting(
        owner: &signer,
    ) acquires DelegationPoolOwnership, DelegationPoolAllowlisting {
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        assert_allowlisting_enabled(pool_address);

        let DelegationPoolAllowlisting { allowlist } = move_from<DelegationPoolAllowlisting>(pool_address);
        // if the allowlist becomes too large, the owner can always remove some delegators
        smart_table::destroy(allowlist);

        event::emit(DisableDelegatorsAllowlisting { pool_address });
    }
}
```


<a id="0x1_delegation_pool_allowlist_delegator"></a>

## Function `allowlist_delegator`

Allowlist a delegator as the pool owner.


```move
module 0x1::delegation_pool {
    public entry fun allowlist_delegator(owner: &signer, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun allowlist_delegator(
        owner: &signer,
        delegator_address: address,
    ) acquires DelegationPoolOwnership, DelegationPoolAllowlisting {
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        assert_allowlisting_enabled(pool_address);

        if (delegator_allowlisted(pool_address, delegator_address)) { return };

        smart_table::add(borrow_mut_delegators_allowlist(pool_address), delegator_address, true);

        event::emit(AllowlistDelegator { pool_address, delegator_address });
    }
}
```


<a id="0x1_delegation_pool_remove_delegator_from_allowlist"></a>

## Function `remove_delegator_from_allowlist`

Remove a delegator from the allowlist as the pool owner, but do not unlock their stake.


```move
module 0x1::delegation_pool {
    public entry fun remove_delegator_from_allowlist(owner: &signer, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun remove_delegator_from_allowlist(
        owner: &signer,
        delegator_address: address,
    ) acquires DelegationPoolOwnership, DelegationPoolAllowlisting {
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        assert_allowlisting_enabled(pool_address);

        if (!delegator_allowlisted(pool_address, delegator_address)) { return };

        smart_table::remove(borrow_mut_delegators_allowlist(pool_address), delegator_address);

        event::emit(RemoveDelegatorFromAllowlist { pool_address, delegator_address });
    }
}
```


<a id="0x1_delegation_pool_evict_delegator"></a>

## Function `evict_delegator`

Evict a delegator that is not allowlisted by unlocking their entire stake.


```move
module 0x1::delegation_pool {
    public entry fun evict_delegator(owner: &signer, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun evict_delegator(
        owner: &signer,
        delegator_address: address,
    ) acquires DelegationPoolOwnership, DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage, DelegationPoolAllowlisting {
        let pool_address = get_owned_pool_address(signer::address_of(owner));
        assert_allowlisting_enabled(pool_address);
        assert!(
            !delegator_allowlisted(pool_address, delegator_address),
            error::invalid_state(ECANNOT_EVICT_ALLOWLISTED_DELEGATOR)
        );

        // synchronize pool in order to query latest balance of delegator
        synchronize_delegation_pool(pool_address);

        let pool = borrow_global<DelegationPool>(pool_address);
        if (get_delegator_active_shares(pool, delegator_address) == 0) { return };

        unlock_internal(delegator_address, pool_address, pool_u64::balance(&pool.active_shares, delegator_address));

        event::emit(EvictDelegator { pool_address, delegator_address });
    }
}
```


<a id="0x1_delegation_pool_add_stake"></a>

## Function `add_stake`

Add `amount` of coins to the delegation pool `pool_address`.


```move
module 0x1::delegation_pool {
    public entry fun add_stake(delegator: &signer, pool_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun add_stake(
        delegator: &signer,
        pool_address: address,
        amount: u64
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage, DelegationPoolAllowlisting {
        // short-circuit if amount to add is 0 so no event is emitted
        if (amount == 0) { return };

        let delegator_address = signer::address_of(delegator);
        assert_delegator_allowlisted(pool_address, delegator_address);

        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);

        // fee to be charged for adding `amount` stake on this delegation pool at this epoch
        let add_stake_fee = get_add_stake_fee(pool_address, amount);

        let pool = borrow_global_mut<DelegationPool>(pool_address);

        // stake the entire amount to the stake pool
        aptos_account::transfer(delegator, pool_address, amount);
        stake::add_stake(&retrieve_stake_pool_owner(pool), amount);

        // but buy shares for delegator just for the remaining amount after fee
        buy_in_active_shares(pool, delegator_address, amount - add_stake_fee);
        assert_min_active_balance(pool, delegator_address);

        // grant temporary ownership over `add_stake` fees to a separate shareholder in order to:
        // - not mistake them for rewards to pay the operator from
        // - distribute them together with the `active` rewards when this epoch ends
        // in order to appreciate all shares on the active pool atomically
        buy_in_active_shares(pool, NULL_SHAREHOLDER, add_stake_fee);

        if (features::module_event_migration_enabled()) {
            event::emit(
                AddStake {
                    pool_address,
                    delegator_address,
                    amount_added: amount,
                    add_stake_fee,
                },
            );
        };

        event::emit_event(
            &mut pool.add_stake_events,
            AddStakeEvent {
                pool_address,
                delegator_address,
                amount_added: amount,
                add_stake_fee,
            },
        );
    }
}
```


<a id="0x1_delegation_pool_unlock"></a>

## Function `unlock`

Unlock `amount` from the active &#43; pending_active stake of `delegator` or
at most how much active stake there is on the stake pool.


```move
module 0x1::delegation_pool {
    public entry fun unlock(delegator: &signer, pool_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun unlock(
        delegator: &signer,
        pool_address: address,
        amount: u64
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        // short-circuit if amount to unlock is 0 so no event is emitted
        if (amount == 0) { return };

        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);

        let delegator_address = signer::address_of(delegator);
        unlock_internal(delegator_address, pool_address, amount);
    }
}
```


<a id="0x1_delegation_pool_unlock_internal"></a>

## Function `unlock_internal`



```move
module 0x1::delegation_pool {
    fun unlock_internal(delegator_address: address, pool_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun unlock_internal(
        delegator_address: address,
        pool_address: address,
        amount: u64
    ) acquires DelegationPool, GovernanceRecords {
        assert!(delegator_address != NULL_SHAREHOLDER, error::invalid_argument(ECANNOT_UNLOCK_NULL_SHAREHOLDER));

        // fail unlock of more stake than `active` on the stake pool
        let (active, _, _, _) = stake::get_stake(pool_address);
        assert!(amount <= active, error::invalid_argument(ENOT_ENOUGH_ACTIVE_STAKE_TO_UNLOCK));

        let pool = borrow_global_mut<DelegationPool>(pool_address);
        amount = coins_to_transfer_to_ensure_min_stake(
            &pool.active_shares,
            pending_inactive_shares_pool(pool),
            delegator_address,
            amount,
        );
        amount = redeem_active_shares(pool, delegator_address, amount);

        stake::unlock(&retrieve_stake_pool_owner(pool), amount);

        buy_in_pending_inactive_shares(pool, delegator_address, amount);
        assert_min_pending_inactive_balance(pool, delegator_address);

        if (features::module_event_migration_enabled()) {
            event::emit(
                UnlockStake {
                    pool_address,
                    delegator_address,
                    amount_unlocked: amount,
                },
            );
        };

        event::emit_event(
            &mut pool.unlock_stake_events,
            UnlockStakeEvent {
                pool_address,
                delegator_address,
                amount_unlocked: amount,
            },
        );
    }
}
```


<a id="0x1_delegation_pool_reactivate_stake"></a>

## Function `reactivate_stake`

Move `amount` of coins from pending_inactive to active.


```move
module 0x1::delegation_pool {
    public entry fun reactivate_stake(delegator: &signer, pool_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun reactivate_stake(
        delegator: &signer,
        pool_address: address,
        amount: u64
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage, DelegationPoolAllowlisting {
        // short-circuit if amount to reactivate is 0 so no event is emitted
        if (amount == 0) { return };

        let delegator_address = signer::address_of(delegator);
        assert_delegator_allowlisted(pool_address, delegator_address);

        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);

        let pool = borrow_global_mut<DelegationPool>(pool_address);
        amount = coins_to_transfer_to_ensure_min_stake(
            pending_inactive_shares_pool(pool),
            &pool.active_shares,
            delegator_address,
            amount,
        );
        let observed_lockup_cycle = pool.observed_lockup_cycle;
        amount = redeem_inactive_shares(pool, delegator_address, amount, observed_lockup_cycle);

        stake::reactivate_stake(&retrieve_stake_pool_owner(pool), amount);

        buy_in_active_shares(pool, delegator_address, amount);
        assert_min_active_balance(pool, delegator_address);

        if (features::module_event_migration_enabled()) {
            event::emit(
                ReactivateStake {
                    pool_address,
                    delegator_address,
                    amount_reactivated: amount,
                },
            );
        };

        event::emit_event(
            &mut pool.reactivate_stake_events,
            ReactivateStakeEvent {
                pool_address,
                delegator_address,
                amount_reactivated: amount,
            },
        );
    }
}
```


<a id="0x1_delegation_pool_withdraw"></a>

## Function `withdraw`

Withdraw `amount` of owned inactive stake from the delegation pool at `pool_address`.


```move
module 0x1::delegation_pool {
    public entry fun withdraw(delegator: &signer, pool_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun withdraw(
        delegator: &signer,
        pool_address: address,
        amount: u64
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert!(amount > 0, error::invalid_argument(EWITHDRAW_ZERO_STAKE));
        // synchronize delegation and stake pools before any user operation
        synchronize_delegation_pool(pool_address);
        withdraw_internal(borrow_global_mut<DelegationPool>(pool_address), signer::address_of(delegator), amount);
    }
}
```


<a id="0x1_delegation_pool_withdraw_internal"></a>

## Function `withdraw_internal`



```move
module 0x1::delegation_pool {
    fun withdraw_internal(pool: &mut delegation_pool::DelegationPool, delegator_address: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun withdraw_internal(
        pool: &mut DelegationPool,
        delegator_address: address,
        amount: u64
    ) acquires GovernanceRecords {
        // TODO: recycle storage when a delegator fully exits the delegation pool.
        // short-circuit if amount to withdraw is 0 so no event is emitted
        if (amount == 0) { return };

        let pool_address = get_pool_address(pool);
        let (withdrawal_exists, withdrawal_olc) = pending_withdrawal_exists(pool, delegator_address);
        // exit if no withdrawal or (it is pending and cannot withdraw pending_inactive stake from stake pool)
        if (!(
            withdrawal_exists &&
                (withdrawal_olc.index < pool.observed_lockup_cycle.index || can_withdraw_pending_inactive(pool_address))
        )) { return };

        if (withdrawal_olc.index == pool.observed_lockup_cycle.index) {
            amount = coins_to_redeem_to_ensure_min_stake(
                pending_inactive_shares_pool(pool),
                delegator_address,
                amount,
            )
        };
        amount = redeem_inactive_shares(pool, delegator_address, amount, withdrawal_olc);

        let stake_pool_owner = &retrieve_stake_pool_owner(pool);
        // stake pool will inactivate entire pending_inactive stake at `stake::withdraw` to make it withdrawable
        // however, bypassing the inactivation of excess stake (inactivated but not withdrawn) ensures
        // the OLC is not advanced indefinitely on `unlock`-`withdraw` paired calls
        if (can_withdraw_pending_inactive(pool_address)) {
            // get excess stake before being entirely inactivated
            let (_, _, _, pending_inactive) = stake::get_stake(pool_address);
            if (withdrawal_olc.index == pool.observed_lockup_cycle.index) {
                // `amount` less excess if withdrawing pending_inactive stake
                pending_inactive = pending_inactive - amount
            };
            // escape excess stake from inactivation
            stake::reactivate_stake(stake_pool_owner, pending_inactive);
            stake::withdraw(stake_pool_owner, amount);
            // restore excess stake to the pending_inactive state
            stake::unlock(stake_pool_owner, pending_inactive);
        } else {
            // no excess stake if `stake::withdraw` does not inactivate at all
            stake::withdraw(stake_pool_owner, amount);
        };
        aptos_account::transfer(stake_pool_owner, delegator_address, amount);

        // commit withdrawal of possibly inactive stake to the `total_coins_inactive`
        // known by the delegation pool in order to not mistake it for slashing at next synchronization
        let (_, inactive, _, _) = stake::get_stake(pool_address);
        pool.total_coins_inactive = inactive;

        if (features::module_event_migration_enabled()) {
            event::emit(
                WithdrawStake {
                    pool_address,
                    delegator_address,
                    amount_withdrawn: amount,
                },
            );
        };

        event::emit_event(
            &mut pool.withdraw_stake_events,
            WithdrawStakeEvent {
                pool_address,
                delegator_address,
                amount_withdrawn: amount,
            },
        );
    }
}
```


<a id="0x1_delegation_pool_pending_withdrawal_exists"></a>

## Function `pending_withdrawal_exists`

Return the unique observed lockup cycle where delegator `delegator_address` may have
unlocking (or already unlocked) stake to be withdrawn from delegation pool `pool`.
A bool is returned to signal if a pending withdrawal exists at all.


```move
module 0x1::delegation_pool {
    fun pending_withdrawal_exists(pool: &delegation_pool::DelegationPool, delegator_address: address): (bool, delegation_pool::ObservedLockupCycle)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun pending_withdrawal_exists(pool: &DelegationPool, delegator_address: address): (bool, ObservedLockupCycle) {
        if (table::contains(&pool.pending_withdrawals, delegator_address)) {
            (true, *table::borrow(&pool.pending_withdrawals, delegator_address))
        } else {
            (false, olc_with_index(0))
        }
    }
}
```


<a id="0x1_delegation_pool_pending_inactive_shares_pool_mut"></a>

## Function `pending_inactive_shares_pool_mut`

Return a mutable reference to the shares pool of `pending_inactive` stake on the
delegation pool, always the last item in `inactive_shares`.


```move
module 0x1::delegation_pool {
    fun pending_inactive_shares_pool_mut(pool: &mut delegation_pool::DelegationPool): &mut pool_u64_unbound::Pool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun pending_inactive_shares_pool_mut(pool: &mut DelegationPool): &mut pool_u64::Pool {
        let observed_lockup_cycle = pool.observed_lockup_cycle;
        table::borrow_mut(&mut pool.inactive_shares, observed_lockup_cycle)
    }
}
```


<a id="0x1_delegation_pool_pending_inactive_shares_pool"></a>

## Function `pending_inactive_shares_pool`



```move
module 0x1::delegation_pool {
    fun pending_inactive_shares_pool(pool: &delegation_pool::DelegationPool): &pool_u64_unbound::Pool
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun pending_inactive_shares_pool(pool: &DelegationPool): &pool_u64::Pool {
        table::borrow(&pool.inactive_shares, pool.observed_lockup_cycle)
    }
}
```


<a id="0x1_delegation_pool_execute_pending_withdrawal"></a>

## Function `execute_pending_withdrawal`

Execute the pending withdrawal of `delegator_address` on delegation pool `pool`
if existing and already inactive to allow the creation of a new one.
`pending_inactive` stake would be left untouched even if withdrawable and should
be explicitly withdrawn by delegator


```move
module 0x1::delegation_pool {
    fun execute_pending_withdrawal(pool: &mut delegation_pool::DelegationPool, delegator_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun execute_pending_withdrawal(pool: &mut DelegationPool, delegator_address: address) acquires GovernanceRecords {
        let (withdrawal_exists, withdrawal_olc) = pending_withdrawal_exists(pool, delegator_address);
        if (withdrawal_exists && withdrawal_olc.index < pool.observed_lockup_cycle.index) {
            withdraw_internal(pool, delegator_address, MAX_U64);
        }
    }
}
```


<a id="0x1_delegation_pool_buy_in_active_shares"></a>

## Function `buy_in_active_shares`

Buy shares into the active pool on behalf of delegator `shareholder` who
deposited `coins_amount`. This function doesn&apos;t make any coin transfer.


```move
module 0x1::delegation_pool {
    fun buy_in_active_shares(pool: &mut delegation_pool::DelegationPool, shareholder: address, coins_amount: u64): u128
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun buy_in_active_shares(
        pool: &mut DelegationPool,
        shareholder: address,
        coins_amount: u64,
    ): u128 acquires GovernanceRecords {
        let new_shares = pool_u64::amount_to_shares(&pool.active_shares, coins_amount);
        // No need to buy 0 shares.
        if (new_shares == 0) { return 0 };

        // Always update governance records before any change to the shares pool.
        let pool_address = get_pool_address(pool);
        if (partial_governance_voting_enabled(pool_address)) {
            update_governance_records_for_buy_in_active_shares(pool, pool_address, new_shares, shareholder);
        };

        pool_u64::buy_in(&mut pool.active_shares, shareholder, coins_amount);
        new_shares
    }
}
```


<a id="0x1_delegation_pool_buy_in_pending_inactive_shares"></a>

## Function `buy_in_pending_inactive_shares`

Buy shares into the pending_inactive pool on behalf of delegator `shareholder` who
redeemed `coins_amount` from the active pool to schedule it for unlocking.
If delegator&apos;s pending withdrawal exists and has been inactivated, execute it firstly
to ensure there is always only one withdrawal request.


```move
module 0x1::delegation_pool {
    fun buy_in_pending_inactive_shares(pool: &mut delegation_pool::DelegationPool, shareholder: address, coins_amount: u64): u128
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun buy_in_pending_inactive_shares(
        pool: &mut DelegationPool,
        shareholder: address,
        coins_amount: u64,
    ): u128 acquires GovernanceRecords {
        let new_shares = pool_u64::amount_to_shares(pending_inactive_shares_pool(pool), coins_amount);
        // never create a new pending withdrawal unless delegator owns some pending_inactive shares
        if (new_shares == 0) { return 0 };

        // Always update governance records before any change to the shares pool.
        let pool_address = get_pool_address(pool);
        if (partial_governance_voting_enabled(pool_address)) {
            update_governance_records_for_buy_in_pending_inactive_shares(pool, pool_address, new_shares, shareholder);
        };

        // cannot buy inactive shares, only pending_inactive at current lockup cycle
        pool_u64::buy_in(pending_inactive_shares_pool_mut(pool), shareholder, coins_amount);

        // execute the pending withdrawal if exists and is inactive before creating a new one
        execute_pending_withdrawal(pool, shareholder);

        // save observed lockup cycle for the new pending withdrawal
        let observed_lockup_cycle = pool.observed_lockup_cycle;
        assert!(*table::borrow_mut_with_default(
            &mut pool.pending_withdrawals,
            shareholder,
            observed_lockup_cycle
        ) == observed_lockup_cycle,
            error::invalid_state(EPENDING_WITHDRAWAL_EXISTS)
        );

        new_shares
    }
}
```


<a id="0x1_delegation_pool_amount_to_shares_to_redeem"></a>

## Function `amount_to_shares_to_redeem`

Convert `coins_amount` of coins to be redeemed from shares pool `shares_pool`
to the exact number of shares to redeem in order to achieve this.


```move
module 0x1::delegation_pool {
    fun amount_to_shares_to_redeem(shares_pool: &pool_u64_unbound::Pool, shareholder: address, coins_amount: u64): u128
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun amount_to_shares_to_redeem(
        shares_pool: &pool_u64::Pool,
        shareholder: address,
        coins_amount: u64,
    ): u128 {
        if (coins_amount >= pool_u64::balance(shares_pool, shareholder)) {
            // cap result at total shares of shareholder to pass `EINSUFFICIENT_SHARES` on subsequent redeem
            pool_u64::shares(shares_pool, shareholder)
        } else {
            pool_u64::amount_to_shares(shares_pool, coins_amount)
        }
    }
}
```


<a id="0x1_delegation_pool_redeem_active_shares"></a>

## Function `redeem_active_shares`

Redeem shares from the active pool on behalf of delegator `shareholder` who
wants to unlock `coins_amount` of its active stake.
Extracted coins will be used to buy shares into the pending_inactive pool and
be available for withdrawal when current OLC ends.


```move
module 0x1::delegation_pool {
    fun redeem_active_shares(pool: &mut delegation_pool::DelegationPool, shareholder: address, coins_amount: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun redeem_active_shares(
        pool: &mut DelegationPool,
        shareholder: address,
        coins_amount: u64,
    ): u64 acquires GovernanceRecords {
        let shares_to_redeem = amount_to_shares_to_redeem(&pool.active_shares, shareholder, coins_amount);
        // silently exit if not a shareholder otherwise redeem would fail with `ESHAREHOLDER_NOT_FOUND`
        if (shares_to_redeem == 0) return 0;

        // Always update governance records before any change to the shares pool.
        let pool_address = get_pool_address(pool);
        if (partial_governance_voting_enabled(pool_address)) {
            update_governanace_records_for_redeem_active_shares(pool, pool_address, shares_to_redeem, shareholder);
        };

        pool_u64::redeem_shares(&mut pool.active_shares, shareholder, shares_to_redeem)
    }
}
```


<a id="0x1_delegation_pool_redeem_inactive_shares"></a>

## Function `redeem_inactive_shares`

Redeem shares from the inactive pool at `lockup_cycle` &lt; current OLC on behalf of
delegator `shareholder` who wants to withdraw `coins_amount` of its unlocked stake.
Redeem shares from the pending_inactive pool at `lockup_cycle` &#61;&#61; current OLC on behalf of
delegator `shareholder` who wants to reactivate `coins_amount` of its unlocking stake.
For latter case, extracted coins will be used to buy shares into the active pool and
escape inactivation when current lockup ends.


```move
module 0x1::delegation_pool {
    fun redeem_inactive_shares(pool: &mut delegation_pool::DelegationPool, shareholder: address, coins_amount: u64, lockup_cycle: delegation_pool::ObservedLockupCycle): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun redeem_inactive_shares(
        pool: &mut DelegationPool,
        shareholder: address,
        coins_amount: u64,
        lockup_cycle: ObservedLockupCycle,
    ): u64 acquires GovernanceRecords {
        let shares_to_redeem = amount_to_shares_to_redeem(
            table::borrow(&pool.inactive_shares, lockup_cycle),
            shareholder,
            coins_amount);
        // silently exit if not a shareholder otherwise redeem would fail with `ESHAREHOLDER_NOT_FOUND`
        if (shares_to_redeem == 0) return 0;

        // Always update governance records before any change to the shares pool.
        let pool_address = get_pool_address(pool);
        // Only redeem shares from the pending_inactive pool at `lockup_cycle` == current OLC.
        if (partial_governance_voting_enabled(pool_address) && lockup_cycle.index == pool.observed_lockup_cycle.index) {
            update_governanace_records_for_redeem_pending_inactive_shares(
                pool,
                pool_address,
                shares_to_redeem,
                shareholder
            );
        };

        let inactive_shares = table::borrow_mut(&mut pool.inactive_shares, lockup_cycle);
        // 1. reaching here means delegator owns inactive/pending_inactive shares at OLC `lockup_cycle`
        let redeemed_coins = pool_u64::redeem_shares(inactive_shares, shareholder, shares_to_redeem);

        // if entirely reactivated pending_inactive stake or withdrawn inactive one,
        // re-enable unlocking for delegator by deleting this pending withdrawal
        if (pool_u64::shares(inactive_shares, shareholder) == 0) {
            // 2. a delegator owns inactive/pending_inactive shares only at the OLC of its pending withdrawal
            // 1 & 2: the pending withdrawal itself has been emptied of shares and can be safely deleted
            table::remove(&mut pool.pending_withdrawals, shareholder);
        };
        // destroy inactive shares pool of past OLC if all its stake has been withdrawn
        if (lockup_cycle.index < pool.observed_lockup_cycle.index && total_coins(inactive_shares) == 0) {
            pool_u64::destroy_empty(table::remove(&mut pool.inactive_shares, lockup_cycle));
        };

        redeemed_coins
    }
}
```


<a id="0x1_delegation_pool_calculate_stake_pool_drift"></a>

## Function `calculate_stake_pool_drift`

Calculate stake deviations between the delegation and stake pools in order to
capture the rewards earned in the meantime, resulted operator commission and
whether the lockup expired on the stake pool.


```move
module 0x1::delegation_pool {
    fun calculate_stake_pool_drift(pool: &delegation_pool::DelegationPool): (bool, u64, u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun calculate_stake_pool_drift(pool: &DelegationPool): (bool, u64, u64, u64, u64) {
        let (active, inactive, pending_active, pending_inactive) = stake::get_stake(get_pool_address(pool));
        assert!(
            inactive >= pool.total_coins_inactive,
            error::invalid_state(ESLASHED_INACTIVE_STAKE_ON_PAST_OLC)
        );
        // determine whether a new lockup cycle has been ended on the stake pool and
        // inactivated SOME `pending_inactive` stake which should stop earning rewards now,
        // thus requiring separation of the `pending_inactive` stake on current observed lockup
        // and the future one on the newly started lockup
        let lockup_cycle_ended = inactive > pool.total_coins_inactive;

        // actual coins on stake pool belonging to the active shares pool
        active = active + pending_active;
        // actual coins on stake pool belonging to the shares pool hosting `pending_inactive` stake
        // at current observed lockup cycle, either pending: `pending_inactive` or already inactivated:
        if (lockup_cycle_ended) {
            // `inactive` on stake pool = any previous `inactive` stake +
            // any previous `pending_inactive` stake and its rewards (both inactivated)
            pending_inactive = inactive - pool.total_coins_inactive
        };

        // on stake-management operations, total coins on the internal shares pools and individual
        // stakes on the stake pool are updated simultaneously, thus the only stakes becoming
        // unsynced are rewards and slashes routed exclusively to/out the stake pool

        // operator `active` rewards not persisted yet to the active shares pool
        let pool_active = total_coins(&pool.active_shares);
        let commission_active = if (active > pool_active) {
            math64::mul_div(active - pool_active, pool.operator_commission_percentage, MAX_FEE)
        } else {
            // handle any slashing applied to `active` stake
            0
        };
        // operator `pending_inactive` rewards not persisted yet to the pending_inactive shares pool
        let pool_pending_inactive = total_coins(pending_inactive_shares_pool(pool));
        let commission_pending_inactive = if (pending_inactive > pool_pending_inactive) {
            math64::mul_div(
                pending_inactive - pool_pending_inactive,
                pool.operator_commission_percentage,
                MAX_FEE
            )
        } else {
            // handle any slashing applied to `pending_inactive` stake
            0
        };

        (lockup_cycle_ended, active, pending_inactive, commission_active, commission_pending_inactive)
    }
}
```


<a id="0x1_delegation_pool_synchronize_delegation_pool"></a>

## Function `synchronize_delegation_pool`

Synchronize delegation and stake pools: distribute yet&#45;undetected rewards to the corresponding internal
shares pools, assign commission to operator and eventually prepare delegation pool for a new lockup cycle.


```move
module 0x1::delegation_pool {
    public entry fun synchronize_delegation_pool(pool_address: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public entry fun synchronize_delegation_pool(
        pool_address: address
    ) acquires DelegationPool, GovernanceRecords, BeneficiaryForOperator, NextCommissionPercentage {
        assert_delegation_pool_exists(pool_address);
        let pool = borrow_global_mut<DelegationPool>(pool_address);
        let (
            lockup_cycle_ended,
            active,
            pending_inactive,
            commission_active,
            commission_pending_inactive
        ) = calculate_stake_pool_drift(pool);

        // zero `pending_active` stake indicates that either there are no `add_stake` fees or
        // previous epoch has ended and should release the shares owning the existing fees
        let (_, _, pending_active, _) = stake::get_stake(pool_address);
        if (pending_active == 0) {
            // renounce ownership over the `add_stake` fees by redeeming all shares of
            // the special shareholder, implicitly their equivalent coins, out of the active shares pool
            redeem_active_shares(pool, NULL_SHAREHOLDER, MAX_U64);
        };

        // distribute rewards remaining after commission, to delegators (to already existing shares)
        // before buying shares for the operator for its entire commission fee
        // otherwise, operator's new shares would additionally appreciate from rewards it does not own

        // update total coins accumulated by `active` + `pending_active` shares
        // redeemed `add_stake` fees are restored and distributed to the rest of the pool as rewards
        pool_u64::update_total_coins(&mut pool.active_shares, active - commission_active);
        // update total coins accumulated by `pending_inactive` shares at current observed lockup cycle
        pool_u64::update_total_coins(
            pending_inactive_shares_pool_mut(pool),
            pending_inactive - commission_pending_inactive
        );

        // reward operator its commission out of uncommitted active rewards (`add_stake` fees already excluded)
        buy_in_active_shares(pool, beneficiary_for_operator(stake::get_operator(pool_address)), commission_active);
        // reward operator its commission out of uncommitted pending_inactive rewards
        buy_in_pending_inactive_shares(
            pool,
            beneficiary_for_operator(stake::get_operator(pool_address)),
            commission_pending_inactive
        );

        event::emit_event(
            &mut pool.distribute_commission_events,
            DistributeCommissionEvent {
                pool_address,
                operator: stake::get_operator(pool_address),
                commission_active,
                commission_pending_inactive,
            },
        );

        if (features::operator_beneficiary_change_enabled()) {
            emit(DistributeCommission {
                pool_address,
                operator: stake::get_operator(pool_address),
                beneficiary: beneficiary_for_operator(stake::get_operator(pool_address)),
                commission_active,
                commission_pending_inactive,
            })
        };

        // advance lockup cycle on delegation pool if already ended on stake pool (AND stake explicitly inactivated)
        if (lockup_cycle_ended) {
            // capture inactive coins over all ended lockup cycles (including this ending one)
            let (_, inactive, _, _) = stake::get_stake(pool_address);
            pool.total_coins_inactive = inactive;

            // advance lockup cycle on the delegation pool
            pool.observed_lockup_cycle.index = pool.observed_lockup_cycle.index + 1;
            // start new lockup cycle with a fresh shares pool for `pending_inactive` stake
            table::add(
                &mut pool.inactive_shares,
                pool.observed_lockup_cycle,
                pool_u64::create_with_scaling_factor(SHARES_SCALING_FACTOR)
            );
        };

        if (is_next_commission_percentage_effective(pool_address)) {
            pool.operator_commission_percentage = borrow_global<NextCommissionPercentage>(
                pool_address
            ).commission_percentage_next_lockup_cycle;
        }
    }
}
```


<a id="0x1_delegation_pool_assert_and_update_proposal_used_voting_power"></a>

## Function `assert_and_update_proposal_used_voting_power`



```move
module 0x1::delegation_pool {
    fun assert_and_update_proposal_used_voting_power(governance_records: &mut delegation_pool::GovernanceRecords, pool_address: address, proposal_id: u64, voting_power: u64)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    inline fun assert_and_update_proposal_used_voting_power(
        governance_records: &mut GovernanceRecords, pool_address: address, proposal_id: u64, voting_power: u64
    ) {
        let stake_pool_remaining_voting_power = aptos_governance::get_remaining_voting_power(pool_address, proposal_id);
        let stake_pool_used_voting_power = aptos_governance::get_voting_power(
            pool_address
        ) - stake_pool_remaining_voting_power;
        let proposal_used_voting_power = smart_table::borrow_mut_with_default(
            &mut governance_records.votes_per_proposal,
            proposal_id,
            0
        );
        // A edge case: Before enabling partial governance voting on a delegation pool, the delegation pool has
        // a voter which can vote with all voting power of this delegation pool. If the voter votes on a proposal after
        // partial governance voting flag is enabled, the delegation pool doesn't have enough voting power on this
        // proposal for all the delegators. To be fair, no one can vote on this proposal through this delegation pool.
        // To detect this case, check if the stake pool had used voting power not through delegation_pool module.
        assert!(
            stake_pool_used_voting_power == *proposal_used_voting_power,
            error::invalid_argument(EALREADY_VOTED_BEFORE_ENABLE_PARTIAL_VOTING)
        );
        *proposal_used_voting_power = *proposal_used_voting_power + voting_power;
    }
}
```


<a id="0x1_delegation_pool_update_governance_records_for_buy_in_active_shares"></a>

## Function `update_governance_records_for_buy_in_active_shares`



```move
module 0x1::delegation_pool {
    fun update_governance_records_for_buy_in_active_shares(pool: &delegation_pool::DelegationPool, pool_address: address, new_shares: u128, shareholder: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_governance_records_for_buy_in_active_shares(
        pool: &DelegationPool, pool_address: address, new_shares: u128, shareholder: address
    ) acquires GovernanceRecords {
        // <active shares> of <shareholder> += <new_shares> ---->
        // <active shares> of <current voter of shareholder> += <new_shares>
        // <active shares> of <next voter of shareholder> += <new_shares>
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let vote_delegation = update_and_borrow_mut_delegator_vote_delegation(pool, governance_records, shareholder);
        let current_voter = vote_delegation.voter;
        let pending_voter = vote_delegation.pending_voter;
        let current_delegated_votes =
            update_and_borrow_mut_delegated_votes(pool, governance_records, current_voter);
        current_delegated_votes.active_shares = current_delegated_votes.active_shares + new_shares;
        if (pending_voter == current_voter) {
            current_delegated_votes.active_shares_next_lockup =
                current_delegated_votes.active_shares_next_lockup + new_shares;
        } else {
            let pending_delegated_votes =
                update_and_borrow_mut_delegated_votes(pool, governance_records, pending_voter);
            pending_delegated_votes.active_shares_next_lockup =
                pending_delegated_votes.active_shares_next_lockup + new_shares;
        };
    }
}
```


<a id="0x1_delegation_pool_update_governance_records_for_buy_in_pending_inactive_shares"></a>

## Function `update_governance_records_for_buy_in_pending_inactive_shares`



```move
module 0x1::delegation_pool {
    fun update_governance_records_for_buy_in_pending_inactive_shares(pool: &delegation_pool::DelegationPool, pool_address: address, new_shares: u128, shareholder: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_governance_records_for_buy_in_pending_inactive_shares(
        pool: &DelegationPool, pool_address: address, new_shares: u128, shareholder: address
    ) acquires GovernanceRecords {
        // <pending inactive shares> of <shareholder> += <new_shares>   ---->
        // <pending inactive shares> of <current voter of shareholder> += <new_shares>
        // no impact on <pending inactive shares> of <next voter of shareholder>
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let current_voter = calculate_and_update_delegator_voter_internal(pool, governance_records, shareholder);
        let current_delegated_votes = update_and_borrow_mut_delegated_votes(pool, governance_records, current_voter);
        current_delegated_votes.pending_inactive_shares = current_delegated_votes.pending_inactive_shares + new_shares;
    }
}
```


<a id="0x1_delegation_pool_update_governanace_records_for_redeem_active_shares"></a>

## Function `update_governanace_records_for_redeem_active_shares`



```move
module 0x1::delegation_pool {
    fun update_governanace_records_for_redeem_active_shares(pool: &delegation_pool::DelegationPool, pool_address: address, shares_to_redeem: u128, shareholder: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_governanace_records_for_redeem_active_shares(
        pool: &DelegationPool, pool_address: address, shares_to_redeem: u128, shareholder: address
    ) acquires GovernanceRecords {
        // <active shares> of <shareholder> -= <shares_to_redeem> ---->
        // <active shares> of <current voter of shareholder> -= <shares_to_redeem>
        // <active shares> of <next voter of shareholder> -= <shares_to_redeem>
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let vote_delegation = update_and_borrow_mut_delegator_vote_delegation(
            pool,
            governance_records,
            shareholder
        );
        let current_voter = vote_delegation.voter;
        let pending_voter = vote_delegation.pending_voter;
        let current_delegated_votes = update_and_borrow_mut_delegated_votes(pool, governance_records, current_voter);
        current_delegated_votes.active_shares = current_delegated_votes.active_shares - shares_to_redeem;
        if (current_voter == pending_voter) {
            current_delegated_votes.active_shares_next_lockup =
                current_delegated_votes.active_shares_next_lockup - shares_to_redeem;
        } else {
            let pending_delegated_votes =
                update_and_borrow_mut_delegated_votes(pool, governance_records, pending_voter);
            pending_delegated_votes.active_shares_next_lockup =
                pending_delegated_votes.active_shares_next_lockup - shares_to_redeem;
        };
    }
}
```


<a id="0x1_delegation_pool_update_governanace_records_for_redeem_pending_inactive_shares"></a>

## Function `update_governanace_records_for_redeem_pending_inactive_shares`



```move
module 0x1::delegation_pool {
    fun update_governanace_records_for_redeem_pending_inactive_shares(pool: &delegation_pool::DelegationPool, pool_address: address, shares_to_redeem: u128, shareholder: address)
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    fun update_governanace_records_for_redeem_pending_inactive_shares(
        pool: &DelegationPool, pool_address: address, shares_to_redeem: u128, shareholder: address
    ) acquires GovernanceRecords {
        // <pending inactive shares> of <shareholder> -= <shares_to_redeem>  ---->
        // <pending inactive shares> of <current voter of shareholder> -= <shares_to_redeem>
        // no impact on <pending inactive shares> of <next voter of shareholder>
        let governance_records = borrow_global_mut<GovernanceRecords>(pool_address);
        let current_voter = calculate_and_update_delegator_voter_internal(pool, governance_records, shareholder);
        let current_delegated_votes = update_and_borrow_mut_delegated_votes(pool, governance_records, current_voter);
        current_delegated_votes.pending_inactive_shares = current_delegated_votes.pending_inactive_shares - shares_to_redeem;
    }
}
```


<a id="0x1_delegation_pool_multiply_then_divide"></a>

## Function `multiply_then_divide`

Deprecated, prefer math64::mul_div


```move
module 0x1::delegation_pool {
    #[deprecated]
    public fun multiply_then_divide(x: u64, y: u64, z: u64): u64
}
```


##### Implementation


```move
module 0x1::delegation_pool {
    public fun multiply_then_divide(x: u64, y: u64, z: u64): u64 {
        math64::mul_div(x, y, z)
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
<td>Every DelegationPool has only one corresponding StakePool stored at the same address.</td>
<td>Critical</td>
<td>Upon calling the initialize_delegation_pool function, a resource account is created from the &quot;owner&quot; signer to host the delegation pool resource and own the underlying stake pool.</td>
<td>Audited that the address of StakePool equals address of DelegationPool and the data invariant on the DelegationPool.</td>
</tr>

<tr>
<td>2</td>
<td>The signer capability within the delegation pool has an address equal to the address of the delegation pool.</td>
<td>Critical</td>
<td>The initialize_delegation_pool function moves the DelegationPool resource to the address associated with stake_pool_signer, which also possesses the signer capability.</td>
<td>Audited that the address of signer cap equals address of DelegationPool.</td>
</tr>

<tr>
<td>3</td>
<td>A delegator holds shares exclusively in one inactive shares pool, which could either be an already inactive pool or the pending_inactive pool.</td>
<td>High</td>
<td>The get_stake function returns the inactive stake owned by a delegator and checks which state the shares are in via the get_pending_withdrawal function.</td>
<td>Audited that either inactive or pending_inactive stake after invoking the get_stake function is zero and both are never non&#45;zero.</td>
</tr>

<tr>
<td>4</td>
<td>The specific pool in which the delegator possesses inactive shares becomes designated as the pending withdrawal pool for that delegator.</td>
<td>Medium</td>
<td>The get_pending_withdrawal function checks if any pending withdrawal exists for a delegate address and if there is neither inactive nor pending_inactive stake, the pending_withdrawal_exists returns false.</td>
<td>This has been audited.</td>
</tr>

<tr>
<td>5</td>
<td>The existence of a pending withdrawal implies that it is associated with a pool where the delegator possesses inactive shares.</td>
<td>Medium</td>
<td>In the get_pending_withdrawal function, if withdrawal_exists is true, the function returns true and a non&#45;zero amount</td>
<td>get_pending_withdrawal has been audited.</td>
</tr>

<tr>
<td>6</td>
<td>An inactive shares pool should have coins allocated to it; otherwise, it should become deleted.</td>
<td>Medium</td>
<td>The redeem_inactive_shares function has a check that destroys the inactive shares pool, given that it is empty.</td>
<td>shares pools have been audited.</td>
</tr>

<tr>
<td>7</td>
<td>The index of the pending withdrawal will not exceed the current OLC on DelegationPool.</td>
<td>High</td>
<td>The get_pending_withdrawal function has a check which ensures that withdrawal_olc.index < pool.observed_lockup_cycle.index.</td>
<td>This has been audited.</td>
</tr>

<tr>
<td>8</td>
<td>Slashing is not possible for inactive stakes.</td>
<td>Critical</td>
<td>The number of inactive staked coins must be greater than or equal to the total_coins_inactive of the pool.</td>
<td>This has been audited.</td>
</tr>

<tr>
<td>9</td>
<td>The delegator&apos;s active or pending inactive stake will always meet or exceed the minimum allowed value.</td>
<td>Medium</td>
<td>The add_stake, unlock and reactivate_stake functions ensure the active_shares or pending_inactive_shares balance for the delegator is greater than or equal to the MIN_COINS_ON_SHARES_POOL value.</td>
<td>Audited the comparison of active_shares or inactive_shares balance for the delegator with the MIN_COINS_ON_SHARES_POOL value.</td>
</tr>

<tr>
<td>10</td>
<td>The delegation pool exists at a given address.</td>
<td>Low</td>
<td>Functions that operate on the DelegationPool abort if there is no DelegationPool struct under the given pool_address.</td>
<td>Audited that there is no DelegationPool structure assigned to the pool_address given as a parameter.</td>
</tr>

<tr>
<td>11</td>
<td>The initialization of the delegation pool is contingent upon enabling the delegation pools feature.</td>
<td>Critical</td>
<td>The initialize_delegation_pool function should proceed if the DELEGATION_POOLS feature is enabled.</td>
<td>This has been audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::delegation_pool {
    pragma verify=false;
}
```
