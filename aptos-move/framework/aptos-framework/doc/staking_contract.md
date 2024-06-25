
<a id="0x1_staking_contract"></a>

# Module `0x1::staking_contract`

Allow stakers and operators to enter a staking contract with reward sharing.
The main accounting logic in a staking contract consists of 2 parts:
1. Tracks how much commission needs to be paid out to the operator. This is tracked with an increasing principal
amount that&apos;s updated every time the operator requests commission, the staker withdraws funds, or the staker
switches operators.
2. Distributions of funds to operators (commissions) and stakers (stake withdrawals) use the shares model provided
by the pool_u64 to track shares that increase in price as the stake pool accumulates rewards.

Example flow:
1. A staker creates a staking contract with an operator by calling create_staking_contract() with 100 coins of
initial stake and commission &#61; 10%. This means the operator will receive 10% of any accumulated rewards. A new stake
pool will be created and hosted in a separate account that&apos;s controlled by the staking contract.
2. The operator sets up a validator node and, once ready, joins the validator set by calling stake::join_validator_set
3. After some time, the stake pool gains rewards and now has 150 coins.
4. Operator can now call request_commission. 10% of (150 &#45; 100) &#61; 5 coins will be unlocked from the stake pool. The
staker&apos;s principal is now updated from 100 to 145 (150 coins &#45; 5 coins of commission). The pending distribution pool
has 5 coins total and the operator owns all 5 shares of it.
5. Some more time has passed. The pool now has 50 more coins in rewards and a total balance of 195. The operator
calls request_commission again. Since the previous 5 coins have now become withdrawable, it&apos;ll be deposited into the
operator&apos;s account first. Their new commission will be 10% of (195 coins &#45; 145 principal) &#61; 5 coins. Principal is
updated to be 190 (195 &#45; 5). Pending distribution pool has 5 coins and operator owns all 5 shares.
6. Staker calls unlock_stake to unlock 50 coins of stake, which gets added to the pending distribution pool. Based
on shares math, staker will be owning 50 shares and operator still owns 5 shares of the 55&#45;coin pending distribution
pool.
7. Some time passes and the 55 coins become fully withdrawable from the stake pool. Due to accumulated rewards, the
55 coins become 70 coins. Calling distribute() distributes 6 coins to the operator and 64 coins to the validator.


-  [Struct `StakingGroupContainer`](#0x1_staking_contract_StakingGroupContainer)
-  [Struct `StakingContract`](#0x1_staking_contract_StakingContract)
-  [Resource `Store`](#0x1_staking_contract_Store)
-  [Resource `BeneficiaryForOperator`](#0x1_staking_contract_BeneficiaryForOperator)
-  [Struct `UpdateCommissionEvent`](#0x1_staking_contract_UpdateCommissionEvent)
-  [Struct `UpdateCommission`](#0x1_staking_contract_UpdateCommission)
-  [Resource `StakingGroupUpdateCommissionEvent`](#0x1_staking_contract_StakingGroupUpdateCommissionEvent)
-  [Struct `CreateStakingContract`](#0x1_staking_contract_CreateStakingContract)
-  [Struct `UpdateVoter`](#0x1_staking_contract_UpdateVoter)
-  [Struct `ResetLockup`](#0x1_staking_contract_ResetLockup)
-  [Struct `AddStake`](#0x1_staking_contract_AddStake)
-  [Struct `RequestCommission`](#0x1_staking_contract_RequestCommission)
-  [Struct `UnlockStake`](#0x1_staking_contract_UnlockStake)
-  [Struct `SwitchOperator`](#0x1_staking_contract_SwitchOperator)
-  [Struct `AddDistribution`](#0x1_staking_contract_AddDistribution)
-  [Struct `Distribute`](#0x1_staking_contract_Distribute)
-  [Struct `SetBeneficiaryForOperator`](#0x1_staking_contract_SetBeneficiaryForOperator)
-  [Struct `CreateStakingContractEvent`](#0x1_staking_contract_CreateStakingContractEvent)
-  [Struct `UpdateVoterEvent`](#0x1_staking_contract_UpdateVoterEvent)
-  [Struct `ResetLockupEvent`](#0x1_staking_contract_ResetLockupEvent)
-  [Struct `AddStakeEvent`](#0x1_staking_contract_AddStakeEvent)
-  [Struct `RequestCommissionEvent`](#0x1_staking_contract_RequestCommissionEvent)
-  [Struct `UnlockStakeEvent`](#0x1_staking_contract_UnlockStakeEvent)
-  [Struct `SwitchOperatorEvent`](#0x1_staking_contract_SwitchOperatorEvent)
-  [Struct `AddDistributionEvent`](#0x1_staking_contract_AddDistributionEvent)
-  [Struct `DistributeEvent`](#0x1_staking_contract_DistributeEvent)
-  [Constants](#@Constants_0)
-  [Function `stake_pool_address`](#0x1_staking_contract_stake_pool_address)
-  [Function `last_recorded_principal`](#0x1_staking_contract_last_recorded_principal)
-  [Function `commission_percentage`](#0x1_staking_contract_commission_percentage)
-  [Function `staking_contract_amounts`](#0x1_staking_contract_staking_contract_amounts)
-  [Function `pending_distribution_counts`](#0x1_staking_contract_pending_distribution_counts)
-  [Function `staking_contract_exists`](#0x1_staking_contract_staking_contract_exists)
-  [Function `beneficiary_for_operator`](#0x1_staking_contract_beneficiary_for_operator)
-  [Function `get_expected_stake_pool_address`](#0x1_staking_contract_get_expected_stake_pool_address)
-  [Function `create_staking_contract`](#0x1_staking_contract_create_staking_contract)
-  [Function `create_staking_contract_with_coins`](#0x1_staking_contract_create_staking_contract_with_coins)
-  [Function `add_stake`](#0x1_staking_contract_add_stake)
-  [Function `update_voter`](#0x1_staking_contract_update_voter)
-  [Function `reset_lockup`](#0x1_staking_contract_reset_lockup)
-  [Function `update_commision`](#0x1_staking_contract_update_commision)
-  [Function `request_commission`](#0x1_staking_contract_request_commission)
-  [Function `request_commission_internal`](#0x1_staking_contract_request_commission_internal)
-  [Function `unlock_stake`](#0x1_staking_contract_unlock_stake)
-  [Function `unlock_rewards`](#0x1_staking_contract_unlock_rewards)
-  [Function `switch_operator_with_same_commission`](#0x1_staking_contract_switch_operator_with_same_commission)
-  [Function `switch_operator`](#0x1_staking_contract_switch_operator)
-  [Function `set_beneficiary_for_operator`](#0x1_staking_contract_set_beneficiary_for_operator)
-  [Function `distribute`](#0x1_staking_contract_distribute)
-  [Function `distribute_internal`](#0x1_staking_contract_distribute_internal)
-  [Function `assert_staking_contract_exists`](#0x1_staking_contract_assert_staking_contract_exists)
-  [Function `add_distribution`](#0x1_staking_contract_add_distribution)
-  [Function `get_staking_contract_amounts_internal`](#0x1_staking_contract_get_staking_contract_amounts_internal)
-  [Function `create_stake_pool`](#0x1_staking_contract_create_stake_pool)
-  [Function `update_distribution_pool`](#0x1_staking_contract_update_distribution_pool)
-  [Function `create_resource_account_seed`](#0x1_staking_contract_create_resource_account_seed)
-  [Function `new_staking_contracts_holder`](#0x1_staking_contract_new_staking_contracts_holder)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `stake_pool_address`](#@Specification_1_stake_pool_address)
    -  [Function `last_recorded_principal`](#@Specification_1_last_recorded_principal)
    -  [Function `commission_percentage`](#@Specification_1_commission_percentage)
    -  [Function `staking_contract_amounts`](#@Specification_1_staking_contract_amounts)
    -  [Function `pending_distribution_counts`](#@Specification_1_pending_distribution_counts)
    -  [Function `staking_contract_exists`](#@Specification_1_staking_contract_exists)
    -  [Function `beneficiary_for_operator`](#@Specification_1_beneficiary_for_operator)
    -  [Function `create_staking_contract`](#@Specification_1_create_staking_contract)
    -  [Function `create_staking_contract_with_coins`](#@Specification_1_create_staking_contract_with_coins)
    -  [Function `add_stake`](#@Specification_1_add_stake)
    -  [Function `update_voter`](#@Specification_1_update_voter)
    -  [Function `reset_lockup`](#@Specification_1_reset_lockup)
    -  [Function `update_commision`](#@Specification_1_update_commision)
    -  [Function `request_commission`](#@Specification_1_request_commission)
    -  [Function `request_commission_internal`](#@Specification_1_request_commission_internal)
    -  [Function `unlock_stake`](#@Specification_1_unlock_stake)
    -  [Function `unlock_rewards`](#@Specification_1_unlock_rewards)
    -  [Function `switch_operator_with_same_commission`](#@Specification_1_switch_operator_with_same_commission)
    -  [Function `switch_operator`](#@Specification_1_switch_operator)
    -  [Function `set_beneficiary_for_operator`](#@Specification_1_set_beneficiary_for_operator)
    -  [Function `distribute`](#@Specification_1_distribute)
    -  [Function `distribute_internal`](#@Specification_1_distribute_internal)
    -  [Function `assert_staking_contract_exists`](#@Specification_1_assert_staking_contract_exists)
    -  [Function `add_distribution`](#@Specification_1_add_distribution)
    -  [Function `get_staking_contract_amounts_internal`](#@Specification_1_get_staking_contract_amounts_internal)
    -  [Function `create_stake_pool`](#@Specification_1_create_stake_pool)
    -  [Function `update_distribution_pool`](#@Specification_1_update_distribution_pool)
    -  [Function `new_staking_contracts_holder`](#@Specification_1_new_staking_contracts_holder)


```move
module 0x1::staking_contract {
    use 0x1::account;
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::bcs;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::pool_u64;
    use 0x1::signer;
    use 0x1::simple_map;
    use 0x1::stake;
    use 0x1::staking_config;
    use 0x1::vector;
}
```


<a id="0x1_staking_contract_StakingGroupContainer"></a>

## Struct `StakingGroupContainer`



```move
module 0x1::staking_contract {
    #[resource_group(#[scope = module_])]
    struct StakingGroupContainer
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


<a id="0x1_staking_contract_StakingContract"></a>

## Struct `StakingContract`



```move
module 0x1::staking_contract {
    struct StakingContract has store
}
```


##### Fields


<dl>
<dt>
`principal: u64`
</dt>
<dd>

</dd>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`owner_cap: stake::OwnerCapability`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
<dt>
`distribution_pool: pool_u64::Pool`
</dt>
<dd>

</dd>
<dt>
`signer_cap: account::SignerCapability`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_Store"></a>

## Resource `Store`



```move
module 0x1::staking_contract {
    struct Store has key
}
```


##### Fields


<dl>
<dt>
`staking_contracts: simple_map::SimpleMap<address, staking_contract::StakingContract>`
</dt>
<dd>

</dd>
<dt>
`create_staking_contract_events: event::EventHandle<staking_contract::CreateStakingContractEvent>`
</dt>
<dd>

</dd>
<dt>
`update_voter_events: event::EventHandle<staking_contract::UpdateVoterEvent>`
</dt>
<dd>

</dd>
<dt>
`reset_lockup_events: event::EventHandle<staking_contract::ResetLockupEvent>`
</dt>
<dd>

</dd>
<dt>
`add_stake_events: event::EventHandle<staking_contract::AddStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`request_commission_events: event::EventHandle<staking_contract::RequestCommissionEvent>`
</dt>
<dd>

</dd>
<dt>
`unlock_stake_events: event::EventHandle<staking_contract::UnlockStakeEvent>`
</dt>
<dd>

</dd>
<dt>
`switch_operator_events: event::EventHandle<staking_contract::SwitchOperatorEvent>`
</dt>
<dd>

</dd>
<dt>
`add_distribution_events: event::EventHandle<staking_contract::AddDistributionEvent>`
</dt>
<dd>

</dd>
<dt>
`distribute_events: event::EventHandle<staking_contract::DistributeEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_BeneficiaryForOperator"></a>

## Resource `BeneficiaryForOperator`



```move
module 0x1::staking_contract {
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


<a id="0x1_staking_contract_UpdateCommissionEvent"></a>

## Struct `UpdateCommissionEvent`



```move
module 0x1::staking_contract {
    struct UpdateCommissionEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`staker: address`
</dt>
<dd>

</dd>
<dt>
`operator: address`
</dt>
<dd>

</dd>
<dt>
`old_commission_percentage: u64`
</dt>
<dd>

</dd>
<dt>
`new_commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_UpdateCommission"></a>

## Struct `UpdateCommission`



```move
module 0x1::staking_contract {
    #[event]
    struct UpdateCommission has drop, store
}
```


##### Fields


<dl>
<dt>
`staker: address`
</dt>
<dd>

</dd>
<dt>
`operator: address`
</dt>
<dd>

</dd>
<dt>
`old_commission_percentage: u64`
</dt>
<dd>

</dd>
<dt>
`new_commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_StakingGroupUpdateCommissionEvent"></a>

## Resource `StakingGroupUpdateCommissionEvent`



```move
module 0x1::staking_contract {
    #[resource_group_member(#[group = 0x1::staking_contract::StakingGroupContainer])]
    struct StakingGroupUpdateCommissionEvent has key
}
```


##### Fields


<dl>
<dt>
`update_commission_events: event::EventHandle<staking_contract::UpdateCommissionEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_CreateStakingContract"></a>

## Struct `CreateStakingContract`



```move
module 0x1::staking_contract {
    #[event]
    struct CreateStakingContract has drop, store
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
`voter: address`
</dt>
<dd>

</dd>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`principal: u64`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_UpdateVoter"></a>

## Struct `UpdateVoter`



```move
module 0x1::staking_contract {
    #[event]
    struct UpdateVoter has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`old_voter: address`
</dt>
<dd>

</dd>
<dt>
`new_voter: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_ResetLockup"></a>

## Struct `ResetLockup`



```move
module 0x1::staking_contract {
    #[event]
    struct ResetLockup has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_AddStake"></a>

## Struct `AddStake`



```move
module 0x1::staking_contract {
    #[event]
    struct AddStake has drop, store
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


<a id="0x1_staking_contract_RequestCommission"></a>

## Struct `RequestCommission`



```move
module 0x1::staking_contract {
    #[event]
    struct RequestCommission has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`accumulated_rewards: u64`
</dt>
<dd>

</dd>
<dt>
`commission_amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_UnlockStake"></a>

## Struct `UnlockStake`



```move
module 0x1::staking_contract {
    #[event]
    struct UnlockStake has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
<dt>
`commission_paid: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_SwitchOperator"></a>

## Struct `SwitchOperator`



```move
module 0x1::staking_contract {
    #[event]
    struct SwitchOperator has drop, store
}
```


##### Fields


<dl>
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
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_AddDistribution"></a>

## Struct `AddDistribution`



```move
module 0x1::staking_contract {
    #[event]
    struct AddDistribution has drop, store
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


<a id="0x1_staking_contract_Distribute"></a>

## Struct `Distribute`



```move
module 0x1::staking_contract {
    #[event]
    struct Distribute has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`recipient: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_SetBeneficiaryForOperator"></a>

## Struct `SetBeneficiaryForOperator`



```move
module 0x1::staking_contract {
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


<a id="0x1_staking_contract_CreateStakingContractEvent"></a>

## Struct `CreateStakingContractEvent`



```move
module 0x1::staking_contract {
    struct CreateStakingContractEvent has drop, store
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
`voter: address`
</dt>
<dd>

</dd>
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`principal: u64`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_UpdateVoterEvent"></a>

## Struct `UpdateVoterEvent`



```move
module 0x1::staking_contract {
    struct UpdateVoterEvent has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`old_voter: address`
</dt>
<dd>

</dd>
<dt>
`new_voter: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_ResetLockupEvent"></a>

## Struct `ResetLockupEvent`



```move
module 0x1::staking_contract {
    struct ResetLockupEvent has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_AddStakeEvent"></a>

## Struct `AddStakeEvent`



```move
module 0x1::staking_contract {
    struct AddStakeEvent has drop, store
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


<a id="0x1_staking_contract_RequestCommissionEvent"></a>

## Struct `RequestCommissionEvent`



```move
module 0x1::staking_contract {
    struct RequestCommissionEvent has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`accumulated_rewards: u64`
</dt>
<dd>

</dd>
<dt>
`commission_amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_UnlockStakeEvent"></a>

## Struct `UnlockStakeEvent`



```move
module 0x1::staking_contract {
    struct UnlockStakeEvent has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
<dt>
`commission_paid: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_SwitchOperatorEvent"></a>

## Struct `SwitchOperatorEvent`



```move
module 0x1::staking_contract {
    struct SwitchOperatorEvent has drop, store
}
```


##### Fields


<dl>
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
<dt>
`pool_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_contract_AddDistributionEvent"></a>

## Struct `AddDistributionEvent`



```move
module 0x1::staking_contract {
    struct AddDistributionEvent has drop, store
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


<a id="0x1_staking_contract_DistributeEvent"></a>

## Struct `DistributeEvent`



```move
module 0x1::staking_contract {
    struct DistributeEvent has drop, store
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
`pool_address: address`
</dt>
<dd>

</dd>
<dt>
`recipient: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_staking_contract_EINVALID_COMMISSION_PERCENTAGE"></a>

Commission percentage has to be between 0 and 100.


```move
module 0x1::staking_contract {
    const EINVALID_COMMISSION_PERCENTAGE: u64 = 2;
}
```


<a id="0x1_staking_contract_EOPERATOR_BENEFICIARY_CHANGE_NOT_SUPPORTED"></a>

Chaning beneficiaries for operators is not supported.


```move
module 0x1::staking_contract {
    const EOPERATOR_BENEFICIARY_CHANGE_NOT_SUPPORTED: u64 = 9;
}
```


<a id="0x1_staking_contract_ECANT_MERGE_STAKING_CONTRACTS"></a>

Staking contracts can&apos;t be merged.


```move
module 0x1::staking_contract {
    const ECANT_MERGE_STAKING_CONTRACTS: u64 = 5;
}
```


<a id="0x1_staking_contract_EINSUFFICIENT_ACTIVE_STAKE_TO_WITHDRAW"></a>

Not enough active stake to withdraw. Some stake might still pending and will be active in the next epoch.


```move
module 0x1::staking_contract {
    const EINSUFFICIENT_ACTIVE_STAKE_TO_WITHDRAW: u64 = 7;
}
```


<a id="0x1_staking_contract_EINSUFFICIENT_STAKE_AMOUNT"></a>

Store amount must be at least the min stake required for a stake pool to join the validator set.


```move
module 0x1::staking_contract {
    const EINSUFFICIENT_STAKE_AMOUNT: u64 = 1;
}
```


<a id="0x1_staking_contract_ENOT_STAKER_OR_OPERATOR_OR_BENEFICIARY"></a>

Caller must be either the staker, operator, or beneficiary.


```move
module 0x1::staking_contract {
    const ENOT_STAKER_OR_OPERATOR_OR_BENEFICIARY: u64 = 8;
}
```


<a id="0x1_staking_contract_ENO_STAKING_CONTRACT_FOUND_FOR_OPERATOR"></a>

No staking contract between the staker and operator found.


```move
module 0x1::staking_contract {
    const ENO_STAKING_CONTRACT_FOUND_FOR_OPERATOR: u64 = 4;
}
```


<a id="0x1_staking_contract_ENO_STAKING_CONTRACT_FOUND_FOR_STAKER"></a>

Staker has no staking contracts.


```move
module 0x1::staking_contract {
    const ENO_STAKING_CONTRACT_FOUND_FOR_STAKER: u64 = 3;
}
```


<a id="0x1_staking_contract_ESTAKING_CONTRACT_ALREADY_EXISTS"></a>

The staking contract already exists and cannot be re&#45;created.


```move
module 0x1::staking_contract {
    const ESTAKING_CONTRACT_ALREADY_EXISTS: u64 = 6;
}
```


<a id="0x1_staking_contract_MAXIMUM_PENDING_DISTRIBUTIONS"></a>

Maximum number of distributions a stake pool can support.


```move
module 0x1::staking_contract {
    const MAXIMUM_PENDING_DISTRIBUTIONS: u64 = 20;
}
```


<a id="0x1_staking_contract_SALT"></a>



```move
module 0x1::staking_contract {
    const SALT: vector<u8> = [97, 112, 116, 111, 115, 95, 102, 114, 97, 109, 101, 119, 111, 114, 107, 58, 58, 115, 116, 97, 107, 105, 110, 103, 95, 99, 111, 110, 116, 114, 97, 99, 116];
}
```


<a id="0x1_staking_contract_stake_pool_address"></a>

## Function `stake_pool_address`

Return the address of the underlying stake pool for the staking contract between the provided staker and
operator.

This errors out the staking contract with the provided staker and operator doesn&apos;t exist.


```move
module 0x1::staking_contract {
    #[view]
    public fun stake_pool_address(staker: address, operator: address): address
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun stake_pool_address(staker: address, operator: address): address acquires Store {
        assert_staking_contract_exists(staker, operator);
        let staking_contracts = &borrow_global<Store>(staker).staking_contracts;
        simple_map::borrow(staking_contracts, &operator).pool_address
    }
}
```


<a id="0x1_staking_contract_last_recorded_principal"></a>

## Function `last_recorded_principal`

Return the last recorded principal (the amount that 100% belongs to the staker with commission already paid for)
for staking contract between the provided staker and operator.

This errors out the staking contract with the provided staker and operator doesn&apos;t exist.


```move
module 0x1::staking_contract {
    #[view]
    public fun last_recorded_principal(staker: address, operator: address): u64
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun last_recorded_principal(staker: address, operator: address): u64 acquires Store {
        assert_staking_contract_exists(staker, operator);
        let staking_contracts = &borrow_global<Store>(staker).staking_contracts;
        simple_map::borrow(staking_contracts, &operator).principal
    }
}
```


<a id="0x1_staking_contract_commission_percentage"></a>

## Function `commission_percentage`

Return percentage of accumulated rewards that will be paid to the operator as commission for staking contract
between the provided staker and operator.

This errors out the staking contract with the provided staker and operator doesn&apos;t exist.


```move
module 0x1::staking_contract {
    #[view]
    public fun commission_percentage(staker: address, operator: address): u64
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun commission_percentage(staker: address, operator: address): u64 acquires Store {
        assert_staking_contract_exists(staker, operator);
        let staking_contracts = &borrow_global<Store>(staker).staking_contracts;
        simple_map::borrow(staking_contracts, &operator).commission_percentage
    }
}
```


<a id="0x1_staking_contract_staking_contract_amounts"></a>

## Function `staking_contract_amounts`

Return a tuple of three numbers:
1. The total active stake in the underlying stake pool
2. The total accumulated rewards that haven&apos;t had commission paid out
3. The commission amount owned from those accumulated rewards.

This errors out the staking contract with the provided staker and operator doesn&apos;t exist.


```move
module 0x1::staking_contract {
    #[view]
    public fun staking_contract_amounts(staker: address, operator: address): (u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun staking_contract_amounts(staker: address, operator: address): (u64, u64, u64) acquires Store {
        assert_staking_contract_exists(staker, operator);
        let staking_contracts = &borrow_global<Store>(staker).staking_contracts;
        let staking_contract = simple_map::borrow(staking_contracts, &operator);
        get_staking_contract_amounts_internal(staking_contract)
    }
}
```


<a id="0x1_staking_contract_pending_distribution_counts"></a>

## Function `pending_distribution_counts`

Return the number of pending distributions (e.g. commission, withdrawals from stakers).

This errors out the staking contract with the provided staker and operator doesn&apos;t exist.


```move
module 0x1::staking_contract {
    #[view]
    public fun pending_distribution_counts(staker: address, operator: address): u64
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun pending_distribution_counts(staker: address, operator: address): u64 acquires Store {
        assert_staking_contract_exists(staker, operator);
        let staking_contracts = &borrow_global<Store>(staker).staking_contracts;
        pool_u64::shareholders_count(&simple_map::borrow(staking_contracts, &operator).distribution_pool)
    }
}
```


<a id="0x1_staking_contract_staking_contract_exists"></a>

## Function `staking_contract_exists`

Return true if the staking contract between the provided staker and operator exists.


```move
module 0x1::staking_contract {
    #[view]
    public fun staking_contract_exists(staker: address, operator: address): bool
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun staking_contract_exists(staker: address, operator: address): bool acquires Store {
        if (!exists<Store>(staker)) {
            return false
        };

        let store = borrow_global<Store>(staker);
        simple_map::contains_key(&store.staking_contracts, &operator)
    }
}
```


<a id="0x1_staking_contract_beneficiary_for_operator"></a>

## Function `beneficiary_for_operator`

Return the beneficiary address of the operator.


```move
module 0x1::staking_contract {
    #[view]
    public fun beneficiary_for_operator(operator: address): address
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun beneficiary_for_operator(operator: address): address acquires BeneficiaryForOperator {
        if (exists<BeneficiaryForOperator>(operator)) {
            return borrow_global<BeneficiaryForOperator>(operator).beneficiary_for_operator
        } else {
            operator
        }
    }
}
```


<a id="0x1_staking_contract_get_expected_stake_pool_address"></a>

## Function `get_expected_stake_pool_address`

Return the address of the stake pool to be created with the provided staker, operator and seed.


```move
module 0x1::staking_contract {
    #[view]
    public fun get_expected_stake_pool_address(staker: address, operator: address, contract_creation_seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun get_expected_stake_pool_address(
        staker: address,
        operator: address,
        contract_creation_seed: vector<u8>,
    ): address {
        let seed = create_resource_account_seed(staker, operator, contract_creation_seed);
        account::create_resource_address(&staker, seed)
    }
}
```


<a id="0x1_staking_contract_create_staking_contract"></a>

## Function `create_staking_contract`

Staker can call this function to create a simple staking contract with a specified operator.


```move
module 0x1::staking_contract {
    public entry fun create_staking_contract(staker: &signer, operator: address, voter: address, amount: u64, commission_percentage: u64, contract_creation_seed: vector<u8>)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun create_staking_contract(
        staker: &signer,
        operator: address,
        voter: address,
        amount: u64,
        commission_percentage: u64,
        // Optional seed used when creating the staking contract account.
        contract_creation_seed: vector<u8>,
    ) acquires Store {
        let staked_coins = coin::withdraw<AptosCoin>(staker, amount);
        create_staking_contract_with_coins(
            staker, operator, voter, staked_coins, commission_percentage, contract_creation_seed);
    }
}
```


<a id="0x1_staking_contract_create_staking_contract_with_coins"></a>

## Function `create_staking_contract_with_coins`

Staker can call this function to create a simple staking contract with a specified operator.


```move
module 0x1::staking_contract {
    public fun create_staking_contract_with_coins(staker: &signer, operator: address, voter: address, coins: coin::Coin<aptos_coin::AptosCoin>, commission_percentage: u64, contract_creation_seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public fun create_staking_contract_with_coins(
        staker: &signer,
        operator: address,
        voter: address,
        coins: Coin<AptosCoin>,
        commission_percentage: u64,
        // Optional seed used when creating the staking contract account.
        contract_creation_seed: vector<u8>,
    ): address acquires Store {
        assert!(
            commission_percentage >= 0 && commission_percentage <= 100,
            error::invalid_argument(EINVALID_COMMISSION_PERCENTAGE),
        );
        // The amount should be at least the min_stake_required, so the stake pool will be eligible to join the
        // validator set.
        let (min_stake_required, _) = staking_config::get_required_stake(&staking_config::get());
        let principal = coin::value(&coins);
        assert!(principal >= min_stake_required, error::invalid_argument(EINSUFFICIENT_STAKE_AMOUNT));

        // Initialize Store resource if this is the first time the staker has delegated to anyone.
        let staker_address = signer::address_of(staker);
        if (!exists<Store>(staker_address)) {
            move_to(staker, new_staking_contracts_holder(staker));
        };

        // Cannot create the staking contract if it already exists.
        let store = borrow_global_mut<Store>(staker_address);
        let staking_contracts = &mut store.staking_contracts;
        assert!(
            !simple_map::contains_key(staking_contracts, &operator),
            error::already_exists(ESTAKING_CONTRACT_ALREADY_EXISTS)
        );

        // Initialize the stake pool in a new resource account. This allows the same staker to contract with multiple
        // different operators.
        let (stake_pool_signer, stake_pool_signer_cap, owner_cap) =
            create_stake_pool(staker, operator, voter, contract_creation_seed);

        // Add the stake to the stake pool.
        stake::add_stake_with_cap(&owner_cap, coins);

        // Create the contract record.
        let pool_address = signer::address_of(&stake_pool_signer);
        simple_map::add(staking_contracts, operator, StakingContract {
            principal,
            pool_address,
            owner_cap,
            commission_percentage,
            // Make sure we don't have too many pending recipients in the distribution pool.
            // Otherwise, a griefing attack is possible where the staker can keep switching operators and create too
            // many pending distributions. This can lead to out-of-gas failure whenever distribute() is called.
            distribution_pool: pool_u64::create(MAXIMUM_PENDING_DISTRIBUTIONS),
            signer_cap: stake_pool_signer_cap,
        });

        if (std::features::module_event_migration_enabled()) {
            emit(CreateStakingContract { operator, voter, pool_address, principal, commission_percentage });
        };
        emit_event(
            &mut store.create_staking_contract_events,
            CreateStakingContractEvent { operator, voter, pool_address, principal, commission_percentage },
        );
        pool_address
    }
}
```


<a id="0x1_staking_contract_add_stake"></a>

## Function `add_stake`

Add more stake to an existing staking contract.


```move
module 0x1::staking_contract {
    public entry fun add_stake(staker: &signer, operator: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun add_stake(staker: &signer, operator: address, amount: u64) acquires Store {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, operator);

        let store = borrow_global_mut<Store>(staker_address);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);

        // Add the stake to the stake pool.
        let staked_coins = coin::withdraw<AptosCoin>(staker, amount);
        stake::add_stake_with_cap(&staking_contract.owner_cap, staked_coins);

        staking_contract.principal = staking_contract.principal + amount;
        let pool_address = staking_contract.pool_address;
        if (std::features::module_event_migration_enabled()) {
            emit(AddStake { operator, pool_address, amount });
        };
        emit_event(
            &mut store.add_stake_events,
            AddStakeEvent { operator, pool_address, amount },
        );
    }
}
```


<a id="0x1_staking_contract_update_voter"></a>

## Function `update_voter`

Convenient function to allow the staker to update the voter address in a staking contract they made.


```move
module 0x1::staking_contract {
    public entry fun update_voter(staker: &signer, operator: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun update_voter(staker: &signer, operator: address, new_voter: address) acquires Store {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, operator);

        let store = borrow_global_mut<Store>(staker_address);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);
        let pool_address = staking_contract.pool_address;
        let old_voter = stake::get_delegated_voter(pool_address);
        stake::set_delegated_voter_with_cap(&staking_contract.owner_cap, new_voter);

        if (std::features::module_event_migration_enabled()) {
            emit(UpdateVoter { operator, pool_address, old_voter, new_voter });
        };
        emit_event(
            &mut store.update_voter_events,
            UpdateVoterEvent { operator, pool_address, old_voter, new_voter },
        );

    }
}
```


<a id="0x1_staking_contract_reset_lockup"></a>

## Function `reset_lockup`

Convenient function to allow the staker to reset their stake pool&apos;s lockup period to start now.


```move
module 0x1::staking_contract {
    public entry fun reset_lockup(staker: &signer, operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun reset_lockup(staker: &signer, operator: address) acquires Store {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, operator);

        let store = borrow_global_mut<Store>(staker_address);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);
        let pool_address = staking_contract.pool_address;
        stake::increase_lockup_with_cap(&staking_contract.owner_cap);

        if (std::features::module_event_migration_enabled()) {
            emit(ResetLockup { operator, pool_address });
        };
        emit_event(&mut store.reset_lockup_events, ResetLockupEvent { operator, pool_address });
    }
}
```


<a id="0x1_staking_contract_update_commision"></a>

## Function `update_commision`

Convenience function to allow a staker to update the commission percentage paid to the operator.
TODO: fix the typo in function name. commision &#45;&gt; commission


```move
module 0x1::staking_contract {
    public entry fun update_commision(staker: &signer, operator: address, new_commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun update_commision(
        staker: &signer,
        operator: address,
        new_commission_percentage: u64
    ) acquires Store, BeneficiaryForOperator, StakingGroupUpdateCommissionEvent {
        assert!(
            new_commission_percentage >= 0 && new_commission_percentage <= 100,
            error::invalid_argument(EINVALID_COMMISSION_PERCENTAGE),
        );

        let staker_address = signer::address_of(staker);
        assert!(exists<Store>(staker_address), error::not_found(ENO_STAKING_CONTRACT_FOUND_FOR_STAKER));

        let store = borrow_global_mut<Store>(staker_address);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);
        distribute_internal(staker_address, operator, staking_contract, &mut store.distribute_events);
        request_commission_internal(
            operator,
            staking_contract,
            &mut store.add_distribution_events,
            &mut store.request_commission_events,
        );
        let old_commission_percentage = staking_contract.commission_percentage;
        staking_contract.commission_percentage = new_commission_percentage;
        if (!exists<StakingGroupUpdateCommissionEvent>(staker_address)) {
            move_to(
                staker,
                StakingGroupUpdateCommissionEvent {
                    update_commission_events: account::new_event_handle<UpdateCommissionEvent>(
                        staker
                    )
                }
            )
        };
        if (std::features::module_event_migration_enabled()) {
            emit(
                UpdateCommission { staker: staker_address, operator, old_commission_percentage, new_commission_percentage }
            );
        };
        emit_event(
            &mut borrow_global_mut<StakingGroupUpdateCommissionEvent>(staker_address).update_commission_events,
            UpdateCommissionEvent { staker: staker_address, operator, old_commission_percentage, new_commission_percentage }
        );
    }
}
```


<a id="0x1_staking_contract_request_commission"></a>

## Function `request_commission`

Unlock commission amount from the stake pool. Operator needs to wait for the amount to become withdrawable
at the end of the stake pool&apos;s lockup period before they can actually can withdraw_commission.

Only staker, operator or beneficiary can call this.


```move
module 0x1::staking_contract {
    public entry fun request_commission(account: &signer, staker: address, operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun request_commission(
        account: &signer,
        staker: address,
        operator: address
    ) acquires Store, BeneficiaryForOperator {
        let account_addr = signer::address_of(account);
        assert!(
            account_addr == staker || account_addr == operator || account_addr == beneficiary_for_operator(operator),
            error::unauthenticated(ENOT_STAKER_OR_OPERATOR_OR_BENEFICIARY)
        );
        assert_staking_contract_exists(staker, operator);

        let store = borrow_global_mut<Store>(staker);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);
        // Short-circuit if zero commission.
        if (staking_contract.commission_percentage == 0) {
            return
        };

        // Force distribution of any already inactive stake.
        distribute_internal(staker, operator, staking_contract, &mut store.distribute_events);

        request_commission_internal(
            operator,
            staking_contract,
            &mut store.add_distribution_events,
            &mut store.request_commission_events,
        );
    }
}
```


<a id="0x1_staking_contract_request_commission_internal"></a>

## Function `request_commission_internal`



```move
module 0x1::staking_contract {
    fun request_commission_internal(operator: address, staking_contract: &mut staking_contract::StakingContract, add_distribution_events: &mut event::EventHandle<staking_contract::AddDistributionEvent>, request_commission_events: &mut event::EventHandle<staking_contract::RequestCommissionEvent>): u64
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun request_commission_internal(
        operator: address,
        staking_contract: &mut StakingContract,
        add_distribution_events: &mut EventHandle<AddDistributionEvent>,
        request_commission_events: &mut EventHandle<RequestCommissionEvent>,
    ): u64 {
        // Unlock just the commission portion from the stake pool.
        let (total_active_stake, accumulated_rewards, commission_amount) =
            get_staking_contract_amounts_internal(staking_contract);
        staking_contract.principal = total_active_stake - commission_amount;

        // Short-circuit if there's no commission to pay.
        if (commission_amount == 0) {
            return 0
        };

        // Add a distribution for the operator.
        add_distribution(operator, staking_contract, operator, commission_amount, add_distribution_events);

        // Request to unlock the commission from the stake pool.
        // This won't become fully unlocked until the stake pool's lockup expires.
        stake::unlock_with_cap(commission_amount, &staking_contract.owner_cap);

        let pool_address = staking_contract.pool_address;
        if (std::features::module_event_migration_enabled()) {
            emit(RequestCommission { operator, pool_address, accumulated_rewards, commission_amount });
        };
        emit_event(
            request_commission_events,
            RequestCommissionEvent { operator, pool_address, accumulated_rewards, commission_amount },
        );

        commission_amount
    }
}
```


<a id="0x1_staking_contract_unlock_stake"></a>

## Function `unlock_stake`

Staker can call this to request withdrawal of part or all of their staking_contract.
This also triggers paying commission to the operator for accounting simplicity.


```move
module 0x1::staking_contract {
    public entry fun unlock_stake(staker: &signer, operator: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun unlock_stake(
        staker: &signer,
        operator: address,
        amount: u64
    ) acquires Store, BeneficiaryForOperator {
        // Short-circuit if amount is 0.
        if (amount == 0) return;

        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, operator);

        let store = borrow_global_mut<Store>(staker_address);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);

        // Force distribution of any already inactive stake.
        distribute_internal(staker_address, operator, staking_contract, &mut store.distribute_events);

        // For simplicity, we request commission to be paid out first. This avoids having to ensure to staker doesn't
        // withdraw into the commission portion.
        let commission_paid = request_commission_internal(
            operator,
            staking_contract,
            &mut store.add_distribution_events,
            &mut store.request_commission_events,
        );

        // If there's less active stake remaining than the amount requested (potentially due to commission),
        // only withdraw up to the active amount.
        let (active, _, _, _) = stake::get_stake(staking_contract.pool_address);
        if (active < amount) {
            amount = active;
        };
        staking_contract.principal = staking_contract.principal - amount;

        // Record a distribution for the staker.
        add_distribution(operator, staking_contract, staker_address, amount, &mut store.add_distribution_events);

        // Request to unlock the distribution amount from the stake pool.
        // This won't become fully unlocked until the stake pool's lockup expires.
        stake::unlock_with_cap(amount, &staking_contract.owner_cap);

        let pool_address = staking_contract.pool_address;
        if (std::features::module_event_migration_enabled()) {
            emit(UnlockStake { pool_address, operator, amount, commission_paid });
        };
        emit_event(
            &mut store.unlock_stake_events,
            UnlockStakeEvent { pool_address, operator, amount, commission_paid },
        );
    }
}
```


<a id="0x1_staking_contract_unlock_rewards"></a>

## Function `unlock_rewards`

Unlock all accumulated rewards since the last recorded principals.


```move
module 0x1::staking_contract {
    public entry fun unlock_rewards(staker: &signer, operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun unlock_rewards(staker: &signer, operator: address) acquires Store, BeneficiaryForOperator {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, operator);

        // Calculate how much rewards belongs to the staker after commission is paid.
        let (_, accumulated_rewards, unpaid_commission) = staking_contract_amounts(staker_address, operator);
        let staker_rewards = accumulated_rewards - unpaid_commission;
        unlock_stake(staker, operator, staker_rewards);
    }
}
```


<a id="0x1_staking_contract_switch_operator_with_same_commission"></a>

## Function `switch_operator_with_same_commission`

Allows staker to switch operator without going through the lenghthy process to unstake, without resetting commission.


```move
module 0x1::staking_contract {
    public entry fun switch_operator_with_same_commission(staker: &signer, old_operator: address, new_operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun switch_operator_with_same_commission(
        staker: &signer,
        old_operator: address,
        new_operator: address,
    ) acquires Store, BeneficiaryForOperator {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, old_operator);

        let commission_percentage = commission_percentage(staker_address, old_operator);
        switch_operator(staker, old_operator, new_operator, commission_percentage);
    }
}
```


<a id="0x1_staking_contract_switch_operator"></a>

## Function `switch_operator`

Allows staker to switch operator without going through the lenghthy process to unstake.


```move
module 0x1::staking_contract {
    public entry fun switch_operator(staker: &signer, old_operator: address, new_operator: address, new_commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun switch_operator(
        staker: &signer,
        old_operator: address,
        new_operator: address,
        new_commission_percentage: u64,
    ) acquires Store, BeneficiaryForOperator {
        let staker_address = signer::address_of(staker);
        assert_staking_contract_exists(staker_address, old_operator);

        // Merging two existing staking contracts is too complex as we'd need to merge two separate stake pools.
        let store = borrow_global_mut<Store>(staker_address);
        let staking_contracts = &mut store.staking_contracts;
        assert!(
            !simple_map::contains_key(staking_contracts, &new_operator),
            error::invalid_state(ECANT_MERGE_STAKING_CONTRACTS),
        );

        let (_, staking_contract) = simple_map::remove(staking_contracts, &old_operator);
        // Force distribution of any already inactive stake.
        distribute_internal(staker_address, old_operator, &mut staking_contract, &mut store.distribute_events);

        // For simplicity, we request commission to be paid out first. This avoids having to ensure to staker doesn't
        // withdraw into the commission portion.
        request_commission_internal(
            old_operator,
            &mut staking_contract,
            &mut store.add_distribution_events,
            &mut store.request_commission_events,
        );

        // Update the staking contract's commission rate and stake pool's operator.
        stake::set_operator_with_cap(&staking_contract.owner_cap, new_operator);
        staking_contract.commission_percentage = new_commission_percentage;

        let pool_address = staking_contract.pool_address;
        simple_map::add(staking_contracts, new_operator, staking_contract);
        if (std::features::module_event_migration_enabled()) {
            emit(SwitchOperator { pool_address, old_operator, new_operator });
        };
        emit_event(
            &mut store.switch_operator_events,
            SwitchOperatorEvent { pool_address, old_operator, new_operator }
        );
    }
}
```


<a id="0x1_staking_contract_set_beneficiary_for_operator"></a>

## Function `set_beneficiary_for_operator`

Allows an operator to change its beneficiary. Any existing unpaid commission rewards will be paid to the new
beneficiary. To ensures payment to the current beneficiary, one should first call `distribute` before switching
the beneficiary. An operator can set one beneficiary for staking contract pools, not a separate one for each pool.


```move
module 0x1::staking_contract {
    public entry fun set_beneficiary_for_operator(operator: &signer, new_beneficiary: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
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


<a id="0x1_staking_contract_distribute"></a>

## Function `distribute`

Allow anyone to distribute already unlocked funds. This does not affect reward compounding and therefore does
not need to be restricted to just the staker or operator.


```move
module 0x1::staking_contract {
    public entry fun distribute(staker: address, operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    public entry fun distribute(staker: address, operator: address) acquires Store, BeneficiaryForOperator {
        assert_staking_contract_exists(staker, operator);
        let store = borrow_global_mut<Store>(staker);
        let staking_contract = simple_map::borrow_mut(&mut store.staking_contracts, &operator);
        distribute_internal(staker, operator, staking_contract, &mut store.distribute_events);
    }
}
```


<a id="0x1_staking_contract_distribute_internal"></a>

## Function `distribute_internal`

Distribute all unlocked (inactive) funds according to distribution shares.


```move
module 0x1::staking_contract {
    fun distribute_internal(staker: address, operator: address, staking_contract: &mut staking_contract::StakingContract, distribute_events: &mut event::EventHandle<staking_contract::DistributeEvent>)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun distribute_internal(
        staker: address,
        operator: address,
        staking_contract: &mut StakingContract,
        distribute_events: &mut EventHandle<DistributeEvent>,
    ) acquires BeneficiaryForOperator {
        let pool_address = staking_contract.pool_address;
        let (_, inactive, _, pending_inactive) = stake::get_stake(pool_address);
        let total_potential_withdrawable = inactive + pending_inactive;
        let coins = stake::withdraw_with_cap(&staking_contract.owner_cap, total_potential_withdrawable);
        let distribution_amount = coin::value(&coins);
        if (distribution_amount == 0) {
            coin::destroy_zero(coins);
            return
        };

        let distribution_pool = &mut staking_contract.distribution_pool;
        update_distribution_pool(
            distribution_pool, distribution_amount, operator, staking_contract.commission_percentage);

        // Buy all recipients out of the distribution pool.
        while (pool_u64::shareholders_count(distribution_pool) > 0) {
            let recipients = pool_u64::shareholders(distribution_pool);
            let recipient = *vector::borrow(&mut recipients, 0);
            let current_shares = pool_u64::shares(distribution_pool, recipient);
            let amount_to_distribute = pool_u64::redeem_shares(distribution_pool, recipient, current_shares);
            // If the recipient is the operator, send the commission to the beneficiary instead.
            if (recipient == operator) {
                recipient = beneficiary_for_operator(operator);
            };
            aptos_account::deposit_coins(recipient, coin::extract(&mut coins, amount_to_distribute));

            if (std::features::module_event_migration_enabled()) {
                emit(Distribute { operator, pool_address, recipient, amount: amount_to_distribute });
            };
            emit_event(
                distribute_events,
                DistributeEvent { operator, pool_address, recipient, amount: amount_to_distribute }
            );
        };

        // In case there's any dust left, send them all to the staker.
        if (coin::value(&coins) > 0) {
            aptos_account::deposit_coins(staker, coins);
            pool_u64::update_total_coins(distribution_pool, 0);
        } else {
            coin::destroy_zero(coins);
        }
    }
}
```


<a id="0x1_staking_contract_assert_staking_contract_exists"></a>

## Function `assert_staking_contract_exists`

Assert that a staking_contract exists for the staker/operator pair.


```move
module 0x1::staking_contract {
    fun assert_staking_contract_exists(staker: address, operator: address)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun assert_staking_contract_exists(staker: address, operator: address) acquires Store {
        assert!(exists<Store>(staker), error::not_found(ENO_STAKING_CONTRACT_FOUND_FOR_STAKER));
        let staking_contracts = &mut borrow_global_mut<Store>(staker).staking_contracts;
        assert!(
            simple_map::contains_key(staking_contracts, &operator),
            error::not_found(ENO_STAKING_CONTRACT_FOUND_FOR_OPERATOR),
        );
    }
}
```


<a id="0x1_staking_contract_add_distribution"></a>

## Function `add_distribution`

Add a new distribution for `recipient` and `amount` to the staking contract&apos;s distributions list.


```move
module 0x1::staking_contract {
    fun add_distribution(operator: address, staking_contract: &mut staking_contract::StakingContract, recipient: address, coins_amount: u64, add_distribution_events: &mut event::EventHandle<staking_contract::AddDistributionEvent>)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun add_distribution(
        operator: address,
        staking_contract: &mut StakingContract,
        recipient: address,
        coins_amount: u64,
        add_distribution_events: &mut EventHandle<AddDistributionEvent>
    ) {
        let distribution_pool = &mut staking_contract.distribution_pool;
        let (_, _, _, total_distribution_amount) = stake::get_stake(staking_contract.pool_address);
        update_distribution_pool(
            distribution_pool, total_distribution_amount, operator, staking_contract.commission_percentage);

        pool_u64::buy_in(distribution_pool, recipient, coins_amount);
        let pool_address = staking_contract.pool_address;
        if (std::features::module_event_migration_enabled()) {
            emit(AddDistribution { operator, pool_address, amount: coins_amount });
        };
        emit_event(
            add_distribution_events,
            AddDistributionEvent { operator, pool_address, amount: coins_amount }
        );
    }
}
```


<a id="0x1_staking_contract_get_staking_contract_amounts_internal"></a>

## Function `get_staking_contract_amounts_internal`

Calculate accumulated rewards and commissions since last update.


```move
module 0x1::staking_contract {
    fun get_staking_contract_amounts_internal(staking_contract: &staking_contract::StakingContract): (u64, u64, u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun get_staking_contract_amounts_internal(staking_contract: &StakingContract): (u64, u64, u64) {
        // Pending_inactive is not included in the calculation because pending_inactive can only come from:
        // 1. Outgoing commissions. This means commission has already been extracted.
        // 2. Stake withdrawals from stakers. This also means commission has already been extracted as
        // request_commission_internal is called in unlock_stake
        let (active, _, pending_active, _) = stake::get_stake(staking_contract.pool_address);
        let total_active_stake = active + pending_active;
        let accumulated_rewards = total_active_stake - staking_contract.principal;
        let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;

        (total_active_stake, accumulated_rewards, commission_amount)
    }
}
```


<a id="0x1_staking_contract_create_stake_pool"></a>

## Function `create_stake_pool`



```move
module 0x1::staking_contract {
    fun create_stake_pool(staker: &signer, operator: address, voter: address, contract_creation_seed: vector<u8>): (signer, account::SignerCapability, stake::OwnerCapability)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun create_stake_pool(
        staker: &signer,
        operator: address,
        voter: address,
        contract_creation_seed: vector<u8>,
    ): (signer, SignerCapability, OwnerCapability) {
        // Generate a seed that will be used to create the resource account that hosts the staking contract.
        let seed = create_resource_account_seed(
            signer::address_of(staker), operator, contract_creation_seed);

        let (stake_pool_signer, stake_pool_signer_cap) = account::create_resource_account(staker, seed);
        stake::initialize_stake_owner(&stake_pool_signer, 0, operator, voter);

        // Extract owner_cap from the StakePool, so we have control over it in the staking_contracts flow.
        // This is stored as part of the staking_contract. Thus, the staker would not have direct control over it without
        // going through well-defined functions in this module.
        let owner_cap = stake::extract_owner_cap(&stake_pool_signer);

        (stake_pool_signer, stake_pool_signer_cap, owner_cap)
    }
}
```


<a id="0x1_staking_contract_update_distribution_pool"></a>

## Function `update_distribution_pool`



```move
module 0x1::staking_contract {
    fun update_distribution_pool(distribution_pool: &mut pool_u64::Pool, updated_total_coins: u64, operator: address, commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun update_distribution_pool(
        distribution_pool: &mut Pool,
        updated_total_coins: u64,
        operator: address,
        commission_percentage: u64,
    ) {
        // Short-circuit and do nothing if the pool's total value has not changed.
        if (pool_u64::total_coins(distribution_pool) == updated_total_coins) {
            return
        };

        // Charge all stakeholders (except for the operator themselves) commission on any rewards earnt relatively to the
        // previous value of the distribution pool.
        let shareholders = &pool_u64::shareholders(distribution_pool);
        vector::for_each_ref(shareholders, |shareholder| {
            let shareholder: address = *shareholder;
            if (shareholder != operator) {
                let shares = pool_u64::shares(distribution_pool, shareholder);
                let previous_worth = pool_u64::balance(distribution_pool, shareholder);
                let current_worth = pool_u64::shares_to_amount_with_total_coins(
                    distribution_pool, shares, updated_total_coins);
                let unpaid_commission = (current_worth - previous_worth) * commission_percentage / 100;
                // Transfer shares from current shareholder to the operator as payment.
                // The value of the shares should use the updated pool's total value.
                let shares_to_transfer = pool_u64::amount_to_shares_with_total_coins(
                    distribution_pool, unpaid_commission, updated_total_coins);
                pool_u64::transfer_shares(distribution_pool, shareholder, operator, shares_to_transfer);
            };
        });

        pool_u64::update_total_coins(distribution_pool, updated_total_coins);
    }
}
```


<a id="0x1_staking_contract_create_resource_account_seed"></a>

## Function `create_resource_account_seed`

Create the seed to derive the resource account address.


```move
module 0x1::staking_contract {
    fun create_resource_account_seed(staker: address, operator: address, contract_creation_seed: vector<u8>): vector<u8>
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun create_resource_account_seed(
        staker: address,
        operator: address,
        contract_creation_seed: vector<u8>,
    ): vector<u8> {
        let seed = bcs::to_bytes(&staker);
        vector::append(&mut seed, bcs::to_bytes(&operator));
        // Include a salt to avoid conflicts with any other modules out there that might also generate
        // deterministic resource accounts for the same staker + operator addresses.
        vector::append(&mut seed, SALT);
        // Add an extra salt given by the staker in case an account with the same address has already been created.
        vector::append(&mut seed, contract_creation_seed);
        seed
    }
}
```


<a id="0x1_staking_contract_new_staking_contracts_holder"></a>

## Function `new_staking_contracts_holder`

Create a new staking_contracts resource.


```move
module 0x1::staking_contract {
    fun new_staking_contracts_holder(staker: &signer): staking_contract::Store
}
```


##### Implementation


```move
module 0x1::staking_contract {
    fun new_staking_contracts_holder(staker: &signer): Store {
        Store {
            staking_contracts: simple_map::create<address, StakingContract>(),
            // Events.
            create_staking_contract_events: account::new_event_handle<CreateStakingContractEvent>(staker),
            update_voter_events: account::new_event_handle<UpdateVoterEvent>(staker),
            reset_lockup_events: account::new_event_handle<ResetLockupEvent>(staker),
            add_stake_events: account::new_event_handle<AddStakeEvent>(staker),
            request_commission_events: account::new_event_handle<RequestCommissionEvent>(staker),
            unlock_stake_events: account::new_event_handle<UnlockStakeEvent>(staker),
            switch_operator_events: account::new_event_handle<SwitchOperatorEvent>(staker),
            add_distribution_events: account::new_event_handle<AddDistributionEvent>(staker),
            distribute_events: account::new_event_handle<DistributeEvent>(staker),
        }
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
<td>The Store structure for the staker exists after the staking contract is created.</td>
<td>Medium</td>
<td>The create_staking_contract_with_coins function ensures that the staker account has a Store structure assigned.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](CreateStakingContractWithCoinsAbortsifAndEnsures).</td>
</tr>

<tr>
<td>2</td>
<td>A staking contract is created and stored in a mapping within the Store resource.</td>
<td>High</td>
<td>The create_staking_contract_with_coins function adds the newly created StakingContract to the staking_contracts map with the operator as a key of the Store resource, effectively storing the staking contract.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](CreateStakingContractWithCoinsAbortsifAndEnsures).</td>
</tr>

<tr>
<td>3</td>
<td>Adding stake to the stake pool increases the principal value of the pool, reflecting the additional stake amount.</td>
<td>High</td>
<td>The add_stake function transfers the specified amount of staked coins from the staker&apos;s account to the stake pool associated with the staking contract. It increases the principal value of the staking contract by the added stake amount.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](add_stake).</td>
</tr>

<tr>
<td>4</td>
<td>The staker may update the voter of a staking contract, enabling them to modify the assigned voter address and ensure it accurately reflects their desired choice.</td>
<td>High</td>
<td>The update_voter function ensures that the voter address in a staking contract may be updated by the staker, resulting in the modification of the delegated voter address in the associated stake pool to reflect the new address provided.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](update_voter).</td>
</tr>

<tr>
<td>5</td>
<td>Only the owner of the stake pool has the permission to reset the lockup period of the pool.</td>
<td>Critical</td>
<td>The reset_lockup function ensures that only the staker who owns the stake pool has the authority to reset the lockup period of the pool.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](reset_lockup).</td>
</tr>

<tr>
<td>6</td>
<td>Unlocked funds are correctly distributed to recipients based on their distribution shares, taking into account the associated commission percentage.</td>
<td>High</td>
<td>The distribution process, implemented in the distribute_internal function, accurately allocates unlocked funds to their intended recipients based on their distribution shares. It guarantees that each recipient receives the correct amount of funds, considering the commission percentage associated with the staking contract.</td>
<td>Audited that the correct amount of unlocked funds is distributed according to distribution shares.</td>
</tr>

<tr>
<td>7</td>
<td>The stake pool ensures that the commission is correctly requested and paid out from the old operator&apos;s stake pool before allowing the switch to the new operator.</td>
<td>High</td>
<td>The switch_operator function initiates the commission payout from the stake pool associated with the old operator, ensuring a smooth transition. Paying out the commission before the switch guarantees that the staker receives the appropriate commission amount and maintains the integrity of the staking process.</td>
<td>Audited that the commission is paid to the old operator.</td>
</tr>

<tr>
<td>8</td>
<td>Stakers can withdraw their funds from the staking contract, ensuring the unlocked amount becomes available for withdrawal after the lockup period.</td>
<td>High</td>
<td>The unlock_stake function ensures that the requested amount is properly unlocked from the stake pool, considering the lockup period and that the funds become available for withdrawal when the lockup expires.</td>
<td>Audited that funds are unlocked properly.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::staking_contract {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_stake_pool_address"></a>

### Function `stake_pool_address`


```move
module 0x1::staking_contract {
    #[view]
    public fun stake_pool_address(staker: address, operator: address): address
}
```



```move
module 0x1::staking_contract {
    include ContractExistsAbortsIf;
    let staking_contracts = global<Store>(staker).staking_contracts;
    ensures result == simple_map::spec_get(staking_contracts, operator).pool_address;
}
```


<a id="@Specification_1_last_recorded_principal"></a>

### Function `last_recorded_principal`


```move
module 0x1::staking_contract {
    #[view]
    public fun last_recorded_principal(staker: address, operator: address): u64
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    include ContractExistsAbortsIf;
    let staking_contracts = global<Store>(staker).staking_contracts;
    ensures result == simple_map::spec_get(staking_contracts, operator).principal;
}
```


<a id="@Specification_1_commission_percentage"></a>

### Function `commission_percentage`


```move
module 0x1::staking_contract {
    #[view]
    public fun commission_percentage(staker: address, operator: address): u64
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    include ContractExistsAbortsIf;
    let staking_contracts = global<Store>(staker).staking_contracts;
    ensures result == simple_map::spec_get(staking_contracts, operator).commission_percentage;
}
```


<a id="@Specification_1_staking_contract_amounts"></a>

### Function `staking_contract_amounts`


```move
module 0x1::staking_contract {
    #[view]
    public fun staking_contract_amounts(staker: address, operator: address): (u64, u64, u64)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    requires staking_contract.commission_percentage >= 0 && staking_contract.commission_percentage <= 100;
    let staking_contracts = global<Store>(staker).staking_contracts;
    let staking_contract = simple_map::spec_get(staking_contracts, operator);
    include ContractExistsAbortsIf;
    include GetStakingContractAmountsAbortsIf { staking_contract };
    let pool_address = staking_contract.pool_address;
    let stake_pool = global<stake::StakePool>(pool_address);
    let active = coin::value(stake_pool.active);
    let pending_active = coin::value(stake_pool.pending_active);
    let total_active_stake = active + pending_active;
    let accumulated_rewards = total_active_stake - staking_contract.principal;
    ensures result_1 == total_active_stake;
    ensures result_2 == accumulated_rewards;
}
```


<a id="@Specification_1_pending_distribution_counts"></a>

### Function `pending_distribution_counts`


```move
module 0x1::staking_contract {
    #[view]
    public fun pending_distribution_counts(staker: address, operator: address): u64
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    include ContractExistsAbortsIf;
    let staking_contracts = global<Store>(staker).staking_contracts;
    let staking_contract = simple_map::spec_get(staking_contracts, operator);
    let shareholders_count = len(staking_contract.distribution_pool.shareholders);
    ensures result == shareholders_count;
}
```


<a id="@Specification_1_staking_contract_exists"></a>

### Function `staking_contract_exists`


```move
module 0x1::staking_contract {
    #[view]
    public fun staking_contract_exists(staker: address, operator: address): bool
}
```



```move
module 0x1::staking_contract {
    aborts_if false;
    ensures result == spec_staking_contract_exists(staker, operator);
}
```



<a id="0x1_staking_contract_spec_staking_contract_exists"></a>


```move
module 0x1::staking_contract {
    fun spec_staking_contract_exists(staker: address, operator: address): bool {
       if (!exists<Store>(staker)) {
           false
       } else {
           let store = global<Store>(staker);
           simple_map::spec_contains_key(store.staking_contracts, operator)
       }
    }
}
```


<a id="@Specification_1_beneficiary_for_operator"></a>

### Function `beneficiary_for_operator`


```move
module 0x1::staking_contract {
    #[view]
    public fun beneficiary_for_operator(operator: address): address
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
}
```


<a id="@Specification_1_create_staking_contract"></a>

### Function `create_staking_contract`


```move
module 0x1::staking_contract {
    public entry fun create_staking_contract(staker: &signer, operator: address, voter: address, amount: u64, commission_percentage: u64, contract_creation_seed: vector<u8>)
}
```

Account is not frozen and sufficient to withdraw.


```move
module 0x1::staking_contract {
    pragma aborts_if_is_partial;
    pragma verify_duration_estimate = 120;
    include PreconditionsInCreateContract;
    include WithdrawAbortsIf<AptosCoin> { account: staker };
    include CreateStakingContractWithCoinsAbortsIfAndEnsures;
}
```


<a id="@Specification_1_create_staking_contract_with_coins"></a>

### Function `create_staking_contract_with_coins`


```move
module 0x1::staking_contract {
    public fun create_staking_contract_with_coins(staker: &signer, operator: address, voter: address, coins: coin::Coin<aptos_coin::AptosCoin>, commission_percentage: u64, contract_creation_seed: vector<u8>): address
}
```

The amount should be at least the min_stake_required, so the stake pool will be eligible to join the validator set.
Initialize Store resource if this is the first time the staker has delegated to anyone.
Cannot create the staking contract if it already exists.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    pragma aborts_if_is_partial;
    include PreconditionsInCreateContract;
    let amount = coins.value;
    include CreateStakingContractWithCoinsAbortsIfAndEnsures { amount };
}
```


<a id="@Specification_1_add_stake"></a>

### Function `add_stake`


```move
module 0x1::staking_contract {
    public entry fun add_stake(staker: &signer, operator: address, amount: u64)
}
```

Account is not frozen and sufficient to withdraw.
Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 600;
    include stake::ResourceRequirement;
    aborts_if reconfiguration_state::spec_is_in_progress();
    let staker_address = signer::address_of(staker);
    include ContractExistsAbortsIf { staker: staker_address };
    let store = global<Store>(staker_address);
    let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
    include WithdrawAbortsIf<AptosCoin> { account: staker };
    let balance = global<coin::CoinStore<AptosCoin>>(staker_address).coin.value;
    let post post_coin = global<coin::CoinStore<AptosCoin>>(staker_address).coin.value;
    ensures post_coin == balance - amount;
    let owner_cap = staking_contract.owner_cap;
    include stake::AddStakeWithCapAbortsIfAndEnsures { owner_cap };
    let post post_store = global<Store>(staker_address);
    let post post_staking_contract = simple_map::spec_get(post_store.staking_contracts, operator);
    aborts_if staking_contract.principal + amount > MAX_U64;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures post_staking_contract.principal == staking_contract.principal + amount;
}
```


<a id="@Specification_1_update_voter"></a>

### Function `update_voter`


```move
module 0x1::staking_contract {
    public entry fun update_voter(staker: &signer, operator: address, new_voter: address)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    let staker_address = signer::address_of(staker);
    include UpdateVoterSchema { staker: staker_address };
    let post store = global<Store>(staker_address);
    let post staking_contract = simple_map::spec_get(store.staking_contracts, operator);
    let post pool_address = staking_contract.owner_cap.pool_address;
    let post new_delegated_voter = global<stake::StakePool>(pool_address).delegated_voter;
    ensures new_delegated_voter == new_voter;
}
```


<a id="@Specification_1_reset_lockup"></a>

### Function `reset_lockup`


```move
module 0x1::staking_contract {
    public entry fun reset_lockup(staker: &signer, operator: address)
}
```

Staking_contract exists the stacker/operator pair.
Only active validator can update locked_until_secs.


```move
module 0x1::staking_contract {
    let staker_address = signer::address_of(staker);
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    include ContractExistsAbortsIf { staker: staker_address };
    include IncreaseLockupWithCapAbortsIf { staker: staker_address };
}
```


<a id="@Specification_1_update_commision"></a>

### Function `update_commision`


```move
module 0x1::staking_contract {
    public entry fun update_commision(staker: &signer, operator: address, new_commission_percentage: u64)
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
    let staker_address = signer::address_of(staker);
    aborts_if new_commission_percentage > 100;
    include ContractExistsAbortsIf { staker: staker_address };
}
```


<a id="@Specification_1_request_commission"></a>

### Function `request_commission`


```move
module 0x1::staking_contract {
    public entry fun request_commission(account: &signer, staker: address, operator: address)
}
```

Only staker or operator can call this.


```move
module 0x1::staking_contract {
    pragma verify = false;
    let account_addr = signer::address_of(account);
    include ContractExistsAbortsIf { staker };
    aborts_if account_addr != staker && account_addr != operator;
}
```


<a id="@Specification_1_request_commission_internal"></a>

### Function `request_commission_internal`


```move
module 0x1::staking_contract {
    fun request_commission_internal(operator: address, staking_contract: &mut staking_contract::StakingContract, add_distribution_events: &mut event::EventHandle<staking_contract::AddDistributionEvent>, request_commission_events: &mut event::EventHandle<staking_contract::RequestCommissionEvent>): u64
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
    include GetStakingContractAmountsAbortsIf;
}
```


<a id="@Specification_1_unlock_stake"></a>

### Function `unlock_stake`


```move
module 0x1::staking_contract {
    public entry fun unlock_stake(staker: &signer, operator: address, amount: u64)
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
    requires amount > 0;
    let staker_address = signer::address_of(staker);
    include ContractExistsAbortsIf { staker: staker_address };
}
```


<a id="@Specification_1_unlock_rewards"></a>

### Function `unlock_rewards`


```move
module 0x1::staking_contract {
    public entry fun unlock_rewards(staker: &signer, operator: address)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify = false;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    requires staking_contract.commission_percentage >= 0 && staking_contract.commission_percentage <= 100;
    let staker_address = signer::address_of(staker);
    let staking_contracts = global<Store>(staker_address).staking_contracts;
    let staking_contract = simple_map::spec_get(staking_contracts, operator);
    include ContractExistsAbortsIf { staker: staker_address };
}
```


<a id="@Specification_1_switch_operator_with_same_commission"></a>

### Function `switch_operator_with_same_commission`


```move
module 0x1::staking_contract {
    public entry fun switch_operator_with_same_commission(staker: &signer, old_operator: address, new_operator: address)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    pragma aborts_if_is_partial;
    let staker_address = signer::address_of(staker);
    include ContractExistsAbortsIf { staker: staker_address, operator: old_operator };
}
```


<a id="@Specification_1_switch_operator"></a>

### Function `switch_operator`


```move
module 0x1::staking_contract {
    public entry fun switch_operator(staker: &signer, old_operator: address, new_operator: address, new_commission_percentage: u64)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify = false;
    let staker_address = signer::address_of(staker);
    include ContractExistsAbortsIf { staker: staker_address, operator: old_operator };
    let store = global<Store>(staker_address);
    let staking_contracts = store.staking_contracts;
    aborts_if simple_map::spec_contains_key(staking_contracts, new_operator);
}
```


<a id="@Specification_1_set_beneficiary_for_operator"></a>

### Function `set_beneficiary_for_operator`


```move
module 0x1::staking_contract {
    public entry fun set_beneficiary_for_operator(operator: &signer, new_beneficiary: address)
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
}
```


<a id="@Specification_1_distribute"></a>

### Function `distribute`


```move
module 0x1::staking_contract {
    public entry fun distribute(staker: address, operator: address)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    pragma aborts_if_is_partial;
    include ContractExistsAbortsIf;
}
```


<a id="@Specification_1_distribute_internal"></a>

### Function `distribute_internal`


```move
module 0x1::staking_contract {
    fun distribute_internal(staker: address, operator: address, staking_contract: &mut staking_contract::StakingContract, distribute_events: &mut event::EventHandle<staking_contract::DistributeEvent>)
}
```

The StakePool exists under the pool_address of StakingContract.
The value of inactive and pending_inactive in the stake_pool is up to MAX_U64.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    pragma aborts_if_is_partial;
    let pool_address = staking_contract.pool_address;
    let stake_pool = borrow_global<stake::StakePool>(pool_address);
    aborts_if !exists<stake::StakePool>(pool_address);
    aborts_if stake_pool.inactive.value + stake_pool.pending_inactive.value > MAX_U64;
    aborts_if !exists<stake::StakePool>(staking_contract.owner_cap.pool_address);
}
```


<a id="@Specification_1_assert_staking_contract_exists"></a>

### Function `assert_staking_contract_exists`


```move
module 0x1::staking_contract {
    fun assert_staking_contract_exists(staker: address, operator: address)
}
```

Staking_contract exists the stacker/operator pair.


```move
module 0x1::staking_contract {
    include ContractExistsAbortsIf;
}
```


<a id="@Specification_1_add_distribution"></a>

### Function `add_distribution`


```move
module 0x1::staking_contract {
    fun add_distribution(operator: address, staking_contract: &mut staking_contract::StakingContract, recipient: address, coins_amount: u64, add_distribution_events: &mut event::EventHandle<staking_contract::AddDistributionEvent>)
}
```



```move
module 0x1::staking_contract {
    pragma verify = false;
}
```


<a id="@Specification_1_get_staking_contract_amounts_internal"></a>

### Function `get_staking_contract_amounts_internal`


```move
module 0x1::staking_contract {
    fun get_staking_contract_amounts_internal(staking_contract: &staking_contract::StakingContract): (u64, u64, u64)
}
```

The StakePool exists under the pool_address of StakingContract.


```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    include GetStakingContractAmountsAbortsIf;
    let pool_address = staking_contract.pool_address;
    let stake_pool = global<stake::StakePool>(pool_address);
    let active = coin::value(stake_pool.active);
    let pending_active = coin::value(stake_pool.pending_active);
    let total_active_stake = active + pending_active;
    let accumulated_rewards = total_active_stake - staking_contract.principal;
    let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;
    ensures result_1 == total_active_stake;
    ensures result_2 == accumulated_rewards;
    ensures result_3 == commission_amount;
}
```


<a id="@Specification_1_create_stake_pool"></a>

### Function `create_stake_pool`


```move
module 0x1::staking_contract {
    fun create_stake_pool(staker: &signer, operator: address, voter: address, contract_creation_seed: vector<u8>): (signer, account::SignerCapability, stake::OwnerCapability)
}
```



```move
module 0x1::staking_contract {
    pragma verify_duration_estimate = 120;
    include stake::ResourceRequirement;
    let staker_address = signer::address_of(staker);
    let seed_0 = bcs::to_bytes(staker_address);
    let seed_1 = concat(concat(concat(seed_0, bcs::to_bytes(operator)), SALT), contract_creation_seed);
    let resource_addr = account::spec_create_resource_address(staker_address, seed_1);
    include CreateStakePoolAbortsIf { resource_addr };
    ensures exists<account::Account>(resource_addr);
    let post post_account = global<account::Account>(resource_addr);
    ensures post_account.authentication_key == account::ZERO_AUTH_KEY;
    ensures post_account.signer_capability_offer.for == std::option::spec_some(resource_addr);
    ensures exists<stake::StakePool>(resource_addr);
    let post post_owner_cap = global<stake::OwnerCapability>(resource_addr);
    let post post_pool_address = post_owner_cap.pool_address;
    let post post_stake_pool = global<stake::StakePool>(post_pool_address);
    let post post_operator = post_stake_pool.operator_address;
    let post post_delegated_voter = post_stake_pool.delegated_voter;
    ensures resource_addr != operator ==> post_operator == operator;
    ensures resource_addr != voter ==> post_delegated_voter == voter;
    ensures signer::address_of(result_1) == resource_addr;
    ensures result_2 == SignerCapability { account: resource_addr };
    ensures result_3 == OwnerCapability { pool_address: resource_addr };
}
```


<a id="@Specification_1_update_distribution_pool"></a>

### Function `update_distribution_pool`


```move
module 0x1::staking_contract {
    fun update_distribution_pool(distribution_pool: &mut pool_u64::Pool, updated_total_coins: u64, operator: address, commission_percentage: u64)
}
```



```move
module 0x1::staking_contract {
    pragma aborts_if_is_partial;
}
```


<a id="@Specification_1_new_staking_contracts_holder"></a>

### Function `new_staking_contracts_holder`


```move
module 0x1::staking_contract {
    fun new_staking_contracts_holder(staker: &signer): staking_contract::Store
}
```

The Account exists under the staker.
The guid_creation_num of the ccount resource is up to MAX_U64.


```move
module 0x1::staking_contract {
    include NewStakingContractsHolderAbortsIf;
}
```



<a id="0x1_staking_contract_NewStakingContractsHolderAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema NewStakingContractsHolderAbortsIf {
        staker: signer;
        let addr = signer::address_of(staker);
        let account = global<account::Account>(addr);
        aborts_if !exists<account::Account>(addr);
        aborts_if account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
        aborts_if account.guid_creation_num + 9 > MAX_U64;
    }
}
```

The Store exists under the staker.
a staking_contract exists for the staker/operator pair.


<a id="0x1_staking_contract_ContractExistsAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema ContractExistsAbortsIf {
        staker: address;
        operator: address;
        aborts_if !exists<Store>(staker);
        let staking_contracts = global<Store>(staker).staking_contracts;
        aborts_if !simple_map::spec_contains_key(staking_contracts, operator);
    }
}
```



<a id="0x1_staking_contract_UpdateVoterSchema"></a>


```move
module 0x1::staking_contract {
    schema UpdateVoterSchema {
        staker: address;
        operator: address;
        let store = global<Store>(staker);
        let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
        let pool_address = staking_contract.pool_address;
        aborts_if !exists<stake::StakePool>(pool_address);
        aborts_if !exists<stake::StakePool>(staking_contract.owner_cap.pool_address);
        include ContractExistsAbortsIf;
    }
}
```



<a id="0x1_staking_contract_WithdrawAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema WithdrawAbortsIf<CoinType> {
        account: signer;
        amount: u64;
        let account_addr = signer::address_of(account);
        let coin_store = global<coin::CoinStore<CoinType>>(account_addr);
        let balance = coin_store.coin.value;
        aborts_if !exists<coin::CoinStore<CoinType>>(account_addr);
        aborts_if coin_store.frozen;
        aborts_if balance < amount;
    }
}
```



<a id="0x1_staking_contract_GetStakingContractAmountsAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema GetStakingContractAmountsAbortsIf {
        staking_contract: StakingContract;
        let pool_address = staking_contract.pool_address;
        let stake_pool = global<stake::StakePool>(pool_address);
        let active = coin::value(stake_pool.active);
        let pending_active = coin::value(stake_pool.pending_active);
        let total_active_stake = active + pending_active;
        let accumulated_rewards = total_active_stake - staking_contract.principal;
        aborts_if !exists<stake::StakePool>(pool_address);
        aborts_if active + pending_active > MAX_U64;
        aborts_if total_active_stake < staking_contract.principal;
        aborts_if accumulated_rewards * staking_contract.commission_percentage > MAX_U64;
    }
}
```



<a id="0x1_staking_contract_IncreaseLockupWithCapAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema IncreaseLockupWithCapAbortsIf {
        staker: address;
        operator: address;
        let store = global<Store>(staker);
        let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
        let pool_address = staking_contract.owner_cap.pool_address;
        aborts_if !stake::stake_pool_exists(pool_address);
        aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
        let config = global<staking_config::StakingConfig>(@aptos_framework);
        let stake_pool = global<stake::StakePool>(pool_address);
        let old_locked_until_secs = stake_pool.locked_until_secs;
        let seconds = global<timestamp::CurrentTimeMicroseconds>(
            @aptos_framework
        ).microseconds / timestamp::MICRO_CONVERSION_FACTOR;
        let new_locked_until_secs = seconds + config.recurring_lockup_duration_secs;
        aborts_if seconds + config.recurring_lockup_duration_secs > MAX_U64;
        aborts_if old_locked_until_secs > new_locked_until_secs || old_locked_until_secs == new_locked_until_secs;
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        let post post_store = global<Store>(staker);
        let post post_staking_contract = simple_map::spec_get(post_store.staking_contracts, operator);
        let post post_stake_pool = global<stake::StakePool>(post_staking_contract.owner_cap.pool_address);
        ensures post_stake_pool.locked_until_secs == new_locked_until_secs;
    }
}
```



<a id="0x1_staking_contract_CreateStakingContractWithCoinsAbortsIfAndEnsures"></a>


```move
module 0x1::staking_contract {
    schema CreateStakingContractWithCoinsAbortsIfAndEnsures {
        staker: signer;
        operator: address;
        voter: address;
        amount: u64;
        commission_percentage: u64;
        contract_creation_seed: vector<u8>;
        aborts_if commission_percentage > 100;
        aborts_if !exists<staking_config::StakingConfig>(@aptos_framework);
        let config = global<staking_config::StakingConfig>(@aptos_framework);
        let min_stake_required = config.minimum_stake;
        aborts_if amount < min_stake_required;
        let staker_address = signer::address_of(staker);
        let account = global<account::Account>(staker_address);
        aborts_if !exists<Store>(staker_address) && !exists<account::Account>(staker_address);
        aborts_if !exists<Store>(staker_address) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    // This enforces ### high&#45;level&#45;req&#45;1
    [#high&#45;level&#45;req](high&#45;level requirement 1):
        ensures exists<Store>(staker_address);
        let store = global<Store>(staker_address);
        let staking_contracts = store.staking_contracts;
        let owner_cap = simple_map::spec_get(store.staking_contracts, operator).owner_cap;
        let post post_store = global<Store>(staker_address);
        let post post_staking_contracts = post_store.staking_contracts;
    }
}
```



<a id="0x1_staking_contract_PreconditionsInCreateContract"></a>


```move
module 0x1::staking_contract {
    schema PreconditionsInCreateContract {
        requires exists<stake::ValidatorPerformance>(@aptos_framework);
        requires exists<stake::ValidatorSet>(@aptos_framework);
        requires exists<staking_config::StakingRewardsConfig>(
            @aptos_framework
        ) || !std::features::spec_periodical_reward_rate_decrease_enabled();
        requires exists<stake::ValidatorFees>(@aptos_framework);
        requires exists<aptos_framework::timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        requires exists<stake::AptosCoinCapabilities>(@aptos_framework);
    }
}
```



<a id="0x1_staking_contract_CreateStakePoolAbortsIf"></a>


```move
module 0x1::staking_contract {
    schema CreateStakePoolAbortsIf {
        resource_addr: address;
        operator: address;
        voter: address;
        contract_creation_seed: vector<u8>;
        let acc = global<account::Account>(resource_addr);
        aborts_if exists<account::Account>(resource_addr) && (len(
            acc.signer_capability_offer.for.vec
        ) != 0 || acc.sequence_number != 0);
        aborts_if !exists<account::Account>(resource_addr) && len(bcs::to_bytes(resource_addr)) != 32;
        aborts_if len(account::ZERO_AUTH_KEY) != 32;
        aborts_if exists<stake::ValidatorConfig>(resource_addr);
        let allowed = global<stake::AllowedValidators>(@aptos_framework);
        aborts_if exists<stake::AllowedValidators>(@aptos_framework) && !contains(allowed.accounts, resource_addr);
        aborts_if exists<stake::StakePool>(resource_addr);
        aborts_if exists<stake::OwnerCapability>(resource_addr);
        aborts_if exists<account::Account>(
            resource_addr
        ) && acc.guid_creation_num + 12 >= account::MAX_GUID_CREATION_NUM;
    }
}
```
