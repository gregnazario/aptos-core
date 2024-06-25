
<a id="0x1_staking_config"></a>

# Module `0x1::staking_config`

Provides the configuration for staking and rewards


-  [Resource `StakingConfig`](#0x1_staking_config_StakingConfig)
-  [Resource `StakingRewardsConfig`](#0x1_staking_config_StakingRewardsConfig)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_staking_config_initialize)
-  [Function `reward_rate`](#0x1_staking_config_reward_rate)
-  [Function `initialize_rewards`](#0x1_staking_config_initialize_rewards)
-  [Function `get`](#0x1_staking_config_get)
-  [Function `get_allow_validator_set_change`](#0x1_staking_config_get_allow_validator_set_change)
-  [Function `get_required_stake`](#0x1_staking_config_get_required_stake)
-  [Function `get_recurring_lockup_duration`](#0x1_staking_config_get_recurring_lockup_duration)
-  [Function `get_reward_rate`](#0x1_staking_config_get_reward_rate)
-  [Function `get_voting_power_increase_limit`](#0x1_staking_config_get_voting_power_increase_limit)
-  [Function `calculate_and_save_latest_epoch_rewards_rate`](#0x1_staking_config_calculate_and_save_latest_epoch_rewards_rate)
-  [Function `calculate_and_save_latest_rewards_config`](#0x1_staking_config_calculate_and_save_latest_rewards_config)
-  [Function `update_required_stake`](#0x1_staking_config_update_required_stake)
-  [Function `update_recurring_lockup_duration_secs`](#0x1_staking_config_update_recurring_lockup_duration_secs)
-  [Function `update_rewards_rate`](#0x1_staking_config_update_rewards_rate)
-  [Function `update_rewards_config`](#0x1_staking_config_update_rewards_config)
-  [Function `update_voting_power_increase_limit`](#0x1_staking_config_update_voting_power_increase_limit)
-  [Function `validate_required_stake`](#0x1_staking_config_validate_required_stake)
-  [Function `validate_rewards_config`](#0x1_staking_config_validate_rewards_config)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Resource `StakingConfig`](#@Specification_1_StakingConfig)
    -  [Resource `StakingRewardsConfig`](#@Specification_1_StakingRewardsConfig)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `reward_rate`](#@Specification_1_reward_rate)
    -  [Function `initialize_rewards`](#@Specification_1_initialize_rewards)
    -  [Function `get`](#@Specification_1_get)
    -  [Function `get_reward_rate`](#@Specification_1_get_reward_rate)
    -  [Function `calculate_and_save_latest_epoch_rewards_rate`](#@Specification_1_calculate_and_save_latest_epoch_rewards_rate)
    -  [Function `calculate_and_save_latest_rewards_config`](#@Specification_1_calculate_and_save_latest_rewards_config)
    -  [Function `update_required_stake`](#@Specification_1_update_required_stake)
    -  [Function `update_recurring_lockup_duration_secs`](#@Specification_1_update_recurring_lockup_duration_secs)
    -  [Function `update_rewards_rate`](#@Specification_1_update_rewards_rate)
    -  [Function `update_rewards_config`](#@Specification_1_update_rewards_config)
    -  [Function `update_voting_power_increase_limit`](#@Specification_1_update_voting_power_increase_limit)
    -  [Function `validate_required_stake`](#@Specification_1_validate_required_stake)
    -  [Function `validate_rewards_config`](#@Specification_1_validate_rewards_config)


```move
module 0x1::staking_config {
    use 0x1::error;
    use 0x1::features;
    use 0x1::fixed_point64;
    use 0x1::math_fixed64;
    use 0x1::system_addresses;
    use 0x1::timestamp;
}
```


<a id="0x1_staking_config_StakingConfig"></a>

## Resource `StakingConfig`

Validator set configurations that will be stored with the @aptos_framework account.


```move
module 0x1::staking_config {
    struct StakingConfig has copy, drop, key
}
```


##### Fields


<dl>
<dt>
`minimum_stake: u64`
</dt>
<dd>

</dd>
<dt>
`maximum_stake: u64`
</dt>
<dd>

</dd>
<dt>
`recurring_lockup_duration_secs: u64`
</dt>
<dd>

</dd>
<dt>
`allow_validator_set_change: bool`
</dt>
<dd>

</dd>
<dt>
`rewards_rate: u64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`voting_power_increase_limit: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_config_StakingRewardsConfig"></a>

## Resource `StakingRewardsConfig`

Staking reward configurations that will be stored with the @aptos_framework account.


```move
module 0x1::staking_config {
    struct StakingRewardsConfig has copy, drop, key
}
```


##### Fields


<dl>
<dt>
`rewards_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
<dt>
`min_rewards_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_period_in_secs: u64`
</dt>
<dd>

</dd>
<dt>
`last_rewards_rate_period_start_in_secs: u64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_decrease_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_staking_config_MAX_U64"></a>



```move
module 0x1::staking_config {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_staking_config_BPS_DENOMINATOR"></a>

Denominator of number in basis points. 1 bps(basis points) &#61; 0.01%.


```move
module 0x1::staking_config {
    const BPS_DENOMINATOR: u64 = 10000;
}
```


<a id="0x1_staking_config_EDEPRECATED_FUNCTION"></a>

The function has been deprecated.


```move
module 0x1::staking_config {
    const EDEPRECATED_FUNCTION: u64 = 10;
}
```


<a id="0x1_staking_config_EDISABLED_FUNCTION"></a>

The function is disabled or hasn&apos;t been enabled.


```move
module 0x1::staking_config {
    const EDISABLED_FUNCTION: u64 = 11;
}
```


<a id="0x1_staking_config_EINVALID_LAST_REWARDS_RATE_PERIOD_START"></a>

Specified start time of last rewards rate period is invalid, which must be not late than the current timestamp.


```move
module 0x1::staking_config {
    const EINVALID_LAST_REWARDS_RATE_PERIOD_START: u64 = 7;
}
```


<a id="0x1_staking_config_EINVALID_MIN_REWARDS_RATE"></a>

Specified min rewards rate is invalid, which must be within [0, rewards_rate].


```move
module 0x1::staking_config {
    const EINVALID_MIN_REWARDS_RATE: u64 = 6;
}
```


<a id="0x1_staking_config_EINVALID_REWARDS_RATE"></a>

Specified rewards rate is invalid, which must be within [0, MAX_REWARDS_RATE].


```move
module 0x1::staking_config {
    const EINVALID_REWARDS_RATE: u64 = 5;
}
```


<a id="0x1_staking_config_EINVALID_REWARDS_RATE_DECREASE_RATE"></a>

Specified rewards rate decrease rate is invalid, which must be not greater than BPS_DENOMINATOR.


```move
module 0x1::staking_config {
    const EINVALID_REWARDS_RATE_DECREASE_RATE: u64 = 8;
}
```


<a id="0x1_staking_config_EINVALID_REWARDS_RATE_PERIOD"></a>

Specified rewards rate period is invalid. It must be larger than 0 and cannot be changed if configured.


```move
module 0x1::staking_config {
    const EINVALID_REWARDS_RATE_PERIOD: u64 = 9;
}
```


<a id="0x1_staking_config_EINVALID_STAKE_RANGE"></a>

Specified stake range is invalid. Max must be greater than min.


```move
module 0x1::staking_config {
    const EINVALID_STAKE_RANGE: u64 = 3;
}
```


<a id="0x1_staking_config_EINVALID_VOTING_POWER_INCREASE_LIMIT"></a>

The voting power increase limit percentage must be within (0, 50].


```move
module 0x1::staking_config {
    const EINVALID_VOTING_POWER_INCREASE_LIMIT: u64 = 4;
}
```


<a id="0x1_staking_config_EZERO_LOCKUP_DURATION"></a>

Stake lockup duration cannot be zero.


```move
module 0x1::staking_config {
    const EZERO_LOCKUP_DURATION: u64 = 1;
}
```


<a id="0x1_staking_config_EZERO_REWARDS_RATE_DENOMINATOR"></a>

Reward rate denominator cannot be zero.


```move
module 0x1::staking_config {
    const EZERO_REWARDS_RATE_DENOMINATOR: u64 = 2;
}
```


<a id="0x1_staking_config_MAX_REWARDS_RATE"></a>

Limit the maximum value of `rewards_rate` in order to avoid any arithmetic overflow.


```move
module 0x1::staking_config {
    const MAX_REWARDS_RATE: u64 = 1000000;
}
```


<a id="0x1_staking_config_ONE_YEAR_IN_SECS"></a>

1 year &#61;&gt; 365 &#42; 24 &#42; 60 &#42; 60


```move
module 0x1::staking_config {
    const ONE_YEAR_IN_SECS: u64 = 31536000;
}
```


<a id="0x1_staking_config_initialize"></a>

## Function `initialize`

Only called during genesis.


```move
module 0x1::staking_config {
    public(friend) fun initialize(aptos_framework: &signer, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public(friend) fun initialize(
        aptos_framework: &signer,
        minimum_stake: u64,
        maximum_stake: u64,
        recurring_lockup_duration_secs: u64,
        allow_validator_set_change: bool,
        rewards_rate: u64,
        rewards_rate_denominator: u64,
        voting_power_increase_limit: u64,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        // This can fail genesis but is necessary so that any misconfigurations can be corrected before genesis succeeds
        validate_required_stake(minimum_stake, maximum_stake);

        assert!(recurring_lockup_duration_secs > 0, error::invalid_argument(EZERO_LOCKUP_DURATION));
        assert!(
            rewards_rate_denominator > 0,
            error::invalid_argument(EZERO_REWARDS_RATE_DENOMINATOR),
        );
        assert!(
            voting_power_increase_limit > 0 && voting_power_increase_limit <= 50,
            error::invalid_argument(EINVALID_VOTING_POWER_INCREASE_LIMIT),
        );

        // `rewards_rate` which is the numerator is limited to be `<= MAX_REWARDS_RATE` in order to avoid the arithmetic
        // overflow in the rewards calculation. `rewards_rate_denominator` can be adjusted to get the desired rewards
        // rate (i.e., rewards_rate / rewards_rate_denominator).
        assert!(rewards_rate <= MAX_REWARDS_RATE, error::invalid_argument(EINVALID_REWARDS_RATE));

        // We assert that (rewards_rate / rewards_rate_denominator <= 1).
        assert!(rewards_rate <= rewards_rate_denominator, error::invalid_argument(EINVALID_REWARDS_RATE));

        move_to(aptos_framework, StakingConfig {
            minimum_stake,
            maximum_stake,
            recurring_lockup_duration_secs,
            allow_validator_set_change,
            rewards_rate,
            rewards_rate_denominator,
            voting_power_increase_limit,
        });
    }
}
```


<a id="0x1_staking_config_reward_rate"></a>

## Function `reward_rate`

Return the reward rate of this epoch as a tuple (numerator, denominator).


```move
module 0x1::staking_config {
    #[view]
    public fun reward_rate(): (u64, u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun reward_rate(): (u64, u64) acquires StakingRewardsConfig, StakingConfig {
        get_reward_rate(borrow_global<StakingConfig>(@aptos_framework))
    }
}
```


<a id="0x1_staking_config_initialize_rewards"></a>

## Function `initialize_rewards`

Initialize rewards configurations.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::staking_config {
    public fun initialize_rewards(aptos_framework: &signer, rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, last_rewards_rate_period_start_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun initialize_rewards(
        aptos_framework: &signer,
        rewards_rate: FixedPoint64,
        min_rewards_rate: FixedPoint64,
        rewards_rate_period_in_secs: u64,
        last_rewards_rate_period_start_in_secs: u64,
        rewards_rate_decrease_rate: FixedPoint64,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        validate_rewards_config(
            rewards_rate,
            min_rewards_rate,
            rewards_rate_period_in_secs,
            rewards_rate_decrease_rate,
        );
        assert!(
            timestamp::now_seconds() >= last_rewards_rate_period_start_in_secs,
            error::invalid_argument(EINVALID_LAST_REWARDS_RATE_PERIOD_START)
        );

        move_to(aptos_framework, StakingRewardsConfig {
            rewards_rate,
            min_rewards_rate,
            rewards_rate_period_in_secs,
            last_rewards_rate_period_start_in_secs,
            rewards_rate_decrease_rate,
        });
    }
}
```


<a id="0x1_staking_config_get"></a>

## Function `get`



```move
module 0x1::staking_config {
    public fun get(): staking_config::StakingConfig
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get(): StakingConfig acquires StakingConfig {
        *borrow_global<StakingConfig>(@aptos_framework)
    }
}
```


<a id="0x1_staking_config_get_allow_validator_set_change"></a>

## Function `get_allow_validator_set_change`

Return whether validator set changes are allowed


```move
module 0x1::staking_config {
    public fun get_allow_validator_set_change(config: &staking_config::StakingConfig): bool
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get_allow_validator_set_change(config: &StakingConfig): bool {
        config.allow_validator_set_change
    }
}
```


<a id="0x1_staking_config_get_required_stake"></a>

## Function `get_required_stake`

Return the required min/max stake.


```move
module 0x1::staking_config {
    public fun get_required_stake(config: &staking_config::StakingConfig): (u64, u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get_required_stake(config: &StakingConfig): (u64, u64) {
        (config.minimum_stake, config.maximum_stake)
    }
}
```


<a id="0x1_staking_config_get_recurring_lockup_duration"></a>

## Function `get_recurring_lockup_duration`

Return the recurring lockup duration that every validator is automatically renewed for (unless they unlock and
withdraw all funds).


```move
module 0x1::staking_config {
    public fun get_recurring_lockup_duration(config: &staking_config::StakingConfig): u64
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get_recurring_lockup_duration(config: &StakingConfig): u64 {
        config.recurring_lockup_duration_secs
    }
}
```


<a id="0x1_staking_config_get_reward_rate"></a>

## Function `get_reward_rate`

Return the reward rate of this epoch.


```move
module 0x1::staking_config {
    public fun get_reward_rate(config: &staking_config::StakingConfig): (u64, u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get_reward_rate(config: &StakingConfig): (u64, u64) acquires StakingRewardsConfig {
        if (features::periodical_reward_rate_decrease_enabled()) {
            let epoch_rewards_rate = borrow_global<StakingRewardsConfig>(@aptos_framework).rewards_rate;
            if (fixed_point64::is_zero(epoch_rewards_rate)) {
                (0u64, 1u64)
            } else {
                // Maximize denominator for higher precision.
                // Restriction: nominator <= MAX_REWARDS_RATE && denominator <= MAX_U64
                let denominator = fixed_point64::divide_u128((MAX_REWARDS_RATE as u128), epoch_rewards_rate);
                if (denominator > MAX_U64) {
                    denominator = MAX_U64
                };
                let nominator = (fixed_point64::multiply_u128(denominator, epoch_rewards_rate) as u64);
                (nominator, (denominator as u64))
            }
        } else {
            (config.rewards_rate, config.rewards_rate_denominator)
        }
    }
}
```


<a id="0x1_staking_config_get_voting_power_increase_limit"></a>

## Function `get_voting_power_increase_limit`

Return the joining limit %.


```move
module 0x1::staking_config {
    public fun get_voting_power_increase_limit(config: &staking_config::StakingConfig): u64
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun get_voting_power_increase_limit(config: &StakingConfig): u64 {
        config.voting_power_increase_limit
    }
}
```


<a id="0x1_staking_config_calculate_and_save_latest_epoch_rewards_rate"></a>

## Function `calculate_and_save_latest_epoch_rewards_rate`

Calculate and save the latest rewards rate.


```move
module 0x1::staking_config {
    public(friend) fun calculate_and_save_latest_epoch_rewards_rate(): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::staking_config {
    public(friend) fun calculate_and_save_latest_epoch_rewards_rate(): FixedPoint64 acquires StakingRewardsConfig {
        assert!(features::periodical_reward_rate_decrease_enabled(), error::invalid_state(EDISABLED_FUNCTION));
        let staking_rewards_config = calculate_and_save_latest_rewards_config();
        staking_rewards_config.rewards_rate
    }
}
```


<a id="0x1_staking_config_calculate_and_save_latest_rewards_config"></a>

## Function `calculate_and_save_latest_rewards_config`

Calculate and return the up&#45;to&#45;date StakingRewardsConfig.


```move
module 0x1::staking_config {
    fun calculate_and_save_latest_rewards_config(): staking_config::StakingRewardsConfig
}
```


##### Implementation


```move
module 0x1::staking_config {
    fun calculate_and_save_latest_rewards_config(): StakingRewardsConfig acquires StakingRewardsConfig {
        let staking_rewards_config = borrow_global_mut<StakingRewardsConfig>(@aptos_framework);
        let current_time_in_secs = timestamp::now_seconds();
        assert!(
            current_time_in_secs >= staking_rewards_config.last_rewards_rate_period_start_in_secs,
            error::invalid_argument(EINVALID_LAST_REWARDS_RATE_PERIOD_START)
        );
        if (current_time_in_secs - staking_rewards_config.last_rewards_rate_period_start_in_secs < staking_rewards_config.rewards_rate_period_in_secs) {
            return *staking_rewards_config
        };
        // Rewards rate decrease rate cannot be greater than 100%. Otherwise rewards rate will be negative.
        assert!(
            fixed_point64::ceil(staking_rewards_config.rewards_rate_decrease_rate) <= 1,
            error::invalid_argument(EINVALID_REWARDS_RATE_DECREASE_RATE)
        );
        let new_rate = math_fixed64::mul_div(
            staking_rewards_config.rewards_rate,
            fixed_point64::sub(
                fixed_point64::create_from_u128(1),
                staking_rewards_config.rewards_rate_decrease_rate,
            ),
            fixed_point64::create_from_u128(1),
        );
        new_rate = fixed_point64::max(new_rate, staking_rewards_config.min_rewards_rate);

        staking_rewards_config.rewards_rate = new_rate;
        staking_rewards_config.last_rewards_rate_period_start_in_secs =
            staking_rewards_config.last_rewards_rate_period_start_in_secs +
            staking_rewards_config.rewards_rate_period_in_secs;
        return *staking_rewards_config
    }
}
```


<a id="0x1_staking_config_update_required_stake"></a>

## Function `update_required_stake`

Update the min and max stake amounts.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::staking_config {
    public fun update_required_stake(aptos_framework: &signer, minimum_stake: u64, maximum_stake: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun update_required_stake(
        aptos_framework: &signer,
        minimum_stake: u64,
        maximum_stake: u64,
    ) acquires StakingConfig {
        system_addresses::assert_aptos_framework(aptos_framework);
        validate_required_stake(minimum_stake, maximum_stake);

        let staking_config = borrow_global_mut<StakingConfig>(@aptos_framework);
        staking_config.minimum_stake = minimum_stake;
        staking_config.maximum_stake = maximum_stake;
    }
}
```


<a id="0x1_staking_config_update_recurring_lockup_duration_secs"></a>

## Function `update_recurring_lockup_duration_secs`

Update the recurring lockup duration.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::staking_config {
    public fun update_recurring_lockup_duration_secs(aptos_framework: &signer, new_recurring_lockup_duration_secs: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun update_recurring_lockup_duration_secs(
        aptos_framework: &signer,
        new_recurring_lockup_duration_secs: u64,
    ) acquires StakingConfig {
        assert!(new_recurring_lockup_duration_secs > 0, error::invalid_argument(EZERO_LOCKUP_DURATION));
        system_addresses::assert_aptos_framework(aptos_framework);

        let staking_config = borrow_global_mut<StakingConfig>(@aptos_framework);
        staking_config.recurring_lockup_duration_secs = new_recurring_lockup_duration_secs;
    }
}
```


<a id="0x1_staking_config_update_rewards_rate"></a>

## Function `update_rewards_rate`

DEPRECATING
Update the rewards rate.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::staking_config {
    public fun update_rewards_rate(aptos_framework: &signer, new_rewards_rate: u64, new_rewards_rate_denominator: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun update_rewards_rate(
        aptos_framework: &signer,
        new_rewards_rate: u64,
        new_rewards_rate_denominator: u64,
    ) acquires StakingConfig {
        assert!(!features::periodical_reward_rate_decrease_enabled(), error::invalid_state(EDEPRECATED_FUNCTION));
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            new_rewards_rate_denominator > 0,
            error::invalid_argument(EZERO_REWARDS_RATE_DENOMINATOR),
        );
        // `rewards_rate` which is the numerator is limited to be `<= MAX_REWARDS_RATE` in order to avoid the arithmetic
        // overflow in the rewards calculation. `rewards_rate_denominator` can be adjusted to get the desired rewards
        // rate (i.e., rewards_rate / rewards_rate_denominator).
        assert!(new_rewards_rate <= MAX_REWARDS_RATE, error::invalid_argument(EINVALID_REWARDS_RATE));

        // We assert that (rewards_rate / rewards_rate_denominator <= 1).
        assert!(new_rewards_rate <= new_rewards_rate_denominator, error::invalid_argument(EINVALID_REWARDS_RATE));

        let staking_config = borrow_global_mut<StakingConfig>(@aptos_framework);
        staking_config.rewards_rate = new_rewards_rate;
        staking_config.rewards_rate_denominator = new_rewards_rate_denominator;
    }
}
```


<a id="0x1_staking_config_update_rewards_config"></a>

## Function `update_rewards_config`



```move
module 0x1::staking_config {
    public fun update_rewards_config(aptos_framework: &signer, rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun update_rewards_config(
        aptos_framework: &signer,
        rewards_rate: FixedPoint64,
        min_rewards_rate: FixedPoint64,
        rewards_rate_period_in_secs: u64,
        rewards_rate_decrease_rate: FixedPoint64,
    ) acquires StakingRewardsConfig {
        system_addresses::assert_aptos_framework(aptos_framework);

        validate_rewards_config(
            rewards_rate,
            min_rewards_rate,
            rewards_rate_period_in_secs,
            rewards_rate_decrease_rate,
        );

        let staking_rewards_config = borrow_global_mut<StakingRewardsConfig>(@aptos_framework);
        // Currently rewards_rate_period_in_secs is not allowed to be changed because this could bring complicated
        // logics. At the moment the argument is just a placeholder for future use.
        assert!(
            rewards_rate_period_in_secs == staking_rewards_config.rewards_rate_period_in_secs,
            error::invalid_argument(EINVALID_REWARDS_RATE_PERIOD),
        );
        staking_rewards_config.rewards_rate = rewards_rate;
        staking_rewards_config.min_rewards_rate = min_rewards_rate;
        staking_rewards_config.rewards_rate_period_in_secs = rewards_rate_period_in_secs;
        staking_rewards_config.rewards_rate_decrease_rate = rewards_rate_decrease_rate;
    }
}
```


<a id="0x1_staking_config_update_voting_power_increase_limit"></a>

## Function `update_voting_power_increase_limit`

Update the joining limit %.
Can only be called as part of the Aptos governance proposal process established by the AptosGovernance module.


```move
module 0x1::staking_config {
    public fun update_voting_power_increase_limit(aptos_framework: &signer, new_voting_power_increase_limit: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    public fun update_voting_power_increase_limit(
        aptos_framework: &signer,
        new_voting_power_increase_limit: u64,
    ) acquires StakingConfig {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            new_voting_power_increase_limit > 0 && new_voting_power_increase_limit <= 50,
            error::invalid_argument(EINVALID_VOTING_POWER_INCREASE_LIMIT),
        );

        let staking_config = borrow_global_mut<StakingConfig>(@aptos_framework);
        staking_config.voting_power_increase_limit = new_voting_power_increase_limit;
    }
}
```


<a id="0x1_staking_config_validate_required_stake"></a>

## Function `validate_required_stake`



```move
module 0x1::staking_config {
    fun validate_required_stake(minimum_stake: u64, maximum_stake: u64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    fun validate_required_stake(minimum_stake: u64, maximum_stake: u64) {
        assert!(minimum_stake <= maximum_stake && maximum_stake > 0, error::invalid_argument(EINVALID_STAKE_RANGE));
    }
}
```


<a id="0x1_staking_config_validate_rewards_config"></a>

## Function `validate_rewards_config`



```move
module 0x1::staking_config {
    fun validate_rewards_config(rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```


##### Implementation


```move
module 0x1::staking_config {
    fun validate_rewards_config(
        rewards_rate: FixedPoint64,
        min_rewards_rate: FixedPoint64,
        rewards_rate_period_in_secs: u64,
        rewards_rate_decrease_rate: FixedPoint64,
    ) {
        // Bound rewards rate to avoid arithmetic overflow.
        assert!(
            less_or_equal(rewards_rate, fixed_point64::create_from_u128((1u128))),
            error::invalid_argument(EINVALID_REWARDS_RATE)
        );
        assert!(
            less_or_equal(min_rewards_rate, rewards_rate),
            error::invalid_argument(EINVALID_MIN_REWARDS_RATE)
        );
        // Rewards rate decrease rate cannot be greater than 100%. Otherwise rewards rate will be negative.
        assert!(
            fixed_point64::ceil(rewards_rate_decrease_rate) <= 1,
            error::invalid_argument(EINVALID_REWARDS_RATE_DECREASE_RATE)
        );
        // This field, rewards_rate_period_in_secs must be greater than 0.
        // TODO: rewards_rate_period_in_secs should be longer than the epoch duration but reading epoch duration causes a circular dependency.
        assert!(
            rewards_rate_period_in_secs > 0,
            error::invalid_argument(EINVALID_REWARDS_RATE_PERIOD),
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
<td>The ability to initialize the staking config and staking rewards resources, as well as the ability to update the staking config and staking rewards should only be available to the Aptos framework account.</td>
<td>Medium</td>
<td>The function initialize and initialize_rewards are used to initialize the StakingConfig and StakingRewardConfig resources. Updating the resources, can be done using the update_required_stake, update_recurring_lockup_duration_secs, update_rewards_rate, update_rewards_config, update_voting_power_increase_limit functions, which ensure that the signer is aptos_framework using the assert_aptos_framework function.</td>
<td>Verified via [#high&#45;level&#45;req&#45;1.1](initialize), [#high&#45;level&#45;req&#45;1.2](initialize_rewards), [#high&#45;level&#45;req&#45;1.3](update_required_stake), [#high&#45;level&#45;req&#45;1.4](update_recurring_lockup_duration_secs), [#high&#45;level&#45;req&#45;1.5](update_rewards_rate), [#high&#45;level&#45;req&#45;1.6](update_rewards_config), and [#high&#45;level&#45;req&#45;1.7](update_voting_power_increase_limit).</td>
</tr>

<tr>
<td>2</td>
<td>The voting power increase, in a staking config resource, should always be greater than 0 and less or equal to 50.</td>
<td>High</td>
<td>During the initialization and update of the staking config, the value of voting_power_increase_limit is ensured to be in the range of (0 to 50].</td>
<td>Ensured via [#high&#45;level&#45;req&#45;2.1](initialize) and [#high&#45;level&#45;req&#45;2.2](update_voting_power_increase_limit). Formally verified via [#high&#45;level&#45;req&#45;2.3](StakingConfig).</td>
</tr>

<tr>
<td>3</td>
<td>The recurring lockup duration, in a staking config resource, should always be greater than 0.</td>
<td>Medium</td>
<td>During the initialization and update of the staking config, the value of recurring_lockup_duration_secs is ensured to be greater than 0.</td>
<td>Ensured via [#high&#45;level&#45;req&#45;3.1](initialize) and [#high&#45;level&#45;req&#45;3.2](update_recurring_lockup_duration_secs). Formally verified via [#high&#45;level&#45;req&#45;3.3](StakingConfig).</td>
</tr>

<tr>
<td>4</td>
<td>The calculation of rewards should not be possible if the last reward rate period just started.</td>
<td>High</td>
<td>The function calculate_and_save_latest_rewards_config ensures that last_rewards_rate_period_start_in_secs is greater or equal to the current timestamp.</td>
<td>Formally verified in [#high&#45;level&#45;req&#45;4](StakingRewardsConfigEnabledRequirement).</td>
</tr>

<tr>
<td>5</td>
<td>The rewards rate should always be less than or equal to 100%.</td>
<td>High</td>
<td>When initializing and updating the rewards rate, it is ensured that the rewards_rate is less or equal to MAX_REWARDS_RATE, otherwise rewards rate will be negative.</td>
<td>Verified via [#high&#45;level&#45;req&#45;5](StakingConfig).</td>
</tr>

<tr>
<td>6</td>
<td>The reward rate&apos;s denominator should never be 0.</td>
<td>High</td>
<td>While initializing and updating the rewards rate, rewards_rate_denominator is ensured to be greater than 0.</td>
<td>Verified via [#high&#45;level&#45;req&#45;6](StakingConfig).</td>
</tr>

<tr>
<td>7</td>
<td>The reward rate&apos;s nominator and dominator ratio should always be less or equal to 1.</td>
<td>High</td>
<td>When initializing and updating the rewards rate, it is ensured that rewards_rate is less or equal to rewards_rate_denominator.</td>
<td>Verified via [#high&#45;level&#45;req&#45;7](StakingConfig).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::staking_config {
    invariant [suspendable] chain_status::is_operating() ==> exists<StakingConfig>(@aptos_framework);
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_StakingConfig"></a>

### Resource `StakingConfig`


```move
module 0x1::staking_config {
    struct StakingConfig has copy, drop, key
}
```


<dl>
<dt>
`minimum_stake: u64`
</dt>
<dd>

</dd>
<dt>
`maximum_stake: u64`
</dt>
<dd>

</dd>
<dt>
`recurring_lockup_duration_secs: u64`
</dt>
<dd>

</dd>
<dt>
`allow_validator_set_change: bool`
</dt>
<dd>

</dd>
<dt>
`rewards_rate: u64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`voting_power_increase_limit: u64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::staking_config {
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    invariant rewards_rate <= MAX_REWARDS_RATE;
// This enforces ### high&#45;level&#45;req&#45;6
[#high&#45;level&#45;req](high&#45;level requirement 6):
    invariant rewards_rate_denominator > 0;
// This enforces ### high&#45;level&#45;req&#45;7
[#high&#45;level&#45;req](high&#45;level requirement 7):
    invariant rewards_rate <= rewards_rate_denominator;
// This enforces ### high&#45;level&#45;req&#45;3.3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    invariant recurring_lockup_duration_secs > 0;
// This enforces ### high&#45;level&#45;req&#45;2.3
[#high&#45;level&#45;req](high&#45;level requirement 2):
    invariant voting_power_increase_limit > 0 && voting_power_increase_limit <= 50;
}
```


<a id="@Specification_1_StakingRewardsConfig"></a>

### Resource `StakingRewardsConfig`


```move
module 0x1::staking_config {
    struct StakingRewardsConfig has copy, drop, key
}
```


<dl>
<dt>
`rewards_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
<dt>
`min_rewards_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_period_in_secs: u64`
</dt>
<dd>

</dd>
<dt>
`last_rewards_rate_period_start_in_secs: u64`
</dt>
<dd>

</dd>
<dt>
`rewards_rate_decrease_rate: fixed_point64::FixedPoint64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::staking_config {
    invariant fixed_point64::spec_less_or_equal(
        rewards_rate,
        fixed_point64::spec_create_from_u128((1u128)));
    invariant fixed_point64::spec_less_or_equal(min_rewards_rate, rewards_rate);
    invariant rewards_rate_period_in_secs > 0;
    invariant fixed_point64::spec_ceil(rewards_rate_decrease_rate) <= 1;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::staking_config {
    public(friend) fun initialize(aptos_framework: &signer, minimum_stake: u64, maximum_stake: u64, recurring_lockup_duration_secs: u64, allow_validator_set_change: bool, rewards_rate: u64, rewards_rate_denominator: u64, voting_power_increase_limit: u64)
}
```

Caller must be @aptos_framework.
The maximum_stake must be greater than maximum_stake in the range of Specified stake and the maximum_stake greater than zero.
The rewards_rate_denominator must greater than zero.
Only this %0&#45;%50 of current total voting power is allowed to join the validator set in each epoch.
The `rewards_rate` which is the numerator is limited to be `<= MAX_REWARDS_RATE` in order to avoid the arithmetic overflow in the rewards calculation.
rewards_rate/rewards_rate_denominator &lt;&#61; 1.
StakingConfig does not exist under the aptos_framework before creating it.


```move
module 0x1::staking_config {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
    aborts_if minimum_stake > maximum_stake || maximum_stake == 0;
// This enforces ### high&#45;level&#45;req&#45;3.1
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if recurring_lockup_duration_secs == 0;
    aborts_if rewards_rate_denominator == 0;
// This enforces ### high&#45;level&#45;req&#45;2.1
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if voting_power_increase_limit == 0 || voting_power_increase_limit > 50;
    aborts_if rewards_rate > MAX_REWARDS_RATE;
    aborts_if rewards_rate > rewards_rate_denominator;
    aborts_if exists<StakingConfig>(addr);
    ensures exists<StakingConfig>(addr);
}
```


<a id="@Specification_1_reward_rate"></a>

### Function `reward_rate`


```move
module 0x1::staking_config {
    #[view]
    public fun reward_rate(): (u64, u64)
}
```



```move
module 0x1::staking_config {
    let config = global<StakingConfig>(@aptos_framework);
    aborts_if !exists<StakingConfig>(@aptos_framework);
    include StakingRewardsConfigRequirement;
    ensures (features::spec_periodical_reward_rate_decrease_enabled() &&
        (global<StakingRewardsConfig>(@aptos_framework).rewards_rate.value as u64) != 0) ==>
        result_1 <= MAX_REWARDS_RATE && result_2 <= MAX_U64;
}
```


<a id="@Specification_1_initialize_rewards"></a>

### Function `initialize_rewards`


```move
module 0x1::staking_config {
    public fun initialize_rewards(aptos_framework: &signer, rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, last_rewards_rate_period_start_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```

Caller must be @aptos_framework.
last_rewards_rate_period_start_in_secs cannot be later than now.
Abort at any condition in StakingRewardsConfigValidationAborts.
StakingRewardsConfig does not exist under the aptos_framework before creating it.


```move
module 0x1::staking_config {
    pragma verify_duration_estimate = 120;
    requires exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.2
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
    aborts_if last_rewards_rate_period_start_in_secs > timestamp::spec_now_seconds();
    include StakingRewardsConfigValidationAbortsIf;
    aborts_if exists<StakingRewardsConfig>(addr);
    ensures exists<StakingRewardsConfig>(addr);
}
```


<a id="@Specification_1_get"></a>

### Function `get`


```move
module 0x1::staking_config {
    public fun get(): staking_config::StakingConfig
}
```



```move
module 0x1::staking_config {
    aborts_if !exists<StakingConfig>(@aptos_framework);
}
```


<a id="@Specification_1_get_reward_rate"></a>

### Function `get_reward_rate`


```move
module 0x1::staking_config {
    public fun get_reward_rate(config: &staking_config::StakingConfig): (u64, u64)
}
```



```move
module 0x1::staking_config {
    include StakingRewardsConfigRequirement;
    ensures (features::spec_periodical_reward_rate_decrease_enabled() &&
        (global<StakingRewardsConfig>(@aptos_framework).rewards_rate.value as u64) != 0) ==>
            result_1 <= MAX_REWARDS_RATE && result_2 <= MAX_U64;
}
```


<a id="@Specification_1_calculate_and_save_latest_epoch_rewards_rate"></a>

### Function `calculate_and_save_latest_epoch_rewards_rate`


```move
module 0x1::staking_config {
    public(friend) fun calculate_and_save_latest_epoch_rewards_rate(): fixed_point64::FixedPoint64
}
```



```move
module 0x1::staking_config {
    pragma verify_duration_estimate = 120;
    aborts_if !exists<StakingRewardsConfig>(@aptos_framework);
    aborts_if !features::spec_periodical_reward_rate_decrease_enabled();
    include StakingRewardsConfigRequirement;
}
```


<a id="@Specification_1_calculate_and_save_latest_rewards_config"></a>

### Function `calculate_and_save_latest_rewards_config`


```move
module 0x1::staking_config {
    fun calculate_and_save_latest_rewards_config(): staking_config::StakingRewardsConfig
}
```



```move
module 0x1::staking_config {
    pragma verify_duration_estimate = 120;
    requires features::spec_periodical_reward_rate_decrease_enabled();
    include StakingRewardsConfigRequirement;
    aborts_if !exists<StakingRewardsConfig>(@aptos_framework);
}
```


<a id="@Specification_1_update_required_stake"></a>

### Function `update_required_stake`


```move
module 0x1::staking_config {
    public fun update_required_stake(aptos_framework: &signer, minimum_stake: u64, maximum_stake: u64)
}
```

Caller must be @aptos_framework.
The maximum_stake must be greater than maximum_stake in the range of Specified stake and the maximum_stake greater than zero.
The StakingConfig is under @aptos_framework.


```move
module 0x1::staking_config {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.3
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
    aborts_if minimum_stake > maximum_stake || maximum_stake == 0;
    aborts_if !exists<StakingConfig>(@aptos_framework);
    ensures global<StakingConfig>(@aptos_framework).minimum_stake == minimum_stake &&
        global<StakingConfig>(@aptos_framework).maximum_stake == maximum_stake;
}
```


<a id="@Specification_1_update_recurring_lockup_duration_secs"></a>

### Function `update_recurring_lockup_duration_secs`


```move
module 0x1::staking_config {
    public fun update_recurring_lockup_duration_secs(aptos_framework: &signer, new_recurring_lockup_duration_secs: u64)
}
```

Caller must be @aptos_framework.
The new_recurring_lockup_duration_secs must greater than zero.
The StakingConfig is under @aptos_framework.


```move
module 0x1::staking_config {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.4
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
// This enforces ### high&#45;level&#45;req&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if new_recurring_lockup_duration_secs == 0;
    aborts_if !exists<StakingConfig>(@aptos_framework);
    ensures global<StakingConfig>(@aptos_framework).recurring_lockup_duration_secs == new_recurring_lockup_duration_secs;
}
```


<a id="@Specification_1_update_rewards_rate"></a>

### Function `update_rewards_rate`


```move
module 0x1::staking_config {
    public fun update_rewards_rate(aptos_framework: &signer, new_rewards_rate: u64, new_rewards_rate_denominator: u64)
}
```

Caller must be @aptos_framework.
The new_rewards_rate_denominator must greater than zero.
The StakingConfig is under @aptos_framework.
The `rewards_rate` which is the numerator is limited to be `<= MAX_REWARDS_RATE` in order to avoid the arithmetic overflow in the rewards calculation.
rewards_rate/rewards_rate_denominator &lt;&#61; 1.


```move
module 0x1::staking_config {
    aborts_if features::spec_periodical_reward_rate_decrease_enabled();
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.5
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
    aborts_if new_rewards_rate_denominator == 0;
    aborts_if !exists<StakingConfig>(@aptos_framework);
    aborts_if new_rewards_rate > MAX_REWARDS_RATE;
    aborts_if new_rewards_rate > new_rewards_rate_denominator;
    let post staking_config = global<StakingConfig>(@aptos_framework);
    ensures staking_config.rewards_rate == new_rewards_rate;
    ensures staking_config.rewards_rate_denominator == new_rewards_rate_denominator;
}
```


<a id="@Specification_1_update_rewards_config"></a>

### Function `update_rewards_config`


```move
module 0x1::staking_config {
    public fun update_rewards_config(aptos_framework: &signer, rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```

Caller must be @aptos_framework.
StakingRewardsConfig is under the @aptos_framework.


```move
module 0x1::staking_config {
    pragma verify_duration_estimate = 120;
    include StakingRewardsConfigRequirement;
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.6
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
    aborts_if global<StakingRewardsConfig>(@aptos_framework).rewards_rate_period_in_secs != rewards_rate_period_in_secs;
    include StakingRewardsConfigValidationAbortsIf;
    aborts_if !exists<StakingRewardsConfig>(addr);
    let post staking_rewards_config = global<StakingRewardsConfig>(@aptos_framework);
    ensures staking_rewards_config.rewards_rate == rewards_rate;
    ensures staking_rewards_config.min_rewards_rate == min_rewards_rate;
    ensures staking_rewards_config.rewards_rate_period_in_secs == rewards_rate_period_in_secs;
    ensures staking_rewards_config.rewards_rate_decrease_rate == rewards_rate_decrease_rate;
}
```


<a id="@Specification_1_update_voting_power_increase_limit"></a>

### Function `update_voting_power_increase_limit`


```move
module 0x1::staking_config {
    public fun update_voting_power_increase_limit(aptos_framework: &signer, new_voting_power_increase_limit: u64)
}
```

Caller must be @aptos_framework.
Only this %0&#45;%50 of current total voting power is allowed to join the validator set in each epoch.
The StakingConfig is under @aptos_framework.


```move
module 0x1::staking_config {
    let addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.7
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if addr != @aptos_framework;
// This enforces ### high&#45;level&#45;req&#45;2.2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if new_voting_power_increase_limit == 0 || new_voting_power_increase_limit > 50;
    aborts_if !exists<StakingConfig>(@aptos_framework);
    ensures global<StakingConfig>(@aptos_framework).voting_power_increase_limit == new_voting_power_increase_limit;
}
```


<a id="@Specification_1_validate_required_stake"></a>

### Function `validate_required_stake`


```move
module 0x1::staking_config {
    fun validate_required_stake(minimum_stake: u64, maximum_stake: u64)
}
```

The maximum_stake must be greater than maximum_stake in the range of Specified stake and the maximum_stake greater than zero.


```move
module 0x1::staking_config {
    aborts_if minimum_stake > maximum_stake || maximum_stake == 0;
}
```


<a id="@Specification_1_validate_rewards_config"></a>

### Function `validate_rewards_config`


```move
module 0x1::staking_config {
    fun validate_rewards_config(rewards_rate: fixed_point64::FixedPoint64, min_rewards_rate: fixed_point64::FixedPoint64, rewards_rate_period_in_secs: u64, rewards_rate_decrease_rate: fixed_point64::FixedPoint64)
}
```

Abort at any condition in StakingRewardsConfigValidationAborts.


```move
module 0x1::staking_config {
    include StakingRewardsConfigValidationAbortsIf;
}
```

rewards_rate must be within [0, 1].
min_rewards_rate must be not greater than rewards_rate.
rewards_rate_period_in_secs must be greater than 0.
rewards_rate_decrease_rate must be within [0,1].


<a id="0x1_staking_config_StakingRewardsConfigValidationAbortsIf"></a>


```move
module 0x1::staking_config {
    schema StakingRewardsConfigValidationAbortsIf {
        rewards_rate: FixedPoint64;
        min_rewards_rate: FixedPoint64;
        rewards_rate_period_in_secs: u64;
        rewards_rate_decrease_rate: FixedPoint64;
        aborts_if fixed_point64::spec_greater(
            rewards_rate,
            fixed_point64::spec_create_from_u128((1u128)));
        aborts_if fixed_point64::spec_greater(min_rewards_rate, rewards_rate);
        aborts_if rewards_rate_period_in_secs == 0;
        aborts_if fixed_point64::spec_ceil(rewards_rate_decrease_rate) > 1;
    }
}
```



<a id="0x1_staking_config_StakingRewardsConfigRequirement"></a>


```move
module 0x1::staking_config {
    schema StakingRewardsConfigRequirement {
        requires exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
        include features::spec_periodical_reward_rate_decrease_enabled() ==> StakingRewardsConfigEnabledRequirement;
    }
}
```



<a id="0x1_staking_config_StakingRewardsConfigEnabledRequirement"></a>


```move
module 0x1::staking_config {
    schema StakingRewardsConfigEnabledRequirement {
        requires exists<StakingRewardsConfig>(@aptos_framework);
        let staking_rewards_config = global<StakingRewardsConfig>(@aptos_framework);
        let rewards_rate = staking_rewards_config.rewards_rate;
        let min_rewards_rate = staking_rewards_config.min_rewards_rate;
        let rewards_rate_period_in_secs = staking_rewards_config.rewards_rate_period_in_secs;
        let last_rewards_rate_period_start_in_secs = staking_rewards_config.last_rewards_rate_period_start_in_secs;
        let rewards_rate_decrease_rate = staking_rewards_config.rewards_rate_decrease_rate;
        requires fixed_point64::spec_less_or_equal(
            rewards_rate,
            fixed_point64::spec_create_from_u128((1u128)));
        requires fixed_point64::spec_less_or_equal(min_rewards_rate, rewards_rate);
        requires rewards_rate_period_in_secs > 0;
    // This enforces ### high&#45;level&#45;req&#45;4
    [#high&#45;level&#45;req](high&#45;level requirement 4):
        requires last_rewards_rate_period_start_in_secs <= timestamp::spec_now_seconds();
        requires fixed_point64::spec_ceil(rewards_rate_decrease_rate) <= 1;
    }
}
```
