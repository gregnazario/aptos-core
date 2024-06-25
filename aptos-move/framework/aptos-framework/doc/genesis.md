
<a id="0x1_genesis"></a>

# Module `0x1::genesis`



-  [Struct `AccountMap`](#0x1_genesis_AccountMap)
-  [Struct `EmployeeAccountMap`](#0x1_genesis_EmployeeAccountMap)
-  [Struct `ValidatorConfiguration`](#0x1_genesis_ValidatorConfiguration)
-  [Struct `ValidatorConfigurationWithCommission`](#0x1_genesis_ValidatorConfigurationWithCommission)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_genesis_initialize)
-  [Function `initialize_aptos_coin`](#0x1_genesis_initialize_aptos_coin)
-  [Function `initialize_core_resources_and_aptos_coin`](#0x1_genesis_initialize_core_resources_and_aptos_coin)
-  [Function `create_accounts`](#0x1_genesis_create_accounts)
-  [Function `create_account`](#0x1_genesis_create_account)
-  [Function `create_employee_validators`](#0x1_genesis_create_employee_validators)
-  [Function `create_initialize_validators_with_commission`](#0x1_genesis_create_initialize_validators_with_commission)
-  [Function `create_initialize_validators`](#0x1_genesis_create_initialize_validators)
-  [Function `create_initialize_validator`](#0x1_genesis_create_initialize_validator)
-  [Function `initialize_validator`](#0x1_genesis_initialize_validator)
-  [Function `set_genesis_end`](#0x1_genesis_set_genesis_end)
-  [Function `initialize_for_verification`](#0x1_genesis_initialize_for_verification)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `initialize_aptos_coin`](#@Specification_1_initialize_aptos_coin)
    -  [Function `create_initialize_validators_with_commission`](#@Specification_1_create_initialize_validators_with_commission)
    -  [Function `create_initialize_validators`](#@Specification_1_create_initialize_validators)
    -  [Function `create_initialize_validator`](#@Specification_1_create_initialize_validator)
    -  [Function `set_genesis_end`](#@Specification_1_set_genesis_end)
    -  [Function `initialize_for_verification`](#@Specification_1_initialize_for_verification)


```move
module 0x1::genesis {
    use 0x1::account;
    use 0x1::aggregator_factory;
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::aptos_governance;
    use 0x1::block;
    use 0x1::chain_id;
    use 0x1::chain_status;
    use 0x1::coin;
    use 0x1::consensus_config;
    use 0x1::create_signer;
    use 0x1::error;
    use 0x1::execution_config;
    use 0x1::features;
    use 0x1::fixed_point32;
    use 0x1::gas_schedule;
    use 0x1::reconfiguration;
    use 0x1::simple_map;
    use 0x1::stake;
    use 0x1::staking_config;
    use 0x1::staking_contract;
    use 0x1::state_storage;
    use 0x1::storage_gas;
    use 0x1::timestamp;
    use 0x1::transaction_fee;
    use 0x1::transaction_validation;
    use 0x1::vector;
    use 0x1::version;
    use 0x1::vesting;
}
```


<a id="0x1_genesis_AccountMap"></a>

## Struct `AccountMap`



```move
module 0x1::genesis {
    struct AccountMap has drop
}
```


##### Fields


<dl>
<dt>
`account_address: address`
</dt>
<dd>

</dd>
<dt>
`balance: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_genesis_EmployeeAccountMap"></a>

## Struct `EmployeeAccountMap`



```move
module 0x1::genesis {
    struct EmployeeAccountMap has copy, drop
}
```


##### Fields


<dl>
<dt>
`accounts: vector<address>`
</dt>
<dd>

</dd>
<dt>
`validator: genesis::ValidatorConfigurationWithCommission`
</dt>
<dd>

</dd>
<dt>
`vesting_schedule_numerator: vector<u64>`
</dt>
<dd>

</dd>
<dt>
`vesting_schedule_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`beneficiary_resetter: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_genesis_ValidatorConfiguration"></a>

## Struct `ValidatorConfiguration`



```move
module 0x1::genesis {
    struct ValidatorConfiguration has copy, drop
}
```


##### Fields


<dl>
<dt>
`owner_address: address`
</dt>
<dd>

</dd>
<dt>
`operator_address: address`
</dt>
<dd>

</dd>
<dt>
`voter_address: address`
</dt>
<dd>

</dd>
<dt>
`stake_amount: u64`
</dt>
<dd>

</dd>
<dt>
`consensus_pubkey: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`proof_of_possession: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`network_addresses: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`full_node_network_addresses: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_genesis_ValidatorConfigurationWithCommission"></a>

## Struct `ValidatorConfigurationWithCommission`



```move
module 0x1::genesis {
    struct ValidatorConfigurationWithCommission has copy, drop
}
```


##### Fields


<dl>
<dt>
`validator_config: genesis::ValidatorConfiguration`
</dt>
<dd>

</dd>
<dt>
`commission_percentage: u64`
</dt>
<dd>

</dd>
<dt>
`join_during_genesis: bool`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_genesis_EACCOUNT_DOES_NOT_EXIST"></a>



```move
module 0x1::genesis {
    const EACCOUNT_DOES_NOT_EXIST: u64 = 2;
}
```


<a id="0x1_genesis_EDUPLICATE_ACCOUNT"></a>



```move
module 0x1::genesis {
    const EDUPLICATE_ACCOUNT: u64 = 1;
}
```


<a id="0x1_genesis_initialize"></a>

## Function `initialize`

Genesis step 1: Initialize aptos framework account and core modules on chain.


```move
module 0x1::genesis {
    fun initialize(gas_schedule: vector<u8>, chain_id: u8, initial_version: u64, consensus_config: vector<u8>, execution_config: vector<u8>, epoch_interval_microsecs: u64, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun initialize(
        gas_schedule: vector<u8>,
        chain_id: u8,
        initial_version: u64,
        consensus_config: vector<u8>,
        execution_config: vector<u8>,
        epoch_interval_microsecs: u64,
        minimum_stake: u64,
        maximum_stake: u64,
        recurring_lockup_duration_secs: u64,
        allow_validator_set_change: bool,
        rewards_rate: u64,
        rewards_rate_denominator: u64,
        voting_power_increase_limit: u64,
    ) {
        // Initialize the aptos framework account. This is the account where system resources and modules will be
        // deployed to. This will be entirely managed by on-chain governance and no entities have the key or privileges
        // to use this account.
        let (aptos_framework_account, aptos_framework_signer_cap) = account::create_framework_reserved_account(@aptos_framework);
        // Initialize account configs on aptos framework account.
        account::initialize(&aptos_framework_account);

        transaction_validation::initialize(
            &aptos_framework_account,
            b"script_prologue",
            b"module_prologue",
            b"multi_agent_script_prologue",
            b"epilogue",
        );

        // Give the decentralized on-chain governance control over the core framework account.
        aptos_governance::store_signer_cap(&aptos_framework_account, @aptos_framework, aptos_framework_signer_cap);

        // put reserved framework reserved accounts under aptos governance
        let framework_reserved_addresses = vector<address>[@0x2, @0x3, @0x4, @0x5, @0x6, @0x7, @0x8, @0x9, @0xa];
        while (!vector::is_empty(&framework_reserved_addresses)) {
            let address = vector::pop_back<address>(&mut framework_reserved_addresses);
            let (_, framework_signer_cap) = account::create_framework_reserved_account(address);
            aptos_governance::store_signer_cap(&aptos_framework_account, address, framework_signer_cap);
        };

        consensus_config::initialize(&aptos_framework_account, consensus_config);
        execution_config::set(&aptos_framework_account, execution_config);
        version::initialize(&aptos_framework_account, initial_version);
        stake::initialize(&aptos_framework_account);
        staking_config::initialize(
            &aptos_framework_account,
            minimum_stake,
            maximum_stake,
            recurring_lockup_duration_secs,
            allow_validator_set_change,
            rewards_rate,
            rewards_rate_denominator,
            voting_power_increase_limit,
        );
        storage_gas::initialize(&aptos_framework_account);
        gas_schedule::initialize(&aptos_framework_account, gas_schedule);

        // Ensure we can create aggregators for supply, but not enable it for common use just yet.
        aggregator_factory::initialize_aggregator_factory(&aptos_framework_account);
        coin::initialize_supply_config(&aptos_framework_account);

        chain_id::initialize(&aptos_framework_account, chain_id);
        reconfiguration::initialize(&aptos_framework_account);
        block::initialize(&aptos_framework_account, epoch_interval_microsecs);
        state_storage::initialize(&aptos_framework_account);
        timestamp::set_time_has_started(&aptos_framework_account);
    }
}
```


<a id="0x1_genesis_initialize_aptos_coin"></a>

## Function `initialize_aptos_coin`

Genesis step 2: Initialize Aptos coin.


```move
module 0x1::genesis {
    fun initialize_aptos_coin(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun initialize_aptos_coin(aptos_framework: &signer) {
        let (burn_cap, mint_cap) = aptos_coin::initialize(aptos_framework);

        coin::create_coin_conversion_map(aptos_framework);
        coin::create_pairing<AptosCoin>(aptos_framework);

        // Give stake module MintCapability<AptosCoin> so it can mint rewards.
        stake::store_aptos_coin_mint_cap(aptos_framework, mint_cap);
        // Give transaction_fee module BurnCapability<AptosCoin> so it can burn gas.
        transaction_fee::store_aptos_coin_burn_cap(aptos_framework, burn_cap);
        // Give transaction_fee module MintCapability<AptosCoin> so it can mint refunds.
        transaction_fee::store_aptos_coin_mint_cap(aptos_framework, mint_cap);
    }
}
```


<a id="0x1_genesis_initialize_core_resources_and_aptos_coin"></a>

## Function `initialize_core_resources_and_aptos_coin`

Only called for testnets and e2e tests.


```move
module 0x1::genesis {
    fun initialize_core_resources_and_aptos_coin(aptos_framework: &signer, core_resources_auth_key: vector<u8>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun initialize_core_resources_and_aptos_coin(
        aptos_framework: &signer,
        core_resources_auth_key: vector<u8>,
    ) {
        let (burn_cap, mint_cap) = aptos_coin::initialize(aptos_framework);

        coin::create_coin_conversion_map(aptos_framework);
        coin::create_pairing<AptosCoin>(aptos_framework);

        // Give stake module MintCapability<AptosCoin> so it can mint rewards.
        stake::store_aptos_coin_mint_cap(aptos_framework, mint_cap);
        // Give transaction_fee module BurnCapability<AptosCoin> so it can burn gas.
        transaction_fee::store_aptos_coin_burn_cap(aptos_framework, burn_cap);
        // Give transaction_fee module MintCapability<AptosCoin> so it can mint refunds.
        transaction_fee::store_aptos_coin_mint_cap(aptos_framework, mint_cap);

        let core_resources = account::create_account(@core_resources);
        account::rotate_authentication_key_internal(&core_resources, core_resources_auth_key);
        aptos_account::register_apt(&core_resources); // registers APT store
        aptos_coin::configure_accounts_for_test(aptos_framework, &core_resources, mint_cap);
    }
}
```


<a id="0x1_genesis_create_accounts"></a>

## Function `create_accounts`



```move
module 0x1::genesis {
    fun create_accounts(aptos_framework: &signer, accounts: vector<genesis::AccountMap>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_accounts(aptos_framework: &signer, accounts: vector<AccountMap>) {
        let unique_accounts = vector::empty();
        vector::for_each_ref(&accounts, |account_map| {
            let account_map: &AccountMap = account_map;
            assert!(
                !vector::contains(&unique_accounts, &account_map.account_address),
                error::already_exists(EDUPLICATE_ACCOUNT),
            );
            vector::push_back(&mut unique_accounts, account_map.account_address);

            create_account(
                aptos_framework,
                account_map.account_address,
                account_map.balance,
            );
        });
    }
}
```


<a id="0x1_genesis_create_account"></a>

## Function `create_account`

This creates an funds an account if it doesn&apos;t exist.
If it exists, it just returns the signer.


```move
module 0x1::genesis {
    fun create_account(aptos_framework: &signer, account_address: address, balance: u64): signer
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_account(aptos_framework: &signer, account_address: address, balance: u64): signer {
        if (account::exists_at(account_address)) {
            create_signer(account_address)
        } else {
            let account = account::create_account(account_address);
            coin::register<AptosCoin>(&account);
            aptos_coin::mint(aptos_framework, account_address, balance);
            account
        }
    }
}
```


<a id="0x1_genesis_create_employee_validators"></a>

## Function `create_employee_validators`



```move
module 0x1::genesis {
    fun create_employee_validators(employee_vesting_start: u64, employee_vesting_period_duration: u64, employees: vector<genesis::EmployeeAccountMap>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_employee_validators(
        employee_vesting_start: u64,
        employee_vesting_period_duration: u64,
        employees: vector<EmployeeAccountMap>,
    ) {
        let unique_accounts = vector::empty();

        vector::for_each_ref(&employees, |employee_group| {
            let j = 0;
            let employee_group: &EmployeeAccountMap = employee_group;
            let num_employees_in_group = vector::length(&employee_group.accounts);

            let buy_ins = simple_map::create();

            while (j < num_employees_in_group) {
                let account = vector::borrow(&employee_group.accounts, j);
                assert!(
                    !vector::contains(&unique_accounts, account),
                    error::already_exists(EDUPLICATE_ACCOUNT),
                );
                vector::push_back(&mut unique_accounts, *account);

                let employee = create_signer(*account);
                let total = coin::balance<AptosCoin>(*account);
                let coins = coin::withdraw<AptosCoin>(&employee, total);
                simple_map::add(&mut buy_ins, *account, coins);

                j = j + 1;
            };

            let j = 0;
            let num_vesting_events = vector::length(&employee_group.vesting_schedule_numerator);
            let schedule = vector::empty();

            while (j < num_vesting_events) {
                let numerator = vector::borrow(&employee_group.vesting_schedule_numerator, j);
                let event = fixed_point32::create_from_rational(*numerator, employee_group.vesting_schedule_denominator);
                vector::push_back(&mut schedule, event);

                j = j + 1;
            };

            let vesting_schedule = vesting::create_vesting_schedule(
                schedule,
                employee_vesting_start,
                employee_vesting_period_duration,
            );

            let admin = employee_group.validator.validator_config.owner_address;
            let admin_signer = &create_signer(admin);
            let contract_address = vesting::create_vesting_contract(
                admin_signer,
                &employee_group.accounts,
                buy_ins,
                vesting_schedule,
                admin,
                employee_group.validator.validator_config.operator_address,
                employee_group.validator.validator_config.voter_address,
                employee_group.validator.commission_percentage,
                x"",
            );
            let pool_address = vesting::stake_pool_address(contract_address);

            if (employee_group.beneficiary_resetter != @0x0) {
                vesting::set_beneficiary_resetter(admin_signer, contract_address, employee_group.beneficiary_resetter);
            };

            let validator = &employee_group.validator.validator_config;
            assert!(
                account::exists_at(validator.owner_address),
                error::not_found(EACCOUNT_DOES_NOT_EXIST),
            );
            assert!(
                account::exists_at(validator.operator_address),
                error::not_found(EACCOUNT_DOES_NOT_EXIST),
            );
            assert!(
                account::exists_at(validator.voter_address),
                error::not_found(EACCOUNT_DOES_NOT_EXIST),
            );
            if (employee_group.validator.join_during_genesis) {
                initialize_validator(pool_address, validator);
            };
        });
    }
}
```


<a id="0x1_genesis_create_initialize_validators_with_commission"></a>

## Function `create_initialize_validators_with_commission`



```move
module 0x1::genesis {
    fun create_initialize_validators_with_commission(aptos_framework: &signer, use_staking_contract: bool, validators: vector<genesis::ValidatorConfigurationWithCommission>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_initialize_validators_with_commission(
        aptos_framework: &signer,
        use_staking_contract: bool,
        validators: vector<ValidatorConfigurationWithCommission>,
    ) {
        vector::for_each_ref(&validators, |validator| {
            let validator: &ValidatorConfigurationWithCommission = validator;
            create_initialize_validator(aptos_framework, validator, use_staking_contract);
        });

        // Destroy the aptos framework account's ability to mint coins now that we're done with setting up the initial
        // validators.
        aptos_coin::destroy_mint_cap(aptos_framework);

        stake::on_new_epoch();
    }
}
```


<a id="0x1_genesis_create_initialize_validators"></a>

## Function `create_initialize_validators`

Sets up the initial validator set for the network.
The validator &quot;owner&quot; accounts, and their authentication
Addresses (and keys) are encoded in the `owners`
Each validator signs consensus messages with the private key corresponding to the Ed25519
public key in `consensus_pubkeys`.
Finally, each validator must specify the network address
(see types/src/network_address/mod.rs) for itself and its full nodes.

Network address fields are a vector per account, where each entry is a vector of addresses
encoded in a single BCS byte array.


```move
module 0x1::genesis {
    fun create_initialize_validators(aptos_framework: &signer, validators: vector<genesis::ValidatorConfiguration>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_initialize_validators(aptos_framework: &signer, validators: vector<ValidatorConfiguration>) {
        let validators_with_commission = vector::empty();
        vector::for_each_reverse(validators, |validator| {
            let validator_with_commission = ValidatorConfigurationWithCommission {
                validator_config: validator,
                commission_percentage: 0,
                join_during_genesis: true,
            };
            vector::push_back(&mut validators_with_commission, validator_with_commission);
        });

        create_initialize_validators_with_commission(aptos_framework, false, validators_with_commission);
    }
}
```


<a id="0x1_genesis_create_initialize_validator"></a>

## Function `create_initialize_validator`



```move
module 0x1::genesis {
    fun create_initialize_validator(aptos_framework: &signer, commission_config: &genesis::ValidatorConfigurationWithCommission, use_staking_contract: bool)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun create_initialize_validator(
        aptos_framework: &signer,
        commission_config: &ValidatorConfigurationWithCommission,
        use_staking_contract: bool,
    ) {
        let validator = &commission_config.validator_config;

        let owner = &create_account(aptos_framework, validator.owner_address, validator.stake_amount);
        create_account(aptos_framework, validator.operator_address, 0);
        create_account(aptos_framework, validator.voter_address, 0);

        // Initialize the stake pool and join the validator set.
        let pool_address = if (use_staking_contract) {
            staking_contract::create_staking_contract(
                owner,
                validator.operator_address,
                validator.voter_address,
                validator.stake_amount,
                commission_config.commission_percentage,
                x"",
            );
            staking_contract::stake_pool_address(validator.owner_address, validator.operator_address)
        } else {
            stake::initialize_stake_owner(
                owner,
                validator.stake_amount,
                validator.operator_address,
                validator.voter_address,
            );
            validator.owner_address
        };

        if (commission_config.join_during_genesis) {
            initialize_validator(pool_address, validator);
        };
    }
}
```


<a id="0x1_genesis_initialize_validator"></a>

## Function `initialize_validator`



```move
module 0x1::genesis {
    fun initialize_validator(pool_address: address, validator: &genesis::ValidatorConfiguration)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun initialize_validator(pool_address: address, validator: &ValidatorConfiguration) {
        let operator = &create_signer(validator.operator_address);

        stake::rotate_consensus_key(
            operator,
            pool_address,
            validator.consensus_pubkey,
            validator.proof_of_possession,
        );
        stake::update_network_and_fullnode_addresses(
            operator,
            pool_address,
            validator.network_addresses,
            validator.full_node_network_addresses,
        );
        stake::join_validator_set_internal(operator, pool_address);
    }
}
```


<a id="0x1_genesis_set_genesis_end"></a>

## Function `set_genesis_end`

The last step of genesis.


```move
module 0x1::genesis {
    fun set_genesis_end(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun set_genesis_end(aptos_framework: &signer) {
        chain_status::set_genesis_end(aptos_framework);
    }
}
```


<a id="0x1_genesis_initialize_for_verification"></a>

## Function `initialize_for_verification`



```move
module 0x1::genesis {
    #[verify_only]
    fun initialize_for_verification(gas_schedule: vector<u8>, chain_id: u8, initial_version: u64, consensus_config: vector<u8>, execution_config: vector<u8>, epoch_interval_microsecs: u64, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64, aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64, accounts: vector<genesis::AccountMap>, employee_vesting_start: u64, employee_vesting_period_duration: u64, employees: vector<genesis::EmployeeAccountMap>, validators: vector<genesis::ValidatorConfigurationWithCommission>)
}
```


##### Implementation


```move
module 0x1::genesis {
    fun initialize_for_verification(
        gas_schedule: vector<u8>,
        chain_id: u8,
        initial_version: u64,
        consensus_config: vector<u8>,
        execution_config: vector<u8>,
        epoch_interval_microsecs: u64,
        minimum_stake: u64,
        maximum_stake: u64,
        recurring_lockup_duration_secs: u64,
        allow_validator_set_change: bool,
        rewards_rate: u64,
        rewards_rate_denominator: u64,
        voting_power_increase_limit: u64,
        aptos_framework: &signer,
        min_voting_threshold: u128,
        required_proposer_stake: u64,
        voting_duration_secs: u64,
        accounts: vector<AccountMap>,
        employee_vesting_start: u64,
        employee_vesting_period_duration: u64,
        employees: vector<EmployeeAccountMap>,
        validators: vector<ValidatorConfigurationWithCommission>
    ) {
        initialize(
            gas_schedule,
            chain_id,
            initial_version,
            consensus_config,
            execution_config,
            epoch_interval_microsecs,
            minimum_stake,
            maximum_stake,
            recurring_lockup_duration_secs,
            allow_validator_set_change,
            rewards_rate,
            rewards_rate_denominator,
            voting_power_increase_limit
        );
        features::change_feature_flags_for_verification(aptos_framework, vector[1, 2], vector[]);
        initialize_aptos_coin(aptos_framework);
        aptos_governance::initialize_for_verification(
            aptos_framework,
            min_voting_threshold,
            required_proposer_stake,
            voting_duration_secs
        );
        create_accounts(aptos_framework, accounts);
        create_employee_validators(employee_vesting_start, employee_vesting_period_duration, employees);
        create_initialize_validators_with_commission(aptos_framework, true, validators);
        set_genesis_end(aptos_framework);
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
<td>All the core resources and modules should be created during genesis and owned by the Aptos framework account.</td>
<td>Critical</td>
<td>Resources created during genesis initialization: GovernanceResponsbility, ConsensusConfig, ExecutionConfig, Version, SetVersionCapability, ValidatorSet, ValidatorPerformance, StakingConfig, StorageGasConfig, StorageGas, GasScheduleV2, AggregatorFactory, SupplyConfig, ChainId, Configuration, BlockResource, StateStorageUsage, CurrentTimeMicroseconds. If some of the resources were to be owned by a malicious account, it could lead to the compromise of the chain, as these are core resources. It should be formally verified by a post condition to ensure that all the critical resources are owned by the Aptos framework.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>Addresses ranging from 0x0 &#45; 0xa should be reserved for the framework and part of aptos governance.</td>
<td>Critical</td>
<td>The function genesis::initialize calls account::create_framework_reserved_account for addresses 0x0, 0x2, 0x3, 0x4, ..., 0xa which creates an account and authentication_key for them. This should be formally verified by ensuring that at the beginning of the genesis::initialize function no Account resource exists for the reserved addresses, and at the end of the function, an Account resource exists.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](initialize).</td>
</tr>

<tr>
<td>3</td>
<td>The Aptos coin should be initialized during genesis and only the Aptos framework account should own the mint and burn capabilities for the APT token.</td>
<td>Critical</td>
<td>Both mint and burn capabilities are wrapped inside the stake::AptosCoinCapabilities and transaction_fee::AptosCoinCapabilities resources which are stored under the aptos framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](initialize_aptos_coin).</td>
</tr>

<tr>
<td>4</td>
<td>An initial set of validators should exist before the end of genesis.</td>
<td>Low</td>
<td>To ensure that there will be a set of validators available to validate the genesis block, the length of the ValidatorSet.active_validators vector should be &gt; 0.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](set_genesis_end).</td>
</tr>

<tr>
<td>5</td>
<td>The end of genesis should be marked on chain.</td>
<td>Low</td>
<td>The end of genesis is marked, on chain, via the chain_status::GenesisEndMarker resource. The ownership of this resource marks the operating state of the chain.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](set_genesis_end).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::genesis {
    pragma verify = true;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::genesis {
    fun initialize(gas_schedule: vector<u8>, chain_id: u8, initial_version: u64, consensus_config: vector<u8>, execution_config: vector<u8>, epoch_interval_microsecs: u64, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64)
}
```



```move
module 0x1::genesis {
    pragma aborts_if_is_partial;
    include InitalizeRequires;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if exists<account::Account>(@0x0);
    aborts_if exists<account::Account>(@0x2);
    aborts_if exists<account::Account>(@0x3);
    aborts_if exists<account::Account>(@0x4);
    aborts_if exists<account::Account>(@0x5);
    aborts_if exists<account::Account>(@0x6);
    aborts_if exists<account::Account>(@0x7);
    aborts_if exists<account::Account>(@0x8);
    aborts_if exists<account::Account>(@0x9);
    aborts_if exists<account::Account>(@0xa);
    ensures exists<account::Account>(@0x0);
    ensures exists<account::Account>(@0x2);
    ensures exists<account::Account>(@0x3);
    ensures exists<account::Account>(@0x4);
    ensures exists<account::Account>(@0x5);
    ensures exists<account::Account>(@0x6);
    ensures exists<account::Account>(@0x7);
    ensures exists<account::Account>(@0x8);
    ensures exists<account::Account>(@0x9);
    ensures exists<account::Account>(@0xa);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<aptos_governance::GovernanceResponsbility>(@aptos_framework);
    ensures exists<consensus_config::ConsensusConfig>(@aptos_framework);
    ensures exists<execution_config::ExecutionConfig>(@aptos_framework);
    ensures exists<version::Version>(@aptos_framework);
    ensures exists<stake::ValidatorSet>(@aptos_framework);
    ensures exists<stake::ValidatorPerformance>(@aptos_framework);
    ensures exists<storage_gas::StorageGasConfig>(@aptos_framework);
    ensures exists<storage_gas::StorageGas>(@aptos_framework);
    ensures exists<gas_schedule::GasScheduleV2>(@aptos_framework);
    ensures exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    ensures exists<coin::SupplyConfig>(@aptos_framework);
    ensures exists<chain_id::ChainId>(@aptos_framework);
    ensures exists<reconfiguration::Configuration>(@aptos_framework);
    ensures exists<block::BlockResource>(@aptos_framework);
    ensures exists<state_storage::StateStorageUsage>(@aptos_framework);
    ensures exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    ensures exists<account::Account>(@aptos_framework);
    ensures exists<version::SetVersionCapability>(@aptos_framework);
    ensures exists<staking_config::StakingConfig>(@aptos_framework);
}
```


<a id="@Specification_1_initialize_aptos_coin"></a>

### Function `initialize_aptos_coin`


```move
module 0x1::genesis {
    fun initialize_aptos_coin(aptos_framework: &signer)
}
```



```move
module 0x1::genesis {
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    requires !exists<stake::AptosCoinCapabilities>(@aptos_framework);
    ensures exists<stake::AptosCoinCapabilities>(@aptos_framework);
    requires exists<transaction_fee::AptosCoinCapabilities>(@aptos_framework);
    ensures exists<transaction_fee::AptosCoinCapabilities>(@aptos_framework);
}
```


<a id="@Specification_1_create_initialize_validators_with_commission"></a>

### Function `create_initialize_validators_with_commission`


```move
module 0x1::genesis {
    fun create_initialize_validators_with_commission(aptos_framework: &signer, use_staking_contract: bool, validators: vector<genesis::ValidatorConfigurationWithCommission>)
}
```



```move
module 0x1::genesis {
    pragma verify_duration_estimate = 120;
    include stake::ResourceRequirement;
    include stake::GetReconfigStartTimeRequirement;
    include CompareTimeRequires;
    include aptos_coin::ExistsAptosCoin;
}
```


<a id="@Specification_1_create_initialize_validators"></a>

### Function `create_initialize_validators`


```move
module 0x1::genesis {
    fun create_initialize_validators(aptos_framework: &signer, validators: vector<genesis::ValidatorConfiguration>)
}
```



```move
module 0x1::genesis {
    pragma verify_duration_estimate = 120;
    include stake::ResourceRequirement;
    include stake::GetReconfigStartTimeRequirement;
    include CompareTimeRequires;
    include aptos_coin::ExistsAptosCoin;
}
```


<a id="@Specification_1_create_initialize_validator"></a>

### Function `create_initialize_validator`


```move
module 0x1::genesis {
    fun create_initialize_validator(aptos_framework: &signer, commission_config: &genesis::ValidatorConfigurationWithCommission, use_staking_contract: bool)
}
```



```move
module 0x1::genesis {
    include stake::ResourceRequirement;
}
```


<a id="@Specification_1_set_genesis_end"></a>

### Function `set_genesis_end`


```move
module 0x1::genesis {
    fun set_genesis_end(aptos_framework: &signer)
}
```



```move
module 0x1::genesis {
    pragma delegate_invariants_to_caller;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    requires len(global<stake::ValidatorSet>(@aptos_framework).active_validators) >= 1;
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    let addr = std::signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if exists<chain_status::GenesisEndMarker>(@aptos_framework);
    ensures global<chain_status::GenesisEndMarker>(@aptos_framework) == chain_status::GenesisEndMarker {};
}
```



<a id="0x1_genesis_InitalizeRequires"></a>


```move
module 0x1::genesis {
    schema InitalizeRequires {
        execution_config: vector<u8>;
        requires !exists<account::Account>(@aptos_framework);
        requires chain_status::is_operating();
        requires len(execution_config) > 0;
        requires exists<staking_config::StakingRewardsConfig>(@aptos_framework);
        requires exists<stake::ValidatorFees>(@aptos_framework);
        requires exists<coin::CoinInfo<AptosCoin>>(@aptos_framework);
        include CompareTimeRequires;
        include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
    }
}
```



<a id="0x1_genesis_CompareTimeRequires"></a>


```move
module 0x1::genesis {
    schema CompareTimeRequires {
        let staking_rewards_config = global<staking_config::StakingRewardsConfig>(@aptos_framework);
        requires staking_rewards_config.last_rewards_rate_period_start_in_secs <= timestamp::spec_now_seconds();
    }
}
```


<a id="@Specification_1_initialize_for_verification"></a>

### Function `initialize_for_verification`


```move
module 0x1::genesis {
    #[verify_only]
    fun initialize_for_verification(gas_schedule: vector<u8>, chain_id: u8, initial_version: u64, consensus_config: vector<u8>, execution_config: vector<u8>, epoch_interval_microsecs: u64, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64, aptos_framework: &signer, min_voting_threshold: u128, required_proposer_stake: u64, voting_duration_secs: u64, accounts: vector<genesis::AccountMap>, employee_vesting_start: u64, employee_vesting_period_duration: u64, employees: vector<genesis::EmployeeAccountMap>, validators: vector<genesis::ValidatorConfigurationWithCommission>)
}
```



```move
module 0x1::genesis {
    pragma verify_duration_estimate = 120;
    include InitalizeRequires;
}
```
