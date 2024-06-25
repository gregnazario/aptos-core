
<a id="0x1_dispatchable_fungible_asset"></a>

# Module `0x1::dispatchable_fungible_asset`

This defines the fungible asset module that can issue fungible asset of any `Metadata` object. The
metadata object can be any object that equipped with `Metadata` resource.

The dispatchable_fungible_asset wraps the existing fungible_asset module and adds the ability for token issuer
to customize the logic for withdraw and deposit operations. For example:

&#45; Deflation token: a fixed percentage of token will be destructed upon transfer.
&#45; Transfer allowlist: token can only be transfered to addresses in the allow list.
&#45; Predicated transfer: transfer can only happen when some certain predicate has been met.
&#45; Loyalty token: a fixed loyalty will be paid to a designated address when a fungible asset transfer happens

The api listed here intended to be an in&#45;place replacement for defi applications that uses fungible_asset api directly
and is safe for non&#45;dispatchable (aka vanilla) fungible assets as well.

See AIP&#45;73 for further discussion


-  [Resource `TransferRefStore`](#0x1_dispatchable_fungible_asset_TransferRefStore)
-  [Constants](#@Constants_0)
-  [Function `register_dispatch_functions`](#0x1_dispatchable_fungible_asset_register_dispatch_functions)
-  [Function `withdraw`](#0x1_dispatchable_fungible_asset_withdraw)
-  [Function `deposit`](#0x1_dispatchable_fungible_asset_deposit)
-  [Function `transfer`](#0x1_dispatchable_fungible_asset_transfer)
-  [Function `transfer_assert_minimum_deposit`](#0x1_dispatchable_fungible_asset_transfer_assert_minimum_deposit)
-  [Function `derived_balance`](#0x1_dispatchable_fungible_asset_derived_balance)
-  [Function `borrow_transfer_ref`](#0x1_dispatchable_fungible_asset_borrow_transfer_ref)
-  [Function `dispatchable_withdraw`](#0x1_dispatchable_fungible_asset_dispatchable_withdraw)
-  [Function `dispatchable_deposit`](#0x1_dispatchable_fungible_asset_dispatchable_deposit)
-  [Function `dispatchable_derived_balance`](#0x1_dispatchable_fungible_asset_dispatchable_derived_balance)
-  [Specification](#@Specification_1)
    -  [Function `dispatchable_withdraw`](#@Specification_1_dispatchable_withdraw)
    -  [Function `dispatchable_deposit`](#@Specification_1_dispatchable_deposit)
    -  [Function `dispatchable_derived_balance`](#@Specification_1_dispatchable_derived_balance)


```move
module 0x1::dispatchable_fungible_asset {
    use 0x1::error;
    use 0x1::features;
    use 0x1::function_info;
    use 0x1::fungible_asset;
    use 0x1::object;
    use 0x1::option;
}
```


<a id="0x1_dispatchable_fungible_asset_TransferRefStore"></a>

## Resource `TransferRefStore`



```move
module 0x1::dispatchable_fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct TransferRefStore has key
}
```


##### Fields


<dl>
<dt>
`transfer_ref: fungible_asset::TransferRef`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_dispatchable_fungible_asset_ENOT_ACTIVATED"></a>

Feature is not activated yet on the network.


```move
module 0x1::dispatchable_fungible_asset {
    const ENOT_ACTIVATED: u64 = 3;
}
```


<a id="0x1_dispatchable_fungible_asset_EAMOUNT_MISMATCH"></a>

Recipient is not getting the guaranteed value;


```move
module 0x1::dispatchable_fungible_asset {
    const EAMOUNT_MISMATCH: u64 = 2;
}
```


<a id="0x1_dispatchable_fungible_asset_ENOT_LOADED"></a>

Dispatch target is not loaded.


```move
module 0x1::dispatchable_fungible_asset {
    const ENOT_LOADED: u64 = 4;
}
```


<a id="0x1_dispatchable_fungible_asset_ESTORE_NOT_FOUND"></a>

TransferRefStore doesn&apos;t exist on the fungible asset type.


```move
module 0x1::dispatchable_fungible_asset {
    const ESTORE_NOT_FOUND: u64 = 1;
}
```


<a id="0x1_dispatchable_fungible_asset_register_dispatch_functions"></a>

## Function `register_dispatch_functions`



```move
module 0x1::dispatchable_fungible_asset {
    public fun register_dispatch_functions(constructor_ref: &object::ConstructorRef, withdraw_function: option::Option<function_info::FunctionInfo>, deposit_function: option::Option<function_info::FunctionInfo>, derived_balance_function: option::Option<function_info::FunctionInfo>)
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public fun register_dispatch_functions(
        constructor_ref: &ConstructorRef,
        withdraw_function: Option<FunctionInfo>,
        deposit_function: Option<FunctionInfo>,
        derived_balance_function: Option<FunctionInfo>,
    ) {
        fungible_asset::register_dispatch_functions(
            constructor_ref,
            withdraw_function,
            deposit_function,
            derived_balance_function,
        );
        let store_obj = &object::generate_signer(constructor_ref);
        move_to<TransferRefStore>(
            store_obj,
            TransferRefStore {
                transfer_ref: fungible_asset::generate_transfer_ref(constructor_ref),
            }
        );
    }
}
```


<a id="0x1_dispatchable_fungible_asset_withdraw"></a>

## Function `withdraw`

Withdraw `amount` of the fungible asset from `store` by the owner.

The semantics of deposit will be governed by the function specified in DispatchFunctionStore.


```move
module 0x1::dispatchable_fungible_asset {
    public fun withdraw<T: key>(owner: &signer, store: object::Object<T>, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public fun withdraw<T: key>(
        owner: &signer,
        store: Object<T>,
        amount: u64,
    ): FungibleAsset acquires TransferRefStore {
        fungible_asset::withdraw_sanity_check(owner, store, false);
        let func_opt = fungible_asset::withdraw_dispatch_function(store);
        if (option::is_some(&func_opt)) {
            assert!(
                features::dispatchable_fungible_asset_enabled(),
                error::aborted(ENOT_ACTIVATED)
            );
            let start_balance = fungible_asset::balance(store);
            let func = option::borrow(&func_opt);
            function_info::load_module_from_function(func);
            let fa = dispatchable_withdraw(
                store,
                amount,
                borrow_transfer_ref(store),
                func,
            );
            let end_balance = fungible_asset::balance(store);
            assert!(amount <= start_balance - end_balance, error::aborted(EAMOUNT_MISMATCH));
            fa
        } else {
            fungible_asset::withdraw_internal(object::object_address(&store), amount)
        }
    }
}
```


<a id="0x1_dispatchable_fungible_asset_deposit"></a>

## Function `deposit`

Deposit `amount` of the fungible asset to `store`.

The semantics of deposit will be governed by the function specified in DispatchFunctionStore.


```move
module 0x1::dispatchable_fungible_asset {
    public fun deposit<T: key>(store: object::Object<T>, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public fun deposit<T: key>(store: Object<T>, fa: FungibleAsset) acquires TransferRefStore {
        fungible_asset::deposit_sanity_check(store, false);
        let func_opt = fungible_asset::deposit_dispatch_function(store);
        if (option::is_some(&func_opt)) {
            assert!(
                features::dispatchable_fungible_asset_enabled(),
                error::aborted(ENOT_ACTIVATED)
            );
            let func = option::borrow(&func_opt);
            function_info::load_module_from_function(func);
            dispatchable_deposit(
                store,
                fa,
                borrow_transfer_ref(store),
                func
            )
        } else {
            fungible_asset::deposit_internal(object::object_address(&store), fa)
        }
    }
}
```


<a id="0x1_dispatchable_fungible_asset_transfer"></a>

## Function `transfer`

Transfer an `amount` of fungible asset from `from_store`, which should be owned by `sender`, to `receiver`.
Note: it does not move the underlying object.


```move
module 0x1::dispatchable_fungible_asset {
    public entry fun transfer<T: key>(sender: &signer, from: object::Object<T>, to: object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public entry fun transfer<T: key>(
        sender: &signer,
        from: Object<T>,
        to: Object<T>,
        amount: u64,
    ) acquires TransferRefStore {
        let fa = withdraw(sender, from, amount);
        deposit(to, fa);
    }
}
```


<a id="0x1_dispatchable_fungible_asset_transfer_assert_minimum_deposit"></a>

## Function `transfer_assert_minimum_deposit`

Transfer an `amount` of fungible asset from `from_store`, which should be owned by `sender`, to `receiver`.
The recipient is guranteed to receive asset greater than the expected amount.
Note: it does not move the underlying object.


```move
module 0x1::dispatchable_fungible_asset {
    public entry fun transfer_assert_minimum_deposit<T: key>(sender: &signer, from: object::Object<T>, to: object::Object<T>, amount: u64, expected: u64)
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public entry fun transfer_assert_minimum_deposit<T: key>(
        sender: &signer,
        from: Object<T>,
        to: Object<T>,
        amount: u64,
        expected: u64
    ) acquires TransferRefStore {
        let start = fungible_asset::balance(to);
        let fa = withdraw(sender, from, amount);
        deposit(to, fa);
        let end = fungible_asset::balance(to);
        assert!(end - start >= expected, error::aborted(EAMOUNT_MISMATCH));
    }
}
```


<a id="0x1_dispatchable_fungible_asset_derived_balance"></a>

## Function `derived_balance`

Get the derived value of store using the overloaded hook.

The semantics of value will be governed by the function specified in DispatchFunctionStore.


```move
module 0x1::dispatchable_fungible_asset {
    #[view]
    public fun derived_balance<T: key>(store: object::Object<T>): u64
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    public fun derived_balance<T: key>(store: Object<T>): u64 {
        let func_opt = fungible_asset::derived_balance_dispatch_function(store);
        if (option::is_some(&func_opt)) {
            assert!(
                features::dispatchable_fungible_asset_enabled(),
                error::aborted(ENOT_ACTIVATED)
            );
            let func = option::borrow(&func_opt);
            function_info::load_module_from_function(func);
            dispatchable_derived_balance(store, func)
        } else {
            fungible_asset::balance(store)
        }
    }
}
```


<a id="0x1_dispatchable_fungible_asset_borrow_transfer_ref"></a>

## Function `borrow_transfer_ref`



```move
module 0x1::dispatchable_fungible_asset {
    fun borrow_transfer_ref<T: key>(metadata: object::Object<T>): &fungible_asset::TransferRef
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    inline fun borrow_transfer_ref<T: key>(metadata: Object<T>): &TransferRef acquires TransferRefStore {
        let metadata_addr = object::object_address(
            &fungible_asset::store_metadata(metadata)
        );
        assert!(
            exists<TransferRefStore>(metadata_addr),
            error::not_found(ESTORE_NOT_FOUND)
        );
        &borrow_global<TransferRefStore>(metadata_addr).transfer_ref
    }
}
```


<a id="0x1_dispatchable_fungible_asset_dispatchable_withdraw"></a>

## Function `dispatchable_withdraw`



```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_withdraw<T: key>(store: object::Object<T>, amount: u64, transfer_ref: &fungible_asset::TransferRef, function: &function_info::FunctionInfo): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    native fun dispatchable_withdraw<T: key>(
        store: Object<T>,
        amount: u64,
        transfer_ref: &TransferRef,
        function: &FunctionInfo,
    ): FungibleAsset;
}
```


<a id="0x1_dispatchable_fungible_asset_dispatchable_deposit"></a>

## Function `dispatchable_deposit`



```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_deposit<T: key>(store: object::Object<T>, fa: fungible_asset::FungibleAsset, transfer_ref: &fungible_asset::TransferRef, function: &function_info::FunctionInfo)
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    native fun dispatchable_deposit<T: key>(
        store: Object<T>,
        fa: FungibleAsset,
        transfer_ref: &TransferRef,
        function: &FunctionInfo,
    );
}
```


<a id="0x1_dispatchable_fungible_asset_dispatchable_derived_balance"></a>

## Function `dispatchable_derived_balance`



```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_derived_balance<T: key>(store: object::Object<T>, function: &function_info::FunctionInfo): u64
}
```


##### Implementation


```move
module 0x1::dispatchable_fungible_asset {
    native fun dispatchable_derived_balance<T: key>(
        store: Object<T>,
        function: &FunctionInfo,
    ): u64;
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x1::dispatchable_fungible_asset {
    pragma verify = false;
}
```


<a id="@Specification_1_dispatchable_withdraw"></a>

### Function `dispatchable_withdraw`


```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_withdraw<T: key>(store: object::Object<T>, amount: u64, transfer_ref: &fungible_asset::TransferRef, function: &function_info::FunctionInfo): fungible_asset::FungibleAsset
}
```



```move
module 0x1::dispatchable_fungible_asset {
    pragma opaque;
}
```


<a id="@Specification_1_dispatchable_deposit"></a>

### Function `dispatchable_deposit`


```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_deposit<T: key>(store: object::Object<T>, fa: fungible_asset::FungibleAsset, transfer_ref: &fungible_asset::TransferRef, function: &function_info::FunctionInfo)
}
```



```move
module 0x1::dispatchable_fungible_asset {
    pragma opaque;
}
```


<a id="@Specification_1_dispatchable_derived_balance"></a>

### Function `dispatchable_derived_balance`


```move
module 0x1::dispatchable_fungible_asset {
    fun dispatchable_derived_balance<T: key>(store: object::Object<T>, function: &function_info::FunctionInfo): u64
}
```



```move
module 0x1::dispatchable_fungible_asset {
    pragma opaque;
}
```
