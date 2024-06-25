
<a id="0x1_type_info"></a>

# Module `0x1::type_info`



-  [Struct `TypeInfo`](#0x1_type_info_TypeInfo)
-  [Constants](#@Constants_0)
-  [Function `account_address`](#0x1_type_info_account_address)
-  [Function `module_name`](#0x1_type_info_module_name)
-  [Function `struct_name`](#0x1_type_info_struct_name)
-  [Function `chain_id`](#0x1_type_info_chain_id)
-  [Function `type_of`](#0x1_type_info_type_of)
-  [Function `type_name`](#0x1_type_info_type_name)
-  [Function `chain_id_internal`](#0x1_type_info_chain_id_internal)
-  [Function `size_of_val`](#0x1_type_info_size_of_val)
-  [Function `verify_type_of`](#0x1_type_info_verify_type_of)
-  [Function `verify_type_of_generic`](#0x1_type_info_verify_type_of_generic)
-  [Specification](#@Specification_1)
    -  [Function `chain_id`](#@Specification_1_chain_id)
    -  [Function `type_of`](#@Specification_1_type_of)
    -  [Function `type_name`](#@Specification_1_type_name)
    -  [Function `chain_id_internal`](#@Specification_1_chain_id_internal)
    -  [Function `size_of_val`](#@Specification_1_size_of_val)
    -  [Function `verify_type_of_generic`](#@Specification_1_verify_type_of_generic)


```move
module 0x1::type_info {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::features;
    use 0x1::string;
}
```


<a id="0x1_type_info_TypeInfo"></a>

## Struct `TypeInfo`



```move
module 0x1::type_info {
    struct TypeInfo has copy, drop, store
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
`module_name: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`struct_name: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_type_info_E_NATIVE_FUN_NOT_AVAILABLE"></a>



```move
module 0x1::type_info {
    const E_NATIVE_FUN_NOT_AVAILABLE: u64 = 1;
}
```


<a id="0x1_type_info_account_address"></a>

## Function `account_address`



```move
module 0x1::type_info {
    public fun account_address(type_info: &type_info::TypeInfo): address
}
```


##### Implementation


```move
module 0x1::type_info {
    public fun account_address(type_info: &TypeInfo): address {
        type_info.account_address
    }
}
```


<a id="0x1_type_info_module_name"></a>

## Function `module_name`



```move
module 0x1::type_info {
    public fun module_name(type_info: &type_info::TypeInfo): vector<u8>
}
```


##### Implementation


```move
module 0x1::type_info {
    public fun module_name(type_info: &TypeInfo): vector<u8> {
        type_info.module_name
    }
}
```


<a id="0x1_type_info_struct_name"></a>

## Function `struct_name`



```move
module 0x1::type_info {
    public fun struct_name(type_info: &type_info::TypeInfo): vector<u8>
}
```


##### Implementation


```move
module 0x1::type_info {
    public fun struct_name(type_info: &TypeInfo): vector<u8> {
        type_info.struct_name
    }
}
```


<a id="0x1_type_info_chain_id"></a>

## Function `chain_id`

Returns the current chain ID, mirroring what `aptos_framework::chain_id::get()` would return, except in `#[test]`
functions, where this will always return `4u8` as the chain ID, whereas `aptos_framework::chain_id::get()` will
return whichever ID was passed to `aptos_framework::chain_id::initialize_for_test()`.


```move
module 0x1::type_info {
    public fun chain_id(): u8
}
```


##### Implementation


```move
module 0x1::type_info {
    public fun chain_id(): u8 {
        if (!features::aptos_stdlib_chain_id_enabled()) {
            abort(std::error::invalid_state(E_NATIVE_FUN_NOT_AVAILABLE))
        };

        chain_id_internal()
    }
}
```


<a id="0x1_type_info_type_of"></a>

## Function `type_of`

Return the `TypeInfo` struct containing  for the type `T`.


```move
module 0x1::type_info {
    public fun type_of<T>(): type_info::TypeInfo
}
```


##### Implementation


```move
module 0x1::type_info {
    public native fun type_of<T>(): TypeInfo;
}
```


<a id="0x1_type_info_type_name"></a>

## Function `type_name`

Return the human readable string for the type, including the address, module name, and any type arguments.
Example: 0x1::coin::CoinStore&lt;0x1::aptos_coin::AptosCoin&gt;
Or: 0x1::table::Table&lt;0x1::string::String, 0x1::string::String&gt;


```move
module 0x1::type_info {
    public fun type_name<T>(): string::String
}
```


##### Implementation


```move
module 0x1::type_info {
    public native fun type_name<T>(): String;
}
```


<a id="0x1_type_info_chain_id_internal"></a>

## Function `chain_id_internal`



```move
module 0x1::type_info {
    fun chain_id_internal(): u8
}
```


##### Implementation


```move
module 0x1::type_info {
    native fun chain_id_internal(): u8;
}
```


<a id="0x1_type_info_size_of_val"></a>

## Function `size_of_val`

Return the BCS size, in bytes, of value at `val_ref`.

See the [BCS spec](https://github.com/diem/bcs)

See `test_size_of_val()` for an analysis of common types and
nesting patterns, as well as `test_size_of_val_vectors()` for an
analysis of vector size dynamism.


```move
module 0x1::type_info {
    public fun size_of_val<T>(val_ref: &T): u64
}
```


##### Implementation


```move
module 0x1::type_info {
    public fun size_of_val<T>(val_ref: &T): u64 {
        // Return vector length of vectorized BCS representation.
        vector::length(&bcs::to_bytes(val_ref))
    }
}
```


<a id="0x1_type_info_verify_type_of"></a>

## Function `verify_type_of`



```move
module 0x1::type_info {
    #[verify_only]
    fun verify_type_of()
}
```


##### Implementation


```move
module 0x1::type_info {
    fun verify_type_of() {
        let type_info = type_of<TypeInfo>();
        let account_address = account_address(&type_info);
        let module_name = module_name(&type_info);
        let struct_name = struct_name(&type_info);
        spec {
            assert account_address == @aptos_std;
            assert module_name == b"type_info";
            assert struct_name == b"TypeInfo";
        };
    }
}
```


<a id="0x1_type_info_verify_type_of_generic"></a>

## Function `verify_type_of_generic`



```move
module 0x1::type_info {
    #[verify_only]
    fun verify_type_of_generic<T>()
}
```


##### Implementation


```move
module 0x1::type_info {
    fun verify_type_of_generic<T>() {
        let type_info = type_of<T>();
        let account_address = account_address(&type_info);
        let module_name = module_name(&type_info);
        let struct_name = struct_name(&type_info);
        spec {
            assert account_address == type_of<T>().account_address;
            assert module_name == type_of<T>().module_name;
            assert struct_name == type_of<T>().struct_name;
        };
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_chain_id"></a>

### Function `chain_id`


```move
module 0x1::type_info {
    public fun chain_id(): u8
}
```



```move
module 0x1::type_info {
    aborts_if !features::spec_is_enabled(features::APTOS_STD_CHAIN_ID_NATIVES);
    ensures result == spec_chain_id_internal();
}
```


<a id="@Specification_1_type_of"></a>

### Function `type_of`


```move
module 0x1::type_info {
    public fun type_of<T>(): type_info::TypeInfo
}
```



<a id="@Specification_1_type_name"></a>

### Function `type_name`


```move
module 0x1::type_info {
    public fun type_name<T>(): string::String
}
```



<a id="@Specification_1_chain_id_internal"></a>

### Function `chain_id_internal`


```move
module 0x1::type_info {
    fun chain_id_internal(): u8
}
```



```move
module 0x1::type_info {
    pragma opaque;
    aborts_if false;
    ensures result == spec_chain_id_internal();
}
```



<a id="0x1_type_info_spec_chain_id_internal"></a>


```move
module 0x1::type_info {
    fun spec_chain_id_internal(): u8;
}
```



<a id="0x1_type_info_spec_size_of_val"></a>


```move
module 0x1::type_info {
    fun spec_size_of_val<T>(val_ref: T): u64 {
       len(std::bcs::serialize(val_ref))
    }
}
```


<a id="@Specification_1_size_of_val"></a>

### Function `size_of_val`


```move
module 0x1::type_info {
    public fun size_of_val<T>(val_ref: &T): u64
}
```



```move
module 0x1::type_info {
    aborts_if false;
    ensures result == spec_size_of_val<T>(val_ref);
}
```


<a id="@Specification_1_verify_type_of_generic"></a>

### Function `verify_type_of_generic`


```move
module 0x1::type_info {
    #[verify_only]
    fun verify_type_of_generic<T>()
}
```



```move
module 0x1::type_info {
    aborts_if !spec_is_struct<T>();
}
```



<a id="0x1_type_info_spec_is_struct"></a>


```move
module 0x1::type_info {
    native fun spec_is_struct<T>(): bool;
}
```
