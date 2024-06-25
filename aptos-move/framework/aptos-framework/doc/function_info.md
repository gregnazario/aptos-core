
<a id="0x1_function_info"></a>

# Module `0x1::function_info`

The `function_info` module defines the `FunctionInfo` type which simulates a function pointer.


-  [Struct `FunctionInfo`](#0x1_function_info_FunctionInfo)
-  [Constants](#@Constants_0)
-  [Function `new_function_info`](#0x1_function_info_new_function_info)
-  [Function `new_function_info_from_address`](#0x1_function_info_new_function_info_from_address)
-  [Function `check_dispatch_type_compatibility`](#0x1_function_info_check_dispatch_type_compatibility)
-  [Function `load_module_from_function`](#0x1_function_info_load_module_from_function)
-  [Function `check_dispatch_type_compatibility_impl`](#0x1_function_info_check_dispatch_type_compatibility_impl)
-  [Function `is_identifier`](#0x1_function_info_is_identifier)
-  [Function `load_function_impl`](#0x1_function_info_load_function_impl)
-  [Specification](#@Specification_1)
    -  [Function `check_dispatch_type_compatibility_impl`](#@Specification_1_check_dispatch_type_compatibility_impl)
    -  [Function `load_function_impl`](#@Specification_1_load_function_impl)


```move
module 0x1::function_info {
    use 0x1::error;
    use 0x1::features;
    use 0x1::signer;
    use 0x1::string;
}
```


<a id="0x1_function_info_FunctionInfo"></a>

## Struct `FunctionInfo`

A `String` holds a sequence of bytes which is guaranteed to be in utf8 format.


```move
module 0x1::function_info {
    struct FunctionInfo has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`module_address: address`
</dt>
<dd>

</dd>
<dt>
`module_name: string::String`
</dt>
<dd>

</dd>
<dt>
`function_name: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_function_info_EINVALID_FUNCTION"></a>

Function specified in the FunctionInfo doesn&apos;t exist on chain.


```move
module 0x1::function_info {
    const EINVALID_FUNCTION: u64 = 2;
}
```


<a id="0x1_function_info_EINVALID_IDENTIFIER"></a>

String is not a valid Move identifier


```move
module 0x1::function_info {
    const EINVALID_IDENTIFIER: u64 = 1;
}
```


<a id="0x1_function_info_ENOT_ACTIVATED"></a>

Feature hasn&apos;t been activated yet.


```move
module 0x1::function_info {
    const ENOT_ACTIVATED: u64 = 3;
}
```


<a id="0x1_function_info_new_function_info"></a>

## Function `new_function_info`

Creates a new function info from names.


```move
module 0x1::function_info {
    public fun new_function_info(module_signer: &signer, module_name: string::String, function_name: string::String): function_info::FunctionInfo
}
```


##### Implementation


```move
module 0x1::function_info {
    public fun new_function_info(
        module_signer: &signer,
        module_name: String,
        function_name: String,
    ): FunctionInfo {
        new_function_info_from_address(
            signer::address_of(module_signer),
            module_name,
            function_name,
        )
    }
}
```


<a id="0x1_function_info_new_function_info_from_address"></a>

## Function `new_function_info_from_address`



```move
module 0x1::function_info {
    public(friend) fun new_function_info_from_address(module_address: address, module_name: string::String, function_name: string::String): function_info::FunctionInfo
}
```


##### Implementation


```move
module 0x1::function_info {
    public(friend) fun new_function_info_from_address(
        module_address: address,
        module_name: String,
        function_name: String,
    ): FunctionInfo {
        assert!(
            is_identifier(string::bytes(&module_name)),
            EINVALID_IDENTIFIER
        );
        assert!(
            is_identifier(string::bytes(&function_name)),
            EINVALID_IDENTIFIER
        );
        FunctionInfo {
            module_address,
            module_name,
            function_name,
        }
    }
}
```


<a id="0x1_function_info_check_dispatch_type_compatibility"></a>

## Function `check_dispatch_type_compatibility`

Check if the dispatch target function meets the type requirements of the disptach entry point.

framework_function is the dispatch native function defined in the aptos_framework.
dispatch_target is the function passed in by the user.

dispatch_target should have the same signature (same argument type, same generics constraint) except
that the framework_function will have a `&FunctionInfo` in the last argument that will instruct the VM which
function to jump to.

dispatch_target also needs to be public so the type signature will remain unchanged.


```move
module 0x1::function_info {
    public(friend) fun check_dispatch_type_compatibility(framework_function: &function_info::FunctionInfo, dispatch_target: &function_info::FunctionInfo): bool
}
```


##### Implementation


```move
module 0x1::function_info {
    public(friend) fun check_dispatch_type_compatibility(
        framework_function: &FunctionInfo,
        dispatch_target: &FunctionInfo,
    ): bool {
        assert!(
            features::dispatchable_fungible_asset_enabled(),
            error::aborted(ENOT_ACTIVATED)
        );
        load_function_impl(dispatch_target);
        check_dispatch_type_compatibility_impl(framework_function, dispatch_target)
    }
}
```


<a id="0x1_function_info_load_module_from_function"></a>

## Function `load_module_from_function`

Load up a function into VM&apos;s loader and charge for its dependencies

It is &#42;&#42;critical&#42;&#42; to make sure that this function is invoked before `check_dispatch_type_compatibility`
or performing any other dispatching logic to ensure:
1. We properly charge gas for the function to dispatch.
2. The function is loaded in the cache so that we can perform further type checking/dispatching logic.

Calling `check_dispatch_type_compatibility_impl` or dispatch without loading up the module would yield an error
if such module isn&apos;t accessed previously in the transaction.


```move
module 0x1::function_info {
    public(friend) fun load_module_from_function(f: &function_info::FunctionInfo)
}
```


##### Implementation


```move
module 0x1::function_info {
    public(friend) fun load_module_from_function(f: &FunctionInfo) {
        load_function_impl(f)
    }
}
```


<a id="0x1_function_info_check_dispatch_type_compatibility_impl"></a>

## Function `check_dispatch_type_compatibility_impl`



```move
module 0x1::function_info {
    fun check_dispatch_type_compatibility_impl(lhs: &function_info::FunctionInfo, r: &function_info::FunctionInfo): bool
}
```


##### Implementation


```move
module 0x1::function_info {
    native fun check_dispatch_type_compatibility_impl(lhs: &FunctionInfo, r: &FunctionInfo): bool;
}
```


<a id="0x1_function_info_is_identifier"></a>

## Function `is_identifier`



```move
module 0x1::function_info {
    fun is_identifier(s: &vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::function_info {
    native fun is_identifier(s: &vector<u8>): bool;
}
```


<a id="0x1_function_info_load_function_impl"></a>

## Function `load_function_impl`



```move
module 0x1::function_info {
    fun load_function_impl(f: &function_info::FunctionInfo)
}
```


##### Implementation


```move
module 0x1::function_info {
    native fun load_function_impl(f: &FunctionInfo);
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::function_info {
    pragma verify = false;
}
```


<a id="@Specification_1_check_dispatch_type_compatibility_impl"></a>

### Function `check_dispatch_type_compatibility_impl`


```move
module 0x1::function_info {
    fun check_dispatch_type_compatibility_impl(lhs: &function_info::FunctionInfo, r: &function_info::FunctionInfo): bool
}
```



```move
module 0x1::function_info {
    pragma opaque;
}
```


<a id="@Specification_1_load_function_impl"></a>

### Function `load_function_impl`


```move
module 0x1::function_info {
    fun load_function_impl(f: &function_info::FunctionInfo)
}
```



```move
module 0x1::function_info {
    pragma opaque;
}
```
