
<a id="0x1_randomness_api_v0_config"></a>

# Module `0x1::randomness_api_v0_config`



-  [Resource `RequiredGasDeposit`](#0x1_randomness_api_v0_config_RequiredGasDeposit)
-  [Resource `AllowCustomMaxGasFlag`](#0x1_randomness_api_v0_config_AllowCustomMaxGasFlag)
-  [Function `initialize`](#0x1_randomness_api_v0_config_initialize)
-  [Function `set_for_next_epoch`](#0x1_randomness_api_v0_config_set_for_next_epoch)
-  [Function `set_allow_max_gas_flag_for_next_epoch`](#0x1_randomness_api_v0_config_set_allow_max_gas_flag_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_randomness_api_v0_config_on_new_epoch)
-  [Specification](#@Specification_0)


```move
module 0x1::randomness_api_v0_config {
    use 0x1::chain_status;
    use 0x1::config_buffer;
    use 0x1::option;
    use 0x1::system_addresses;
}
```


<a id="0x1_randomness_api_v0_config_RequiredGasDeposit"></a>

## Resource `RequiredGasDeposit`



```move
module 0x1::randomness_api_v0_config {
    struct RequiredGasDeposit has drop, store, key
}
```


##### Fields


<dl>
<dt>
`gas_amount: option::Option<u64>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_randomness_api_v0_config_AllowCustomMaxGasFlag"></a>

## Resource `AllowCustomMaxGasFlag`

If this flag is set, `max_gas` specified inside `#[randomness()]` will be used as the required deposit.


```move
module 0x1::randomness_api_v0_config {
    struct AllowCustomMaxGasFlag has drop, store, key
}
```


##### Fields


<dl>
<dt>
`value: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_randomness_api_v0_config_initialize"></a>

## Function `initialize`

Only used in genesis.


```move
module 0x1::randomness_api_v0_config {
    fun initialize(framework: &signer, required_amount: randomness_api_v0_config::RequiredGasDeposit, allow_custom_max_gas_flag: randomness_api_v0_config::AllowCustomMaxGasFlag)
}
```


##### Implementation


```move
module 0x1::randomness_api_v0_config {
    fun initialize(framework: &signer, required_amount: RequiredGasDeposit, allow_custom_max_gas_flag: AllowCustomMaxGasFlag) {
        system_addresses::assert_aptos_framework(framework);
        chain_status::assert_genesis();
        move_to(framework, required_amount);
        move_to(framework, allow_custom_max_gas_flag);
    }
}
```


<a id="0x1_randomness_api_v0_config_set_for_next_epoch"></a>

## Function `set_for_next_epoch`

This can be called by on&#45;chain governance to update `RequiredGasDeposit` for the next epoch.


```move
module 0x1::randomness_api_v0_config {
    public fun set_for_next_epoch(framework: &signer, gas_amount: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::randomness_api_v0_config {
    public fun set_for_next_epoch(framework: &signer, gas_amount: Option<u64>) {
        system_addresses::assert_aptos_framework(framework);
        config_buffer::upsert(RequiredGasDeposit { gas_amount });
    }
}
```


<a id="0x1_randomness_api_v0_config_set_allow_max_gas_flag_for_next_epoch"></a>

## Function `set_allow_max_gas_flag_for_next_epoch`

This can be called by on&#45;chain governance to update `AllowCustomMaxGasFlag` for the next epoch.


```move
module 0x1::randomness_api_v0_config {
    public fun set_allow_max_gas_flag_for_next_epoch(framework: &signer, value: bool)
}
```


##### Implementation


```move
module 0x1::randomness_api_v0_config {
    public fun set_allow_max_gas_flag_for_next_epoch(framework: &signer, value: bool) {
        system_addresses::assert_aptos_framework(framework);
        config_buffer::upsert(AllowCustomMaxGasFlag { value } );
    }
}
```


<a id="0x1_randomness_api_v0_config_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `RequiredGasDeposit`, if there is any.


```move
module 0x1::randomness_api_v0_config {
    public fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::randomness_api_v0_config {
    public fun on_new_epoch(framework: &signer) acquires RequiredGasDeposit, AllowCustomMaxGasFlag {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<RequiredGasDeposit>()) {
            let new_config = config_buffer::extract<RequiredGasDeposit>();
            if (exists<RequiredGasDeposit>(@aptos_framework)) {
                *borrow_global_mut<RequiredGasDeposit>(@aptos_framework) = new_config;
            } else {
                move_to(framework, new_config);
            }
        };
        if (config_buffer::does_exist<AllowCustomMaxGasFlag>()) {
            let new_config = config_buffer::extract<AllowCustomMaxGasFlag>();
            if (exists<AllowCustomMaxGasFlag>(@aptos_framework)) {
                *borrow_global_mut<AllowCustomMaxGasFlag>(@aptos_framework) = new_config;
            } else {
                move_to(framework, new_config);
            }
        }
    }
}
```


<a id="@Specification_0"></a>

## Specification



```move
module 0x1::randomness_api_v0_config {
    pragma verify = false;
}
```
