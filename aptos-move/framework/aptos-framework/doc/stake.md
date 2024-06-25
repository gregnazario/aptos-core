
<a id="0x1_stake"></a>

# Module `0x1::stake`


Validator lifecycle:
1. Prepare a validator node set up and call stake::initialize_validator
2. Once ready to deposit stake (or have funds assigned by a staking service in exchange for ownership capability),
call stake::add_stake (or &#42;_with_cap versions if called from the staking service)
3. Call stake::join_validator_set (or _with_cap version) to join the active validator set. Changes are effective in
the next epoch.
4. Validate and gain rewards. The stake will automatically be locked up for a fixed duration (set by governance) and
automatically renewed at expiration.
5. At any point, if the validator operator wants to update the consensus key or network/fullnode addresses, they can
call stake::rotate_consensus_key and stake::update_network_and_fullnode_addresses. Similar to changes to stake, the
changes to consensus key/network/fullnode addresses are only effective in the next epoch.
6. Validator can request to unlock their stake at any time. However, their stake will only become withdrawable when
their current lockup expires. This can be at most as long as the fixed lockup duration.
7. After exiting, the validator can either explicitly leave the validator set by calling stake::leave_validator_set
or if their stake drops below the min required, they would get removed at the end of the epoch.
8. Validator can always rejoin the validator set by going through steps 2&#45;3 again.
9. An owner can always switch operators by calling stake::set_operator.
10. An owner can always switch designated voter by calling stake::set_designated_voter.


-  [Resource `OwnerCapability`](#0x1_stake_OwnerCapability)
-  [Resource `StakePool`](#0x1_stake_StakePool)
-  [Resource `ValidatorConfig`](#0x1_stake_ValidatorConfig)
-  [Struct `ValidatorInfo`](#0x1_stake_ValidatorInfo)
-  [Resource `ValidatorSet`](#0x1_stake_ValidatorSet)
-  [Resource `AptosCoinCapabilities`](#0x1_stake_AptosCoinCapabilities)
-  [Struct `IndividualValidatorPerformance`](#0x1_stake_IndividualValidatorPerformance)
-  [Resource `ValidatorPerformance`](#0x1_stake_ValidatorPerformance)
-  [Struct `RegisterValidatorCandidateEvent`](#0x1_stake_RegisterValidatorCandidateEvent)
-  [Struct `RegisterValidatorCandidate`](#0x1_stake_RegisterValidatorCandidate)
-  [Struct `SetOperatorEvent`](#0x1_stake_SetOperatorEvent)
-  [Struct `SetOperator`](#0x1_stake_SetOperator)
-  [Struct `AddStakeEvent`](#0x1_stake_AddStakeEvent)
-  [Struct `AddStake`](#0x1_stake_AddStake)
-  [Struct `ReactivateStakeEvent`](#0x1_stake_ReactivateStakeEvent)
-  [Struct `ReactivateStake`](#0x1_stake_ReactivateStake)
-  [Struct `RotateConsensusKeyEvent`](#0x1_stake_RotateConsensusKeyEvent)
-  [Struct `RotateConsensusKey`](#0x1_stake_RotateConsensusKey)
-  [Struct `UpdateNetworkAndFullnodeAddressesEvent`](#0x1_stake_UpdateNetworkAndFullnodeAddressesEvent)
-  [Struct `UpdateNetworkAndFullnodeAddresses`](#0x1_stake_UpdateNetworkAndFullnodeAddresses)
-  [Struct `IncreaseLockupEvent`](#0x1_stake_IncreaseLockupEvent)
-  [Struct `IncreaseLockup`](#0x1_stake_IncreaseLockup)
-  [Struct `JoinValidatorSetEvent`](#0x1_stake_JoinValidatorSetEvent)
-  [Struct `JoinValidatorSet`](#0x1_stake_JoinValidatorSet)
-  [Struct `DistributeRewardsEvent`](#0x1_stake_DistributeRewardsEvent)
-  [Struct `DistributeRewards`](#0x1_stake_DistributeRewards)
-  [Struct `UnlockStakeEvent`](#0x1_stake_UnlockStakeEvent)
-  [Struct `UnlockStake`](#0x1_stake_UnlockStake)
-  [Struct `WithdrawStakeEvent`](#0x1_stake_WithdrawStakeEvent)
-  [Struct `WithdrawStake`](#0x1_stake_WithdrawStake)
-  [Struct `LeaveValidatorSetEvent`](#0x1_stake_LeaveValidatorSetEvent)
-  [Struct `LeaveValidatorSet`](#0x1_stake_LeaveValidatorSet)
-  [Resource `ValidatorFees`](#0x1_stake_ValidatorFees)
-  [Resource `AllowedValidators`](#0x1_stake_AllowedValidators)
-  [Resource `Ghost$ghost_valid_perf`](#0x1_stake_Ghost$ghost_valid_perf)
-  [Resource `Ghost$ghost_proposer_idx`](#0x1_stake_Ghost$ghost_proposer_idx)
-  [Resource `Ghost$ghost_active_num`](#0x1_stake_Ghost$ghost_active_num)
-  [Resource `Ghost$ghost_pending_inactive_num`](#0x1_stake_Ghost$ghost_pending_inactive_num)
-  [Constants](#@Constants_0)
-  [Function `initialize_validator_fees`](#0x1_stake_initialize_validator_fees)
-  [Function `add_transaction_fee`](#0x1_stake_add_transaction_fee)
-  [Function `get_lockup_secs`](#0x1_stake_get_lockup_secs)
-  [Function `get_remaining_lockup_secs`](#0x1_stake_get_remaining_lockup_secs)
-  [Function `get_stake`](#0x1_stake_get_stake)
-  [Function `get_validator_state`](#0x1_stake_get_validator_state)
-  [Function `get_current_epoch_voting_power`](#0x1_stake_get_current_epoch_voting_power)
-  [Function `get_delegated_voter`](#0x1_stake_get_delegated_voter)
-  [Function `get_operator`](#0x1_stake_get_operator)
-  [Function `get_owned_pool_address`](#0x1_stake_get_owned_pool_address)
-  [Function `get_validator_index`](#0x1_stake_get_validator_index)
-  [Function `get_current_epoch_proposal_counts`](#0x1_stake_get_current_epoch_proposal_counts)
-  [Function `get_validator_config`](#0x1_stake_get_validator_config)
-  [Function `stake_pool_exists`](#0x1_stake_stake_pool_exists)
-  [Function `initialize`](#0x1_stake_initialize)
-  [Function `store_aptos_coin_mint_cap`](#0x1_stake_store_aptos_coin_mint_cap)
-  [Function `remove_validators`](#0x1_stake_remove_validators)
-  [Function `initialize_stake_owner`](#0x1_stake_initialize_stake_owner)
-  [Function `initialize_validator`](#0x1_stake_initialize_validator)
-  [Function `initialize_owner`](#0x1_stake_initialize_owner)
-  [Function `extract_owner_cap`](#0x1_stake_extract_owner_cap)
-  [Function `deposit_owner_cap`](#0x1_stake_deposit_owner_cap)
-  [Function `destroy_owner_cap`](#0x1_stake_destroy_owner_cap)
-  [Function `set_operator`](#0x1_stake_set_operator)
-  [Function `set_operator_with_cap`](#0x1_stake_set_operator_with_cap)
-  [Function `set_delegated_voter`](#0x1_stake_set_delegated_voter)
-  [Function `set_delegated_voter_with_cap`](#0x1_stake_set_delegated_voter_with_cap)
-  [Function `add_stake`](#0x1_stake_add_stake)
-  [Function `add_stake_with_cap`](#0x1_stake_add_stake_with_cap)
-  [Function `reactivate_stake`](#0x1_stake_reactivate_stake)
-  [Function `reactivate_stake_with_cap`](#0x1_stake_reactivate_stake_with_cap)
-  [Function `rotate_consensus_key`](#0x1_stake_rotate_consensus_key)
-  [Function `update_network_and_fullnode_addresses`](#0x1_stake_update_network_and_fullnode_addresses)
-  [Function `increase_lockup`](#0x1_stake_increase_lockup)
-  [Function `increase_lockup_with_cap`](#0x1_stake_increase_lockup_with_cap)
-  [Function `join_validator_set`](#0x1_stake_join_validator_set)
-  [Function `join_validator_set_internal`](#0x1_stake_join_validator_set_internal)
-  [Function `unlock`](#0x1_stake_unlock)
-  [Function `unlock_with_cap`](#0x1_stake_unlock_with_cap)
-  [Function `withdraw`](#0x1_stake_withdraw)
-  [Function `withdraw_with_cap`](#0x1_stake_withdraw_with_cap)
-  [Function `leave_validator_set`](#0x1_stake_leave_validator_set)
-  [Function `is_current_epoch_validator`](#0x1_stake_is_current_epoch_validator)
-  [Function `update_performance_statistics`](#0x1_stake_update_performance_statistics)
-  [Function `on_new_epoch`](#0x1_stake_on_new_epoch)
-  [Function `cur_validator_consensus_infos`](#0x1_stake_cur_validator_consensus_infos)
-  [Function `next_validator_consensus_infos`](#0x1_stake_next_validator_consensus_infos)
-  [Function `validator_consensus_infos_from_validator_set`](#0x1_stake_validator_consensus_infos_from_validator_set)
-  [Function `addresses_from_validator_infos`](#0x1_stake_addresses_from_validator_infos)
-  [Function `update_stake_pool`](#0x1_stake_update_stake_pool)
-  [Function `get_reconfig_start_time_secs`](#0x1_stake_get_reconfig_start_time_secs)
-  [Function `calculate_rewards_amount`](#0x1_stake_calculate_rewards_amount)
-  [Function `distribute_rewards`](#0x1_stake_distribute_rewards)
-  [Function `append`](#0x1_stake_append)
-  [Function `find_validator`](#0x1_stake_find_validator)
-  [Function `generate_validator_info`](#0x1_stake_generate_validator_info)
-  [Function `get_next_epoch_voting_power`](#0x1_stake_get_next_epoch_voting_power)
-  [Function `update_voting_power_increase`](#0x1_stake_update_voting_power_increase)
-  [Function `assert_stake_pool_exists`](#0x1_stake_assert_stake_pool_exists)
-  [Function `configure_allowed_validators`](#0x1_stake_configure_allowed_validators)
-  [Function `is_allowed`](#0x1_stake_is_allowed)
-  [Function `assert_owner_cap_exists`](#0x1_stake_assert_owner_cap_exists)
-  [Function `assert_reconfig_not_in_progress`](#0x1_stake_assert_reconfig_not_in_progress)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Resource `ValidatorSet`](#@Specification_1_ValidatorSet)
    -  [Function `initialize_validator_fees`](#@Specification_1_initialize_validator_fees)
    -  [Function `add_transaction_fee`](#@Specification_1_add_transaction_fee)
    -  [Function `get_validator_state`](#@Specification_1_get_validator_state)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `remove_validators`](#@Specification_1_remove_validators)
    -  [Function `initialize_stake_owner`](#@Specification_1_initialize_stake_owner)
    -  [Function `initialize_validator`](#@Specification_1_initialize_validator)
    -  [Function `extract_owner_cap`](#@Specification_1_extract_owner_cap)
    -  [Function `deposit_owner_cap`](#@Specification_1_deposit_owner_cap)
    -  [Function `set_operator_with_cap`](#@Specification_1_set_operator_with_cap)
    -  [Function `set_delegated_voter_with_cap`](#@Specification_1_set_delegated_voter_with_cap)
    -  [Function `add_stake`](#@Specification_1_add_stake)
    -  [Function `add_stake_with_cap`](#@Specification_1_add_stake_with_cap)
    -  [Function `reactivate_stake_with_cap`](#@Specification_1_reactivate_stake_with_cap)
    -  [Function `rotate_consensus_key`](#@Specification_1_rotate_consensus_key)
    -  [Function `update_network_and_fullnode_addresses`](#@Specification_1_update_network_and_fullnode_addresses)
    -  [Function `increase_lockup_with_cap`](#@Specification_1_increase_lockup_with_cap)
    -  [Function `join_validator_set`](#@Specification_1_join_validator_set)
    -  [Function `unlock_with_cap`](#@Specification_1_unlock_with_cap)
    -  [Function `withdraw`](#@Specification_1_withdraw)
    -  [Function `leave_validator_set`](#@Specification_1_leave_validator_set)
    -  [Function `is_current_epoch_validator`](#@Specification_1_is_current_epoch_validator)
    -  [Function `update_performance_statistics`](#@Specification_1_update_performance_statistics)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)
    -  [Function `next_validator_consensus_infos`](#@Specification_1_next_validator_consensus_infos)
    -  [Function `validator_consensus_infos_from_validator_set`](#@Specification_1_validator_consensus_infos_from_validator_set)
    -  [Function `update_stake_pool`](#@Specification_1_update_stake_pool)
    -  [Function `get_reconfig_start_time_secs`](#@Specification_1_get_reconfig_start_time_secs)
    -  [Function `calculate_rewards_amount`](#@Specification_1_calculate_rewards_amount)
    -  [Function `distribute_rewards`](#@Specification_1_distribute_rewards)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `find_validator`](#@Specification_1_find_validator)
    -  [Function `update_voting_power_increase`](#@Specification_1_update_voting_power_increase)
    -  [Function `assert_stake_pool_exists`](#@Specification_1_assert_stake_pool_exists)
    -  [Function `configure_allowed_validators`](#@Specification_1_configure_allowed_validators)
    -  [Function `assert_owner_cap_exists`](#@Specification_1_assert_owner_cap_exists)


```move
module 0x1::stake {
    use 0x1::account;
    use 0x1::aptos_coin;
    use 0x1::bls12381;
    use 0x1::chain_status;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::fixed_point64;
    use 0x1::math64;
    use 0x1::option;
    use 0x1::reconfiguration_state;
    use 0x1::signer;
    use 0x1::staking_config;
    use 0x1::system_addresses;
    use 0x1::table;
    use 0x1::timestamp;
    use 0x1::validator_consensus_info;
    use 0x1::vector;
}
```


<a id="0x1_stake_OwnerCapability"></a>

## Resource `OwnerCapability`

Capability that represents ownership and can be used to control the validator and the associated stake pool.
Having this be separate from the signer for the account that the validator resources are hosted at allows
modules to have control over a validator.


```move
module 0x1::stake {
    struct OwnerCapability has store, key
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


<a id="0x1_stake_StakePool"></a>

## Resource `StakePool`

Each validator has a separate StakePool resource and can provide a stake.
Changes in stake for an active validator:
1. If a validator calls add_stake, the newly added stake is moved to pending_active.
2. If validator calls unlock, their stake is moved to pending_inactive.
2. When the next epoch starts, any pending_inactive stake is moved to inactive and can be withdrawn.
Any pending_active stake is moved to active and adds to the validator&apos;s voting power.

Changes in stake for an inactive validator:
1. If a validator calls add_stake, the newly added stake is moved directly to active.
2. If validator calls unlock, their stake is moved directly to inactive.
3. When the next epoch starts, the validator can be activated if their active stake is more than the minimum.


```move
module 0x1::stake {
    struct StakePool has key
}
```


##### Fields


<dl>
<dt>
`active: coin::Coin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`inactive: coin::Coin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`pending_active: coin::Coin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`pending_inactive: coin::Coin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`locked_until_secs: u64`
</dt>
<dd>

</dd>
<dt>
`operator_address: address`
</dt>
<dd>

</dd>
<dt>
`delegated_voter: address`
</dt>
<dd>

</dd>
<dt>
`initialize_validator_events: event::EventHandle<stake::RegisterValidatorCandidateEvent>`
</dt>
<dd>

</dd>
<dt>
`set_operator_events: event::EventHandle<stake::SetOperatorEvent>`
</dt>
<dd>

</dd>
<dt>
`add_stake_events: event::EventHandle<stake::AddStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`reactivate_stake_events: event::EventHandle<stake::ReactivateStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`rotate_consensus_key_events: event::EventHandle<stake::RotateConsensusKeyEvent>`
</dt>
<dd>

</dd>
<dt>
`update_network_and_fullnode_addresses_events: event::EventHandle<stake::UpdateNetworkAndFullnodeAddressesEvent>`
</dt>
<dd>

</dd>
<dt>
`increase_lockup_events: event::EventHandle<stake::IncreaseLockupEvent>`
</dt>
<dd>

</dd>
<dt>
`join_validator_set_events: event::EventHandle<stake::JoinValidatorSetEvent>`
</dt>
<dd>

</dd>
<dt>
`distribute_rewards_events: event::EventHandle<stake::DistributeRewardsEvent>`
</dt>
<dd>

</dd>
<dt>
`unlock_stake_events: event::EventHandle<stake::UnlockStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`withdraw_stake_events: event::EventHandle<stake::WithdrawStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`leave_validator_set_events: event::EventHandle<stake::LeaveValidatorSetEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ValidatorConfig"></a>

## Resource `ValidatorConfig`

Validator info stored in validator address.


```move
module 0x1::stake {
    struct ValidatorConfig has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`fullnode_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`validator_index: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ValidatorInfo"></a>

## Struct `ValidatorInfo`

Consensus information per validator, stored in ValidatorSet.


```move
module 0x1::stake {
    struct ValidatorInfo has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`addr: address`
</dt>
<dd>

</dd>
<dt>
`voting_power: u64`
</dt>
<dd>

</dd>
<dt>
`config: stake::ValidatorConfig`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ValidatorSet"></a>

## Resource `ValidatorSet`

Full ValidatorSet, stored in @aptos_framework.
1. join_validator_set adds to pending_active queue.
2. leave_valdiator_set moves from active to pending_inactive queue.
3. on_new_epoch processes two pending queues and refresh ValidatorInfo from the owner&apos;s address.


```move
module 0x1::stake {
    struct ValidatorSet has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`consensus_scheme: u8`
</dt>
<dd>

</dd>
<dt>
`active_validators: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`pending_inactive: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`pending_active: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`total_voting_power: u128`
</dt>
<dd>

</dd>
<dt>
`total_joining_power: u128`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_AptosCoinCapabilities"></a>

## Resource `AptosCoinCapabilities`

AptosCoin capabilities, set during genesis and stored in @CoreResource account.
This allows the Stake module to mint rewards to stakers.


```move
module 0x1::stake {
    struct AptosCoinCapabilities has key
}
```


##### Fields


<dl>
<dt>
`mint_cap: coin::MintCapability<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_IndividualValidatorPerformance"></a>

## Struct `IndividualValidatorPerformance`



```move
module 0x1::stake {
    struct IndividualValidatorPerformance has drop, store
}
```


##### Fields


<dl>
<dt>
`successful_proposals: u64`
</dt>
<dd>

</dd>
<dt>
`failed_proposals: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ValidatorPerformance"></a>

## Resource `ValidatorPerformance`



```move
module 0x1::stake {
    struct ValidatorPerformance has key
}
```


##### Fields


<dl>
<dt>
`validators: vector<stake::IndividualValidatorPerformance>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_RegisterValidatorCandidateEvent"></a>

## Struct `RegisterValidatorCandidateEvent`



```move
module 0x1::stake {
    struct RegisterValidatorCandidateEvent has drop, store
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


<a id="0x1_stake_RegisterValidatorCandidate"></a>

## Struct `RegisterValidatorCandidate`



```move
module 0x1::stake {
    #[event]
    struct RegisterValidatorCandidate has drop, store
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


<a id="0x1_stake_SetOperatorEvent"></a>

## Struct `SetOperatorEvent`



```move
module 0x1::stake {
    struct SetOperatorEvent has drop, store
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
`old_operator: address`
</dt>
<dd>

</dd>
<dt>
`new_operator: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_SetOperator"></a>

## Struct `SetOperator`



```move
module 0x1::stake {
    #[event]
    struct SetOperator has drop, store
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
`old_operator: address`
</dt>
<dd>

</dd>
<dt>
`new_operator: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_AddStakeEvent"></a>

## Struct `AddStakeEvent`



```move
module 0x1::stake {
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
`amount_added: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_AddStake"></a>

## Struct `AddStake`



```move
module 0x1::stake {
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
`amount_added: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ReactivateStakeEvent"></a>

## Struct `ReactivateStakeEvent`



```move
module 0x1::stake {
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
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_ReactivateStake"></a>

## Struct `ReactivateStake`



```move
module 0x1::stake {
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
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_RotateConsensusKeyEvent"></a>

## Struct `RotateConsensusKeyEvent`



```move
module 0x1::stake {
    struct RotateConsensusKeyEvent has drop, store
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
`old_consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_RotateConsensusKey"></a>

## Struct `RotateConsensusKey`



```move
module 0x1::stake {
    #[event]
    struct RotateConsensusKey has drop, store
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
`old_consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_UpdateNetworkAndFullnodeAddressesEvent"></a>

## Struct `UpdateNetworkAndFullnodeAddressesEvent`



```move
module 0x1::stake {
    struct UpdateNetworkAndFullnodeAddressesEvent has drop, store
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
`old_network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`old_fullnode_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_fullnode_addresses: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_UpdateNetworkAndFullnodeAddresses"></a>

## Struct `UpdateNetworkAndFullnodeAddresses`



```move
module 0x1::stake {
    #[event]
    struct UpdateNetworkAndFullnodeAddresses has drop, store
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
`old_network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`old_fullnode_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`new_fullnode_addresses: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_IncreaseLockupEvent"></a>

## Struct `IncreaseLockupEvent`



```move
module 0x1::stake {
    struct IncreaseLockupEvent has drop, store
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
`old_locked_until_secs: u64`
</dt>
<dd>

</dd>
<dt>
`new_locked_until_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_IncreaseLockup"></a>

## Struct `IncreaseLockup`



```move
module 0x1::stake {
    #[event]
    struct IncreaseLockup has drop, store
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
`old_locked_until_secs: u64`
</dt>
<dd>

</dd>
<dt>
`new_locked_until_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_JoinValidatorSetEvent"></a>

## Struct `JoinValidatorSetEvent`



```move
module 0x1::stake {
    struct JoinValidatorSetEvent has drop, store
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


<a id="0x1_stake_JoinValidatorSet"></a>

## Struct `JoinValidatorSet`



```move
module 0x1::stake {
    #[event]
    struct JoinValidatorSet has drop, store
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


<a id="0x1_stake_DistributeRewardsEvent"></a>

## Struct `DistributeRewardsEvent`



```move
module 0x1::stake {
    struct DistributeRewardsEvent has drop, store
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
`rewards_amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_DistributeRewards"></a>

## Struct `DistributeRewards`



```move
module 0x1::stake {
    #[event]
    struct DistributeRewards has drop, store
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
`rewards_amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_UnlockStakeEvent"></a>

## Struct `UnlockStakeEvent`



```move
module 0x1::stake {
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
`amount_unlocked: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_UnlockStake"></a>

## Struct `UnlockStake`



```move
module 0x1::stake {
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
`amount_unlocked: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_WithdrawStakeEvent"></a>

## Struct `WithdrawStakeEvent`



```move
module 0x1::stake {
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
`amount_withdrawn: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_WithdrawStake"></a>

## Struct `WithdrawStake`



```move
module 0x1::stake {
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
`amount_withdrawn: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_LeaveValidatorSetEvent"></a>

## Struct `LeaveValidatorSetEvent`



```move
module 0x1::stake {
    struct LeaveValidatorSetEvent has drop, store
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


<a id="0x1_stake_LeaveValidatorSet"></a>

## Struct `LeaveValidatorSet`



```move
module 0x1::stake {
    #[event]
    struct LeaveValidatorSet has drop, store
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


<a id="0x1_stake_ValidatorFees"></a>

## Resource `ValidatorFees`

Stores transaction fees assigned to validators. All fees are distributed to validators
at the end of the epoch.


```move
module 0x1::stake {
    struct ValidatorFees has key
}
```


##### Fields


<dl>
<dt>
`fees_table: table::Table<address, coin::Coin<aptos_coin::AptosCoin>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_AllowedValidators"></a>

## Resource `AllowedValidators`

This provides an ACL for Testnet purposes. In testnet, everyone is a whale, a whale can be a validator.
This allows a testnet to bring additional entities into the validator set without compromising the
security of the testnet. This will NOT be enabled in Mainnet.


```move
module 0x1::stake {
    struct AllowedValidators has key
}
```


##### Fields


<dl>
<dt>
`accounts: vector<address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_Ghost$ghost_valid_perf"></a>

## Resource `Ghost$ghost_valid_perf`



```move
module 0x1::stake {
    struct Ghost$ghost_valid_perf has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: stake::ValidatorPerformance`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_Ghost$ghost_proposer_idx"></a>

## Resource `Ghost$ghost_proposer_idx`



```move
module 0x1::stake {
    struct Ghost$ghost_proposer_idx has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: option::Option<u64>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_Ghost$ghost_active_num"></a>

## Resource `Ghost$ghost_active_num`



```move
module 0x1::stake {
    struct Ghost$ghost_active_num has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stake_Ghost$ghost_pending_inactive_num"></a>

## Resource `Ghost$ghost_pending_inactive_num`



```move
module 0x1::stake {
    struct Ghost$ghost_pending_inactive_num has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_stake_MAX_U64"></a>



```move
module 0x1::stake {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_stake_EALREADY_REGISTERED"></a>

Account is already registered as a validator candidate.


```move
module 0x1::stake {
    const EALREADY_REGISTERED: u64 = 8;
}
```


<a id="0x1_stake_MAX_REWARDS_RATE"></a>

Limit the maximum value of `rewards_rate` in order to avoid any arithmetic overflow.


```move
module 0x1::stake {
    const MAX_REWARDS_RATE: u64 = 1000000;
}
```


<a id="0x1_stake_EALREADY_ACTIVE_VALIDATOR"></a>

Account is already a validator or pending validator.


```move
module 0x1::stake {
    const EALREADY_ACTIVE_VALIDATOR: u64 = 4;
}
```


<a id="0x1_stake_EFEES_TABLE_ALREADY_EXISTS"></a>

Table to store collected transaction fees for each validator already exists.


```move
module 0x1::stake {
    const EFEES_TABLE_ALREADY_EXISTS: u64 = 19;
}
```


<a id="0x1_stake_EINELIGIBLE_VALIDATOR"></a>

Validator is not defined in the ACL of entities allowed to be validators


```move
module 0x1::stake {
    const EINELIGIBLE_VALIDATOR: u64 = 17;
}
```


<a id="0x1_stake_EINVALID_LOCKUP"></a>

Cannot update stake pool&apos;s lockup to earlier than current lockup.


```move
module 0x1::stake {
    const EINVALID_LOCKUP: u64 = 18;
}
```


<a id="0x1_stake_EINVALID_PUBLIC_KEY"></a>

Invalid consensus public key


```move
module 0x1::stake {
    const EINVALID_PUBLIC_KEY: u64 = 11;
}
```


<a id="0x1_stake_ELAST_VALIDATOR"></a>

Can&apos;t remove last validator.


```move
module 0x1::stake {
    const ELAST_VALIDATOR: u64 = 6;
}
```


<a id="0x1_stake_ENOT_OPERATOR"></a>

Account does not have the right operator capability.


```move
module 0x1::stake {
    const ENOT_OPERATOR: u64 = 9;
}
```


<a id="0x1_stake_ENOT_VALIDATOR"></a>

Account is not a validator.


```move
module 0x1::stake {
    const ENOT_VALIDATOR: u64 = 5;
}
```


<a id="0x1_stake_ENO_POST_GENESIS_VALIDATOR_SET_CHANGE_ALLOWED"></a>

Validators cannot join or leave post genesis on this test network.


```move
module 0x1::stake {
    const ENO_POST_GENESIS_VALIDATOR_SET_CHANGE_ALLOWED: u64 = 10;
}
```


<a id="0x1_stake_EOWNER_CAP_ALREADY_EXISTS"></a>

An account cannot own more than one owner capability.


```move
module 0x1::stake {
    const EOWNER_CAP_ALREADY_EXISTS: u64 = 16;
}
```


<a id="0x1_stake_EOWNER_CAP_NOT_FOUND"></a>

Owner capability does not exist at the provided account.


```move
module 0x1::stake {
    const EOWNER_CAP_NOT_FOUND: u64 = 15;
}
```


<a id="0x1_stake_ERECONFIGURATION_IN_PROGRESS"></a>

Validator set change temporarily disabled because of in&#45;progress reconfiguration.


```move
module 0x1::stake {
    const ERECONFIGURATION_IN_PROGRESS: u64 = 20;
}
```


<a id="0x1_stake_ESTAKE_EXCEEDS_MAX"></a>

Total stake exceeds maximum allowed.


```move
module 0x1::stake {
    const ESTAKE_EXCEEDS_MAX: u64 = 7;
}
```


<a id="0x1_stake_ESTAKE_POOL_DOES_NOT_EXIST"></a>

Stake pool does not exist at the provided pool address.


```move
module 0x1::stake {
    const ESTAKE_POOL_DOES_NOT_EXIST: u64 = 14;
}
```


<a id="0x1_stake_ESTAKE_TOO_HIGH"></a>

Too much stake to join validator set.


```move
module 0x1::stake {
    const ESTAKE_TOO_HIGH: u64 = 3;
}
```


<a id="0x1_stake_ESTAKE_TOO_LOW"></a>

Not enough stake to join validator set.


```move
module 0x1::stake {
    const ESTAKE_TOO_LOW: u64 = 2;
}
```


<a id="0x1_stake_EVALIDATOR_CONFIG"></a>

Validator Config not published.


```move
module 0x1::stake {
    const EVALIDATOR_CONFIG: u64 = 1;
}
```


<a id="0x1_stake_EVALIDATOR_SET_TOO_LARGE"></a>

Validator set exceeds the limit


```move
module 0x1::stake {
    const EVALIDATOR_SET_TOO_LARGE: u64 = 12;
}
```


<a id="0x1_stake_EVOTING_POWER_INCREASE_EXCEEDS_LIMIT"></a>

Voting power increase has exceeded the limit for this current epoch.


```move
module 0x1::stake {
    const EVOTING_POWER_INCREASE_EXCEEDS_LIMIT: u64 = 13;
}
```


<a id="0x1_stake_MAX_VALIDATOR_SET_SIZE"></a>

Limit the maximum size to u16::max, it&apos;s the current limit of the bitvec
https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/crates/aptos&#45;bitvec/src/lib.rs#L20


```move
module 0x1::stake {
    const MAX_VALIDATOR_SET_SIZE: u64 = 65536;
}
```


<a id="0x1_stake_VALIDATOR_STATUS_ACTIVE"></a>



```move
module 0x1::stake {
    const VALIDATOR_STATUS_ACTIVE: u64 = 2;
}
```


<a id="0x1_stake_VALIDATOR_STATUS_INACTIVE"></a>



```move
module 0x1::stake {
    const VALIDATOR_STATUS_INACTIVE: u64 = 4;
}
```


<a id="0x1_stake_VALIDATOR_STATUS_PENDING_ACTIVE"></a>

Validator status enum. We can switch to proper enum later once Move supports it.


```move
module 0x1::stake {
    const VALIDATOR_STATUS_PENDING_ACTIVE: u64 = 1;
}
```


<a id="0x1_stake_VALIDATOR_STATUS_PENDING_INACTIVE"></a>



```move
module 0x1::stake {
    const VALIDATOR_STATUS_PENDING_INACTIVE: u64 = 3;
}
```


<a id="0x1_stake_initialize_validator_fees"></a>

## Function `initialize_validator_fees`

Initializes the resource storing information about collected transaction fees per validator.
Used by `transaction_fee.move` to initialize fee collection and distribution.


```move
module 0x1::stake {
    public(friend) fun initialize_validator_fees(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun initialize_validator_fees(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            !exists<ValidatorFees>(@aptos_framework),
            error::already_exists(EFEES_TABLE_ALREADY_EXISTS)
        );
        move_to(aptos_framework, ValidatorFees { fees_table: table::new() });
    }
}
```


<a id="0x1_stake_add_transaction_fee"></a>

## Function `add_transaction_fee`

Stores the transaction fee collected to the specified validator address.


```move
module 0x1::stake {
    public(friend) fun add_transaction_fee(validator_addr: address, fee: coin::Coin<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun add_transaction_fee(validator_addr: address, fee: Coin<AptosCoin>) acquires ValidatorFees {
        let fees_table = &mut borrow_global_mut<ValidatorFees>(@aptos_framework).fees_table;
        if (table::contains(fees_table, validator_addr)) {
            let collected_fee = table::borrow_mut(fees_table, validator_addr);
            coin::merge(collected_fee, fee);
        } else {
            table::add(fees_table, validator_addr, fee);
        }
    }
}
```


<a id="0x1_stake_get_lockup_secs"></a>

## Function `get_lockup_secs`

Return the lockup expiration of the stake pool at `pool_address`.
This will throw an error if there&apos;s no stake pool at `pool_address`.


```move
module 0x1::stake {
    #[view]
    public fun get_lockup_secs(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_lockup_secs(pool_address: address): u64 acquires StakePool {
        assert_stake_pool_exists(pool_address);
        borrow_global<StakePool>(pool_address).locked_until_secs
    }
}
```


<a id="0x1_stake_get_remaining_lockup_secs"></a>

## Function `get_remaining_lockup_secs`

Return the remaining lockup of the stake pool at `pool_address`.
This will throw an error if there&apos;s no stake pool at `pool_address`.


```move
module 0x1::stake {
    #[view]
    public fun get_remaining_lockup_secs(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_remaining_lockup_secs(pool_address: address): u64 acquires StakePool {
        assert_stake_pool_exists(pool_address);
        let lockup_time = borrow_global<StakePool>(pool_address).locked_until_secs;
        if (lockup_time <= timestamp::now_seconds()) {
            0
        } else {
            lockup_time - timestamp::now_seconds()
        }
    }
}
```


<a id="0x1_stake_get_stake"></a>

## Function `get_stake`

Return the different stake amounts for `pool_address` (whether the validator is active or not).
The returned amounts are for (active, inactive, pending_active, pending_inactive) stake respectively.


```move
module 0x1::stake {
    #[view]
    public fun get_stake(pool_address: address): (u64, u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_stake(pool_address: address): (u64, u64, u64, u64) acquires StakePool {
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global<StakePool>(pool_address);
        (
            coin::value(&stake_pool.active),
            coin::value(&stake_pool.inactive),
            coin::value(&stake_pool.pending_active),
            coin::value(&stake_pool.pending_inactive),
        )
    }
}
```


<a id="0x1_stake_get_validator_state"></a>

## Function `get_validator_state`

Returns the validator&apos;s state.


```move
module 0x1::stake {
    #[view]
    public fun get_validator_state(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_validator_state(pool_address: address): u64 acquires ValidatorSet {
        let validator_set = borrow_global<ValidatorSet>(@aptos_framework);
        if (option::is_some(&find_validator(&validator_set.pending_active, pool_address))) {
            VALIDATOR_STATUS_PENDING_ACTIVE
        } else if (option::is_some(&find_validator(&validator_set.active_validators, pool_address))) {
            VALIDATOR_STATUS_ACTIVE
        } else if (option::is_some(&find_validator(&validator_set.pending_inactive, pool_address))) {
            VALIDATOR_STATUS_PENDING_INACTIVE
        } else {
            VALIDATOR_STATUS_INACTIVE
        }
    }
}
```


<a id="0x1_stake_get_current_epoch_voting_power"></a>

## Function `get_current_epoch_voting_power`

Return the voting power of the validator in the current epoch.
This is the same as the validator&apos;s total active and pending_inactive stake.


```move
module 0x1::stake {
    #[view]
    public fun get_current_epoch_voting_power(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_current_epoch_voting_power(pool_address: address): u64 acquires StakePool, ValidatorSet {
        assert_stake_pool_exists(pool_address);
        let validator_state = get_validator_state(pool_address);
        // Both active and pending inactive validators can still vote in the current epoch.
        if (validator_state == VALIDATOR_STATUS_ACTIVE || validator_state == VALIDATOR_STATUS_PENDING_INACTIVE) {
            let active_stake = coin::value(&borrow_global<StakePool>(pool_address).active);
            let pending_inactive_stake = coin::value(&borrow_global<StakePool>(pool_address).pending_inactive);
            active_stake + pending_inactive_stake
        } else {
            0
        }
    }
}
```


<a id="0x1_stake_get_delegated_voter"></a>

## Function `get_delegated_voter`

Return the delegated voter of the validator at `pool_address`.


```move
module 0x1::stake {
    #[view]
    public fun get_delegated_voter(pool_address: address): address
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_delegated_voter(pool_address: address): address acquires StakePool {
        assert_stake_pool_exists(pool_address);
        borrow_global<StakePool>(pool_address).delegated_voter
    }
}
```


<a id="0x1_stake_get_operator"></a>

## Function `get_operator`

Return the operator of the validator at `pool_address`.


```move
module 0x1::stake {
    #[view]
    public fun get_operator(pool_address: address): address
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_operator(pool_address: address): address acquires StakePool {
        assert_stake_pool_exists(pool_address);
        borrow_global<StakePool>(pool_address).operator_address
    }
}
```


<a id="0x1_stake_get_owned_pool_address"></a>

## Function `get_owned_pool_address`

Return the pool address in `owner_cap`.


```move
module 0x1::stake {
    public fun get_owned_pool_address(owner_cap: &stake::OwnerCapability): address
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_owned_pool_address(owner_cap: &OwnerCapability): address {
        owner_cap.pool_address
    }
}
```


<a id="0x1_stake_get_validator_index"></a>

## Function `get_validator_index`

Return the validator index for `pool_address`.


```move
module 0x1::stake {
    #[view]
    public fun get_validator_index(pool_address: address): u64
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_validator_index(pool_address: address): u64 acquires ValidatorConfig {
        assert_stake_pool_exists(pool_address);
        borrow_global<ValidatorConfig>(pool_address).validator_index
    }
}
```


<a id="0x1_stake_get_current_epoch_proposal_counts"></a>

## Function `get_current_epoch_proposal_counts`

Return the number of successful and failed proposals for the proposal at the given validator index.


```move
module 0x1::stake {
    #[view]
    public fun get_current_epoch_proposal_counts(validator_index: u64): (u64, u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_current_epoch_proposal_counts(validator_index: u64): (u64, u64) acquires ValidatorPerformance {
        let validator_performances = &borrow_global<ValidatorPerformance>(@aptos_framework).validators;
        let validator_performance = vector::borrow(validator_performances, validator_index);
        (validator_performance.successful_proposals, validator_performance.failed_proposals)
    }
}
```


<a id="0x1_stake_get_validator_config"></a>

## Function `get_validator_config`

Return the validator&apos;s config.


```move
module 0x1::stake {
    #[view]
    public fun get_validator_config(pool_address: address): (vector<u8>, vector<u8>, vector<u8>)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun get_validator_config(
        pool_address: address
    ): (vector<u8>, vector<u8>, vector<u8>) acquires ValidatorConfig {
        assert_stake_pool_exists(pool_address);
        let validator_config = borrow_global<ValidatorConfig>(pool_address);
        (validator_config.consensus_pubkey, validator_config.network_addresses, validator_config.fullnode_addresses)
    }
}
```


<a id="0x1_stake_stake_pool_exists"></a>

## Function `stake_pool_exists`



```move
module 0x1::stake {
    #[view]
    public fun stake_pool_exists(addr: address): bool
}
```


##### Implementation


```move
module 0x1::stake {
    public fun stake_pool_exists(addr: address): bool {
        exists<StakePool>(addr)
    }
}
```


<a id="0x1_stake_initialize"></a>

## Function `initialize`

Initialize validator set to the core resource account.


```move
module 0x1::stake {
    public(friend) fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);

        move_to(aptos_framework, ValidatorSet {
            consensus_scheme: 0,
            active_validators: vector::empty(),
            pending_active: vector::empty(),
            pending_inactive: vector::empty(),
            total_voting_power: 0,
            total_joining_power: 0,
        });

        move_to(aptos_framework, ValidatorPerformance {
            validators: vector::empty(),
        });
    }
}
```


<a id="0x1_stake_store_aptos_coin_mint_cap"></a>

## Function `store_aptos_coin_mint_cap`

This is only called during Genesis, which is where MintCapability&lt;AptosCoin&gt; can be created.
Beyond genesis, no one can create AptosCoin mint/burn capabilities.


```move
module 0x1::stake {
    public(friend) fun store_aptos_coin_mint_cap(aptos_framework: &signer, mint_cap: coin::MintCapability<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun store_aptos_coin_mint_cap(aptos_framework: &signer, mint_cap: MintCapability<AptosCoin>) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, AptosCoinCapabilities { mint_cap })
    }
}
```


<a id="0x1_stake_remove_validators"></a>

## Function `remove_validators`

Allow on chain governance to remove validators from the validator set.


```move
module 0x1::stake {
    public fun remove_validators(aptos_framework: &signer, validators: &vector<address>)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun remove_validators(
        aptos_framework: &signer,
        validators: &vector<address>,
    ) acquires ValidatorSet {
        assert_reconfig_not_in_progress();
        system_addresses::assert_aptos_framework(aptos_framework);
        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        let active_validators = &mut validator_set.active_validators;
        let pending_inactive = &mut validator_set.pending_inactive;
        spec {
            update ghost_active_num = len(active_validators);
            update ghost_pending_inactive_num = len(pending_inactive);
        };
        let len_validators = vector::length(validators);
        let i = 0;
        // Remove each validator from the validator set.
        while ({
            spec {
                invariant i <= len_validators;
                invariant spec_validators_are_initialized(active_validators);
                invariant spec_validator_indices_are_valid(active_validators);
                invariant spec_validators_are_initialized(pending_inactive);
                invariant spec_validator_indices_are_valid(pending_inactive);
                invariant ghost_active_num + ghost_pending_inactive_num == len(active_validators) + len(pending_inactive);
            };
            i < len_validators
        }) {
            let validator = *vector::borrow(validators, i);
            let validator_index = find_validator(active_validators, validator);
            if (option::is_some(&validator_index)) {
                let validator_info = vector::swap_remove(active_validators, *option::borrow(&validator_index));
                vector::push_back(pending_inactive, validator_info);
                spec {
                    update ghost_active_num = ghost_active_num - 1;
                    update ghost_pending_inactive_num = ghost_pending_inactive_num + 1;
                };
            };
            i = i + 1;
        };
    }
}
```


<a id="0x1_stake_initialize_stake_owner"></a>

## Function `initialize_stake_owner`

Initialize the validator account and give ownership to the signing account
except it leaves the ValidatorConfig to be set by another entity.
Note: this triggers setting the operator and owner, set it to the account&apos;s address
to set later.


```move
module 0x1::stake {
    public entry fun initialize_stake_owner(owner: &signer, initial_stake_amount: u64, operator: address, voter: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun initialize_stake_owner(
        owner: &signer,
        initial_stake_amount: u64,
        operator: address,
        voter: address,
    ) acquires AllowedValidators, OwnerCapability, StakePool, ValidatorSet {
        initialize_owner(owner);
        move_to(owner, ValidatorConfig {
            consensus_pubkey: vector::empty(),
            network_addresses: vector::empty(),
            fullnode_addresses: vector::empty(),
            validator_index: 0,
        });

        if (initial_stake_amount > 0) {
            add_stake(owner, initial_stake_amount);
        };

        let account_address = signer::address_of(owner);
        if (account_address != operator) {
            set_operator(owner, operator)
        };
        if (account_address != voter) {
            set_delegated_voter(owner, voter)
        };
    }
}
```


<a id="0x1_stake_initialize_validator"></a>

## Function `initialize_validator`

Initialize the validator account and give ownership to the signing account.


```move
module 0x1::stake {
    public entry fun initialize_validator(account: &signer, consensus_pubkey: vector<u8>, proof_of_possession: vector<u8>, network_addresses: vector<u8>, fullnode_addresses: vector<u8>)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun initialize_validator(
        account: &signer,
        consensus_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
        network_addresses: vector<u8>,
        fullnode_addresses: vector<u8>,
    ) acquires AllowedValidators {
        // Checks the public key has a valid proof-of-possession to prevent rogue-key attacks.
        let pubkey_from_pop = &mut bls12381::public_key_from_bytes_with_pop(
            consensus_pubkey,
            &proof_of_possession_from_bytes(proof_of_possession)
        );
        assert!(option::is_some(pubkey_from_pop), error::invalid_argument(EINVALID_PUBLIC_KEY));

        initialize_owner(account);
        move_to(account, ValidatorConfig {
            consensus_pubkey,
            network_addresses,
            fullnode_addresses,
            validator_index: 0,
        });
    }
}
```


<a id="0x1_stake_initialize_owner"></a>

## Function `initialize_owner`



```move
module 0x1::stake {
    fun initialize_owner(owner: &signer)
}
```


##### Implementation


```move
module 0x1::stake {
    fun initialize_owner(owner: &signer) acquires AllowedValidators {
        let owner_address = signer::address_of(owner);
        assert!(is_allowed(owner_address), error::not_found(EINELIGIBLE_VALIDATOR));
        assert!(!stake_pool_exists(owner_address), error::already_exists(EALREADY_REGISTERED));

        move_to(owner, StakePool {
            active: coin::zero<AptosCoin>(),
            pending_active: coin::zero<AptosCoin>(),
            pending_inactive: coin::zero<AptosCoin>(),
            inactive: coin::zero<AptosCoin>(),
            locked_until_secs: 0,
            operator_address: owner_address,
            delegated_voter: owner_address,
            // Events.
            initialize_validator_events: account::new_event_handle<RegisterValidatorCandidateEvent>(owner),
            set_operator_events: account::new_event_handle<SetOperatorEvent>(owner),
            add_stake_events: account::new_event_handle<AddStakeEvent>(owner),
            reactivate_stake_events: account::new_event_handle<ReactivateStakeEvent>(owner),
            rotate_consensus_key_events: account::new_event_handle<RotateConsensusKeyEvent>(owner),
            update_network_and_fullnode_addresses_events: account::new_event_handle<UpdateNetworkAndFullnodeAddressesEvent>(
                owner
            ),
            increase_lockup_events: account::new_event_handle<IncreaseLockupEvent>(owner),
            join_validator_set_events: account::new_event_handle<JoinValidatorSetEvent>(owner),
            distribute_rewards_events: account::new_event_handle<DistributeRewardsEvent>(owner),
            unlock_stake_events: account::new_event_handle<UnlockStakeEvent>(owner),
            withdraw_stake_events: account::new_event_handle<WithdrawStakeEvent>(owner),
            leave_validator_set_events: account::new_event_handle<LeaveValidatorSetEvent>(owner),
        });

        move_to(owner, OwnerCapability { pool_address: owner_address });
    }
}
```


<a id="0x1_stake_extract_owner_cap"></a>

## Function `extract_owner_cap`

Extract and return owner capability from the signing account.


```move
module 0x1::stake {
    public fun extract_owner_cap(owner: &signer): stake::OwnerCapability
}
```


##### Implementation


```move
module 0x1::stake {
    public fun extract_owner_cap(owner: &signer): OwnerCapability acquires OwnerCapability {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        move_from<OwnerCapability>(owner_address)
    }
}
```


<a id="0x1_stake_deposit_owner_cap"></a>

## Function `deposit_owner_cap`

Deposit `owner_cap` into `account`. This requires `account` to not already have ownership of another
staking pool.


```move
module 0x1::stake {
    public fun deposit_owner_cap(owner: &signer, owner_cap: stake::OwnerCapability)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun deposit_owner_cap(owner: &signer, owner_cap: OwnerCapability) {
        assert!(!exists<OwnerCapability>(signer::address_of(owner)), error::not_found(EOWNER_CAP_ALREADY_EXISTS));
        move_to(owner, owner_cap);
    }
}
```


<a id="0x1_stake_destroy_owner_cap"></a>

## Function `destroy_owner_cap`

Destroy `owner_cap`.


```move
module 0x1::stake {
    public fun destroy_owner_cap(owner_cap: stake::OwnerCapability)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun destroy_owner_cap(owner_cap: OwnerCapability) {
        let OwnerCapability { pool_address: _ } = owner_cap;
    }
}
```


<a id="0x1_stake_set_operator"></a>

## Function `set_operator`

Allows an owner to change the operator of the stake pool.


```move
module 0x1::stake {
    public entry fun set_operator(owner: &signer, new_operator: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun set_operator(owner: &signer, new_operator: address) acquires OwnerCapability, StakePool {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        set_operator_with_cap(ownership_cap, new_operator);
    }
}
```


<a id="0x1_stake_set_operator_with_cap"></a>

## Function `set_operator_with_cap`

Allows an account with ownership capability to change the operator of the stake pool.


```move
module 0x1::stake {
    public fun set_operator_with_cap(owner_cap: &stake::OwnerCapability, new_operator: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun set_operator_with_cap(owner_cap: &OwnerCapability, new_operator: address) acquires StakePool {
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        let old_operator = stake_pool.operator_address;
        stake_pool.operator_address = new_operator;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                SetOperator {
                    pool_address,
                    old_operator,
                    new_operator,
                },
            );
        };

        event::emit_event(
            &mut stake_pool.set_operator_events,
            SetOperatorEvent {
                pool_address,
                old_operator,
                new_operator,
            },
        );
    }
}
```


<a id="0x1_stake_set_delegated_voter"></a>

## Function `set_delegated_voter`

Allows an owner to change the delegated voter of the stake pool.


```move
module 0x1::stake {
    public entry fun set_delegated_voter(owner: &signer, new_voter: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun set_delegated_voter(owner: &signer, new_voter: address) acquires OwnerCapability, StakePool {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        set_delegated_voter_with_cap(ownership_cap, new_voter);
    }
}
```


<a id="0x1_stake_set_delegated_voter_with_cap"></a>

## Function `set_delegated_voter_with_cap`

Allows an owner to change the delegated voter of the stake pool.


```move
module 0x1::stake {
    public fun set_delegated_voter_with_cap(owner_cap: &stake::OwnerCapability, new_voter: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun set_delegated_voter_with_cap(owner_cap: &OwnerCapability, new_voter: address) acquires StakePool {
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        stake_pool.delegated_voter = new_voter;
    }
}
```


<a id="0x1_stake_add_stake"></a>

## Function `add_stake`

Add `amount` of coins from the `account` owning the StakePool.


```move
module 0x1::stake {
    public entry fun add_stake(owner: &signer, amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun add_stake(owner: &signer, amount: u64) acquires OwnerCapability, StakePool, ValidatorSet {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        add_stake_with_cap(ownership_cap, coin::withdraw<AptosCoin>(owner, amount));
    }
}
```


<a id="0x1_stake_add_stake_with_cap"></a>

## Function `add_stake_with_cap`

Add `coins` into `pool_address`. this requires the corresponding `owner_cap` to be passed in.


```move
module 0x1::stake {
    public fun add_stake_with_cap(owner_cap: &stake::OwnerCapability, coins: coin::Coin<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun add_stake_with_cap(owner_cap: &OwnerCapability, coins: Coin<AptosCoin>) acquires StakePool, ValidatorSet {
        assert_reconfig_not_in_progress();
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);

        let amount = coin::value(&coins);
        if (amount == 0) {
            coin::destroy_zero(coins);
            return
        };

        // Only track and validate voting power increase for active and pending_active validator.
        // Pending_inactive validator will be removed from the validator set in the next epoch.
        // Inactive validator's total stake will be tracked when they join the validator set.
        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        // Search directly rather using get_validator_state to save on unnecessary loops.
        if (option::is_some(&find_validator(&validator_set.active_validators, pool_address)) ||
            option::is_some(&find_validator(&validator_set.pending_active, pool_address))) {
            update_voting_power_increase(amount);
        };

        // Add to pending_active if it's a current validator because the stake is not counted until the next epoch.
        // Otherwise, the delegation can be added to active directly as the validator is also activated in the epoch.
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        if (is_current_epoch_validator(pool_address)) {
            coin::merge<AptosCoin>(&mut stake_pool.pending_active, coins);
        } else {
            coin::merge<AptosCoin>(&mut stake_pool.active, coins);
        };

        let (_, maximum_stake) = staking_config::get_required_stake(&staking_config::get());
        let voting_power = get_next_epoch_voting_power(stake_pool);
        assert!(voting_power <= maximum_stake, error::invalid_argument(ESTAKE_EXCEEDS_MAX));

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                AddStake {
                    pool_address,
                    amount_added: amount,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.add_stake_events,
            AddStakeEvent {
                pool_address,
                amount_added: amount,
            },
        );
    }
}
```


<a id="0x1_stake_reactivate_stake"></a>

## Function `reactivate_stake`

Move `amount` of coins from pending_inactive to active.


```move
module 0x1::stake {
    public entry fun reactivate_stake(owner: &signer, amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun reactivate_stake(owner: &signer, amount: u64) acquires OwnerCapability, StakePool {
        assert_reconfig_not_in_progress();
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        reactivate_stake_with_cap(ownership_cap, amount);
    }
}
```


<a id="0x1_stake_reactivate_stake_with_cap"></a>

## Function `reactivate_stake_with_cap`



```move
module 0x1::stake {
    public fun reactivate_stake_with_cap(owner_cap: &stake::OwnerCapability, amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun reactivate_stake_with_cap(owner_cap: &OwnerCapability, amount: u64) acquires StakePool {
        assert_reconfig_not_in_progress();
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);

        // Cap the amount to reactivate by the amount in pending_inactive.
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        let total_pending_inactive = coin::value(&stake_pool.pending_inactive);
        amount = min(amount, total_pending_inactive);

        // Since this does not count as a voting power change (pending inactive still counts as voting power in the
        // current epoch), stake can be immediately moved from pending inactive to active.
        // We also don't need to check voting power increase as there's none.
        let reactivated_coins = coin::extract(&mut stake_pool.pending_inactive, amount);
        coin::merge(&mut stake_pool.active, reactivated_coins);

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                ReactivateStake {
                    pool_address,
                    amount,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.reactivate_stake_events,
            ReactivateStakeEvent {
                pool_address,
                amount,
            },
        );
    }
}
```


<a id="0x1_stake_rotate_consensus_key"></a>

## Function `rotate_consensus_key`

Rotate the consensus key of the validator, it&apos;ll take effect in next epoch.


```move
module 0x1::stake {
    public entry fun rotate_consensus_key(operator: &signer, pool_address: address, new_consensus_pubkey: vector<u8>, proof_of_possession: vector<u8>)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun rotate_consensus_key(
        operator: &signer,
        pool_address: address,
        new_consensus_pubkey: vector<u8>,
        proof_of_possession: vector<u8>,
    ) acquires StakePool, ValidatorConfig {
        assert_reconfig_not_in_progress();
        assert_stake_pool_exists(pool_address);

        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        assert!(signer::address_of(operator) == stake_pool.operator_address, error::unauthenticated(ENOT_OPERATOR));

        assert!(exists<ValidatorConfig>(pool_address), error::not_found(EVALIDATOR_CONFIG));
        let validator_info = borrow_global_mut<ValidatorConfig>(pool_address);
        let old_consensus_pubkey = validator_info.consensus_pubkey;
        // Checks the public key has a valid proof-of-possession to prevent rogue-key attacks.
        let pubkey_from_pop = &mut bls12381::public_key_from_bytes_with_pop(
            new_consensus_pubkey,
            &proof_of_possession_from_bytes(proof_of_possession)
        );
        assert!(option::is_some(pubkey_from_pop), error::invalid_argument(EINVALID_PUBLIC_KEY));
        validator_info.consensus_pubkey = new_consensus_pubkey;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                RotateConsensusKey {
                    pool_address,
                    old_consensus_pubkey,
                    new_consensus_pubkey,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.rotate_consensus_key_events,
            RotateConsensusKeyEvent {
                pool_address,
                old_consensus_pubkey,
                new_consensus_pubkey,
            },
        );
    }
}
```


<a id="0x1_stake_update_network_and_fullnode_addresses"></a>

## Function `update_network_and_fullnode_addresses`

Update the network and full node addresses of the validator. This only takes effect in the next epoch.


```move
module 0x1::stake {
    public entry fun update_network_and_fullnode_addresses(operator: &signer, pool_address: address, new_network_addresses: vector<u8>, new_fullnode_addresses: vector<u8>)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun update_network_and_fullnode_addresses(
        operator: &signer,
        pool_address: address,
        new_network_addresses: vector<u8>,
        new_fullnode_addresses: vector<u8>,
    ) acquires StakePool, ValidatorConfig {
        assert_reconfig_not_in_progress();
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        assert!(signer::address_of(operator) == stake_pool.operator_address, error::unauthenticated(ENOT_OPERATOR));
        assert!(exists<ValidatorConfig>(pool_address), error::not_found(EVALIDATOR_CONFIG));
        let validator_info = borrow_global_mut<ValidatorConfig>(pool_address);
        let old_network_addresses = validator_info.network_addresses;
        validator_info.network_addresses = new_network_addresses;
        let old_fullnode_addresses = validator_info.fullnode_addresses;
        validator_info.fullnode_addresses = new_fullnode_addresses;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                UpdateNetworkAndFullnodeAddresses {
                    pool_address,
                    old_network_addresses,
                    new_network_addresses,
                    old_fullnode_addresses,
                    new_fullnode_addresses,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.update_network_and_fullnode_addresses_events,
            UpdateNetworkAndFullnodeAddressesEvent {
                pool_address,
                old_network_addresses,
                new_network_addresses,
                old_fullnode_addresses,
                new_fullnode_addresses,
            },
        );

    }
}
```


<a id="0x1_stake_increase_lockup"></a>

## Function `increase_lockup`

Similar to increase_lockup_with_cap but will use ownership capability from the signing account.


```move
module 0x1::stake {
    public entry fun increase_lockup(owner: &signer)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun increase_lockup(owner: &signer) acquires OwnerCapability, StakePool {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        increase_lockup_with_cap(ownership_cap);
    }
}
```


<a id="0x1_stake_increase_lockup_with_cap"></a>

## Function `increase_lockup_with_cap`

Unlock from active delegation, it&apos;s moved to pending_inactive if locked_until_secs &lt; current_time or
directly inactive if it&apos;s not from an active validator.


```move
module 0x1::stake {
    public fun increase_lockup_with_cap(owner_cap: &stake::OwnerCapability)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun increase_lockup_with_cap(owner_cap: &OwnerCapability) acquires StakePool {
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);
        let config = staking_config::get();

        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        let old_locked_until_secs = stake_pool.locked_until_secs;
        let new_locked_until_secs = timestamp::now_seconds() + staking_config::get_recurring_lockup_duration(&config);
        assert!(old_locked_until_secs < new_locked_until_secs, error::invalid_argument(EINVALID_LOCKUP));
        stake_pool.locked_until_secs = new_locked_until_secs;

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                IncreaseLockup {
                    pool_address,
                    old_locked_until_secs,
                    new_locked_until_secs,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.increase_lockup_events,
            IncreaseLockupEvent {
                pool_address,
                old_locked_until_secs,
                new_locked_until_secs,
            },
        );
    }
}
```


<a id="0x1_stake_join_validator_set"></a>

## Function `join_validator_set`

This can only called by the operator of the validator/staking pool.


```move
module 0x1::stake {
    public entry fun join_validator_set(operator: &signer, pool_address: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun join_validator_set(
        operator: &signer,
        pool_address: address
    ) acquires StakePool, ValidatorConfig, ValidatorSet {
        assert!(
            staking_config::get_allow_validator_set_change(&staking_config::get()),
            error::invalid_argument(ENO_POST_GENESIS_VALIDATOR_SET_CHANGE_ALLOWED),
        );

        join_validator_set_internal(operator, pool_address);
    }
}
```


<a id="0x1_stake_join_validator_set_internal"></a>

## Function `join_validator_set_internal`

Request to have `pool_address` join the validator set. Can only be called after calling `initialize_validator`.
If the validator has the required stake (more than minimum and less than maximum allowed), they will be
added to the pending_active queue. All validators in this queue will be added to the active set when the next
epoch starts (eligibility will be rechecked).

This internal version can only be called by the Genesis module during Genesis.


```move
module 0x1::stake {
    public(friend) fun join_validator_set_internal(operator: &signer, pool_address: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun join_validator_set_internal(
        operator: &signer,
        pool_address: address
    ) acquires StakePool, ValidatorConfig, ValidatorSet {
        assert_reconfig_not_in_progress();
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        assert!(signer::address_of(operator) == stake_pool.operator_address, error::unauthenticated(ENOT_OPERATOR));
        assert!(
            get_validator_state(pool_address) == VALIDATOR_STATUS_INACTIVE,
            error::invalid_state(EALREADY_ACTIVE_VALIDATOR),
        );

        let config = staking_config::get();
        let (minimum_stake, maximum_stake) = staking_config::get_required_stake(&config);
        let voting_power = get_next_epoch_voting_power(stake_pool);
        assert!(voting_power >= minimum_stake, error::invalid_argument(ESTAKE_TOO_LOW));
        assert!(voting_power <= maximum_stake, error::invalid_argument(ESTAKE_TOO_HIGH));

        // Track and validate voting power increase.
        update_voting_power_increase(voting_power);

        // Add validator to pending_active, to be activated in the next epoch.
        let validator_config = borrow_global_mut<ValidatorConfig>(pool_address);
        assert!(!vector::is_empty(&validator_config.consensus_pubkey), error::invalid_argument(EINVALID_PUBLIC_KEY));

        // Validate the current validator set size has not exceeded the limit.
        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        vector::push_back(
            &mut validator_set.pending_active,
            generate_validator_info(pool_address, stake_pool, *validator_config)
        );
        let validator_set_size = vector::length(&validator_set.active_validators) + vector::length(
            &validator_set.pending_active
        );
        assert!(validator_set_size <= MAX_VALIDATOR_SET_SIZE, error::invalid_argument(EVALIDATOR_SET_TOO_LARGE));

        if (std::features::module_event_migration_enabled()) {
            event::emit(JoinValidatorSet { pool_address });
        };
        event::emit_event(
            &mut stake_pool.join_validator_set_events,
            JoinValidatorSetEvent { pool_address },
        );
    }
}
```


<a id="0x1_stake_unlock"></a>

## Function `unlock`

Similar to unlock_with_cap but will use ownership capability from the signing account.


```move
module 0x1::stake {
    public entry fun unlock(owner: &signer, amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun unlock(owner: &signer, amount: u64) acquires OwnerCapability, StakePool {
        assert_reconfig_not_in_progress();
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        unlock_with_cap(amount, ownership_cap);
    }
}
```


<a id="0x1_stake_unlock_with_cap"></a>

## Function `unlock_with_cap`

Unlock `amount` from the active stake. Only possible if the lockup has expired.


```move
module 0x1::stake {
    public fun unlock_with_cap(amount: u64, owner_cap: &stake::OwnerCapability)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun unlock_with_cap(amount: u64, owner_cap: &OwnerCapability) acquires StakePool {
        assert_reconfig_not_in_progress();
        // Short-circuit if amount to unlock is 0 so we don't emit events.
        if (amount == 0) {
            return
        };

        // Unlocked coins are moved to pending_inactive. When the current lockup cycle expires, they will be moved into
        // inactive in the earliest possible epoch transition.
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        // Cap amount to unlock by maximum active stake.
        let amount = min(amount, coin::value(&stake_pool.active));
        let unlocked_stake = coin::extract(&mut stake_pool.active, amount);
        coin::merge<AptosCoin>(&mut stake_pool.pending_inactive, unlocked_stake);

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                UnlockStake {
                    pool_address,
                    amount_unlocked: amount,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.unlock_stake_events,
            UnlockStakeEvent {
                pool_address,
                amount_unlocked: amount,
            },
        );
    }
}
```


<a id="0x1_stake_withdraw"></a>

## Function `withdraw`

Withdraw from `account`&apos;s inactive stake.


```move
module 0x1::stake {
    public entry fun withdraw(owner: &signer, withdraw_amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun withdraw(
        owner: &signer,
        withdraw_amount: u64
    ) acquires OwnerCapability, StakePool, ValidatorSet {
        let owner_address = signer::address_of(owner);
        assert_owner_cap_exists(owner_address);
        let ownership_cap = borrow_global<OwnerCapability>(owner_address);
        let coins = withdraw_with_cap(ownership_cap, withdraw_amount);
        coin::deposit<AptosCoin>(owner_address, coins);
    }
}
```


<a id="0x1_stake_withdraw_with_cap"></a>

## Function `withdraw_with_cap`

Withdraw from `pool_address`&apos;s inactive stake with the corresponding `owner_cap`.


```move
module 0x1::stake {
    public fun withdraw_with_cap(owner_cap: &stake::OwnerCapability, withdraw_amount: u64): coin::Coin<aptos_coin::AptosCoin>
}
```


##### Implementation


```move
module 0x1::stake {
    public fun withdraw_with_cap(
        owner_cap: &OwnerCapability,
        withdraw_amount: u64
    ): Coin<AptosCoin> acquires StakePool, ValidatorSet {
        assert_reconfig_not_in_progress();
        let pool_address = owner_cap.pool_address;
        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        // There's an edge case where a validator unlocks their stake and leaves the validator set before
        // the stake is fully unlocked (the current lockup cycle has not expired yet).
        // This can leave their stake stuck in pending_inactive even after the current lockup cycle expires.
        if (get_validator_state(pool_address) == VALIDATOR_STATUS_INACTIVE &&
            timestamp::now_seconds() >= stake_pool.locked_until_secs) {
            let pending_inactive_stake = coin::extract_all(&mut stake_pool.pending_inactive);
            coin::merge(&mut stake_pool.inactive, pending_inactive_stake);
        };

        // Cap withdraw amount by total inactive coins.
        withdraw_amount = min(withdraw_amount, coin::value(&stake_pool.inactive));
        if (withdraw_amount == 0) return coin::zero<AptosCoin>();

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                WithdrawStake {
                    pool_address,
                    amount_withdrawn: withdraw_amount,
                },
            );
        };
        event::emit_event(
            &mut stake_pool.withdraw_stake_events,
            WithdrawStakeEvent {
                pool_address,
                amount_withdrawn: withdraw_amount,
            },
        );

        coin::extract(&mut stake_pool.inactive, withdraw_amount)
    }
}
```


<a id="0x1_stake_leave_validator_set"></a>

## Function `leave_validator_set`

Request to have `pool_address` leave the validator set. The validator is only actually removed from the set when
the next epoch starts.
The last validator in the set cannot leave. This is an edge case that should never happen as long as the network
is still operational.

Can only be called by the operator of the validator/staking pool.


```move
module 0x1::stake {
    public entry fun leave_validator_set(operator: &signer, pool_address: address)
}
```


##### Implementation


```move
module 0x1::stake {
    public entry fun leave_validator_set(
        operator: &signer,
        pool_address: address
    ) acquires StakePool, ValidatorSet {
        assert_reconfig_not_in_progress();
        let config = staking_config::get();
        assert!(
            staking_config::get_allow_validator_set_change(&config),
            error::invalid_argument(ENO_POST_GENESIS_VALIDATOR_SET_CHANGE_ALLOWED),
        );

        assert_stake_pool_exists(pool_address);
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        // Account has to be the operator.
        assert!(signer::address_of(operator) == stake_pool.operator_address, error::unauthenticated(ENOT_OPERATOR));

        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        // If the validator is still pending_active, directly kick the validator out.
        let maybe_pending_active_index = find_validator(&validator_set.pending_active, pool_address);
        if (option::is_some(&maybe_pending_active_index)) {
            vector::swap_remove(
                &mut validator_set.pending_active, option::extract(&mut maybe_pending_active_index));

            // Decrease the voting power increase as the pending validator's voting power was added when they requested
            // to join. Now that they changed their mind, their voting power should not affect the joining limit of this
            // epoch.
            let validator_stake = (get_next_epoch_voting_power(stake_pool) as u128);
            // total_joining_power should be larger than validator_stake but just in case there has been a small
            // rounding error somewhere that can lead to an underflow, we still want to allow this transaction to
            // succeed.
            if (validator_set.total_joining_power > validator_stake) {
                validator_set.total_joining_power = validator_set.total_joining_power - validator_stake;
            } else {
                validator_set.total_joining_power = 0;
            };
        } else {
            // Validate that the validator is already part of the validator set.
            let maybe_active_index = find_validator(&validator_set.active_validators, pool_address);
            assert!(option::is_some(&maybe_active_index), error::invalid_state(ENOT_VALIDATOR));
            let validator_info = vector::swap_remove(
                &mut validator_set.active_validators, option::extract(&mut maybe_active_index));
            assert!(vector::length(&validator_set.active_validators) > 0, error::invalid_state(ELAST_VALIDATOR));
            vector::push_back(&mut validator_set.pending_inactive, validator_info);

            if (std::features::module_event_migration_enabled()) {
                event::emit(LeaveValidatorSet { pool_address });
            };
            event::emit_event(
                &mut stake_pool.leave_validator_set_events,
                LeaveValidatorSetEvent {
                    pool_address,
                },
            );
        };
    }
}
```


<a id="0x1_stake_is_current_epoch_validator"></a>

## Function `is_current_epoch_validator`

Returns true if the current validator can still vote in the current epoch.
This includes validators that requested to leave but are still in the pending_inactive queue and will be removed
when the epoch starts.


```move
module 0x1::stake {
    public fun is_current_epoch_validator(pool_address: address): bool
}
```


##### Implementation


```move
module 0x1::stake {
    public fun is_current_epoch_validator(pool_address: address): bool acquires ValidatorSet {
        assert_stake_pool_exists(pool_address);
        let validator_state = get_validator_state(pool_address);
        validator_state == VALIDATOR_STATUS_ACTIVE || validator_state == VALIDATOR_STATUS_PENDING_INACTIVE
    }
}
```


<a id="0x1_stake_update_performance_statistics"></a>

## Function `update_performance_statistics`

Update the validator performance (proposal statistics). This is only called by block::prologue().
This function cannot abort.


```move
module 0x1::stake {
    public(friend) fun update_performance_statistics(proposer_index: option::Option<u64>, failed_proposer_indices: vector<u64>)
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun update_performance_statistics(
        proposer_index: Option<u64>,
        failed_proposer_indices: vector<u64>
    ) acquires ValidatorPerformance {
        // Validator set cannot change until the end of the epoch, so the validator index in arguments should
        // match with those of the validators in ValidatorPerformance resource.
        let validator_perf = borrow_global_mut<ValidatorPerformance>(@aptos_framework);
        let validator_len = vector::length(&validator_perf.validators);

        spec {
            update ghost_valid_perf = validator_perf;
            update ghost_proposer_idx = proposer_index;
        };
        // proposer_index is an option because it can be missing (for NilBlocks)
        if (option::is_some(&proposer_index)) {
            let cur_proposer_index = option::extract(&mut proposer_index);
            // Here, and in all other vector::borrow, skip any validator indices that are out of bounds,
            // this ensures that this function doesn't abort if there are out of bounds errors.
            if (cur_proposer_index < validator_len) {
                let validator = vector::borrow_mut(&mut validator_perf.validators, cur_proposer_index);
                spec {
                    assume validator.successful_proposals + 1 <= MAX_U64;
                };
                validator.successful_proposals = validator.successful_proposals + 1;
            };
        };

        let f = 0;
        let f_len = vector::length(&failed_proposer_indices);
        while ({
            spec {
                invariant len(validator_perf.validators) == validator_len;
                invariant (option::spec_is_some(ghost_proposer_idx) && option::spec_borrow(
                    ghost_proposer_idx
                ) < validator_len) ==>
                    (validator_perf.validators[option::spec_borrow(ghost_proposer_idx)].successful_proposals ==
                        ghost_valid_perf.validators[option::spec_borrow(ghost_proposer_idx)].successful_proposals + 1);
            };
            f < f_len
        }) {
            let validator_index = *vector::borrow(&failed_proposer_indices, f);
            if (validator_index < validator_len) {
                let validator = vector::borrow_mut(&mut validator_perf.validators, validator_index);
                spec {
                    assume validator.failed_proposals + 1 <= MAX_U64;
                };
                validator.failed_proposals = validator.failed_proposals + 1;
            };
            f = f + 1;
        };
    }
}
```


<a id="0x1_stake_on_new_epoch"></a>

## Function `on_new_epoch`

Triggered during a reconfiguration. This function shouldn&apos;t abort.

1. Distribute transaction fees and rewards to stake pools of active and pending inactive validators (requested
to leave but not yet removed).
2. Officially move pending active stake to active and move pending inactive stake to inactive.
The staking pool&apos;s voting power in this new epoch will be updated to the total active stake.
3. Add pending active validators to the active set if they satisfy requirements so they can vote and remove
pending inactive validators so they no longer can vote.
4. The validator&apos;s voting power in the validator set is updated to be the corresponding staking pool&apos;s voting
power.


```move
module 0x1::stake {
    public(friend) fun on_new_epoch()
}
```


##### Implementation


```move
module 0x1::stake {
    public(friend) fun on_new_epoch(
    ) acquires StakePool, AptosCoinCapabilities, ValidatorConfig, ValidatorPerformance, ValidatorSet, ValidatorFees {
        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        let config = staking_config::get();
        let validator_perf = borrow_global_mut<ValidatorPerformance>(@aptos_framework);

        // Process pending stake and distribute transaction fees and rewards for each currently active validator.
        vector::for_each_ref(&validator_set.active_validators, |validator| {
            let validator: &ValidatorInfo = validator;
            update_stake_pool(validator_perf, validator.addr, &config);
        });

        // Process pending stake and distribute transaction fees and rewards for each currently pending_inactive validator
        // (requested to leave but not removed yet).
        vector::for_each_ref(&validator_set.pending_inactive, |validator| {
            let validator: &ValidatorInfo = validator;
            update_stake_pool(validator_perf, validator.addr, &config);
        });

        // Activate currently pending_active validators.
        append(&mut validator_set.active_validators, &mut validator_set.pending_active);

        // Officially deactivate all pending_inactive validators. They will now no longer receive rewards.
        validator_set.pending_inactive = vector::empty();

        // Update active validator set so that network address/public key change takes effect.
        // Moreover, recalculate the total voting power, and deactivate the validator whose
        // voting power is less than the minimum required stake.
        let next_epoch_validators = vector::empty();
        let (minimum_stake, _) = staking_config::get_required_stake(&config);
        let vlen = vector::length(&validator_set.active_validators);
        let total_voting_power = 0;
        let i = 0;
        while ({
            spec {
                invariant spec_validators_are_initialized(next_epoch_validators);
                invariant i <= vlen;
            };
            i < vlen
        }) {
            let old_validator_info = vector::borrow_mut(&mut validator_set.active_validators, i);
            let pool_address = old_validator_info.addr;
            let validator_config = borrow_global_mut<ValidatorConfig>(pool_address);
            let stake_pool = borrow_global_mut<StakePool>(pool_address);
            let new_validator_info = generate_validator_info(pool_address, stake_pool, *validator_config);

            // A validator needs at least the min stake required to join the validator set.
            if (new_validator_info.voting_power >= minimum_stake) {
                spec {
                    assume total_voting_power + new_validator_info.voting_power <= MAX_U128;
                };
                total_voting_power = total_voting_power + (new_validator_info.voting_power as u128);
                vector::push_back(&mut next_epoch_validators, new_validator_info);
            };
            i = i + 1;
        };

        validator_set.active_validators = next_epoch_validators;
        validator_set.total_voting_power = total_voting_power;
        validator_set.total_joining_power = 0;

        // Update validator indices, reset performance scores, and renew lockups.
        validator_perf.validators = vector::empty();
        let recurring_lockup_duration_secs = staking_config::get_recurring_lockup_duration(&config);
        let vlen = vector::length(&validator_set.active_validators);
        let validator_index = 0;
        while ({
            spec {
                invariant spec_validators_are_initialized(validator_set.active_validators);
                invariant len(validator_set.pending_active) == 0;
                invariant len(validator_set.pending_inactive) == 0;
                invariant 0 <= validator_index && validator_index <= vlen;
                invariant vlen == len(validator_set.active_validators);
                invariant forall i in 0..validator_index:
                    global<ValidatorConfig>(validator_set.active_validators[i].addr).validator_index < validator_index;
                invariant forall i in 0..validator_index:
                    validator_set.active_validators[i].config.validator_index < validator_index;
                invariant len(validator_perf.validators) == validator_index;
            };
            validator_index < vlen
        }) {
            let validator_info = vector::borrow_mut(&mut validator_set.active_validators, validator_index);
            validator_info.config.validator_index = validator_index;
            let validator_config = borrow_global_mut<ValidatorConfig>(validator_info.addr);
            validator_config.validator_index = validator_index;

            vector::push_back(&mut validator_perf.validators, IndividualValidatorPerformance {
                successful_proposals: 0,
                failed_proposals: 0,
            });

            // Automatically renew a validator's lockup for validators that will still be in the validator set in the
            // next epoch.
            let stake_pool = borrow_global_mut<StakePool>(validator_info.addr);
            let now_secs = timestamp::now_seconds();
            let reconfig_start_secs = if (chain_status::is_operating()) {
                get_reconfig_start_time_secs()
            } else {
                now_secs
            };
            if (stake_pool.locked_until_secs <= reconfig_start_secs) {
                spec {
                    assume now_secs + recurring_lockup_duration_secs <= MAX_U64;
                };
                stake_pool.locked_until_secs = now_secs + recurring_lockup_duration_secs;
            };

            validator_index = validator_index + 1;
        };

        if (features::periodical_reward_rate_decrease_enabled()) {
            // Update rewards rate after reward distribution.
            staking_config::calculate_and_save_latest_epoch_rewards_rate();
        };
    }
}
```


<a id="0x1_stake_cur_validator_consensus_infos"></a>

## Function `cur_validator_consensus_infos`

Return the `ValidatorConsensusInfo` of each current validator, sorted by current validator index.


```move
module 0x1::stake {
    public fun cur_validator_consensus_infos(): vector<validator_consensus_info::ValidatorConsensusInfo>
}
```


##### Implementation


```move
module 0x1::stake {
    public fun cur_validator_consensus_infos(): vector<ValidatorConsensusInfo> acquires ValidatorSet {
        let validator_set = borrow_global<ValidatorSet>(@aptos_framework);
        validator_consensus_infos_from_validator_set(validator_set)
    }
}
```


<a id="0x1_stake_next_validator_consensus_infos"></a>

## Function `next_validator_consensus_infos`



```move
module 0x1::stake {
    public fun next_validator_consensus_infos(): vector<validator_consensus_info::ValidatorConsensusInfo>
}
```


##### Implementation


```move
module 0x1::stake {
    public fun next_validator_consensus_infos(): vector<ValidatorConsensusInfo> acquires ValidatorSet, ValidatorPerformance, StakePool, ValidatorFees, ValidatorConfig {
        // Init.
        let cur_validator_set = borrow_global<ValidatorSet>(@aptos_framework);
        let staking_config = staking_config::get();
        let validator_perf = borrow_global<ValidatorPerformance>(@aptos_framework);
        let (minimum_stake, _) = staking_config::get_required_stake(&staking_config);
        let (rewards_rate, rewards_rate_denominator) = staking_config::get_reward_rate(&staking_config);

        // Compute new validator set.
        let new_active_validators = vector[];
        let num_new_actives = 0;
        let candidate_idx = 0;
        let new_total_power = 0;
        let num_cur_actives = vector::length(&cur_validator_set.active_validators);
        let num_cur_pending_actives = vector::length(&cur_validator_set.pending_active);
        spec {
            assume num_cur_actives + num_cur_pending_actives <= MAX_U64;
        };
        let num_candidates = num_cur_actives + num_cur_pending_actives;
        while ({
            spec {
                invariant candidate_idx <= num_candidates;
                invariant spec_validators_are_initialized(new_active_validators);
                invariant len(new_active_validators) == num_new_actives;
                invariant forall i in 0..len(new_active_validators):
                    new_active_validators[i].config.validator_index == i;
                invariant num_new_actives <= candidate_idx;
                invariant spec_validators_are_initialized(new_active_validators);
            };
            candidate_idx < num_candidates
        }) {
            let candidate_in_current_validator_set = candidate_idx < num_cur_actives;
            let candidate = if (candidate_idx < num_cur_actives) {
                vector::borrow(&cur_validator_set.active_validators, candidate_idx)
            } else {
                vector::borrow(&cur_validator_set.pending_active, candidate_idx - num_cur_actives)
            };
            let stake_pool = borrow_global<StakePool>(candidate.addr);
            let cur_active = coin::value(&stake_pool.active);
            let cur_pending_active = coin::value(&stake_pool.pending_active);
            let cur_pending_inactive = coin::value(&stake_pool.pending_inactive);

            let cur_reward = if (candidate_in_current_validator_set && cur_active > 0) {
                spec {
                    assert candidate.config.validator_index < len(validator_perf.validators);
                };
                let cur_perf = vector::borrow(&validator_perf.validators, candidate.config.validator_index);
                spec {
                    assume cur_perf.successful_proposals + cur_perf.failed_proposals <= MAX_U64;
                };
                calculate_rewards_amount(cur_active, cur_perf.successful_proposals, cur_perf.successful_proposals + cur_perf.failed_proposals, rewards_rate, rewards_rate_denominator)
            } else {
                0
            };

            let cur_fee = 0;
            if (features::collect_and_distribute_gas_fees()) {
                let fees_table = &borrow_global<ValidatorFees>(@aptos_framework).fees_table;
                if (table::contains(fees_table, candidate.addr)) {
                    let fee_coin = table::borrow(fees_table, candidate.addr);
                    cur_fee = coin::value(fee_coin);
                }
            };

            let lockup_expired = get_reconfig_start_time_secs() >= stake_pool.locked_until_secs;
            spec {
                assume cur_active + cur_pending_active + cur_reward + cur_fee <= MAX_U64;
                assume cur_active + cur_pending_inactive + cur_pending_active + cur_reward + cur_fee <= MAX_U64;
            };
            let new_voting_power =
                cur_active
                + if (lockup_expired) { 0 } else { cur_pending_inactive }
                + cur_pending_active
                + cur_reward + cur_fee;

            if (new_voting_power >= minimum_stake) {
                let config = *borrow_global<ValidatorConfig>(candidate.addr);
                config.validator_index = num_new_actives;
                let new_validator_info = ValidatorInfo {
                    addr: candidate.addr,
                    voting_power: new_voting_power,
                    config,
                };

                // Update ValidatorSet.
                spec {
                    assume new_total_power + new_voting_power <= MAX_U128;
                };
                new_total_power = new_total_power + (new_voting_power as u128);
                vector::push_back(&mut new_active_validators, new_validator_info);
                num_new_actives = num_new_actives + 1;

            };
            candidate_idx = candidate_idx + 1;
        };

        let new_validator_set = ValidatorSet {
            consensus_scheme: cur_validator_set.consensus_scheme,
            active_validators: new_active_validators,
            pending_inactive: vector[],
            pending_active: vector[],
            total_voting_power: new_total_power,
            total_joining_power: 0,
        };

        validator_consensus_infos_from_validator_set(&new_validator_set)
    }
}
```


<a id="0x1_stake_validator_consensus_infos_from_validator_set"></a>

## Function `validator_consensus_infos_from_validator_set`



```move
module 0x1::stake {
    fun validator_consensus_infos_from_validator_set(validator_set: &stake::ValidatorSet): vector<validator_consensus_info::ValidatorConsensusInfo>
}
```


##### Implementation


```move
module 0x1::stake {
    fun validator_consensus_infos_from_validator_set(validator_set: &ValidatorSet): vector<ValidatorConsensusInfo> {
        let validator_consensus_infos = vector[];

        let num_active = vector::length(&validator_set.active_validators);
        let num_pending_inactive = vector::length(&validator_set.pending_inactive);
        spec {
            assume num_active + num_pending_inactive <= MAX_U64;
        };
        let total = num_active + num_pending_inactive;

        // Pre-fill the return value with dummy values.
        let idx = 0;
        while ({
            spec {
                invariant idx <= len(validator_set.active_validators) + len(validator_set.pending_inactive);
                invariant len(validator_consensus_infos) == idx;
                invariant len(validator_consensus_infos) <= len(validator_set.active_validators) + len(validator_set.pending_inactive);
            };
            idx < total
        }) {
            vector::push_back(&mut validator_consensus_infos, validator_consensus_info::default());
            idx = idx + 1;
        };
        spec {
            assert len(validator_consensus_infos) == len(validator_set.active_validators) + len(validator_set.pending_inactive);
            assert spec_validator_indices_are_valid_config(validator_set.active_validators,
                len(validator_set.active_validators) + len(validator_set.pending_inactive));
        };

        vector::for_each_ref(&validator_set.active_validators, |obj| {
            let vi: &ValidatorInfo = obj;
            spec {
                assume len(validator_consensus_infos) == len(validator_set.active_validators) + len(validator_set.pending_inactive);
                assert vi.config.validator_index < len(validator_consensus_infos);
            };
            let vci = vector::borrow_mut(&mut validator_consensus_infos, vi.config.validator_index);
            *vci = validator_consensus_info::new(
                vi.addr,
                vi.config.consensus_pubkey,
                vi.voting_power
            );
            spec {
                assert len(validator_consensus_infos) == len(validator_set.active_validators) + len(validator_set.pending_inactive);
            };
        });

        vector::for_each_ref(&validator_set.pending_inactive, |obj| {
            let vi: &ValidatorInfo = obj;
            spec {
                assume len(validator_consensus_infos) == len(validator_set.active_validators) + len(validator_set.pending_inactive);
                assert vi.config.validator_index < len(validator_consensus_infos);
            };
            let vci = vector::borrow_mut(&mut validator_consensus_infos, vi.config.validator_index);
            *vci = validator_consensus_info::new(
                vi.addr,
                vi.config.consensus_pubkey,
                vi.voting_power
            );
            spec {
                assert len(validator_consensus_infos) == len(validator_set.active_validators) + len(validator_set.pending_inactive);
            };
        });

        validator_consensus_infos
    }
}
```


<a id="0x1_stake_addresses_from_validator_infos"></a>

## Function `addresses_from_validator_infos`



```move
module 0x1::stake {
    fun addresses_from_validator_infos(infos: &vector<stake::ValidatorInfo>): vector<address>
}
```


##### Implementation


```move
module 0x1::stake {
    fun addresses_from_validator_infos(infos: &vector<ValidatorInfo>): vector<address> {
        vector::map_ref(infos, |obj| {
            let info: &ValidatorInfo = obj;
            info.addr
        })
    }
}
```


<a id="0x1_stake_update_stake_pool"></a>

## Function `update_stake_pool`

Calculate the stake amount of a stake pool for the next epoch.
Update individual validator&apos;s stake pool if `commit == true`.

1. distribute transaction fees to active/pending_inactive delegations
2. distribute rewards to active/pending_inactive delegations
3. process pending_active, pending_inactive correspondingly
This function shouldn&apos;t abort.


```move
module 0x1::stake {
    fun update_stake_pool(validator_perf: &stake::ValidatorPerformance, pool_address: address, staking_config: &staking_config::StakingConfig)
}
```


##### Implementation


```move
module 0x1::stake {
    fun update_stake_pool(
        validator_perf: &ValidatorPerformance,
        pool_address: address,
        staking_config: &StakingConfig,
    ) acquires StakePool, AptosCoinCapabilities, ValidatorConfig, ValidatorFees {
        let stake_pool = borrow_global_mut<StakePool>(pool_address);
        let validator_config = borrow_global<ValidatorConfig>(pool_address);
        let cur_validator_perf = vector::borrow(&validator_perf.validators, validator_config.validator_index);
        let num_successful_proposals = cur_validator_perf.successful_proposals;
        spec {
            // The following addition should not overflow because `num_total_proposals` cannot be larger than 86400,
            // the maximum number of proposals in a day (1 proposal per second).
            assume cur_validator_perf.successful_proposals + cur_validator_perf.failed_proposals <= MAX_U64;
        };
        let num_total_proposals = cur_validator_perf.successful_proposals + cur_validator_perf.failed_proposals;
        let (rewards_rate, rewards_rate_denominator) = staking_config::get_reward_rate(staking_config);
        let rewards_active = distribute_rewards(
            &mut stake_pool.active,
            num_successful_proposals,
            num_total_proposals,
            rewards_rate,
            rewards_rate_denominator
        );
        let rewards_pending_inactive = distribute_rewards(
            &mut stake_pool.pending_inactive,
            num_successful_proposals,
            num_total_proposals,
            rewards_rate,
            rewards_rate_denominator
        );
        spec {
            assume rewards_active + rewards_pending_inactive <= MAX_U64;
        };
        let rewards_amount = rewards_active + rewards_pending_inactive;
        // Pending active stake can now be active.
        coin::merge(&mut stake_pool.active, coin::extract_all(&mut stake_pool.pending_active));

        // Additionally, distribute transaction fees.
        if (features::collect_and_distribute_gas_fees()) {
            let fees_table = &mut borrow_global_mut<ValidatorFees>(@aptos_framework).fees_table;
            if (table::contains(fees_table, pool_address)) {
                let coin = table::remove(fees_table, pool_address);
                coin::merge(&mut stake_pool.active, coin);
            };
        };

        // Pending inactive stake is only fully unlocked and moved into inactive if the current lockup cycle has expired
        let current_lockup_expiration = stake_pool.locked_until_secs;
        if (get_reconfig_start_time_secs() >= current_lockup_expiration) {
            coin::merge(
                &mut stake_pool.inactive,
                coin::extract_all(&mut stake_pool.pending_inactive),
            );
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(DistributeRewards { pool_address, rewards_amount });
        };
        event::emit_event(
            &mut stake_pool.distribute_rewards_events,
            DistributeRewardsEvent {
                pool_address,
                rewards_amount,
            },
        );
    }
}
```


<a id="0x1_stake_get_reconfig_start_time_secs"></a>

## Function `get_reconfig_start_time_secs`

Assuming we are in a middle of a reconfiguration (no matter it is immediate or async), get its start time.


```move
module 0x1::stake {
    fun get_reconfig_start_time_secs(): u64
}
```


##### Implementation


```move
module 0x1::stake {
    fun get_reconfig_start_time_secs(): u64 {
        if (reconfiguration_state::is_initialized()) {
            reconfiguration_state::start_time_secs()
        } else {
            timestamp::now_seconds()
        }
    }
}
```


<a id="0x1_stake_calculate_rewards_amount"></a>

## Function `calculate_rewards_amount`

Calculate the rewards amount.


```move
module 0x1::stake {
    fun calculate_rewards_amount(stake_amount: u64, num_successful_proposals: u64, num_total_proposals: u64, rewards_rate: u64, rewards_rate_denominator: u64): u64
}
```


##### Implementation


```move
module 0x1::stake {
    fun calculate_rewards_amount(
        stake_amount: u64,
        num_successful_proposals: u64,
        num_total_proposals: u64,
        rewards_rate: u64,
        rewards_rate_denominator: u64,
    ): u64 {
        spec {
            // The following condition must hold because
            // (1) num_successful_proposals <= num_total_proposals, and
            // (2) `num_total_proposals` cannot be larger than 86400, the maximum number of proposals
            //     in a day (1 proposal per second), and `num_total_proposals` is reset to 0 every epoch.
            assume num_successful_proposals * MAX_REWARDS_RATE <= MAX_U64;
        };
        // The rewards amount is equal to (stake amount * rewards rate * performance multiplier).
        // We do multiplication in u128 before division to avoid the overflow and minimize the rounding error.
        let rewards_numerator = (stake_amount as u128) * (rewards_rate as u128) * (num_successful_proposals as u128);
        let rewards_denominator = (rewards_rate_denominator as u128) * (num_total_proposals as u128);
        if (rewards_denominator > 0) {
            ((rewards_numerator / rewards_denominator) as u64)
        } else {
            0
        }
    }
}
```


<a id="0x1_stake_distribute_rewards"></a>

## Function `distribute_rewards`

Mint rewards corresponding to current epoch&apos;s `stake` and `num_successful_votes`.


```move
module 0x1::stake {
    fun distribute_rewards(stake: &mut coin::Coin<aptos_coin::AptosCoin>, num_successful_proposals: u64, num_total_proposals: u64, rewards_rate: u64, rewards_rate_denominator: u64): u64
}
```


##### Implementation


```move
module 0x1::stake {
    fun distribute_rewards(
        stake: &mut Coin<AptosCoin>,
        num_successful_proposals: u64,
        num_total_proposals: u64,
        rewards_rate: u64,
        rewards_rate_denominator: u64,
    ): u64 acquires AptosCoinCapabilities {
        let stake_amount = coin::value(stake);
        let rewards_amount = if (stake_amount > 0) {
            calculate_rewards_amount(
                stake_amount,
                num_successful_proposals,
                num_total_proposals,
                rewards_rate,
                rewards_rate_denominator
            )
        } else {
            0
        };
        if (rewards_amount > 0) {
            let mint_cap = &borrow_global<AptosCoinCapabilities>(@aptos_framework).mint_cap;
            let rewards = coin::mint(rewards_amount, mint_cap);
            coin::merge(stake, rewards);
        };
        rewards_amount
    }
}
```


<a id="0x1_stake_append"></a>

## Function `append`



```move
module 0x1::stake {
    fun append<T>(v1: &mut vector<T>, v2: &mut vector<T>)
}
```


##### Implementation


```move
module 0x1::stake {
    fun append<T>(v1: &mut vector<T>, v2: &mut vector<T>) {
        while (!vector::is_empty(v2)) {
            vector::push_back(v1, vector::pop_back(v2));
        }
    }
}
```


<a id="0x1_stake_find_validator"></a>

## Function `find_validator`



```move
module 0x1::stake {
    fun find_validator(v: &vector<stake::ValidatorInfo>, addr: address): option::Option<u64>
}
```


##### Implementation


```move
module 0x1::stake {
    fun find_validator(v: &vector<ValidatorInfo>, addr: address): Option<u64> {
        let i = 0;
        let len = vector::length(v);
        while ({
            spec {
                invariant !(exists j in 0..i: v[j].addr == addr);
            };
            i < len
        }) {
            if (vector::borrow(v, i).addr == addr) {
                return option::some(i)
            };
            i = i + 1;
        };
        option::none()
    }
}
```


<a id="0x1_stake_generate_validator_info"></a>

## Function `generate_validator_info`



```move
module 0x1::stake {
    fun generate_validator_info(addr: address, stake_pool: &stake::StakePool, config: stake::ValidatorConfig): stake::ValidatorInfo
}
```


##### Implementation


```move
module 0x1::stake {
    fun generate_validator_info(addr: address, stake_pool: &StakePool, config: ValidatorConfig): ValidatorInfo {
        let voting_power = get_next_epoch_voting_power(stake_pool);
        ValidatorInfo {
            addr,
            voting_power,
            config,
        }
    }
}
```


<a id="0x1_stake_get_next_epoch_voting_power"></a>

## Function `get_next_epoch_voting_power`

Returns validator&apos;s next epoch voting power, including pending_active, active, and pending_inactive stake.


```move
module 0x1::stake {
    fun get_next_epoch_voting_power(stake_pool: &stake::StakePool): u64
}
```


##### Implementation


```move
module 0x1::stake {
    fun get_next_epoch_voting_power(stake_pool: &StakePool): u64 {
        let value_pending_active = coin::value(&stake_pool.pending_active);
        let value_active = coin::value(&stake_pool.active);
        let value_pending_inactive = coin::value(&stake_pool.pending_inactive);
        spec {
            assume value_pending_active + value_active + value_pending_inactive <= MAX_U64;
        };
        value_pending_active + value_active + value_pending_inactive
    }
}
```


<a id="0x1_stake_update_voting_power_increase"></a>

## Function `update_voting_power_increase`



```move
module 0x1::stake {
    fun update_voting_power_increase(increase_amount: u64)
}
```


##### Implementation


```move
module 0x1::stake {
    fun update_voting_power_increase(increase_amount: u64) acquires ValidatorSet {
        let validator_set = borrow_global_mut<ValidatorSet>(@aptos_framework);
        let voting_power_increase_limit =
            (staking_config::get_voting_power_increase_limit(&staking_config::get()) as u128);
        validator_set.total_joining_power = validator_set.total_joining_power + (increase_amount as u128);

        // Only validator voting power increase if the current validator set's voting power > 0.
        if (validator_set.total_voting_power > 0) {
            assert!(
                validator_set.total_joining_power <= validator_set.total_voting_power * voting_power_increase_limit / 100,
                error::invalid_argument(EVOTING_POWER_INCREASE_EXCEEDS_LIMIT),
            );
        }
    }
}
```


<a id="0x1_stake_assert_stake_pool_exists"></a>

## Function `assert_stake_pool_exists`



```move
module 0x1::stake {
    fun assert_stake_pool_exists(pool_address: address)
}
```


##### Implementation


```move
module 0x1::stake {
    fun assert_stake_pool_exists(pool_address: address) {
        assert!(stake_pool_exists(pool_address), error::invalid_argument(ESTAKE_POOL_DOES_NOT_EXIST));
    }
}
```


<a id="0x1_stake_configure_allowed_validators"></a>

## Function `configure_allowed_validators`



```move
module 0x1::stake {
    public fun configure_allowed_validators(aptos_framework: &signer, accounts: vector<address>)
}
```


##### Implementation


```move
module 0x1::stake {
    public fun configure_allowed_validators(
        aptos_framework: &signer,
        accounts: vector<address>
    ) acquires AllowedValidators {
        let aptos_framework_address = signer::address_of(aptos_framework);
        system_addresses::assert_aptos_framework(aptos_framework);
        if (!exists<AllowedValidators>(aptos_framework_address)) {
            move_to(aptos_framework, AllowedValidators { accounts });
        } else {
            let allowed = borrow_global_mut<AllowedValidators>(aptos_framework_address);
            allowed.accounts = accounts;
        }
    }
}
```


<a id="0x1_stake_is_allowed"></a>

## Function `is_allowed`



```move
module 0x1::stake {
    fun is_allowed(account: address): bool
}
```


##### Implementation


```move
module 0x1::stake {
    fun is_allowed(account: address): bool acquires AllowedValidators {
        if (!exists<AllowedValidators>(@aptos_framework)) {
            true
        } else {
            let allowed = borrow_global<AllowedValidators>(@aptos_framework);
            vector::contains(&allowed.accounts, &account)
        }
    }
}
```


<a id="0x1_stake_assert_owner_cap_exists"></a>

## Function `assert_owner_cap_exists`



```move
module 0x1::stake {
    fun assert_owner_cap_exists(owner: address)
}
```


##### Implementation


```move
module 0x1::stake {
    fun assert_owner_cap_exists(owner: address) {
        assert!(exists<OwnerCapability>(owner), error::not_found(EOWNER_CAP_NOT_FOUND));
    }
}
```


<a id="0x1_stake_assert_reconfig_not_in_progress"></a>

## Function `assert_reconfig_not_in_progress`



```move
module 0x1::stake {
    fun assert_reconfig_not_in_progress()
}
```


##### Implementation


```move
module 0x1::stake {
    fun assert_reconfig_not_in_progress() {
        assert!(!reconfiguration_state::is_in_progress(), error::invalid_state(ERECONFIGURATION_IN_PROGRESS));
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
<td>The validator set resource stores consensus information for each validator. The consensus scheme remains consistent across all validators within the set.</td>
<td>Low</td>
<td>The consensus_scheme attribute within ValidatorSet initializes with the value zero during the module&apos;s initialization and its value remains unchanged afterward.</td>
<td>Formally verified by the data invariant of [#high&#45;level&#45;req&#45;1](ValidatorSet).</td>
</tr>

<tr>
<td>2</td>
<td>The owner of a validator is immutable.</td>
<td>Low</td>
<td>During the initialization of a validator, the owner attribute becomes the signer&apos;s address. This assignment establishes the signer as the owner and controller of the validator entity. Subsequently, the owner attribute remains unchanged throughout the validator&apos;s lifespan, maintaining its assigned value without any modifications.</td>
<td>Formally verified in the schema [#high&#45;level&#45;req&#45;2](ValidatorOwnerNoChange).</td>
</tr>

<tr>
<td>3</td>
<td>The total staked value in the stake pool should remain constant, excluding operations related to adding and withdrawing.</td>
<td>Low</td>
<td>The total staked value (AptosCoin) of a stake pool is grouped by: active, inactive, pending_active, and pending_inactive. The stake value remains constant except during the execution of the add_stake_with_cap or withdraw_with_cap functions or on_new_epoch (which distributes the reward).</td>
<td>Formally specified in the schema [#high&#45;level&#45;req&#45;3](StakedValueNoChange).</td>
</tr>

<tr>
<td>4</td>
<td>During each epoch, the following operations should be consistently performed without aborting: rewards distribution, validator activation/deactivation, updates to validator sets and voting power, and renewal of lockups.</td>
<td>Low</td>
<td>The on_new_epoch function is triggered at each epoch boundary to perform distribution of the transaction fee, updates to active/inactive stakes, updates to pending active/inactive validators and adjusts voting power of the validators without aborting.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](on_new_epoch). This also requires a manual review to verify the state updates of the stake pool.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::stake {
    pragma verify = true;
    invariant [suspendable] exists<ValidatorSet>(@aptos_framework) ==> validator_set_is_valid();
    invariant [suspendable] chain_status::is_operating() ==> exists<AptosCoinCapabilities>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==> exists<ValidatorPerformance>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==> exists<ValidatorSet>(@aptos_framework);
    apply ValidatorOwnerNoChange to *;
    apply ValidatorNotChangeDuringReconfig to * except on_new_epoch;
    apply StakePoolNotChangeDuringReconfig to * except on_new_epoch, update_stake_pool;
<a id="0x1_stake_ghost_valid_perf"></a>
    global ghost_valid_perf: ValidatorPerformance;
<a id="0x1_stake_ghost_proposer_idx"></a>
    global ghost_proposer_idx: Option<u64>;
<a id="0x1_stake_ghost_active_num"></a>
    global ghost_active_num: u64;
<a id="0x1_stake_ghost_pending_inactive_num"></a>
    global ghost_pending_inactive_num: u64;
}
```


<a id="@Specification_1_ValidatorSet"></a>

### Resource `ValidatorSet`


```move
module 0x1::stake {
    struct ValidatorSet has copy, drop, store, key
}
```


<dl>
<dt>
`consensus_scheme: u8`
</dt>
<dd>

</dd>
<dt>
`active_validators: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`pending_inactive: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`pending_active: vector<stake::ValidatorInfo>`
</dt>
<dd>

</dd>
<dt>
`total_voting_power: u128`
</dt>
<dd>

</dd>
<dt>
`total_joining_power: u128`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::stake {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    invariant consensus_scheme == 0;
}
```



<a id="0x1_stake_ValidatorNotChangeDuringReconfig"></a>


```move
module 0x1::stake {
    schema ValidatorNotChangeDuringReconfig {
        ensures (reconfiguration_state::spec_is_in_progress() && old(exists<ValidatorSet>(@aptos_framework))) ==>
            old(global<ValidatorSet>(@aptos_framework)) == global<ValidatorSet>(@aptos_framework);
    }
}
```



<a id="0x1_stake_StakePoolNotChangeDuringReconfig"></a>


```move
module 0x1::stake {
    schema StakePoolNotChangeDuringReconfig {
        ensures forall a: address where old(exists<StakePool>(a)): reconfiguration_state::spec_is_in_progress() ==>
            (old(global<StakePool>(a).pending_inactive) == global<StakePool>(a).pending_inactive &&
            old(global<StakePool>(a).pending_active) == global<StakePool>(a).pending_active &&
            old(global<StakePool>(a).inactive) == global<StakePool>(a).inactive &&
            old(global<StakePool>(a).active) == global<StakePool>(a).active);
    }
}
```



<a id="0x1_stake_ValidatorOwnerNoChange"></a>


```move
module 0x1::stake {
    schema ValidatorOwnerNoChange {
    // This enforces ### high&#45;level&#45;req&#45;2
    [#high&#45;level&#45;req](high&#45;level requirement 2):
        ensures forall addr: address where old(exists<OwnerCapability>(addr)):
            old(global<OwnerCapability>(addr)).pool_address == global<OwnerCapability>(addr).pool_address;
    }
}
```



<a id="0x1_stake_StakedValueNochange"></a>


```move
module 0x1::stake {
    schema StakedValueNochange {
        pool_address: address;
        let stake_pool = global<StakePool>(pool_address);
        let post post_stake_pool = global<StakePool>(pool_address);
    // This enforces ### high&#45;level&#45;req&#45;3
    [#high&#45;level&#45;req](high&#45;level requirement 3):
        ensures stake_pool.active.value + stake_pool.inactive.value + stake_pool.pending_active.value + stake_pool.pending_inactive.value ==
            post_stake_pool.active.value + post_stake_pool.inactive.value + post_stake_pool.pending_active.value + post_stake_pool.pending_inactive.value;
    }
}
```



<a id="0x1_stake_validator_set_is_valid"></a>


```move
module 0x1::stake {
    fun validator_set_is_valid(): bool {
       let validator_set = global<ValidatorSet>(@aptos_framework);
       validator_set_is_valid_impl(validator_set)
    }
}
```



<a id="0x1_stake_validator_set_is_valid_impl"></a>


```move
module 0x1::stake {
    fun validator_set_is_valid_impl(validator_set: ValidatorSet): bool {
       spec_validators_are_initialized(validator_set.active_validators) &&
           spec_validators_are_initialized(validator_set.pending_inactive) &&
           spec_validators_are_initialized(validator_set.pending_active) &&
           spec_validator_indices_are_valid(validator_set.active_validators) &&
           spec_validator_indices_are_valid(validator_set.pending_inactive)
           && spec_validator_indices_active_pending_inactive(validator_set)
    }
}
```


<a id="@Specification_1_initialize_validator_fees"></a>

### Function `initialize_validator_fees`


```move
module 0x1::stake {
    public(friend) fun initialize_validator_fees(aptos_framework: &signer)
}
```



```move
module 0x1::stake {
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if exists<ValidatorFees>(aptos_addr);
    ensures exists<ValidatorFees>(aptos_addr);
}
```


<a id="@Specification_1_add_transaction_fee"></a>

### Function `add_transaction_fee`


```move
module 0x1::stake {
    public(friend) fun add_transaction_fee(validator_addr: address, fee: coin::Coin<aptos_coin::AptosCoin>)
}
```



```move
module 0x1::stake {
    aborts_if !exists<ValidatorFees>(@aptos_framework);
    let fees_table = global<ValidatorFees>(@aptos_framework).fees_table;
    let post post_fees_table = global<ValidatorFees>(@aptos_framework).fees_table;
    let collected_fee = table::spec_get(fees_table, validator_addr);
    let post post_collected_fee = table::spec_get(post_fees_table, validator_addr);
    ensures if (table::spec_contains(fees_table, validator_addr)) {
        post_collected_fee.value == collected_fee.value + fee.value
    } else {
        table::spec_contains(post_fees_table, validator_addr) &&
        table::spec_get(post_fees_table, validator_addr) == fee
    };
}
```


<a id="@Specification_1_get_validator_state"></a>

### Function `get_validator_state`


```move
module 0x1::stake {
    #[view]
    public fun get_validator_state(pool_address: address): u64
}
```



```move
module 0x1::stake {
    aborts_if !exists<ValidatorSet>(@aptos_framework);
    let validator_set = global<ValidatorSet>(@aptos_framework);
    ensures result == VALIDATOR_STATUS_PENDING_ACTIVE ==> spec_contains(validator_set.pending_active, pool_address);
    ensures result == VALIDATOR_STATUS_ACTIVE ==> spec_contains(validator_set.active_validators, pool_address);
    ensures result == VALIDATOR_STATUS_PENDING_INACTIVE ==> spec_contains(validator_set.pending_inactive, pool_address);
    ensures result == VALIDATOR_STATUS_INACTIVE ==> (
        !spec_contains(validator_set.pending_active, pool_address)
            && !spec_contains(validator_set.active_validators, pool_address)
            && !spec_contains(validator_set.pending_inactive, pool_address)
    );
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::stake {
    public(friend) fun initialize(aptos_framework: &signer)
}
```



```move
module 0x1::stake {
    pragma disable_invariants_in_body;
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if exists<ValidatorSet>(aptos_addr);
    aborts_if exists<ValidatorPerformance>(aptos_addr);
    ensures exists<ValidatorSet>(aptos_addr);
    ensures global<ValidatorSet>(aptos_addr).consensus_scheme == 0;
    ensures exists<ValidatorPerformance>(aptos_addr);
}
```


<a id="@Specification_1_remove_validators"></a>

### Function `remove_validators`


```move
module 0x1::stake {
    public fun remove_validators(aptos_framework: &signer, validators: &vector<address>)
}
```



```move
module 0x1::stake {
    requires chain_status::is_operating();
    let validator_set = global<ValidatorSet>(@aptos_framework);
    let post post_validator_set = global<ValidatorSet>(@aptos_framework);
    let active_validators = validator_set.active_validators;
    let post post_active_validators = post_validator_set.active_validators;
    let pending_inactive_validators = validator_set.pending_inactive;
    let post post_pending_inactive_validators = post_validator_set.pending_inactive;
    invariant len(active_validators) > 0;
    ensures len(active_validators) + len(pending_inactive_validators) == len(post_active_validators)
        + len(post_pending_inactive_validators);
}
```


<a id="@Specification_1_initialize_stake_owner"></a>

### Function `initialize_stake_owner`


```move
module 0x1::stake {
    public entry fun initialize_stake_owner(owner: &signer, initial_stake_amount: u64, operator: address, voter: address)
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 120;
    include ResourceRequirement;
    let addr = signer::address_of(owner);
    ensures global<ValidatorConfig>(addr) == ValidatorConfig {
        consensus_pubkey: vector::empty(),
        network_addresses: vector::empty(),
        fullnode_addresses: vector::empty(),
        validator_index: 0,
    };
    ensures global<OwnerCapability>(addr) == OwnerCapability { pool_address: addr };
    let post stakepool = global<StakePool>(addr);
    let post active = stakepool.active.value;
    let post pending_active = stakepool.pending_active.value;
    ensures spec_is_current_epoch_validator(addr) ==>
        pending_active == initial_stake_amount;
    ensures !spec_is_current_epoch_validator(addr) ==>
        active == initial_stake_amount;
}
```


<a id="@Specification_1_initialize_validator"></a>

### Function `initialize_validator`


```move
module 0x1::stake {
    public entry fun initialize_validator(account: &signer, consensus_pubkey: vector<u8>, proof_of_possession: vector<u8>, network_addresses: vector<u8>, fullnode_addresses: vector<u8>)
}
```



```move
module 0x1::stake {
    let pubkey_from_pop = bls12381::spec_public_key_from_bytes_with_pop(
        consensus_pubkey,
        proof_of_possession_from_bytes(proof_of_possession)
    );
    aborts_if !option::spec_is_some(pubkey_from_pop);
    let addr = signer::address_of(account);
    let post_addr = signer::address_of(account);
    let allowed = global<AllowedValidators>(@aptos_framework);
    aborts_if exists<ValidatorConfig>(addr);
    aborts_if exists<AllowedValidators>(@aptos_framework) && !vector::spec_contains(allowed.accounts, addr);
    aborts_if stake_pool_exists(addr);
    aborts_if exists<OwnerCapability>(addr);
    aborts_if !exists<account::Account>(addr);
    aborts_if global<account::Account>(addr).guid_creation_num + 12 > MAX_U64;
    aborts_if global<account::Account>(addr).guid_creation_num + 12 >= account::MAX_GUID_CREATION_NUM;
    ensures exists<StakePool>(post_addr);
    ensures global<OwnerCapability>(post_addr) == OwnerCapability { pool_address: post_addr };
    ensures global<ValidatorConfig>(post_addr) == ValidatorConfig {
        consensus_pubkey,
        network_addresses,
        fullnode_addresses,
        validator_index: 0,
    };
}
```


<a id="@Specification_1_extract_owner_cap"></a>

### Function `extract_owner_cap`


```move
module 0x1::stake {
    public fun extract_owner_cap(owner: &signer): stake::OwnerCapability
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 300;
    let owner_address = signer::address_of(owner);
    aborts_if !exists<OwnerCapability>(owner_address);
    ensures !exists<OwnerCapability>(owner_address);
}
```


<a id="@Specification_1_deposit_owner_cap"></a>

### Function `deposit_owner_cap`


```move
module 0x1::stake {
    public fun deposit_owner_cap(owner: &signer, owner_cap: stake::OwnerCapability)
}
```



```move
module 0x1::stake {
    let owner_address = signer::address_of(owner);
    aborts_if exists<OwnerCapability>(owner_address);
    ensures exists<OwnerCapability>(owner_address);
    ensures global<OwnerCapability>(owner_address) == owner_cap;
}
```


<a id="@Specification_1_set_operator_with_cap"></a>

### Function `set_operator_with_cap`


```move
module 0x1::stake {
    public fun set_operator_with_cap(owner_cap: &stake::OwnerCapability, new_operator: address)
}
```



```move
module 0x1::stake {
    let pool_address = owner_cap.pool_address;
    let post post_stake_pool = global<StakePool>(pool_address);
    modifies global<StakePool>(pool_address);
    include StakedValueNochange;
    ensures post_stake_pool.operator_address == new_operator;
}
```


<a id="@Specification_1_set_delegated_voter_with_cap"></a>

### Function `set_delegated_voter_with_cap`


```move
module 0x1::stake {
    public fun set_delegated_voter_with_cap(owner_cap: &stake::OwnerCapability, new_voter: address)
}
```



```move
module 0x1::stake {
    let pool_address = owner_cap.pool_address;
    let post post_stake_pool = global<StakePool>(pool_address);
    include StakedValueNochange;
    aborts_if !exists<StakePool>(pool_address);
    modifies global<StakePool>(pool_address);
    ensures post_stake_pool.delegated_voter == new_voter;
}
```


<a id="@Specification_1_add_stake"></a>

### Function `add_stake`


```move
module 0x1::stake {
    public entry fun add_stake(owner: &signer, amount: u64)
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 120;
    pragma aborts_if_is_partial;
    aborts_if reconfiguration_state::spec_is_in_progress();
    include ResourceRequirement;
    include AddStakeAbortsIfAndEnsures;
}
```


<a id="@Specification_1_add_stake_with_cap"></a>

### Function `add_stake_with_cap`


```move
module 0x1::stake {
    public fun add_stake_with_cap(owner_cap: &stake::OwnerCapability, coins: coin::Coin<aptos_coin::AptosCoin>)
}
```



```move
module 0x1::stake {
    pragma disable_invariants_in_body;
    pragma verify_duration_estimate = 300;
    include ResourceRequirement;
    let amount = coins.value;
    aborts_if reconfiguration_state::spec_is_in_progress();
    include AddStakeWithCapAbortsIfAndEnsures { amount };
}
```


<a id="@Specification_1_reactivate_stake_with_cap"></a>

### Function `reactivate_stake_with_cap`


```move
module 0x1::stake {
    public fun reactivate_stake_with_cap(owner_cap: &stake::OwnerCapability, amount: u64)
}
```



```move
module 0x1::stake {
    let pool_address = owner_cap.pool_address;
    include StakedValueNochange;
    aborts_if reconfiguration_state::spec_is_in_progress();
    aborts_if !stake_pool_exists(pool_address);
    let pre_stake_pool = global<StakePool>(pool_address);
    let post stake_pool = global<StakePool>(pool_address);
    modifies global<StakePool>(pool_address);
    let min_amount = aptos_std::math64::min(amount, pre_stake_pool.pending_inactive.value);
    ensures stake_pool.pending_inactive.value == pre_stake_pool.pending_inactive.value - min_amount;
    ensures stake_pool.active.value == pre_stake_pool.active.value + min_amount;
}
```


<a id="@Specification_1_rotate_consensus_key"></a>

### Function `rotate_consensus_key`


```move
module 0x1::stake {
    public entry fun rotate_consensus_key(operator: &signer, pool_address: address, new_consensus_pubkey: vector<u8>, proof_of_possession: vector<u8>)
}
```



```move
module 0x1::stake {
    let pre_stake_pool = global<StakePool>(pool_address);
    let post validator_info = global<ValidatorConfig>(pool_address);
    aborts_if reconfiguration_state::spec_is_in_progress();
    aborts_if !exists<StakePool>(pool_address);
    aborts_if signer::address_of(operator) != pre_stake_pool.operator_address;
    aborts_if !exists<ValidatorConfig>(pool_address);
    let pubkey_from_pop = bls12381::spec_public_key_from_bytes_with_pop(
        new_consensus_pubkey,
        proof_of_possession_from_bytes(proof_of_possession)
    );
    aborts_if !option::spec_is_some(pubkey_from_pop);
    modifies global<ValidatorConfig>(pool_address);
    include StakedValueNochange;
    ensures validator_info.consensus_pubkey == new_consensus_pubkey;
}
```


<a id="@Specification_1_update_network_and_fullnode_addresses"></a>

### Function `update_network_and_fullnode_addresses`


```move
module 0x1::stake {
    public entry fun update_network_and_fullnode_addresses(operator: &signer, pool_address: address, new_network_addresses: vector<u8>, new_fullnode_addresses: vector<u8>)
}
```



```move
module 0x1::stake {
    let pre_stake_pool = global<StakePool>(pool_address);
    let post validator_info = global<ValidatorConfig>(pool_address);
    modifies global<ValidatorConfig>(pool_address);
    include StakedValueNochange;
    aborts_if reconfiguration_state::spec_is_in_progress();
    aborts_if !exists<StakePool>(pool_address);
    aborts_if !exists<ValidatorConfig>(pool_address);
    aborts_if signer::address_of(operator) != pre_stake_pool.operator_address;
    ensures validator_info.network_addresses == new_network_addresses;
    ensures validator_info.fullnode_addresses == new_fullnode_addresses;
}
```


<a id="@Specification_1_increase_lockup_with_cap"></a>

### Function `increase_lockup_with_cap`


```move
module 0x1::stake {
    public fun increase_lockup_with_cap(owner_cap: &stake::OwnerCapability)
}
```



```move
module 0x1::stake {
    let config = global<staking_config::StakingConfig>(@aptos_framework);
    let pool_address = owner_cap.pool_address;
    let pre_stake_pool = global<StakePool>(pool_address);
    let post stake_pool = global<StakePool>(pool_address);
    let now_seconds = timestamp::spec_now_seconds();
    let lockup = config.recurring_lockup_duration_secs;
    modifies global<StakePool>(pool_address);
    include StakedValueNochange;
    aborts_if !exists<StakePool>(pool_address);
    aborts_if pre_stake_pool.locked_until_secs >= lockup + now_seconds;
    aborts_if lockup + now_seconds > MAX_U64;
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
    ensures stake_pool.locked_until_secs == lockup + now_seconds;
}
```


<a id="@Specification_1_join_validator_set"></a>

### Function `join_validator_set`


```move
module 0x1::stake {
    public entry fun join_validator_set(operator: &signer, pool_address: address)
}
```



```move
module 0x1::stake {
    pragma disable_invariants_in_body;
    aborts_if !staking_config::get_allow_validator_set_change(staking_config::get());
    aborts_if !exists<StakePool>(pool_address);
    aborts_if !exists<ValidatorConfig>(pool_address);
    aborts_if !exists<StakingConfig>(@aptos_framework);
    aborts_if !exists<ValidatorSet>(@aptos_framework);
    aborts_if reconfiguration_state::spec_is_in_progress();
    let stake_pool = global<StakePool>(pool_address);
    let validator_set = global<ValidatorSet>(@aptos_framework);
    let post p_validator_set = global<ValidatorSet>(@aptos_framework);
    aborts_if signer::address_of(operator) != stake_pool.operator_address;
    aborts_if option::spec_is_some(spec_find_validator(validator_set.active_validators, pool_address)) ||
                option::spec_is_some(spec_find_validator(validator_set.pending_inactive, pool_address)) ||
                    option::spec_is_some(spec_find_validator(validator_set.pending_active, pool_address));
    let config = staking_config::get();
    let voting_power = get_next_epoch_voting_power(stake_pool);
    let minimum_stake = config.minimum_stake;
    let maximum_stake = config.maximum_stake;
    aborts_if voting_power < minimum_stake;
    aborts_if voting_power >maximum_stake;
    let validator_config = global<ValidatorConfig>(pool_address);
    aborts_if vector::is_empty(validator_config.consensus_pubkey);
    let validator_set_size = vector::length(validator_set.active_validators) + vector::length(validator_set.pending_active) + 1;
    aborts_if validator_set_size > MAX_VALIDATOR_SET_SIZE;
    let voting_power_increase_limit = (staking_config::get_voting_power_increase_limit(config) as u128);
    aborts_if (validator_set.total_joining_power + (voting_power as u128)) > MAX_U128;
    aborts_if validator_set.total_voting_power * voting_power_increase_limit > MAX_U128;
    aborts_if validator_set.total_voting_power > 0 &&
        (validator_set.total_joining_power + (voting_power as u128)) * 100 > validator_set.total_voting_power * voting_power_increase_limit;
    let post p_validator_info = ValidatorInfo {
        addr: pool_address,
        voting_power,
        config: validator_config,
    };
    ensures validator_set.total_joining_power + voting_power == p_validator_set.total_joining_power;
    ensures vector::spec_contains(p_validator_set.pending_active, p_validator_info);
}
```


<a id="@Specification_1_unlock_with_cap"></a>

### Function `unlock_with_cap`


```move
module 0x1::stake {
    public fun unlock_with_cap(amount: u64, owner_cap: &stake::OwnerCapability)
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 300;
    let pool_address = owner_cap.pool_address;
    let pre_stake_pool = global<StakePool>(pool_address);
    let post stake_pool = global<StakePool>(pool_address);
    aborts_if reconfiguration_state::spec_is_in_progress();
    aborts_if amount != 0 && !exists<StakePool>(pool_address);
    modifies global<StakePool>(pool_address);
    include StakedValueNochange;
    let min_amount = aptos_std::math64::min(amount,pre_stake_pool.active.value);
    ensures stake_pool.active.value == pre_stake_pool.active.value - min_amount;
    ensures stake_pool.pending_inactive.value == pre_stake_pool.pending_inactive.value + min_amount;
}
```


<a id="@Specification_1_withdraw"></a>

### Function `withdraw`


```move
module 0x1::stake {
    public entry fun withdraw(owner: &signer, withdraw_amount: u64)
}
```



```move
module 0x1::stake {
    pragma verify = false;
    aborts_if reconfiguration_state::spec_is_in_progress();
    let addr = signer::address_of(owner);
    let ownership_cap = global<OwnerCapability>(addr);
    let pool_address = ownership_cap.pool_address;
    let stake_pool = global<StakePool>(pool_address);
    aborts_if !exists<OwnerCapability>(addr);
    aborts_if !exists<StakePool>(pool_address);
    aborts_if !exists<ValidatorSet>(@aptos_framework);
    let validator_set = global<ValidatorSet>(@aptos_framework);
    let bool_find_validator = !option::spec_is_some(spec_find_validator(validator_set.active_validators, pool_address)) &&
                !option::spec_is_some(spec_find_validator(validator_set.pending_inactive, pool_address)) &&
                    !option::spec_is_some(spec_find_validator(validator_set.pending_active, pool_address));
    aborts_if bool_find_validator && !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    let new_withdraw_amount_1 = min(withdraw_amount, stake_pool.inactive.value + stake_pool.pending_inactive.value);
    let new_withdraw_amount_2 = min(withdraw_amount, stake_pool.inactive.value);
    aborts_if bool_find_validator && timestamp::now_seconds() > stake_pool.locked_until_secs &&
                new_withdraw_amount_1 > 0 && stake_pool.inactive.value + stake_pool.pending_inactive.value < new_withdraw_amount_1;
    aborts_if !(bool_find_validator && exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework)) &&
                new_withdraw_amount_2 > 0 && stake_pool.inactive.value < new_withdraw_amount_2;
    aborts_if !exists<coin::CoinStore<AptosCoin>>(addr);
    include coin::DepositAbortsIf<AptosCoin>{account_addr: addr};
    let coin_store = global<coin::CoinStore<AptosCoin>>(addr);
    let post p_coin_store = global<coin::CoinStore<AptosCoin>>(addr);
    ensures bool_find_validator && timestamp::now_seconds() > stake_pool.locked_until_secs
                && exists<account::Account>(addr) && exists<coin::CoinStore<AptosCoin>>(addr) ==>
                    coin_store.coin.value + new_withdraw_amount_1 == p_coin_store.coin.value;
    ensures !(bool_find_validator && exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework))
                && exists<account::Account>(addr) && exists<coin::CoinStore<AptosCoin>>(addr) ==>
                    coin_store.coin.value + new_withdraw_amount_2 == p_coin_store.coin.value;
}
```


<a id="@Specification_1_leave_validator_set"></a>

### Function `leave_validator_set`


```move
module 0x1::stake {
    public entry fun leave_validator_set(operator: &signer, pool_address: address)
}
```



```move
module 0x1::stake {
    pragma disable_invariants_in_body;
    requires chain_status::is_operating();
    aborts_if reconfiguration_state::spec_is_in_progress();
    let config = staking_config::get();
    aborts_if !staking_config::get_allow_validator_set_change(config);
    aborts_if !exists<StakePool>(pool_address);
    aborts_if !exists<ValidatorSet>(@aptos_framework);
    aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
    let stake_pool = global<StakePool>(pool_address);
    aborts_if signer::address_of(operator) != stake_pool.operator_address;
    let validator_set = global<ValidatorSet>(@aptos_framework);
    let validator_find_bool = option::spec_is_some(spec_find_validator(validator_set.pending_active, pool_address));
    let active_validators = validator_set.active_validators;
    let pending_active = validator_set.pending_active;
    let post post_validator_set = global<ValidatorSet>(@aptos_framework);
    let post post_active_validators = post_validator_set.active_validators;
    let pending_inactive_validators = validator_set.pending_inactive;
    let post post_pending_inactive_validators = post_validator_set.pending_inactive;
    ensures len(active_validators) + len(pending_inactive_validators) == len(post_active_validators)
        + len(post_pending_inactive_validators);
    aborts_if !validator_find_bool && !option::spec_is_some(spec_find_validator(active_validators, pool_address));
    aborts_if !validator_find_bool && vector::length(validator_set.active_validators) <= option::spec_borrow(spec_find_validator(active_validators, pool_address));
    aborts_if !validator_find_bool && vector::length(validator_set.active_validators) < 2;
    aborts_if validator_find_bool && vector::length(validator_set.pending_active) <= option::spec_borrow(spec_find_validator(pending_active, pool_address));
    let post p_validator_set = global<ValidatorSet>(@aptos_framework);
    let validator_stake = (get_next_epoch_voting_power(stake_pool) as u128);
    ensures validator_find_bool && validator_set.total_joining_power > validator_stake ==>
                p_validator_set.total_joining_power == validator_set.total_joining_power - validator_stake;
    ensures !validator_find_bool ==> !option::spec_is_some(spec_find_validator(p_validator_set.pending_active, pool_address));
}
```


<a id="@Specification_1_is_current_epoch_validator"></a>

### Function `is_current_epoch_validator`


```move
module 0x1::stake {
    public fun is_current_epoch_validator(pool_address: address): bool
}
```



```move
module 0x1::stake {
    include ResourceRequirement;
    aborts_if !spec_has_stake_pool(pool_address);
    ensures result == spec_is_current_epoch_validator(pool_address);
}
```


<a id="@Specification_1_update_performance_statistics"></a>

### Function `update_performance_statistics`


```move
module 0x1::stake {
    public(friend) fun update_performance_statistics(proposer_index: option::Option<u64>, failed_proposer_indices: vector<u64>)
}
```



```move
module 0x1::stake {
    requires chain_status::is_operating();
    aborts_if false;
    let validator_perf = global<ValidatorPerformance>(@aptos_framework);
    let post post_validator_perf = global<ValidatorPerformance>(@aptos_framework);
    let validator_len = len(validator_perf.validators);
    ensures (option::spec_is_some(ghost_proposer_idx) && option::spec_borrow(ghost_proposer_idx) < validator_len) ==>
        (post_validator_perf.validators[option::spec_borrow(ghost_proposer_idx)].successful_proposals ==
            validator_perf.validators[option::spec_borrow(ghost_proposer_idx)].successful_proposals + 1);
}
```


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::stake {
    public(friend) fun on_new_epoch()
}
```



```move
module 0x1::stake {
    pragma verify = false;
    pragma disable_invariants_in_body;
    include ResourceRequirement;
    include GetReconfigStartTimeRequirement;
    include staking_config::StakingRewardsConfigRequirement;
    include aptos_framework::aptos_coin::ExistsAptosCoin;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if false;
}
```


<a id="@Specification_1_next_validator_consensus_infos"></a>

### Function `next_validator_consensus_infos`


```move
module 0x1::stake {
    public fun next_validator_consensus_infos(): vector<validator_consensus_info::ValidatorConsensusInfo>
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 300;
    aborts_if false;
    include ResourceRequirement;
    include GetReconfigStartTimeRequirement;
    include features::spec_periodical_reward_rate_decrease_enabled() ==> staking_config::StakingRewardsConfigEnabledRequirement;
}
```


<a id="@Specification_1_validator_consensus_infos_from_validator_set"></a>

### Function `validator_consensus_infos_from_validator_set`


```move
module 0x1::stake {
    fun validator_consensus_infos_from_validator_set(validator_set: &stake::ValidatorSet): vector<validator_consensus_info::ValidatorConsensusInfo>
}
```



```move
module 0x1::stake {
    aborts_if false;
    invariant spec_validator_indices_are_valid_config(validator_set.active_validators,
        len(validator_set.active_validators) + len(validator_set.pending_inactive));
    invariant len(validator_set.pending_inactive) == 0 ||
        spec_validator_indices_are_valid_config(validator_set.pending_inactive,
            len(validator_set.active_validators) + len(validator_set.pending_inactive));
}
```



<a id="0x1_stake_AddStakeWithCapAbortsIfAndEnsures"></a>


```move
module 0x1::stake {
    schema AddStakeWithCapAbortsIfAndEnsures {
        owner_cap: OwnerCapability;
        amount: u64;
        let pool_address = owner_cap.pool_address;
        aborts_if !exists<StakePool>(pool_address);
        let config = global<staking_config::StakingConfig>(@aptos_framework);
        let validator_set = global<ValidatorSet>(@aptos_framework);
        let voting_power_increase_limit = config.voting_power_increase_limit;
        let post post_validator_set = global<ValidatorSet>(@aptos_framework);
        let update_voting_power_increase = amount != 0 && (spec_contains(validator_set.active_validators, pool_address)
                                                           || spec_contains(validator_set.pending_active, pool_address));
        aborts_if update_voting_power_increase && validator_set.total_joining_power + amount > MAX_U128;
        ensures update_voting_power_increase ==> post_validator_set.total_joining_power == validator_set.total_joining_power + amount;
        aborts_if update_voting_power_increase && validator_set.total_voting_power > 0
                && validator_set.total_voting_power * voting_power_increase_limit > MAX_U128;
        aborts_if update_voting_power_increase && validator_set.total_voting_power > 0
                && validator_set.total_joining_power + amount > validator_set.total_voting_power * voting_power_increase_limit / 100;
        let stake_pool = global<StakePool>(pool_address);
        let post post_stake_pool = global<StakePool>(pool_address);
        let value_pending_active = stake_pool.pending_active.value;
        let value_active = stake_pool.active.value;
        ensures amount != 0 && spec_is_current_epoch_validator(pool_address) ==> post_stake_pool.pending_active.value == value_pending_active + amount;
        ensures amount != 0 && !spec_is_current_epoch_validator(pool_address) ==> post_stake_pool.active.value == value_active + amount;
        let maximum_stake = config.maximum_stake;
        let value_pending_inactive = stake_pool.pending_inactive.value;
        let next_epoch_voting_power = value_pending_active + value_active + value_pending_inactive;
        let voting_power = next_epoch_voting_power + amount;
        aborts_if amount != 0 && voting_power > MAX_U64;
        aborts_if amount != 0 && voting_power > maximum_stake;
    }
}
```



<a id="0x1_stake_AddStakeAbortsIfAndEnsures"></a>


```move
module 0x1::stake {
    schema AddStakeAbortsIfAndEnsures {
        owner: signer;
        amount: u64;
        let owner_address = signer::address_of(owner);
        aborts_if !exists<OwnerCapability>(owner_address);
        let owner_cap = global<OwnerCapability>(owner_address);
        include AddStakeWithCapAbortsIfAndEnsures { owner_cap };
    }
}
```



<a id="0x1_stake_spec_is_allowed"></a>


```move
module 0x1::stake {
    fun spec_is_allowed(account: address): bool {
       if (!exists<AllowedValidators>(@aptos_framework)) {
           true
       } else {
           let allowed = global<AllowedValidators>(@aptos_framework);
           contains(allowed.accounts, account)
       }
    }
}
```



<a id="0x1_stake_spec_find_validator"></a>


```move
module 0x1::stake {
    fun spec_find_validator(v: vector<ValidatorInfo>, addr: address): Option<u64>;
}
```



<a id="0x1_stake_spec_validators_are_initialized"></a>


```move
module 0x1::stake {
    fun spec_validators_are_initialized(validators: vector<ValidatorInfo>): bool {
       forall i in 0..len(validators):
           spec_has_stake_pool(validators[i].addr) &&
               spec_has_validator_config(validators[i].addr)
    }
}
```



<a id="0x1_stake_spec_validators_are_initialized_addrs"></a>


```move
module 0x1::stake {
    fun spec_validators_are_initialized_addrs(addrs: vector<address>): bool {
       forall i in 0..len(addrs):
           spec_has_stake_pool(addrs[i]) &&
               spec_has_validator_config(addrs[i])
    }
}
```



<a id="0x1_stake_spec_validator_indices_are_valid"></a>


```move
module 0x1::stake {
    fun spec_validator_indices_are_valid(validators: vector<ValidatorInfo>): bool {
       spec_validator_indices_are_valid_addr(validators, spec_validator_index_upper_bound()) &&
           spec_validator_indices_are_valid_config(validators, spec_validator_index_upper_bound())
    }
}
```



<a id="0x1_stake_spec_validator_indices_are_valid_addr"></a>


```move
module 0x1::stake {
    fun spec_validator_indices_are_valid_addr(validators: vector<ValidatorInfo>, upper_bound: u64): bool {
       forall i in 0..len(validators):
           global<ValidatorConfig>(validators[i].addr).validator_index < upper_bound
    }
}
```



<a id="0x1_stake_spec_validator_indices_are_valid_config"></a>


```move
module 0x1::stake {
    fun spec_validator_indices_are_valid_config(validators: vector<ValidatorInfo>, upper_bound: u64): bool {
       forall i in 0..len(validators):
           validators[i].config.validator_index < upper_bound
    }
}
```



<a id="0x1_stake_spec_validator_indices_active_pending_inactive"></a>


```move
module 0x1::stake {
    fun spec_validator_indices_active_pending_inactive(validator_set: ValidatorSet): bool {
       len(validator_set.pending_inactive) + len(validator_set.active_validators) == spec_validator_index_upper_bound()
    }
}
```



<a id="0x1_stake_spec_validator_index_upper_bound"></a>


```move
module 0x1::stake {
    fun spec_validator_index_upper_bound(): u64 {
       len(global<ValidatorPerformance>(@aptos_framework).validators)
    }
}
```



<a id="0x1_stake_spec_has_stake_pool"></a>


```move
module 0x1::stake {
    fun spec_has_stake_pool(a: address): bool {
       exists<StakePool>(a)
    }
}
```



<a id="0x1_stake_spec_has_validator_config"></a>


```move
module 0x1::stake {
    fun spec_has_validator_config(a: address): bool {
       exists<ValidatorConfig>(a)
    }
}
```



<a id="0x1_stake_spec_rewards_amount"></a>


```move
module 0x1::stake {
    fun spec_rewards_amount(
       stake_amount: u64,
       num_successful_proposals: u64,
       num_total_proposals: u64,
       rewards_rate: u64,
       rewards_rate_denominator: u64,
    ): u64;
}
```



<a id="0x1_stake_spec_contains"></a>


```move
module 0x1::stake {
    fun spec_contains(validators: vector<ValidatorInfo>, addr: address): bool {
       exists i in 0..len(validators): validators[i].addr == addr
    }
}
```



<a id="0x1_stake_spec_is_current_epoch_validator"></a>


```move
module 0x1::stake {
    fun spec_is_current_epoch_validator(pool_address: address): bool {
       let validator_set = global<ValidatorSet>(@aptos_framework);
       !spec_contains(validator_set.pending_active, pool_address)
           && (spec_contains(validator_set.active_validators, pool_address)
           || spec_contains(validator_set.pending_inactive, pool_address))
    }
}
```



<a id="0x1_stake_ResourceRequirement"></a>


```move
module 0x1::stake {
    schema ResourceRequirement {
        requires exists<AptosCoinCapabilities>(@aptos_framework);
        requires exists<ValidatorPerformance>(@aptos_framework);
        requires exists<ValidatorSet>(@aptos_framework);
        requires exists<StakingConfig>(@aptos_framework);
        requires exists<StakingRewardsConfig>(@aptos_framework) || !features::spec_periodical_reward_rate_decrease_enabled();
        requires exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        requires exists<ValidatorFees>(@aptos_framework);
    }
}
```



<a id="0x1_stake_spec_get_reward_rate_1"></a>


```move
module 0x1::stake {
    fun spec_get_reward_rate_1(config: StakingConfig): num {
       if (features::spec_periodical_reward_rate_decrease_enabled()) {
           let epoch_rewards_rate = global<staking_config::StakingRewardsConfig>(@aptos_framework).rewards_rate;
           if (epoch_rewards_rate.value == 0) {
               0
           } else {
               let denominator_0 = aptos_std::fixed_point64::spec_divide_u128(staking_config::MAX_REWARDS_RATE, epoch_rewards_rate);
               let denominator = if (denominator_0 > MAX_U64) {
                   MAX_U64
               } else {
                   denominator_0
               };
               let nominator = aptos_std::fixed_point64::spec_multiply_u128(denominator, epoch_rewards_rate);
               nominator
           }
       } else {
               config.rewards_rate
       }
    }
}
```



<a id="0x1_stake_spec_get_reward_rate_2"></a>


```move
module 0x1::stake {
    fun spec_get_reward_rate_2(config: StakingConfig): num {
       if (features::spec_periodical_reward_rate_decrease_enabled()) {
           let epoch_rewards_rate = global<staking_config::StakingRewardsConfig>(@aptos_framework).rewards_rate;
           if (epoch_rewards_rate.value == 0) {
               1
           } else {
               let denominator_0 = aptos_std::fixed_point64::spec_divide_u128(staking_config::MAX_REWARDS_RATE, epoch_rewards_rate);
               let denominator = if (denominator_0 > MAX_U64) {
                   MAX_U64
               } else {
                   denominator_0
               };
               denominator
           }
       } else {
               config.rewards_rate_denominator
       }
    }
}
```


<a id="@Specification_1_update_stake_pool"></a>

### Function `update_stake_pool`


```move
module 0x1::stake {
    fun update_stake_pool(validator_perf: &stake::ValidatorPerformance, pool_address: address, staking_config: &staking_config::StakingConfig)
}
```



```move
module 0x1::stake {
    pragma verify_duration_estimate = 300;
    include ResourceRequirement;
    include GetReconfigStartTimeRequirement;
    include staking_config::StakingRewardsConfigRequirement;
    include UpdateStakePoolAbortsIf;
    let stake_pool = global<StakePool>(pool_address);
    let validator_config = global<ValidatorConfig>(pool_address);
    let cur_validator_perf = validator_perf.validators[validator_config.validator_index];
    let num_successful_proposals = cur_validator_perf.successful_proposals;
    let num_total_proposals = cur_validator_perf.successful_proposals + cur_validator_perf.failed_proposals;
    let rewards_rate = spec_get_reward_rate_1(staking_config);
    let rewards_rate_denominator = spec_get_reward_rate_2(staking_config);
    let rewards_amount_1 = if (stake_pool.active.value > 0) {
        spec_rewards_amount(stake_pool.active.value, num_successful_proposals, num_total_proposals, rewards_rate, rewards_rate_denominator)
    } else {
        0
    };
    let rewards_amount_2 = if (stake_pool.pending_inactive.value > 0) {
        spec_rewards_amount(stake_pool.pending_inactive.value, num_successful_proposals, num_total_proposals, rewards_rate, rewards_rate_denominator)
    } else {
        0
    };
    let post post_stake_pool = global<StakePool>(pool_address);
    let post post_active_value = post_stake_pool.active.value;
    let post post_pending_inactive_value = post_stake_pool.pending_inactive.value;
    let fees_table = global<ValidatorFees>(@aptos_framework).fees_table;
    let post post_fees_table = global<ValidatorFees>(@aptos_framework).fees_table;
    let post post_inactive_value = post_stake_pool.inactive.value;
    ensures post_stake_pool.pending_active.value == 0;
    ensures if (features::spec_is_enabled(features::COLLECT_AND_DISTRIBUTE_GAS_FEES) && table::spec_contains(fees_table, pool_address)) {
        !table::spec_contains(post_fees_table, pool_address) &&
        post_active_value == stake_pool.active.value + rewards_amount_1 + stake_pool.pending_active.value + table::spec_get(fees_table, pool_address).value
    } else {
        post_active_value == stake_pool.active.value + rewards_amount_1 + stake_pool.pending_active.value
    };
    ensures if (spec_get_reconfig_start_time_secs() >= stake_pool.locked_until_secs) {
        post_pending_inactive_value == 0 &&
        post_inactive_value == stake_pool.inactive.value + stake_pool.pending_inactive.value + rewards_amount_2
    } else {
        post_pending_inactive_value == stake_pool.pending_inactive.value + rewards_amount_2
    };
}
```



<a id="0x1_stake_UpdateStakePoolAbortsIf"></a>


```move
module 0x1::stake {
    schema UpdateStakePoolAbortsIf {
        pool_address: address;
        validator_perf: ValidatorPerformance;
        aborts_if !exists<StakePool>(pool_address);
        aborts_if !exists<ValidatorConfig>(pool_address);
        aborts_if global<ValidatorConfig>(pool_address).validator_index >= len(validator_perf.validators);
        let aptos_addr = type_info::type_of<AptosCoin>().account_address;
        aborts_if !exists<ValidatorFees>(aptos_addr);
        let stake_pool = global<StakePool>(pool_address);
        include DistributeRewardsAbortsIf {stake: stake_pool.active};
        include DistributeRewardsAbortsIf {stake: stake_pool.pending_inactive};
    }
}
```


<a id="@Specification_1_get_reconfig_start_time_secs"></a>

### Function `get_reconfig_start_time_secs`


```move
module 0x1::stake {
    fun get_reconfig_start_time_secs(): u64
}
```



```move
module 0x1::stake {
    include GetReconfigStartTimeRequirement;
}
```



<a id="0x1_stake_GetReconfigStartTimeRequirement"></a>


```move
module 0x1::stake {
    schema GetReconfigStartTimeRequirement {
        requires exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        include reconfiguration_state::StartTimeSecsRequirement;
    }
}
```



<a id="0x1_stake_spec_get_reconfig_start_time_secs"></a>


```move
module 0x1::stake {
    fun spec_get_reconfig_start_time_secs(): u64 {
       if (exists<reconfiguration_state::State>(@aptos_framework)) {
           reconfiguration_state::spec_start_time_secs()
       } else {
           timestamp::spec_now_seconds()
       }
    }
}
```


<a id="@Specification_1_calculate_rewards_amount"></a>

### Function `calculate_rewards_amount`


```move
module 0x1::stake {
    fun calculate_rewards_amount(stake_amount: u64, num_successful_proposals: u64, num_total_proposals: u64, rewards_rate: u64, rewards_rate_denominator: u64): u64
}
```



```move
module 0x1::stake {
    pragma opaque;
    pragma verify_duration_estimate = 300;
    requires rewards_rate <= MAX_REWARDS_RATE;
    requires rewards_rate_denominator > 0;
    requires rewards_rate <= rewards_rate_denominator;
    requires num_successful_proposals <= num_total_proposals;
    ensures [concrete] (rewards_rate_denominator * num_total_proposals == 0) ==> result == 0;
    ensures [concrete] (rewards_rate_denominator * num_total_proposals > 0) ==> {
        let amount = ((stake_amount * rewards_rate * num_successful_proposals) /
            (rewards_rate_denominator * num_total_proposals));
        result == amount
    };
    aborts_if false;
    ensures [abstract] result == spec_rewards_amount(
        stake_amount,
        num_successful_proposals,
        num_total_proposals,
        rewards_rate,
        rewards_rate_denominator);
}
```


<a id="@Specification_1_distribute_rewards"></a>

### Function `distribute_rewards`


```move
module 0x1::stake {
    fun distribute_rewards(stake: &mut coin::Coin<aptos_coin::AptosCoin>, num_successful_proposals: u64, num_total_proposals: u64, rewards_rate: u64, rewards_rate_denominator: u64): u64
}
```



```move
module 0x1::stake {
    include ResourceRequirement;
    requires rewards_rate <= MAX_REWARDS_RATE;
    requires rewards_rate_denominator > 0;
    requires rewards_rate <= rewards_rate_denominator;
    requires num_successful_proposals <= num_total_proposals;
    include DistributeRewardsAbortsIf;
    ensures old(stake.value) > 0 ==>
        result == spec_rewards_amount(
            old(stake.value),
            num_successful_proposals,
            num_total_proposals,
            rewards_rate,
            rewards_rate_denominator);
    ensures old(stake.value) > 0 ==>
        stake.value == old(stake.value) + spec_rewards_amount(
            old(stake.value),
            num_successful_proposals,
            num_total_proposals,
            rewards_rate,
            rewards_rate_denominator);
    ensures old(stake.value) == 0 ==> result == 0;
    ensures old(stake.value) == 0 ==> stake.value == old(stake.value);
}
```



<a id="0x1_stake_DistributeRewardsAbortsIf"></a>


```move
module 0x1::stake {
    schema DistributeRewardsAbortsIf {
        stake: Coin<AptosCoin>;
        num_successful_proposals: num;
        num_total_proposals: num;
        rewards_rate: num;
        rewards_rate_denominator: num;
        let stake_amount = coin::value(stake);
        let rewards_amount = if (stake_amount > 0) {
            spec_rewards_amount(stake_amount, num_successful_proposals, num_total_proposals, rewards_rate, rewards_rate_denominator)
        } else {
            0
        };
        let amount = rewards_amount;
        let addr = type_info::type_of<AptosCoin>().account_address;
        aborts_if (rewards_amount > 0) && !exists<coin::CoinInfo<AptosCoin>>(addr);
        modifies global<coin::CoinInfo<AptosCoin>>(addr);
        include (rewards_amount > 0) ==> coin::CoinAddAbortsIf<AptosCoin> { amount: amount };
    }
}
```


<a id="@Specification_1_append"></a>

### Function `append`


```move
module 0x1::stake {
    fun append<T>(v1: &mut vector<T>, v2: &mut vector<T>)
}
```



```move
module 0x1::stake {
    pragma opaque, verify = false;
    aborts_if false;
    ensures len(v1) == old(len(v1) + len(v2));
    ensures len(v2) == 0;
    ensures (forall i in 0..old(len(v1)): v1[i] == old(v1[i]));
    ensures (forall i in old(len(v1))..len(v1): v1[i] == old(v2[len(v2) - (i - len(v1)) - 1]));
}
```


<a id="@Specification_1_find_validator"></a>

### Function `find_validator`


```move
module 0x1::stake {
    fun find_validator(v: &vector<stake::ValidatorInfo>, addr: address): option::Option<u64>
}
```



```move
module 0x1::stake {
    pragma opaque;
    aborts_if false;
    ensures option::is_none(result) ==> (forall i in 0..len(v): v[i].addr != addr);
    ensures option::is_some(result) ==> v[option::borrow(result)].addr == addr;
    ensures option::is_some(result) ==> spec_contains(v, addr);
    ensures [abstract] result == spec_find_validator(v,addr);
}
```


<a id="@Specification_1_update_voting_power_increase"></a>

### Function `update_voting_power_increase`


```move
module 0x1::stake {
    fun update_voting_power_increase(increase_amount: u64)
}
```



```move
module 0x1::stake {
    requires !reconfiguration_state::spec_is_in_progress();
    aborts_if !exists<ValidatorSet>(@aptos_framework);
    aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
    let aptos = @aptos_framework;
    let pre_validator_set = global<ValidatorSet>(aptos);
    let post validator_set = global<ValidatorSet>(aptos);
    let staking_config = global<staking_config::StakingConfig>(aptos);
    let voting_power_increase_limit = staking_config.voting_power_increase_limit;
    aborts_if pre_validator_set.total_joining_power + increase_amount > MAX_U128;
    aborts_if pre_validator_set.total_voting_power > 0 && pre_validator_set.total_voting_power * voting_power_increase_limit > MAX_U128;
    aborts_if pre_validator_set.total_voting_power > 0 &&
        pre_validator_set.total_joining_power + increase_amount > pre_validator_set.total_voting_power * voting_power_increase_limit / 100;
    ensures validator_set.total_voting_power > 0 ==>
        validator_set.total_joining_power <= validator_set.total_voting_power * voting_power_increase_limit / 100;
    ensures validator_set.total_joining_power == pre_validator_set.total_joining_power + increase_amount;
}
```


<a id="@Specification_1_assert_stake_pool_exists"></a>

### Function `assert_stake_pool_exists`


```move
module 0x1::stake {
    fun assert_stake_pool_exists(pool_address: address)
}
```



```move
module 0x1::stake {
    aborts_if !stake_pool_exists(pool_address);
}
```


<a id="@Specification_1_configure_allowed_validators"></a>

### Function `configure_allowed_validators`


```move
module 0x1::stake {
    public fun configure_allowed_validators(aptos_framework: &signer, accounts: vector<address>)
}
```



```move
module 0x1::stake {
    let aptos_framework_address = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_framework_address);
    let post allowed = global<AllowedValidators>(aptos_framework_address);
    ensures allowed.accounts == accounts;
}
```


<a id="@Specification_1_assert_owner_cap_exists"></a>

### Function `assert_owner_cap_exists`


```move
module 0x1::stake {
    fun assert_owner_cap_exists(owner: address)
}
```



```move
module 0x1::stake {
    aborts_if !exists<OwnerCapability>(owner);
}
```
