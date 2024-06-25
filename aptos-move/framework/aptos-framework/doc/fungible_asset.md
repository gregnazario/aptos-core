
<a id="0x1_fungible_asset"></a>

# Module `0x1::fungible_asset`

This defines the fungible asset module that can issue fungible asset of any `Metadata` object. The
metadata object can be any object that equipped with `Metadata` resource.


-  [Resource `Supply`](#0x1_fungible_asset_Supply)
-  [Resource `ConcurrentSupply`](#0x1_fungible_asset_ConcurrentSupply)
-  [Resource `Metadata`](#0x1_fungible_asset_Metadata)
-  [Resource `Untransferable`](#0x1_fungible_asset_Untransferable)
-  [Resource `FungibleStore`](#0x1_fungible_asset_FungibleStore)
-  [Resource `DispatchFunctionStore`](#0x1_fungible_asset_DispatchFunctionStore)
-  [Resource `ConcurrentFungibleBalance`](#0x1_fungible_asset_ConcurrentFungibleBalance)
-  [Struct `FungibleAsset`](#0x1_fungible_asset_FungibleAsset)
-  [Struct `MintRef`](#0x1_fungible_asset_MintRef)
-  [Struct `TransferRef`](#0x1_fungible_asset_TransferRef)
-  [Struct `BurnRef`](#0x1_fungible_asset_BurnRef)
-  [Struct `Deposit`](#0x1_fungible_asset_Deposit)
-  [Struct `Withdraw`](#0x1_fungible_asset_Withdraw)
-  [Struct `Frozen`](#0x1_fungible_asset_Frozen)
-  [Resource `FungibleAssetEvents`](#0x1_fungible_asset_FungibleAssetEvents)
-  [Struct `DepositEvent`](#0x1_fungible_asset_DepositEvent)
-  [Struct `WithdrawEvent`](#0x1_fungible_asset_WithdrawEvent)
-  [Struct `FrozenEvent`](#0x1_fungible_asset_FrozenEvent)
-  [Constants](#@Constants_0)
-  [Function `default_to_concurrent_fungible_supply`](#0x1_fungible_asset_default_to_concurrent_fungible_supply)
-  [Function `allow_upgrade_to_concurrent_fungible_balance`](#0x1_fungible_asset_allow_upgrade_to_concurrent_fungible_balance)
-  [Function `default_to_concurrent_fungible_balance`](#0x1_fungible_asset_default_to_concurrent_fungible_balance)
-  [Function `add_fungibility`](#0x1_fungible_asset_add_fungibility)
-  [Function `set_untransferable`](#0x1_fungible_asset_set_untransferable)
-  [Function `is_untransferable`](#0x1_fungible_asset_is_untransferable)
-  [Function `register_dispatch_functions`](#0x1_fungible_asset_register_dispatch_functions)
-  [Function `generate_mint_ref`](#0x1_fungible_asset_generate_mint_ref)
-  [Function `generate_burn_ref`](#0x1_fungible_asset_generate_burn_ref)
-  [Function `generate_transfer_ref`](#0x1_fungible_asset_generate_transfer_ref)
-  [Function `supply`](#0x1_fungible_asset_supply)
-  [Function `maximum`](#0x1_fungible_asset_maximum)
-  [Function `name`](#0x1_fungible_asset_name)
-  [Function `symbol`](#0x1_fungible_asset_symbol)
-  [Function `decimals`](#0x1_fungible_asset_decimals)
-  [Function `icon_uri`](#0x1_fungible_asset_icon_uri)
-  [Function `project_uri`](#0x1_fungible_asset_project_uri)
-  [Function `store_exists`](#0x1_fungible_asset_store_exists)
-  [Function `store_exists_inline`](#0x1_fungible_asset_store_exists_inline)
-  [Function `concurrent_fungible_balance_exists_inline`](#0x1_fungible_asset_concurrent_fungible_balance_exists_inline)
-  [Function `metadata_from_asset`](#0x1_fungible_asset_metadata_from_asset)
-  [Function `store_metadata`](#0x1_fungible_asset_store_metadata)
-  [Function `amount`](#0x1_fungible_asset_amount)
-  [Function `balance`](#0x1_fungible_asset_balance)
-  [Function `is_balance_at_least`](#0x1_fungible_asset_is_balance_at_least)
-  [Function `is_address_balance_at_least`](#0x1_fungible_asset_is_address_balance_at_least)
-  [Function `is_frozen`](#0x1_fungible_asset_is_frozen)
-  [Function `is_store_dispatchable`](#0x1_fungible_asset_is_store_dispatchable)
-  [Function `deposit_dispatch_function`](#0x1_fungible_asset_deposit_dispatch_function)
-  [Function `has_deposit_dispatch_function`](#0x1_fungible_asset_has_deposit_dispatch_function)
-  [Function `withdraw_dispatch_function`](#0x1_fungible_asset_withdraw_dispatch_function)
-  [Function `has_withdraw_dispatch_function`](#0x1_fungible_asset_has_withdraw_dispatch_function)
-  [Function `derived_balance_dispatch_function`](#0x1_fungible_asset_derived_balance_dispatch_function)
-  [Function `asset_metadata`](#0x1_fungible_asset_asset_metadata)
-  [Function `mint_ref_metadata`](#0x1_fungible_asset_mint_ref_metadata)
-  [Function `transfer_ref_metadata`](#0x1_fungible_asset_transfer_ref_metadata)
-  [Function `burn_ref_metadata`](#0x1_fungible_asset_burn_ref_metadata)
-  [Function `transfer`](#0x1_fungible_asset_transfer)
-  [Function `create_store`](#0x1_fungible_asset_create_store)
-  [Function `remove_store`](#0x1_fungible_asset_remove_store)
-  [Function `withdraw`](#0x1_fungible_asset_withdraw)
-  [Function `withdraw_sanity_check`](#0x1_fungible_asset_withdraw_sanity_check)
-  [Function `deposit_sanity_check`](#0x1_fungible_asset_deposit_sanity_check)
-  [Function `deposit`](#0x1_fungible_asset_deposit)
-  [Function `mint`](#0x1_fungible_asset_mint)
-  [Function `mint_internal`](#0x1_fungible_asset_mint_internal)
-  [Function `mint_to`](#0x1_fungible_asset_mint_to)
-  [Function `set_frozen_flag`](#0x1_fungible_asset_set_frozen_flag)
-  [Function `set_frozen_flag_internal`](#0x1_fungible_asset_set_frozen_flag_internal)
-  [Function `burn`](#0x1_fungible_asset_burn)
-  [Function `burn_internal`](#0x1_fungible_asset_burn_internal)
-  [Function `burn_from`](#0x1_fungible_asset_burn_from)
-  [Function `address_burn_from`](#0x1_fungible_asset_address_burn_from)
-  [Function `withdraw_with_ref`](#0x1_fungible_asset_withdraw_with_ref)
-  [Function `deposit_with_ref`](#0x1_fungible_asset_deposit_with_ref)
-  [Function `transfer_with_ref`](#0x1_fungible_asset_transfer_with_ref)
-  [Function `zero`](#0x1_fungible_asset_zero)
-  [Function `extract`](#0x1_fungible_asset_extract)
-  [Function `merge`](#0x1_fungible_asset_merge)
-  [Function `destroy_zero`](#0x1_fungible_asset_destroy_zero)
-  [Function `deposit_internal`](#0x1_fungible_asset_deposit_internal)
-  [Function `withdraw_internal`](#0x1_fungible_asset_withdraw_internal)
-  [Function `increase_supply`](#0x1_fungible_asset_increase_supply)
-  [Function `decrease_supply`](#0x1_fungible_asset_decrease_supply)
-  [Function `borrow_fungible_metadata`](#0x1_fungible_asset_borrow_fungible_metadata)
-  [Function `borrow_fungible_metadata_mut`](#0x1_fungible_asset_borrow_fungible_metadata_mut)
-  [Function `borrow_store_resource`](#0x1_fungible_asset_borrow_store_resource)
-  [Function `upgrade_to_concurrent`](#0x1_fungible_asset_upgrade_to_concurrent)
-  [Function `upgrade_store_to_concurrent`](#0x1_fungible_asset_upgrade_store_to_concurrent)
-  [Function `ensure_store_upgraded_to_concurrent_internal`](#0x1_fungible_asset_ensure_store_upgraded_to_concurrent_internal)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)


```move
module 0x1::fungible_asset {
    use 0x1::aggregator_v2;
    use 0x1::create_signer;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::function_info;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
}
```


<a id="0x1_fungible_asset_Supply"></a>

## Resource `Supply`



```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Supply has key
}
```


##### Fields


<dl>
<dt>
`current: u128`
</dt>
<dd>

</dd>
<dt>
`maximum: option::Option<u128>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_ConcurrentSupply"></a>

## Resource `ConcurrentSupply`



```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct ConcurrentSupply has key
}
```


##### Fields


<dl>
<dt>
`current: aggregator_v2::Aggregator<u128>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_Metadata"></a>

## Resource `Metadata`

Metadata of a Fungible asset


```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Metadata has key
}
```


##### Fields


<dl>
<dt>
`name: string::String`
</dt>
<dd>
 Name of the fungible metadata, i.e., &quot;USDT&quot;.
</dd>
<dt>
`symbol: string::String`
</dt>
<dd>
 Symbol of the fungible metadata, usually a shorter version of the name.
 For example, Singapore Dollar is SGD.
</dd>
<dt>
`decimals: u8`
</dt>
<dd>
 Number of decimals used for display purposes.
 For example, if `decimals` equals `2`, a balance of `505` coins should
 be displayed to a user as `5.05` (`505 / 10 ** 2`).
</dd>
<dt>
`icon_uri: string::String`
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to an image that can be used as the icon for this fungible
 asset.
</dd>
<dt>
`project_uri: string::String`
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the website for the fungible asset.
</dd>
</dl>


<a id="0x1_fungible_asset_Untransferable"></a>

## Resource `Untransferable`

Defines a `FungibleAsset`, such that all `FungibleStore`s stores are untransferable at
the object layer.


```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Untransferable has key
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_FungibleStore"></a>

## Resource `FungibleStore`

The store object that holds fungible assets of a specific type associated with an account.


```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct FungibleStore has key
}
```


##### Fields


<dl>
<dt>
`metadata: object::Object<fungible_asset::Metadata>`
</dt>
<dd>
 The address of the base metadata object.
</dd>
<dt>
`balance: u64`
</dt>
<dd>
 The balance of the fungible metadata.
</dd>
<dt>
`frozen: bool`
</dt>
<dd>
 If true, owner transfer is disabled that only `TransferRef` can move in/out from this store.
</dd>
</dl>


<a id="0x1_fungible_asset_DispatchFunctionStore"></a>

## Resource `DispatchFunctionStore`



```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct DispatchFunctionStore has key
}
```


##### Fields


<dl>
<dt>
`withdraw_function: option::Option<function_info::FunctionInfo>`
</dt>
<dd>

</dd>
<dt>
`deposit_function: option::Option<function_info::FunctionInfo>`
</dt>
<dd>

</dd>
<dt>
`derived_balance_function: option::Option<function_info::FunctionInfo>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_ConcurrentFungibleBalance"></a>

## Resource `ConcurrentFungibleBalance`

The store object that holds concurrent fungible asset balance.


```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct ConcurrentFungibleBalance has key
}
```


##### Fields


<dl>
<dt>
`balance: aggregator_v2::Aggregator<u64>`
</dt>
<dd>
 The balance of the fungible metadata.
</dd>
</dl>


<a id="0x1_fungible_asset_FungibleAsset"></a>

## Struct `FungibleAsset`

FungibleAsset can be passed into function for type safety and to guarantee a specific amount.
FungibleAsset is ephemeral and cannot be stored directly. It must be deposited back into a store.


```move
module 0x1::fungible_asset {
    struct FungibleAsset
}
```


##### Fields


<dl>
<dt>
`metadata: object::Object<fungible_asset::Metadata>`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_MintRef"></a>

## Struct `MintRef`

MintRef can be used to mint the fungible asset into an account&apos;s store.


```move
module 0x1::fungible_asset {
    struct MintRef has drop, store
}
```


##### Fields


<dl>
<dt>
`metadata: object::Object<fungible_asset::Metadata>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_TransferRef"></a>

## Struct `TransferRef`

TransferRef can be used to allow or disallow the owner of fungible assets from transferring the asset
and allow the holder of TransferRef to transfer fungible assets from any account.


```move
module 0x1::fungible_asset {
    struct TransferRef has drop, store
}
```


##### Fields


<dl>
<dt>
`metadata: object::Object<fungible_asset::Metadata>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_BurnRef"></a>

## Struct `BurnRef`

BurnRef can be used to burn fungible assets from a given holder account.


```move
module 0x1::fungible_asset {
    struct BurnRef has drop, store
}
```


##### Fields


<dl>
<dt>
`metadata: object::Object<fungible_asset::Metadata>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_Deposit"></a>

## Struct `Deposit`

Emitted when fungible assets are deposited into a store.


```move
module 0x1::fungible_asset {
    #[event]
    struct Deposit has drop, store
}
```


##### Fields


<dl>
<dt>
`store: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_Withdraw"></a>

## Struct `Withdraw`

Emitted when fungible assets are withdrawn from a store.


```move
module 0x1::fungible_asset {
    #[event]
    struct Withdraw has drop, store
}
```


##### Fields


<dl>
<dt>
`store: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_Frozen"></a>

## Struct `Frozen`

Emitted when a store&apos;s frozen status is updated.


```move
module 0x1::fungible_asset {
    #[event]
    struct Frozen has drop, store
}
```


##### Fields


<dl>
<dt>
`store: address`
</dt>
<dd>

</dd>
<dt>
`frozen: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_FungibleAssetEvents"></a>

## Resource `FungibleAssetEvents`



```move
module 0x1::fungible_asset {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    #[deprecated]
    struct FungibleAssetEvents has key
}
```


##### Fields


<dl>
<dt>
`deposit_events: event::EventHandle<fungible_asset::DepositEvent>`
</dt>
<dd>

</dd>
<dt>
`withdraw_events: event::EventHandle<fungible_asset::WithdrawEvent>`
</dt>
<dd>

</dd>
<dt>
`frozen_events: event::EventHandle<fungible_asset::FrozenEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_DepositEvent"></a>

## Struct `DepositEvent`



```move
module 0x1::fungible_asset {
    #[deprecated]
    struct DepositEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_WithdrawEvent"></a>

## Struct `WithdrawEvent`



```move
module 0x1::fungible_asset {
    #[deprecated]
    struct WithdrawEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_FrozenEvent"></a>

## Struct `FrozenEvent`



```move
module 0x1::fungible_asset {
    #[deprecated]
    struct FrozenEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`frozen: bool`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_fungible_asset_MAX_U128"></a>

Maximum possible coin supply.


```move
module 0x1::fungible_asset {
    const MAX_U128: u128 = 340282366920938463463374607431768211455;
}
```


<a id="0x1_fungible_asset_EALREADY_REGISTERED"></a>

Trying to re&#45;register dispatch hook on a fungible asset.


```move
module 0x1::fungible_asset {
    const EALREADY_REGISTERED: u64 = 29;
}
```


<a id="0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO"></a>

Amount cannot be zero.


```move
module 0x1::fungible_asset {
    const EAMOUNT_CANNOT_BE_ZERO: u64 = 1;
}
```


<a id="0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO"></a>

Cannot destroy non&#45;empty fungible assets.


```move
module 0x1::fungible_asset {
    const EAMOUNT_IS_NOT_ZERO: u64 = 12;
}
```


<a id="0x1_fungible_asset_EAPT_NOT_DISPATCHABLE"></a>

Cannot register dispatch hook for APT.


```move
module 0x1::fungible_asset {
    const EAPT_NOT_DISPATCHABLE: u64 = 31;
}
```


<a id="0x1_fungible_asset_EBALANCE_IS_NOT_ZERO"></a>

Cannot destroy fungible stores with a non&#45;zero balance.


```move
module 0x1::fungible_asset {
    const EBALANCE_IS_NOT_ZERO: u64 = 14;
}
```


<a id="0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

Burn ref and fungible asset do not match.


```move
module 0x1::fungible_asset {
    const EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH: u64 = 13;
}
```


<a id="0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH"></a>

Burn ref and store do not match.


```move
module 0x1::fungible_asset {
    const EBURN_REF_AND_STORE_MISMATCH: u64 = 10;
}
```


<a id="0x1_fungible_asset_ECONCURRENT_BALANCE_NOT_ENABLED"></a>

Flag for Concurrent Supply not enabled


```move
module 0x1::fungible_asset {
    const ECONCURRENT_BALANCE_NOT_ENABLED: u64 = 32;
}
```


<a id="0x1_fungible_asset_ECONCURRENT_SUPPLY_NOT_ENABLED"></a>

Flag for Concurrent Supply not enabled


```move
module 0x1::fungible_asset {
    const ECONCURRENT_SUPPLY_NOT_ENABLED: u64 = 22;
}
```


<a id="0x1_fungible_asset_EDECIMALS_TOO_LARGE"></a>

Decimals is over the maximum of 32


```move
module 0x1::fungible_asset {
    const EDECIMALS_TOO_LARGE: u64 = 17;
}
```


<a id="0x1_fungible_asset_EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided deposit function type doesn&apos;t meet the signature requirement.


```move
module 0x1::fungible_asset {
    const EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH: u64 = 26;
}
```


<a id="0x1_fungible_asset_EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided derived_balance function type doesn&apos;t meet the signature requirement.


```move
module 0x1::fungible_asset {
    const EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH: u64 = 27;
}
```


<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH"></a>

Fungible asset and store do not match.


```move
module 0x1::fungible_asset {
    const EFUNGIBLE_ASSET_AND_STORE_MISMATCH: u64 = 11;
}
```


<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH"></a>

Fungible asset do not match when merging.


```move
module 0x1::fungible_asset {
    const EFUNGIBLE_ASSET_MISMATCH: u64 = 6;
}
```


<a id="0x1_fungible_asset_EFUNGIBLE_METADATA_EXISTENCE"></a>

Fungible metadata does not exist on this account.


```move
module 0x1::fungible_asset {
    const EFUNGIBLE_METADATA_EXISTENCE: u64 = 30;
}
```


<a id="0x1_fungible_asset_EFUNGIBLE_STORE_EXISTENCE"></a>

Flag for the existence of fungible store.


```move
module 0x1::fungible_asset {
    const EFUNGIBLE_STORE_EXISTENCE: u64 = 23;
}
```


<a id="0x1_fungible_asset_EINSUFFICIENT_BALANCE"></a>

Insufficient balance to withdraw or transfer.


```move
module 0x1::fungible_asset {
    const EINSUFFICIENT_BALANCE: u64 = 4;
}
```


<a id="0x1_fungible_asset_EINVALID_DISPATCHABLE_OPERATIONS"></a>

Invalid withdraw/deposit on dispatchable token. The specified token has a dispatchable function hook.
Need to invoke dispatchable_fungible_asset::withdraw/deposit to perform transfer.


```move
module 0x1::fungible_asset {
    const EINVALID_DISPATCHABLE_OPERATIONS: u64 = 28;
}
```


<a id="0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED"></a>

The fungible asset&apos;s supply has exceeded maximum.


```move
module 0x1::fungible_asset {
    const EMAX_SUPPLY_EXCEEDED: u64 = 5;
}
```


<a id="0x1_fungible_asset_EMINT_REF_AND_STORE_MISMATCH"></a>

The mint ref and the store do not match.


```move
module 0x1::fungible_asset {
    const EMINT_REF_AND_STORE_MISMATCH: u64 = 7;
}
```


<a id="0x1_fungible_asset_ENAME_TOO_LONG"></a>

Name of the fungible asset metadata is too long


```move
module 0x1::fungible_asset {
    const ENAME_TOO_LONG: u64 = 15;
}
```


<a id="0x1_fungible_asset_ENOT_METADATA_OWNER"></a>

Account is not the owner of metadata object.


```move
module 0x1::fungible_asset {
    const ENOT_METADATA_OWNER: u64 = 24;
}
```


<a id="0x1_fungible_asset_ENOT_STORE_OWNER"></a>

Account is not the store&apos;s owner.


```move
module 0x1::fungible_asset {
    const ENOT_STORE_OWNER: u64 = 8;
}
```


<a id="0x1_fungible_asset_EOBJECT_IS_DELETABLE"></a>

Fungibility is only available for non&#45;deletable objects.


```move
module 0x1::fungible_asset {
    const EOBJECT_IS_DELETABLE: u64 = 18;
}
```


<a id="0x1_fungible_asset_ESTORE_IS_FROZEN"></a>

Store is disabled from sending and receiving this fungible asset.


```move
module 0x1::fungible_asset {
    const ESTORE_IS_FROZEN: u64 = 3;
}
```


<a id="0x1_fungible_asset_ESUPPLY_NOT_FOUND"></a>

Supply resource is not found for a metadata object.


```move
module 0x1::fungible_asset {
    const ESUPPLY_NOT_FOUND: u64 = 21;
}
```


<a id="0x1_fungible_asset_ESUPPLY_UNDERFLOW"></a>

The fungible asset&apos;s supply will be negative which should be impossible.


```move
module 0x1::fungible_asset {
    const ESUPPLY_UNDERFLOW: u64 = 20;
}
```


<a id="0x1_fungible_asset_ESYMBOL_TOO_LONG"></a>

Symbol of the fungible asset metadata is too long


```move
module 0x1::fungible_asset {
    const ESYMBOL_TOO_LONG: u64 = 16;
}
```


<a id="0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

The transfer ref and the fungible asset do not match.


```move
module 0x1::fungible_asset {
    const ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH: u64 = 2;
}
```


<a id="0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH"></a>

Transfer ref and store do not match.


```move
module 0x1::fungible_asset {
    const ETRANSFER_REF_AND_STORE_MISMATCH: u64 = 9;
}
```


<a id="0x1_fungible_asset_EURI_TOO_LONG"></a>

URI for the icon of the fungible asset metadata is too long


```move
module 0x1::fungible_asset {
    const EURI_TOO_LONG: u64 = 19;
}
```


<a id="0x1_fungible_asset_EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided withdraw function type doesn&apos;t meet the signature requirement.


```move
module 0x1::fungible_asset {
    const EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH: u64 = 25;
}
```


<a id="0x1_fungible_asset_MAX_DECIMALS"></a>



```move
module 0x1::fungible_asset {
    const MAX_DECIMALS: u8 = 32;
}
```


<a id="0x1_fungible_asset_MAX_NAME_LENGTH"></a>



```move
module 0x1::fungible_asset {
    const MAX_NAME_LENGTH: u64 = 32;
}
```


<a id="0x1_fungible_asset_MAX_SYMBOL_LENGTH"></a>



```move
module 0x1::fungible_asset {
    const MAX_SYMBOL_LENGTH: u64 = 10;
}
```


<a id="0x1_fungible_asset_MAX_URI_LENGTH"></a>



```move
module 0x1::fungible_asset {
    const MAX_URI_LENGTH: u64 = 512;
}
```


<a id="0x1_fungible_asset_default_to_concurrent_fungible_supply"></a>

## Function `default_to_concurrent_fungible_supply`



```move
module 0x1::fungible_asset {
    fun default_to_concurrent_fungible_supply(): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun default_to_concurrent_fungible_supply(): bool {
        features::concurrent_fungible_assets_enabled()
    }
}
```


<a id="0x1_fungible_asset_allow_upgrade_to_concurrent_fungible_balance"></a>

## Function `allow_upgrade_to_concurrent_fungible_balance`



```move
module 0x1::fungible_asset {
    fun allow_upgrade_to_concurrent_fungible_balance(): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun allow_upgrade_to_concurrent_fungible_balance(): bool {
        features::concurrent_fungible_balance_enabled()
    }
}
```


<a id="0x1_fungible_asset_default_to_concurrent_fungible_balance"></a>

## Function `default_to_concurrent_fungible_balance`



```move
module 0x1::fungible_asset {
    fun default_to_concurrent_fungible_balance(): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun default_to_concurrent_fungible_balance(): bool {
        features::default_to_concurrent_fungible_balance_enabled()
    }
}
```


<a id="0x1_fungible_asset_add_fungibility"></a>

## Function `add_fungibility`

Make an existing object fungible by adding the Metadata resource.
This returns the capabilities to mint, burn, and transfer.
maximum_supply defines the behavior of maximum supply when monitoring:
&#45; option::none(): Monitoring unlimited supply
(width of the field &#45; MAX_U128 is the implicit maximum supply)
if option::some(MAX_U128) is used, it is treated as unlimited supply.
&#45; option::some(max): Monitoring fixed supply with `max` as the maximum supply.


```move
module 0x1::fungible_asset {
    public fun add_fungibility(constructor_ref: &object::ConstructorRef, maximum_supply: option::Option<u128>, name: string::String, symbol: string::String, decimals: u8, icon_uri: string::String, project_uri: string::String): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun add_fungibility(
        constructor_ref: &ConstructorRef,
        maximum_supply: Option<u128>,
        name: String,
        symbol: String,
        decimals: u8,
        icon_uri: String,
        project_uri: String,
    ): Object<Metadata> {
        assert!(!object::can_generate_delete_ref(constructor_ref), error::invalid_argument(EOBJECT_IS_DELETABLE));
        let metadata_object_signer = &object::generate_signer(constructor_ref);
        assert!(string::length(&name) <= MAX_NAME_LENGTH, error::out_of_range(ENAME_TOO_LONG));
        assert!(string::length(&symbol) <= MAX_SYMBOL_LENGTH, error::out_of_range(ESYMBOL_TOO_LONG));
        assert!(decimals <= MAX_DECIMALS, error::out_of_range(EDECIMALS_TOO_LARGE));
        assert!(string::length(&icon_uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        assert!(string::length(&project_uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        move_to(metadata_object_signer,
            Metadata {
                name,
                symbol,
                decimals,
                icon_uri,
                project_uri,
            }
        );

        if (default_to_concurrent_fungible_supply()) {
            let unlimited = option::is_none(&maximum_supply);
            move_to(metadata_object_signer, ConcurrentSupply {
                current: if (unlimited) {
                    aggregator_v2::create_unbounded_aggregator()
                } else {
                    aggregator_v2::create_aggregator(option::extract(&mut maximum_supply))
                },
            });
        } else {
            move_to(metadata_object_signer, Supply {
                current: 0,
                maximum: maximum_supply
            });
        };

        object::object_from_constructor_ref<Metadata>(constructor_ref)
    }
}
```


<a id="0x1_fungible_asset_set_untransferable"></a>

## Function `set_untransferable`

Set that only untransferable stores can be created for this fungible asset.


```move
module 0x1::fungible_asset {
    public fun set_untransferable(constructor_ref: &object::ConstructorRef)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun set_untransferable(constructor_ref: &ConstructorRef) {
        let metadata_addr = object::address_from_constructor_ref(constructor_ref);
        assert!(exists<Metadata>(metadata_addr), error::not_found(EFUNGIBLE_METADATA_EXISTENCE));
        let metadata_signer = &object::generate_signer(constructor_ref);
        move_to(metadata_signer, Untransferable {});
    }
}
```


<a id="0x1_fungible_asset_is_untransferable"></a>

## Function `is_untransferable`

Returns true if the FA is untransferable.


```move
module 0x1::fungible_asset {
    #[view]
    public fun is_untransferable<T: key>(metadata: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun is_untransferable<T: key>(metadata: Object<T>): bool {
        exists<Untransferable>(object::object_address(&metadata))
    }
}
```


<a id="0x1_fungible_asset_register_dispatch_functions"></a>

## Function `register_dispatch_functions`

Create a fungible asset store whose transfer rule would be overloaded by the provided function.


```move
module 0x1::fungible_asset {
    public(friend) fun register_dispatch_functions(constructor_ref: &object::ConstructorRef, withdraw_function: option::Option<function_info::FunctionInfo>, deposit_function: option::Option<function_info::FunctionInfo>, derived_balance_function: option::Option<function_info::FunctionInfo>)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun register_dispatch_functions(
        constructor_ref: &ConstructorRef,
        withdraw_function: Option<FunctionInfo>,
        deposit_function: Option<FunctionInfo>,
        derived_balance_function: Option<FunctionInfo>,
    ) {
        // Verify that caller type matches callee type so wrongly typed function cannot be registered.
        option::for_each_ref(&withdraw_function, |withdraw_function| {
            let dispatcher_withdraw_function_info = function_info::new_function_info_from_address(
                @aptos_framework,
                string::utf8(b"dispatchable_fungible_asset"),
                string::utf8(b"dispatchable_withdraw"),
            );

            assert!(
                function_info::check_dispatch_type_compatibility(
                    &dispatcher_withdraw_function_info,
                    withdraw_function
                ),
                error::invalid_argument(
                    EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH
                )
            );
        });

        option::for_each_ref(&deposit_function, |deposit_function| {
            let dispatcher_deposit_function_info = function_info::new_function_info_from_address(
                @aptos_framework,
                string::utf8(b"dispatchable_fungible_asset"),
                string::utf8(b"dispatchable_deposit"),
            );
            // Verify that caller type matches callee type so wrongly typed function cannot be registered.
            assert!(
                function_info::check_dispatch_type_compatibility(
                    &dispatcher_deposit_function_info,
                    deposit_function
                ),
                error::invalid_argument(
                    EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH
                )
            );
        });

        option::for_each_ref(&derived_balance_function, |balance_function| {
            let dispatcher_derived_balance_function_info = function_info::new_function_info_from_address(
                @aptos_framework,
                string::utf8(b"dispatchable_fungible_asset"),
                string::utf8(b"dispatchable_derived_balance"),
            );
            // Verify that caller type matches callee type so wrongly typed function cannot be registered.
            assert!(
                function_info::check_dispatch_type_compatibility(
                    &dispatcher_derived_balance_function_info,
                    balance_function
                ),
                error::invalid_argument(
                    EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH
                )
            );
        });

        // Cannot register hook for APT.
        assert!(
            object::address_from_constructor_ref(constructor_ref) != @aptos_fungible_asset,
            error::permission_denied(EAPT_NOT_DISPATCHABLE)
        );
        assert!(
            !object::can_generate_delete_ref(constructor_ref),
            error::invalid_argument(EOBJECT_IS_DELETABLE)
        );
        assert!(
            !exists<DispatchFunctionStore>(
                object::address_from_constructor_ref(constructor_ref)
            ),
            error::already_exists(EALREADY_REGISTERED)
        );
        assert!(
            exists<Metadata>(
                object::address_from_constructor_ref(constructor_ref)
            ),
            error::not_found(EFUNGIBLE_METADATA_EXISTENCE),
        );

        let store_obj = &object::generate_signer(constructor_ref);

        // Store the overload function hook.
        move_to<DispatchFunctionStore>(
            store_obj,
            DispatchFunctionStore {
                withdraw_function,
                deposit_function,
                derived_balance_function,
            }
        );
    }
}
```


<a id="0x1_fungible_asset_generate_mint_ref"></a>

## Function `generate_mint_ref`

Creates a mint ref that can be used to mint fungible assets from the given fungible object&apos;s constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


```move
module 0x1::fungible_asset {
    public fun generate_mint_ref(constructor_ref: &object::ConstructorRef): fungible_asset::MintRef
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun generate_mint_ref(constructor_ref: &ConstructorRef): MintRef {
        let metadata = object::object_from_constructor_ref<Metadata>(constructor_ref);
        MintRef { metadata }
    }
}
```


<a id="0x1_fungible_asset_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a burn ref that can be used to burn fungible assets from the given fungible object&apos;s constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


```move
module 0x1::fungible_asset {
    public fun generate_burn_ref(constructor_ref: &object::ConstructorRef): fungible_asset::BurnRef
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun generate_burn_ref(constructor_ref: &ConstructorRef): BurnRef {
        let metadata = object::object_from_constructor_ref<Metadata>(constructor_ref);
        BurnRef { metadata }
    }
}
```


<a id="0x1_fungible_asset_generate_transfer_ref"></a>

## Function `generate_transfer_ref`

Creates a transfer ref that can be used to freeze/unfreeze/transfer fungible assets from the given fungible
object&apos;s constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


```move
module 0x1::fungible_asset {
    public fun generate_transfer_ref(constructor_ref: &object::ConstructorRef): fungible_asset::TransferRef
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun generate_transfer_ref(constructor_ref: &ConstructorRef): TransferRef {
        let metadata = object::object_from_constructor_ref<Metadata>(constructor_ref);
        TransferRef { metadata }
    }
}
```


<a id="0x1_fungible_asset_supply"></a>

## Function `supply`

Get the current supply from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun supply<T: key>(metadata: object::Object<T>): option::Option<u128>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun supply<T: key>(metadata: Object<T>): Option<u128> acquires Supply, ConcurrentSupply {
        let metadata_address = object::object_address(&metadata);
        if (exists<ConcurrentSupply>(metadata_address)) {
            let supply = borrow_global<ConcurrentSupply>(metadata_address);
            option::some(aggregator_v2::read(&supply.current))
        } else if (exists<Supply>(metadata_address)) {
            let supply = borrow_global<Supply>(metadata_address);
            option::some(supply.current)
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_fungible_asset_maximum"></a>

## Function `maximum`

Get the maximum supply from the `metadata` object.
If supply is unlimited (or set explicitly to MAX_U128), none is returned


```move
module 0x1::fungible_asset {
    #[view]
    public fun maximum<T: key>(metadata: object::Object<T>): option::Option<u128>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun maximum<T: key>(metadata: Object<T>): Option<u128> acquires Supply, ConcurrentSupply {
        let metadata_address = object::object_address(&metadata);
        if (exists<ConcurrentSupply>(metadata_address)) {
            let supply = borrow_global<ConcurrentSupply>(metadata_address);
            let max_value = aggregator_v2::max_value(&supply.current);
            if (max_value == MAX_U128) {
                option::none()
            } else {
                option::some(max_value)
            }
        } else if (exists<Supply>(metadata_address)) {
            let supply = borrow_global<Supply>(metadata_address);
            supply.maximum
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_fungible_asset_name"></a>

## Function `name`

Get the name of the fungible asset from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun name<T: key>(metadata: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun name<T: key>(metadata: Object<T>): String acquires Metadata {
        borrow_fungible_metadata(&metadata).name
    }
}
```


<a id="0x1_fungible_asset_symbol"></a>

## Function `symbol`

Get the symbol of the fungible asset from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun symbol<T: key>(metadata: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun symbol<T: key>(metadata: Object<T>): String acquires Metadata {
        borrow_fungible_metadata(&metadata).symbol
    }
}
```


<a id="0x1_fungible_asset_decimals"></a>

## Function `decimals`

Get the decimals from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun decimals<T: key>(metadata: object::Object<T>): u8
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun decimals<T: key>(metadata: Object<T>): u8 acquires Metadata {
        borrow_fungible_metadata(&metadata).decimals
    }
}
```


<a id="0x1_fungible_asset_icon_uri"></a>

## Function `icon_uri`

Get the icon uri from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun icon_uri<T: key>(metadata: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun icon_uri<T: key>(metadata: Object<T>): String acquires Metadata {
        borrow_fungible_metadata(&metadata).icon_uri
    }
}
```


<a id="0x1_fungible_asset_project_uri"></a>

## Function `project_uri`

Get the project uri from the `metadata` object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun project_uri<T: key>(metadata: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun project_uri<T: key>(metadata: Object<T>): String acquires Metadata {
        borrow_fungible_metadata(&metadata).project_uri
    }
}
```


<a id="0x1_fungible_asset_store_exists"></a>

## Function `store_exists`

Return whether the provided address has a store initialized.


```move
module 0x1::fungible_asset {
    #[view]
    public fun store_exists(store: address): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun store_exists(store: address): bool {
        store_exists_inline(store)
    }
}
```


<a id="0x1_fungible_asset_store_exists_inline"></a>

## Function `store_exists_inline`

Return whether the provided address has a store initialized.


```move
module 0x1::fungible_asset {
    fun store_exists_inline(store: address): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun store_exists_inline(store: address): bool {
        exists<FungibleStore>(store)
    }
}
```


<a id="0x1_fungible_asset_concurrent_fungible_balance_exists_inline"></a>

## Function `concurrent_fungible_balance_exists_inline`

Return whether the provided address has a concurrent fungible balance initialized,
at the fungible store address.


```move
module 0x1::fungible_asset {
    fun concurrent_fungible_balance_exists_inline(store: address): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun concurrent_fungible_balance_exists_inline(store: address): bool {
        exists<ConcurrentFungibleBalance>(store)
    }
}
```


<a id="0x1_fungible_asset_metadata_from_asset"></a>

## Function `metadata_from_asset`

Return the underlying metadata object


```move
module 0x1::fungible_asset {
    public fun metadata_from_asset(fa: &fungible_asset::FungibleAsset): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun metadata_from_asset(fa: &FungibleAsset): Object<Metadata> {
        fa.metadata
    }
}
```


<a id="0x1_fungible_asset_store_metadata"></a>

## Function `store_metadata`

Return the underlying metadata object.


```move
module 0x1::fungible_asset {
    #[view]
    public fun store_metadata<T: key>(store: object::Object<T>): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun store_metadata<T: key>(store: Object<T>): Object<Metadata> acquires FungibleStore {
        borrow_store_resource(&store).metadata
    }
}
```


<a id="0x1_fungible_asset_amount"></a>

## Function `amount`

Return the `amount` of a given fungible asset.


```move
module 0x1::fungible_asset {
    public fun amount(fa: &fungible_asset::FungibleAsset): u64
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun amount(fa: &FungibleAsset): u64 {
        fa.amount
    }
}
```


<a id="0x1_fungible_asset_balance"></a>

## Function `balance`

Get the balance of a given store.


```move
module 0x1::fungible_asset {
    #[view]
    public fun balance<T: key>(store: object::Object<T>): u64
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun balance<T: key>(store: Object<T>): u64 acquires FungibleStore, ConcurrentFungibleBalance {
        let store_addr = object::object_address(&store);
        if (store_exists_inline(store_addr)) {
            let store_balance = borrow_store_resource(&store).balance;
            if (store_balance == 0 && concurrent_fungible_balance_exists_inline(store_addr)) {
                let balance_resource = borrow_global<ConcurrentFungibleBalance>(store_addr);
                aggregator_v2::read(&balance_resource.balance)
            } else {
                store_balance
            }
        } else {
            0
        }
    }
}
```


<a id="0x1_fungible_asset_is_balance_at_least"></a>

## Function `is_balance_at_least`

Check whether the balance of a given store is &gt;&#61; `amount`.


```move
module 0x1::fungible_asset {
    #[view]
    public fun is_balance_at_least<T: key>(store: object::Object<T>, amount: u64): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun is_balance_at_least<T: key>(store: Object<T>, amount: u64): bool acquires FungibleStore, ConcurrentFungibleBalance {
        let store_addr = object::object_address(&store);
        is_address_balance_at_least(store_addr, amount)
    }
}
```


<a id="0x1_fungible_asset_is_address_balance_at_least"></a>

## Function `is_address_balance_at_least`

Check whether the balance of a given store is &gt;&#61; `amount`.


```move
module 0x1::fungible_asset {
    public(friend) fun is_address_balance_at_least(store_addr: address, amount: u64): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun is_address_balance_at_least(store_addr: address, amount: u64): bool acquires FungibleStore, ConcurrentFungibleBalance {
        if (store_exists_inline(store_addr)) {
            let store_balance = borrow_global<FungibleStore>(store_addr).balance;
            if (store_balance == 0 && concurrent_fungible_balance_exists_inline(store_addr)) {
                let balance_resource = borrow_global<ConcurrentFungibleBalance>(store_addr);
                aggregator_v2::is_at_least(&balance_resource.balance, amount)
            } else {
                store_balance >= amount
            }
        } else {
            amount == 0
        }
    }
}
```


<a id="0x1_fungible_asset_is_frozen"></a>

## Function `is_frozen`

Return whether a store is frozen.

If the store has not been created, we default to returning false so deposits can be sent to it.


```move
module 0x1::fungible_asset {
    #[view]
    public fun is_frozen<T: key>(store: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun is_frozen<T: key>(store: Object<T>): bool acquires FungibleStore {
        let store_addr = object::object_address(&store);
        store_exists_inline(store_addr) && borrow_global<FungibleStore>(store_addr).frozen
    }
}
```


<a id="0x1_fungible_asset_is_store_dispatchable"></a>

## Function `is_store_dispatchable`

Return whether a fungible asset type is dispatchable.


```move
module 0x1::fungible_asset {
    #[view]
    public fun is_store_dispatchable<T: key>(store: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun is_store_dispatchable<T: key>(store: Object<T>): bool acquires FungibleStore {
        let fa_store = borrow_store_resource(&store);
        let metadata_addr = object::object_address(&fa_store.metadata);
        exists<DispatchFunctionStore>(metadata_addr)
    }
}
```


<a id="0x1_fungible_asset_deposit_dispatch_function"></a>

## Function `deposit_dispatch_function`



```move
module 0x1::fungible_asset {
    public fun deposit_dispatch_function<T: key>(store: object::Object<T>): option::Option<function_info::FunctionInfo>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun deposit_dispatch_function<T: key>(store: Object<T>): Option<FunctionInfo> acquires FungibleStore, DispatchFunctionStore {
        let fa_store = borrow_store_resource(&store);
        let metadata_addr = object::object_address(&fa_store.metadata);
        if(exists<DispatchFunctionStore>(metadata_addr)) {
            borrow_global<DispatchFunctionStore>(metadata_addr).deposit_function
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_fungible_asset_has_deposit_dispatch_function"></a>

## Function `has_deposit_dispatch_function`



```move
module 0x1::fungible_asset {
    fun has_deposit_dispatch_function(metadata: object::Object<fungible_asset::Metadata>): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    fun has_deposit_dispatch_function(metadata: Object<Metadata>): bool acquires DispatchFunctionStore {
        let metadata_addr = object::object_address(&metadata);
        // Short circuit on APT for better perf
        if(metadata_addr != @aptos_fungible_asset && exists<DispatchFunctionStore>(metadata_addr)) {
            option::is_some(&borrow_global<DispatchFunctionStore>(metadata_addr).deposit_function)
        } else {
            false
        }
    }
}
```


<a id="0x1_fungible_asset_withdraw_dispatch_function"></a>

## Function `withdraw_dispatch_function`



```move
module 0x1::fungible_asset {
    public fun withdraw_dispatch_function<T: key>(store: object::Object<T>): option::Option<function_info::FunctionInfo>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun withdraw_dispatch_function<T: key>(store: Object<T>): Option<FunctionInfo> acquires FungibleStore, DispatchFunctionStore {
        let fa_store = borrow_store_resource(&store);
        let metadata_addr = object::object_address(&fa_store.metadata);
        if(exists<DispatchFunctionStore>(metadata_addr)) {
            borrow_global<DispatchFunctionStore>(metadata_addr).withdraw_function
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_fungible_asset_has_withdraw_dispatch_function"></a>

## Function `has_withdraw_dispatch_function`



```move
module 0x1::fungible_asset {
    fun has_withdraw_dispatch_function(metadata: object::Object<fungible_asset::Metadata>): bool
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    fun has_withdraw_dispatch_function(metadata: Object<Metadata>): bool acquires DispatchFunctionStore {
        let metadata_addr = object::object_address(&metadata);
        // Short circuit on APT for better perf
        if (metadata_addr != @aptos_fungible_asset && exists<DispatchFunctionStore>(metadata_addr)) {
            option::is_some(&borrow_global<DispatchFunctionStore>(metadata_addr).withdraw_function)
        } else {
            false
        }
    }
}
```


<a id="0x1_fungible_asset_derived_balance_dispatch_function"></a>

## Function `derived_balance_dispatch_function`



```move
module 0x1::fungible_asset {
    public(friend) fun derived_balance_dispatch_function<T: key>(store: object::Object<T>): option::Option<function_info::FunctionInfo>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun derived_balance_dispatch_function<T: key>(store: Object<T>): Option<FunctionInfo> acquires FungibleStore, DispatchFunctionStore {
        let fa_store = borrow_store_resource(&store);
        let metadata_addr = object::object_address(&fa_store.metadata);
        if (exists<DispatchFunctionStore>(metadata_addr)) {
            borrow_global<DispatchFunctionStore>(metadata_addr).derived_balance_function
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_fungible_asset_asset_metadata"></a>

## Function `asset_metadata`



```move
module 0x1::fungible_asset {
    public fun asset_metadata(fa: &fungible_asset::FungibleAsset): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun asset_metadata(fa: &FungibleAsset): Object<Metadata> {
        fa.metadata
    }
}
```


<a id="0x1_fungible_asset_mint_ref_metadata"></a>

## Function `mint_ref_metadata`

Get the underlying metadata object from the `MintRef`.


```move
module 0x1::fungible_asset {
    public fun mint_ref_metadata(ref: &fungible_asset::MintRef): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun mint_ref_metadata(ref: &MintRef): Object<Metadata> {
        ref.metadata
    }
}
```


<a id="0x1_fungible_asset_transfer_ref_metadata"></a>

## Function `transfer_ref_metadata`

Get the underlying metadata object from the `TransferRef`.


```move
module 0x1::fungible_asset {
    public fun transfer_ref_metadata(ref: &fungible_asset::TransferRef): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun transfer_ref_metadata(ref: &TransferRef): Object<Metadata> {
        ref.metadata
    }
}
```


<a id="0x1_fungible_asset_burn_ref_metadata"></a>

## Function `burn_ref_metadata`

Get the underlying metadata object from the `BurnRef`.


```move
module 0x1::fungible_asset {
    public fun burn_ref_metadata(ref: &fungible_asset::BurnRef): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun burn_ref_metadata(ref: &BurnRef): Object<Metadata> {
        ref.metadata
    }
}
```


<a id="0x1_fungible_asset_transfer"></a>

## Function `transfer`

Transfer an `amount` of fungible asset from `from_store`, which should be owned by `sender`, to `receiver`.
Note: it does not move the underlying object.


```move
module 0x1::fungible_asset {
    public entry fun transfer<T: key>(sender: &signer, from: object::Object<T>, to: object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public entry fun transfer<T: key>(
        sender: &signer,
        from: Object<T>,
        to: Object<T>,
        amount: u64,
    ) acquires FungibleStore, DispatchFunctionStore, ConcurrentFungibleBalance {
        let fa = withdraw(sender, from, amount);
        deposit(to, fa);
    }
}
```


<a id="0x1_fungible_asset_create_store"></a>

## Function `create_store`

Allow an object to hold a store for fungible assets.
Applications can use this to create multiple stores for isolating fungible assets for different purposes.


```move
module 0x1::fungible_asset {
    public fun create_store<T: key>(constructor_ref: &object::ConstructorRef, metadata: object::Object<T>): object::Object<fungible_asset::FungibleStore>
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun create_store<T: key>(
        constructor_ref: &ConstructorRef,
        metadata: Object<T>,
    ): Object<FungibleStore> {
        let store_obj = &object::generate_signer(constructor_ref);
        move_to(store_obj, FungibleStore {
            metadata: object::convert(metadata),
            balance: 0,
            frozen: false,
        });

        if (is_untransferable(metadata)) {
            object::set_untransferable(constructor_ref);
        };

        if (default_to_concurrent_fungible_balance()) {
            move_to(store_obj, ConcurrentFungibleBalance {
                balance: aggregator_v2::create_unbounded_aggregator(),
            });
        };

        object::object_from_constructor_ref<FungibleStore>(constructor_ref)
    }
}
```


<a id="0x1_fungible_asset_remove_store"></a>

## Function `remove_store`

Used to delete a store.  Requires the store to be completely empty prior to removing it


```move
module 0x1::fungible_asset {
    public fun remove_store(delete_ref: &object::DeleteRef)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun remove_store(delete_ref: &DeleteRef) acquires FungibleStore, FungibleAssetEvents, ConcurrentFungibleBalance {
        let store = &object::object_from_delete_ref<FungibleStore>(delete_ref);
        let addr = object::object_address(store);
        let FungibleStore { metadata: _, balance, frozen: _ }
            = move_from<FungibleStore>(addr);
        assert!(balance == 0, error::permission_denied(EBALANCE_IS_NOT_ZERO));

        if (concurrent_fungible_balance_exists_inline(addr)) {
            let ConcurrentFungibleBalance { balance } = move_from<ConcurrentFungibleBalance>(addr);
            assert!(aggregator_v2::read(&balance) == 0, error::permission_denied(EBALANCE_IS_NOT_ZERO));
        };

        // Cleanup deprecated event handles if exist.
        if (exists<FungibleAssetEvents>(addr)) {
            let FungibleAssetEvents {
                deposit_events,
                withdraw_events,
                frozen_events,
            } = move_from<FungibleAssetEvents>(addr);
            event::destroy_handle(deposit_events);
            event::destroy_handle(withdraw_events);
            event::destroy_handle(frozen_events);
        };
    }
}
```


<a id="0x1_fungible_asset_withdraw"></a>

## Function `withdraw`

Withdraw `amount` of the fungible asset from `store` by the owner.


```move
module 0x1::fungible_asset {
    public fun withdraw<T: key>(owner: &signer, store: object::Object<T>, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun withdraw<T: key>(
        owner: &signer,
        store: Object<T>,
        amount: u64,
    ): FungibleAsset acquires FungibleStore, DispatchFunctionStore, ConcurrentFungibleBalance {
        withdraw_sanity_check(owner, store, true);
        withdraw_internal(object::object_address(&store), amount)
    }
}
```


<a id="0x1_fungible_asset_withdraw_sanity_check"></a>

## Function `withdraw_sanity_check`

Check the permission for withdraw operation.


```move
module 0x1::fungible_asset {
    public(friend) fun withdraw_sanity_check<T: key>(owner: &signer, store: object::Object<T>, abort_on_dispatch: bool)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun withdraw_sanity_check<T: key>(
        owner: &signer,
        store: Object<T>,
        abort_on_dispatch: bool,
    ) acquires FungibleStore, DispatchFunctionStore {
        assert!(object::owns(store, signer::address_of(owner)), error::permission_denied(ENOT_STORE_OWNER));
        let fa_store = borrow_store_resource(&store);
        assert!(
            !abort_on_dispatch || !has_withdraw_dispatch_function(fa_store.metadata),
            error::invalid_argument(EINVALID_DISPATCHABLE_OPERATIONS)
        );
        assert!(!fa_store.frozen, error::permission_denied(ESTORE_IS_FROZEN));
    }
}
```


<a id="0x1_fungible_asset_deposit_sanity_check"></a>

## Function `deposit_sanity_check`

Deposit `amount` of the fungible asset to `store`.


```move
module 0x1::fungible_asset {
    public fun deposit_sanity_check<T: key>(store: object::Object<T>, abort_on_dispatch: bool)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun deposit_sanity_check<T: key>(
        store: Object<T>,
        abort_on_dispatch: bool
    ) acquires FungibleStore, DispatchFunctionStore {
        let fa_store = borrow_store_resource(&store);
        assert!(
            !abort_on_dispatch || !has_deposit_dispatch_function(fa_store.metadata),
            error::invalid_argument(EINVALID_DISPATCHABLE_OPERATIONS)
        );
        assert!(!fa_store.frozen, error::permission_denied(ESTORE_IS_FROZEN));
    }
}
```


<a id="0x1_fungible_asset_deposit"></a>

## Function `deposit`

Deposit `amount` of the fungible asset to `store`.


```move
module 0x1::fungible_asset {
    public fun deposit<T: key>(store: object::Object<T>, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun deposit<T: key>(store: Object<T>, fa: FungibleAsset) acquires FungibleStore, DispatchFunctionStore, ConcurrentFungibleBalance {
        deposit_sanity_check(store, true);
        deposit_internal(object::object_address(&store), fa);
    }
}
```


<a id="0x1_fungible_asset_mint"></a>

## Function `mint`

Mint the specified `amount` of the fungible asset.


```move
module 0x1::fungible_asset {
    public fun mint(ref: &fungible_asset::MintRef, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun mint(ref: &MintRef, amount: u64): FungibleAsset acquires Supply, ConcurrentSupply {
        let metadata = ref.metadata;
        mint_internal(metadata, amount)
    }
}
```


<a id="0x1_fungible_asset_mint_internal"></a>

## Function `mint_internal`

CAN ONLY BE CALLED BY coin.move for migration.


```move
module 0x1::fungible_asset {
    public(friend) fun mint_internal(metadata: object::Object<fungible_asset::Metadata>, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun mint_internal(
        metadata: Object<Metadata>,
        amount: u64
    ): FungibleAsset acquires Supply, ConcurrentSupply {
        increase_supply(&metadata, amount);
        FungibleAsset {
            metadata,
            amount
        }
    }
}
```


<a id="0x1_fungible_asset_mint_to"></a>

## Function `mint_to`

Mint the specified `amount` of the fungible asset to a destination store.


```move
module 0x1::fungible_asset {
    public fun mint_to<T: key>(ref: &fungible_asset::MintRef, store: object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun mint_to<T: key>(ref: &MintRef, store: Object<T>, amount: u64)
    acquires FungibleStore, Supply, ConcurrentSupply, DispatchFunctionStore, ConcurrentFungibleBalance {
        deposit_sanity_check(store, false);
        deposit_internal(object::object_address(&store), mint(ref, amount));
    }
}
```


<a id="0x1_fungible_asset_set_frozen_flag"></a>

## Function `set_frozen_flag`

Enable/disable a store&apos;s ability to do direct transfers of the fungible asset.


```move
module 0x1::fungible_asset {
    public fun set_frozen_flag<T: key>(ref: &fungible_asset::TransferRef, store: object::Object<T>, frozen: bool)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun set_frozen_flag<T: key>(
        ref: &TransferRef,
        store: Object<T>,
        frozen: bool,
    ) acquires FungibleStore {
        assert!(
            ref.metadata == store_metadata(store),
            error::invalid_argument(ETRANSFER_REF_AND_STORE_MISMATCH),
        );
        set_frozen_flag_internal(store, frozen)
    }
}
```


<a id="0x1_fungible_asset_set_frozen_flag_internal"></a>

## Function `set_frozen_flag_internal`



```move
module 0x1::fungible_asset {
    public(friend) fun set_frozen_flag_internal<T: key>(store: object::Object<T>, frozen: bool)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun set_frozen_flag_internal<T: key>(
        store: Object<T>,
        frozen: bool
    ) acquires FungibleStore {
        let store_addr = object::object_address(&store);
        borrow_global_mut<FungibleStore>(store_addr).frozen = frozen;

        event::emit(Frozen { store: store_addr, frozen });
    }
}
```


<a id="0x1_fungible_asset_burn"></a>

## Function `burn`

Burns a fungible asset


```move
module 0x1::fungible_asset {
    public fun burn(ref: &fungible_asset::BurnRef, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun burn(ref: &BurnRef, fa: FungibleAsset) acquires Supply, ConcurrentSupply {
        assert!(
            ref.metadata == metadata_from_asset(&fa),
            error::invalid_argument(EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH)
        );
        burn_internal(fa);
    }
}
```


<a id="0x1_fungible_asset_burn_internal"></a>

## Function `burn_internal`

CAN ONLY BE CALLED BY coin.move for migration.


```move
module 0x1::fungible_asset {
    public(friend) fun burn_internal(fa: fungible_asset::FungibleAsset): u64
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun burn_internal(
        fa: FungibleAsset
    ): u64 acquires Supply, ConcurrentSupply {
        let FungibleAsset {
            metadata,
            amount
        } = fa;
        decrease_supply(&metadata, amount);
        amount
    }
}
```


<a id="0x1_fungible_asset_burn_from"></a>

## Function `burn_from`

Burn the `amount` of the fungible asset from the given store.


```move
module 0x1::fungible_asset {
    public fun burn_from<T: key>(ref: &fungible_asset::BurnRef, store: object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun burn_from<T: key>(
        ref: &BurnRef,
        store: Object<T>,
        amount: u64
    ) acquires FungibleStore, Supply, ConcurrentSupply, ConcurrentFungibleBalance {
        // ref metadata match is checked in burn() call
        burn(ref, withdraw_internal(object::object_address(&store), amount));
    }
}
```


<a id="0x1_fungible_asset_address_burn_from"></a>

## Function `address_burn_from`



```move
module 0x1::fungible_asset {
    public(friend) fun address_burn_from(ref: &fungible_asset::BurnRef, store_addr: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun address_burn_from(
        ref: &BurnRef,
        store_addr: address,
        amount: u64
    ) acquires FungibleStore, Supply, ConcurrentSupply, ConcurrentFungibleBalance {
        // ref metadata match is checked in burn() call
        burn(ref, withdraw_internal(store_addr, amount));
    }
}
```


<a id="0x1_fungible_asset_withdraw_with_ref"></a>

## Function `withdraw_with_ref`

Withdraw `amount` of the fungible asset from the `store` ignoring `frozen`.


```move
module 0x1::fungible_asset {
    public fun withdraw_with_ref<T: key>(ref: &fungible_asset::TransferRef, store: object::Object<T>, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun withdraw_with_ref<T: key>(
        ref: &TransferRef,
        store: Object<T>,
        amount: u64
    ): FungibleAsset acquires FungibleStore, ConcurrentFungibleBalance {
        assert!(
            ref.metadata == store_metadata(store),
            error::invalid_argument(ETRANSFER_REF_AND_STORE_MISMATCH),
        );
        withdraw_internal(object::object_address(&store), amount)
    }
}
```


<a id="0x1_fungible_asset_deposit_with_ref"></a>

## Function `deposit_with_ref`

Deposit the fungible asset into the `store` ignoring `frozen`.


```move
module 0x1::fungible_asset {
    public fun deposit_with_ref<T: key>(ref: &fungible_asset::TransferRef, store: object::Object<T>, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun deposit_with_ref<T: key>(
        ref: &TransferRef,
        store: Object<T>,
        fa: FungibleAsset
    ) acquires FungibleStore, ConcurrentFungibleBalance {
        assert!(
            ref.metadata == fa.metadata,
            error::invalid_argument(ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH)
        );
        deposit_internal(object::object_address(&store), fa);
    }
}
```


<a id="0x1_fungible_asset_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer `amount` of the fungible asset with `TransferRef` even it is frozen.


```move
module 0x1::fungible_asset {
    public fun transfer_with_ref<T: key>(transfer_ref: &fungible_asset::TransferRef, from: object::Object<T>, to: object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun transfer_with_ref<T: key>(
        transfer_ref: &TransferRef,
        from: Object<T>,
        to: Object<T>,
        amount: u64,
    ) acquires FungibleStore, ConcurrentFungibleBalance {
        let fa = withdraw_with_ref(transfer_ref, from, amount);
        deposit_with_ref(transfer_ref, to, fa);
    }
}
```


<a id="0x1_fungible_asset_zero"></a>

## Function `zero`

Create a fungible asset with zero amount.
This can be useful when starting a series of computations where the initial value is 0.


```move
module 0x1::fungible_asset {
    public fun zero<T: key>(metadata: object::Object<T>): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun zero<T: key>(metadata: Object<T>): FungibleAsset {
        FungibleAsset {
            metadata: object::convert(metadata),
            amount: 0,
        }
    }
}
```


<a id="0x1_fungible_asset_extract"></a>

## Function `extract`

Extract a given amount from the given fungible asset and return a new one.


```move
module 0x1::fungible_asset {
    public fun extract(fungible_asset: &mut fungible_asset::FungibleAsset, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun extract(fungible_asset: &mut FungibleAsset, amount: u64): FungibleAsset {
        assert!(fungible_asset.amount >= amount, error::invalid_argument(EINSUFFICIENT_BALANCE));
        fungible_asset.amount = fungible_asset.amount - amount;
        FungibleAsset {
            metadata: fungible_asset.metadata,
            amount,
        }
    }
}
```


<a id="0x1_fungible_asset_merge"></a>

## Function `merge`

&quot;Merges&quot; the two given fungible assets. The fungible asset passed in as `dst_fungible_asset` will have a value
equal to the sum of the two (`dst_fungible_asset` and `src_fungible_asset`).


```move
module 0x1::fungible_asset {
    public fun merge(dst_fungible_asset: &mut fungible_asset::FungibleAsset, src_fungible_asset: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun merge(dst_fungible_asset: &mut FungibleAsset, src_fungible_asset: FungibleAsset) {
        let FungibleAsset { metadata, amount } = src_fungible_asset;
        assert!(metadata == dst_fungible_asset.metadata, error::invalid_argument(EFUNGIBLE_ASSET_MISMATCH));
        dst_fungible_asset.amount = dst_fungible_asset.amount + amount;
    }
}
```


<a id="0x1_fungible_asset_destroy_zero"></a>

## Function `destroy_zero`

Destroy an empty fungible asset.


```move
module 0x1::fungible_asset {
    public fun destroy_zero(fungible_asset: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun destroy_zero(fungible_asset: FungibleAsset) {
        let FungibleAsset { amount, metadata: _ } = fungible_asset;
        assert!(amount == 0, error::invalid_argument(EAMOUNT_IS_NOT_ZERO));
    }
}
```


<a id="0x1_fungible_asset_deposit_internal"></a>

## Function `deposit_internal`



```move
module 0x1::fungible_asset {
    public(friend) fun deposit_internal(store_addr: address, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun deposit_internal(store_addr: address, fa: FungibleAsset) acquires FungibleStore, ConcurrentFungibleBalance {
        let FungibleAsset { metadata, amount } = fa;
        if (amount == 0) return;

        assert!(exists<FungibleStore>(store_addr), error::not_found(EFUNGIBLE_STORE_EXISTENCE));
        let store = borrow_global_mut<FungibleStore>(store_addr);
        assert!(metadata == store.metadata, error::invalid_argument(EFUNGIBLE_ASSET_AND_STORE_MISMATCH));

        if (store.balance == 0 && concurrent_fungible_balance_exists_inline(store_addr)) {
            let balance_resource = borrow_global_mut<ConcurrentFungibleBalance>(store_addr);
            aggregator_v2::add(&mut balance_resource.balance, amount);
        } else {
            store.balance = store.balance + amount;
        };

        event::emit(Deposit { store: store_addr, amount });
    }
}
```


<a id="0x1_fungible_asset_withdraw_internal"></a>

## Function `withdraw_internal`

Extract `amount` of the fungible asset from `store`.


```move
module 0x1::fungible_asset {
    public(friend) fun withdraw_internal(store_addr: address, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public(friend) fun withdraw_internal(
        store_addr: address,
        amount: u64,
    ): FungibleAsset acquires FungibleStore, ConcurrentFungibleBalance {
        assert!(exists<FungibleStore>(store_addr), error::not_found(EFUNGIBLE_STORE_EXISTENCE));

        let store = borrow_global_mut<FungibleStore>(store_addr);
        let metadata = store.metadata;
        if (amount != 0) {
            if (store.balance == 0 && concurrent_fungible_balance_exists_inline(store_addr)) {
                let balance_resource = borrow_global_mut<ConcurrentFungibleBalance>(store_addr);
                assert!(
                    aggregator_v2::try_sub(&mut balance_resource.balance, amount),
                    error::invalid_argument(EINSUFFICIENT_BALANCE)
                );
            } else {
                assert!(store.balance >= amount, error::invalid_argument(EINSUFFICIENT_BALANCE));
                store.balance = store.balance - amount;
            };

            event::emit<Withdraw>(Withdraw { store: store_addr, amount });
        };
        FungibleAsset { metadata, amount }
    }
}
```


<a id="0x1_fungible_asset_increase_supply"></a>

## Function `increase_supply`

Increase the supply of a fungible asset by minting.


```move
module 0x1::fungible_asset {
    fun increase_supply<T: key>(metadata: &object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    fun increase_supply<T: key>(metadata: &Object<T>, amount: u64) acquires Supply, ConcurrentSupply {
        if (amount == 0) {
            return
        };
        let metadata_address = object::object_address(metadata);

        if (exists<ConcurrentSupply>(metadata_address)) {
            let supply = borrow_global_mut<ConcurrentSupply>(metadata_address);
            assert!(
                aggregator_v2::try_add(&mut supply.current, (amount as u128)),
                error::out_of_range(EMAX_SUPPLY_EXCEEDED)
            );
        } else if (exists<Supply>(metadata_address)) {
            let supply = borrow_global_mut<Supply>(metadata_address);
            if (option::is_some(&supply.maximum)) {
                let max = *option::borrow_mut(&mut supply.maximum);
                assert!(
                    max - supply.current >= (amount as u128),
                    error::out_of_range(EMAX_SUPPLY_EXCEEDED)
                )
            };
            supply.current = supply.current + (amount as u128);
        } else {
            abort error::not_found(ESUPPLY_NOT_FOUND)
        }
    }
}
```


<a id="0x1_fungible_asset_decrease_supply"></a>

## Function `decrease_supply`

Decrease the supply of a fungible asset by burning.


```move
module 0x1::fungible_asset {
    fun decrease_supply<T: key>(metadata: &object::Object<T>, amount: u64)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    fun decrease_supply<T: key>(metadata: &Object<T>, amount: u64) acquires Supply, ConcurrentSupply {
        if (amount == 0) {
            return
        };
        let metadata_address = object::object_address(metadata);

        if (exists<ConcurrentSupply>(metadata_address)) {
            let supply = borrow_global_mut<ConcurrentSupply>(metadata_address);

            assert!(
                aggregator_v2::try_sub(&mut supply.current, (amount as u128)),
                error::out_of_range(ESUPPLY_UNDERFLOW)
            );
        } else if (exists<Supply>(metadata_address)) {
            assert!(exists<Supply>(metadata_address), error::not_found(ESUPPLY_NOT_FOUND));
            let supply = borrow_global_mut<Supply>(metadata_address);
            assert!(
                supply.current >= (amount as u128),
                error::invalid_state(ESUPPLY_UNDERFLOW)
            );
            supply.current = supply.current - (amount as u128);
        } else {
            assert!(false, error::not_found(ESUPPLY_NOT_FOUND));
        }
    }
}
```


<a id="0x1_fungible_asset_borrow_fungible_metadata"></a>

## Function `borrow_fungible_metadata`



```move
module 0x1::fungible_asset {
    fun borrow_fungible_metadata<T: key>(metadata: &object::Object<T>): &fungible_asset::Metadata
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun borrow_fungible_metadata<T: key>(
        metadata: &Object<T>
    ): &Metadata acquires Metadata {
        let addr = object::object_address(metadata);
        borrow_global<Metadata>(addr)
    }
}
```


<a id="0x1_fungible_asset_borrow_fungible_metadata_mut"></a>

## Function `borrow_fungible_metadata_mut`



```move
module 0x1::fungible_asset {
    fun borrow_fungible_metadata_mut<T: key>(metadata: &object::Object<T>): &mut fungible_asset::Metadata
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun borrow_fungible_metadata_mut<T: key>(
        metadata: &Object<T>
    ): &mut Metadata acquires Metadata {
        let addr = object::object_address(metadata);
        borrow_global_mut<Metadata>(addr)
    }
}
```


<a id="0x1_fungible_asset_borrow_store_resource"></a>

## Function `borrow_store_resource`



```move
module 0x1::fungible_asset {
    fun borrow_store_resource<T: key>(store: &object::Object<T>): &fungible_asset::FungibleStore
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    inline fun borrow_store_resource<T: key>(store: &Object<T>): &FungibleStore acquires FungibleStore {
        let store_addr = object::object_address(store);
        assert!(exists<FungibleStore>(store_addr), error::not_found(EFUNGIBLE_STORE_EXISTENCE));
        borrow_global<FungibleStore>(store_addr)
    }
}
```


<a id="0x1_fungible_asset_upgrade_to_concurrent"></a>

## Function `upgrade_to_concurrent`



```move
module 0x1::fungible_asset {
    public fun upgrade_to_concurrent(ref: &object::ExtendRef)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public fun upgrade_to_concurrent(
        ref: &ExtendRef,
    ) acquires Supply {
        let metadata_object_address = object::address_from_extend_ref(ref);
        let metadata_object_signer = object::generate_signer_for_extending(ref);
        assert!(
            features::concurrent_fungible_assets_enabled(),
            error::invalid_argument(ECONCURRENT_SUPPLY_NOT_ENABLED)
        );
        assert!(exists<Supply>(metadata_object_address), error::not_found(ESUPPLY_NOT_FOUND));
        let Supply {
            current,
            maximum,
        } = move_from<Supply>(metadata_object_address);

        let unlimited = option::is_none(&maximum);
        let supply = ConcurrentSupply {
            current: if (unlimited) {
                aggregator_v2::create_unbounded_aggregator_with_value(current)
            }
            else {
                aggregator_v2::create_aggregator_with_value(current, option::extract(&mut maximum))
            },
        };
        move_to(&metadata_object_signer, supply);
    }
}
```


<a id="0x1_fungible_asset_upgrade_store_to_concurrent"></a>

## Function `upgrade_store_to_concurrent`



```move
module 0x1::fungible_asset {
    public entry fun upgrade_store_to_concurrent<T: key>(owner: &signer, store: object::Object<T>)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    public entry fun upgrade_store_to_concurrent<T: key>(
        owner: &signer,
        store: Object<T>,
    ) acquires FungibleStore {
        assert!(object::owns(store, signer::address_of(owner)), error::permission_denied(ENOT_STORE_OWNER));
        assert!(!is_frozen(store), error::invalid_argument(ESTORE_IS_FROZEN));
        assert!(allow_upgrade_to_concurrent_fungible_balance(), error::invalid_argument(ECONCURRENT_BALANCE_NOT_ENABLED));
        ensure_store_upgraded_to_concurrent_internal(object::object_address(&store));
    }
}
```


<a id="0x1_fungible_asset_ensure_store_upgraded_to_concurrent_internal"></a>

## Function `ensure_store_upgraded_to_concurrent_internal`

Ensure a known `FungibleStore` has `ConcurrentFungibleBalance`.


```move
module 0x1::fungible_asset {
    fun ensure_store_upgraded_to_concurrent_internal(fungible_store_address: address)
}
```


##### Implementation


```move
module 0x1::fungible_asset {
    fun ensure_store_upgraded_to_concurrent_internal(
        fungible_store_address: address,
    ) acquires FungibleStore {
        if (exists<ConcurrentFungibleBalance>(fungible_store_address)) {
            return
        };
        let store = borrow_global_mut<FungibleStore>(fungible_store_address);
        let balance = aggregator_v2::create_unbounded_aggregator_with_value(store.balance);
        store.balance = 0;
        let object_signer = create_signer::create_signer(fungible_store_address);
        move_to(&object_signer, ConcurrentFungibleBalance { balance });
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
<td>The metadata associated with the fungible asset is subject to precise size constraints.</td>
<td>Medium</td>
<td>The add_fungibility function has size limitations for the name, symbol, number of decimals, icon_uri, and project_uri field of the Metadata resource.</td>
<td>This has been audited.</td>
</tr>

<tr>
<td>2</td>
<td>Adding fungibility to an existing object should initialize the metadata and supply resources and store them under the metadata object address.</td>
<td>Low</td>
<td>The add_fungibility function initializes the Metadata and Supply resources and moves them under the metadata object.</td>
<td>Audited that the Metadata and Supply resources are initialized properly.</td>
</tr>

<tr>
<td>3</td>
<td>Generating mint, burn and transfer references can only be done at object creation time and if the object was added fungibility.</td>
<td>Low</td>
<td>The following functions generate the related references of the Metadata object: 1. generate_mint_ref 2. generate_burn_ref 3. generate_transfer_ref</td>
<td>Audited that the Metadata object exists within the constructor ref.</td>
</tr>

<tr>
<td>4</td>
<td>Only the owner of a store should be allowed to withdraw fungible assets from it.</td>
<td>High</td>
<td>The fungible_asset::withdraw function ensures that the signer owns the store by asserting that the object address matches the address of the signer.</td>
<td>Audited that the address of the signer owns the object.</td>
</tr>

<tr>
<td>5</td>
<td>The transfer, withdrawal and deposit operation should never change the current supply of the fungible asset.</td>
<td>High</td>
<td>The transfer function withdraws the fungible assets from the store and deposits them to the receiver. The withdraw function extracts the fungible asset from the fungible asset store. The deposit function adds the balance to the fungible asset store.</td>
<td>Audited that the supply before and after the operation remains constant.</td>
</tr>

<tr>
<td>6</td>
<td>The owner of the store should only be able to withdraw a certain amount if its store has sufficient balance and is not frozen, unless the withdrawal is performed with a reference, and afterwards the store balance should be decreased.</td>
<td>High</td>
<td>The withdraw function ensures that the store is not frozen before calling withdraw_internal which ensures that the withdrawing amount is greater than 0 and less than the total balance from the store. The withdraw_with_ref ensures that the reference&apos;s metadata matches the store metadata.</td>
<td>Audited that it aborts if the withdrawing store is frozen. Audited that it aborts if the store doesn&apos;t have sufficient balance. Audited that the balance of the withdrawing store is reduced by amount.</td>
</tr>

<tr>
<td>7</td>
<td>Only the same type of fungible assets should be deposited in a fungible asset store, if the store is not frozen, unless the deposit is performed with a reference, and afterwards the store balance should be increased.</td>
<td>High</td>
<td>The deposit function ensures that store is not frozen and proceeds to call the deposit_internal function which validates the store&apos;s metadata and the depositing asset&apos;s metadata followed by increasing the store balance by the given amount. The deposit_with_ref ensures that the reference&apos;s metadata matches the depositing asset&apos;s metadata.</td>
<td>Audited that it aborts if the store is frozen. Audited that it aborts if the asset and asset store are different. Audited that the store&apos;s balance is increased by the deposited amount.</td>
</tr>

<tr>
<td>8</td>
<td>An object should only be allowed to hold one store for fungible assets.</td>
<td>Medium</td>
<td>The create_store function initializes a new FungibleStore resource and moves it under the object address.</td>
<td>Audited that the resource was moved under the object.</td>
</tr>

<tr>
<td>9</td>
<td>When a new store is created, the balance should be set by default to the value zero.</td>
<td>High</td>
<td>The create_store function initializes a new fungible asset store with zero balance and stores it under the given construtorRef object.</td>
<td>Audited that the store is properly initialized with zero balance.</td>
</tr>

<tr>
<td>10</td>
<td>A store should only be deleted if its balance is zero.</td>
<td>Medium</td>
<td>The remove_store function validates the store&apos;s balance and removes the store under the object address.</td>
<td>Audited that aborts if the balance of the store is not zero. Audited that store is removed from the object address.</td>
</tr>

<tr>
<td>11</td>
<td>Minting and burning should alter the total supply value, and the store balances.</td>
<td>High</td>
<td>The mint process increases the total supply by the amount minted using the increase_supply function. The burn process withdraws the burn amount from the given store and decreases the total supply by the amount burned using the decrease_supply function.</td>
<td>Audited the mint and burn functions that the supply was adjusted accordingly.</td>
</tr>

<tr>
<td>12</td>
<td>It must not be possible to burn an amount of fungible assets larger than their current supply.</td>
<td>High</td>
<td>The burn process ensures that the store has enough balance to burn, by asserting that the supply.current &gt;&#61; amount inside the decrease_supply function.</td>
<td>Audited that it aborts if the provided store doesn&apos;t have sufficient balance.</td>
</tr>

<tr>
<td>13</td>
<td>Enabling or disabling store&apos;s frozen status should only be done with a valid transfer reference.</td>
<td>High</td>
<td>The set_frozen_flag function ensures that the TransferRef is provided via function argument and that the store&apos;s metadata matches the metadata from the reference. It then proceeds to update the frozen flag of the store.</td>
<td>Audited that it aborts if the metadata doesn&apos;t match. Audited that the frozen flag is updated properly.</td>
</tr>

<tr>
<td>14</td>
<td>Extracting a specific amount from the fungible asset should be possible only if the total amount that it holds is greater or equal to the provided amount.</td>
<td>High</td>
<td>The extract function validates that the fungible asset has enough balance to extract and then updates it by subtracting the extracted amount.</td>
<td>Audited that it aborts if the asset didn&apos;t have sufficient balance. Audited that the balance of the asset is updated. Audited that the extract function returns the extracted asset.</td>
</tr>

<tr>
<td>15</td>
<td>Merging two fungible assets should only be possible if both share the same metadata.</td>
<td>Medium</td>
<td>The merge function validates the metadata of the src and dst asset.</td>
<td>Audited that it aborts if the metadata of the src and dst are not the same.</td>
</tr>

<tr>
<td>16</td>
<td>Post merging two fungible assets, the source asset should have the amount value equal to the sum of the two.</td>
<td>High</td>
<td>The merge function increases dst_fungible_asset.amount by src_fungible_asset.amount.</td>
<td>Audited that the dst_fungible_asset balance is increased by amount.</td>
</tr>

<tr>
<td>17</td>
<td>Fungible assets with zero balance should be destroyed when the amount reaches value 0.</td>
<td>Medium</td>
<td>The destroy_zero ensures that the balance of the asset has the value 0 and destroy the asset.</td>
<td>Audited that it aborts if the balance of the asset is non zero.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::fungible_asset {
    pragma verify=false;
}
```
