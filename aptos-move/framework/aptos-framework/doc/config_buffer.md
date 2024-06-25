
<a id="0x1_config_buffer"></a>

# Module `0x1::config_buffer`

This wrapper helps store an on&#45;chain config for the next epoch.

Once reconfigure with DKG is introduced, every on&#45;chain config `C` should do the following.
&#45; Support async update when DKG is enabled. This is typically done by 3 steps below.
&#45; Implement `C::set_for_next_epoch()` using `upsert()` function in this module.
&#45; Implement `C::on_new_epoch()` using `extract()` function in this module.
&#45; Update `0x1::reconfiguration_with_dkg::finish()` to call `C::on_new_epoch()`.
&#45; Support sychronous update when DKG is disabled.
This is typically done by implementing `C::set()` to update the config resource directly.

NOTE: on&#45;chain config `0x1::state::ValidatorSet` implemented its own buffer.


-  [Resource `PendingConfigs`](#0x1_config_buffer_PendingConfigs)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_config_buffer_initialize)
-  [Function `does_exist`](#0x1_config_buffer_does_exist)
-  [Function `upsert`](#0x1_config_buffer_upsert)
-  [Function `extract`](#0x1_config_buffer_extract)
-  [Specification](#@Specification_1)
    -  [Function `does_exist`](#@Specification_1_does_exist)
    -  [Function `upsert`](#@Specification_1_upsert)
    -  [Function `extract`](#@Specification_1_extract)


```move
module 0x1::config_buffer {
    use 0x1::any;
    use 0x1::option;
    use 0x1::simple_map;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::type_info;
}
```


<a id="0x1_config_buffer_PendingConfigs"></a>

## Resource `PendingConfigs`



```move
module 0x1::config_buffer {
    struct PendingConfigs has key
}
```


##### Fields


<dl>
<dt>
`configs: simple_map::SimpleMap<string::String, any::Any>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_config_buffer_ESTD_SIGNER_NEEDED"></a>

Config buffer operations failed with permission denied.


```move
module 0x1::config_buffer {
    const ESTD_SIGNER_NEEDED: u64 = 1;
}
```


<a id="0x1_config_buffer_initialize"></a>

## Function `initialize`



```move
module 0x1::config_buffer {
    public fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::config_buffer {
    public fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        if (!exists<PendingConfigs>(@aptos_framework)) {
            move_to(aptos_framework, PendingConfigs {
                configs: simple_map::new(),
            })
        }
    }
}
```


<a id="0x1_config_buffer_does_exist"></a>

## Function `does_exist`

Check whether there is a pending config payload for `T`.


```move
module 0x1::config_buffer {
    public fun does_exist<T: store>(): bool
}
```


##### Implementation


```move
module 0x1::config_buffer {
    public fun does_exist<T: store>(): bool acquires PendingConfigs {
        if (exists<PendingConfigs>(@aptos_framework)) {
            let config = borrow_global<PendingConfigs>(@aptos_framework);
            simple_map::contains_key(&config.configs, &type_info::type_name<T>())
        } else {
            false
        }
    }
}
```


<a id="0x1_config_buffer_upsert"></a>

## Function `upsert`

Upsert an on&#45;chain config to the buffer for the next epoch.

Typically used in `X::set_for_next_epoch()` where X is an on&#45;chain config.


```move
module 0x1::config_buffer {
    public(friend) fun upsert<T: drop, store>(config: T)
}
```


##### Implementation


```move
module 0x1::config_buffer {
    public(friend) fun upsert<T: drop + store>(config: T) acquires PendingConfigs {
        let configs = borrow_global_mut<PendingConfigs>(@aptos_framework);
        let key = type_info::type_name<T>();
        let value = any::pack(config);
        simple_map::upsert(&mut configs.configs, key, value);
    }
}
```


<a id="0x1_config_buffer_extract"></a>

## Function `extract`

Take the buffered config `T` out (buffer cleared). Abort if the buffer is empty.
Should only be used at the end of a reconfiguration.

Typically used in `X::on_new_epoch()` where X is an on&#45;chaon config.


```move
module 0x1::config_buffer {
    public fun extract<T: store>(): T
}
```


##### Implementation


```move
module 0x1::config_buffer {
    public fun extract<T: store>(): T acquires PendingConfigs {
        let configs = borrow_global_mut<PendingConfigs>(@aptos_framework);
        let key = type_info::type_name<T>();
        let (_, value_packed) = simple_map::remove(&mut configs.configs, &key);
        any::unpack(value_packed)
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::config_buffer {
    pragma verify = true;
}
```


<a id="@Specification_1_does_exist"></a>

### Function `does_exist`


```move
module 0x1::config_buffer {
    public fun does_exist<T: store>(): bool
}
```



```move
module 0x1::config_buffer {
    aborts_if false;
    let type_name = type_info::type_name<T>();
    ensures result == spec_fun_does_exist<T>(type_name);
}
```



<a id="0x1_config_buffer_spec_fun_does_exist"></a>


```move
module 0x1::config_buffer {
    fun spec_fun_does_exist<T: store>(type_name: String): bool {
       if (exists<PendingConfigs>(@aptos_framework)) {
           let config = global<PendingConfigs>(@aptos_framework);
           simple_map::spec_contains_key(config.configs, type_name)
       } else {
           false
       }
    }
}
```


<a id="@Specification_1_upsert"></a>

### Function `upsert`


```move
module 0x1::config_buffer {
    public(friend) fun upsert<T: drop, store>(config: T)
}
```



```move
module 0x1::config_buffer {
    aborts_if !exists<PendingConfigs>(@aptos_framework);
}
```


<a id="@Specification_1_extract"></a>

### Function `extract`


```move
module 0x1::config_buffer {
    public fun extract<T: store>(): T
}
```



```move
module 0x1::config_buffer {
    aborts_if !exists<PendingConfigs>(@aptos_framework);
    include ExtractAbortsIf<T>;
}
```



<a id="0x1_config_buffer_ExtractAbortsIf"></a>


```move
module 0x1::config_buffer {
    schema ExtractAbortsIf<T> {
        let configs = global<PendingConfigs>(@aptos_framework);
        let key = type_info::type_name<T>();
        aborts_if !simple_map::spec_contains_key(configs.configs, key);
        include any::UnpackAbortsIf<T> {
            x: simple_map::spec_get(configs.configs, key)
        };
    }
}
```



<a id="0x1_config_buffer_SetForNextEpochAbortsIf"></a>


```move
module 0x1::config_buffer {
    schema SetForNextEpochAbortsIf {
        account: &signer;
        config: vector<u8>;
        let account_addr = std::signer::address_of(account);
        aborts_if account_addr != @aptos_framework;
        aborts_if len(config) == 0;
        aborts_if !exists<PendingConfigs>(@aptos_framework);
    }
}
```



<a id="0x1_config_buffer_OnNewEpochAbortsIf"></a>


```move
module 0x1::config_buffer {
    schema OnNewEpochAbortsIf<T> {
        let type_name = type_info::type_name<T>();
        let configs = global<PendingConfigs>(@aptos_framework);
        include spec_fun_does_exist<T>(type_name) ==> any::UnpackAbortsIf<T> {
            x: simple_map::spec_get(configs.configs, type_name)
        };
    }
}
```



<a id="0x1_config_buffer_OnNewEpochRequirement"></a>


```move
module 0x1::config_buffer {
    schema OnNewEpochRequirement<T> {
        let type_name = type_info::type_name<T>();
        let configs = global<PendingConfigs>(@aptos_framework);
        include spec_fun_does_exist<T>(type_name) ==> any::UnpackRequirement<T> {
            x: simple_map::spec_get(configs.configs, type_name)
        };
    }
}
```
