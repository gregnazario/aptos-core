
<a id="0x1_staking_proxy"></a>

# Module `0x1::staking_proxy`



-  [Function `set_operator`](#0x1_staking_proxy_set_operator)
-  [Function `set_voter`](#0x1_staking_proxy_set_voter)
-  [Function `set_vesting_contract_operator`](#0x1_staking_proxy_set_vesting_contract_operator)
-  [Function `set_staking_contract_operator`](#0x1_staking_proxy_set_staking_contract_operator)
-  [Function `set_stake_pool_operator`](#0x1_staking_proxy_set_stake_pool_operator)
-  [Function `set_vesting_contract_voter`](#0x1_staking_proxy_set_vesting_contract_voter)
-  [Function `set_staking_contract_voter`](#0x1_staking_proxy_set_staking_contract_voter)
-  [Function `set_stake_pool_voter`](#0x1_staking_proxy_set_stake_pool_voter)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `set_operator`](#@Specification_0_set_operator)
    -  [Function `set_voter`](#@Specification_0_set_voter)
    -  [Function `set_vesting_contract_operator`](#@Specification_0_set_vesting_contract_operator)
    -  [Function `set_staking_contract_operator`](#@Specification_0_set_staking_contract_operator)
    -  [Function `set_stake_pool_operator`](#@Specification_0_set_stake_pool_operator)
    -  [Function `set_vesting_contract_voter`](#@Specification_0_set_vesting_contract_voter)
    -  [Function `set_staking_contract_voter`](#@Specification_0_set_staking_contract_voter)
    -  [Function `set_stake_pool_voter`](#@Specification_0_set_stake_pool_voter)


```move
module 0x1::staking_proxy {
    use 0x1::signer;
    use 0x1::stake;
    use 0x1::staking_contract;
    use 0x1::vesting;
}
```


<a id="0x1_staking_proxy_set_operator"></a>

## Function `set_operator`



```move
module 0x1::staking_proxy {
    public entry fun set_operator(owner: &signer, old_operator: address, new_operator: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_operator(owner: &signer, old_operator: address, new_operator: address) {
        set_vesting_contract_operator(owner, old_operator, new_operator);
        set_staking_contract_operator(owner, old_operator, new_operator);
        set_stake_pool_operator(owner, new_operator);
    }
}
```


<a id="0x1_staking_proxy_set_voter"></a>

## Function `set_voter`



```move
module 0x1::staking_proxy {
    public entry fun set_voter(owner: &signer, operator: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_voter(owner: &signer, operator: address, new_voter: address) {
        set_vesting_contract_voter(owner, operator, new_voter);
        set_staking_contract_voter(owner, operator, new_voter);
        set_stake_pool_voter(owner, new_voter);
    }
}
```


<a id="0x1_staking_proxy_set_vesting_contract_operator"></a>

## Function `set_vesting_contract_operator`



```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_operator(owner: &signer, old_operator: address, new_operator: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_operator(owner: &signer, old_operator: address, new_operator: address) {
        let owner_address = signer::address_of(owner);
        let vesting_contracts = &vesting::vesting_contracts(owner_address);
        vector::for_each_ref(vesting_contracts, |vesting_contract| {
            let vesting_contract = *vesting_contract;
            if (vesting::operator(vesting_contract) == old_operator) {
                let current_commission_percentage = vesting::operator_commission_percentage(vesting_contract);
                vesting::update_operator(owner, vesting_contract, new_operator, current_commission_percentage);
            };
        });
    }
}
```


<a id="0x1_staking_proxy_set_staking_contract_operator"></a>

## Function `set_staking_contract_operator`



```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_operator(owner: &signer, old_operator: address, new_operator: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_operator(owner: &signer, old_operator: address, new_operator: address) {
        let owner_address = signer::address_of(owner);
        if (staking_contract::staking_contract_exists(owner_address, old_operator)) {
            let current_commission_percentage = staking_contract::commission_percentage(owner_address, old_operator);
            staking_contract::switch_operator(owner, old_operator, new_operator, current_commission_percentage);
        };
    }
}
```


<a id="0x1_staking_proxy_set_stake_pool_operator"></a>

## Function `set_stake_pool_operator`



```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_operator(owner: &signer, new_operator: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_operator(owner: &signer, new_operator: address) {
        let owner_address = signer::address_of(owner);
        if (stake::stake_pool_exists(owner_address)) {
            stake::set_operator(owner, new_operator);
        };
    }
}
```


<a id="0x1_staking_proxy_set_vesting_contract_voter"></a>

## Function `set_vesting_contract_voter`



```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_voter(owner: &signer, operator: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_voter(owner: &signer, operator: address, new_voter: address) {
        let owner_address = signer::address_of(owner);
        let vesting_contracts = &vesting::vesting_contracts(owner_address);
        vector::for_each_ref(vesting_contracts, |vesting_contract| {
            let vesting_contract = *vesting_contract;
            if (vesting::operator(vesting_contract) == operator) {
                vesting::update_voter(owner, vesting_contract, new_voter);
            };
        });
    }
}
```


<a id="0x1_staking_proxy_set_staking_contract_voter"></a>

## Function `set_staking_contract_voter`



```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_voter(owner: &signer, operator: address, new_voter: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_voter(owner: &signer, operator: address, new_voter: address) {
        let owner_address = signer::address_of(owner);
        if (staking_contract::staking_contract_exists(owner_address, operator)) {
            staking_contract::update_voter(owner, operator, new_voter);
        };
    }
}
```


<a id="0x1_staking_proxy_set_stake_pool_voter"></a>

## Function `set_stake_pool_voter`



```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_voter(owner: &signer, new_voter: address)
}
```


##### Implementation


```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_voter(owner: &signer, new_voter: address) {
        if (stake::stake_pool_exists(signer::address_of(owner))) {
            stake::set_delegated_voter(owner, new_voter);
        };
    }
}
```


<a id="@Specification_0"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>When updating the Vesting operator, it should be updated throughout all depending units.</td>
<td>Medium</td>
<td>The VestingContract contains a StakingInfo object that has an operator field, and this operator is mapped to a StakingContract object that in turn encompasses a StakePool object where the operator matches.</td>
<td>Audited that it ensures the two operator fields hold the new value after the update.</td>
</tr>

<tr>
<td>2</td>
<td>When updating the Vesting voter, it should be updated throughout all depending units.</td>
<td>Medium</td>
<td>The VestingContract contains a StakingInfo object that has an operator field, and this operator is mapped to a StakingContract object that in turn encompasses a StakePool object where the operator matches.</td>
<td>Audited that it ensures the two operator fields hold the new value after the update.</td>
</tr>

<tr>
<td>3</td>
<td>The operator and voter of a Vesting Contract should only be updated by the owner of the contract.</td>
<td>High</td>
<td>The owner&#45;operator&#45;voter model, as defined in the documentation, grants distinct abilities to each role. Therefore, it&apos;s crucial to ensure that only the owner has the authority to modify the operator or voter, to prevent the compromise of the StakePool.</td>
<td>Audited that it ensures the signer owns the AdminStore resource and that the operator or voter intended for the update actually exists.</td>
</tr>

<tr>
<td>4</td>
<td>The operator and voter of a Staking Contract should only be updated by the owner of the contract.</td>
<td>High</td>
<td>The owner&#45;operator&#45;voter model, as defined in the documentation, grants distinct abilities to each role. Therefore, it&apos;s crucial to ensure that only the owner has the authority to modify the operator or voter, to prevent the compromise of the StakePool.</td>
<td>Audited the patterns of updating operators and voters in the staking contract.</td>
</tr>

<tr>
<td>5</td>
<td>Staking Contract&apos;s operators should be unique inside a store.</td>
<td>Medium</td>
<td>Duplicates among operators could result in incorrectly updating the operator or voter associated with the incorrect StakingContract.</td>
<td>Enforced via [https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/aptos&#45;move/framework/aptos&#45;framework/sources/staking_contract.move#L87](SimpleMap).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::staking_proxy {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_0_set_operator"></a>

### Function `set_operator`


```move
module 0x1::staking_proxy {
    public entry fun set_operator(owner: &signer, old_operator: address, new_operator: address)
}
```

Aborts if conditions of SetStakePoolOperator are not met


```move
module 0x1::staking_proxy {
    pragma verify = false;
    pragma aborts_if_is_partial;
    include SetStakePoolOperator;
    include SetStakingContractOperator;
}
```


<a id="@Specification_0_set_voter"></a>

### Function `set_voter`


```move
module 0x1::staking_proxy {
    public entry fun set_voter(owner: &signer, operator: address, new_voter: address)
}
```

Aborts if conditions of SetStackingContractVoter and SetStackPoolVoterAbortsIf are not met


```move
module 0x1::staking_proxy {
    pragma aborts_if_is_partial;
    include SetStakingContractVoter;
    include SetStakePoolVoterAbortsIf;
}
```


<a id="@Specification_0_set_vesting_contract_operator"></a>

### Function `set_vesting_contract_operator`


```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_operator(owner: &signer, old_operator: address, new_operator: address)
}
```



```move
module 0x1::staking_proxy {
    pragma verify = false;
}
```


<a id="@Specification_0_set_staking_contract_operator"></a>

### Function `set_staking_contract_operator`


```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_operator(owner: &signer, old_operator: address, new_operator: address)
}
```



```move
module 0x1::staking_proxy {
    pragma aborts_if_is_partial;
    pragma verify = false;
    include SetStakingContractOperator;
}
```



<a id="0x1_staking_proxy_SetStakingContractOperator"></a>


```move
module 0x1::staking_proxy {
    schema SetStakingContractOperator {
        owner: signer;
        old_operator: address;
        new_operator: address;
        let owner_address = signer::address_of(owner);
        let store = global<Store>(owner_address);
        let staking_contract_exists = exists<Store>(owner_address) && simple_map::spec_contains_key(store.staking_contracts, old_operator);
        aborts_if staking_contract_exists && simple_map::spec_contains_key(store.staking_contracts, new_operator);
        let post post_store = global<Store>(owner_address);
        ensures staking_contract_exists ==> !simple_map::spec_contains_key(post_store.staking_contracts, old_operator);
        let staking_contract = simple_map::spec_get(store.staking_contracts, old_operator);
        let stake_pool = global<stake::StakePool>(staking_contract.pool_address);
        let active = coin::value(stake_pool.active);
        let pending_active = coin::value(stake_pool.pending_active);
        let total_active_stake = active + pending_active;
        let accumulated_rewards = total_active_stake - staking_contract.principal;
        let commission_amount = accumulated_rewards * staking_contract.commission_percentage / 100;
        aborts_if staking_contract_exists && !exists<stake::StakePool>(staking_contract.pool_address);
        ensures staking_contract_exists ==>
            simple_map::spec_get(post_store.staking_contracts, new_operator).principal == total_active_stake - commission_amount;
        let pool_address = staking_contract.owner_cap.pool_address;
        let current_commission_percentage = staking_contract.commission_percentage;
        aborts_if staking_contract_exists && commission_amount != 0 && !exists<stake::StakePool>(pool_address);
        ensures staking_contract_exists && commission_amount != 0 ==>
            global<stake::StakePool>(pool_address).operator_address == new_operator
            && simple_map::spec_get(post_store.staking_contracts, new_operator).commission_percentage == current_commission_percentage;
        ensures staking_contract_exists ==> simple_map::spec_contains_key(post_store.staking_contracts, new_operator);
    }
}
```


<a id="@Specification_0_set_stake_pool_operator"></a>

### Function `set_stake_pool_operator`


```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_operator(owner: &signer, new_operator: address)
}
```

Aborts if stake_pool is exists and when OwnerCapability or stake_pool_exists
One of them are not exists


```move
module 0x1::staking_proxy {
    include SetStakePoolOperator;
}
```



<a id="0x1_staking_proxy_SetStakePoolOperator"></a>


```move
module 0x1::staking_proxy {
    schema SetStakePoolOperator {
        owner: &signer;
        new_operator: address;
        let owner_address = signer::address_of(owner);
        let ownership_cap = borrow_global<stake::OwnerCapability>(owner_address);
        let pool_address = ownership_cap.pool_address;
        aborts_if stake::stake_pool_exists(owner_address) && !(exists<stake::OwnerCapability>(owner_address) && stake::stake_pool_exists(pool_address));
        ensures stake::stake_pool_exists(owner_address) ==> global<stake::StakePool>(pool_address).operator_address == new_operator;
    }
}
```


<a id="@Specification_0_set_vesting_contract_voter"></a>

### Function `set_vesting_contract_voter`


```move
module 0x1::staking_proxy {
    public entry fun set_vesting_contract_voter(owner: &signer, operator: address, new_voter: address)
}
```



```move
module 0x1::staking_proxy {
    pragma verify = false;
}
```


<a id="@Specification_0_set_staking_contract_voter"></a>

### Function `set_staking_contract_voter`


```move
module 0x1::staking_proxy {
    public entry fun set_staking_contract_voter(owner: &signer, operator: address, new_voter: address)
}
```



```move
module 0x1::staking_proxy {
    include SetStakingContractVoter;
}
```

Make sure staking_contract_exists first
Then abort if the resource is not exist


<a id="0x1_staking_proxy_SetStakingContractVoter"></a>


```move
module 0x1::staking_proxy {
    schema SetStakingContractVoter {
        owner: &signer;
        operator: address;
        new_voter: address;
        let owner_address = signer::address_of(owner);
        let staker = owner_address;
        let store = global<Store>(staker);
        let staking_contract_exists = exists<Store>(staker) && simple_map::spec_contains_key(store.staking_contracts, operator);
        let staker_address = owner_address;
        let staking_contract = simple_map::spec_get(store.staking_contracts, operator);
        let pool_address = staking_contract.pool_address;
        let pool_address1 = staking_contract.owner_cap.pool_address;
        aborts_if staking_contract_exists && !exists<stake::StakePool>(pool_address);
        aborts_if staking_contract_exists && !exists<stake::StakePool>(staking_contract.owner_cap.pool_address);
        ensures staking_contract_exists ==> global<stake::StakePool>(pool_address1).delegated_voter == new_voter;
    }
}
```


<a id="@Specification_0_set_stake_pool_voter"></a>

### Function `set_stake_pool_voter`


```move
module 0x1::staking_proxy {
    public entry fun set_stake_pool_voter(owner: &signer, new_voter: address)
}
```



```move
module 0x1::staking_proxy {
    include SetStakePoolVoterAbortsIf;
}
```



<a id="0x1_staking_proxy_SetStakePoolVoterAbortsIf"></a>


```move
module 0x1::staking_proxy {
    schema SetStakePoolVoterAbortsIf {
        owner: &signer;
        new_voter: address;
        let owner_address = signer::address_of(owner);
        let ownership_cap = global<stake::OwnerCapability>(owner_address);
        let pool_address = ownership_cap.pool_address;
        aborts_if stake::stake_pool_exists(owner_address) && !(exists<stake::OwnerCapability>(owner_address) && stake::stake_pool_exists(pool_address));
        ensures stake::stake_pool_exists(owner_address) ==> global<stake::StakePool>(pool_address).delegated_voter == new_voter;
    }
}
```
