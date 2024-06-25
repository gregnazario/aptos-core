
<a id="0x1_vesting"></a>

# Module `0x1::vesting`


Simple vesting contract that allows specifying how much APT coins should be vesting in each fixed&#45;size period. The
vesting contract also comes with staking and allows shareholders to withdraw rewards anytime.

Vesting schedule is represented as a vector of distributions. For example, a vesting schedule of
[3/48, 3/48, 1/48] means that after the vesting starts:
1. The first and second periods will vest 3/48 of the total original grant.
2. The third period will vest 1/48.
3. All subsequent periods will also vest 1/48 (last distribution in the schedule) until the original grant runs out.

Shareholder flow:
1. Admin calls create_vesting_contract with a schedule of [3/48, 3/48, 1/48] with a vesting cliff of 1 year and
vesting period of 1 month.
2. After a month, a shareholder calls unlock_rewards to request rewards. They can also call vest() which would also
unlocks rewards but since the 1 year cliff has not passed (vesting has not started), vest() would not release any of
the original grant.
3. After the unlocked rewards become fully withdrawable (as it&apos;s subject to staking lockup), shareholders can call
distribute() to send all withdrawable funds to all shareholders based on the original grant&apos;s shares structure.
4. After 1 year and 1 month, the vesting schedule now starts. Shareholders call vest() to unlock vested coins. vest()
checks the schedule and unlocks 3/48 of the original grant in addition to any accumulated rewards since last
unlock_rewards(). Once the unlocked coins become withdrawable, shareholders can call distribute().
5. Assuming the shareholders forgot to call vest() for 2 months, when they call vest() again, they will unlock vested
tokens for the next period since last vest. This would be for the first month they missed. They can call vest() a
second time to unlock for the second month they missed.

Admin flow:
1. After creating the vesting contract, admin cannot change the vesting schedule.
2. Admin can call update_voter, update_operator, or reset_lockup at any time to update the underlying staking
contract.
3. Admin can also call update_beneficiary for any shareholder. This would send all distributions (rewards, vested
coins) of that shareholder to the beneficiary account. By defalt, if a beneficiary is not set, the distributions are
send directly to the shareholder account.
4. Admin can call terminate_vesting_contract to terminate the vesting. This would first finish any distribution but
will prevent any further rewards or vesting distributions from being created. Once the locked up stake becomes
withdrawable, admin can call admin_withdraw to withdraw all funds to the vesting contract&apos;s withdrawal address.


-  [Struct `VestingSchedule`](#0x1_vesting_VestingSchedule)
-  [Struct `StakingInfo`](#0x1_vesting_StakingInfo)
-  [Resource `VestingContract`](#0x1_vesting_VestingContract)
-  [Resource `VestingAccountManagement`](#0x1_vesting_VestingAccountManagement)
-  [Resource `AdminStore`](#0x1_vesting_AdminStore)
-  [Struct `CreateVestingContract`](#0x1_vesting_CreateVestingContract)
-  [Struct `UpdateOperator`](#0x1_vesting_UpdateOperator)
-  [Struct `UpdateVoter`](#0x1_vesting_UpdateVoter)
-  [Struct `ResetLockup`](#0x1_vesting_ResetLockup)
-  [Struct `SetBeneficiary`](#0x1_vesting_SetBeneficiary)
-  [Struct `UnlockRewards`](#0x1_vesting_UnlockRewards)
-  [Struct `Vest`](#0x1_vesting_Vest)
-  [Struct `Distribute`](#0x1_vesting_Distribute)
-  [Struct `Terminate`](#0x1_vesting_Terminate)
-  [Struct `AdminWithdraw`](#0x1_vesting_AdminWithdraw)
-  [Struct `CreateVestingContractEvent`](#0x1_vesting_CreateVestingContractEvent)
-  [Struct `UpdateOperatorEvent`](#0x1_vesting_UpdateOperatorEvent)
-  [Struct `UpdateVoterEvent`](#0x1_vesting_UpdateVoterEvent)
-  [Struct `ResetLockupEvent`](#0x1_vesting_ResetLockupEvent)
-  [Struct `SetBeneficiaryEvent`](#0x1_vesting_SetBeneficiaryEvent)
-  [Struct `UnlockRewardsEvent`](#0x1_vesting_UnlockRewardsEvent)
-  [Struct `VestEvent`](#0x1_vesting_VestEvent)
-  [Struct `DistributeEvent`](#0x1_vesting_DistributeEvent)
-  [Struct `TerminateEvent`](#0x1_vesting_TerminateEvent)
-  [Struct `AdminWithdrawEvent`](#0x1_vesting_AdminWithdrawEvent)
-  [Constants](#@Constants_0)
-  [Function `stake_pool_address`](#0x1_vesting_stake_pool_address)
-  [Function `vesting_start_secs`](#0x1_vesting_vesting_start_secs)
-  [Function `period_duration_secs`](#0x1_vesting_period_duration_secs)
-  [Function `remaining_grant`](#0x1_vesting_remaining_grant)
-  [Function `beneficiary`](#0x1_vesting_beneficiary)
-  [Function `operator_commission_percentage`](#0x1_vesting_operator_commission_percentage)
-  [Function `vesting_contracts`](#0x1_vesting_vesting_contracts)
-  [Function `operator`](#0x1_vesting_operator)
-  [Function `voter`](#0x1_vesting_voter)
-  [Function `vesting_schedule`](#0x1_vesting_vesting_schedule)
-  [Function `total_accumulated_rewards`](#0x1_vesting_total_accumulated_rewards)
-  [Function `accumulated_rewards`](#0x1_vesting_accumulated_rewards)
-  [Function `shareholders`](#0x1_vesting_shareholders)
-  [Function `shareholder`](#0x1_vesting_shareholder)
-  [Function `create_vesting_schedule`](#0x1_vesting_create_vesting_schedule)
-  [Function `create_vesting_contract`](#0x1_vesting_create_vesting_contract)
-  [Function `unlock_rewards`](#0x1_vesting_unlock_rewards)
-  [Function `unlock_rewards_many`](#0x1_vesting_unlock_rewards_many)
-  [Function `vest`](#0x1_vesting_vest)
-  [Function `vest_many`](#0x1_vesting_vest_many)
-  [Function `distribute`](#0x1_vesting_distribute)
-  [Function `distribute_many`](#0x1_vesting_distribute_many)
-  [Function `terminate_vesting_contract`](#0x1_vesting_terminate_vesting_contract)
-  [Function `admin_withdraw`](#0x1_vesting_admin_withdraw)
-  [Function `update_operator`](#0x1_vesting_update_operator)
-  [Function `update_operator_with_same_commission`](#0x1_vesting_update_operator_with_same_commission)
-  [Function `update_commission_percentage`](#0x1_vesting_update_commission_percentage)
-  [Function `update_voter`](#0x1_vesting_update_voter)
-  [Function `reset_lockup`](#0x1_vesting_reset_lockup)
-  [Function `set_beneficiary`](#0x1_vesting_set_beneficiary)
-  [Function `reset_beneficiary`](#0x1_vesting_reset_beneficiary)
-  [Function `set_management_role`](#0x1_vesting_set_management_role)
-  [Function `set_beneficiary_resetter`](#0x1_vesting_set_beneficiary_resetter)
-  [Function `set_beneficiary_for_operator`](#0x1_vesting_set_beneficiary_for_operator)
-  [Function `get_role_holder`](#0x1_vesting_get_role_holder)
-  [Function `get_vesting_account_signer`](#0x1_vesting_get_vesting_account_signer)
-  [Function `get_vesting_account_signer_internal`](#0x1_vesting_get_vesting_account_signer_internal)
-  [Function `create_vesting_contract_account`](#0x1_vesting_create_vesting_contract_account)
-  [Function `verify_admin`](#0x1_vesting_verify_admin)
-  [Function `assert_vesting_contract_exists`](#0x1_vesting_assert_vesting_contract_exists)
-  [Function `assert_active_vesting_contract`](#0x1_vesting_assert_active_vesting_contract)
-  [Function `unlock_stake`](#0x1_vesting_unlock_stake)
-  [Function `withdraw_stake`](#0x1_vesting_withdraw_stake)
-  [Function `get_beneficiary`](#0x1_vesting_get_beneficiary)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `stake_pool_address`](#@Specification_1_stake_pool_address)
    -  [Function `vesting_start_secs`](#@Specification_1_vesting_start_secs)
    -  [Function `period_duration_secs`](#@Specification_1_period_duration_secs)
    -  [Function `remaining_grant`](#@Specification_1_remaining_grant)
    -  [Function `beneficiary`](#@Specification_1_beneficiary)
    -  [Function `operator_commission_percentage`](#@Specification_1_operator_commission_percentage)
    -  [Function `vesting_contracts`](#@Specification_1_vesting_contracts)
    -  [Function `operator`](#@Specification_1_operator)
    -  [Function `voter`](#@Specification_1_voter)
    -  [Function `vesting_schedule`](#@Specification_1_vesting_schedule)
    -  [Function `total_accumulated_rewards`](#@Specification_1_total_accumulated_rewards)
    -  [Function `accumulated_rewards`](#@Specification_1_accumulated_rewards)
    -  [Function `shareholders`](#@Specification_1_shareholders)
    -  [Function `shareholder`](#@Specification_1_shareholder)
    -  [Function `create_vesting_schedule`](#@Specification_1_create_vesting_schedule)
    -  [Function `create_vesting_contract`](#@Specification_1_create_vesting_contract)
    -  [Function `unlock_rewards`](#@Specification_1_unlock_rewards)
    -  [Function `unlock_rewards_many`](#@Specification_1_unlock_rewards_many)
    -  [Function `vest`](#@Specification_1_vest)
    -  [Function `vest_many`](#@Specification_1_vest_many)
    -  [Function `distribute`](#@Specification_1_distribute)
    -  [Function `distribute_many`](#@Specification_1_distribute_many)
    -  [Function `terminate_vesting_contract`](#@Specification_1_terminate_vesting_contract)
    -  [Function `admin_withdraw`](#@Specification_1_admin_withdraw)
    -  [Function `update_operator`](#@Specification_1_update_operator)
    -  [Function `update_operator_with_same_commission`](#@Specification_1_update_operator_with_same_commission)
    -  [Function `update_commission_percentage`](#@Specification_1_update_commission_percentage)
    -  [Function `update_voter`](#@Specification_1_update_voter)
    -  [Function `reset_lockup`](#@Specification_1_reset_lockup)
    -  [Function `set_beneficiary`](#@Specification_1_set_beneficiary)
    -  [Function `reset_beneficiary`](#@Specification_1_reset_beneficiary)
    -  [Function `set_management_role`](#@Specification_1_set_management_role)
    -  [Function `set_beneficiary_resetter`](#@Specification_1_set_beneficiary_resetter)
    -  [Function `set_beneficiary_for_operator`](#@Specification_1_set_beneficiary_for_operator)
    -  [Function `get_role_holder`](#@Specification_1_get_role_holder)
    -  [Function `get_vesting_account_signer`](#@Specification_1_get_vesting_account_signer)
    -  [Function `get_vesting_account_signer_internal`](#@Specification_1_get_vesting_account_signer_internal)
    -  [Function `create_vesting_contract_account`](#@Specification_1_create_vesting_contract_account)
    -  [Function `verify_admin`](#@Specification_1_verify_admin)
    -  [Function `assert_vesting_contract_exists`](#@Specification_1_assert_vesting_contract_exists)
    -  [Function `assert_active_vesting_contract`](#@Specification_1_assert_active_vesting_contract)
    -  [Function `unlock_stake`](#@Specification_1_unlock_stake)
    -  [Function `withdraw_stake`](#@Specification_1_withdraw_stake)
    -  [Function `get_beneficiary`](#@Specification_1_get_beneficiary)


```move
module 0x1::vesting {
    use 0x1::account;
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::bcs;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::fixed_point32;
    use 0x1::math64;
    use 0x1::pool_u64;
    use 0x1::signer;
    use 0x1::simple_map;
    use 0x1::stake;
    use 0x1::staking_contract;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::timestamp;
    use 0x1::vector;
}
```


<a id="0x1_vesting_VestingSchedule"></a>

## Struct `VestingSchedule`



```move
module 0x1::vesting {
    struct VestingSchedule has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`schedule: vector<fixed_point32::FixedPoint32>`
</dt>
<dd>

</dd>
<dt>
`start_timestamp_secs: u64`
</dt>
<dd>

</dd>
<dt>
`period_duration: u64`
</dt>
<dd>

</dd>
<dt>
`last_vested_period: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_StakingInfo"></a>

## Struct `StakingInfo`



```move
module 0x1::vesting {
    struct StakingInfo has store
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
`voter: address`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_VestingContract"></a>

## Resource `VestingContract`



```move
module 0x1::vesting {
    struct VestingContract has key
}
```


##### Fields


<dl>
<dt>
`state: u64`
</dt>
<dd>

</dd>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`grant_pool: pool_u64::Pool`
</dt>
<dd>

</dd>
<dt>
`beneficiaries: simple_map::SimpleMap<address, address>`
</dt>
<dd>

</dd>
<dt>
`vesting_schedule: vesting::VestingSchedule`
</dt>
<dd>

</dd>
<dt>
`withdrawal_address: address`
</dt>
<dd>

</dd>
<dt>
`staking: vesting::StakingInfo`
</dt>
<dd>

</dd>
<dt>
`remaining_grant: u64`
</dt>
<dd>

</dd>
<dt>
`signer_cap: account::SignerCapability`
</dt>
<dd>

</dd>
<dt>
`update_operator_events: event::EventHandle<vesting::UpdateOperatorEvent>`
</dt>
<dd>

</dd>
<dt>
`update_voter_events: event::EventHandle<vesting::UpdateVoterEvent>`
</dt>
<dd>

</dd>
<dt>
`reset_lockup_events: event::EventHandle<vesting::ResetLockupEvent>`
</dt>
<dd>

</dd>
<dt>
`set_beneficiary_events: event::EventHandle<vesting::SetBeneficiaryEvent>`
</dt>
<dd>

</dd>
<dt>
`unlock_rewards_events: event::EventHandle<vesting::UnlockRewardsEvent>`
</dt>
<dd>

</dd>
<dt>
`vest_events: event::EventHandle<vesting::VestEvent>`
</dt>
<dd>

</dd>
<dt>
`distribute_events: event::EventHandle<vesting::DistributeEvent>`
</dt>
<dd>

</dd>
<dt>
`terminate_events: event::EventHandle<vesting::TerminateEvent>`
</dt>
<dd>

</dd>
<dt>
`admin_withdraw_events: event::EventHandle<vesting::AdminWithdrawEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_VestingAccountManagement"></a>

## Resource `VestingAccountManagement`



```move
module 0x1::vesting {
    struct VestingAccountManagement has key
}
```


##### Fields


<dl>
<dt>
`roles: simple_map::SimpleMap<string::String, address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_AdminStore"></a>

## Resource `AdminStore`



```move
module 0x1::vesting {
    struct AdminStore has key
}
```


##### Fields


<dl>
<dt>
`vesting_contracts: vector<address>`
</dt>
<dd>

</dd>
<dt>
`nonce: u64`
</dt>
<dd>

</dd>
<dt>
`create_events: event::EventHandle<vesting::CreateVestingContractEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_CreateVestingContract"></a>

## Struct `CreateVestingContract`



```move
module 0x1::vesting {
    #[event]
    struct CreateVestingContract has drop, store
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
`grant_amount: u64`
</dt>
<dd>

</dd>
<dt>
`withdrawal_address: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_UpdateOperator"></a>

## Struct `UpdateOperator`



```move
module 0x1::vesting {
    #[event]
    struct UpdateOperator has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
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
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_UpdateVoter"></a>

## Struct `UpdateVoter`



```move
module 0x1::vesting {
    #[event]
    struct UpdateVoter has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
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


<a id="0x1_vesting_ResetLockup"></a>

## Struct `ResetLockup`



```move
module 0x1::vesting {
    #[event]
    struct ResetLockup has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`new_lockup_expiration_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_SetBeneficiary"></a>

## Struct `SetBeneficiary`



```move
module 0x1::vesting {
    #[event]
    struct SetBeneficiary has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`shareholder: address`
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


<a id="0x1_vesting_UnlockRewards"></a>

## Struct `UnlockRewards`



```move
module 0x1::vesting {
    #[event]
    struct UnlockRewards has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_Vest"></a>

## Struct `Vest`



```move
module 0x1::vesting {
    #[event]
    struct Vest has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`period_vested: u64`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_Distribute"></a>

## Struct `Distribute`



```move
module 0x1::vesting {
    #[event]
    struct Distribute has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_Terminate"></a>

## Struct `Terminate`



```move
module 0x1::vesting {
    #[event]
    struct Terminate has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_AdminWithdraw"></a>

## Struct `AdminWithdraw`



```move
module 0x1::vesting {
    #[event]
    struct AdminWithdraw has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_CreateVestingContractEvent"></a>

## Struct `CreateVestingContractEvent`



```move
module 0x1::vesting {
    struct CreateVestingContractEvent has drop, store
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
`grant_amount: u64`
</dt>
<dd>

</dd>
<dt>
`withdrawal_address: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_UpdateOperatorEvent"></a>

## Struct `UpdateOperatorEvent`



```move
module 0x1::vesting {
    struct UpdateOperatorEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
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
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_UpdateVoterEvent"></a>

## Struct `UpdateVoterEvent`



```move
module 0x1::vesting {
    struct UpdateVoterEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
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


<a id="0x1_vesting_ResetLockupEvent"></a>

## Struct `ResetLockupEvent`



```move
module 0x1::vesting {
    struct ResetLockupEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`new_lockup_expiration_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_SetBeneficiaryEvent"></a>

## Struct `SetBeneficiaryEvent`



```move
module 0x1::vesting {
    struct SetBeneficiaryEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`shareholder: address`
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


<a id="0x1_vesting_UnlockRewardsEvent"></a>

## Struct `UnlockRewardsEvent`



```move
module 0x1::vesting {
    struct UnlockRewardsEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_VestEvent"></a>

## Struct `VestEvent`



```move
module 0x1::vesting {
    struct VestEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`staking_pool_address: address`
</dt>
<dd>

</dd>
<dt>
`period_vested: u64`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_DistributeEvent"></a>

## Struct `DistributeEvent`



```move
module 0x1::vesting {
    struct DistributeEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_TerminateEvent"></a>

## Struct `TerminateEvent`



```move
module 0x1::vesting {
    struct TerminateEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vesting_AdminWithdrawEvent"></a>

## Struct `AdminWithdrawEvent`



```move
module 0x1::vesting {
    struct AdminWithdrawEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`admin: address`
</dt>
<dd>

</dd>
<dt>
`vesting_contract_address: address`
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


<a id="0x1_vesting_EEMPTY_VESTING_SCHEDULE"></a>

Vesting schedule cannot be empty.


```move
module 0x1::vesting {
    const EEMPTY_VESTING_SCHEDULE: u64 = 2;
}
```


<a id="0x1_vesting_EINVALID_WITHDRAWAL_ADDRESS"></a>

Withdrawal address is invalid.


```move
module 0x1::vesting {
    const EINVALID_WITHDRAWAL_ADDRESS: u64 = 1;
}
```


<a id="0x1_vesting_ENOT_ADMIN"></a>

The signer is not the admin of the vesting contract.


```move
module 0x1::vesting {
    const ENOT_ADMIN: u64 = 7;
}
```


<a id="0x1_vesting_ENO_SHAREHOLDERS"></a>

Shareholders list cannot be empty.


```move
module 0x1::vesting {
    const ENO_SHAREHOLDERS: u64 = 4;
}
```


<a id="0x1_vesting_EPENDING_STAKE_FOUND"></a>

Cannot terminate the vesting contract with pending active stake. Need to wait until next epoch.


```move
module 0x1::vesting {
    const EPENDING_STAKE_FOUND: u64 = 11;
}
```


<a id="0x1_vesting_EPERMISSION_DENIED"></a>

Account is not admin or does not have the required role to take this action.


```move
module 0x1::vesting {
    const EPERMISSION_DENIED: u64 = 15;
}
```


<a id="0x1_vesting_EROLE_NOT_FOUND"></a>

The vesting account has no such management role.


```move
module 0x1::vesting {
    const EROLE_NOT_FOUND: u64 = 14;
}
```


<a id="0x1_vesting_ESHARES_LENGTH_MISMATCH"></a>

The length of shareholders and shares lists don&apos;t match.


```move
module 0x1::vesting {
    const ESHARES_LENGTH_MISMATCH: u64 = 5;
}
```


<a id="0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION"></a>

Zero items were provided to a &#42;_many function.


```move
module 0x1::vesting {
    const EVEC_EMPTY_FOR_MANY_FUNCTION: u64 = 16;
}
```


<a id="0x1_vesting_EVESTING_ACCOUNT_HAS_NO_ROLES"></a>

Vesting account has no other management roles beside admin.


```move
module 0x1::vesting {
    const EVESTING_ACCOUNT_HAS_NO_ROLES: u64 = 13;
}
```


<a id="0x1_vesting_EVESTING_CONTRACT_NOT_ACTIVE"></a>

Vesting contract needs to be in active state.


```move
module 0x1::vesting {
    const EVESTING_CONTRACT_NOT_ACTIVE: u64 = 8;
}
```


<a id="0x1_vesting_EVESTING_CONTRACT_NOT_FOUND"></a>

No vesting contract found at provided address.


```move
module 0x1::vesting {
    const EVESTING_CONTRACT_NOT_FOUND: u64 = 10;
}
```


<a id="0x1_vesting_EVESTING_CONTRACT_STILL_ACTIVE"></a>

Admin can only withdraw from an inactive (paused or terminated) vesting contract.


```move
module 0x1::vesting {
    const EVESTING_CONTRACT_STILL_ACTIVE: u64 = 9;
}
```


<a id="0x1_vesting_EVESTING_START_TOO_SOON"></a>

Vesting cannot start before or at the current block timestamp. Has to be in the future.


```move
module 0x1::vesting {
    const EVESTING_START_TOO_SOON: u64 = 6;
}
```


<a id="0x1_vesting_EZERO_GRANT"></a>

Grant amount cannot be 0.


```move
module 0x1::vesting {
    const EZERO_GRANT: u64 = 12;
}
```


<a id="0x1_vesting_EZERO_VESTING_SCHEDULE_PERIOD"></a>

Vesting period cannot be 0.


```move
module 0x1::vesting {
    const EZERO_VESTING_SCHEDULE_PERIOD: u64 = 3;
}
```


<a id="0x1_vesting_MAXIMUM_SHAREHOLDERS"></a>

Maximum number of shareholders a vesting pool can support.


```move
module 0x1::vesting {
    const MAXIMUM_SHAREHOLDERS: u64 = 30;
}
```


<a id="0x1_vesting_ROLE_BENEFICIARY_RESETTER"></a>

Roles that can manage certain aspects of the vesting account beyond the main admin.


```move
module 0x1::vesting {
    const ROLE_BENEFICIARY_RESETTER: vector<u8> = [82, 79, 76, 69, 95, 66, 69, 78, 69, 70, 73, 67, 73, 65, 82, 89, 95, 82, 69, 83, 69, 84, 84, 69, 82];
}
```


<a id="0x1_vesting_VESTING_POOL_ACTIVE"></a>

Vesting contract states.
Vesting contract is active and distributions can be made.


```move
module 0x1::vesting {
    const VESTING_POOL_ACTIVE: u64 = 1;
}
```


<a id="0x1_vesting_VESTING_POOL_SALT"></a>



```move
module 0x1::vesting {
    const VESTING_POOL_SALT: vector<u8> = [97, 112, 116, 111, 115, 95, 102, 114, 97, 109, 101, 119, 111, 114, 107, 58, 58, 118, 101, 115, 116, 105, 110, 103];
}
```


<a id="0x1_vesting_VESTING_POOL_TERMINATED"></a>

Vesting contract has been terminated and all funds have been released back to the withdrawal address.


```move
module 0x1::vesting {
    const VESTING_POOL_TERMINATED: u64 = 2;
}
```


<a id="0x1_vesting_stake_pool_address"></a>

## Function `stake_pool_address`

Return the address of the underlying stake pool (separate resource account) of the vesting contract.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun stake_pool_address(vesting_contract_address: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun stake_pool_address(vesting_contract_address: address): address acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).staking.pool_address
    }
}
```


<a id="0x1_vesting_vesting_start_secs"></a>

## Function `vesting_start_secs`

Return the vesting start timestamp (in seconds) of the vesting contract.
Vesting will start at this time, and once a full period has passed, the first vest will become unlocked.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun vesting_start_secs(vesting_contract_address: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun vesting_start_secs(vesting_contract_address: address): u64 acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).vesting_schedule.start_timestamp_secs
    }
}
```


<a id="0x1_vesting_period_duration_secs"></a>

## Function `period_duration_secs`

Return the duration of one vesting period (in seconds).
Each vest is released after one full period has started, starting from the specified start_timestamp_secs.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun period_duration_secs(vesting_contract_address: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun period_duration_secs(vesting_contract_address: address): u64 acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).vesting_schedule.period_duration
    }
}
```


<a id="0x1_vesting_remaining_grant"></a>

## Function `remaining_grant`

Return the remaining grant, consisting of unvested coins that have not been distributed to shareholders.
Prior to start_timestamp_secs, the remaining grant will always be equal to the original grant.
Once vesting has started, and vested tokens are distributed, the remaining grant will decrease over time,
according to the vesting schedule.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun remaining_grant(vesting_contract_address: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun remaining_grant(vesting_contract_address: address): u64 acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).remaining_grant
    }
}
```


<a id="0x1_vesting_beneficiary"></a>

## Function `beneficiary`

Return the beneficiary account of the specified shareholder in a vesting contract.
This is the same as the shareholder address by default and only different if it&apos;s been explicitly set.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun beneficiary(vesting_contract_address: address, shareholder: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun beneficiary(vesting_contract_address: address, shareholder: address): address acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        get_beneficiary(borrow_global<VestingContract>(vesting_contract_address), shareholder)
    }
}
```


<a id="0x1_vesting_operator_commission_percentage"></a>

## Function `operator_commission_percentage`

Return the percentage of accumulated rewards that is paid to the operator as commission.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun operator_commission_percentage(vesting_contract_address: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun operator_commission_percentage(vesting_contract_address: address): u64 acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).staking.commission_percentage
    }
}
```


<a id="0x1_vesting_vesting_contracts"></a>

## Function `vesting_contracts`

Return all the vesting contracts a given address is an admin of.


```move
module 0x1::vesting {
    #[view]
    public fun vesting_contracts(admin: address): vector<address>
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun vesting_contracts(admin: address): vector<address> acquires AdminStore {
        if (!exists<AdminStore>(admin)) {
            vector::empty<address>()
        } else {
            borrow_global<AdminStore>(admin).vesting_contracts
        }
    }
}
```


<a id="0x1_vesting_operator"></a>

## Function `operator`

Return the operator who runs the validator for the vesting contract.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun operator(vesting_contract_address: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun operator(vesting_contract_address: address): address acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).staking.operator
    }
}
```


<a id="0x1_vesting_voter"></a>

## Function `voter`

Return the voter who will be voting on on&#45;chain governance proposals on behalf of the vesting contract&apos;s stake
pool.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun voter(vesting_contract_address: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun voter(vesting_contract_address: address): address acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).staking.voter
    }
}
```


<a id="0x1_vesting_vesting_schedule"></a>

## Function `vesting_schedule`

Return the vesting contract&apos;s vesting schedule. The core schedule is represented as a list of u64&#45;based
fractions, where the rightmmost 32 bits can be divided by 2^32 to get the fraction, and anything else is the
whole number.

For example 3/48, or 0.0625, will be represented as 268435456. The fractional portion would be
268435456 / 2^32 &#61; 0.0625. Since there are fewer than 32 bits, the whole number portion is effectively 0.
So 268435456 &#61; 0.0625.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun vesting_schedule(vesting_contract_address: address): vesting::VestingSchedule
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun vesting_schedule(vesting_contract_address: address): VestingSchedule acquires VestingContract {
        assert_vesting_contract_exists(vesting_contract_address);
        borrow_global<VestingContract>(vesting_contract_address).vesting_schedule
    }
}
```


<a id="0x1_vesting_total_accumulated_rewards"></a>

## Function `total_accumulated_rewards`

Return the total accumulated rewards that have not been distributed to shareholders of the vesting contract.
This excludes any unpaid commission that the operator has not collected.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun total_accumulated_rewards(vesting_contract_address: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun total_accumulated_rewards(vesting_contract_address: address): u64 acquires VestingContract {
        assert_active_vesting_contract(vesting_contract_address);

        let vesting_contract = borrow_global<VestingContract>(vesting_contract_address);
        let (total_active_stake, _, commission_amount) =
            staking_contract::staking_contract_amounts(vesting_contract_address, vesting_contract.staking.operator);
        total_active_stake - vesting_contract.remaining_grant - commission_amount
    }
}
```


<a id="0x1_vesting_accumulated_rewards"></a>

## Function `accumulated_rewards`

Return the accumulated rewards that have not been distributed to the provided shareholder. Caller can also pass
the beneficiary address instead of shareholder address.

This errors out if the vesting contract with the provided address doesn&apos;t exist.


```move
module 0x1::vesting {
    #[view]
    public fun accumulated_rewards(vesting_contract_address: address, shareholder_or_beneficiary: address): u64
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun accumulated_rewards(
        vesting_contract_address: address, shareholder_or_beneficiary: address): u64 acquires VestingContract {
        assert_active_vesting_contract(vesting_contract_address);

        let total_accumulated_rewards = total_accumulated_rewards(vesting_contract_address);
        let shareholder = shareholder(vesting_contract_address, shareholder_or_beneficiary);
        let vesting_contract = borrow_global<VestingContract>(vesting_contract_address);
        let shares = pool_u64::shares(&vesting_contract.grant_pool, shareholder);
        pool_u64::shares_to_amount_with_total_coins(&vesting_contract.grant_pool, shares, total_accumulated_rewards)
    }
}
```


<a id="0x1_vesting_shareholders"></a>

## Function `shareholders`

Return the list of all shareholders in the vesting contract.


```move
module 0x1::vesting {
    #[view]
    public fun shareholders(vesting_contract_address: address): vector<address>
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun shareholders(vesting_contract_address: address): vector<address> acquires VestingContract {
        assert_active_vesting_contract(vesting_contract_address);

        let vesting_contract = borrow_global<VestingContract>(vesting_contract_address);
        pool_u64::shareholders(&vesting_contract.grant_pool)
    }
}
```


<a id="0x1_vesting_shareholder"></a>

## Function `shareholder`

Return the shareholder address given the beneficiary address in a given vesting contract. If there are multiple
shareholders with the same beneficiary address, only the first shareholder is returned. If the given beneficiary
address is actually a shareholder address, just return the address back.

This returns 0x0 if no shareholder is found for the given beneficiary / the address is not a shareholder itself.


```move
module 0x1::vesting {
    #[view]
    public fun shareholder(vesting_contract_address: address, shareholder_or_beneficiary: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun shareholder(
        vesting_contract_address: address,
        shareholder_or_beneficiary: address
    ): address acquires VestingContract {
        assert_active_vesting_contract(vesting_contract_address);

        let shareholders = &shareholders(vesting_contract_address);
        if (vector::contains(shareholders, &shareholder_or_beneficiary)) {
            return shareholder_or_beneficiary
        };
        let vesting_contract = borrow_global<VestingContract>(vesting_contract_address);
        let result = @0x0;
        vector::any(shareholders, |shareholder| {
            if (shareholder_or_beneficiary == get_beneficiary(vesting_contract, *shareholder)) {
                result = *shareholder;
                true
            } else {
                false
            }
        });

        result
    }
}
```


<a id="0x1_vesting_create_vesting_schedule"></a>

## Function `create_vesting_schedule`

Create a vesting schedule with the given schedule of distributions, a vesting start time and period duration.


```move
module 0x1::vesting {
    public fun create_vesting_schedule(schedule: vector<fixed_point32::FixedPoint32>, start_timestamp_secs: u64, period_duration: u64): vesting::VestingSchedule
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun create_vesting_schedule(
        schedule: vector<FixedPoint32>,
        start_timestamp_secs: u64,
        period_duration: u64,
    ): VestingSchedule {
        assert!(vector::length(&schedule) > 0, error::invalid_argument(EEMPTY_VESTING_SCHEDULE));
        assert!(period_duration > 0, error::invalid_argument(EZERO_VESTING_SCHEDULE_PERIOD));
        assert!(
            start_timestamp_secs >= timestamp::now_seconds(),
            error::invalid_argument(EVESTING_START_TOO_SOON),
        );

        VestingSchedule {
            schedule,
            start_timestamp_secs,
            period_duration,
            last_vested_period: 0,
        }
    }
}
```


<a id="0x1_vesting_create_vesting_contract"></a>

## Function `create_vesting_contract`

Create a vesting contract with a given configurations.


```move
module 0x1::vesting {
    public fun create_vesting_contract(admin: &signer, shareholders: &vector<address>, buy_ins: simple_map::SimpleMap<address, coin::Coin<aptos_coin::AptosCoin>>, vesting_schedule: vesting::VestingSchedule, withdrawal_address: address, operator: address, voter: address, commission_percentage: u64, contract_creation_seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun create_vesting_contract(
        admin: &signer,
        shareholders: &vector<address>,
        buy_ins: SimpleMap<address, Coin<AptosCoin>>,
        vesting_schedule: VestingSchedule,
        withdrawal_address: address,
        operator: address,
        voter: address,
        commission_percentage: u64,
        // Optional seed used when creating the staking contract account.
        contract_creation_seed: vector<u8>,
    ): address acquires AdminStore {
        assert!(
            !system_addresses::is_reserved_address(withdrawal_address),
            error::invalid_argument(EINVALID_WITHDRAWAL_ADDRESS),
        );
        assert_account_is_registered_for_apt(withdrawal_address);
        assert!(vector::length(shareholders) > 0, error::invalid_argument(ENO_SHAREHOLDERS));
        assert!(
            simple_map::length(&buy_ins) == vector::length(shareholders),
            error::invalid_argument(ESHARES_LENGTH_MISMATCH),
        );

        // Create a coins pool to track shareholders and shares of the grant.
        let grant = coin::zero<AptosCoin>();
        let grant_amount = 0;
        let grant_pool = pool_u64::create(MAXIMUM_SHAREHOLDERS);
        vector::for_each_ref(shareholders, |shareholder| {
            let shareholder: address = *shareholder;
            let (_, buy_in) = simple_map::remove(&mut buy_ins, &shareholder);
            let buy_in_amount = coin::value(&buy_in);
            coin::merge(&mut grant, buy_in);
            pool_u64::buy_in(
                &mut grant_pool,
                shareholder,
                buy_in_amount,
            );
            grant_amount = grant_amount + buy_in_amount;
        });
        assert!(grant_amount > 0, error::invalid_argument(EZERO_GRANT));

        // If this is the first time this admin account has created a vesting contract, initialize the admin store.
        let admin_address = signer::address_of(admin);
        if (!exists<AdminStore>(admin_address)) {
            move_to(admin, AdminStore {
                vesting_contracts: vector::empty<address>(),
                nonce: 0,
                create_events: new_event_handle<CreateVestingContractEvent>(admin),
            });
        };

        // Initialize the vesting contract in a new resource account. This allows the same admin to create multiple
        // pools.
        let (contract_signer, contract_signer_cap) = create_vesting_contract_account(admin, contract_creation_seed);
        let pool_address = staking_contract::create_staking_contract_with_coins(
            &contract_signer, operator, voter, grant, commission_percentage, contract_creation_seed);

        // Add the newly created vesting contract's address to the admin store.
        let contract_address = signer::address_of(&contract_signer);
        let admin_store = borrow_global_mut<AdminStore>(admin_address);
        vector::push_back(&mut admin_store.vesting_contracts, contract_address);
        if (std::features::module_event_migration_enabled()) {
            emit(
                CreateVestingContract {
                    operator,
                    voter,
                    withdrawal_address,
                    grant_amount,
                    vesting_contract_address: contract_address,
                    staking_pool_address: pool_address,
                    commission_percentage,
                },
            );
        };
        emit_event(
            &mut admin_store.create_events,
            CreateVestingContractEvent {
                operator,
                voter,
                withdrawal_address,
                grant_amount,
                vesting_contract_address: contract_address,
                staking_pool_address: pool_address,
                commission_percentage,
            },
        );

        move_to(&contract_signer, VestingContract {
            state: VESTING_POOL_ACTIVE,
            admin: admin_address,
            grant_pool,
            beneficiaries: simple_map::create<address, address>(),
            vesting_schedule,
            withdrawal_address,
            staking: StakingInfo { pool_address, operator, voter, commission_percentage },
            remaining_grant: grant_amount,
            signer_cap: contract_signer_cap,
            update_operator_events: new_event_handle<UpdateOperatorEvent>(&contract_signer),
            update_voter_events: new_event_handle<UpdateVoterEvent>(&contract_signer),
            reset_lockup_events: new_event_handle<ResetLockupEvent>(&contract_signer),
            set_beneficiary_events: new_event_handle<SetBeneficiaryEvent>(&contract_signer),
            unlock_rewards_events: new_event_handle<UnlockRewardsEvent>(&contract_signer),
            vest_events: new_event_handle<VestEvent>(&contract_signer),
            distribute_events: new_event_handle<DistributeEvent>(&contract_signer),
            terminate_events: new_event_handle<TerminateEvent>(&contract_signer),
            admin_withdraw_events: new_event_handle<AdminWithdrawEvent>(&contract_signer),
        });

        simple_map::destroy_empty(buy_ins);
        contract_address
    }
}
```


<a id="0x1_vesting_unlock_rewards"></a>

## Function `unlock_rewards`

Unlock any accumulated rewards.


```move
module 0x1::vesting {
    public entry fun unlock_rewards(contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun unlock_rewards(contract_address: address) acquires VestingContract {
        let accumulated_rewards = total_accumulated_rewards(contract_address);
        let vesting_contract = borrow_global<VestingContract>(contract_address);
        unlock_stake(vesting_contract, accumulated_rewards);
    }
}
```


<a id="0x1_vesting_unlock_rewards_many"></a>

## Function `unlock_rewards_many`

Call `unlock_rewards` for many vesting contracts.


```move
module 0x1::vesting {
    public entry fun unlock_rewards_many(contract_addresses: vector<address>)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun unlock_rewards_many(contract_addresses: vector<address>) acquires VestingContract {
        let len = vector::length(&contract_addresses);

        assert!(len != 0, error::invalid_argument(EVEC_EMPTY_FOR_MANY_FUNCTION));

        vector::for_each_ref(&contract_addresses, |contract_address| {
            let contract_address: address = *contract_address;
            unlock_rewards(contract_address);
        });
    }
}
```


<a id="0x1_vesting_vest"></a>

## Function `vest`

Unlock any vested portion of the grant.


```move
module 0x1::vesting {
    public entry fun vest(contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun vest(contract_address: address) acquires VestingContract {
        // Unlock all rewards first, if any.
        unlock_rewards(contract_address);

        // Unlock the vested amount. This amount will become withdrawable when the underlying stake pool's lockup
        // expires.
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        // Short-circuit if vesting hasn't started yet.
        if (vesting_contract.vesting_schedule.start_timestamp_secs > timestamp::now_seconds()) {
            return
        };

        // Check if the next vested period has already passed. If not, short-circuit since there's nothing to vest.
        let vesting_schedule = &mut vesting_contract.vesting_schedule;
        let last_vested_period = vesting_schedule.last_vested_period;
        let next_period_to_vest = last_vested_period + 1;
        let last_completed_period =
            (timestamp::now_seconds() - vesting_schedule.start_timestamp_secs) / vesting_schedule.period_duration;
        if (last_completed_period < next_period_to_vest) {
            return
        };

        // Calculate how much has vested, excluding rewards.
        // Index is 0-based while period is 1-based so we need to subtract 1.
        let schedule = &vesting_schedule.schedule;
        let schedule_index = next_period_to_vest - 1;
        let vesting_fraction = if (schedule_index < vector::length(schedule)) {
            *vector::borrow(schedule, schedule_index)
        } else {
            // Last vesting schedule fraction will repeat until the grant runs out.
            *vector::borrow(schedule, vector::length(schedule) - 1)
        };
        let total_grant = pool_u64::total_coins(&vesting_contract.grant_pool);
        let vested_amount = fixed_point32::multiply_u64(total_grant, vesting_fraction);
        // Cap vested amount by the remaining grant amount so we don't try to distribute more than what's remaining.
        vested_amount = min(vested_amount, vesting_contract.remaining_grant);
        vesting_contract.remaining_grant = vesting_contract.remaining_grant - vested_amount;
        vesting_schedule.last_vested_period = next_period_to_vest;
        unlock_stake(vesting_contract, vested_amount);

        if (std::features::module_event_migration_enabled()) {
            emit(
                Vest {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    staking_pool_address: vesting_contract.staking.pool_address,
                    period_vested: next_period_to_vest,
                    amount: vested_amount,
                },
            );
        };
        emit_event(
            &mut vesting_contract.vest_events,
            VestEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                staking_pool_address: vesting_contract.staking.pool_address,
                period_vested: next_period_to_vest,
                amount: vested_amount,
            },
        );
    }
}
```


<a id="0x1_vesting_vest_many"></a>

## Function `vest_many`

Call `vest` for many vesting contracts.


```move
module 0x1::vesting {
    public entry fun vest_many(contract_addresses: vector<address>)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun vest_many(contract_addresses: vector<address>) acquires VestingContract {
        let len = vector::length(&contract_addresses);

        assert!(len != 0, error::invalid_argument(EVEC_EMPTY_FOR_MANY_FUNCTION));

        vector::for_each_ref(&contract_addresses, |contract_address| {
            let contract_address = *contract_address;
            vest(contract_address);
        });
    }
}
```


<a id="0x1_vesting_distribute"></a>

## Function `distribute`

Distribute any withdrawable stake from the stake pool.


```move
module 0x1::vesting {
    public entry fun distribute(contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun distribute(contract_address: address) acquires VestingContract {
        assert_active_vesting_contract(contract_address);

        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        let coins = withdraw_stake(vesting_contract, contract_address);
        let total_distribution_amount = coin::value(&coins);
        if (total_distribution_amount == 0) {
            coin::destroy_zero(coins);
            return
        };

        // Distribute coins to all shareholders in the vesting contract.
        let grant_pool = &vesting_contract.grant_pool;
        let shareholders = &pool_u64::shareholders(grant_pool);
        vector::for_each_ref(shareholders, |shareholder| {
            let shareholder = *shareholder;
            let shares = pool_u64::shares(grant_pool, shareholder);
            let amount = pool_u64::shares_to_amount_with_total_coins(grant_pool, shares, total_distribution_amount);
            let share_of_coins = coin::extract(&mut coins, amount);
            let recipient_address = get_beneficiary(vesting_contract, shareholder);
            aptos_account::deposit_coins(recipient_address, share_of_coins);
        });

        // Send any remaining "dust" (leftover due to rounding error) to the withdrawal address.
        if (coin::value(&coins) > 0) {
            aptos_account::deposit_coins(vesting_contract.withdrawal_address, coins);
        } else {
            coin::destroy_zero(coins);
        };

        if (std::features::module_event_migration_enabled()) {
            emit(
                Distribute {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    amount: total_distribution_amount,
                },
            );
        };
        emit_event(
            &mut vesting_contract.distribute_events,
            DistributeEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                amount: total_distribution_amount,
            },
        );
    }
}
```


<a id="0x1_vesting_distribute_many"></a>

## Function `distribute_many`

Call `distribute` for many vesting contracts.


```move
module 0x1::vesting {
    public entry fun distribute_many(contract_addresses: vector<address>)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun distribute_many(contract_addresses: vector<address>) acquires VestingContract {
        let len = vector::length(&contract_addresses);

        assert!(len != 0, error::invalid_argument(EVEC_EMPTY_FOR_MANY_FUNCTION));

        vector::for_each_ref(&contract_addresses, |contract_address| {
            let contract_address = *contract_address;
            distribute(contract_address);
        });
    }
}
```


<a id="0x1_vesting_terminate_vesting_contract"></a>

## Function `terminate_vesting_contract`

Terminate the vesting contract and send all funds back to the withdrawal address.


```move
module 0x1::vesting {
    public entry fun terminate_vesting_contract(admin: &signer, contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun terminate_vesting_contract(admin: &signer, contract_address: address) acquires VestingContract {
        assert_active_vesting_contract(contract_address);

        // Distribute all withdrawable coins, which should have been from previous rewards withdrawal or vest.
        distribute(contract_address);

        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let (active_stake, _, pending_active_stake, _) = stake::get_stake(vesting_contract.staking.pool_address);
        assert!(pending_active_stake == 0, error::invalid_state(EPENDING_STAKE_FOUND));

        // Unlock all remaining active stake.
        vesting_contract.state = VESTING_POOL_TERMINATED;
        vesting_contract.remaining_grant = 0;
        unlock_stake(vesting_contract, active_stake);

        if (std::features::module_event_migration_enabled()) {
            emit(
                Terminate {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                },
            );
        };
        emit_event(
            &mut vesting_contract.terminate_events,
            TerminateEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
            },
        );
    }
}
```


<a id="0x1_vesting_admin_withdraw"></a>

## Function `admin_withdraw`

Withdraw all funds to the preset vesting contract&apos;s withdrawal address. This can only be called if the contract
has already been terminated.


```move
module 0x1::vesting {
    public entry fun admin_withdraw(admin: &signer, contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun admin_withdraw(admin: &signer, contract_address: address) acquires VestingContract {
        let vesting_contract = borrow_global<VestingContract>(contract_address);
        assert!(
            vesting_contract.state == VESTING_POOL_TERMINATED,
            error::invalid_state(EVESTING_CONTRACT_STILL_ACTIVE)
        );

        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let coins = withdraw_stake(vesting_contract, contract_address);
        let amount = coin::value(&coins);
        if (amount == 0) {
            coin::destroy_zero(coins);
            return
        };
        aptos_account::deposit_coins(vesting_contract.withdrawal_address, coins);

        if (std::features::module_event_migration_enabled()) {
            emit(
                AdminWithdraw {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    amount,
                },
            );
        };
        emit_event(
            &mut vesting_contract.admin_withdraw_events,
            AdminWithdrawEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                amount,
            },
        );
    }
}
```


<a id="0x1_vesting_update_operator"></a>

## Function `update_operator`



```move
module 0x1::vesting {
    public entry fun update_operator(admin: &signer, contract_address: address, new_operator: address, commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun update_operator(
        admin: &signer,
        contract_address: address,
        new_operator: address,
        commission_percentage: u64,
    ) acquires VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        let old_operator = vesting_contract.staking.operator;
        staking_contract::switch_operator(contract_signer, old_operator, new_operator, commission_percentage);
        vesting_contract.staking.operator = new_operator;
        vesting_contract.staking.commission_percentage = commission_percentage;

        if (std::features::module_event_migration_enabled()) {
            emit(
                UpdateOperator {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    staking_pool_address: vesting_contract.staking.pool_address,
                    old_operator,
                    new_operator,
                    commission_percentage,
                },
            );
        };
        emit_event(
            &mut vesting_contract.update_operator_events,
            UpdateOperatorEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                staking_pool_address: vesting_contract.staking.pool_address,
                old_operator,
                new_operator,
                commission_percentage,
            },
        );
    }
}
```


<a id="0x1_vesting_update_operator_with_same_commission"></a>

## Function `update_operator_with_same_commission`



```move
module 0x1::vesting {
    public entry fun update_operator_with_same_commission(admin: &signer, contract_address: address, new_operator: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun update_operator_with_same_commission(
        admin: &signer,
        contract_address: address,
        new_operator: address,
    ) acquires VestingContract {
        let commission_percentage = operator_commission_percentage(contract_address);
        update_operator(admin, contract_address, new_operator, commission_percentage);
    }
}
```


<a id="0x1_vesting_update_commission_percentage"></a>

## Function `update_commission_percentage`



```move
module 0x1::vesting {
    public entry fun update_commission_percentage(admin: &signer, contract_address: address, new_commission_percentage: u64)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun update_commission_percentage(
        admin: &signer,
        contract_address: address,
        new_commission_percentage: u64,
    ) acquires VestingContract {
        let operator = operator(contract_address);
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        staking_contract::update_commision(contract_signer, operator, new_commission_percentage);
        vesting_contract.staking.commission_percentage = new_commission_percentage;
        // This function does not emit an event. Instead, `staking_contract::update_commission_percentage`
        // emits the event for this commission percentage update.
    }
}
```


<a id="0x1_vesting_update_voter"></a>

## Function `update_voter`



```move
module 0x1::vesting {
    public entry fun update_voter(admin: &signer, contract_address: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun update_voter(
        admin: &signer,
        contract_address: address,
        new_voter: address,
    ) acquires VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        let old_voter = vesting_contract.staking.voter;
        staking_contract::update_voter(contract_signer, vesting_contract.staking.operator, new_voter);
        vesting_contract.staking.voter = new_voter;

        if (std::features::module_event_migration_enabled()) {
            emit(
                UpdateVoter {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    staking_pool_address: vesting_contract.staking.pool_address,
                    old_voter,
                    new_voter,
                },
            );
        };
        emit_event(
            &mut vesting_contract.update_voter_events,
            UpdateVoterEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                staking_pool_address: vesting_contract.staking.pool_address,
                old_voter,
                new_voter,
            },
        );
    }
}
```


<a id="0x1_vesting_reset_lockup"></a>

## Function `reset_lockup`



```move
module 0x1::vesting {
    public entry fun reset_lockup(admin: &signer, contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun reset_lockup(
        admin: &signer,
        contract_address: address,
    ) acquires VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        staking_contract::reset_lockup(contract_signer, vesting_contract.staking.operator);

        if (std::features::module_event_migration_enabled()) {
            emit(
                ResetLockup {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    staking_pool_address: vesting_contract.staking.pool_address,
                    new_lockup_expiration_secs: stake::get_lockup_secs(vesting_contract.staking.pool_address),
                },
            );
        };
        emit_event(
            &mut vesting_contract.reset_lockup_events,
            ResetLockupEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                staking_pool_address: vesting_contract.staking.pool_address,
                new_lockup_expiration_secs: stake::get_lockup_secs(vesting_contract.staking.pool_address),
            },
        );
    }
}
```


<a id="0x1_vesting_set_beneficiary"></a>

## Function `set_beneficiary`



```move
module 0x1::vesting {
    public entry fun set_beneficiary(admin: &signer, contract_address: address, shareholder: address, new_beneficiary: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun set_beneficiary(
        admin: &signer,
        contract_address: address,
        shareholder: address,
        new_beneficiary: address,
    ) acquires VestingContract {
        // Verify that the beneficiary account is set up to receive APT. This is a requirement so distribute() wouldn't
        // fail and block all other accounts from receiving APT if one beneficiary is not registered.
        assert_account_is_registered_for_apt(new_beneficiary);

        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);

        let old_beneficiary = get_beneficiary(vesting_contract, shareholder);
        let beneficiaries = &mut vesting_contract.beneficiaries;
        if (simple_map::contains_key(beneficiaries, &shareholder)) {
            let beneficiary = simple_map::borrow_mut(beneficiaries, &shareholder);
            *beneficiary = new_beneficiary;
        } else {
            simple_map::add(beneficiaries, shareholder, new_beneficiary);
        };

        if (std::features::module_event_migration_enabled()) {
            emit(
                SetBeneficiary {
                    admin: vesting_contract.admin,
                    vesting_contract_address: contract_address,
                    shareholder,
                    old_beneficiary,
                    new_beneficiary,
                },
            );
        };
        emit_event(
            &mut vesting_contract.set_beneficiary_events,
            SetBeneficiaryEvent {
                admin: vesting_contract.admin,
                vesting_contract_address: contract_address,
                shareholder,
                old_beneficiary,
                new_beneficiary,
            },
        );
    }
}
```


<a id="0x1_vesting_reset_beneficiary"></a>

## Function `reset_beneficiary`

Remove the beneficiary for the given shareholder. All distributions will sent directly to the shareholder
account.


```move
module 0x1::vesting {
    public entry fun reset_beneficiary(account: &signer, contract_address: address, shareholder: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun reset_beneficiary(
        account: &signer,
        contract_address: address,
        shareholder: address,
    ) acquires VestingAccountManagement, VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        let addr = signer::address_of(account);
        assert!(
            addr == vesting_contract.admin ||
                addr == get_role_holder(contract_address, utf8(ROLE_BENEFICIARY_RESETTER)),
            error::permission_denied(EPERMISSION_DENIED),
        );

        let beneficiaries = &mut vesting_contract.beneficiaries;
        if (simple_map::contains_key(beneficiaries, &shareholder)) {
            simple_map::remove(beneficiaries, &shareholder);
        };
    }
}
```


<a id="0x1_vesting_set_management_role"></a>

## Function `set_management_role`



```move
module 0x1::vesting {
    public entry fun set_management_role(admin: &signer, contract_address: address, role: string::String, role_holder: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun set_management_role(
        admin: &signer,
        contract_address: address,
        role: String,
        role_holder: address,
    ) acquires VestingAccountManagement, VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);

        if (!exists<VestingAccountManagement>(contract_address)) {
            let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
            move_to(contract_signer, VestingAccountManagement {
                roles: simple_map::create<String, address>(),
            })
        };
        let roles = &mut borrow_global_mut<VestingAccountManagement>(contract_address).roles;
        if (simple_map::contains_key(roles, &role)) {
            *simple_map::borrow_mut(roles, &role) = role_holder;
        } else {
            simple_map::add(roles, role, role_holder);
        };
    }
}
```


<a id="0x1_vesting_set_beneficiary_resetter"></a>

## Function `set_beneficiary_resetter`



```move
module 0x1::vesting {
    public entry fun set_beneficiary_resetter(admin: &signer, contract_address: address, beneficiary_resetter: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun set_beneficiary_resetter(
        admin: &signer,
        contract_address: address,
        beneficiary_resetter: address,
    ) acquires VestingAccountManagement, VestingContract {
        set_management_role(admin, contract_address, utf8(ROLE_BENEFICIARY_RESETTER), beneficiary_resetter);
    }
}
```


<a id="0x1_vesting_set_beneficiary_for_operator"></a>

## Function `set_beneficiary_for_operator`

Set the beneficiary for the operator.


```move
module 0x1::vesting {
    public entry fun set_beneficiary_for_operator(operator: &signer, new_beneficiary: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    public entry fun set_beneficiary_for_operator(
        operator: &signer,
        new_beneficiary: address,
    ) {
        staking_contract::set_beneficiary_for_operator(operator, new_beneficiary);
    }
}
```


<a id="0x1_vesting_get_role_holder"></a>

## Function `get_role_holder`



```move
module 0x1::vesting {
    public fun get_role_holder(contract_address: address, role: string::String): address
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun get_role_holder(contract_address: address, role: String): address acquires VestingAccountManagement {
        assert!(exists<VestingAccountManagement>(contract_address), error::not_found(EVESTING_ACCOUNT_HAS_NO_ROLES));
        let roles = &borrow_global<VestingAccountManagement>(contract_address).roles;
        assert!(simple_map::contains_key(roles, &role), error::not_found(EROLE_NOT_FOUND));
        *simple_map::borrow(roles, &role)
    }
}
```


<a id="0x1_vesting_get_vesting_account_signer"></a>

## Function `get_vesting_account_signer`

For emergency use in case the admin needs emergency control of vesting contract account.
This doesn&apos;t give the admin total power as the admin would still need to follow the rules set by
staking_contract and stake modules.


```move
module 0x1::vesting {
    public fun get_vesting_account_signer(admin: &signer, contract_address: address): signer
}
```


##### Implementation


```move
module 0x1::vesting {
    public fun get_vesting_account_signer(admin: &signer, contract_address: address): signer acquires VestingContract {
        let vesting_contract = borrow_global_mut<VestingContract>(contract_address);
        verify_admin(admin, vesting_contract);
        get_vesting_account_signer_internal(vesting_contract)
    }
}
```


<a id="0x1_vesting_get_vesting_account_signer_internal"></a>

## Function `get_vesting_account_signer_internal`



```move
module 0x1::vesting {
    fun get_vesting_account_signer_internal(vesting_contract: &vesting::VestingContract): signer
}
```


##### Implementation


```move
module 0x1::vesting {
    fun get_vesting_account_signer_internal(vesting_contract: &VestingContract): signer {
        account::create_signer_with_capability(&vesting_contract.signer_cap)
    }
}
```


<a id="0x1_vesting_create_vesting_contract_account"></a>

## Function `create_vesting_contract_account`

Create a salt for generating the resource accounts that will be holding the VestingContract.
This address should be deterministic for the same admin and vesting contract creation nonce.


```move
module 0x1::vesting {
    fun create_vesting_contract_account(admin: &signer, contract_creation_seed: vector<u8>): (signer, account::SignerCapability)
}
```


##### Implementation


```move
module 0x1::vesting {
    fun create_vesting_contract_account(
        admin: &signer,
        contract_creation_seed: vector<u8>,
    ): (signer, SignerCapability) acquires AdminStore {
        let admin_store = borrow_global_mut<AdminStore>(signer::address_of(admin));
        let seed = bcs::to_bytes(&signer::address_of(admin));
        vector::append(&mut seed, bcs::to_bytes(&admin_store.nonce));
        admin_store.nonce = admin_store.nonce + 1;

        // Include a salt to avoid conflicts with any other modules out there that might also generate
        // deterministic resource accounts for the same admin address + nonce.
        vector::append(&mut seed, VESTING_POOL_SALT);
        vector::append(&mut seed, contract_creation_seed);

        let (account_signer, signer_cap) = account::create_resource_account(admin, seed);
        // Register the vesting contract account to receive APT as it'll be sent to it when claiming unlocked stake from
        // the underlying staking contract.
        coin::register<AptosCoin>(&account_signer);

        (account_signer, signer_cap)
    }
}
```


<a id="0x1_vesting_verify_admin"></a>

## Function `verify_admin`



```move
module 0x1::vesting {
    fun verify_admin(admin: &signer, vesting_contract: &vesting::VestingContract)
}
```


##### Implementation


```move
module 0x1::vesting {
    fun verify_admin(admin: &signer, vesting_contract: &VestingContract) {
        assert!(signer::address_of(admin) == vesting_contract.admin, error::unauthenticated(ENOT_ADMIN));
    }
}
```


<a id="0x1_vesting_assert_vesting_contract_exists"></a>

## Function `assert_vesting_contract_exists`



```move
module 0x1::vesting {
    fun assert_vesting_contract_exists(contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    fun assert_vesting_contract_exists(contract_address: address) {
        assert!(exists<VestingContract>(contract_address), error::not_found(EVESTING_CONTRACT_NOT_FOUND));
    }
}
```


<a id="0x1_vesting_assert_active_vesting_contract"></a>

## Function `assert_active_vesting_contract`



```move
module 0x1::vesting {
    fun assert_active_vesting_contract(contract_address: address)
}
```


##### Implementation


```move
module 0x1::vesting {
    fun assert_active_vesting_contract(contract_address: address) acquires VestingContract {
        assert_vesting_contract_exists(contract_address);
        let vesting_contract = borrow_global<VestingContract>(contract_address);
        assert!(vesting_contract.state == VESTING_POOL_ACTIVE, error::invalid_state(EVESTING_CONTRACT_NOT_ACTIVE));
    }
}
```


<a id="0x1_vesting_unlock_stake"></a>

## Function `unlock_stake`



```move
module 0x1::vesting {
    fun unlock_stake(vesting_contract: &vesting::VestingContract, amount: u64)
}
```


##### Implementation


```move
module 0x1::vesting {
    fun unlock_stake(vesting_contract: &VestingContract, amount: u64) {
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        staking_contract::unlock_stake(contract_signer, vesting_contract.staking.operator, amount);
    }
}
```


<a id="0x1_vesting_withdraw_stake"></a>

## Function `withdraw_stake`



```move
module 0x1::vesting {
    fun withdraw_stake(vesting_contract: &vesting::VestingContract, contract_address: address): coin::Coin<aptos_coin::AptosCoin>
}
```


##### Implementation


```move
module 0x1::vesting {
    fun withdraw_stake(vesting_contract: &VestingContract, contract_address: address): Coin<AptosCoin> {
        // Claim any withdrawable distribution from the staking contract. The withdrawn coins will be sent directly to
        // the vesting contract's account.
        staking_contract::distribute(contract_address, vesting_contract.staking.operator);
        let withdrawn_coins = coin::balance<AptosCoin>(contract_address);
        let contract_signer = &get_vesting_account_signer_internal(vesting_contract);
        coin::withdraw<AptosCoin>(contract_signer, withdrawn_coins)
    }
}
```


<a id="0x1_vesting_get_beneficiary"></a>

## Function `get_beneficiary`



```move
module 0x1::vesting {
    fun get_beneficiary(contract: &vesting::VestingContract, shareholder: address): address
}
```


##### Implementation


```move
module 0x1::vesting {
    fun get_beneficiary(contract: &VestingContract, shareholder: address): address {
        if (simple_map::contains_key(&contract.beneficiaries, &shareholder)) {
            *simple_map::borrow(&contract.beneficiaries, &shareholder)
        } else {
            shareholder
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
<td>In order to retrieve the address of the underlying stake pool, the vesting start timestamp of the vesting contract, the duration of the vesting period, the remaining grant of a vesting contract, the beneficiary account of a shareholder in a vesting contract, the percentage of accumulated rewards that is paid to the operator as commission, the operator who runs the validator, the voter who will be voting on&#45;chain, and the vesting schedule of a vesting contract, the supplied vesting contract should exist.</td>
<td>Low</td>
<td>The vesting_start_secs, period_duration_secs, remaining_grant, beneficiary, operator_commission_percentage, operator, voter, and vesting_schedule functions ensure that the supplied vesting contract address exists by calling the assert_vesting_contract_exists function.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](assert_vesting_contract_exists).</td>
</tr>

<tr>
<td>2</td>
<td>The vesting pool should not exceed a maximum of 30 shareholders.</td>
<td>Medium</td>
<td>The maximum number of shareholders a vesting pool can support is stored as a constant in MAXIMUM_SHAREHOLDERS which is passed to the pool_u64::create function.</td>
<td>Formally verified via a [#high&#45;level&#45;spec&#45;2](global invariant).</td>
</tr>

<tr>
<td>3</td>
<td>Retrieving all the vesting contracts of a given address and retrieving the list of beneficiaries from a vesting contract should never fail.</td>
<td>Medium</td>
<td>The function vesting_contracts checks if the supplied admin address contains an AdminStore resource and returns all the vesting contracts as a vector&lt;address&gt;. Otherwise it returns an empty vector. The function get_beneficiary checks for a given vesting contract, a specific shareholder exists, and if so, the beneficiary will be returned, otherwise it will simply return the address of the shareholder.</td>
<td>Formally verified via [#high&#45;level&#45;spec&#45;3.1](vesting_contracts) and [#high&#45;level&#45;spec&#45;3.2](get_beneficiary).</td>
</tr>

<tr>
<td>4</td>
<td>The shareholders should be able to start vesting only after the vesting cliff and the first vesting period have transpired.</td>
<td>High</td>
<td>The end of the vesting cliff is stored under VestingContract.vesting_schedule.start_timestamp_secs. The vest function always checks that timestamp::now_seconds is greater or equal to the end of the vesting cliff period.</td>
<td>Audited the check for the end of vesting cliff: [https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/aptos&#45;move/framework/aptos&#45;framework/sources/vesting.move#L566](vest) module.</td>
</tr>

<tr>
<td>5</td>
<td>In order to retrieve the total accumulated rewards that have not been distributed, the accumulated rewards of a given beneficiary, the list of al shareholders in a vesting contract,the shareholder address given the beneficiary address in a given vesting contract, to terminate a vesting contract and to distribute any withdrawable stake from the stake pool, the supplied vesting contract should exist and be active.</td>
<td>Low</td>
<td>The distribute, terminate_vesting_contract, shareholder, shareholders, accumulated_rewards, and total_accumulated_rewards functions ensure that the supplied vesting contract address exists and is active by calling the assert_active_vesting_contract function.</td>
<td>Formally verified via [#high&#45;level&#45;spec&#45;5](ActiveVestingContractAbortsIf).</td>
</tr>

<tr>
<td>6</td>
<td>A new vesting schedule should not be allowed to start vesting in the past or to supply an empty schedule or for the period duration to be zero.</td>
<td>High</td>
<td>The create_vesting_schedule function ensures that the length of the schedule vector is greater than 0, that the period duration is greater than 0 and that the start_timestamp_secs is greater or equal to timestamp::now_seconds.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;6](create_vesting_schedule).</td>
</tr>

<tr>
<td>7</td>
<td>The shareholders should be able to vest the tokens from previous periods.</td>
<td>High</td>
<td>When vesting, the last_completed_period is checked against the next period to vest. This allows to unlock vested tokens for the next period since last vested, in case they didn&apos;t call vest for some periods.</td>
<td>Audited that vesting doesn&apos;t skip periods, but gradually increments to allow shareholders to retrieve all the vested tokens.</td>
</tr>

<tr>
<td>8</td>
<td>Actions such as obtaining a list of shareholders, calculating accrued rewards, distributing withdrawable stake, and terminating the vesting contract should be accessible exclusively while the vesting contract remains active.</td>
<td>Low</td>
<td>Restricting access to inactive vesting contracts is achieved through the assert_active_vesting_contract function.</td>
<td>Formally verified via [#high&#45;level&#45;spec&#45;8](ActiveVestingContractAbortsIf).</td>
</tr>

<tr>
<td>9</td>
<td>The ability to terminate a vesting contract should only be available to the owner.</td>
<td>High</td>
<td>Limiting the access of accounts to specific function, is achieved by asserting that the signer matches the admin of the VestingContract.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;9](verify_admin).</td>
</tr>

<tr>
<td>10</td>
<td>A new vesting contract should not be allowed to have an empty list of shareholders, have a different amount of shareholders than buy&#45;ins, and provide a withdrawal address which is either reserved or not registered for apt.</td>
<td>High</td>
<td>The create_vesting_contract function ensures that the withdrawal_address is not a reserved address, that it is registered for apt, that the list of shareholders is non&#45;empty, and that the amount of shareholders matches the amount of buy_ins.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;10](create_vesting_contract).</td>
</tr>

<tr>
<td>11</td>
<td>Creating a vesting contract account should require the signer (admin) to own an admin store and should enforce that the seed of the resource account is composed of the admin store&apos;s nonce, the vesting pool salt, and the custom contract creation seed.</td>
<td>Medium</td>
<td>The create_vesting_contract_account concatenates to the seed first the admin_store.nonce then the VESTING_POOL_SALT then the contract_creation_seed and then it is passed to the create_resource_account function.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;11](create_vesting_contract_account).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::vesting {
    pragma verify = true;
    pragma aborts_if_is_strict;
// This enforces ### high&#45;level&#45;spec&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    invariant forall a: address where exists<VestingContract>(a):
        global<VestingContract>(a).grant_pool.shareholders_limit <= MAXIMUM_SHAREHOLDERS;
}
```


<a id="@Specification_1_stake_pool_address"></a>

### Function `stake_pool_address`


```move
module 0x1::vesting {
    #[view]
    public fun stake_pool_address(vesting_contract_address: address): address
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_vesting_start_secs"></a>

### Function `vesting_start_secs`


```move
module 0x1::vesting {
    #[view]
    public fun vesting_start_secs(vesting_contract_address: address): u64
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_period_duration_secs"></a>

### Function `period_duration_secs`


```move
module 0x1::vesting {
    #[view]
    public fun period_duration_secs(vesting_contract_address: address): u64
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_remaining_grant"></a>

### Function `remaining_grant`


```move
module 0x1::vesting {
    #[view]
    public fun remaining_grant(vesting_contract_address: address): u64
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_beneficiary"></a>

### Function `beneficiary`


```move
module 0x1::vesting {
    #[view]
    public fun beneficiary(vesting_contract_address: address, shareholder: address): address
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_operator_commission_percentage"></a>

### Function `operator_commission_percentage`


```move
module 0x1::vesting {
    #[view]
    public fun operator_commission_percentage(vesting_contract_address: address): u64
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_vesting_contracts"></a>

### Function `vesting_contracts`


```move
module 0x1::vesting {
    #[view]
    public fun vesting_contracts(admin: address): vector<address>
}
```



```move
module 0x1::vesting {
// This enforces ### high&#45;level&#45;spec&#45;3.1
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if false;
}
```


<a id="@Specification_1_operator"></a>

### Function `operator`


```move
module 0x1::vesting {
    #[view]
    public fun operator(vesting_contract_address: address): address
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_voter"></a>

### Function `voter`


```move
module 0x1::vesting {
    #[view]
    public fun voter(vesting_contract_address: address): address
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_vesting_schedule"></a>

### Function `vesting_schedule`


```move
module 0x1::vesting {
    #[view]
    public fun vesting_schedule(vesting_contract_address: address): vesting::VestingSchedule
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(vesting_contract_address);
}
```


<a id="@Specification_1_total_accumulated_rewards"></a>

### Function `total_accumulated_rewards`


```move
module 0x1::vesting {
    #[view]
    public fun total_accumulated_rewards(vesting_contract_address: address): u64
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include TotalAccumulatedRewardsAbortsIf;
}
```



<a id="0x1_vesting_TotalAccumulatedRewardsAbortsIf"></a>


```move
module 0x1::vesting {
    schema TotalAccumulatedRewardsAbortsIf {
        vesting_contract_address: address;
        requires staking_contract.commission_percentage >= 0 && staking_contract.commission_percentage <= 100;
        include ActiveVestingContractAbortsIf<VestingContract>{contract_address: vesting_contract_address};
        let vesting_contract = global<VestingContract>(vesting_contract_address);
        let staker = vesting_contract_address;
        let operator = vesting_contract.staking.operator;
        let staking_contracts = global<staking_contract::Store>(staker).staking_contracts;
        let staking_contract = simple_map::spec_get(staking_contracts, operator);
        aborts_if !exists<staking_contract::Store>(staker);
        aborts_if !simple_map::spec_contains_key(staking_contracts, operator);
        let pool_address = staking_contract.pool_address;
        let stake_pool = global<stake::StakePool>(pool_address);
        let active = coin::value(stake_pool.active);
        let pending_active = coin::value(stake_pool.pending_active);
        let total_active_stake = active + pending_active;
        let accumulated_rewards = total_active_stake - staking_contract.principal;
        let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;
        aborts_if !exists<stake::StakePool>(pool_address);
        aborts_if active + pending_active > MAX_U64;
        aborts_if total_active_stake < staking_contract.principal;
        aborts_if accumulated_rewards * staking_contract.commission_percentage > MAX_U64;
        aborts_if (vesting_contract.remaining_grant + commission_amount) > total_active_stake;
        aborts_if total_active_stake < vesting_contract.remaining_grant;
    }
}
```


<a id="@Specification_1_accumulated_rewards"></a>

### Function `accumulated_rewards`


```move
module 0x1::vesting {
    #[view]
    public fun accumulated_rewards(vesting_contract_address: address, shareholder_or_beneficiary: address): u64
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include TotalAccumulatedRewardsAbortsIf;
    let vesting_contract = global<VestingContract>(vesting_contract_address);
    let operator = vesting_contract.staking.operator;
    let staking_contracts = global<staking_contract::Store>(vesting_contract_address).staking_contracts;
    let staking_contract = simple_map::spec_get(staking_contracts, operator);
    let pool_address = staking_contract.pool_address;
    let stake_pool = global<stake::StakePool>(pool_address);
    let active = coin::value(stake_pool.active);
    let pending_active = coin::value(stake_pool.pending_active);
    let total_active_stake = active + pending_active;
    let accumulated_rewards = total_active_stake - staking_contract.principal;
    let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;
    let total_accumulated_rewards = total_active_stake - vesting_contract.remaining_grant - commission_amount;
    let shareholder = spec_shareholder(vesting_contract_address, shareholder_or_beneficiary);
    let pool = vesting_contract.grant_pool;
    let shares = pool_u64::spec_shares(pool, shareholder);
    aborts_if pool.total_coins > 0 && pool.total_shares > 0
        && (shares * total_accumulated_rewards) / pool.total_shares > MAX_U64;
    ensures result == pool_u64::spec_shares_to_amount_with_total_coins(pool, shares, total_accumulated_rewards);
}
```


<a id="@Specification_1_shareholders"></a>

### Function `shareholders`


```move
module 0x1::vesting {
    #[view]
    public fun shareholders(vesting_contract_address: address): vector<address>
}
```



```move
module 0x1::vesting {
    include ActiveVestingContractAbortsIf<VestingContract>{contract_address: vesting_contract_address};
}
```



<a id="0x1_vesting_spec_shareholder"></a>


```move
module 0x1::vesting {
    fun spec_shareholder(vesting_contract_address: address, shareholder_or_beneficiary: address): address;
}
```


<a id="@Specification_1_shareholder"></a>

### Function `shareholder`


```move
module 0x1::vesting {
    #[view]
    public fun shareholder(vesting_contract_address: address, shareholder_or_beneficiary: address): address
}
```



```move
module 0x1::vesting {
    pragma opaque;
    include ActiveVestingContractAbortsIf<VestingContract>{contract_address: vesting_contract_address};
    ensures [abstract] result == spec_shareholder(vesting_contract_address, shareholder_or_beneficiary);
}
```


<a id="@Specification_1_create_vesting_schedule"></a>

### Function `create_vesting_schedule`


```move
module 0x1::vesting {
    public fun create_vesting_schedule(schedule: vector<fixed_point32::FixedPoint32>, start_timestamp_secs: u64, period_duration: u64): vesting::VestingSchedule
}
```



```move
module 0x1::vesting {
// This enforces ### high&#45;level&#45;req&#45;6
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !(len(schedule) > 0);
    aborts_if !(period_duration > 0);
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if !(start_timestamp_secs >= timestamp::now_seconds());
}
```


<a id="@Specification_1_create_vesting_contract"></a>

### Function `create_vesting_contract`


```move
module 0x1::vesting {
    public fun create_vesting_contract(admin: &signer, shareholders: &vector<address>, buy_ins: simple_map::SimpleMap<address, coin::Coin<aptos_coin::AptosCoin>>, vesting_schedule: vesting::VestingSchedule, withdrawal_address: address, operator: address, voter: address, commission_percentage: u64, contract_creation_seed: vector<u8>): address
}
```



```move
module 0x1::vesting {
    pragma verify = false;
// This enforces ### high&#45;level&#45;req&#45;10
[#high&#45;level&#45;req](high&#45;level requirement 10):
    aborts_if withdrawal_address == @aptos_framework || withdrawal_address == @vm_reserved;
    aborts_if !exists<account::Account>(withdrawal_address);
    aborts_if !exists<coin::CoinStore<AptosCoin>>(withdrawal_address);
    aborts_if len(shareholders) == 0;
    aborts_if simple_map::spec_len(buy_ins) != len(shareholders);
    ensures global<VestingContract>(result).grant_pool.shareholders_limit == 30;
}
```


<a id="@Specification_1_unlock_rewards"></a>

### Function `unlock_rewards`


```move
module 0x1::vesting {
    public entry fun unlock_rewards(contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include UnlockRewardsAbortsIf;
}
```



<a id="0x1_vesting_UnlockRewardsAbortsIf"></a>


```move
module 0x1::vesting {
    schema UnlockRewardsAbortsIf {
        contract_address: address;
        include TotalAccumulatedRewardsAbortsIf { vesting_contract_address: contract_address };
        let vesting_contract = global<VestingContract>(contract_address);
        let operator = vesting_contract.staking.operator;
        let staking_contracts = global<staking_contract::Store>(contract_address).staking_contracts;
        let staking_contract = simple_map::spec_get(staking_contracts, operator);
        let pool_address = staking_contract.pool_address;
        let stake_pool = global<stake::StakePool>(pool_address);
        let active = coin::value(stake_pool.active);
        let pending_active = coin::value(stake_pool.pending_active);
        let total_active_stake = active + pending_active;
        let accumulated_rewards = total_active_stake - staking_contract.principal;
        let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;
        let amount = total_active_stake - vesting_contract.remaining_grant - commission_amount;
        include UnlockStakeAbortsIf { vesting_contract, amount };
    }
}
```


<a id="@Specification_1_unlock_rewards_many"></a>

### Function `unlock_rewards_many`


```move
module 0x1::vesting {
    public entry fun unlock_rewards_many(contract_addresses: vector<address>)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    aborts_if len(contract_addresses) == 0;
    include PreconditionAbortsIf;
}
```


<a id="@Specification_1_vest"></a>

### Function `vest`


```move
module 0x1::vesting {
    public entry fun vest(contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include UnlockRewardsAbortsIf;
}
```


<a id="@Specification_1_vest_many"></a>

### Function `vest_many`


```move
module 0x1::vesting {
    public entry fun vest_many(contract_addresses: vector<address>)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    aborts_if len(contract_addresses) == 0;
    include PreconditionAbortsIf;
}
```


<a id="@Specification_1_distribute"></a>

### Function `distribute`


```move
module 0x1::vesting {
    public entry fun distribute(contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include ActiveVestingContractAbortsIf<VestingContract>;
    let vesting_contract = global<VestingContract>(contract_address);
    include WithdrawStakeAbortsIf { vesting_contract };
}
```


<a id="@Specification_1_distribute_many"></a>

### Function `distribute_many`


```move
module 0x1::vesting {
    public entry fun distribute_many(contract_addresses: vector<address>)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    aborts_if len(contract_addresses) == 0;
}
```


<a id="@Specification_1_terminate_vesting_contract"></a>

### Function `terminate_vesting_contract`


```move
module 0x1::vesting {
    public entry fun terminate_vesting_contract(admin: &signer, contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include ActiveVestingContractAbortsIf<VestingContract>;
    let vesting_contract = global<VestingContract>(contract_address);
    include WithdrawStakeAbortsIf { vesting_contract };
}
```


<a id="@Specification_1_admin_withdraw"></a>

### Function `admin_withdraw`


```move
module 0x1::vesting {
    public entry fun admin_withdraw(admin: &signer, contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    let vesting_contract = global<VestingContract>(contract_address);
    aborts_if vesting_contract.state != VESTING_POOL_TERMINATED;
    include VerifyAdminAbortsIf;
    include WithdrawStakeAbortsIf { vesting_contract };
}
```


<a id="@Specification_1_update_operator"></a>

### Function `update_operator`


```move
module 0x1::vesting {
    public entry fun update_operator(admin: &signer, contract_address: address, new_operator: address, commission_percentage: u64)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include VerifyAdminAbortsIf;
    let vesting_contract = global<VestingContract>(contract_address);
    let acc = vesting_contract.signer_cap.account;
    let old_operator = vesting_contract.staking.operator;
    include staking_contract::ContractExistsAbortsIf { staker: acc, operator: old_operator };
    let store = global<staking_contract::Store>(acc);
    let staking_contracts = store.staking_contracts;
    aborts_if simple_map::spec_contains_key(staking_contracts, new_operator);
    let staking_contract = simple_map::spec_get(staking_contracts, old_operator);
    include DistributeInternalAbortsIf { staker: acc, operator: old_operator, staking_contract, distribute_events: store.distribute_events };
}
```


<a id="@Specification_1_update_operator_with_same_commission"></a>

### Function `update_operator_with_same_commission`


```move
module 0x1::vesting {
    public entry fun update_operator_with_same_commission(admin: &signer, contract_address: address, new_operator: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
}
```


<a id="@Specification_1_update_commission_percentage"></a>

### Function `update_commission_percentage`


```move
module 0x1::vesting {
    public entry fun update_commission_percentage(admin: &signer, contract_address: address, new_commission_percentage: u64)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
}
```


<a id="@Specification_1_update_voter"></a>

### Function `update_voter`


```move
module 0x1::vesting {
    public entry fun update_voter(admin: &signer, contract_address: address, new_voter: address)
}
```



```move
module 0x1::vesting {
    pragma verify_duration_estimate = 300;
    include VerifyAdminAbortsIf;
    let vesting_contract = global<VestingContract>(contract_address);
    let operator = vesting_contract.staking.operator;
    let staker = vesting_contract.signer_cap.account;
    include staking_contract::UpdateVoterSchema;
}
```


<a id="@Specification_1_reset_lockup"></a>

### Function `reset_lockup`


```move
module 0x1::vesting {
    public entry fun reset_lockup(admin: &signer, contract_address: address)
}
```



```move
module 0x1::vesting {
    pragma verify_duration_estimate = 300;
    aborts_if !exists<VestingContract>(contract_address);
    let vesting_contract = global<VestingContract>(contract_address);
    aborts_if signer::address_of(admin) != vesting_contract.admin;
    let operator = vesting_contract.staking.operator;
    let staker = vesting_contract.signer_cap.account;
    include staking_contract::ContractExistsAbortsIf {staker, operator};
    include staking_contract::IncreaseLockupWithCapAbortsIf {staker, operator};
    let store = global<staking_contract::Store>(staker);
    let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
    let pool_address = staking_contract.owner_cap.pool_address;
    aborts_if !exists<stake::StakePool>(vesting_contract.staking.pool_address);
}
```


<a id="@Specification_1_set_beneficiary"></a>

### Function `set_beneficiary`


```move
module 0x1::vesting {
    public entry fun set_beneficiary(admin: &signer, contract_address: address, shareholder: address, new_beneficiary: address)
}
```



```move
module 0x1::vesting {
    pragma verify_duration_estimate = 300;
    pragma aborts_if_is_partial;
    aborts_if !account::exists_at(new_beneficiary);
    aborts_if !coin::spec_is_account_registered<AptosCoin>(new_beneficiary);
    include VerifyAdminAbortsIf;
    let post vesting_contract = global<VestingContract>(contract_address);
    ensures simple_map::spec_contains_key(vesting_contract.beneficiaries,shareholder);
}
```


<a id="@Specification_1_reset_beneficiary"></a>

### Function `reset_beneficiary`


```move
module 0x1::vesting {
    public entry fun reset_beneficiary(account: &signer, contract_address: address, shareholder: address)
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingContract>(contract_address);
    let addr = signer::address_of(account);
    let vesting_contract = global<VestingContract>(contract_address);
    aborts_if addr != vesting_contract.admin && !std::string::spec_internal_check_utf8(ROLE_BENEFICIARY_RESETTER);
    aborts_if addr != vesting_contract.admin && !exists<VestingAccountManagement>(contract_address);
    let roles = global<VestingAccountManagement>(contract_address).roles;
    let role = std::string::spec_utf8(ROLE_BENEFICIARY_RESETTER);
    aborts_if addr != vesting_contract.admin && !simple_map::spec_contains_key(roles, role);
    aborts_if addr != vesting_contract.admin && addr != simple_map::spec_get(roles, role);
    let post post_vesting_contract = global<VestingContract>(contract_address);
    ensures !simple_map::spec_contains_key(post_vesting_contract.beneficiaries,shareholder);
}
```


<a id="@Specification_1_set_management_role"></a>

### Function `set_management_role`


```move
module 0x1::vesting {
    public entry fun set_management_role(admin: &signer, contract_address: address, role: string::String, role_holder: address)
}
```



```move
module 0x1::vesting {
    pragma aborts_if_is_partial;
    include SetManagementRoleAbortsIf;
}
```


<a id="@Specification_1_set_beneficiary_resetter"></a>

### Function `set_beneficiary_resetter`


```move
module 0x1::vesting {
    public entry fun set_beneficiary_resetter(admin: &signer, contract_address: address, beneficiary_resetter: address)
}
```



```move
module 0x1::vesting {
    pragma aborts_if_is_partial;
    aborts_if !std::string::spec_internal_check_utf8(ROLE_BENEFICIARY_RESETTER);
    include SetManagementRoleAbortsIf;
}
```


<a id="@Specification_1_set_beneficiary_for_operator"></a>

### Function `set_beneficiary_for_operator`


```move
module 0x1::vesting {
    public entry fun set_beneficiary_for_operator(operator: &signer, new_beneficiary: address)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
}
```


<a id="@Specification_1_get_role_holder"></a>

### Function `get_role_holder`


```move
module 0x1::vesting {
    public fun get_role_holder(contract_address: address, role: string::String): address
}
```



```move
module 0x1::vesting {
    aborts_if !exists<VestingAccountManagement>(contract_address);
    let roles = global<VestingAccountManagement>(contract_address).roles;
    aborts_if !simple_map::spec_contains_key(roles,role);
}
```


<a id="@Specification_1_get_vesting_account_signer"></a>

### Function `get_vesting_account_signer`


```move
module 0x1::vesting {
    public fun get_vesting_account_signer(admin: &signer, contract_address: address): signer
}
```



```move
module 0x1::vesting {
    include VerifyAdminAbortsIf;
}
```


<a id="@Specification_1_get_vesting_account_signer_internal"></a>

### Function `get_vesting_account_signer_internal`


```move
module 0x1::vesting {
    fun get_vesting_account_signer_internal(vesting_contract: &vesting::VestingContract): signer
}
```



```move
module 0x1::vesting {
    aborts_if false;
}
```



<a id="0x1_vesting_spec_get_vesting_account_signer"></a>


```move
module 0x1::vesting {
    fun spec_get_vesting_account_signer(vesting_contract: VestingContract): signer;
}
```


<a id="@Specification_1_create_vesting_contract_account"></a>

### Function `create_vesting_contract_account`


```move
module 0x1::vesting {
    fun create_vesting_contract_account(admin: &signer, contract_creation_seed: vector<u8>): (signer, account::SignerCapability)
}
```



```move
module 0x1::vesting {
    pragma verify_duration_estimate = 300;
    let admin_addr = signer::address_of(admin);
    let admin_store = global<AdminStore>(admin_addr);
    let seed = bcs::to_bytes(admin_addr);
    let nonce = bcs::to_bytes(admin_store.nonce);
    let first = concat(seed, nonce);
    let second = concat(first, VESTING_POOL_SALT);
    let end = concat(second, contract_creation_seed);
// This enforces ### high&#45;level&#45;req&#45;11
[#high&#45;level&#45;req](high&#45;level requirement 11):
    let resource_addr = account::spec_create_resource_address(admin_addr, end);
    aborts_if !exists<AdminStore>(admin_addr);
    aborts_if len(account::ZERO_AUTH_KEY) != 32;
    aborts_if admin_store.nonce + 1 > MAX_U64;
    let ea = account::exists_at(resource_addr);
    include if (ea) account::CreateResourceAccountAbortsIf else account::CreateAccountAbortsIf {addr: resource_addr};
    let acc = global<account::Account>(resource_addr);
    let post post_acc = global<account::Account>(resource_addr);
    aborts_if !exists<coin::CoinStore<AptosCoin>>(resource_addr) && !aptos_std::type_info::spec_is_struct<AptosCoin>();
    aborts_if !exists<coin::CoinStore<AptosCoin>>(resource_addr) && ea && acc.guid_creation_num + 2 > MAX_U64;
    aborts_if !exists<coin::CoinStore<AptosCoin>>(resource_addr) && ea && acc.guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
    ensures exists<account::Account>(resource_addr) && post_acc.authentication_key == account::ZERO_AUTH_KEY &&
            exists<coin::CoinStore<AptosCoin>>(resource_addr);
    ensures signer::address_of(result_1) == resource_addr;
    ensures result_2.account == resource_addr;
}
```


<a id="@Specification_1_verify_admin"></a>

### Function `verify_admin`


```move
module 0x1::vesting {
    fun verify_admin(admin: &signer, vesting_contract: &vesting::VestingContract)
}
```



```move
module 0x1::vesting {
// This enforces ### high&#45;level&#45;req&#45;9
[#high&#45;level&#45;req](high&#45;level requirement 9):
    aborts_if signer::address_of(admin) != vesting_contract.admin;
}
```


<a id="@Specification_1_assert_vesting_contract_exists"></a>

### Function `assert_vesting_contract_exists`


```move
module 0x1::vesting {
    fun assert_vesting_contract_exists(contract_address: address)
}
```



```move
module 0x1::vesting {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if !exists<VestingContract>(contract_address);
}
```


<a id="@Specification_1_assert_active_vesting_contract"></a>

### Function `assert_active_vesting_contract`


```move
module 0x1::vesting {
    fun assert_active_vesting_contract(contract_address: address)
}
```



```move
module 0x1::vesting {
    include ActiveVestingContractAbortsIf<VestingContract>;
}
```


<a id="@Specification_1_unlock_stake"></a>

### Function `unlock_stake`


```move
module 0x1::vesting {
    fun unlock_stake(vesting_contract: &vesting::VestingContract, amount: u64)
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include UnlockStakeAbortsIf;
}
```



<a id="0x1_vesting_UnlockStakeAbortsIf"></a>


```move
module 0x1::vesting {
    schema UnlockStakeAbortsIf {
        vesting_contract: &VestingContract;
        amount: u64;
        let acc = vesting_contract.signer_cap.account;
        let operator = vesting_contract.staking.operator;
        include amount != 0 ==> staking_contract::ContractExistsAbortsIf { staker: acc, operator };
        let store = global<staking_contract::Store>(acc);
        let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
        include amount != 0 ==> DistributeInternalAbortsIf { staker: acc, operator, staking_contract, distribute_events: store.distribute_events };
    }
}
```


<a id="@Specification_1_withdraw_stake"></a>

### Function `withdraw_stake`


```move
module 0x1::vesting {
    fun withdraw_stake(vesting_contract: &vesting::VestingContract, contract_address: address): coin::Coin<aptos_coin::AptosCoin>
}
```



```move
module 0x1::vesting {
    pragma verify = false;
    include WithdrawStakeAbortsIf;
}
```



<a id="0x1_vesting_WithdrawStakeAbortsIf"></a>


```move
module 0x1::vesting {
    schema WithdrawStakeAbortsIf {
        vesting_contract: &VestingContract;
        contract_address: address;
        let operator = vesting_contract.staking.operator;
        include staking_contract::ContractExistsAbortsIf { staker: contract_address, operator };
        let store = global<staking_contract::Store>(contract_address);
        let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
        include DistributeInternalAbortsIf { staker: contract_address, operator, staking_contract, distribute_events: store.distribute_events };
    }
}
```



<a id="0x1_vesting_DistributeInternalAbortsIf"></a>


```move
module 0x1::vesting {
    schema DistributeInternalAbortsIf {
        staker: address;
        operator: address;
        staking_contract: staking_contract::StakingContract;
        distribute_events: EventHandle<staking_contract::DistributeEvent>;
        let pool_address = staking_contract.pool_address;
        aborts_if !exists<stake::StakePool>(pool_address);
        let stake_pool = global<stake::StakePool>(pool_address);
        let inactive = stake_pool.inactive.value;
        let pending_inactive = stake_pool.pending_inactive.value;
        aborts_if inactive + pending_inactive > MAX_U64;
        let total_potential_withdrawable = inactive + pending_inactive;
        let pool_address_1 = staking_contract.owner_cap.pool_address;
        aborts_if !exists<stake::StakePool>(pool_address_1);
        let stake_pool_1 = global<stake::StakePool>(pool_address_1);
        aborts_if !exists<stake::ValidatorSet>(@aptos_framework);
        let validator_set = global<stake::ValidatorSet>(@aptos_framework);
        let inactive_state = !stake::spec_contains(validator_set.pending_active, pool_address_1)
            && !stake::spec_contains(validator_set.active_validators, pool_address_1)
            && !stake::spec_contains(validator_set.pending_inactive, pool_address_1);
        let inactive_1 = stake_pool_1.inactive.value;
        let pending_inactive_1 = stake_pool_1.pending_inactive.value;
        let new_inactive_1 = inactive_1 + pending_inactive_1;
        aborts_if inactive_state && timestamp::spec_now_seconds() >= stake_pool_1.locked_until_secs
            && inactive_1 + pending_inactive_1 > MAX_U64;
    }
}
```


<a id="@Specification_1_get_beneficiary"></a>

### Function `get_beneficiary`


```move
module 0x1::vesting {
    fun get_beneficiary(contract: &vesting::VestingContract, shareholder: address): address
}
```



```move
module 0x1::vesting {
// This enforces ### high&#45;level&#45;spec&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if false;
}
```



<a id="0x1_vesting_SetManagementRoleAbortsIf"></a>


```move
module 0x1::vesting {
    schema SetManagementRoleAbortsIf {
        contract_address: address;
        admin: signer;
        aborts_if !exists<VestingContract>(contract_address);
        let vesting_contract = global<VestingContract>(contract_address);
        aborts_if signer::address_of(admin) != vesting_contract.admin;
    }
}
```



<a id="0x1_vesting_VerifyAdminAbortsIf"></a>


```move
module 0x1::vesting {
    schema VerifyAdminAbortsIf {
        contract_address: address;
        admin: signer;
        aborts_if !exists<VestingContract>(contract_address);
        let vesting_contract = global<VestingContract>(contract_address);
        aborts_if signer::address_of(admin) != vesting_contract.admin;
    }
}
```



<a id="0x1_vesting_ActiveVestingContractAbortsIf"></a>


```move
module 0x1::vesting {
    schema ActiveVestingContractAbortsIf<VestingContract> {
        contract_address: address;
    // This enforces ### high&#45;level&#45;spec&#45;5
    [#high&#45;level&#45;req](high&#45;level requirement 5):
        aborts_if !exists<VestingContract>(contract_address);
        let vesting_contract = global<VestingContract>(contract_address);
    // This enforces ### high&#45;level&#45;spec&#45;8
    [#high&#45;level&#45;req](high&#45;level requirement 8):
        aborts_if vesting_contract.state != VESTING_POOL_ACTIVE;
    }
}
```
