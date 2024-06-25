
<a id="0x1_reconfiguration_with_dkg"></a>

# Module `0x1::reconfiguration_with_dkg`

Reconfiguration with DKG helper functions.


-  [Function `try_start`](#0x1_reconfiguration_with_dkg_try_start)
-  [Function `finish`](#0x1_reconfiguration_with_dkg_finish)
-  [Function `finish_with_dkg_result`](#0x1_reconfiguration_with_dkg_finish_with_dkg_result)
-  [Specification](#@Specification_0)
    -  [Function `try_start`](#@Specification_0_try_start)
    -  [Function `finish`](#@Specification_0_finish)
    -  [Function `finish_with_dkg_result`](#@Specification_0_finish_with_dkg_result)


```move
module 0x1::reconfiguration_with_dkg {
    use 0x1::consensus_config;
    use 0x1::dkg;
    use 0x1::execution_config;
    use 0x1::features;
    use 0x1::gas_schedule;
    use 0x1::jwk_consensus_config;
    use 0x1::jwks;
    use 0x1::keyless_account;
    use 0x1::option;
    use 0x1::randomness_api_v0_config;
    use 0x1::randomness_config;
    use 0x1::randomness_config_seqnum;
    use 0x1::reconfiguration;
    use 0x1::reconfiguration_state;
    use 0x1::stake;
    use 0x1::system_addresses;
    use 0x1::validator_consensus_info;
    use 0x1::version;
}
```


<a id="0x1_reconfiguration_with_dkg_try_start"></a>

## Function `try_start`

Trigger a reconfiguration with DKG.
Do nothing if one is already in progress.


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun try_start()
}
```


##### Implementation


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun try_start() {
        let incomplete_dkg_session = dkg::incomplete_session();
        if (option::is_some(&incomplete_dkg_session)) {
            let session = option::borrow(&incomplete_dkg_session);
            if (dkg::session_dealer_epoch(session) == reconfiguration::current_epoch()) {
                return
            }
        };
        reconfiguration_state::on_reconfig_start();
        let cur_epoch = reconfiguration::current_epoch();
        dkg::start(
            cur_epoch,
            randomness_config::current(),
            stake::cur_validator_consensus_infos(),
            stake::next_validator_consensus_infos(),
        );
    }
}
```


<a id="0x1_reconfiguration_with_dkg_finish"></a>

## Function `finish`

Clear incomplete DKG session, if it exists.
Apply buffered on&#45;chain configs (except for ValidatorSet, which is done inside `reconfiguration::reconfigure()`).
Re&#45;enable validator set changes.
Run the default reconfiguration to enter the new epoch.


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun finish(framework: &signer)
}
```


##### Implementation


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun finish(framework: &signer) {
        system_addresses::assert_aptos_framework(framework);
        dkg::try_clear_incomplete_session(framework);
        consensus_config::on_new_epoch(framework);
        execution_config::on_new_epoch(framework);
        gas_schedule::on_new_epoch(framework);
        std::version::on_new_epoch(framework);
        features::on_new_epoch(framework);
        jwk_consensus_config::on_new_epoch(framework);
        jwks::on_new_epoch(framework);
        keyless_account::on_new_epoch(framework);
        randomness_config_seqnum::on_new_epoch(framework);
        randomness_config::on_new_epoch(framework);
        randomness_api_v0_config::on_new_epoch(framework);
        reconfiguration::reconfigure();
    }
}
```


<a id="0x1_reconfiguration_with_dkg_finish_with_dkg_result"></a>

## Function `finish_with_dkg_result`

Complete the current reconfiguration with DKG.
Abort if no DKG is in progress.


```move
module 0x1::reconfiguration_with_dkg {
    fun finish_with_dkg_result(account: &signer, dkg_result: vector<u8>)
}
```


##### Implementation


```move
module 0x1::reconfiguration_with_dkg {
    fun finish_with_dkg_result(account: &signer, dkg_result: vector<u8>) {
        dkg::finish(dkg_result);
        finish(account);
    }
}
```


<a id="@Specification_0"></a>

## Specification



```move
module 0x1::reconfiguration_with_dkg {
    pragma verify = true;
}
```


<a id="@Specification_0_try_start"></a>

### Function `try_start`


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun try_start()
}
```



```move
module 0x1::reconfiguration_with_dkg {
    pragma verify_duration_estimate = 120;
    requires exists<reconfiguration::Configuration>(@aptos_framework);
    requires chain_status::is_operating();
    include stake::ResourceRequirement;
    include stake::GetReconfigStartTimeRequirement;
    include features::spec_periodical_reward_rate_decrease_enabled(
    ) ==> staking_config::StakingRewardsConfigEnabledRequirement;
    aborts_if false;
    pragma verify_duration_estimate = 600;
}
```


<a id="@Specification_0_finish"></a>

### Function `finish`


```move
module 0x1::reconfiguration_with_dkg {
    public(friend) fun finish(framework: &signer)
}
```



```move
module 0x1::reconfiguration_with_dkg {
    pragma verify_duration_estimate = 1500;
    include FinishRequirement;
    aborts_if false;
}
```



<a id="0x1_reconfiguration_with_dkg_FinishRequirement"></a>


```move
module 0x1::reconfiguration_with_dkg {
    schema FinishRequirement {
        framework: signer;
        requires signer::address_of(framework) == @aptos_framework;
        requires chain_status::is_operating();
        requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
        include staking_config::StakingRewardsConfigRequirement;
        requires exists<stake::ValidatorFees>(@aptos_framework);
        include transaction_fee::RequiresCollectedFeesPerValueLeqBlockAptosSupply;
        requires exists<features::Features>(@std);
        include config_buffer::OnNewEpochRequirement<version::Version>;
        include config_buffer::OnNewEpochRequirement<gas_schedule::GasScheduleV2>;
        include config_buffer::OnNewEpochRequirement<execution_config::ExecutionConfig>;
        include config_buffer::OnNewEpochRequirement<consensus_config::ConsensusConfig>;
        include config_buffer::OnNewEpochRequirement<jwks::SupportedOIDCProviders>;
        include config_buffer::OnNewEpochRequirement<randomness_config::RandomnessConfig>;
        include config_buffer::OnNewEpochRequirement<randomness_config_seqnum::RandomnessConfigSeqNum>;
        include config_buffer::OnNewEpochRequirement<randomness_api_v0_config::AllowCustomMaxGasFlag>;
        include config_buffer::OnNewEpochRequirement<randomness_api_v0_config::RequiredGasDeposit>;
        include config_buffer::OnNewEpochRequirement<jwk_consensus_config::JWKConsensusConfig>;
        include config_buffer::OnNewEpochRequirement<keyless_account::Configuration>;
        include config_buffer::OnNewEpochRequirement<keyless_account::Groth16VerificationKey>;
    }
}
```


<a id="@Specification_0_finish_with_dkg_result"></a>

### Function `finish_with_dkg_result`


```move
module 0x1::reconfiguration_with_dkg {
    fun finish_with_dkg_result(account: &signer, dkg_result: vector<u8>)
}
```



```move
module 0x1::reconfiguration_with_dkg {
    pragma verify_duration_estimate = 1500;
    include FinishRequirement {
        framework: account
    };
    requires dkg::has_incomplete_session();
    aborts_if false;
}
```
