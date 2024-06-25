
<a id="0x1_coin"></a>

# Module `0x1::coin`

This module provides the foundation for typesafe Coins.


-  [Struct `Coin`](#0x1_coin_Coin)
-  [Struct `AggregatableCoin`](#0x1_coin_AggregatableCoin)
-  [Resource `CoinStore`](#0x1_coin_CoinStore)
-  [Resource `SupplyConfig`](#0x1_coin_SupplyConfig)
-  [Resource `CoinInfo`](#0x1_coin_CoinInfo)
-  [Struct `CoinDeposit`](#0x1_coin_CoinDeposit)
-  [Struct `CoinWithdraw`](#0x1_coin_CoinWithdraw)
-  [Struct `Deposit`](#0x1_coin_Deposit)
-  [Struct `Withdraw`](#0x1_coin_Withdraw)
-  [Struct `DepositEvent`](#0x1_coin_DepositEvent)
-  [Struct `WithdrawEvent`](#0x1_coin_WithdrawEvent)
-  [Struct `CoinEventHandleDeletion`](#0x1_coin_CoinEventHandleDeletion)
-  [Struct `PairCreation`](#0x1_coin_PairCreation)
-  [Resource `MigrationFlag`](#0x1_coin_MigrationFlag)
-  [Struct `MintCapability`](#0x1_coin_MintCapability)
-  [Struct `FreezeCapability`](#0x1_coin_FreezeCapability)
-  [Struct `BurnCapability`](#0x1_coin_BurnCapability)
-  [Resource `CoinConversionMap`](#0x1_coin_CoinConversionMap)
-  [Resource `PairedCoinType`](#0x1_coin_PairedCoinType)
-  [Resource `PairedFungibleAssetRefs`](#0x1_coin_PairedFungibleAssetRefs)
-  [Struct `MintRefReceipt`](#0x1_coin_MintRefReceipt)
-  [Struct `TransferRefReceipt`](#0x1_coin_TransferRefReceipt)
-  [Struct `BurnRefReceipt`](#0x1_coin_BurnRefReceipt)
-  [Resource `Ghost$supply`](#0x1_coin_Ghost$supply)
-  [Resource `Ghost$aggregate_supply`](#0x1_coin_Ghost$aggregate_supply)
-  [Constants](#@Constants_0)
-  [Function `paired_metadata`](#0x1_coin_paired_metadata)
-  [Function `create_coin_conversion_map`](#0x1_coin_create_coin_conversion_map)
-  [Function `create_pairing`](#0x1_coin_create_pairing)
-  [Function `is_apt`](#0x1_coin_is_apt)
-  [Function `create_and_return_paired_metadata_if_not_exist`](#0x1_coin_create_and_return_paired_metadata_if_not_exist)
-  [Function `ensure_paired_metadata`](#0x1_coin_ensure_paired_metadata)
-  [Function `paired_coin`](#0x1_coin_paired_coin)
-  [Function `coin_to_fungible_asset`](#0x1_coin_coin_to_fungible_asset)
-  [Function `fungible_asset_to_coin`](#0x1_coin_fungible_asset_to_coin)
-  [Function `assert_paired_metadata_exists`](#0x1_coin_assert_paired_metadata_exists)
-  [Function `paired_mint_ref_exists`](#0x1_coin_paired_mint_ref_exists)
-  [Function `get_paired_mint_ref`](#0x1_coin_get_paired_mint_ref)
-  [Function `return_paired_mint_ref`](#0x1_coin_return_paired_mint_ref)
-  [Function `paired_transfer_ref_exists`](#0x1_coin_paired_transfer_ref_exists)
-  [Function `get_paired_transfer_ref`](#0x1_coin_get_paired_transfer_ref)
-  [Function `return_paired_transfer_ref`](#0x1_coin_return_paired_transfer_ref)
-  [Function `paired_burn_ref_exists`](#0x1_coin_paired_burn_ref_exists)
-  [Function `get_paired_burn_ref`](#0x1_coin_get_paired_burn_ref)
-  [Function `convert_and_take_paired_burn_ref`](#0x1_coin_convert_and_take_paired_burn_ref)
-  [Function `return_paired_burn_ref`](#0x1_coin_return_paired_burn_ref)
-  [Function `borrow_paired_burn_ref`](#0x1_coin_borrow_paired_burn_ref)
-  [Function `initialize_supply_config`](#0x1_coin_initialize_supply_config)
-  [Function `allow_supply_upgrades`](#0x1_coin_allow_supply_upgrades)
-  [Function `initialize_aggregatable_coin`](#0x1_coin_initialize_aggregatable_coin)
-  [Function `is_aggregatable_coin_zero`](#0x1_coin_is_aggregatable_coin_zero)
-  [Function `drain_aggregatable_coin`](#0x1_coin_drain_aggregatable_coin)
-  [Function `merge_aggregatable_coin`](#0x1_coin_merge_aggregatable_coin)
-  [Function `collect_into_aggregatable_coin`](#0x1_coin_collect_into_aggregatable_coin)
-  [Function `calculate_amount_to_withdraw`](#0x1_coin_calculate_amount_to_withdraw)
-  [Function `maybe_convert_to_fungible_store`](#0x1_coin_maybe_convert_to_fungible_store)
-  [Function `migrate_to_fungible_store`](#0x1_coin_migrate_to_fungible_store)
-  [Function `coin_address`](#0x1_coin_coin_address)
-  [Function `balance`](#0x1_coin_balance)
-  [Function `is_balance_at_least`](#0x1_coin_is_balance_at_least)
-  [Function `coin_balance`](#0x1_coin_coin_balance)
-  [Function `is_coin_initialized`](#0x1_coin_is_coin_initialized)
-  [Function `is_coin_store_frozen`](#0x1_coin_is_coin_store_frozen)
-  [Function `is_account_registered`](#0x1_coin_is_account_registered)
-  [Function `name`](#0x1_coin_name)
-  [Function `symbol`](#0x1_coin_symbol)
-  [Function `decimals`](#0x1_coin_decimals)
-  [Function `supply`](#0x1_coin_supply)
-  [Function `coin_supply`](#0x1_coin_coin_supply)
-  [Function `burn`](#0x1_coin_burn)
-  [Function `burn_from`](#0x1_coin_burn_from)
-  [Function `deposit`](#0x1_coin_deposit)
-  [Function `migrated_primary_fungible_store_exists`](#0x1_coin_migrated_primary_fungible_store_exists)
-  [Function `force_deposit`](#0x1_coin_force_deposit)
-  [Function `destroy_zero`](#0x1_coin_destroy_zero)
-  [Function `extract`](#0x1_coin_extract)
-  [Function `extract_all`](#0x1_coin_extract_all)
-  [Function `freeze_coin_store`](#0x1_coin_freeze_coin_store)
-  [Function `unfreeze_coin_store`](#0x1_coin_unfreeze_coin_store)
-  [Function `upgrade_supply`](#0x1_coin_upgrade_supply)
-  [Function `initialize`](#0x1_coin_initialize)
-  [Function `initialize_with_parallelizable_supply`](#0x1_coin_initialize_with_parallelizable_supply)
-  [Function `initialize_internal`](#0x1_coin_initialize_internal)
-  [Function `merge`](#0x1_coin_merge)
-  [Function `mint`](#0x1_coin_mint)
-  [Function `register`](#0x1_coin_register)
-  [Function `transfer`](#0x1_coin_transfer)
-  [Function `value`](#0x1_coin_value)
-  [Function `withdraw`](#0x1_coin_withdraw)
-  [Function `zero`](#0x1_coin_zero)
-  [Function `destroy_freeze_cap`](#0x1_coin_destroy_freeze_cap)
-  [Function `destroy_mint_cap`](#0x1_coin_destroy_mint_cap)
-  [Function `destroy_burn_cap`](#0x1_coin_destroy_burn_cap)
-  [Function `mint_internal`](#0x1_coin_mint_internal)
-  [Function `burn_internal`](#0x1_coin_burn_internal)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Struct `AggregatableCoin`](#@Specification_1_AggregatableCoin)
    -  [Function `coin_to_fungible_asset`](#@Specification_1_coin_to_fungible_asset)
    -  [Function `fungible_asset_to_coin`](#@Specification_1_fungible_asset_to_coin)
    -  [Function `initialize_supply_config`](#@Specification_1_initialize_supply_config)
    -  [Function `allow_supply_upgrades`](#@Specification_1_allow_supply_upgrades)
    -  [Function `initialize_aggregatable_coin`](#@Specification_1_initialize_aggregatable_coin)
    -  [Function `is_aggregatable_coin_zero`](#@Specification_1_is_aggregatable_coin_zero)
    -  [Function `drain_aggregatable_coin`](#@Specification_1_drain_aggregatable_coin)
    -  [Function `merge_aggregatable_coin`](#@Specification_1_merge_aggregatable_coin)
    -  [Function `collect_into_aggregatable_coin`](#@Specification_1_collect_into_aggregatable_coin)
    -  [Function `maybe_convert_to_fungible_store`](#@Specification_1_maybe_convert_to_fungible_store)
    -  [Function `coin_address`](#@Specification_1_coin_address)
    -  [Function `balance`](#@Specification_1_balance)
    -  [Function `is_coin_initialized`](#@Specification_1_is_coin_initialized)
    -  [Function `is_account_registered`](#@Specification_1_is_account_registered)
    -  [Function `name`](#@Specification_1_name)
    -  [Function `symbol`](#@Specification_1_symbol)
    -  [Function `decimals`](#@Specification_1_decimals)
    -  [Function `supply`](#@Specification_1_supply)
    -  [Function `coin_supply`](#@Specification_1_coin_supply)
    -  [Function `burn`](#@Specification_1_burn)
    -  [Function `burn_from`](#@Specification_1_burn_from)
    -  [Function `deposit`](#@Specification_1_deposit)
    -  [Function `force_deposit`](#@Specification_1_force_deposit)
    -  [Function `destroy_zero`](#@Specification_1_destroy_zero)
    -  [Function `extract`](#@Specification_1_extract)
    -  [Function `extract_all`](#@Specification_1_extract_all)
    -  [Function `freeze_coin_store`](#@Specification_1_freeze_coin_store)
    -  [Function `unfreeze_coin_store`](#@Specification_1_unfreeze_coin_store)
    -  [Function `upgrade_supply`](#@Specification_1_upgrade_supply)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `initialize_with_parallelizable_supply`](#@Specification_1_initialize_with_parallelizable_supply)
    -  [Function `initialize_internal`](#@Specification_1_initialize_internal)
    -  [Function `merge`](#@Specification_1_merge)
    -  [Function `mint`](#@Specification_1_mint)
    -  [Function `register`](#@Specification_1_register)
    -  [Function `transfer`](#@Specification_1_transfer)
    -  [Function `withdraw`](#@Specification_1_withdraw)
    -  [Function `mint_internal`](#@Specification_1_mint_internal)
    -  [Function `burn_internal`](#@Specification_1_burn_internal)


```move
module 0x1::coin {
    use 0x1::account;
    use 0x1::aggregator;
    use 0x1::aggregator_factory;
    use 0x1::create_signer;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::fungible_asset;
    use 0x1::guid;
    use 0x1::object;
    use 0x1::option;
    use 0x1::optional_aggregator;
    use 0x1::primary_fungible_store;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::table;
    use 0x1::type_info;
}
```


<a id="0x1_coin_Coin"></a>

## Struct `Coin`

Core data structures
Main structure representing a coin/token in an account&apos;s custody.


```move
module 0x1::coin {
    struct Coin<CoinType> has store
}
```


##### Fields


<dl>
<dt>
`value: u64`
</dt>
<dd>
 Amount of coin this address has.
</dd>
</dl>


<a id="0x1_coin_AggregatableCoin"></a>

## Struct `AggregatableCoin`

Represents a coin with aggregator as its value. This allows to update
the coin in every transaction avoiding read&#45;modify&#45;write conflicts. Only
used for gas fees distribution by Aptos Framework (0x1).


```move
module 0x1::coin {
    struct AggregatableCoin<CoinType> has store
}
```


##### Fields


<dl>
<dt>
`value: aggregator::Aggregator`
</dt>
<dd>
 Amount of aggregatable coin this address has.
</dd>
</dl>


<a id="0x1_coin_CoinStore"></a>

## Resource `CoinStore`

A holder of a specific coin types and associated event handles.
These are kept in a single resource to ensure locality of data.


```move
module 0x1::coin {
    struct CoinStore<CoinType> has key
}
```


##### Fields


<dl>
<dt>
`coin: coin::Coin<CoinType>`
</dt>
<dd>

</dd>
<dt>
`frozen: bool`
</dt>
<dd>

</dd>
<dt>
`deposit_events: event::EventHandle<coin::DepositEvent>`
</dt>
<dd>

</dd>
<dt>
`withdraw_events: event::EventHandle<coin::WithdrawEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_SupplyConfig"></a>

## Resource `SupplyConfig`

Configuration that controls the behavior of total coin supply. If the field
is set, coin creators are allowed to upgrade to parallelizable implementations.


```move
module 0x1::coin {
    struct SupplyConfig has key
}
```


##### Fields


<dl>
<dt>
`allow_upgrades: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_CoinInfo"></a>

## Resource `CoinInfo`

Information about a specific coin type. Stored on the creator of the coin&apos;s account.


```move
module 0x1::coin {
    struct CoinInfo<CoinType> has key
}
```


##### Fields


<dl>
<dt>
`name: string::String`
</dt>
<dd>

</dd>
<dt>
`symbol: string::String`
</dt>
<dd>
 Symbol of the coin, usually a shorter version of the name.
 For example, Singapore Dollar is SGD.
</dd>
<dt>
`decimals: u8`
</dt>
<dd>
 Number of decimals used to get its user representation.
 For example, if `decimals` equals `2`, a balance of `505` coins should
 be displayed to a user as `5.05` (`505 / 10 ** 2`).
</dd>
<dt>
`supply: option::Option<optional_aggregator::OptionalAggregator>`
</dt>
<dd>
 Amount of this coin type in existence.
</dd>
</dl>


<a id="0x1_coin_CoinDeposit"></a>

## Struct `CoinDeposit`

Module event emitted when some amount of a coin is deposited into an account.


```move
module 0x1::coin {
    #[event]
    struct CoinDeposit has drop, store
}
```


##### Fields


<dl>
<dt>
`coin_type: string::String`
</dt>
<dd>

</dd>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_CoinWithdraw"></a>

## Struct `CoinWithdraw`

Module event emitted when some amount of a coin is withdrawn from an account.


```move
module 0x1::coin {
    #[event]
    struct CoinWithdraw has drop, store
}
```


##### Fields


<dl>
<dt>
`coin_type: string::String`
</dt>
<dd>

</dd>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_Deposit"></a>

## Struct `Deposit`



```move
module 0x1::coin {
    #[event]
    #[deprecated]
    struct Deposit<CoinType> has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_Withdraw"></a>

## Struct `Withdraw`



```move
module 0x1::coin {
    #[event]
    #[deprecated]
    struct Withdraw<CoinType> has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_DepositEvent"></a>

## Struct `DepositEvent`

Event emitted when some amount of a coin is deposited into an account.


```move
module 0x1::coin {
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


<a id="0x1_coin_WithdrawEvent"></a>

## Struct `WithdrawEvent`

Event emitted when some amount of a coin is withdrawn from an account.


```move
module 0x1::coin {
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


<a id="0x1_coin_CoinEventHandleDeletion"></a>

## Struct `CoinEventHandleDeletion`

Module event emitted when the event handles related to coin store is deleted.


```move
module 0x1::coin {
    #[event]
    struct CoinEventHandleDeletion has drop, store
}
```


##### Fields


<dl>
<dt>
`event_handle_creation_address: address`
</dt>
<dd>

</dd>
<dt>
`deleted_deposit_event_handle_creation_number: u64`
</dt>
<dd>

</dd>
<dt>
`deleted_withdraw_event_handle_creation_number: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_PairCreation"></a>

## Struct `PairCreation`

Module event emitted when a new pair of coin and fungible asset is created.


```move
module 0x1::coin {
    #[event]
    struct PairCreation has drop, store
}
```


##### Fields


<dl>
<dt>
`coin_type: type_info::TypeInfo`
</dt>
<dd>

</dd>
<dt>
`fungible_asset_metadata_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_MigrationFlag"></a>

## Resource `MigrationFlag`

The flag the existence of which indicates the primary fungible store is created by the migration from CoinStore.


```move
module 0x1::coin {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct MigrationFlag has key
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


<a id="0x1_coin_MintCapability"></a>

## Struct `MintCapability`

Capability required to mint coins.


```move
module 0x1::coin {
    struct MintCapability<CoinType> has copy, store
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


<a id="0x1_coin_FreezeCapability"></a>

## Struct `FreezeCapability`

Capability required to freeze a coin store.


```move
module 0x1::coin {
    struct FreezeCapability<CoinType> has copy, store
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


<a id="0x1_coin_BurnCapability"></a>

## Struct `BurnCapability`

Capability required to burn coins.


```move
module 0x1::coin {
    struct BurnCapability<CoinType> has copy, store
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


<a id="0x1_coin_CoinConversionMap"></a>

## Resource `CoinConversionMap`

The mapping between coin and fungible asset.


```move
module 0x1::coin {
    struct CoinConversionMap has key
}
```


##### Fields


<dl>
<dt>
`coin_to_fungible_asset_map: table::Table<type_info::TypeInfo, object::Object<fungible_asset::Metadata>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_PairedCoinType"></a>

## Resource `PairedCoinType`

The paired coin type info stored in fungible asset metadata object.


```move
module 0x1::coin {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct PairedCoinType has key
}
```


##### Fields


<dl>
<dt>
`type: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_PairedFungibleAssetRefs"></a>

## Resource `PairedFungibleAssetRefs`

The refs of the paired fungible asset.


```move
module 0x1::coin {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct PairedFungibleAssetRefs has key
}
```


##### Fields


<dl>
<dt>
`mint_ref_opt: option::Option<fungible_asset::MintRef>`
</dt>
<dd>

</dd>
<dt>
`transfer_ref_opt: option::Option<fungible_asset::TransferRef>`
</dt>
<dd>

</dd>
<dt>
`burn_ref_opt: option::Option<fungible_asset::BurnRef>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_MintRefReceipt"></a>

## Struct `MintRefReceipt`

The hot potato receipt for flash borrowing MintRef.


```move
module 0x1::coin {
    struct MintRefReceipt
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


<a id="0x1_coin_TransferRefReceipt"></a>

## Struct `TransferRefReceipt`

The hot potato receipt for flash borrowing TransferRef.


```move
module 0x1::coin {
    struct TransferRefReceipt
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


<a id="0x1_coin_BurnRefReceipt"></a>

## Struct `BurnRefReceipt`

The hot potato receipt for flash borrowing BurnRef.


```move
module 0x1::coin {
    struct BurnRefReceipt
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


<a id="0x1_coin_Ghost$supply"></a>

## Resource `Ghost$supply`



```move
module 0x1::coin {
    struct Ghost$supply<CoinType> has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: num`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_Ghost$aggregate_supply"></a>

## Resource `Ghost$aggregate_supply`



```move
module 0x1::coin {
    struct Ghost$aggregate_supply<CoinType> has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`v: num`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_coin_MAX_U64"></a>

Maximum possible aggregatable coin value.


```move
module 0x1::coin {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_coin_MAX_U128"></a>

Maximum possible coin supply.


```move
module 0x1::coin {
    const MAX_U128: u128 = 340282366920938463463374607431768211455;
}
```


<a id="0x1_coin_EINSUFFICIENT_BALANCE"></a>

Not enough coins to complete transaction


```move
module 0x1::coin {
    const EINSUFFICIENT_BALANCE: u64 = 6;
}
```


<a id="0x1_coin_EAGGREGATABLE_COIN_VALUE_TOO_LARGE"></a>

The value of aggregatable coin used for transaction fees redistribution does not fit in u64.


```move
module 0x1::coin {
    const EAGGREGATABLE_COIN_VALUE_TOO_LARGE: u64 = 14;
}
```


<a id="0x1_coin_EAPT_PAIRING_IS_NOT_ENABLED"></a>

APT pairing is not eanbled yet.


```move
module 0x1::coin {
    const EAPT_PAIRING_IS_NOT_ENABLED: u64 = 28;
}
```


<a id="0x1_coin_EBURN_REF_NOT_FOUND"></a>

The BurnRef does not exist.


```move
module 0x1::coin {
    const EBURN_REF_NOT_FOUND: u64 = 25;
}
```


<a id="0x1_coin_EBURN_REF_RECEIPT_MISMATCH"></a>

The BurnRefReceipt does not match the BurnRef to be returned.


```move
module 0x1::coin {
    const EBURN_REF_RECEIPT_MISMATCH: u64 = 24;
}
```


<a id="0x1_coin_ECOIN_CONVERSION_MAP_NOT_FOUND"></a>

The coin converison map is not created yet.


```move
module 0x1::coin {
    const ECOIN_CONVERSION_MAP_NOT_FOUND: u64 = 27;
}
```


<a id="0x1_coin_ECOIN_INFO_ADDRESS_MISMATCH"></a>

Address of account which is used to initialize a coin `CoinType` doesn&apos;t match the deployer of module


```move
module 0x1::coin {
    const ECOIN_INFO_ADDRESS_MISMATCH: u64 = 1;
}
```


<a id="0x1_coin_ECOIN_INFO_ALREADY_PUBLISHED"></a>

`CoinType` is already initialized as a coin


```move
module 0x1::coin {
    const ECOIN_INFO_ALREADY_PUBLISHED: u64 = 2;
}
```


<a id="0x1_coin_ECOIN_INFO_NOT_PUBLISHED"></a>

`CoinType` hasn&apos;t been initialized as a coin


```move
module 0x1::coin {
    const ECOIN_INFO_NOT_PUBLISHED: u64 = 3;
}
```


<a id="0x1_coin_ECOIN_NAME_TOO_LONG"></a>

Name of the coin is too long


```move
module 0x1::coin {
    const ECOIN_NAME_TOO_LONG: u64 = 12;
}
```


<a id="0x1_coin_ECOIN_STORE_ALREADY_PUBLISHED"></a>

Deprecated. Account already has `CoinStore` registered for `CoinType`


```move
module 0x1::coin {
    const ECOIN_STORE_ALREADY_PUBLISHED: u64 = 4;
}
```


<a id="0x1_coin_ECOIN_STORE_NOT_PUBLISHED"></a>

Account hasn&apos;t registered `CoinStore` for `CoinType`


```move
module 0x1::coin {
    const ECOIN_STORE_NOT_PUBLISHED: u64 = 5;
}
```


<a id="0x1_coin_ECOIN_SUPPLY_UPGRADE_NOT_SUPPORTED"></a>

Cannot upgrade the total supply of coins to different implementation.


```move
module 0x1::coin {
    const ECOIN_SUPPLY_UPGRADE_NOT_SUPPORTED: u64 = 11;
}
```


<a id="0x1_coin_ECOIN_SYMBOL_TOO_LONG"></a>

Symbol of the coin is too long


```move
module 0x1::coin {
    const ECOIN_SYMBOL_TOO_LONG: u64 = 13;
}
```


<a id="0x1_coin_ECOIN_TO_FUNGIBLE_ASSET_FEATURE_NOT_ENABLED"></a>

The feature of migration from coin to fungible asset is not enabled.


```move
module 0x1::coin {
    const ECOIN_TO_FUNGIBLE_ASSET_FEATURE_NOT_ENABLED: u64 = 18;
}
```


<a id="0x1_coin_ECOIN_TYPE_MISMATCH"></a>

The coin type from the map does not match the calling function type argument.


```move
module 0x1::coin {
    const ECOIN_TYPE_MISMATCH: u64 = 17;
}
```


<a id="0x1_coin_EDESTRUCTION_OF_NONZERO_TOKEN"></a>

Cannot destroy non&#45;zero coins


```move
module 0x1::coin {
    const EDESTRUCTION_OF_NONZERO_TOKEN: u64 = 7;
}
```


<a id="0x1_coin_EFROZEN"></a>

CoinStore is frozen. Coins cannot be deposited or withdrawn


```move
module 0x1::coin {
    const EFROZEN: u64 = 10;
}
```


<a id="0x1_coin_EMIGRATION_FRAMEWORK_NOT_ENABLED"></a>

The migration process from coin to fungible asset is not enabled yet.


```move
module 0x1::coin {
    const EMIGRATION_FRAMEWORK_NOT_ENABLED: u64 = 26;
}
```


<a id="0x1_coin_EMINT_REF_NOT_FOUND"></a>

The MintRef does not exist.


```move
module 0x1::coin {
    const EMINT_REF_NOT_FOUND: u64 = 21;
}
```


<a id="0x1_coin_EMINT_REF_RECEIPT_MISMATCH"></a>

The MintRefReceipt does not match the MintRef to be returned.


```move
module 0x1::coin {
    const EMINT_REF_RECEIPT_MISMATCH: u64 = 20;
}
```


<a id="0x1_coin_EPAIRED_COIN"></a>

Error regarding paired coin type of the fungible asset metadata.


```move
module 0x1::coin {
    const EPAIRED_COIN: u64 = 15;
}
```


<a id="0x1_coin_EPAIRED_FUNGIBLE_ASSET"></a>

Error regarding paired fungible asset metadata of a coin type.


```move
module 0x1::coin {
    const EPAIRED_FUNGIBLE_ASSET: u64 = 16;
}
```


<a id="0x1_coin_EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND"></a>

PairedFungibleAssetRefs resource does not exist.


```move
module 0x1::coin {
    const EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND: u64 = 19;
}
```


<a id="0x1_coin_ETRANSFER_REF_NOT_FOUND"></a>

The TransferRef does not exist.


```move
module 0x1::coin {
    const ETRANSFER_REF_NOT_FOUND: u64 = 23;
}
```


<a id="0x1_coin_ETRANSFER_REF_RECEIPT_MISMATCH"></a>

The TransferRefReceipt does not match the TransferRef to be returned.


```move
module 0x1::coin {
    const ETRANSFER_REF_RECEIPT_MISMATCH: u64 = 22;
}
```


<a id="0x1_coin_MAX_COIN_NAME_LENGTH"></a>



```move
module 0x1::coin {
    const MAX_COIN_NAME_LENGTH: u64 = 32;
}
```


<a id="0x1_coin_MAX_COIN_SYMBOL_LENGTH"></a>



```move
module 0x1::coin {
    const MAX_COIN_SYMBOL_LENGTH: u64 = 10;
}
```


<a id="0x1_coin_paired_metadata"></a>

## Function `paired_metadata`

Get the paired fungible asset metadata object of a coin type. If not exist, return option::none().


```move
module 0x1::coin {
    #[view]
    public fun paired_metadata<CoinType>(): option::Option<object::Object<fungible_asset::Metadata>>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun paired_metadata<CoinType>(): Option<Object<Metadata>> acquires CoinConversionMap {
        if (exists<CoinConversionMap>(@aptos_framework) && features::coin_to_fungible_asset_migration_feature_enabled(
        )) {
            let map = &borrow_global<CoinConversionMap>(@aptos_framework).coin_to_fungible_asset_map;
            let type = type_info::type_of<CoinType>();
            if (table::contains(map, type)) {
                return option::some(*table::borrow(map, type))
            }
        };
        option::none()
    }
}
```


<a id="0x1_coin_create_coin_conversion_map"></a>

## Function `create_coin_conversion_map`



```move
module 0x1::coin {
    public entry fun create_coin_conversion_map(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun create_coin_conversion_map(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        if (!exists<CoinConversionMap>(@aptos_framework)) {
            move_to(aptos_framework, CoinConversionMap {
                coin_to_fungible_asset_map: table::new(),
            })
        };
    }
}
```


<a id="0x1_coin_create_pairing"></a>

## Function `create_pairing`

Create APT pairing by passing `AptosCoin`.


```move
module 0x1::coin {
    public entry fun create_pairing<CoinType>(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun create_pairing<CoinType>(
        aptos_framework: &signer
    ) acquires CoinConversionMap, CoinInfo {
        system_addresses::assert_aptos_framework(aptos_framework);
        create_and_return_paired_metadata_if_not_exist<CoinType>(true);
    }
}
```


<a id="0x1_coin_is_apt"></a>

## Function `is_apt`



```move
module 0x1::coin {
    fun is_apt<CoinType>(): bool
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun is_apt<CoinType>(): bool {
        type_info::type_name<CoinType>() == string::utf8(b"0x1::aptos_coin::AptosCoin")
    }
}
```


<a id="0x1_coin_create_and_return_paired_metadata_if_not_exist"></a>

## Function `create_and_return_paired_metadata_if_not_exist`



```move
module 0x1::coin {
    fun create_and_return_paired_metadata_if_not_exist<CoinType>(allow_apt_creation: bool): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun create_and_return_paired_metadata_if_not_exist<CoinType>(allow_apt_creation: bool): Object<Metadata> {
        assert!(
            features::coin_to_fungible_asset_migration_feature_enabled(),
            error::invalid_state(EMIGRATION_FRAMEWORK_NOT_ENABLED)
        );
        assert!(exists<CoinConversionMap>(@aptos_framework), error::not_found(ECOIN_CONVERSION_MAP_NOT_FOUND));
        let map = borrow_global_mut<CoinConversionMap>(@aptos_framework);
        let type = type_info::type_of<CoinType>();
        if (!table::contains(&map.coin_to_fungible_asset_map, type)) {
            let is_apt = is_apt<CoinType>();
            assert!(!is_apt || allow_apt_creation, error::invalid_state(EAPT_PAIRING_IS_NOT_ENABLED));
            let metadata_object_cref =
                if (is_apt) {
                    object::create_sticky_object_at_address(@aptos_framework, @aptos_fungible_asset)
                } else {
                    object::create_named_object(
                        &create_signer::create_signer(@aptos_fungible_asset),
                        *string::bytes(&type_info::type_name<CoinType>())
                    )
                };
            primary_fungible_store::create_primary_store_enabled_fungible_asset(
                &metadata_object_cref,
                option::map(coin_supply<CoinType>(), |_| MAX_U128),
                name<CoinType>(),
                symbol<CoinType>(),
                decimals<CoinType>(),
                string::utf8(b""),
                string::utf8(b""),
            );

            let metadata_object_signer = &object::generate_signer(&metadata_object_cref);
            let type = type_info::type_of<CoinType>();
            move_to(metadata_object_signer, PairedCoinType { type });
            let metadata_obj = object::object_from_constructor_ref(&metadata_object_cref);

            table::add(&mut map.coin_to_fungible_asset_map, type, metadata_obj);
            event::emit(PairCreation {
                coin_type: type,
                fungible_asset_metadata_address: object_address(&metadata_obj)
            });

            // Generates all three refs
            let mint_ref = fungible_asset::generate_mint_ref(&metadata_object_cref);
            let transfer_ref = fungible_asset::generate_transfer_ref(&metadata_object_cref);
            let burn_ref = fungible_asset::generate_burn_ref(&metadata_object_cref);
            move_to(metadata_object_signer,
                PairedFungibleAssetRefs {
                    mint_ref_opt: option::some(mint_ref),
                    transfer_ref_opt: option::some(transfer_ref),
                    burn_ref_opt: option::some(burn_ref),
                }
            );
        };
        *table::borrow(&map.coin_to_fungible_asset_map, type)
    }
}
```


<a id="0x1_coin_ensure_paired_metadata"></a>

## Function `ensure_paired_metadata`

Get the paired fungible asset metadata object of a coin type, create if not exist.


```move
module 0x1::coin {
    public(friend) fun ensure_paired_metadata<CoinType>(): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun ensure_paired_metadata<CoinType>(): Object<Metadata> acquires CoinConversionMap, CoinInfo {
        create_and_return_paired_metadata_if_not_exist<CoinType>(false)
    }
}
```


<a id="0x1_coin_paired_coin"></a>

## Function `paired_coin`

Get the paired coin type of a fungible asset metadata object.


```move
module 0x1::coin {
    #[view]
    public fun paired_coin(metadata: object::Object<fungible_asset::Metadata>): option::Option<type_info::TypeInfo>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun paired_coin(metadata: Object<Metadata>): Option<TypeInfo> acquires PairedCoinType {
        let metadata_addr = object::object_address(&metadata);
        if (exists<PairedCoinType>(metadata_addr)) {
            option::some(borrow_global<PairedCoinType>(metadata_addr).type)
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_coin_coin_to_fungible_asset"></a>

## Function `coin_to_fungible_asset`

Conversion from coin to fungible asset


```move
module 0x1::coin {
    public fun coin_to_fungible_asset<CoinType>(coin: coin::Coin<CoinType>): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::coin {
    public fun coin_to_fungible_asset<CoinType>(
        coin: Coin<CoinType>
    ): FungibleAsset acquires CoinConversionMap, CoinInfo {
        let metadata = ensure_paired_metadata<CoinType>();
        let amount = burn_internal(coin);
        fungible_asset::mint_internal(metadata, amount)
    }
}
```


<a id="0x1_coin_fungible_asset_to_coin"></a>

## Function `fungible_asset_to_coin`

Conversion from fungible asset to coin. Not public to push the migration to FA.


```move
module 0x1::coin {
    fun fungible_asset_to_coin<CoinType>(fungible_asset: fungible_asset::FungibleAsset): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    fun fungible_asset_to_coin<CoinType>(
        fungible_asset: FungibleAsset
    ): Coin<CoinType> acquires CoinInfo, PairedCoinType {
        let metadata_addr = object::object_address(&fungible_asset::metadata_from_asset(&fungible_asset));
        assert!(
            object::object_exists<PairedCoinType>(metadata_addr),
            error::not_found(EPAIRED_COIN)
        );
        let coin_type_info = borrow_global<PairedCoinType>(metadata_addr).type;
        assert!(coin_type_info == type_info::type_of<CoinType>(), error::invalid_argument(ECOIN_TYPE_MISMATCH));
        let amount = fungible_asset::burn_internal(fungible_asset);
        mint_internal<CoinType>(amount)
    }
}
```


<a id="0x1_coin_assert_paired_metadata_exists"></a>

## Function `assert_paired_metadata_exists`



```move
module 0x1::coin {
    fun assert_paired_metadata_exists<CoinType>(): object::Object<fungible_asset::Metadata>
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun assert_paired_metadata_exists<CoinType>(): Object<Metadata> {
        let metadata_opt = paired_metadata<CoinType>();
        assert!(option::is_some(&metadata_opt), error::not_found(EPAIRED_FUNGIBLE_ASSET));
        option::destroy_some(metadata_opt)
    }
}
```


<a id="0x1_coin_paired_mint_ref_exists"></a>

## Function `paired_mint_ref_exists`

Check whether `MintRef` has not been taken.


```move
module 0x1::coin {
    #[view]
    public fun paired_mint_ref_exists<CoinType>(): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun paired_mint_ref_exists<CoinType>(): bool acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        option::is_some(&borrow_global<PairedFungibleAssetRefs>(metadata_addr).mint_ref_opt)
    }
}
```


<a id="0x1_coin_get_paired_mint_ref"></a>

## Function `get_paired_mint_ref`

Get the `MintRef` of paired fungible asset of a coin type from `MintCapability`.


```move
module 0x1::coin {
    public fun get_paired_mint_ref<CoinType>(_: &coin::MintCapability<CoinType>): (fungible_asset::MintRef, coin::MintRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun get_paired_mint_ref<CoinType>(
        _: &MintCapability<CoinType>
    ): (MintRef, MintRefReceipt) acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        let mint_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).mint_ref_opt;
        assert!(option::is_some(mint_ref_opt), error::not_found(EMINT_REF_NOT_FOUND));
        (option::extract(mint_ref_opt), MintRefReceipt { metadata })
    }
}
```


<a id="0x1_coin_return_paired_mint_ref"></a>

## Function `return_paired_mint_ref`

Return the `MintRef` with the hot potato receipt.


```move
module 0x1::coin {
    public fun return_paired_mint_ref(mint_ref: fungible_asset::MintRef, receipt: coin::MintRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun return_paired_mint_ref(mint_ref: MintRef, receipt: MintRefReceipt) acquires PairedFungibleAssetRefs {
        let MintRefReceipt { metadata } = receipt;
        assert!(
            fungible_asset::mint_ref_metadata(&mint_ref) == metadata,
            error::invalid_argument(EMINT_REF_RECEIPT_MISMATCH)
        );
        let metadata_addr = object_address(&metadata);
        let mint_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).mint_ref_opt;
        option::fill(mint_ref_opt, mint_ref);
    }
}
```


<a id="0x1_coin_paired_transfer_ref_exists"></a>

## Function `paired_transfer_ref_exists`

Check whether `TransferRef` still exists.


```move
module 0x1::coin {
    #[view]
    public fun paired_transfer_ref_exists<CoinType>(): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun paired_transfer_ref_exists<CoinType>(): bool acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        option::is_some(&borrow_global<PairedFungibleAssetRefs>(metadata_addr).transfer_ref_opt)
    }
}
```


<a id="0x1_coin_get_paired_transfer_ref"></a>

## Function `get_paired_transfer_ref`

Get the TransferRef of paired fungible asset of a coin type from `FreezeCapability`.


```move
module 0x1::coin {
    public fun get_paired_transfer_ref<CoinType>(_: &coin::FreezeCapability<CoinType>): (fungible_asset::TransferRef, coin::TransferRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun get_paired_transfer_ref<CoinType>(
        _: &FreezeCapability<CoinType>
    ): (TransferRef, TransferRefReceipt) acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        let transfer_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).transfer_ref_opt;
        assert!(option::is_some(transfer_ref_opt), error::not_found(ETRANSFER_REF_NOT_FOUND));
        (option::extract(transfer_ref_opt), TransferRefReceipt { metadata })
    }
}
```


<a id="0x1_coin_return_paired_transfer_ref"></a>

## Function `return_paired_transfer_ref`

Return the `TransferRef` with the hot potato receipt.


```move
module 0x1::coin {
    public fun return_paired_transfer_ref(transfer_ref: fungible_asset::TransferRef, receipt: coin::TransferRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun return_paired_transfer_ref(
        transfer_ref: TransferRef,
        receipt: TransferRefReceipt
    ) acquires PairedFungibleAssetRefs {
        let TransferRefReceipt { metadata } = receipt;
        assert!(
            fungible_asset::transfer_ref_metadata(&transfer_ref) == metadata,
            error::invalid_argument(ETRANSFER_REF_RECEIPT_MISMATCH)
        );
        let metadata_addr = object_address(&metadata);
        let transfer_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).transfer_ref_opt;
        option::fill(transfer_ref_opt, transfer_ref);
    }
}
```


<a id="0x1_coin_paired_burn_ref_exists"></a>

## Function `paired_burn_ref_exists`

Check whether `BurnRef` has not been taken.


```move
module 0x1::coin {
    #[view]
    public fun paired_burn_ref_exists<CoinType>(): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun paired_burn_ref_exists<CoinType>(): bool acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        option::is_some(&borrow_global<PairedFungibleAssetRefs>(metadata_addr).burn_ref_opt)
    }
}
```


<a id="0x1_coin_get_paired_burn_ref"></a>

## Function `get_paired_burn_ref`

Get the `BurnRef` of paired fungible asset of a coin type from `BurnCapability`.


```move
module 0x1::coin {
    public fun get_paired_burn_ref<CoinType>(_: &coin::BurnCapability<CoinType>): (fungible_asset::BurnRef, coin::BurnRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun get_paired_burn_ref<CoinType>(
        _: &BurnCapability<CoinType>
    ): (BurnRef, BurnRefReceipt) acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        let burn_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).burn_ref_opt;
        assert!(option::is_some(burn_ref_opt), error::not_found(EBURN_REF_NOT_FOUND));
        (option::extract(burn_ref_opt), BurnRefReceipt { metadata })
    }
}
```


<a id="0x1_coin_convert_and_take_paired_burn_ref"></a>

## Function `convert_and_take_paired_burn_ref`



```move
module 0x1::coin {
    public fun convert_and_take_paired_burn_ref<CoinType>(burn_cap: coin::BurnCapability<CoinType>): fungible_asset::BurnRef
}
```


##### Implementation


```move
module 0x1::coin {
    public fun convert_and_take_paired_burn_ref<CoinType>(
        burn_cap: BurnCapability<CoinType>
    ): BurnRef acquires CoinConversionMap, PairedFungibleAssetRefs {
        destroy_burn_cap(burn_cap);
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        let burn_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).burn_ref_opt;
        assert!(option::is_some(burn_ref_opt), error::not_found(EBURN_REF_NOT_FOUND));
        option::extract(burn_ref_opt)
    }
}
```


<a id="0x1_coin_return_paired_burn_ref"></a>

## Function `return_paired_burn_ref`

Return the `BurnRef` with the hot potato receipt.


```move
module 0x1::coin {
    public fun return_paired_burn_ref(burn_ref: fungible_asset::BurnRef, receipt: coin::BurnRefReceipt)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun return_paired_burn_ref(
        burn_ref: BurnRef,
        receipt: BurnRefReceipt
    ) acquires PairedFungibleAssetRefs {
        let BurnRefReceipt { metadata } = receipt;
        assert!(
            fungible_asset::burn_ref_metadata(&burn_ref) == metadata,
            error::invalid_argument(EBURN_REF_RECEIPT_MISMATCH)
        );
        let metadata_addr = object_address(&metadata);
        let burn_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).burn_ref_opt;
        option::fill(burn_ref_opt, burn_ref);
    }
}
```


<a id="0x1_coin_borrow_paired_burn_ref"></a>

## Function `borrow_paired_burn_ref`



```move
module 0x1::coin {
    fun borrow_paired_burn_ref<CoinType>(_: &coin::BurnCapability<CoinType>): &fungible_asset::BurnRef
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun borrow_paired_burn_ref<CoinType>(
        _: &BurnCapability<CoinType>
    ): &BurnRef acquires CoinConversionMap, PairedFungibleAssetRefs {
        let metadata = assert_paired_metadata_exists<CoinType>();
        let metadata_addr = object_address(&metadata);
        assert!(exists<PairedFungibleAssetRefs>(metadata_addr), error::internal(EPAIRED_FUNGIBLE_ASSET_REFS_NOT_FOUND));
        let burn_ref_opt = &mut borrow_global_mut<PairedFungibleAssetRefs>(metadata_addr).burn_ref_opt;
        assert!(option::is_some(burn_ref_opt), error::not_found(EBURN_REF_NOT_FOUND));
        option::borrow(burn_ref_opt)
    }
}
```


<a id="0x1_coin_initialize_supply_config"></a>

## Function `initialize_supply_config`

Publishes supply configuration. Initially, upgrading is not allowed.


```move
module 0x1::coin {
    public(friend) fun initialize_supply_config(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun initialize_supply_config(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, SupplyConfig { allow_upgrades: false });
    }
}
```


<a id="0x1_coin_allow_supply_upgrades"></a>

## Function `allow_supply_upgrades`

This should be called by on&#45;chain governance to update the config and allow
or disallow upgradability of total supply.


```move
module 0x1::coin {
    public fun allow_supply_upgrades(aptos_framework: &signer, allowed: bool)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun allow_supply_upgrades(aptos_framework: &signer, allowed: bool) acquires SupplyConfig {
        system_addresses::assert_aptos_framework(aptos_framework);
        let allow_upgrades = &mut borrow_global_mut<SupplyConfig>(@aptos_framework).allow_upgrades;
        *allow_upgrades = allowed;
    }
}
```


<a id="0x1_coin_initialize_aggregatable_coin"></a>

## Function `initialize_aggregatable_coin`

Creates a new aggregatable coin with value overflowing on `limit`. Note that this function can
only be called by Aptos Framework (0x1) account for now because of `create_aggregator`.


```move
module 0x1::coin {
    public(friend) fun initialize_aggregatable_coin<CoinType>(aptos_framework: &signer): coin::AggregatableCoin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun initialize_aggregatable_coin<CoinType>(aptos_framework: &signer): AggregatableCoin<CoinType> {
        let aggregator = aggregator_factory::create_aggregator(aptos_framework, MAX_U64);
        AggregatableCoin<CoinType> {
            value: aggregator,
        }
    }
}
```


<a id="0x1_coin_is_aggregatable_coin_zero"></a>

## Function `is_aggregatable_coin_zero`

Returns true if the value of aggregatable coin is zero.


```move
module 0x1::coin {
    public(friend) fun is_aggregatable_coin_zero<CoinType>(coin: &coin::AggregatableCoin<CoinType>): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun is_aggregatable_coin_zero<CoinType>(coin: &AggregatableCoin<CoinType>): bool {
        let amount = aggregator::read(&coin.value);
        amount == 0
    }
}
```


<a id="0x1_coin_drain_aggregatable_coin"></a>

## Function `drain_aggregatable_coin`

Drains the aggregatable coin, setting it to zero and returning a standard coin.


```move
module 0x1::coin {
    public(friend) fun drain_aggregatable_coin<CoinType>(coin: &mut coin::AggregatableCoin<CoinType>): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun drain_aggregatable_coin<CoinType>(coin: &mut AggregatableCoin<CoinType>): Coin<CoinType> {
        spec {
            // TODO: The data invariant is not properly assumed from CollectedFeesPerBlock.
            assume aggregator::spec_get_limit(coin.value) == MAX_U64;
        };
        let amount = aggregator::read(&coin.value);
        assert!(amount <= MAX_U64, error::out_of_range(EAGGREGATABLE_COIN_VALUE_TOO_LARGE));
        spec {
            update aggregate_supply<CoinType> = aggregate_supply<CoinType> - amount;
        };
        aggregator::sub(&mut coin.value, amount);
        spec {
            update supply<CoinType> = supply<CoinType> + amount;
        };
        Coin<CoinType> {
            value: (amount as u64),
        }
    }
}
```


<a id="0x1_coin_merge_aggregatable_coin"></a>

## Function `merge_aggregatable_coin`

Merges `coin` into aggregatable coin (`dst_coin`).


```move
module 0x1::coin {
    public(friend) fun merge_aggregatable_coin<CoinType>(dst_coin: &mut coin::AggregatableCoin<CoinType>, coin: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun merge_aggregatable_coin<CoinType>(
        dst_coin: &mut AggregatableCoin<CoinType>,
        coin: Coin<CoinType>
    ) {
        spec {
            update supply<CoinType> = supply<CoinType> - coin.value;
        };
        let Coin { value } = coin;
        let amount = (value as u128);
        spec {
            update aggregate_supply<CoinType> = aggregate_supply<CoinType> + amount;
        };
        aggregator::add(&mut dst_coin.value, amount);
    }
}
```


<a id="0x1_coin_collect_into_aggregatable_coin"></a>

## Function `collect_into_aggregatable_coin`

Collects a specified amount of coin form an account into aggregatable coin.


```move
module 0x1::coin {
    public(friend) fun collect_into_aggregatable_coin<CoinType>(account_addr: address, amount: u64, dst_coin: &mut coin::AggregatableCoin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun collect_into_aggregatable_coin<CoinType>(
        account_addr: address,
        amount: u64,
        dst_coin: &mut AggregatableCoin<CoinType>,
    ) acquires CoinStore, CoinConversionMap, CoinInfo, PairedCoinType {
        // Skip collecting if amount is zero.
        if (amount == 0) {
            return
        };

        let (coin_amount_to_collect, fa_amount_to_collect) = calculate_amount_to_withdraw<CoinType>(
            account_addr,
            amount
        );
        let coin = if (coin_amount_to_collect > 0) {
            let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
            extract(&mut coin_store.coin, coin_amount_to_collect)
        } else {
            zero()
        };
        if (fa_amount_to_collect > 0) {
            let store_addr = primary_fungible_store::primary_store_address(
                account_addr,
                option::destroy_some(paired_metadata<CoinType>())
            );
            let fa = fungible_asset::withdraw_internal(store_addr, fa_amount_to_collect);
            merge(&mut coin, fungible_asset_to_coin<CoinType>(fa));
        };
        merge_aggregatable_coin(dst_coin, coin);
    }
}
```


<a id="0x1_coin_calculate_amount_to_withdraw"></a>

## Function `calculate_amount_to_withdraw`



```move
module 0x1::coin {
    fun calculate_amount_to_withdraw<CoinType>(account_addr: address, amount: u64): (u64, u64)
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun calculate_amount_to_withdraw<CoinType>(
        account_addr: address,
        amount: u64
    ): (u64, u64) {
        let coin_balance = coin_balance<CoinType>(account_addr);
        if (coin_balance >= amount) {
            (amount, 0)
        } else {
            let metadata = paired_metadata<CoinType>();
            if (option::is_some(&metadata) && primary_fungible_store::primary_store_exists(
                account_addr,
                option::destroy_some(metadata)
            ))
                (coin_balance, amount - coin_balance)
            else
                abort error::invalid_argument(EINSUFFICIENT_BALANCE)
        }
    }
}
```


<a id="0x1_coin_maybe_convert_to_fungible_store"></a>

## Function `maybe_convert_to_fungible_store`



```move
module 0x1::coin {
    fun maybe_convert_to_fungible_store<CoinType>(account: address)
}
```


##### Implementation


```move
module 0x1::coin {
    fun maybe_convert_to_fungible_store<CoinType>(account: address) acquires CoinStore, CoinConversionMap, CoinInfo {
        if (!features::coin_to_fungible_asset_migration_feature_enabled()) {
            abort error::unavailable(ECOIN_TO_FUNGIBLE_ASSET_FEATURE_NOT_ENABLED)
        };
        assert!(is_coin_initialized<CoinType>(), error::invalid_argument(ECOIN_INFO_NOT_PUBLISHED));

        let metadata = ensure_paired_metadata<CoinType>();
        let store = primary_fungible_store::ensure_primary_store_exists(account, metadata);
        let store_address = object::object_address(&store);
        if (exists<CoinStore<CoinType>>(account)) {
            let CoinStore<CoinType> { coin, frozen, deposit_events, withdraw_events } = move_from<CoinStore<CoinType>>(
                account
            );
            event::emit(
                CoinEventHandleDeletion {
                    event_handle_creation_address: guid::creator_address(
                        event::guid(&deposit_events)
                    ),
                    deleted_deposit_event_handle_creation_number: guid::creation_num(event::guid(&deposit_events)),
                    deleted_withdraw_event_handle_creation_number: guid::creation_num(event::guid(&withdraw_events))
                }
            );
            event::destroy_handle(deposit_events);
            event::destroy_handle(withdraw_events);
            if (coin.value == 0) {
                destroy_zero(coin);
            } else {
                fungible_asset::deposit(store, coin_to_fungible_asset(coin));
            };
            // Note:
            // It is possible the primary fungible store may already exist before this function call.
            // In this case, if the account owns a frozen CoinStore and an unfrozen primary fungible store, this
            // function would convert and deposit the rest coin into the primary store and freeze it to make the
            // `frozen` semantic as consistent as possible.
            if (frozen != fungible_asset::is_frozen(store)) {
                fungible_asset::set_frozen_flag_internal(store, frozen);
            }
        };
        if (!exists<MigrationFlag>(store_address)) {
            move_to(&create_signer::create_signer(store_address), MigrationFlag {});
        }
    }
}
```


<a id="0x1_coin_migrate_to_fungible_store"></a>

## Function `migrate_to_fungible_store`

Voluntarily migrate to fungible store for `CoinType` if not yet.


```move
module 0x1::coin {
    public entry fun migrate_to_fungible_store<CoinType>(account: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun migrate_to_fungible_store<CoinType>(
        account: &signer
    ) acquires CoinStore, CoinConversionMap, CoinInfo {
        maybe_convert_to_fungible_store<CoinType>(signer::address_of(account));
    }
}
```


<a id="0x1_coin_coin_address"></a>

## Function `coin_address`

A helper function that returns the address of CoinType.


```move
module 0x1::coin {
    fun coin_address<CoinType>(): address
}
```


##### Implementation


```move
module 0x1::coin {
    fun coin_address<CoinType>(): address {
        let type_info = type_info::type_of<CoinType>();
        type_info::account_address(&type_info)
    }
}
```


<a id="0x1_coin_balance"></a>

## Function `balance`

Returns the balance of `owner` for provided `CoinType` and its paired FA if exists.


```move
module 0x1::coin {
    #[view]
    public fun balance<CoinType>(owner: address): u64
}
```


##### Implementation


```move
module 0x1::coin {
    public fun balance<CoinType>(owner: address): u64 acquires CoinConversionMap, CoinStore {
        let paired_metadata = paired_metadata<CoinType>();
        coin_balance<CoinType>(owner) + if (option::is_some(&paired_metadata)) {
            primary_fungible_store::balance(
                owner,
                option::extract(&mut paired_metadata)
            )
        } else { 0 }
    }
}
```


<a id="0x1_coin_is_balance_at_least"></a>

## Function `is_balance_at_least`

Returns whether the balance of `owner` for provided `CoinType` and its paired FA is &gt;&#61; `amount`.


```move
module 0x1::coin {
    #[view]
    public fun is_balance_at_least<CoinType>(owner: address, amount: u64): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun is_balance_at_least<CoinType>(owner: address, amount: u64): bool acquires CoinConversionMap, CoinStore {
        let coin_balance = coin_balance<CoinType>(owner);
        if (coin_balance >= amount) {
            return true
        };

        let paired_metadata = paired_metadata<CoinType>();
        let left_amount = amount - coin_balance;
        if (option::is_some(&paired_metadata)) {
            primary_fungible_store::is_balance_at_least(
                owner,
                option::extract(&mut paired_metadata),
                left_amount
            )
        } else { false }
    }
}
```


<a id="0x1_coin_coin_balance"></a>

## Function `coin_balance`



```move
module 0x1::coin {
    fun coin_balance<CoinType>(owner: address): u64
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun coin_balance<CoinType>(owner: address): u64 {
        if (exists<CoinStore<CoinType>>(owner)) {
            borrow_global<CoinStore<CoinType>>(owner).coin.value
        } else {
            0
        }
    }
}
```


<a id="0x1_coin_is_coin_initialized"></a>

## Function `is_coin_initialized`

Returns `true` if the type `CoinType` is an initialized coin.


```move
module 0x1::coin {
    #[view]
    public fun is_coin_initialized<CoinType>(): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun is_coin_initialized<CoinType>(): bool {
        exists<CoinInfo<CoinType>>(coin_address<CoinType>())
    }
}
```


<a id="0x1_coin_is_coin_store_frozen"></a>

## Function `is_coin_store_frozen`

Returns `true` is account_addr has frozen the CoinStore or if it&apos;s not registered at all


```move
module 0x1::coin {
    #[view]
    public fun is_coin_store_frozen<CoinType>(account_addr: address): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun is_coin_store_frozen<CoinType>(
        account_addr: address
    ): bool acquires CoinStore, CoinConversionMap {
        if (!is_account_registered<CoinType>(account_addr)) {
            return true
        };

        let coin_store = borrow_global<CoinStore<CoinType>>(account_addr);
        coin_store.frozen
    }
}
```


<a id="0x1_coin_is_account_registered"></a>

## Function `is_account_registered`

Returns `true` if `account_addr` is registered to receive `CoinType`.


```move
module 0x1::coin {
    #[view]
    public fun is_account_registered<CoinType>(account_addr: address): bool
}
```


##### Implementation


```move
module 0x1::coin {
    public fun is_account_registered<CoinType>(account_addr: address): bool acquires CoinConversionMap {
        assert!(is_coin_initialized<CoinType>(), error::invalid_argument(ECOIN_INFO_NOT_PUBLISHED));
        if (exists<CoinStore<CoinType>>(account_addr)) {
            true
        } else {
            let paired_metadata_opt = paired_metadata<CoinType>();
            (option::is_some(
                &paired_metadata_opt
            ) && migrated_primary_fungible_store_exists(account_addr, option::destroy_some(paired_metadata_opt)))
        }
    }
}
```


<a id="0x1_coin_name"></a>

## Function `name`

Returns the name of the coin.


```move
module 0x1::coin {
    #[view]
    public fun name<CoinType>(): string::String
}
```


##### Implementation


```move
module 0x1::coin {
    public fun name<CoinType>(): string::String acquires CoinInfo {
        borrow_global<CoinInfo<CoinType>>(coin_address<CoinType>()).name
    }
}
```


<a id="0x1_coin_symbol"></a>

## Function `symbol`

Returns the symbol of the coin, usually a shorter version of the name.


```move
module 0x1::coin {
    #[view]
    public fun symbol<CoinType>(): string::String
}
```


##### Implementation


```move
module 0x1::coin {
    public fun symbol<CoinType>(): string::String acquires CoinInfo {
        borrow_global<CoinInfo<CoinType>>(coin_address<CoinType>()).symbol
    }
}
```


<a id="0x1_coin_decimals"></a>

## Function `decimals`

Returns the number of decimals used to get its user representation.
For example, if `decimals` equals `2`, a balance of `505` coins should
be displayed to a user as `5.05` (`505 / 10 ** 2`).


```move
module 0x1::coin {
    #[view]
    public fun decimals<CoinType>(): u8
}
```


##### Implementation


```move
module 0x1::coin {
    public fun decimals<CoinType>(): u8 acquires CoinInfo {
        borrow_global<CoinInfo<CoinType>>(coin_address<CoinType>()).decimals
    }
}
```


<a id="0x1_coin_supply"></a>

## Function `supply`

Returns the amount of coin in existence.


```move
module 0x1::coin {
    #[view]
    public fun supply<CoinType>(): option::Option<u128>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun supply<CoinType>(): Option<u128> acquires CoinInfo, CoinConversionMap {
        let coin_supply = coin_supply<CoinType>();
        let metadata = paired_metadata<CoinType>();
        if (option::is_some(&metadata)) {
            let fungible_asset_supply = fungible_asset::supply(option::extract(&mut metadata));
            if (option::is_some(&coin_supply)) {
                let supply = option::borrow_mut(&mut coin_supply);
                *supply = *supply + option::destroy_some(fungible_asset_supply);
            };
        };
        coin_supply
    }
}
```


<a id="0x1_coin_coin_supply"></a>

## Function `coin_supply`

Returns the amount of coin in existence.


```move
module 0x1::coin {
    #[view]
    public fun coin_supply<CoinType>(): option::Option<u128>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun coin_supply<CoinType>(): Option<u128> acquires CoinInfo {
        let maybe_supply = &borrow_global<CoinInfo<CoinType>>(coin_address<CoinType>()).supply;
        if (option::is_some(maybe_supply)) {
            // We do track supply, in this case read from optional aggregator.
            let supply = option::borrow(maybe_supply);
            let value = optional_aggregator::read(supply);
            option::some(value)
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_coin_burn"></a>

## Function `burn`

Burn `coin` with capability.
The capability `_cap` should be passed as a reference to `BurnCapability<CoinType>`.


```move
module 0x1::coin {
    public fun burn<CoinType>(coin: coin::Coin<CoinType>, _cap: &coin::BurnCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun burn<CoinType>(coin: Coin<CoinType>, _cap: &BurnCapability<CoinType>) acquires CoinInfo {
        burn_internal(coin);
    }
}
```


<a id="0x1_coin_burn_from"></a>

## Function `burn_from`

Burn `coin` from the specified `account` with capability.
The capability `burn_cap` should be passed as a reference to `BurnCapability<CoinType>`.
This function shouldn&apos;t fail as it&apos;s called as part of transaction fee burning.

Note: This bypasses CoinStore::frozen &#45;&#45; coins within a frozen CoinStore can be burned.


```move
module 0x1::coin {
    public fun burn_from<CoinType>(account_addr: address, amount: u64, burn_cap: &coin::BurnCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun burn_from<CoinType>(
        account_addr: address,
        amount: u64,
        burn_cap: &BurnCapability<CoinType>,
    ) acquires CoinInfo, CoinStore, CoinConversionMap, PairedFungibleAssetRefs {
        // Skip burning if amount is zero. This shouldn't error out as it's called as part of transaction fee burning.
        if (amount == 0) {
            return
        };

        let (coin_amount_to_burn, fa_amount_to_burn) = calculate_amount_to_withdraw<CoinType>(
            account_addr,
            amount
        );
        if (coin_amount_to_burn > 0) {
            let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
            let coin_to_burn = extract(&mut coin_store.coin, coin_amount_to_burn);
            burn(coin_to_burn, burn_cap);
        };
        if (fa_amount_to_burn > 0) {
            fungible_asset::burn_from(
                borrow_paired_burn_ref(burn_cap),
                primary_fungible_store::primary_store(account_addr, option::destroy_some(paired_metadata<CoinType>())),
                fa_amount_to_burn
            );
        };
    }
}
```


<a id="0x1_coin_deposit"></a>

## Function `deposit`

Deposit the coin balance into the recipient&apos;s account and emit an event.


```move
module 0x1::coin {
    public fun deposit<CoinType>(account_addr: address, coin: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun deposit<CoinType>(
        account_addr: address,
        coin: Coin<CoinType>
    ) acquires CoinStore, CoinConversionMap, CoinInfo {
        if (exists<CoinStore<CoinType>>(account_addr)) {
            let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
            assert!(
                !coin_store.frozen,
                error::permission_denied(EFROZEN),
            );
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    CoinDeposit { coin_type: type_name<CoinType>(), account: account_addr, amount: coin.value }
                );
            };
            event::emit_event<DepositEvent>(
                &mut coin_store.deposit_events,
                DepositEvent { amount: coin.value },
            );
            merge(&mut coin_store.coin, coin);
        } else {
            let metadata = paired_metadata<CoinType>();
            if (option::is_some(&metadata) && migrated_primary_fungible_store_exists(
                account_addr,
                option::destroy_some(metadata)
            )) {
                primary_fungible_store::deposit(account_addr, coin_to_fungible_asset(coin));
            } else {
                abort error::not_found(ECOIN_STORE_NOT_PUBLISHED)
            };
        }
    }
}
```


<a id="0x1_coin_migrated_primary_fungible_store_exists"></a>

## Function `migrated_primary_fungible_store_exists`



```move
module 0x1::coin {
    fun migrated_primary_fungible_store_exists(account_address: address, metadata: object::Object<fungible_asset::Metadata>): bool
}
```


##### Implementation


```move
module 0x1::coin {
    inline fun migrated_primary_fungible_store_exists(
        account_address: address,
        metadata: Object<Metadata>
    ): bool {
        let primary_store_address = primary_fungible_store::primary_store_address<Metadata>(account_address, metadata);
        fungible_asset::store_exists(primary_store_address) && (
            // migration flag is needed, until we start defaulting new accounts to APT PFS
            features::new_accounts_default_to_fa_apt_store_enabled() || exists<MigrationFlag>(primary_store_address)
        )
    }
}
```


<a id="0x1_coin_force_deposit"></a>

## Function `force_deposit`

Deposit the coin balance into the recipient&apos;s account without checking if the account is frozen.
This is for internal use only and doesn&apos;t emit an DepositEvent.


```move
module 0x1::coin {
    public(friend) fun force_deposit<CoinType>(account_addr: address, coin: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun force_deposit<CoinType>(
        account_addr: address,
        coin: Coin<CoinType>
    ) acquires CoinStore, CoinConversionMap, CoinInfo {
        if (exists<CoinStore<CoinType>>(account_addr)) {
            let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
            merge(&mut coin_store.coin, coin);
        } else {
            let metadata = paired_metadata<CoinType>();
            if (option::is_some(&metadata) && migrated_primary_fungible_store_exists(
                account_addr,
                option::destroy_some(metadata)
            )) {
                let fa = coin_to_fungible_asset(coin);
                let metadata = fungible_asset::asset_metadata(&fa);
                let store = primary_fungible_store::primary_store(account_addr, metadata);
                fungible_asset::deposit_internal(object::object_address(&store), fa);
            } else {
                abort error::not_found(ECOIN_STORE_NOT_PUBLISHED)
            }
        }
    }
}
```


<a id="0x1_coin_destroy_zero"></a>

## Function `destroy_zero`

Destroys a zero&#45;value coin. Calls will fail if the `value` in the passed&#45;in `token` is non&#45;zero
so it is impossible to &quot;burn&quot; any non&#45;zero amount of `Coin` without having
a `BurnCapability` for the specific `CoinType`.


```move
module 0x1::coin {
    public fun destroy_zero<CoinType>(zero_coin: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun destroy_zero<CoinType>(zero_coin: Coin<CoinType>) {
        spec {
            update supply<CoinType> = supply<CoinType> - zero_coin.value;
        };
        let Coin { value } = zero_coin;
        assert!(value == 0, error::invalid_argument(EDESTRUCTION_OF_NONZERO_TOKEN))
    }
}
```


<a id="0x1_coin_extract"></a>

## Function `extract`

Extracts `amount` from the passed&#45;in `coin`, where the original token is modified in place.


```move
module 0x1::coin {
    public fun extract<CoinType>(coin: &mut coin::Coin<CoinType>, amount: u64): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun extract<CoinType>(coin: &mut Coin<CoinType>, amount: u64): Coin<CoinType> {
        assert!(coin.value >= amount, error::invalid_argument(EINSUFFICIENT_BALANCE));
        spec {
            update supply<CoinType> = supply<CoinType> - amount;
        };
        coin.value = coin.value - amount;
        spec {
            update supply<CoinType> = supply<CoinType> + amount;
        };
        Coin { value: amount }
    }
}
```


<a id="0x1_coin_extract_all"></a>

## Function `extract_all`

Extracts the entire amount from the passed&#45;in `coin`, where the original token is modified in place.


```move
module 0x1::coin {
    public fun extract_all<CoinType>(coin: &mut coin::Coin<CoinType>): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun extract_all<CoinType>(coin: &mut Coin<CoinType>): Coin<CoinType> {
        let total_value = coin.value;
        spec {
            update supply<CoinType> = supply<CoinType> - coin.value;
        };
        coin.value = 0;
        spec {
            update supply<CoinType> = supply<CoinType> + total_value;
        };
        Coin { value: total_value }
    }
}
```


<a id="0x1_coin_freeze_coin_store"></a>

## Function `freeze_coin_store`

Freeze a CoinStore to prevent transfers


```move
module 0x1::coin {
    #[legacy_entry_fun]
    public entry fun freeze_coin_store<CoinType>(account_addr: address, _freeze_cap: &coin::FreezeCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun freeze_coin_store<CoinType>(
        account_addr: address,
        _freeze_cap: &FreezeCapability<CoinType>,
    ) acquires CoinStore {
        let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
        coin_store.frozen = true;
    }
}
```


<a id="0x1_coin_unfreeze_coin_store"></a>

## Function `unfreeze_coin_store`

Unfreeze a CoinStore to allow transfers


```move
module 0x1::coin {
    #[legacy_entry_fun]
    public entry fun unfreeze_coin_store<CoinType>(account_addr: address, _freeze_cap: &coin::FreezeCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun unfreeze_coin_store<CoinType>(
        account_addr: address,
        _freeze_cap: &FreezeCapability<CoinType>,
    ) acquires CoinStore {
        let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
        coin_store.frozen = false;
    }
}
```


<a id="0x1_coin_upgrade_supply"></a>

## Function `upgrade_supply`

Upgrade total supply to use a parallelizable implementation if it is
available.


```move
module 0x1::coin {
    public entry fun upgrade_supply<CoinType>(account: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun upgrade_supply<CoinType>(account: &signer) acquires CoinInfo, SupplyConfig {
        let account_addr = signer::address_of(account);

        // Only coin creators can upgrade total supply.
        assert!(
            coin_address<CoinType>() == account_addr,
            error::invalid_argument(ECOIN_INFO_ADDRESS_MISMATCH),
        );

        // Can only succeed once on-chain governance agreed on the upgrade.
        assert!(
            borrow_global_mut<SupplyConfig>(@aptos_framework).allow_upgrades,
            error::permission_denied(ECOIN_SUPPLY_UPGRADE_NOT_SUPPORTED)
        );

        let maybe_supply = &mut borrow_global_mut<CoinInfo<CoinType>>(account_addr).supply;
        if (option::is_some(maybe_supply)) {
            let supply = option::borrow_mut(maybe_supply);

            // If supply is tracked and the current implementation uses an integer - upgrade.
            if (!optional_aggregator::is_parallelizable(supply)) {
                optional_aggregator::switch(supply);
            }
        }
    }
}
```


<a id="0x1_coin_initialize"></a>

## Function `initialize`

Creates a new Coin with given `CoinType` and returns minting/freezing/burning capabilities.
The given signer also becomes the account hosting the information  about the coin
(name, supply, etc.). Supply is initialized as non&#45;parallelizable integer.


```move
module 0x1::coin {
    public fun initialize<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun initialize<CoinType>(
        account: &signer,
        name: string::String,
        symbol: string::String,
        decimals: u8,
        monitor_supply: bool,
    ): (BurnCapability<CoinType>, FreezeCapability<CoinType>, MintCapability<CoinType>) {
        initialize_internal(account, name, symbol, decimals, monitor_supply, false)
    }
}
```


<a id="0x1_coin_initialize_with_parallelizable_supply"></a>

## Function `initialize_with_parallelizable_supply`

Same as `initialize` but supply can be initialized to parallelizable aggregator.


```move
module 0x1::coin {
    public(friend) fun initialize_with_parallelizable_supply<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public(friend) fun initialize_with_parallelizable_supply<CoinType>(
        account: &signer,
        name: string::String,
        symbol: string::String,
        decimals: u8,
        monitor_supply: bool,
    ): (BurnCapability<CoinType>, FreezeCapability<CoinType>, MintCapability<CoinType>) {
        system_addresses::assert_aptos_framework(account);
        initialize_internal(account, name, symbol, decimals, monitor_supply, true)
    }
}
```


<a id="0x1_coin_initialize_internal"></a>

## Function `initialize_internal`



```move
module 0x1::coin {
    fun initialize_internal<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool, parallelizable: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    fun initialize_internal<CoinType>(
        account: &signer,
        name: string::String,
        symbol: string::String,
        decimals: u8,
        monitor_supply: bool,
        parallelizable: bool,
    ): (BurnCapability<CoinType>, FreezeCapability<CoinType>, MintCapability<CoinType>) {
        let account_addr = signer::address_of(account);

        assert!(
            coin_address<CoinType>() == account_addr,
            error::invalid_argument(ECOIN_INFO_ADDRESS_MISMATCH),
        );

        assert!(
            !exists<CoinInfo<CoinType>>(account_addr),
            error::already_exists(ECOIN_INFO_ALREADY_PUBLISHED),
        );

        assert!(string::length(&name) <= MAX_COIN_NAME_LENGTH, error::invalid_argument(ECOIN_NAME_TOO_LONG));
        assert!(string::length(&symbol) <= MAX_COIN_SYMBOL_LENGTH, error::invalid_argument(ECOIN_SYMBOL_TOO_LONG));

        let coin_info = CoinInfo<CoinType> {
            name,
            symbol,
            decimals,
            supply: if (monitor_supply) {
                option::some(
                    optional_aggregator::new(MAX_U128, parallelizable)
                )
            } else { option::none() },
        };
        move_to(account, coin_info);

        (BurnCapability<CoinType> {}, FreezeCapability<CoinType> {}, MintCapability<CoinType> {})
    }
}
```


<a id="0x1_coin_merge"></a>

## Function `merge`

&quot;Merges&quot; the two given coins.  The coin passed in as `dst_coin` will have a value equal
to the sum of the two tokens (`dst_coin` and `source_coin`).


```move
module 0x1::coin {
    public fun merge<CoinType>(dst_coin: &mut coin::Coin<CoinType>, source_coin: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun merge<CoinType>(dst_coin: &mut Coin<CoinType>, source_coin: Coin<CoinType>) {
        spec {
            assume dst_coin.value + source_coin.value <= MAX_U64;
        };
        spec {
            update supply<CoinType> = supply<CoinType> - source_coin.value;
        };
        let Coin { value } = source_coin;
        spec {
            update supply<CoinType> = supply<CoinType> + value;
        };
        dst_coin.value = dst_coin.value + value;
    }
}
```


<a id="0x1_coin_mint"></a>

## Function `mint`

Mint new `Coin` with capability.
The capability `_cap` should be passed as reference to `MintCapability<CoinType>`.
Returns minted `Coin`.


```move
module 0x1::coin {
    public fun mint<CoinType>(amount: u64, _cap: &coin::MintCapability<CoinType>): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun mint<CoinType>(
        amount: u64,
        _cap: &MintCapability<CoinType>,
    ): Coin<CoinType> acquires CoinInfo {
        mint_internal<CoinType>(amount)
    }
}
```


<a id="0x1_coin_register"></a>

## Function `register`



```move
module 0x1::coin {
    public fun register<CoinType>(account: &signer)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun register<CoinType>(account: &signer) acquires CoinConversionMap {
        let account_addr = signer::address_of(account);
        // Short-circuit and do nothing if account is already registered for CoinType.
        if (is_account_registered<CoinType>(account_addr)) {
            return
        };

        account::register_coin<CoinType>(account_addr);
        let coin_store = CoinStore<CoinType> {
            coin: Coin { value: 0 },
            frozen: false,
            deposit_events: account::new_event_handle<DepositEvent>(account),
            withdraw_events: account::new_event_handle<WithdrawEvent>(account),
        };
        move_to(account, coin_store);
    }
}
```


<a id="0x1_coin_transfer"></a>

## Function `transfer`

Transfers `amount` of coins `CoinType` from `from` to `to`.


```move
module 0x1::coin {
    public entry fun transfer<CoinType>(from: &signer, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::coin {
    public entry fun transfer<CoinType>(
        from: &signer,
        to: address,
        amount: u64,
    ) acquires CoinStore, CoinConversionMap, CoinInfo, PairedCoinType {
        let coin = withdraw<CoinType>(from, amount);
        deposit(to, coin);
    }
}
```


<a id="0x1_coin_value"></a>

## Function `value`

Returns the `value` passed in `coin`.


```move
module 0x1::coin {
    public fun value<CoinType>(coin: &coin::Coin<CoinType>): u64
}
```


##### Implementation


```move
module 0x1::coin {
    public fun value<CoinType>(coin: &Coin<CoinType>): u64 {
        coin.value
    }
}
```


<a id="0x1_coin_withdraw"></a>

## Function `withdraw`

Withdraw specified `amount` of coin `CoinType` from the signing account.


```move
module 0x1::coin {
    public fun withdraw<CoinType>(account: &signer, amount: u64): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun withdraw<CoinType>(
        account: &signer,
        amount: u64,
    ): Coin<CoinType> acquires CoinStore, CoinConversionMap, CoinInfo, PairedCoinType {
        let account_addr = signer::address_of(account);

        let (coin_amount_to_withdraw, fa_amount_to_withdraw) = calculate_amount_to_withdraw<CoinType>(
            account_addr,
            amount
        );
        let withdrawn_coin = if (coin_amount_to_withdraw > 0) {
            let coin_store = borrow_global_mut<CoinStore<CoinType>>(account_addr);
            assert!(
                !coin_store.frozen,
                error::permission_denied(EFROZEN),
            );
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    CoinWithdraw {
                        coin_type: type_name<CoinType>(), account: account_addr, amount: coin_amount_to_withdraw
                    }
                );
            };
            event::emit_event<WithdrawEvent>(
                &mut coin_store.withdraw_events,
                WithdrawEvent { amount: coin_amount_to_withdraw },
            );
            extract(&mut coin_store.coin, coin_amount_to_withdraw)
        } else {
            zero()
        };
        if (fa_amount_to_withdraw > 0) {
            let fa = primary_fungible_store::withdraw(
                account,
                option::destroy_some(paired_metadata<CoinType>()),
                fa_amount_to_withdraw
            );
            merge(&mut withdrawn_coin, fungible_asset_to_coin(fa));
        };
        withdrawn_coin
    }
}
```


<a id="0x1_coin_zero"></a>

## Function `zero`

Create a new `Coin<CoinType>` with a value of `0`.


```move
module 0x1::coin {
    public fun zero<CoinType>(): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    public fun zero<CoinType>(): Coin<CoinType> {
        spec {
            update supply<CoinType> = supply<CoinType> + 0;
        };
        Coin<CoinType> {
            value: 0
        }
    }
}
```


<a id="0x1_coin_destroy_freeze_cap"></a>

## Function `destroy_freeze_cap`

Destroy a freeze capability. Freeze capability is dangerous and therefore should be destroyed if not used.


```move
module 0x1::coin {
    public fun destroy_freeze_cap<CoinType>(freeze_cap: coin::FreezeCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun destroy_freeze_cap<CoinType>(freeze_cap: FreezeCapability<CoinType>) {
        let FreezeCapability<CoinType> {} = freeze_cap;
    }
}
```


<a id="0x1_coin_destroy_mint_cap"></a>

## Function `destroy_mint_cap`

Destroy a mint capability.


```move
module 0x1::coin {
    public fun destroy_mint_cap<CoinType>(mint_cap: coin::MintCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun destroy_mint_cap<CoinType>(mint_cap: MintCapability<CoinType>) {
        let MintCapability<CoinType> {} = mint_cap;
    }
}
```


<a id="0x1_coin_destroy_burn_cap"></a>

## Function `destroy_burn_cap`

Destroy a burn capability.


```move
module 0x1::coin {
    public fun destroy_burn_cap<CoinType>(burn_cap: coin::BurnCapability<CoinType>)
}
```


##### Implementation


```move
module 0x1::coin {
    public fun destroy_burn_cap<CoinType>(burn_cap: BurnCapability<CoinType>) {
        let BurnCapability<CoinType> {} = burn_cap;
    }
}
```


<a id="0x1_coin_mint_internal"></a>

## Function `mint_internal`



```move
module 0x1::coin {
    fun mint_internal<CoinType>(amount: u64): coin::Coin<CoinType>
}
```


##### Implementation


```move
module 0x1::coin {
    fun mint_internal<CoinType>(amount: u64): Coin<CoinType> acquires CoinInfo {
        if (amount == 0) {
            return Coin<CoinType> {
                value: 0
            }
        };

        let maybe_supply = &mut borrow_global_mut<CoinInfo<CoinType>>(coin_address<CoinType>()).supply;
        if (option::is_some(maybe_supply)) {
            let supply = option::borrow_mut(maybe_supply);
            spec {
                use aptos_framework::optional_aggregator;
                use aptos_framework::aggregator;
                assume optional_aggregator::is_parallelizable(supply) ==> (aggregator::spec_aggregator_get_val(
                    option::borrow(supply.aggregator)
                )
                    + amount <= aggregator::spec_get_limit(option::borrow(supply.aggregator)));
                assume !optional_aggregator::is_parallelizable(supply) ==>
                    (option::borrow(supply.integer).value + amount <= option::borrow(supply.integer).limit);
            };
            optional_aggregator::add(supply, (amount as u128));
        };
        spec {
            update supply<CoinType> = supply<CoinType> + amount;
        };
        Coin<CoinType> { value: amount }
    }
}
```


<a id="0x1_coin_burn_internal"></a>

## Function `burn_internal`



```move
module 0x1::coin {
    fun burn_internal<CoinType>(coin: coin::Coin<CoinType>): u64
}
```


##### Implementation


```move
module 0x1::coin {
    fun burn_internal<CoinType>(coin: Coin<CoinType>): u64 acquires CoinInfo {
        spec {
            update supply<CoinType> = supply<CoinType> - coin.value;
        };
        let Coin { value: amount } = coin;
        if (amount != 0) {
            let maybe_supply = &mut borrow_global_mut<CoinInfo<CoinType>>(coin_address<CoinType>()).supply;
            if (option::is_some(maybe_supply)) {
                let supply = option::borrow_mut(maybe_supply);
                optional_aggregator::sub(supply, (amount as u128));
            };
        };
        amount
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
<td>Only the owner of a coin may mint, burn or freeze coins.</td>
<td>Critical</td>
<td>Acquiring capabilities for a particular CoinType may only occur if the caller has a signer for the module declaring that type. The initialize function returns these capabilities to the caller.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;1.1](upgrade_supply) and [#high&#45;level&#45;req&#45;1.2](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>Each coin may only be created exactly once.</td>
<td>Medium</td>
<td>The initialization function may only be called once.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;2](initialize).</td>
</tr>

<tr>
<td>3</td>
<td>The merging of coins may only be done on coins of the same type.</td>
<td>Critical</td>
<td>The merge function is limited to merging coins of the same type only.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;3](merge).</td>
</tr>

<tr>
<td>4</td>
<td>The supply of a coin is only affected by burn and mint operations.</td>
<td>High</td>
<td>Only mint and burn operations on a coin alter the total supply of coins.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;4](TotalSupplyNoChange).</td>
</tr>

<tr>
<td>5</td>
<td>Users may register an account for a coin multiple times idempotently.</td>
<td>Medium</td>
<td>The register function should work idempotently. Importantly, it should not abort if the coin is already registered.</td>
<td>Formally verified via aborts_if on [#high&#45;level&#45;req&#45;5](register).</td>
</tr>

<tr>
<td>6</td>
<td>Coin operations should fail if the user has not registered for the coin.</td>
<td>Medium</td>
<td>Coin operations may succeed only on valid user coin registration.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;6.1](balance), [#high&#45;level&#45;req&#45;6.2](burn_from), [#high&#45;level&#45;req&#45;6.3](freeze), [#high&#45;level&#45;req&#45;6.4](unfreeze), [#high&#45;level&#45;req&#45;6.5](transfer) and [#high&#45;level&#45;req&#45;6.6](withdraw).</td>
</tr>

<tr>
<td>7</td>
<td>It should always be possible to (1) determine if a coin exists, and (2) determine if a user registered an account with a particular coin. If a coin exists, it should always be possible to request the following information of the coin: (1) Name, (2) Symbol, and (3) Supply.</td>
<td>Low</td>
<td>The following functions should never abort: (1) is_coin_initialized, and (2) is_account_registered. The following functions should not abort if the coin exists: (1) name, (2) symbol, and (3) supply.</td>
<td>Formally Verified in corresponding functions: [#high&#45;level&#45;req&#45;7.1](is_coin_initialized), [#high&#45;level&#45;req&#45;7.2](is_account_registered), [#high&#45;level&#45;req&#45;7.3](name), [#high&#45;level&#45;req&#45;7.4](symbol) and [#high&#45;level&#45;req&#45;7.5](supply).</td>
</tr>

<tr>
<td>8</td>
<td>Coin operations should fail if the user&apos;s CoinStore is frozen.</td>
<td>Medium</td>
<td>If the CoinStore of an address is frozen, coin operations are disallowed.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;8.1](withdraw), [#high&#45;level&#45;req&#45;8.2](transfer) and [#high&#45;level&#45;req&#45;8.3](deposit).</td>
</tr>

<tr>
<td>9</td>
<td>Utilizing AggregatableCoins does not violate other critical invariants, such as (4).</td>
<td>High</td>
<td>Utilizing AggregatableCoin does not change the real&#45;supply of any token.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;9](TotalSupplyNoChange).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::coin {
    pragma verify = true;
<a id="0x1_coin_supply"></a>
    global supply<CoinType>: num;
<a id="0x1_coin_aggregate_supply"></a>
    global aggregate_supply<CoinType>: num;
    apply TotalSupplyTracked<CoinType> to *<CoinType> except
    initialize, initialize_internal, initialize_with_parallelizable_supply;
}
```



<a id="0x1_coin_spec_fun_supply_tracked"></a>


```move
module 0x1::coin {
    fun spec_fun_supply_tracked<CoinType>(val: u64, supply: Option<OptionalAggregator>): bool {
       option::spec_is_some(supply) ==> val == optional_aggregator::optional_aggregator_value
           (option::spec_borrow(supply))
    }
}
```



<a id="0x1_coin_TotalSupplyTracked"></a>


```move
module 0x1::coin {
    schema TotalSupplyTracked<CoinType> {
        ensures old(spec_fun_supply_tracked<CoinType>(supply<CoinType> + aggregate_supply<CoinType>,
            global<CoinInfo<CoinType>>(type_info::type_of<CoinType>().account_address).supply)) ==>
            spec_fun_supply_tracked<CoinType>(supply<CoinType> + aggregate_supply<CoinType>,
                global<CoinInfo<CoinType>>(type_info::type_of<CoinType>().account_address).supply);
    }
}
```



<a id="0x1_coin_spec_fun_supply_no_change"></a>


```move
module 0x1::coin {
    fun spec_fun_supply_no_change<CoinType>(old_supply: Option<OptionalAggregator>,
                                                supply: Option<OptionalAggregator>): bool {
       option::spec_is_some(old_supply) ==> optional_aggregator::optional_aggregator_value
           (option::spec_borrow(old_supply)) == optional_aggregator::optional_aggregator_value
           (option::spec_borrow(supply))
    }
}
```



<a id="0x1_coin_TotalSupplyNoChange"></a>


```move
module 0x1::coin {
    schema TotalSupplyNoChange<CoinType> {
        let old_supply = global<CoinInfo<CoinType>>(type_info::type_of<CoinType>().account_address).supply;
        let post supply = global<CoinInfo<CoinType>>(type_info::type_of<CoinType>().account_address).supply;
        ensures spec_fun_supply_no_change<CoinType>(old_supply, supply);
    }
}
```



<a id="0x1_coin_spec_is_account_registered"></a>


```move
module 0x1::coin {
    fun spec_is_account_registered<CoinType>(account_addr: address): bool {
       let paired_metadata_opt = spec_paired_metadata<CoinType>();
       exists<CoinStore<CoinType>>(account_addr) || (option::spec_is_some(
           paired_metadata_opt
       ) && primary_fungible_store::spec_primary_store_exists(account_addr, option::spec_borrow(paired_metadata_opt)))
    }
}
```



<a id="0x1_coin_CoinSubAbortsIf"></a>


```move
module 0x1::coin {
    schema CoinSubAbortsIf<CoinType> {
        amount: u64;
        let addr = type_info::type_of<CoinType>().account_address;
        let maybe_supply = global<CoinInfo<CoinType>>(addr).supply;
        include (option::is_some(
            maybe_supply
        )) ==> optional_aggregator::SubAbortsIf { optional_aggregator: option::borrow(maybe_supply), value: amount };
    }
}
```



<a id="0x1_coin_CoinAddAbortsIf"></a>


```move
module 0x1::coin {
    schema CoinAddAbortsIf<CoinType> {
        amount: u64;
        let addr = type_info::type_of<CoinType>().account_address;
        let maybe_supply = global<CoinInfo<CoinType>>(addr).supply;
        include (option::is_some(
            maybe_supply
        )) ==> optional_aggregator::AddAbortsIf { optional_aggregator: option::borrow(maybe_supply), value: amount };
    }
}
```



<a id="0x1_coin_AbortsIfNotExistCoinInfo"></a>


```move
module 0x1::coin {
    schema AbortsIfNotExistCoinInfo<CoinType> {
        let addr = type_info::type_of<CoinType>().account_address;
        aborts_if !exists<CoinInfo<CoinType>>(addr);
    }
}
```


<a id="@Specification_1_AggregatableCoin"></a>

### Struct `AggregatableCoin`


```move
module 0x1::coin {
    struct AggregatableCoin<CoinType> has store
}
```


<dl>
<dt>
`value: aggregator::Aggregator`
</dt>
<dd>
 Amount of aggregatable coin this address has.
</dd>
</dl>



```move
module 0x1::coin {
    invariant aggregator::spec_get_limit(value) == MAX_U64;
}
```


<a id="@Specification_1_coin_to_fungible_asset"></a>

### Function `coin_to_fungible_asset`


```move
module 0x1::coin {
    public fun coin_to_fungible_asset<CoinType>(coin: coin::Coin<CoinType>): fungible_asset::FungibleAsset
}
```



```move
module 0x1::coin {
    pragma verify = false;
    let addr = type_info::type_of<CoinType>().account_address;
    modifies global<CoinInfo<CoinType>>(addr);
}
```


<a id="@Specification_1_fungible_asset_to_coin"></a>

### Function `fungible_asset_to_coin`


```move
module 0x1::coin {
    fun fungible_asset_to_coin<CoinType>(fungible_asset: fungible_asset::FungibleAsset): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    pragma verify = false;
}
```


<a id="@Specification_1_initialize_supply_config"></a>

### Function `initialize_supply_config`


```move
module 0x1::coin {
    public(friend) fun initialize_supply_config(aptos_framework: &signer)
}
```

Can only be initialized once.
Can only be published by reserved addresses.


```move
module 0x1::coin {
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if exists<SupplyConfig>(aptos_addr);
    ensures !global<SupplyConfig>(aptos_addr).allow_upgrades;
    ensures exists<SupplyConfig>(aptos_addr);
}
```


<a id="@Specification_1_allow_supply_upgrades"></a>

### Function `allow_supply_upgrades`


```move
module 0x1::coin {
    public fun allow_supply_upgrades(aptos_framework: &signer, allowed: bool)
}
```

Can only be updated by `@aptos_framework`.


```move
module 0x1::coin {
    modifies global<SupplyConfig>(@aptos_framework);
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if !exists<SupplyConfig>(aptos_addr);
    let post allow_upgrades_post = global<SupplyConfig>(@aptos_framework);
    ensures allow_upgrades_post.allow_upgrades == allowed;
}
```


<a id="@Specification_1_initialize_aggregatable_coin"></a>

### Function `initialize_aggregatable_coin`


```move
module 0x1::coin {
    public(friend) fun initialize_aggregatable_coin<CoinType>(aptos_framework: &signer): coin::AggregatableCoin<CoinType>
}
```



```move
module 0x1::coin {
    include system_addresses::AbortsIfNotAptosFramework { account: aptos_framework };
    include aggregator_factory::CreateAggregatorInternalAbortsIf;
}
```


<a id="@Specification_1_is_aggregatable_coin_zero"></a>

### Function `is_aggregatable_coin_zero`


```move
module 0x1::coin {
    public(friend) fun is_aggregatable_coin_zero<CoinType>(coin: &coin::AggregatableCoin<CoinType>): bool
}
```



```move
module 0x1::coin {
    aborts_if false;
    ensures result == (aggregator::spec_read(coin.value) == 0);
}
```


<a id="@Specification_1_drain_aggregatable_coin"></a>

### Function `drain_aggregatable_coin`


```move
module 0x1::coin {
    public(friend) fun drain_aggregatable_coin<CoinType>(coin: &mut coin::AggregatableCoin<CoinType>): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    aborts_if aggregator::spec_read(coin.value) > MAX_U64;
    ensures result.value == aggregator::spec_aggregator_get_val(old(coin).value);
}
```


<a id="@Specification_1_merge_aggregatable_coin"></a>

### Function `merge_aggregatable_coin`


```move
module 0x1::coin {
    public(friend) fun merge_aggregatable_coin<CoinType>(dst_coin: &mut coin::AggregatableCoin<CoinType>, coin: coin::Coin<CoinType>)
}
```



```move
module 0x1::coin {
    let aggr = dst_coin.value;
    let post p_aggr = dst_coin.value;
    aborts_if aggregator::spec_aggregator_get_val(aggr)
        + coin.value > aggregator::spec_get_limit(aggr);
    aborts_if aggregator::spec_aggregator_get_val(aggr)
        + coin.value > MAX_U128;
    ensures aggregator::spec_aggregator_get_val(aggr) + coin.value == aggregator::spec_aggregator_get_val(p_aggr);
}
```


<a id="@Specification_1_collect_into_aggregatable_coin"></a>

### Function `collect_into_aggregatable_coin`


```move
module 0x1::coin {
    public(friend) fun collect_into_aggregatable_coin<CoinType>(account_addr: address, amount: u64, dst_coin: &mut coin::AggregatableCoin<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    let aggr = dst_coin.value;
    let post p_aggr = dst_coin.value;
    let coin_store = global<CoinStore<CoinType>>(account_addr);
    let post p_coin_store = global<CoinStore<CoinType>>(account_addr);
    aborts_if amount > 0 && !exists<CoinStore<CoinType>>(account_addr);
    aborts_if amount > 0 && coin_store.coin.value < amount;
    aborts_if amount > 0 && aggregator::spec_aggregator_get_val(aggr)
        + amount > aggregator::spec_get_limit(aggr);
    aborts_if amount > 0 && aggregator::spec_aggregator_get_val(aggr)
        + amount > MAX_U128;
    ensures aggregator::spec_aggregator_get_val(aggr) + amount == aggregator::spec_aggregator_get_val(p_aggr);
    ensures coin_store.coin.value - amount == p_coin_store.coin.value;
}
```


<a id="@Specification_1_maybe_convert_to_fungible_store"></a>

### Function `maybe_convert_to_fungible_store`


```move
module 0x1::coin {
    fun maybe_convert_to_fungible_store<CoinType>(account: address)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    modifies global<CoinInfo<CoinType>>(account);
    modifies global<CoinStore<CoinType>>(account);
}
```



<a id="0x1_coin_DepositAbortsIf"></a>


```move
module 0x1::coin {
    schema DepositAbortsIf<CoinType> {
        account_addr: address;
        let coin_store = global<CoinStore<CoinType>>(account_addr);
        aborts_if !exists<CoinStore<CoinType>>(account_addr);
        aborts_if coin_store.frozen;
    }
}
```


<a id="@Specification_1_coin_address"></a>

### Function `coin_address`


```move
module 0x1::coin {
    fun coin_address<CoinType>(): address
}
```

Get address by reflection.


```move
module 0x1::coin {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == type_info::type_of<CoinType>().account_address;
}
```


<a id="@Specification_1_balance"></a>

### Function `balance`


```move
module 0x1::coin {
    #[view]
    public fun balance<CoinType>(owner: address): u64
}
```



```move
module 0x1::coin {
    pragma verify = false;
    aborts_if !exists<CoinStore<CoinType>>(owner);
    ensures result == global<CoinStore<CoinType>>(owner).coin.value;
}
```


<a id="@Specification_1_is_coin_initialized"></a>

### Function `is_coin_initialized`


```move
module 0x1::coin {
    #[view]
    public fun is_coin_initialized<CoinType>(): bool
}
```



```move
module 0x1::coin {
// This enforces ### high&#45;level&#45;req&#45;7.1
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if false;
}
```


<a id="@Specification_1_is_account_registered"></a>

### Function `is_account_registered`


```move
module 0x1::coin {
    #[view]
    public fun is_account_registered<CoinType>(account_addr: address): bool
}
```



```move
module 0x1::coin {
    pragma aborts_if_is_partial;
    aborts_if false;
}
```



<a id="0x1_coin_get_coin_supply_opt"></a>


```move
module 0x1::coin {
    fun get_coin_supply_opt<CoinType>(): Option<OptionalAggregator> {
       global<CoinInfo<CoinType>>(type_info::type_of<CoinType>().account_address).supply
    }
}
```



<a id="0x1_coin_spec_paired_metadata"></a>


```move
module 0x1::coin {
    fun spec_paired_metadata<CoinType>(): Option<Object<Metadata>> {
       if (exists<CoinConversionMap>(@aptos_framework)) {
           let map = global<CoinConversionMap>(@aptos_framework).coin_to_fungible_asset_map;
           if (table::spec_contains(map, type_info::type_of<CoinType>())) {
               let metadata = table::spec_get(map, type_info::type_of<CoinType>());
               option::spec_some(metadata)
           } else {
               option::spec_none()
           }
       } else {
           option::spec_none()
       }
    }
}
```


<a id="@Specification_1_name"></a>

### Function `name`


```move
module 0x1::coin {
    #[view]
    public fun name<CoinType>(): string::String
}
```



```move
module 0x1::coin {
// This enforces ### high&#45;level&#45;req&#45;7.3
[#high&#45;level&#45;req](high&#45;level requirement 7):
    include AbortsIfNotExistCoinInfo<CoinType>;
}
```


<a id="@Specification_1_symbol"></a>

### Function `symbol`


```move
module 0x1::coin {
    #[view]
    public fun symbol<CoinType>(): string::String
}
```



```move
module 0x1::coin {
// This enforces ### high&#45;level&#45;req&#45;7.4
[#high&#45;level&#45;req](high&#45;level requirement 7):
    include AbortsIfNotExistCoinInfo<CoinType>;
}
```


<a id="@Specification_1_decimals"></a>

### Function `decimals`


```move
module 0x1::coin {
    #[view]
    public fun decimals<CoinType>(): u8
}
```



```move
module 0x1::coin {
    include AbortsIfNotExistCoinInfo<CoinType>;
}
```


<a id="@Specification_1_supply"></a>

### Function `supply`


```move
module 0x1::coin {
    #[view]
    public fun supply<CoinType>(): option::Option<u128>
}
```



```move
module 0x1::coin {
    pragma verify = false;
}
```


<a id="@Specification_1_coin_supply"></a>

### Function `coin_supply`


```move
module 0x1::coin {
    #[view]
    public fun coin_supply<CoinType>(): option::Option<u128>
}
```



```move
module 0x1::coin {
    let coin_addr = type_info::type_of<CoinType>().account_address;
// This enforces ### high&#45;level&#45;req&#45;7.5
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if !exists<CoinInfo<CoinType>>(coin_addr);
    let maybe_supply = global<CoinInfo<CoinType>>(coin_addr).supply;
    let supply = option::spec_borrow(maybe_supply);
    let value = optional_aggregator::optional_aggregator_value(supply);
    ensures if (option::spec_is_some(maybe_supply)) {
        result == option::spec_some(value)
    } else {
        option::spec_is_none(result)
    };
}
```


<a id="@Specification_1_burn"></a>

### Function `burn`


```move
module 0x1::coin {
    public fun burn<CoinType>(coin: coin::Coin<CoinType>, _cap: &coin::BurnCapability<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    let addr = type_info::type_of<CoinType>().account_address;
    modifies global<CoinInfo<CoinType>>(addr);
    include AbortsIfNotExistCoinInfo<CoinType>;
    aborts_if coin.value == 0;
    include CoinSubAbortsIf<CoinType> { amount: coin.value };
    ensures supply<CoinType> == old(supply<CoinType>) - coin.value;
}
```


<a id="@Specification_1_burn_from"></a>

### Function `burn_from`


```move
module 0x1::coin {
    public fun burn_from<CoinType>(account_addr: address, amount: u64, burn_cap: &coin::BurnCapability<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    let addr = type_info::type_of<CoinType>().account_address;
    let coin_store = global<CoinStore<CoinType>>(account_addr);
    let post post_coin_store = global<CoinStore<CoinType>>(account_addr);
    modifies global<CoinInfo<CoinType>>(addr);
    modifies global<CoinStore<CoinType>>(account_addr);
// This enforces ### high&#45;level&#45;req&#45;6.2
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if amount != 0 && !exists<CoinInfo<CoinType>>(addr);
    aborts_if amount != 0 && !exists<CoinStore<CoinType>>(account_addr);
    aborts_if coin_store.coin.value < amount;
    let maybe_supply = global<CoinInfo<CoinType>>(addr).supply;
    let supply_aggr = option::spec_borrow(maybe_supply);
    let value = optional_aggregator::optional_aggregator_value(supply_aggr);
    let post post_maybe_supply = global<CoinInfo<CoinType>>(addr).supply;
    let post post_supply = option::spec_borrow(post_maybe_supply);
    let post post_value = optional_aggregator::optional_aggregator_value(post_supply);
    aborts_if option::spec_is_some(maybe_supply) && value < amount;
    ensures post_coin_store.coin.value == coin_store.coin.value - amount;
// This enforces ### high&#45;level&#45;req&#45;5
[managed_coin.md#high&#45;level&#45;req](high&#45;level requirement 5) of the [managed_coin.md](managed_coin) module:
    ensures if (option::spec_is_some(maybe_supply)) {
        post_value == value - amount
    } else {
        option::spec_is_none(post_maybe_supply)
    };
    ensures supply<CoinType> == old(supply<CoinType>) - amount;
}
```


<a id="@Specification_1_deposit"></a>

### Function `deposit`


```move
module 0x1::coin {
    public fun deposit<CoinType>(account_addr: address, coin: coin::Coin<CoinType>)
}
```

`account_addr` is not frozen.


```move
module 0x1::coin {
    pragma verify = false;
    modifies global<CoinInfo<CoinType>>(account_addr);
// This enforces ### high&#45;level&#45;req&#45;8.3
[#high&#45;level&#45;req](high&#45;level requirement 8):
    include DepositAbortsIf<CoinType>;
    ensures global<CoinStore<CoinType>>(account_addr).coin.value == old(
        global<CoinStore<CoinType>>(account_addr)
    ).coin.value + coin.value;
}
```


<a id="@Specification_1_force_deposit"></a>

### Function `force_deposit`


```move
module 0x1::coin {
    public(friend) fun force_deposit<CoinType>(account_addr: address, coin: coin::Coin<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    modifies global<CoinStore<CoinType>>(account_addr);
    aborts_if !exists<CoinStore<CoinType>>(account_addr);
    ensures global<CoinStore<CoinType>>(account_addr).coin.value == old(
        global<CoinStore<CoinType>>(account_addr)
    ).coin.value + coin.value;
}
```


<a id="@Specification_1_destroy_zero"></a>

### Function `destroy_zero`


```move
module 0x1::coin {
    public fun destroy_zero<CoinType>(zero_coin: coin::Coin<CoinType>)
}
```

The value of `zero_coin` must be 0.


```move
module 0x1::coin {
    aborts_if zero_coin.value > 0;
}
```


<a id="@Specification_1_extract"></a>

### Function `extract`


```move
module 0x1::coin {
    public fun extract<CoinType>(coin: &mut coin::Coin<CoinType>, amount: u64): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    aborts_if coin.value < amount;
    ensures result.value == amount;
    ensures coin.value == old(coin.value) - amount;
}
```


<a id="@Specification_1_extract_all"></a>

### Function `extract_all`


```move
module 0x1::coin {
    public fun extract_all<CoinType>(coin: &mut coin::Coin<CoinType>): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    ensures result.value == old(coin).value;
    ensures coin.value == 0;
}
```


<a id="@Specification_1_freeze_coin_store"></a>

### Function `freeze_coin_store`


```move
module 0x1::coin {
    #[legacy_entry_fun]
    public entry fun freeze_coin_store<CoinType>(account_addr: address, _freeze_cap: &coin::FreezeCapability<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    modifies global<CoinStore<CoinType>>(account_addr);
// This enforces ### high&#45;level&#45;req&#45;6.3
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !exists<CoinStore<CoinType>>(account_addr);
    let post coin_store = global<CoinStore<CoinType>>(account_addr);
    ensures coin_store.frozen;
}
```


<a id="@Specification_1_unfreeze_coin_store"></a>

### Function `unfreeze_coin_store`


```move
module 0x1::coin {
    #[legacy_entry_fun]
    public entry fun unfreeze_coin_store<CoinType>(account_addr: address, _freeze_cap: &coin::FreezeCapability<CoinType>)
}
```



```move
module 0x1::coin {
    pragma verify = false;
    modifies global<CoinStore<CoinType>>(account_addr);
// This enforces ### high&#45;level&#45;req&#45;6.4
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !exists<CoinStore<CoinType>>(account_addr);
    let post coin_store = global<CoinStore<CoinType>>(account_addr);
    ensures !coin_store.frozen;
}
```


<a id="@Specification_1_upgrade_supply"></a>

### Function `upgrade_supply`


```move
module 0x1::coin {
    public entry fun upgrade_supply<CoinType>(account: &signer)
}
```

The creator of `CoinType` must be `@aptos_framework`.
`SupplyConfig` allow upgrade.


```move
module 0x1::coin {
    let account_addr = signer::address_of(account);
    let coin_address = type_info::type_of<CoinType>().account_address;
    aborts_if coin_address != account_addr;
    aborts_if !exists<SupplyConfig>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1.1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if !exists<CoinInfo<CoinType>>(account_addr);
    let supply_config = global<SupplyConfig>(@aptos_framework);
    aborts_if !supply_config.allow_upgrades;
    modifies global<CoinInfo<CoinType>>(account_addr);
    let maybe_supply = global<CoinInfo<CoinType>>(account_addr).supply;
    let supply = option::spec_borrow(maybe_supply);
    let value = optional_aggregator::optional_aggregator_value(supply);
    let post post_maybe_supply = global<CoinInfo<CoinType>>(account_addr).supply;
    let post post_supply = option::spec_borrow(post_maybe_supply);
    let post post_value = optional_aggregator::optional_aggregator_value(post_supply);
    let supply_no_parallel = option::spec_is_some(maybe_supply) &&
        !optional_aggregator::is_parallelizable(supply);
    aborts_if supply_no_parallel && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    ensures supply_no_parallel ==>
        optional_aggregator::is_parallelizable(post_supply) && post_value == value;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::coin {
    public fun initialize<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```



```move
module 0x1::coin {
    let account_addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;1.2
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if type_info::type_of<CoinType>().account_address != account_addr;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if exists<CoinInfo<CoinType>>(account_addr);
    aborts_if string::length(name) > MAX_COIN_NAME_LENGTH;
    aborts_if string::length(symbol) > MAX_COIN_SYMBOL_LENGTH;
}
```


<a id="@Specification_1_initialize_with_parallelizable_supply"></a>

### Function `initialize_with_parallelizable_supply`


```move
module 0x1::coin {
    public(friend) fun initialize_with_parallelizable_supply<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```



```move
module 0x1::coin {
    let addr = signer::address_of(account);
    aborts_if addr != @aptos_framework;
    aborts_if monitor_supply && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    include InitializeInternalSchema<CoinType> {
        name: name.bytes,
        symbol: symbol.bytes
    };
    ensures exists<CoinInfo<CoinType>>(addr);
}
```


<a id="@Specification_1_initialize_internal"></a>

### Function `initialize_internal`


```move
module 0x1::coin {
    fun initialize_internal<CoinType>(account: &signer, name: string::String, symbol: string::String, decimals: u8, monitor_supply: bool, parallelizable: bool): (coin::BurnCapability<CoinType>, coin::FreezeCapability<CoinType>, coin::MintCapability<CoinType>)
}
```



```move
module 0x1::coin {
    include InitializeInternalSchema<CoinType> {
        name: name.bytes,
        symbol: symbol.bytes
    };
    let account_addr = signer::address_of(account);
    let post coin_info = global<CoinInfo<CoinType>>(account_addr);
    let post supply = option::spec_borrow(coin_info.supply);
    let post value = optional_aggregator::optional_aggregator_value(supply);
    let post limit = optional_aggregator::optional_aggregator_limit(supply);
    modifies global<CoinInfo<CoinType>>(account_addr);
    aborts_if monitor_supply && parallelizable
        && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;2
[managed_coin.md#high&#45;level&#45;req](high&#45;level requirement 2) of the [managed_coin.md](managed_coin) module:
    ensures exists<CoinInfo<CoinType>>(account_addr)
        && coin_info.name == name
        && coin_info.symbol == symbol
        && coin_info.decimals == decimals;
    ensures if (monitor_supply) {
        value == 0 && limit == MAX_U128
            && (parallelizable == optional_aggregator::is_parallelizable(supply))
    } else {
        option::spec_is_none(coin_info.supply)
    };
    ensures result_1 == BurnCapability<CoinType> {};
    ensures result_2 == FreezeCapability<CoinType> {};
    ensures result_3 == MintCapability<CoinType> {};
}
```


<a id="@Specification_1_merge"></a>

### Function `merge`


```move
module 0x1::coin {
    public fun merge<CoinType>(dst_coin: &mut coin::Coin<CoinType>, source_coin: coin::Coin<CoinType>)
}
```



```move
module 0x1::coin {
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures dst_coin.value == old(dst_coin.value) + source_coin.value;
}
```


<a id="@Specification_1_mint"></a>

### Function `mint`


```move
module 0x1::coin {
    public fun mint<CoinType>(amount: u64, _cap: &coin::MintCapability<CoinType>): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    let addr = type_info::type_of<CoinType>().account_address;
    modifies global<CoinInfo<CoinType>>(addr);
}
```


<a id="@Specification_1_register"></a>

### Function `register`


```move
module 0x1::coin {
    public fun register<CoinType>(account: &signer)
}
```

An account can only be registered once.
Updating `Account.guid_creation_num` will not overflow.


```move
module 0x1::coin {
    pragma verify = false;
}
```


<a id="@Specification_1_transfer"></a>

### Function `transfer`


```move
module 0x1::coin {
    public entry fun transfer<CoinType>(from: &signer, to: address, amount: u64)
}
```

`from` and `to` account not frozen.
`from` and `to` not the same address.
`from` account sufficient balance.


```move
module 0x1::coin {
    pragma verify = false;
    let account_addr_from = signer::address_of(from);
    let coin_store_from = global<CoinStore<CoinType>>(account_addr_from);
    let post coin_store_post_from = global<CoinStore<CoinType>>(account_addr_from);
    let coin_store_to = global<CoinStore<CoinType>>(to);
    let post coin_store_post_to = global<CoinStore<CoinType>>(to);
// This enforces ### high&#45;level&#45;req&#45;6.5
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !exists<CoinStore<CoinType>>(account_addr_from);
    aborts_if !exists<CoinStore<CoinType>>(to);
// This enforces ### high&#45;level&#45;req&#45;8.2
[#high&#45;level&#45;req](high&#45;level requirement 8):
    aborts_if coin_store_from.frozen;
    aborts_if coin_store_to.frozen;
    aborts_if coin_store_from.coin.value < amount;
    ensures account_addr_from != to ==> coin_store_post_from.coin.value ==
        coin_store_from.coin.value - amount;
    ensures account_addr_from != to ==> coin_store_post_to.coin.value == coin_store_to.coin.value + amount;
    ensures account_addr_from == to ==> coin_store_post_from.coin.value == coin_store_from.coin.value;
}
```


<a id="@Specification_1_withdraw"></a>

### Function `withdraw`


```move
module 0x1::coin {
    public fun withdraw<CoinType>(account: &signer, amount: u64): coin::Coin<CoinType>
}
```

Account is not frozen and sufficient balance.


```move
module 0x1::coin {
    pragma verify = false;
    include WithdrawAbortsIf<CoinType>;
    modifies global<CoinStore<CoinType>>(account_addr);
    let account_addr = signer::address_of(account);
    let coin_store = global<CoinStore<CoinType>>(account_addr);
    let balance = coin_store.coin.value;
    let post coin_post = global<CoinStore<CoinType>>(account_addr).coin.value;
    ensures coin_post == balance - amount;
    ensures result == Coin<CoinType> { value: amount };
}
```



<a id="0x1_coin_WithdrawAbortsIf"></a>


```move
module 0x1::coin {
    schema WithdrawAbortsIf<CoinType> {
        account: &signer;
        amount: u64;
        let account_addr = signer::address_of(account);
        let coin_store = global<CoinStore<CoinType>>(account_addr);
        let balance = coin_store.coin.value;
    // This enforces ### high&#45;level&#45;req&#45;6.6
    [#high&#45;level&#45;req](high&#45;level requirement 6):
        aborts_if !exists<CoinStore<CoinType>>(account_addr);
    // This enforces ### high&#45;level&#45;req&#45;8.1
    [#high&#45;level&#45;req](high&#45;level requirement 8):
        aborts_if coin_store.frozen;
        aborts_if balance < amount;
    }
}
```


<a id="@Specification_1_mint_internal"></a>

### Function `mint_internal`


```move
module 0x1::coin {
    fun mint_internal<CoinType>(amount: u64): coin::Coin<CoinType>
}
```



```move
module 0x1::coin {
    let addr = type_info::type_of<CoinType>().account_address;
    modifies global<CoinInfo<CoinType>>(addr);
    aborts_if (amount != 0) && !exists<CoinInfo<CoinType>>(addr);
    ensures supply<CoinType> == old(supply<CoinType>) + amount;
    ensures result.value == amount;
}
```


<a id="@Specification_1_burn_internal"></a>

### Function `burn_internal`


```move
module 0x1::coin {
    fun burn_internal<CoinType>(coin: coin::Coin<CoinType>): u64
}
```



```move
module 0x1::coin {
    pragma verify = false;
    let addr = type_info::type_of<CoinType>().account_address;
    modifies global<CoinInfo<CoinType>>(addr);
}
```
