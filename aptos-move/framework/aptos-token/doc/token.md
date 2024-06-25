
<a id="0x3_token"></a>

# Module `0x3::token`

This module provides the foundation for Tokens.
Checkout our developer doc on our token standard https://aptos.dev/standards


-  [Struct `Token`](#0x3_token_Token)
-  [Struct `TokenId`](#0x3_token_TokenId)
-  [Struct `TokenDataId`](#0x3_token_TokenDataId)
-  [Struct `TokenData`](#0x3_token_TokenData)
-  [Struct `Royalty`](#0x3_token_Royalty)
-  [Struct `TokenMutabilityConfig`](#0x3_token_TokenMutabilityConfig)
-  [Resource `TokenStore`](#0x3_token_TokenStore)
-  [Struct `CollectionMutabilityConfig`](#0x3_token_CollectionMutabilityConfig)
-  [Resource `Collections`](#0x3_token_Collections)
-  [Struct `CollectionData`](#0x3_token_CollectionData)
-  [Struct `WithdrawCapability`](#0x3_token_WithdrawCapability)
-  [Struct `DepositEvent`](#0x3_token_DepositEvent)
-  [Struct `Deposit`](#0x3_token_Deposit)
-  [Struct `WithdrawEvent`](#0x3_token_WithdrawEvent)
-  [Struct `Withdraw`](#0x3_token_Withdraw)
-  [Struct `CreateTokenDataEvent`](#0x3_token_CreateTokenDataEvent)
-  [Struct `CreateTokenData`](#0x3_token_CreateTokenData)
-  [Struct `MintTokenEvent`](#0x3_token_MintTokenEvent)
-  [Struct `MintToken`](#0x3_token_MintToken)
-  [Struct `BurnTokenEvent`](#0x3_token_BurnTokenEvent)
-  [Struct `BurnToken`](#0x3_token_BurnToken)
-  [Struct `MutateTokenPropertyMapEvent`](#0x3_token_MutateTokenPropertyMapEvent)
-  [Struct `MutateTokenPropertyMap`](#0x3_token_MutateTokenPropertyMap)
-  [Struct `CreateCollectionEvent`](#0x3_token_CreateCollectionEvent)
-  [Struct `CreateCollection`](#0x3_token_CreateCollection)
-  [Constants](#@Constants_0)
-  [Function `create_collection_script`](#0x3_token_create_collection_script)
-  [Function `create_token_script`](#0x3_token_create_token_script)
-  [Function `mint_script`](#0x3_token_mint_script)
-  [Function `mutate_token_properties`](#0x3_token_mutate_token_properties)
-  [Function `direct_transfer_script`](#0x3_token_direct_transfer_script)
-  [Function `opt_in_direct_transfer`](#0x3_token_opt_in_direct_transfer)
-  [Function `transfer_with_opt_in`](#0x3_token_transfer_with_opt_in)
-  [Function `burn_by_creator`](#0x3_token_burn_by_creator)
-  [Function `burn`](#0x3_token_burn)
-  [Function `mutate_collection_description`](#0x3_token_mutate_collection_description)
-  [Function `mutate_collection_uri`](#0x3_token_mutate_collection_uri)
-  [Function `mutate_collection_maximum`](#0x3_token_mutate_collection_maximum)
-  [Function `mutate_tokendata_maximum`](#0x3_token_mutate_tokendata_maximum)
-  [Function `mutate_tokendata_uri`](#0x3_token_mutate_tokendata_uri)
-  [Function `mutate_tokendata_royalty`](#0x3_token_mutate_tokendata_royalty)
-  [Function `mutate_tokendata_description`](#0x3_token_mutate_tokendata_description)
-  [Function `mutate_tokendata_property`](#0x3_token_mutate_tokendata_property)
-  [Function `mutate_one_token`](#0x3_token_mutate_one_token)
-  [Function `create_royalty`](#0x3_token_create_royalty)
-  [Function `deposit_token`](#0x3_token_deposit_token)
-  [Function `direct_deposit_with_opt_in`](#0x3_token_direct_deposit_with_opt_in)
-  [Function `direct_transfer`](#0x3_token_direct_transfer)
-  [Function `initialize_token_store`](#0x3_token_initialize_token_store)
-  [Function `merge`](#0x3_token_merge)
-  [Function `split`](#0x3_token_split)
-  [Function `token_id`](#0x3_token_token_id)
-  [Function `transfer`](#0x3_token_transfer)
-  [Function `create_withdraw_capability`](#0x3_token_create_withdraw_capability)
-  [Function `withdraw_with_capability`](#0x3_token_withdraw_with_capability)
-  [Function `partial_withdraw_with_capability`](#0x3_token_partial_withdraw_with_capability)
-  [Function `withdraw_token`](#0x3_token_withdraw_token)
-  [Function `create_collection`](#0x3_token_create_collection)
-  [Function `check_collection_exists`](#0x3_token_check_collection_exists)
-  [Function `check_tokendata_exists`](#0x3_token_check_tokendata_exists)
-  [Function `create_tokendata`](#0x3_token_create_tokendata)
-  [Function `get_collection_supply`](#0x3_token_get_collection_supply)
-  [Function `get_collection_description`](#0x3_token_get_collection_description)
-  [Function `get_collection_uri`](#0x3_token_get_collection_uri)
-  [Function `get_collection_maximum`](#0x3_token_get_collection_maximum)
-  [Function `get_token_supply`](#0x3_token_get_token_supply)
-  [Function `get_tokendata_largest_property_version`](#0x3_token_get_tokendata_largest_property_version)
-  [Function `get_token_id`](#0x3_token_get_token_id)
-  [Function `get_direct_transfer`](#0x3_token_get_direct_transfer)
-  [Function `create_token_mutability_config`](#0x3_token_create_token_mutability_config)
-  [Function `create_collection_mutability_config`](#0x3_token_create_collection_mutability_config)
-  [Function `mint_token`](#0x3_token_mint_token)
-  [Function `mint_token_to`](#0x3_token_mint_token_to)
-  [Function `create_token_id`](#0x3_token_create_token_id)
-  [Function `create_token_data_id`](#0x3_token_create_token_data_id)
-  [Function `create_token_id_raw`](#0x3_token_create_token_id_raw)
-  [Function `balance_of`](#0x3_token_balance_of)
-  [Function `has_token_store`](#0x3_token_has_token_store)
-  [Function `get_royalty`](#0x3_token_get_royalty)
-  [Function `get_royalty_numerator`](#0x3_token_get_royalty_numerator)
-  [Function `get_royalty_denominator`](#0x3_token_get_royalty_denominator)
-  [Function `get_royalty_payee`](#0x3_token_get_royalty_payee)
-  [Function `get_token_amount`](#0x3_token_get_token_amount)
-  [Function `get_token_id_fields`](#0x3_token_get_token_id_fields)
-  [Function `get_token_data_id_fields`](#0x3_token_get_token_data_id_fields)
-  [Function `get_property_map`](#0x3_token_get_property_map)
-  [Function `get_tokendata_maximum`](#0x3_token_get_tokendata_maximum)
-  [Function `get_tokendata_uri`](#0x3_token_get_tokendata_uri)
-  [Function `get_tokendata_description`](#0x3_token_get_tokendata_description)
-  [Function `get_tokendata_royalty`](#0x3_token_get_tokendata_royalty)
-  [Function `get_tokendata_id`](#0x3_token_get_tokendata_id)
-  [Function `get_tokendata_mutability_config`](#0x3_token_get_tokendata_mutability_config)
-  [Function `get_token_mutability_maximum`](#0x3_token_get_token_mutability_maximum)
-  [Function `get_token_mutability_royalty`](#0x3_token_get_token_mutability_royalty)
-  [Function `get_token_mutability_uri`](#0x3_token_get_token_mutability_uri)
-  [Function `get_token_mutability_description`](#0x3_token_get_token_mutability_description)
-  [Function `get_token_mutability_default_properties`](#0x3_token_get_token_mutability_default_properties)
-  [Function `get_collection_mutability_config`](#0x3_token_get_collection_mutability_config)
-  [Function `get_collection_mutability_description`](#0x3_token_get_collection_mutability_description)
-  [Function `get_collection_mutability_uri`](#0x3_token_get_collection_mutability_uri)
-  [Function `get_collection_mutability_maximum`](#0x3_token_get_collection_mutability_maximum)
-  [Function `destroy_token_data`](#0x3_token_destroy_token_data)
-  [Function `destroy_collection_data`](#0x3_token_destroy_collection_data)
-  [Function `withdraw_with_event_internal`](#0x3_token_withdraw_with_event_internal)
-  [Function `update_token_property_internal`](#0x3_token_update_token_property_internal)
-  [Function `direct_deposit`](#0x3_token_direct_deposit)
-  [Function `assert_collection_exists`](#0x3_token_assert_collection_exists)
-  [Function `assert_tokendata_exists`](#0x3_token_assert_tokendata_exists)
-  [Function `assert_non_standard_reserved_property`](#0x3_token_assert_non_standard_reserved_property)
-  [Function `initialize_token_script`](#0x3_token_initialize_token_script)
-  [Function `initialize_token`](#0x3_token_initialize_token)
-  [Specification](#@Specification_1)
    -  [Function `create_collection_script`](#@Specification_1_create_collection_script)
    -  [Function `create_token_script`](#@Specification_1_create_token_script)
    -  [Function `mint_script`](#@Specification_1_mint_script)
    -  [Function `mutate_token_properties`](#@Specification_1_mutate_token_properties)
    -  [Function `direct_transfer_script`](#@Specification_1_direct_transfer_script)
    -  [Function `opt_in_direct_transfer`](#@Specification_1_opt_in_direct_transfer)
    -  [Function `transfer_with_opt_in`](#@Specification_1_transfer_with_opt_in)
    -  [Function `burn_by_creator`](#@Specification_1_burn_by_creator)
    -  [Function `burn`](#@Specification_1_burn)
    -  [Function `mutate_collection_description`](#@Specification_1_mutate_collection_description)
    -  [Function `mutate_collection_uri`](#@Specification_1_mutate_collection_uri)
    -  [Function `mutate_collection_maximum`](#@Specification_1_mutate_collection_maximum)
    -  [Function `mutate_tokendata_maximum`](#@Specification_1_mutate_tokendata_maximum)
    -  [Function `mutate_tokendata_uri`](#@Specification_1_mutate_tokendata_uri)
    -  [Function `mutate_tokendata_royalty`](#@Specification_1_mutate_tokendata_royalty)
    -  [Function `mutate_tokendata_description`](#@Specification_1_mutate_tokendata_description)
    -  [Function `mutate_tokendata_property`](#@Specification_1_mutate_tokendata_property)
    -  [Function `mutate_one_token`](#@Specification_1_mutate_one_token)
    -  [Function `create_royalty`](#@Specification_1_create_royalty)
    -  [Function `deposit_token`](#@Specification_1_deposit_token)
    -  [Function `direct_deposit_with_opt_in`](#@Specification_1_direct_deposit_with_opt_in)
    -  [Function `direct_transfer`](#@Specification_1_direct_transfer)
    -  [Function `initialize_token_store`](#@Specification_1_initialize_token_store)
    -  [Function `merge`](#@Specification_1_merge)
    -  [Function `split`](#@Specification_1_split)
    -  [Function `transfer`](#@Specification_1_transfer)
    -  [Function `withdraw_with_capability`](#@Specification_1_withdraw_with_capability)
    -  [Function `partial_withdraw_with_capability`](#@Specification_1_partial_withdraw_with_capability)
    -  [Function `withdraw_token`](#@Specification_1_withdraw_token)
    -  [Function `create_collection`](#@Specification_1_create_collection)
    -  [Function `check_collection_exists`](#@Specification_1_check_collection_exists)
    -  [Function `check_tokendata_exists`](#@Specification_1_check_tokendata_exists)
    -  [Function `create_tokendata`](#@Specification_1_create_tokendata)
    -  [Function `get_collection_supply`](#@Specification_1_get_collection_supply)
    -  [Function `get_collection_description`](#@Specification_1_get_collection_description)
    -  [Function `get_collection_uri`](#@Specification_1_get_collection_uri)
    -  [Function `get_collection_maximum`](#@Specification_1_get_collection_maximum)
    -  [Function `get_token_supply`](#@Specification_1_get_token_supply)
    -  [Function `get_tokendata_largest_property_version`](#@Specification_1_get_tokendata_largest_property_version)
    -  [Function `create_token_mutability_config`](#@Specification_1_create_token_mutability_config)
    -  [Function `create_collection_mutability_config`](#@Specification_1_create_collection_mutability_config)
    -  [Function `mint_token`](#@Specification_1_mint_token)
    -  [Function `mint_token_to`](#@Specification_1_mint_token_to)
    -  [Function `create_token_data_id`](#@Specification_1_create_token_data_id)
    -  [Function `create_token_id_raw`](#@Specification_1_create_token_id_raw)
    -  [Function `get_royalty`](#@Specification_1_get_royalty)
    -  [Function `get_property_map`](#@Specification_1_get_property_map)
    -  [Function `get_tokendata_maximum`](#@Specification_1_get_tokendata_maximum)
    -  [Function `get_tokendata_uri`](#@Specification_1_get_tokendata_uri)
    -  [Function `get_tokendata_description`](#@Specification_1_get_tokendata_description)
    -  [Function `get_tokendata_royalty`](#@Specification_1_get_tokendata_royalty)
    -  [Function `get_tokendata_mutability_config`](#@Specification_1_get_tokendata_mutability_config)
    -  [Function `get_collection_mutability_config`](#@Specification_1_get_collection_mutability_config)
    -  [Function `withdraw_with_event_internal`](#@Specification_1_withdraw_with_event_internal)
    -  [Function `update_token_property_internal`](#@Specification_1_update_token_property_internal)
    -  [Function `direct_deposit`](#@Specification_1_direct_deposit)
    -  [Function `assert_collection_exists`](#@Specification_1_assert_collection_exists)
    -  [Function `assert_tokendata_exists`](#@Specification_1_assert_tokendata_exists)
    -  [Function `assert_non_standard_reserved_property`](#@Specification_1_assert_non_standard_reserved_property)
    -  [Function `initialize_token_script`](#@Specification_1_initialize_token_script)
    -  [Function `initialize_token`](#@Specification_1_initialize_token)


```move
module 0x3::token {
    use 0x1::account;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::table;
    use 0x1::timestamp;
    use 0x3::property_map;
    use 0x3::token_event_store;
}
```


<a id="0x3_token_Token"></a>

## Struct `Token`



```move
module 0x3::token {
    struct Token has store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>
 the amount of tokens. Only property_version &#61; 0 can have a value bigger than 1.
</dd>
<dt>
`token_properties: property_map::PropertyMap`
</dt>
<dd>
 The properties with this token.
 when property_version &#61; 0, the token_properties are the same as default_properties in TokenData, we don&apos;t store it.
 when the property_map mutates, a new property_version is assigned to the token.
</dd>
</dl>


<a id="0x3_token_TokenId"></a>

## Struct `TokenId`

global unique identifier of a token


```move
module 0x3::token {
    struct TokenId has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`token_data_id: token::TokenDataId`
</dt>
<dd>
 the id to the common token data shared by token with different property_version
</dd>
<dt>
`property_version: u64`
</dt>
<dd>
 The version of the property map; when a fungible token is mutated, a new property version is created and assigned to the token to make it an NFT
</dd>
</dl>


<a id="0x3_token_TokenDataId"></a>

## Struct `TokenDataId`

globally unique identifier of tokendata


```move
module 0x3::token {
    struct TokenDataId has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`creator: address`
</dt>
<dd>
 The address of the creator, eg: 0xcafe
</dd>
<dt>
`collection: string::String`
</dt>
<dd>
 The name of collection; this is unique under the same account, eg: &quot;Aptos Animal Collection&quot;
</dd>
<dt>
`name: string::String`
</dt>
<dd>
 The name of the token; this is the same as the name field of TokenData
</dd>
</dl>


<a id="0x3_token_TokenData"></a>

## Struct `TokenData`

The shared TokenData by tokens with different property_version


```move
module 0x3::token {
    struct TokenData has store
}
```


##### Fields


<dl>
<dt>
`maximum: u64`
</dt>
<dd>
 The maximal number of tokens that can be minted under this TokenData; if the maximum is 0, there is no limit
</dd>
<dt>
`largest_property_version: u64`
</dt>
<dd>
 The current largest property version of all tokens with this TokenData
</dd>
<dt>
`supply: u64`
</dt>
<dd>
 The number of tokens with this TokenData. Supply is only tracked for the limited token whose maximum is not 0
</dd>
<dt>
`uri: string::String`
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off&#45;chain storage; the URL length should be less than 512 characters, eg: https://arweave.net/Fmmn4ul&#45;7Mv6vzm7JwE69O&#45;I&#45;vd6Bz2QriJO1niwCh4
</dd>
<dt>
`royalty: token::Royalty`
</dt>
<dd>
 The denominator and numerator for calculating the royalty fee; it also contains payee account address for depositing the Royalty
</dd>
<dt>
`name: string::String`
</dt>
<dd>
 The name of the token, which should be unique within the collection; the length of name should be smaller than 128, characters, eg: &quot;Aptos Animal #1234&quot;
</dd>
<dt>
`description: string::String`
</dt>
<dd>
 Describes this Token
</dd>
<dt>
`default_properties: property_map::PropertyMap`
</dt>
<dd>
 The properties are stored in the TokenData that are shared by all tokens
</dd>
<dt>
`mutability_config: token::TokenMutabilityConfig`
</dt>
<dd>
 Control the TokenData field mutability
</dd>
</dl>


<a id="0x3_token_Royalty"></a>

## Struct `Royalty`

The royalty of a token


```move
module 0x3::token {
    struct Royalty has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`royalty_points_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`royalty_points_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`payee_address: address`
</dt>
<dd>
 if the token is jointly owned by multiple creators, the group of creators should create a shared account.
 the payee_address will be the shared account address.
</dd>
</dl>


<a id="0x3_token_TokenMutabilityConfig"></a>

## Struct `TokenMutabilityConfig`

This config specifies which fields in the TokenData are mutable


```move
module 0x3::token {
    struct TokenMutabilityConfig has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`maximum: bool`
</dt>
<dd>
 control if the token maximum is mutable
</dd>
<dt>
`uri: bool`
</dt>
<dd>
 control if the token uri is mutable
</dd>
<dt>
`royalty: bool`
</dt>
<dd>
 control if the token royalty is mutable
</dd>
<dt>
`description: bool`
</dt>
<dd>
 control if the token description is mutable
</dd>
<dt>
`properties: bool`
</dt>
<dd>
 control if the property map is mutable
</dd>
</dl>


<a id="0x3_token_TokenStore"></a>

## Resource `TokenStore`

Represents token resources owned by token owner


```move
module 0x3::token {
    struct TokenStore has key
}
```


##### Fields


<dl>
<dt>
`tokens: table::Table<token::TokenId, token::Token>`
</dt>
<dd>
 the tokens owned by a token owner
</dd>
<dt>
`direct_transfer: bool`
</dt>
<dd>

</dd>
<dt>
`deposit_events: event::EventHandle<token::DepositEvent>`
</dt>
<dd>

</dd>
<dt>
`withdraw_events: event::EventHandle<token::WithdrawEvent>`
</dt>
<dd>

</dd>
<dt>
`burn_events: event::EventHandle<token::BurnTokenEvent>`
</dt>
<dd>

</dd>
<dt>
`mutate_token_property_events: event::EventHandle<token::MutateTokenPropertyMapEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CollectionMutabilityConfig"></a>

## Struct `CollectionMutabilityConfig`

This config specifies which fields in the Collection are mutable


```move
module 0x3::token {
    struct CollectionMutabilityConfig has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`description: bool`
</dt>
<dd>
 control if description is mutable
</dd>
<dt>
`uri: bool`
</dt>
<dd>
 control if uri is mutable
</dd>
<dt>
`maximum: bool`
</dt>
<dd>
 control if collection maxium is mutable
</dd>
</dl>


<a id="0x3_token_Collections"></a>

## Resource `Collections`

Represent collection and token metadata for a creator


```move
module 0x3::token {
    struct Collections has key
}
```


##### Fields


<dl>
<dt>
`collection_data: table::Table<string::String, token::CollectionData>`
</dt>
<dd>

</dd>
<dt>
`token_data: table::Table<token::TokenDataId, token::TokenData>`
</dt>
<dd>

</dd>
<dt>
`create_collection_events: event::EventHandle<token::CreateCollectionEvent>`
</dt>
<dd>

</dd>
<dt>
`create_token_data_events: event::EventHandle<token::CreateTokenDataEvent>`
</dt>
<dd>

</dd>
<dt>
`mint_token_events: event::EventHandle<token::MintTokenEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CollectionData"></a>

## Struct `CollectionData`

Represent the collection metadata


```move
module 0x3::token {
    struct CollectionData has store
}
```


##### Fields


<dl>
<dt>
`description: string::String`
</dt>
<dd>
 A description for the token collection Eg: &quot;Aptos Toad Overload&quot;
</dd>
<dt>
`name: string::String`
</dt>
<dd>
 The collection name, which should be unique among all collections by the creator; the name should also be smaller than 128 characters, eg: &quot;Animal Collection&quot;
</dd>
<dt>
`uri: string::String`
</dt>
<dd>
 The URI for the collection; its length should be smaller than 512 characters
</dd>
<dt>
`supply: u64`
</dt>
<dd>
 The number of different TokenData entries in this collection
</dd>
<dt>
`maximum: u64`
</dt>
<dd>
 If maximal is a non&#45;zero value, the number of created TokenData entries should be smaller or equal to this maximum
 If maximal is 0, Aptos doesn&apos;t track the supply of this collection, and there is no limit
</dd>
<dt>
`mutability_config: token::CollectionMutabilityConfig`
</dt>
<dd>
 control which collectionData field is mutable
</dd>
</dl>


<a id="0x3_token_WithdrawCapability"></a>

## Struct `WithdrawCapability`

capability to withdraw without signer, this struct should be non&#45;copyable


```move
module 0x3::token {
    struct WithdrawCapability has drop, store
}
```


##### Fields


<dl>
<dt>
`token_owner: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
<dt>
`expiration_sec: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_DepositEvent"></a>

## Struct `DepositEvent`

Set of data sent to the event stream during a receive


```move
module 0x3::token {
    struct DepositEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_Deposit"></a>

## Struct `Deposit`

Set of data sent to the event stream during a receive


```move
module 0x3::token {
    #[event]
    struct Deposit has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_WithdrawEvent"></a>

## Struct `WithdrawEvent`

Set of data sent to the event stream during a withdrawal


```move
module 0x3::token {
    struct WithdrawEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_Withdraw"></a>

## Struct `Withdraw`

Set of data sent to the event stream during a withdrawal


```move
module 0x3::token {
    #[event]
    struct Withdraw has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CreateTokenDataEvent"></a>

## Struct `CreateTokenDataEvent`

token creation event id of token created


```move
module 0x3::token {
    struct CreateTokenDataEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenDataId`
</dt>
<dd>

</dd>
<dt>
`description: string::String`
</dt>
<dd>

</dd>
<dt>
`maximum: u64`
</dt>
<dd>

</dd>
<dt>
`uri: string::String`
</dt>
<dd>

</dd>
<dt>
`royalty_payee_address: address`
</dt>
<dd>

</dd>
<dt>
`royalty_points_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`royalty_points_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`name: string::String`
</dt>
<dd>

</dd>
<dt>
`mutability_config: token::TokenMutabilityConfig`
</dt>
<dd>

</dd>
<dt>
`property_keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`property_values: vector<vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`property_types: vector<string::String>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CreateTokenData"></a>

## Struct `CreateTokenData`



```move
module 0x3::token {
    #[event]
    struct CreateTokenData has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenDataId`
</dt>
<dd>

</dd>
<dt>
`description: string::String`
</dt>
<dd>

</dd>
<dt>
`maximum: u64`
</dt>
<dd>

</dd>
<dt>
`uri: string::String`
</dt>
<dd>

</dd>
<dt>
`royalty_payee_address: address`
</dt>
<dd>

</dd>
<dt>
`royalty_points_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`royalty_points_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`name: string::String`
</dt>
<dd>

</dd>
<dt>
`mutability_config: token::TokenMutabilityConfig`
</dt>
<dd>

</dd>
<dt>
`property_keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`property_values: vector<vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`property_types: vector<string::String>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_MintTokenEvent"></a>

## Struct `MintTokenEvent`

mint token event. This event triggered when creator adds more supply to existing token


```move
module 0x3::token {
    struct MintTokenEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenDataId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_MintToken"></a>

## Struct `MintToken`



```move
module 0x3::token {
    #[event]
    struct MintToken has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenDataId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_BurnTokenEvent"></a>

## Struct `BurnTokenEvent`



```move
module 0x3::token {
    struct BurnTokenEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_BurnToken"></a>

## Struct `BurnToken`



```move
module 0x3::token {
    #[event]
    struct BurnToken has drop, store
}
```


##### Fields


<dl>
<dt>
`id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_MutateTokenPropertyMapEvent"></a>

## Struct `MutateTokenPropertyMapEvent`



```move
module 0x3::token {
    struct MutateTokenPropertyMapEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`old_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`new_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`values: vector<vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`types: vector<string::String>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_MutateTokenPropertyMap"></a>

## Struct `MutateTokenPropertyMap`



```move
module 0x3::token {
    #[event]
    struct MutateTokenPropertyMap has drop, store
}
```


##### Fields


<dl>
<dt>
`old_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`new_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`values: vector<vector<u8>>`
</dt>
<dd>

</dd>
<dt>
`types: vector<string::String>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CreateCollectionEvent"></a>

## Struct `CreateCollectionEvent`

create collection event with creator address and collection name


```move
module 0x3::token {
    struct CreateCollectionEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`creator: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`uri: string::String`
</dt>
<dd>

</dd>
<dt>
`description: string::String`
</dt>
<dd>

</dd>
<dt>
`maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_CreateCollection"></a>

## Struct `CreateCollection`



```move
module 0x3::token {
    #[event]
    struct CreateCollection has drop, store
}
```


##### Fields


<dl>
<dt>
`creator: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`uri: string::String`
</dt>
<dd>

</dd>
<dt>
`description: string::String`
</dt>
<dd>

</dd>
<dt>
`maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x3_token_EINSUFFICIENT_BALANCE"></a>

Insufficient token balance


```move
module 0x3::token {
    const EINSUFFICIENT_BALANCE: u64 = 5;
}
```


<a id="0x3_token_EURI_TOO_LONG"></a>

The URI is too long


```move
module 0x3::token {
    const EURI_TOO_LONG: u64 = 27;
}
```


<a id="0x3_token_MAX_URI_LENGTH"></a>



```move
module 0x3::token {
    const MAX_URI_LENGTH: u64 = 512;
}
```


<a id="0x3_token_BURNABLE_BY_CREATOR"></a>



```move
module 0x3::token {
    const BURNABLE_BY_CREATOR: vector<u8> = [84, 79, 75, 69, 78, 95, 66, 85, 82, 78, 65, 66, 76, 69, 95, 66, 89, 95, 67, 82, 69, 65, 84, 79, 82];
}
```


<a id="0x3_token_BURNABLE_BY_OWNER"></a>



```move
module 0x3::token {
    const BURNABLE_BY_OWNER: vector<u8> = [84, 79, 75, 69, 78, 95, 66, 85, 82, 78, 65, 66, 76, 69, 95, 66, 89, 95, 79, 87, 78, 69, 82];
}
```


<a id="0x3_token_COLLECTION_DESCRIPTION_MUTABLE_IND"></a>



```move
module 0x3::token {
    const COLLECTION_DESCRIPTION_MUTABLE_IND: u64 = 0;
}
```


<a id="0x3_token_COLLECTION_MAX_MUTABLE_IND"></a>



```move
module 0x3::token {
    const COLLECTION_MAX_MUTABLE_IND: u64 = 2;
}
```


<a id="0x3_token_COLLECTION_URI_MUTABLE_IND"></a>



```move
module 0x3::token {
    const COLLECTION_URI_MUTABLE_IND: u64 = 1;
}
```


<a id="0x3_token_EALREADY_HAS_BALANCE"></a>

The token has balance and cannot be initialized


```move
module 0x3::token {
    const EALREADY_HAS_BALANCE: u64 = 0;
}
```


<a id="0x3_token_ECANNOT_UPDATE_RESERVED_PROPERTY"></a>

Reserved fields for token contract
Cannot be updated by user


```move
module 0x3::token {
    const ECANNOT_UPDATE_RESERVED_PROPERTY: u64 = 32;
}
```


<a id="0x3_token_ECOLLECTIONS_NOT_PUBLISHED"></a>

There isn&apos;t any collection under this account


```move
module 0x3::token {
    const ECOLLECTIONS_NOT_PUBLISHED: u64 = 1;
}
```


<a id="0x3_token_ECOLLECTION_ALREADY_EXISTS"></a>

The collection already exists


```move
module 0x3::token {
    const ECOLLECTION_ALREADY_EXISTS: u64 = 3;
}
```


<a id="0x3_token_ECOLLECTION_NAME_TOO_LONG"></a>

The collection name is too long


```move
module 0x3::token {
    const ECOLLECTION_NAME_TOO_LONG: u64 = 25;
}
```


<a id="0x3_token_ECOLLECTION_NOT_PUBLISHED"></a>

Cannot find collection in creator&apos;s account


```move
module 0x3::token {
    const ECOLLECTION_NOT_PUBLISHED: u64 = 2;
}
```


<a id="0x3_token_ECREATE_WOULD_EXCEED_COLLECTION_MAXIMUM"></a>

Exceeds the collection&apos;s maximal number of token_data


```move
module 0x3::token {
    const ECREATE_WOULD_EXCEED_COLLECTION_MAXIMUM: u64 = 4;
}
```


<a id="0x3_token_ECREATOR_CANNOT_BURN_TOKEN"></a>

Token is not burnable by creator


```move
module 0x3::token {
    const ECREATOR_CANNOT_BURN_TOKEN: u64 = 31;
}
```


<a id="0x3_token_EFIELD_NOT_MUTABLE"></a>

The field is not mutable


```move
module 0x3::token {
    const EFIELD_NOT_MUTABLE: u64 = 13;
}
```


<a id="0x3_token_EINSUFFICIENT_WITHDRAW_CAPABILITY_AMOUNT"></a>

Withdraw capability doesn&apos;t have sufficient amount


```move
module 0x3::token {
    const EINSUFFICIENT_WITHDRAW_CAPABILITY_AMOUNT: u64 = 38;
}
```


<a id="0x3_token_EINVALID_MAXIMUM"></a>

Collection or tokendata maximum must be larger than supply


```move
module 0x3::token {
    const EINVALID_MAXIMUM: u64 = 36;
}
```


<a id="0x3_token_EINVALID_ROYALTY_NUMERATOR_DENOMINATOR"></a>

Royalty invalid if the numerator is larger than the denominator


```move
module 0x3::token {
    const EINVALID_ROYALTY_NUMERATOR_DENOMINATOR: u64 = 34;
}
```


<a id="0x3_token_EINVALID_TOKEN_MERGE"></a>

Cannot merge the two tokens with different token id


```move
module 0x3::token {
    const EINVALID_TOKEN_MERGE: u64 = 6;
}
```


<a id="0x3_token_EMINT_WOULD_EXCEED_TOKEN_MAXIMUM"></a>

Exceed the token data maximal allowed


```move
module 0x3::token {
    const EMINT_WOULD_EXCEED_TOKEN_MAXIMUM: u64 = 7;
}
```


<a id="0x3_token_ENFT_NAME_TOO_LONG"></a>

The NFT name is too long


```move
module 0x3::token {
    const ENFT_NAME_TOO_LONG: u64 = 26;
}
```


<a id="0x3_token_ENFT_NOT_SPLITABLE"></a>

Cannot split a token that only has 1 amount


```move
module 0x3::token {
    const ENFT_NOT_SPLITABLE: u64 = 18;
}
```


<a id="0x3_token_ENO_BURN_CAPABILITY"></a>

No burn capability


```move
module 0x3::token {
    const ENO_BURN_CAPABILITY: u64 = 8;
}
```


<a id="0x3_token_ENO_BURN_TOKEN_WITH_ZERO_AMOUNT"></a>

Cannot burn 0 Token


```move
module 0x3::token {
    const ENO_BURN_TOKEN_WITH_ZERO_AMOUNT: u64 = 29;
}
```


<a id="0x3_token_ENO_DEPOSIT_TOKEN_WITH_ZERO_AMOUNT"></a>

Cannot deposit a Token with 0 amount


```move
module 0x3::token {
    const ENO_DEPOSIT_TOKEN_WITH_ZERO_AMOUNT: u64 = 28;
}
```


<a id="0x3_token_ENO_MINT_CAPABILITY"></a>

No mint capability


```move
module 0x3::token {
    const ENO_MINT_CAPABILITY: u64 = 19;
}
```


<a id="0x3_token_ENO_MUTATE_CAPABILITY"></a>

Not authorized to mutate


```move
module 0x3::token {
    const ENO_MUTATE_CAPABILITY: u64 = 14;
}
```


<a id="0x3_token_ENO_TOKEN_IN_TOKEN_STORE"></a>

Token not in the token store


```move
module 0x3::token {
    const ENO_TOKEN_IN_TOKEN_STORE: u64 = 15;
}
```


<a id="0x3_token_EOWNER_CANNOT_BURN_TOKEN"></a>

Token is not burnable by owner


```move
module 0x3::token {
    const EOWNER_CANNOT_BURN_TOKEN: u64 = 30;
}
```


<a id="0x3_token_EPROPERTY_RESERVED_BY_STANDARD"></a>

The property is reserved by token standard


```move
module 0x3::token {
    const EPROPERTY_RESERVED_BY_STANDARD: u64 = 40;
}
```


<a id="0x3_token_EROYALTY_PAYEE_ACCOUNT_DOES_NOT_EXIST"></a>

Royalty payee account does not exist


```move
module 0x3::token {
    const EROYALTY_PAYEE_ACCOUNT_DOES_NOT_EXIST: u64 = 35;
}
```


<a id="0x3_token_ETOKEN_CANNOT_HAVE_ZERO_AMOUNT"></a>

TOKEN with 0 amount is not allowed


```move
module 0x3::token {
    const ETOKEN_CANNOT_HAVE_ZERO_AMOUNT: u64 = 33;
}
```


<a id="0x3_token_ETOKEN_DATA_ALREADY_EXISTS"></a>

TokenData already exists


```move
module 0x3::token {
    const ETOKEN_DATA_ALREADY_EXISTS: u64 = 9;
}
```


<a id="0x3_token_ETOKEN_DATA_NOT_PUBLISHED"></a>

TokenData not published


```move
module 0x3::token {
    const ETOKEN_DATA_NOT_PUBLISHED: u64 = 10;
}
```


<a id="0x3_token_ETOKEN_PROPERTIES_COUNT_NOT_MATCH"></a>

Token Properties count doesn&apos;t match


```move
module 0x3::token {
    const ETOKEN_PROPERTIES_COUNT_NOT_MATCH: u64 = 37;
}
```


<a id="0x3_token_ETOKEN_SPLIT_AMOUNT_LARGER_OR_EQUAL_TO_TOKEN_AMOUNT"></a>

Cannot split token to an amount larger than its amount


```move
module 0x3::token {
    const ETOKEN_SPLIT_AMOUNT_LARGER_OR_EQUAL_TO_TOKEN_AMOUNT: u64 = 12;
}
```


<a id="0x3_token_ETOKEN_STORE_NOT_PUBLISHED"></a>

TokenStore doesn&apos;t exist


```move
module 0x3::token {
    const ETOKEN_STORE_NOT_PUBLISHED: u64 = 11;
}
```


<a id="0x3_token_EUSER_NOT_OPT_IN_DIRECT_TRANSFER"></a>

User didn&apos;t opt&#45;in direct transfer


```move
module 0x3::token {
    const EUSER_NOT_OPT_IN_DIRECT_TRANSFER: u64 = 16;
}
```


<a id="0x3_token_EWITHDRAW_PROOF_EXPIRES"></a>

Withdraw proof expires


```move
module 0x3::token {
    const EWITHDRAW_PROOF_EXPIRES: u64 = 39;
}
```


<a id="0x3_token_EWITHDRAW_ZERO"></a>

Cannot withdraw 0 token


```move
module 0x3::token {
    const EWITHDRAW_ZERO: u64 = 17;
}
```


<a id="0x3_token_MAX_COLLECTION_NAME_LENGTH"></a>



```move
module 0x3::token {
    const MAX_COLLECTION_NAME_LENGTH: u64 = 128;
}
```


<a id="0x3_token_MAX_NFT_NAME_LENGTH"></a>



```move
module 0x3::token {
    const MAX_NFT_NAME_LENGTH: u64 = 128;
}
```


<a id="0x3_token_TOKEN_DESCRIPTION_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_DESCRIPTION_MUTABLE_IND: u64 = 3;
}
```


<a id="0x3_token_TOKEN_MAX_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_MAX_MUTABLE_IND: u64 = 0;
}
```


<a id="0x3_token_TOKEN_PROPERTY_MUTABLE"></a>



```move
module 0x3::token {
    const TOKEN_PROPERTY_MUTABLE: vector<u8> = [84, 79, 75, 69, 78, 95, 80, 82, 79, 80, 69, 82, 84, 89, 95, 77, 85, 84, 65, 84, 66, 76, 69];
}
```


<a id="0x3_token_TOKEN_PROPERTY_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_PROPERTY_MUTABLE_IND: u64 = 4;
}
```


<a id="0x3_token_TOKEN_PROPERTY_VALUE_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_PROPERTY_VALUE_MUTABLE_IND: u64 = 5;
}
```


<a id="0x3_token_TOKEN_ROYALTY_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_ROYALTY_MUTABLE_IND: u64 = 2;
}
```


<a id="0x3_token_TOKEN_URI_MUTABLE_IND"></a>



```move
module 0x3::token {
    const TOKEN_URI_MUTABLE_IND: u64 = 1;
}
```


<a id="0x3_token_create_collection_script"></a>

## Function `create_collection_script`

create a empty token collection with parameters


```move
module 0x3::token {
    public entry fun create_collection_script(creator: &signer, name: string::String, description: string::String, uri: string::String, maximum: u64, mutate_setting: vector<bool>)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun create_collection_script(
        creator: &signer,
        name: String,
        description: String,
        uri: String,
        maximum: u64,
        mutate_setting: vector<bool>,
    ) acquires Collections {
        create_collection(
            creator,
            name,
            description,
            uri,
            maximum,
            mutate_setting
        );
    }
}
```


<a id="0x3_token_create_token_script"></a>

## Function `create_token_script`

create token with raw inputs


```move
module 0x3::token {
    public entry fun create_token_script(account: &signer, collection: string::String, name: string::String, description: string::String, balance: u64, maximum: u64, uri: string::String, royalty_payee_address: address, royalty_points_denominator: u64, royalty_points_numerator: u64, mutate_setting: vector<bool>, property_keys: vector<string::String>, property_values: vector<vector<u8>>, property_types: vector<string::String>)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun create_token_script(
        account: &signer,
        collection: String,
        name: String,
        description: String,
        balance: u64,
        maximum: u64,
        uri: String,
        royalty_payee_address: address,
        royalty_points_denominator: u64,
        royalty_points_numerator: u64,
        mutate_setting: vector<bool>,
        property_keys: vector<String>,
        property_values: vector<vector<u8>>,
        property_types: vector<String>
    ) acquires Collections, TokenStore {
        let token_mut_config = create_token_mutability_config(&mutate_setting);
        let tokendata_id = create_tokendata(
            account,
            collection,
            name,
            description,
            maximum,
            uri,
            royalty_payee_address,
            royalty_points_denominator,
            royalty_points_numerator,
            token_mut_config,
            property_keys,
            property_values,
            property_types
        );

        mint_token(
            account,
            tokendata_id,
            balance,
        );
    }
}
```


<a id="0x3_token_mint_script"></a>

## Function `mint_script`

Mint more token from an existing token_data. Mint only adds more token to property_version 0


```move
module 0x3::token {
    public entry fun mint_script(account: &signer, token_data_address: address, collection: string::String, name: string::String, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun mint_script(
        account: &signer,
        token_data_address: address,
        collection: String,
        name: String,
        amount: u64,
    ) acquires Collections, TokenStore {
        let token_data_id = create_token_data_id(
            token_data_address,
            collection,
            name,
        );
        // only creator of the tokendata can mint more tokens for now
        assert!(token_data_id.creator == signer::address_of(account), error::permission_denied(ENO_MINT_CAPABILITY));
        mint_token(
            account,
            token_data_id,
            amount,
        );
    }
}
```


<a id="0x3_token_mutate_token_properties"></a>

## Function `mutate_token_properties`

mutate the token property and save the new property in TokenStore
if the token property_version is 0, we will create a new property_version per token to generate a new token_id per token
if the token property_version is not 0, we will just update the propertyMap and use the existing token_id (property_version)


```move
module 0x3::token {
    public entry fun mutate_token_properties(account: &signer, token_owner: address, creator: address, collection_name: string::String, token_name: string::String, token_property_version: u64, amount: u64, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun mutate_token_properties(
        account: &signer,
        token_owner: address,
        creator: address,
        collection_name: String,
        token_name: String,
        token_property_version: u64,
        amount: u64,
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>,
    ) acquires Collections, TokenStore {
        assert!(signer::address_of(account) == creator, error::not_found(ENO_MUTATE_CAPABILITY));
        let i = 0;
        let token_id = create_token_id_raw(
            creator,
            collection_name,
            token_name,
            token_property_version,
        );
        // give a new property_version for each token
        while (i < amount) {
            mutate_one_token(account, token_owner, token_id, keys, values, types);
            i = i + 1;
        };
    }
}
```


<a id="0x3_token_direct_transfer_script"></a>

## Function `direct_transfer_script`



```move
module 0x3::token {
    public entry fun direct_transfer_script(sender: &signer, receiver: &signer, creators_address: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun direct_transfer_script(
        sender: &signer,
        receiver: &signer,
        creators_address: address,
        collection: String,
        name: String,
        property_version: u64,
        amount: u64,
    ) acquires TokenStore {
        let token_id = create_token_id_raw(creators_address, collection, name, property_version);
        direct_transfer(sender, receiver, token_id, amount);
    }
}
```


<a id="0x3_token_opt_in_direct_transfer"></a>

## Function `opt_in_direct_transfer`



```move
module 0x3::token {
    public entry fun opt_in_direct_transfer(account: &signer, opt_in: bool)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun opt_in_direct_transfer(account: &signer, opt_in: bool) acquires TokenStore {
        let addr = signer::address_of(account);
        initialize_token_store(account);
        let opt_in_flag = &mut borrow_global_mut<TokenStore>(addr).direct_transfer;
        *opt_in_flag = opt_in;
        token_event_store::emit_token_opt_in_event(account, opt_in);
    }
}
```


<a id="0x3_token_transfer_with_opt_in"></a>

## Function `transfer_with_opt_in`

Transfers `amount` of tokens from `from` to `to`.
The receiver `to` has to opt&#45;in direct transfer first


```move
module 0x3::token {
    public entry fun transfer_with_opt_in(from: &signer, creator: address, collection_name: string::String, token_name: string::String, token_property_version: u64, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun transfer_with_opt_in(
        from: &signer,
        creator: address,
        collection_name: String,
        token_name: String,
        token_property_version: u64,
        to: address,
        amount: u64,
    ) acquires TokenStore {
        let token_id = create_token_id_raw(creator, collection_name, token_name, token_property_version);
        transfer(from, token_id, to, amount);
    }
}
```


<a id="0x3_token_burn_by_creator"></a>

## Function `burn_by_creator`

Burn a token by creator when the token&apos;s BURNABLE_BY_CREATOR is true
The token is owned at address owner


```move
module 0x3::token {
    public entry fun burn_by_creator(creator: &signer, owner: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun burn_by_creator(
        creator: &signer,
        owner: address,
        collection: String,
        name: String,
        property_version: u64,
        amount: u64,
    ) acquires Collections, TokenStore {
        let creator_address = signer::address_of(creator);
        assert!(amount > 0, error::invalid_argument(ENO_BURN_TOKEN_WITH_ZERO_AMOUNT));
        let token_id = create_token_id_raw(creator_address, collection, name, property_version);
        let creator_addr = token_id.token_data_id.creator;
        assert!(
            exists<Collections>(creator_addr),
            error::not_found(ECOLLECTIONS_NOT_PUBLISHED),
        );

        let collections = borrow_global_mut<Collections>(creator_address);
        assert!(
            table::contains(&collections.token_data, token_id.token_data_id),
            error::not_found(ETOKEN_DATA_NOT_PUBLISHED),
        );

        let token_data = table::borrow_mut(
            &mut collections.token_data,
            token_id.token_data_id,
        );

        // The property should be explicitly set in the property_map for creator to burn the token
        assert!(
            property_map::contains_key(&token_data.default_properties, &string::utf8(BURNABLE_BY_CREATOR)),
            error::permission_denied(ECREATOR_CANNOT_BURN_TOKEN)
        );

        let burn_by_creator_flag = property_map::read_bool(&token_data.default_properties, &string::utf8(BURNABLE_BY_CREATOR));
        assert!(burn_by_creator_flag, error::permission_denied(ECREATOR_CANNOT_BURN_TOKEN));

        // Burn the tokens.
        let Token { id: _, amount: burned_amount, token_properties: _ } = withdraw_with_event_internal(owner, token_id, amount);
        let token_store = borrow_global_mut<TokenStore>(owner);
        if (std::features::module_event_migration_enabled()) {
            event::emit(BurnToken { id: token_id, amount: burned_amount });
        };
        event::emit_event<BurnTokenEvent>(
            &mut token_store.burn_events,
            BurnTokenEvent { id: token_id, amount: burned_amount }
        );

        if (token_data.maximum > 0) {
            token_data.supply = token_data.supply - burned_amount;

            // Delete the token_data if supply drops to 0.
            if (token_data.supply == 0) {
                destroy_token_data(table::remove(&mut collections.token_data, token_id.token_data_id));

                // update the collection supply
                let collection_data = table::borrow_mut(
                    &mut collections.collection_data,
                    token_id.token_data_id.collection
                );
                if (collection_data.maximum > 0) {
                    collection_data.supply = collection_data.supply - 1;
                    // delete the collection data if the collection supply equals 0
                    if (collection_data.supply == 0) {
                        destroy_collection_data(table::remove(&mut collections.collection_data, collection_data.name));
                    };
                };
            };
        };
    }
}
```


<a id="0x3_token_burn"></a>

## Function `burn`

Burn a token by the token owner


```move
module 0x3::token {
    public entry fun burn(owner: &signer, creators_address: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun burn(
        owner: &signer,
        creators_address: address,
        collection: String,
        name: String,
        property_version: u64,
        amount: u64
    ) acquires Collections, TokenStore {
        assert!(amount > 0, error::invalid_argument(ENO_BURN_TOKEN_WITH_ZERO_AMOUNT));
        let token_id = create_token_id_raw(creators_address, collection, name, property_version);
        let creator_addr = token_id.token_data_id.creator;
        assert!(
            exists<Collections>(creator_addr),
            error::not_found(ECOLLECTIONS_NOT_PUBLISHED),
        );

        let collections = borrow_global_mut<Collections>(creator_addr);
        assert!(
            table::contains(&collections.token_data, token_id.token_data_id),
            error::not_found(ETOKEN_DATA_NOT_PUBLISHED),
        );

        let token_data = table::borrow_mut(
            &mut collections.token_data,
            token_id.token_data_id,
        );

        assert!(
            property_map::contains_key(&token_data.default_properties, &string::utf8(BURNABLE_BY_OWNER)),
            error::permission_denied(EOWNER_CANNOT_BURN_TOKEN)
        );
        let burn_by_owner_flag = property_map::read_bool(&token_data.default_properties, &string::utf8(BURNABLE_BY_OWNER));
        assert!(burn_by_owner_flag, error::permission_denied(EOWNER_CANNOT_BURN_TOKEN));

        // Burn the tokens.
        let Token { id: _, amount: burned_amount, token_properties: _ } = withdraw_token(owner, token_id, amount);
        let token_store = borrow_global_mut<TokenStore>(signer::address_of(owner));
        if (std::features::module_event_migration_enabled()) {
            event::emit(BurnToken { id: token_id, amount: burned_amount });
        };
        event::emit_event<BurnTokenEvent>(
            &mut token_store.burn_events,
            BurnTokenEvent { id: token_id, amount: burned_amount }
        );

        // Decrease the supply correspondingly by the amount of tokens burned.
        let token_data = table::borrow_mut(
            &mut collections.token_data,
            token_id.token_data_id,
        );

        // only update the supply if we tracking the supply and maximal
        // maximal == 0 is reserved for unlimited token and collection with no tracking info.
        if (token_data.maximum > 0) {
            token_data.supply = token_data.supply - burned_amount;

            // Delete the token_data if supply drops to 0.
            if (token_data.supply == 0) {
                destroy_token_data(table::remove(&mut collections.token_data, token_id.token_data_id));

                // update the collection supply
                let collection_data = table::borrow_mut(
                    &mut collections.collection_data,
                    token_id.token_data_id.collection
                );

                // only update and check the supply for unlimited collection
                if (collection_data.maximum > 0){
                    collection_data.supply = collection_data.supply - 1;
                    // delete the collection data if the collection supply equals 0
                    if (collection_data.supply == 0) {
                        destroy_collection_data(table::remove(&mut collections.collection_data, collection_data.name));
                    };
                };
            };
        };
    }
}
```


<a id="0x3_token_mutate_collection_description"></a>

## Function `mutate_collection_description`



```move
module 0x3::token {
    public fun mutate_collection_description(creator: &signer, collection_name: string::String, description: string::String)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_collection_description(creator: &signer, collection_name: String, description: String) acquires Collections {
        let creator_address = signer::address_of(creator);
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        assert!(collection_data.mutability_config.description, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_collection_description_mutate_event(creator, collection_name, collection_data.description, description);
        collection_data.description = description;
    }
}
```


<a id="0x3_token_mutate_collection_uri"></a>

## Function `mutate_collection_uri`



```move
module 0x3::token {
    public fun mutate_collection_uri(creator: &signer, collection_name: string::String, uri: string::String)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_collection_uri(creator: &signer, collection_name: String, uri: String) acquires Collections {
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::invalid_argument(EURI_TOO_LONG));
        let creator_address = signer::address_of(creator);
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        assert!(collection_data.mutability_config.uri, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_collection_uri_mutate_event(creator, collection_name, collection_data.uri , uri);
        collection_data.uri = uri;
    }
}
```


<a id="0x3_token_mutate_collection_maximum"></a>

## Function `mutate_collection_maximum`



```move
module 0x3::token {
    public fun mutate_collection_maximum(creator: &signer, collection_name: string::String, maximum: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_collection_maximum(creator: &signer, collection_name: String, maximum: u64) acquires Collections {
        let creator_address = signer::address_of(creator);
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        // cannot change maximum from 0 and cannot change maximum to 0
        assert!(collection_data.maximum != 0 && maximum != 0, error::invalid_argument(EINVALID_MAXIMUM));
        assert!(maximum >= collection_data.supply, error::invalid_argument(EINVALID_MAXIMUM));
        assert!(collection_data.mutability_config.maximum, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_collection_maximum_mutate_event(creator, collection_name, collection_data.maximum, maximum);
        collection_data.maximum = maximum;
    }
}
```


<a id="0x3_token_mutate_tokendata_maximum"></a>

## Function `mutate_tokendata_maximum`



```move
module 0x3::token {
    public fun mutate_tokendata_maximum(creator: &signer, token_data_id: token::TokenDataId, maximum: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_tokendata_maximum(creator: &signer, token_data_id: TokenDataId, maximum: u64) acquires Collections {
        assert_tokendata_exists(creator, token_data_id);
        let all_token_data = &mut borrow_global_mut<Collections>(token_data_id.creator).token_data;
        let token_data = table::borrow_mut(all_token_data, token_data_id);
        // cannot change maximum from 0 and cannot change maximum to 0
        assert!(token_data.maximum != 0 && maximum != 0, error::invalid_argument(EINVALID_MAXIMUM));
        assert!(maximum >= token_data.supply, error::invalid_argument(EINVALID_MAXIMUM));
        assert!(token_data.mutability_config.maximum, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_token_maximum_mutate_event(creator, token_data_id.collection, token_data_id.name, token_data.maximum, maximum);
        token_data.maximum = maximum;
    }
}
```


<a id="0x3_token_mutate_tokendata_uri"></a>

## Function `mutate_tokendata_uri`



```move
module 0x3::token {
    public fun mutate_tokendata_uri(creator: &signer, token_data_id: token::TokenDataId, uri: string::String)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_tokendata_uri(
        creator: &signer,
        token_data_id: TokenDataId,
        uri: String
    ) acquires Collections {
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::invalid_argument(EURI_TOO_LONG));
        assert_tokendata_exists(creator, token_data_id);

        let all_token_data = &mut borrow_global_mut<Collections>(token_data_id.creator).token_data;
        let token_data = table::borrow_mut(all_token_data, token_data_id);
        assert!(token_data.mutability_config.uri, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_token_uri_mutate_event(creator, token_data_id.collection, token_data_id.name, token_data.uri ,uri);
        token_data.uri = uri;
    }
}
```


<a id="0x3_token_mutate_tokendata_royalty"></a>

## Function `mutate_tokendata_royalty`



```move
module 0x3::token {
    public fun mutate_tokendata_royalty(creator: &signer, token_data_id: token::TokenDataId, royalty: token::Royalty)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_tokendata_royalty(creator: &signer, token_data_id: TokenDataId, royalty: Royalty) acquires Collections {
        assert_tokendata_exists(creator, token_data_id);

        let all_token_data = &mut borrow_global_mut<Collections>(token_data_id.creator).token_data;
        let token_data = table::borrow_mut(all_token_data, token_data_id);
        assert!(token_data.mutability_config.royalty, error::permission_denied(EFIELD_NOT_MUTABLE));

        token_event_store::emit_token_royalty_mutate_event(
            creator,
            token_data_id.collection,
            token_data_id.name,
            token_data.royalty.royalty_points_numerator,
            token_data.royalty.royalty_points_denominator,
            token_data.royalty.payee_address,
            royalty.royalty_points_numerator,
            royalty.royalty_points_denominator,
            royalty.payee_address
        );
        token_data.royalty = royalty;
    }
}
```


<a id="0x3_token_mutate_tokendata_description"></a>

## Function `mutate_tokendata_description`



```move
module 0x3::token {
    public fun mutate_tokendata_description(creator: &signer, token_data_id: token::TokenDataId, description: string::String)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_tokendata_description(creator: &signer, token_data_id: TokenDataId, description: String) acquires Collections {
        assert_tokendata_exists(creator, token_data_id);

        let all_token_data = &mut borrow_global_mut<Collections>(token_data_id.creator).token_data;
        let token_data = table::borrow_mut(all_token_data, token_data_id);
        assert!(token_data.mutability_config.description, error::permission_denied(EFIELD_NOT_MUTABLE));
        token_event_store::emit_token_descrition_mutate_event(creator, token_data_id.collection, token_data_id.name, token_data.description, description);
        token_data.description = description;
    }
}
```


<a id="0x3_token_mutate_tokendata_property"></a>

## Function `mutate_tokendata_property`

Allow creator to mutate the default properties in TokenData


```move
module 0x3::token {
    public fun mutate_tokendata_property(creator: &signer, token_data_id: token::TokenDataId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_tokendata_property(
        creator: &signer,
        token_data_id: TokenDataId,
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>,
    ) acquires Collections {
        assert_tokendata_exists(creator, token_data_id);
        let key_len = vector::length(&keys);
        let val_len = vector::length(&values);
        let typ_len = vector::length(&types);
        assert!(key_len == val_len, error::invalid_state(ETOKEN_PROPERTIES_COUNT_NOT_MATCH));
        assert!(key_len == typ_len, error::invalid_state(ETOKEN_PROPERTIES_COUNT_NOT_MATCH));

        let all_token_data = &mut borrow_global_mut<Collections>(token_data_id.creator).token_data;
        let token_data = table::borrow_mut(all_token_data, token_data_id);
        assert!(token_data.mutability_config.properties, error::permission_denied(EFIELD_NOT_MUTABLE));
        let i: u64 = 0;
        let old_values: vector<Option<PropertyValue>> = vector::empty();
        let new_values: vector<PropertyValue> = vector::empty();
        assert_non_standard_reserved_property(&keys);
        while (i < vector::length(&keys)){
            let key = vector::borrow(&keys, i);
            let old_pv = if (property_map::contains_key(&token_data.default_properties, key)) {
                option::some(*property_map::borrow(&token_data.default_properties, key))
            } else {
                option::none<PropertyValue>()
            };
            vector::push_back(&mut old_values, old_pv);
            let new_pv = property_map::create_property_value_raw(*vector::borrow(&values, i), *vector::borrow(&types, i));
            vector::push_back(&mut new_values, new_pv);
            if (option::is_some(&old_pv)) {
                property_map::update_property_value(&mut token_data.default_properties, key, new_pv);
            } else {
                property_map::add(&mut token_data.default_properties, *key, new_pv);
            };
            i = i + 1;
        };
        token_event_store::emit_default_property_mutate_event(creator, token_data_id.collection, token_data_id.name, keys, old_values, new_values);
    }
}
```


<a id="0x3_token_mutate_one_token"></a>

## Function `mutate_one_token`

Mutate the token_properties of one token.


```move
module 0x3::token {
    public fun mutate_one_token(account: &signer, token_owner: address, token_id: token::TokenId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>): token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun mutate_one_token(
        account: &signer,
        token_owner: address,
        token_id: TokenId,
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>,
    ): TokenId acquires Collections, TokenStore {
        let creator = token_id.token_data_id.creator;
        assert!(signer::address_of(account) == creator, error::permission_denied(ENO_MUTATE_CAPABILITY));
        // validate if the properties is mutable
        assert!(exists<Collections>(creator), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &mut borrow_global_mut<Collections>(
            creator
        ).token_data;

        assert!(table::contains(all_token_data, token_id.token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        let token_data = table::borrow_mut(all_token_data, token_id.token_data_id);

        // if default property is mutatable, token property is alwasy mutable
        // we only need to check TOKEN_PROPERTY_MUTABLE when default property is immutable
        if (!token_data.mutability_config.properties) {
            assert!(
                property_map::contains_key(&token_data.default_properties, &string::utf8(TOKEN_PROPERTY_MUTABLE)),
                error::permission_denied(EFIELD_NOT_MUTABLE)
            );

            let token_prop_mutable = property_map::read_bool(&token_data.default_properties, &string::utf8(TOKEN_PROPERTY_MUTABLE));
            assert!(token_prop_mutable, error::permission_denied(EFIELD_NOT_MUTABLE));
        };

        // check if the property_version is 0 to determine if we need to update the property_version
        if (token_id.property_version == 0) {
            let token = withdraw_with_event_internal(token_owner, token_id, 1);
            // give a new property_version for each token
            let cur_property_version = token_data.largest_property_version + 1;
            let new_token_id = create_token_id(token_id.token_data_id, cur_property_version);
            let new_token = Token {
                id: new_token_id,
                amount: 1,
                token_properties: token_data.default_properties,
            };
            direct_deposit(token_owner, new_token);
            update_token_property_internal(token_owner, new_token_id, keys, values, types);
            if (std::features::module_event_migration_enabled()) {
                event::emit(MutateTokenPropertyMap {
                    old_id: token_id,
                    new_id: new_token_id,
                    keys,
                    values,
                    types
                });
            };
            event::emit_event<MutateTokenPropertyMapEvent>(
                &mut borrow_global_mut<TokenStore>(token_owner).mutate_token_property_events,
                MutateTokenPropertyMapEvent {
                    old_id: token_id,
                    new_id: new_token_id,
                    keys,
                    values,
                    types
                },
            );

            token_data.largest_property_version = cur_property_version;
            // burn the orignial property_version 0 token after mutation
            let Token { id: _, amount: _, token_properties: _ } = token;
            new_token_id
        } else {
            // only 1 copy for the token with property verion bigger than 0
            update_token_property_internal(token_owner, token_id, keys, values, types);
            if (std::features::module_event_migration_enabled()) {
                event::emit(MutateTokenPropertyMap {
                    old_id: token_id,
                    new_id: token_id,
                    keys,
                    values,
                    types
                });
            };
            event::emit_event<MutateTokenPropertyMapEvent>(
                &mut borrow_global_mut<TokenStore>(token_owner).mutate_token_property_events,
                MutateTokenPropertyMapEvent {
                    old_id: token_id,
                    new_id: token_id,
                    keys,
                    values,
                    types
                },
            );
            token_id
        }
    }
}
```


<a id="0x3_token_create_royalty"></a>

## Function `create_royalty`



```move
module 0x3::token {
    public fun create_royalty(royalty_points_numerator: u64, royalty_points_denominator: u64, payee_address: address): token::Royalty
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_royalty(royalty_points_numerator: u64, royalty_points_denominator: u64, payee_address: address): Royalty {
        assert!(royalty_points_numerator <= royalty_points_denominator, error::invalid_argument(EINVALID_ROYALTY_NUMERATOR_DENOMINATOR));
        assert!(account::exists_at(payee_address), error::invalid_argument(EROYALTY_PAYEE_ACCOUNT_DOES_NOT_EXIST));
        Royalty {
            royalty_points_numerator,
            royalty_points_denominator,
            payee_address
        }
    }
}
```


<a id="0x3_token_deposit_token"></a>

## Function `deposit_token`

Deposit the token balance into the owner&apos;s account and emit an event.


```move
module 0x3::token {
    public fun deposit_token(account: &signer, token: token::Token)
}
```


##### Implementation


```move
module 0x3::token {
    public fun deposit_token(account: &signer, token: Token) acquires TokenStore {
        let account_addr = signer::address_of(account);
        initialize_token_store(account);
        direct_deposit(account_addr, token)
    }
}
```


<a id="0x3_token_direct_deposit_with_opt_in"></a>

## Function `direct_deposit_with_opt_in`

direct deposit if user opt in direct transfer


```move
module 0x3::token {
    public fun direct_deposit_with_opt_in(account_addr: address, token: token::Token)
}
```


##### Implementation


```move
module 0x3::token {
    public fun direct_deposit_with_opt_in(account_addr: address, token: Token) acquires TokenStore {
        let opt_in_transfer = borrow_global<TokenStore>(account_addr).direct_transfer;
        assert!(opt_in_transfer, error::permission_denied(EUSER_NOT_OPT_IN_DIRECT_TRANSFER));
        direct_deposit(account_addr, token);
    }
}
```


<a id="0x3_token_direct_transfer"></a>

## Function `direct_transfer`



```move
module 0x3::token {
    public fun direct_transfer(sender: &signer, receiver: &signer, token_id: token::TokenId, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun direct_transfer(
        sender: &signer,
        receiver: &signer,
        token_id: TokenId,
        amount: u64,
    ) acquires TokenStore {
        let token = withdraw_token(sender, token_id, amount);
        deposit_token(receiver, token);
    }
}
```


<a id="0x3_token_initialize_token_store"></a>

## Function `initialize_token_store`



```move
module 0x3::token {
    public fun initialize_token_store(account: &signer)
}
```


##### Implementation


```move
module 0x3::token {
    public fun initialize_token_store(account: &signer) {
        if (!exists<TokenStore>(signer::address_of(account))) {
            move_to(
                account,
                TokenStore {
                    tokens: table::new(),
                    direct_transfer: false,
                    deposit_events: account::new_event_handle<DepositEvent>(account),
                    withdraw_events: account::new_event_handle<WithdrawEvent>(account),
                    burn_events: account::new_event_handle<BurnTokenEvent>(account),
                    mutate_token_property_events: account::new_event_handle<MutateTokenPropertyMapEvent>(account),
                },
            );
        }
    }
}
```


<a id="0x3_token_merge"></a>

## Function `merge`



```move
module 0x3::token {
    public fun merge(dst_token: &mut token::Token, source_token: token::Token)
}
```


##### Implementation


```move
module 0x3::token {
    public fun merge(dst_token: &mut Token, source_token: Token) {
        assert!(&dst_token.id == &source_token.id, error::invalid_argument(EINVALID_TOKEN_MERGE));
        dst_token.amount = dst_token.amount + source_token.amount;
        let Token { id: _, amount: _, token_properties: _ } = source_token;
    }
}
```


<a id="0x3_token_split"></a>

## Function `split`



```move
module 0x3::token {
    public fun split(dst_token: &mut token::Token, amount: u64): token::Token
}
```


##### Implementation


```move
module 0x3::token {
    public fun split(dst_token: &mut Token, amount: u64): Token {
        assert!(dst_token.id.property_version == 0, error::invalid_state(ENFT_NOT_SPLITABLE));
        assert!(dst_token.amount > amount, error::invalid_argument(ETOKEN_SPLIT_AMOUNT_LARGER_OR_EQUAL_TO_TOKEN_AMOUNT));
        assert!(amount > 0, error::invalid_argument(ETOKEN_CANNOT_HAVE_ZERO_AMOUNT));
        dst_token.amount = dst_token.amount - amount;
        Token {
            id: dst_token.id,
            amount,
            token_properties: property_map::empty(),
        }
    }
}
```


<a id="0x3_token_token_id"></a>

## Function `token_id`



```move
module 0x3::token {
    public fun token_id(token: &token::Token): &token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun token_id(token: &Token): &TokenId {
        &token.id
    }
}
```


<a id="0x3_token_transfer"></a>

## Function `transfer`

Transfers `amount` of tokens from `from` to `to`.


```move
module 0x3::token {
    public fun transfer(from: &signer, id: token::TokenId, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun transfer(
        from: &signer,
        id: TokenId,
        to: address,
        amount: u64,
    ) acquires TokenStore {
        let opt_in_transfer = borrow_global<TokenStore>(to).direct_transfer;
        assert!(opt_in_transfer, error::permission_denied(EUSER_NOT_OPT_IN_DIRECT_TRANSFER));
        let token = withdraw_token(from, id, amount);
        direct_deposit(to, token);
    }
}
```


<a id="0x3_token_create_withdraw_capability"></a>

## Function `create_withdraw_capability`

Token owner can create this one&#45;time withdraw capability with an expiration time


```move
module 0x3::token {
    public fun create_withdraw_capability(owner: &signer, token_id: token::TokenId, amount: u64, expiration_sec: u64): token::WithdrawCapability
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_withdraw_capability(
        owner: &signer,
        token_id: TokenId,
        amount: u64,
        expiration_sec: u64,
    ): WithdrawCapability {
        WithdrawCapability {
            token_owner: signer::address_of(owner),
            token_id,
            amount,
            expiration_sec,
        }
    }
}
```


<a id="0x3_token_withdraw_with_capability"></a>

## Function `withdraw_with_capability`

Withdraw the token with a capability


```move
module 0x3::token {
    public fun withdraw_with_capability(withdraw_proof: token::WithdrawCapability): token::Token
}
```


##### Implementation


```move
module 0x3::token {
    public fun withdraw_with_capability(
        withdraw_proof: WithdrawCapability,
    ): Token acquires TokenStore {
        // verify the delegation hasn't expired yet
        assert!(timestamp::now_seconds() <= withdraw_proof.expiration_sec, error::invalid_argument(EWITHDRAW_PROOF_EXPIRES));

        withdraw_with_event_internal(
            withdraw_proof.token_owner,
            withdraw_proof.token_id,
            withdraw_proof.amount,
        )
    }
}
```


<a id="0x3_token_partial_withdraw_with_capability"></a>

## Function `partial_withdraw_with_capability`

Withdraw the token with a capability.


```move
module 0x3::token {
    public fun partial_withdraw_with_capability(withdraw_proof: token::WithdrawCapability, withdraw_amount: u64): (token::Token, option::Option<token::WithdrawCapability>)
}
```


##### Implementation


```move
module 0x3::token {
    public fun partial_withdraw_with_capability(
        withdraw_proof: WithdrawCapability,
        withdraw_amount: u64,
    ): (Token, Option<WithdrawCapability>) acquires TokenStore {
        // verify the delegation hasn't expired yet
        assert!(timestamp::now_seconds() <= withdraw_proof.expiration_sec, error::invalid_argument(EWITHDRAW_PROOF_EXPIRES));

        assert!(withdraw_amount <= withdraw_proof.amount, error::invalid_argument(EINSUFFICIENT_WITHDRAW_CAPABILITY_AMOUNT));

        let res: Option<WithdrawCapability> = if (withdraw_amount == withdraw_proof.amount) {
            option::none<WithdrawCapability>()
        } else {
            option::some(
                WithdrawCapability {
                    token_owner: withdraw_proof.token_owner,
                    token_id: withdraw_proof.token_id,
                    amount: withdraw_proof.amount - withdraw_amount,
                    expiration_sec: withdraw_proof.expiration_sec,
                }
            )
        };

        (
            withdraw_with_event_internal(
                withdraw_proof.token_owner,
                withdraw_proof.token_id,
                withdraw_amount,
            ),
            res
        )

    }
}
```


<a id="0x3_token_withdraw_token"></a>

## Function `withdraw_token`



```move
module 0x3::token {
    public fun withdraw_token(account: &signer, id: token::TokenId, amount: u64): token::Token
}
```


##### Implementation


```move
module 0x3::token {
    public fun withdraw_token(
        account: &signer,
        id: TokenId,
        amount: u64,
    ): Token acquires TokenStore {
        let account_addr = signer::address_of(account);
        withdraw_with_event_internal(account_addr, id, amount)
    }
}
```


<a id="0x3_token_create_collection"></a>

## Function `create_collection`

Create a new collection to hold tokens


```move
module 0x3::token {
    public fun create_collection(creator: &signer, name: string::String, description: string::String, uri: string::String, maximum: u64, mutate_setting: vector<bool>)
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_collection(
        creator: &signer,
        name: String,
        description: String,
        uri: String,
        maximum: u64,
        mutate_setting: vector<bool>
    ) acquires Collections {
        assert!(string::length(&name) <= MAX_COLLECTION_NAME_LENGTH, error::invalid_argument(ECOLLECTION_NAME_TOO_LONG));
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::invalid_argument(EURI_TOO_LONG));
        let account_addr = signer::address_of(creator);
        if (!exists<Collections>(account_addr)) {
            move_to(
                creator,
                Collections {
                    collection_data: table::new(),
                    token_data: table::new(),
                    create_collection_events: account::new_event_handle<CreateCollectionEvent>(creator),
                    create_token_data_events: account::new_event_handle<CreateTokenDataEvent>(creator),
                    mint_token_events: account::new_event_handle<MintTokenEvent>(creator),
                },
            )
        };

        let collection_data = &mut borrow_global_mut<Collections>(account_addr).collection_data;

        assert!(
            !table::contains(collection_data, name),
            error::already_exists(ECOLLECTION_ALREADY_EXISTS),
        );

        let mutability_config = create_collection_mutability_config(&mutate_setting);
        let collection = CollectionData {
            description,
            name: name,
            uri,
            supply: 0,
            maximum,
            mutability_config
        };

        table::add(collection_data, name, collection);
        let collection_handle = borrow_global_mut<Collections>(account_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CreateCollection {
                    creator: account_addr,
                    collection_name: name,
                    uri,
                    description,
                    maximum,
                }
            );
        };
        event::emit_event<CreateCollectionEvent>(
            &mut collection_handle.create_collection_events,
            CreateCollectionEvent {
                creator: account_addr,
                collection_name: name,
                uri,
                description,
                maximum,
            }
        );
    }
}
```


<a id="0x3_token_check_collection_exists"></a>

## Function `check_collection_exists`



```move
module 0x3::token {
    public fun check_collection_exists(creator: address, name: string::String): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun check_collection_exists(creator: address, name: String): bool acquires Collections {
        assert!(
            exists<Collections>(creator),
            error::not_found(ECOLLECTIONS_NOT_PUBLISHED),
        );

        let collection_data = &borrow_global<Collections>(creator).collection_data;
        table::contains(collection_data, name)
    }
}
```


<a id="0x3_token_check_tokendata_exists"></a>

## Function `check_tokendata_exists`



```move
module 0x3::token {
    public fun check_tokendata_exists(creator: address, collection_name: string::String, token_name: string::String): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun check_tokendata_exists(creator: address, collection_name: String, token_name: String): bool acquires Collections {
        assert!(
            exists<Collections>(creator),
            error::not_found(ECOLLECTIONS_NOT_PUBLISHED),
        );

        let token_data = &borrow_global<Collections>(creator).token_data;
        let token_data_id = create_token_data_id(creator, collection_name, token_name);
        table::contains(token_data, token_data_id)
    }
}
```


<a id="0x3_token_create_tokendata"></a>

## Function `create_tokendata`



```move
module 0x3::token {
    public fun create_tokendata(account: &signer, collection: string::String, name: string::String, description: string::String, maximum: u64, uri: string::String, royalty_payee_address: address, royalty_points_denominator: u64, royalty_points_numerator: u64, token_mutate_config: token::TokenMutabilityConfig, property_keys: vector<string::String>, property_values: vector<vector<u8>>, property_types: vector<string::String>): token::TokenDataId
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_tokendata(
        account: &signer,
        collection: String,
        name: String,
        description: String,
        maximum: u64,
        uri: String,
        royalty_payee_address: address,
        royalty_points_denominator: u64,
        royalty_points_numerator: u64,
        token_mutate_config: TokenMutabilityConfig,
        property_keys: vector<String>,
        property_values: vector<vector<u8>>,
        property_types: vector<String>
    ): TokenDataId acquires Collections {
        assert!(string::length(&name) <= MAX_NFT_NAME_LENGTH, error::invalid_argument(ENFT_NAME_TOO_LONG));
        assert!(string::length(&collection) <= MAX_COLLECTION_NAME_LENGTH, error::invalid_argument(ECOLLECTION_NAME_TOO_LONG));
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::invalid_argument(EURI_TOO_LONG));
        assert!(royalty_points_numerator <= royalty_points_denominator, error::invalid_argument(EINVALID_ROYALTY_NUMERATOR_DENOMINATOR));

        let account_addr = signer::address_of(account);
        assert!(
            exists<Collections>(account_addr),
            error::not_found(ECOLLECTIONS_NOT_PUBLISHED),
        );
        let collections = borrow_global_mut<Collections>(account_addr);

        let token_data_id = create_token_data_id(account_addr, collection, name);

        assert!(
            table::contains(&collections.collection_data, token_data_id.collection),
            error::not_found(ECOLLECTION_NOT_PUBLISHED),
        );
        assert!(
            !table::contains(&collections.token_data, token_data_id),
            error::already_exists(ETOKEN_DATA_ALREADY_EXISTS),
        );

        let collection = table::borrow_mut(&mut collections.collection_data, token_data_id.collection);

        // if collection maximum == 0, user don't want to enforce supply constraint.
        // we don't track supply to make token creation parallelizable
        if (collection.maximum > 0) {
            collection.supply = collection.supply + 1;
            assert!(
                collection.maximum >= collection.supply,
                error::invalid_argument(ECREATE_WOULD_EXCEED_COLLECTION_MAXIMUM),
            );
        };

        let token_data = TokenData {
            maximum,
            largest_property_version: 0,
            supply: 0,
            uri,
            royalty: create_royalty(royalty_points_numerator, royalty_points_denominator, royalty_payee_address),
            name,
            description,
            default_properties: property_map::new(property_keys, property_values, property_types),
            mutability_config: token_mutate_config,
        };

        table::add(&mut collections.token_data, token_data_id, token_data);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CreateTokenData {
                    id: token_data_id,
                    description,
                    maximum,
                    uri,
                    royalty_payee_address,
                    royalty_points_denominator,
                    royalty_points_numerator,
                    name,
                    mutability_config: token_mutate_config,
                    property_keys,
                    property_values,
                    property_types,
                }
            );
        };

        event::emit_event<CreateTokenDataEvent>(
            &mut collections.create_token_data_events,
            CreateTokenDataEvent {
                id: token_data_id,
                description,
                maximum,
                uri,
                royalty_payee_address,
                royalty_points_denominator,
                royalty_points_numerator,
                name,
                mutability_config: token_mutate_config,
                property_keys,
                property_values,
                property_types,
            },
        );
        token_data_id
    }
}
```


<a id="0x3_token_get_collection_supply"></a>

## Function `get_collection_supply`

return the number of distinct token_data_id created under this collection


```move
module 0x3::token {
    public fun get_collection_supply(creator_address: address, collection_name: string::String): option::Option<u64>
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_supply(creator_address: address, collection_name: String): Option<u64> acquires Collections {
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);

        if (collection_data.maximum > 0) {
            option::some(collection_data.supply)
        } else {
            option::none()
        }
    }
}
```


<a id="0x3_token_get_collection_description"></a>

## Function `get_collection_description`



```move
module 0x3::token {
    public fun get_collection_description(creator_address: address, collection_name: string::String): string::String
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_description(creator_address: address, collection_name: String): String acquires Collections {
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        collection_data.description
    }
}
```


<a id="0x3_token_get_collection_uri"></a>

## Function `get_collection_uri`



```move
module 0x3::token {
    public fun get_collection_uri(creator_address: address, collection_name: string::String): string::String
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_uri(creator_address: address, collection_name: String): String acquires Collections {
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        collection_data.uri
    }
}
```


<a id="0x3_token_get_collection_maximum"></a>

## Function `get_collection_maximum`



```move
module 0x3::token {
    public fun get_collection_maximum(creator_address: address, collection_name: string::String): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_maximum(creator_address: address, collection_name: String): u64 acquires Collections {
        assert_collection_exists(creator_address, collection_name);
        let collection_data = table::borrow_mut(&mut borrow_global_mut<Collections>(creator_address).collection_data, collection_name);
        collection_data.maximum
    }
}
```


<a id="0x3_token_get_token_supply"></a>

## Function `get_token_supply`

return the number of distinct token_id created under this TokenData


```move
module 0x3::token {
    public fun get_token_supply(creator_address: address, token_data_id: token::TokenDataId): option::Option<u64>
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_supply(creator_address: address, token_data_id: TokenDataId): Option<u64> acquires Collections {
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_address).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        let token_data = table::borrow(all_token_data, token_data_id);

        if (token_data.maximum > 0) {
            option::some(token_data.supply)
        } else {
            option::none<u64>()
        }
    }
}
```


<a id="0x3_token_get_tokendata_largest_property_version"></a>

## Function `get_tokendata_largest_property_version`

return the largest_property_version of this TokenData


```move
module 0x3::token {
    public fun get_tokendata_largest_property_version(creator_address: address, token_data_id: token::TokenDataId): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_largest_property_version(creator_address: address, token_data_id: TokenDataId): u64 acquires Collections {
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_address).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        table::borrow(all_token_data, token_data_id).largest_property_version
    }
}
```


<a id="0x3_token_get_token_id"></a>

## Function `get_token_id`

return the TokenId for a given Token


```move
module 0x3::token {
    public fun get_token_id(token: &token::Token): token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_id(token: &Token): TokenId {
        token.id
    }
}
```


<a id="0x3_token_get_direct_transfer"></a>

## Function `get_direct_transfer`



```move
module 0x3::token {
    public fun get_direct_transfer(receiver: address): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_direct_transfer(receiver: address): bool acquires TokenStore {
        if (!exists<TokenStore>(receiver)) {
            return false
        };

        borrow_global<TokenStore>(receiver).direct_transfer
    }
}
```


<a id="0x3_token_create_token_mutability_config"></a>

## Function `create_token_mutability_config`



```move
module 0x3::token {
    public fun create_token_mutability_config(mutate_setting: &vector<bool>): token::TokenMutabilityConfig
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_token_mutability_config(mutate_setting: &vector<bool>): TokenMutabilityConfig {
        TokenMutabilityConfig {
            maximum: *vector::borrow(mutate_setting, TOKEN_MAX_MUTABLE_IND),
            uri: *vector::borrow(mutate_setting, TOKEN_URI_MUTABLE_IND),
            royalty: *vector::borrow(mutate_setting, TOKEN_ROYALTY_MUTABLE_IND),
            description: *vector::borrow(mutate_setting, TOKEN_DESCRIPTION_MUTABLE_IND),
            properties: *vector::borrow(mutate_setting, TOKEN_PROPERTY_MUTABLE_IND),
        }
    }
}
```


<a id="0x3_token_create_collection_mutability_config"></a>

## Function `create_collection_mutability_config`



```move
module 0x3::token {
    public fun create_collection_mutability_config(mutate_setting: &vector<bool>): token::CollectionMutabilityConfig
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_collection_mutability_config(mutate_setting: &vector<bool>): CollectionMutabilityConfig {
        CollectionMutabilityConfig {
            description: *vector::borrow(mutate_setting, COLLECTION_DESCRIPTION_MUTABLE_IND),
            uri: *vector::borrow(mutate_setting, COLLECTION_URI_MUTABLE_IND),
            maximum: *vector::borrow(mutate_setting, COLLECTION_MAX_MUTABLE_IND),
        }
    }
}
```


<a id="0x3_token_mint_token"></a>

## Function `mint_token`



```move
module 0x3::token {
    public fun mint_token(account: &signer, token_data_id: token::TokenDataId, amount: u64): token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun mint_token(
        account: &signer,
        token_data_id: TokenDataId,
        amount: u64,
    ): TokenId acquires Collections, TokenStore {
        assert!(token_data_id.creator == signer::address_of(account), error::permission_denied(ENO_MINT_CAPABILITY));
        let creator_addr = token_data_id.creator;
        let all_token_data = &mut borrow_global_mut<Collections>(creator_addr).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        let token_data = table::borrow_mut(all_token_data, token_data_id);

        if (token_data.maximum > 0) {
            assert!(token_data.supply + amount <= token_data.maximum, error::invalid_argument(EMINT_WOULD_EXCEED_TOKEN_MAXIMUM));
            token_data.supply = token_data.supply + amount;
        };

        // we add more tokens with property_version 0
        let token_id = create_token_id(token_data_id, 0);
        if (std::features::module_event_migration_enabled()) {
            event::emit(MintToken { id: token_data_id, amount })
        };
        event::emit_event<MintTokenEvent>(
            &mut borrow_global_mut<Collections>(creator_addr).mint_token_events,
            MintTokenEvent {
                id: token_data_id,
                amount,
            }
        );

        deposit_token(account,
            Token {
                id: token_id,
                amount,
                token_properties: property_map::empty(), // same as default properties no need to store
            }
        );

        token_id
    }
}
```


<a id="0x3_token_mint_token_to"></a>

## Function `mint_token_to`

create tokens and directly deposite to receiver&apos;s address. The receiver should opt&#45;in direct transfer


```move
module 0x3::token {
    public fun mint_token_to(account: &signer, receiver: address, token_data_id: token::TokenDataId, amount: u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun mint_token_to(
        account: &signer,
        receiver: address,
        token_data_id: TokenDataId,
        amount: u64,
    ) acquires Collections, TokenStore {
        assert!(exists<TokenStore>(receiver), error::not_found(ETOKEN_STORE_NOT_PUBLISHED));
        let opt_in_transfer = borrow_global<TokenStore>(receiver).direct_transfer;
        assert!(opt_in_transfer, error::permission_denied(EUSER_NOT_OPT_IN_DIRECT_TRANSFER));

        assert!(token_data_id.creator == signer::address_of(account), error::permission_denied(ENO_MINT_CAPABILITY));
        let creator_addr = token_data_id.creator;
        let all_token_data = &mut borrow_global_mut<Collections>(creator_addr).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        let token_data = table::borrow_mut(all_token_data, token_data_id);

        if (token_data.maximum > 0) {
            assert!(token_data.supply + amount <= token_data.maximum, error::invalid_argument(EMINT_WOULD_EXCEED_TOKEN_MAXIMUM));
            token_data.supply = token_data.supply + amount;
        };

        // we add more tokens with property_version 0
        let token_id = create_token_id(token_data_id, 0);

        if (std::features::module_event_migration_enabled()) {
            event::emit(MintToken { id: token_data_id, amount })
        };
        event::emit_event<MintTokenEvent>(
            &mut borrow_global_mut<Collections>(creator_addr).mint_token_events,
            MintTokenEvent {
                id: token_data_id,
                amount,
            }
        );

        direct_deposit(receiver,
            Token {
                id: token_id,
                amount,
                token_properties: property_map::empty(), // same as default properties no need to store
            }
        );
    }
}
```


<a id="0x3_token_create_token_id"></a>

## Function `create_token_id`



```move
module 0x3::token {
    public fun create_token_id(token_data_id: token::TokenDataId, property_version: u64): token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_token_id(token_data_id: TokenDataId, property_version: u64): TokenId {
        TokenId {
            token_data_id,
            property_version,
        }
    }
}
```


<a id="0x3_token_create_token_data_id"></a>

## Function `create_token_data_id`



```move
module 0x3::token {
    public fun create_token_data_id(creator: address, collection: string::String, name: string::String): token::TokenDataId
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_token_data_id(
        creator: address,
        collection: String,
        name: String,
    ): TokenDataId {
        assert!(string::length(&collection) <= MAX_COLLECTION_NAME_LENGTH, error::invalid_argument(ECOLLECTION_NAME_TOO_LONG));
        assert!(string::length(&name) <= MAX_NFT_NAME_LENGTH, error::invalid_argument(ENFT_NAME_TOO_LONG));
        TokenDataId { creator, collection, name }
    }
}
```


<a id="0x3_token_create_token_id_raw"></a>

## Function `create_token_id_raw`



```move
module 0x3::token {
    public fun create_token_id_raw(creator: address, collection: string::String, name: string::String, property_version: u64): token::TokenId
}
```


##### Implementation


```move
module 0x3::token {
    public fun create_token_id_raw(
        creator: address,
        collection: String,
        name: String,
        property_version: u64,
    ): TokenId {
        TokenId {
            token_data_id: create_token_data_id(creator, collection, name),
            property_version,
        }
    }
}
```


<a id="0x3_token_balance_of"></a>

## Function `balance_of`



```move
module 0x3::token {
    public fun balance_of(owner: address, id: token::TokenId): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun balance_of(owner: address, id: TokenId): u64 acquires TokenStore {
        if (!exists<TokenStore>(owner)) {
            return 0
        };
        let token_store = borrow_global<TokenStore>(owner);
        if (table::contains(&token_store.tokens, id)) {
            table::borrow(&token_store.tokens, id).amount
        } else {
            0
        }
    }
}
```


<a id="0x3_token_has_token_store"></a>

## Function `has_token_store`



```move
module 0x3::token {
    public fun has_token_store(owner: address): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun has_token_store(owner: address): bool {
        exists<TokenStore>(owner)
    }
}
```


<a id="0x3_token_get_royalty"></a>

## Function `get_royalty`



```move
module 0x3::token {
    public fun get_royalty(token_id: token::TokenId): token::Royalty
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_royalty(token_id: TokenId): Royalty acquires Collections {
        let token_data_id = token_id.token_data_id;
        get_tokendata_royalty(token_data_id)
    }
}
```


<a id="0x3_token_get_royalty_numerator"></a>

## Function `get_royalty_numerator`



```move
module 0x3::token {
    public fun get_royalty_numerator(royalty: &token::Royalty): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_royalty_numerator(royalty: &Royalty): u64 {
        royalty.royalty_points_numerator
    }
}
```


<a id="0x3_token_get_royalty_denominator"></a>

## Function `get_royalty_denominator`



```move
module 0x3::token {
    public fun get_royalty_denominator(royalty: &token::Royalty): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_royalty_denominator(royalty: &Royalty): u64 {
        royalty.royalty_points_denominator
    }
}
```


<a id="0x3_token_get_royalty_payee"></a>

## Function `get_royalty_payee`



```move
module 0x3::token {
    public fun get_royalty_payee(royalty: &token::Royalty): address
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_royalty_payee(royalty: &Royalty): address {
        royalty.payee_address
    }
}
```


<a id="0x3_token_get_token_amount"></a>

## Function `get_token_amount`



```move
module 0x3::token {
    public fun get_token_amount(token: &token::Token): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_amount(token: &Token): u64 {
        token.amount
    }
}
```


<a id="0x3_token_get_token_id_fields"></a>

## Function `get_token_id_fields`

return the creator address, collection name, token name and property_version


```move
module 0x3::token {
    public fun get_token_id_fields(token_id: &token::TokenId): (address, string::String, string::String, u64)
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_id_fields(token_id: &TokenId): (address, String, String, u64) {
        (
            token_id.token_data_id.creator,
            token_id.token_data_id.collection,
            token_id.token_data_id.name,
            token_id.property_version,
        )
    }
}
```


<a id="0x3_token_get_token_data_id_fields"></a>

## Function `get_token_data_id_fields`



```move
module 0x3::token {
    public fun get_token_data_id_fields(token_data_id: &token::TokenDataId): (address, string::String, string::String)
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_data_id_fields(token_data_id: &TokenDataId): (address, String, String) {
        (
            token_data_id.creator,
            token_data_id.collection,
            token_data_id.name,
        )
    }
}
```


<a id="0x3_token_get_property_map"></a>

## Function `get_property_map`

return a copy of the token property map.
if property_version &#61; 0, return the default property map
if property_version &gt; 0, return the property value stored at owner&apos;s token store


```move
module 0x3::token {
    public fun get_property_map(owner: address, token_id: token::TokenId): property_map::PropertyMap
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_property_map(owner: address, token_id: TokenId): PropertyMap acquires Collections, TokenStore {
        assert!(balance_of(owner, token_id) > 0, error::not_found(EINSUFFICIENT_BALANCE));
        // if property_version = 0, return default property map
        if (token_id.property_version == 0) {
            let creator_addr = token_id.token_data_id.creator;
            let all_token_data = &borrow_global<Collections>(creator_addr).token_data;
            assert!(table::contains(all_token_data, token_id.token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
            let token_data = table::borrow(all_token_data, token_id.token_data_id);
            token_data.default_properties
        } else {
            let tokens = &borrow_global<TokenStore>(owner).tokens;
            table::borrow(tokens, token_id).token_properties
        }
    }
}
```


<a id="0x3_token_get_tokendata_maximum"></a>

## Function `get_tokendata_maximum`



```move
module 0x3::token {
    public fun get_tokendata_maximum(token_data_id: token::TokenDataId): u64
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_maximum(token_data_id: TokenDataId): u64 acquires Collections {
        let creator_address = token_data_id.creator;
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_address).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));

        let token_data = table::borrow(all_token_data, token_data_id);
        token_data.maximum
    }
}
```


<a id="0x3_token_get_tokendata_uri"></a>

## Function `get_tokendata_uri`



```move
module 0x3::token {
    public fun get_tokendata_uri(creator: address, token_data_id: token::TokenDataId): string::String
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_uri(creator: address, token_data_id: TokenDataId): String acquires Collections {
        assert!(exists<Collections>(creator), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));

        let token_data = table::borrow(all_token_data, token_data_id);
        token_data.uri
    }
}
```


<a id="0x3_token_get_tokendata_description"></a>

## Function `get_tokendata_description`



```move
module 0x3::token {
    public fun get_tokendata_description(token_data_id: token::TokenDataId): string::String
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_description(token_data_id: TokenDataId): String acquires Collections {
        let creator_address = token_data_id.creator;
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_address).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));

        let token_data = table::borrow(all_token_data, token_data_id);
        token_data.description
    }
}
```


<a id="0x3_token_get_tokendata_royalty"></a>

## Function `get_tokendata_royalty`



```move
module 0x3::token {
    public fun get_tokendata_royalty(token_data_id: token::TokenDataId): token::Royalty
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_royalty(token_data_id: TokenDataId): Royalty acquires Collections {
        let creator_address = token_data_id.creator;
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_address).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));

        let token_data = table::borrow(all_token_data, token_data_id);
        token_data.royalty
    }
}
```


<a id="0x3_token_get_tokendata_id"></a>

## Function `get_tokendata_id`

return the token_data_id from the token_id


```move
module 0x3::token {
    public fun get_tokendata_id(token_id: token::TokenId): token::TokenDataId
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_id(token_id: TokenId): TokenDataId {
        token_id.token_data_id
    }
}
```


<a id="0x3_token_get_tokendata_mutability_config"></a>

## Function `get_tokendata_mutability_config`

return the mutation setting of the token


```move
module 0x3::token {
    public fun get_tokendata_mutability_config(token_data_id: token::TokenDataId): token::TokenMutabilityConfig
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_tokendata_mutability_config(token_data_id: TokenDataId): TokenMutabilityConfig acquires Collections {
        let creator_addr = token_data_id.creator;
        assert!(exists<Collections>(creator_addr), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &borrow_global<Collections>(creator_addr).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
        table::borrow(all_token_data, token_data_id).mutability_config
    }
}
```


<a id="0x3_token_get_token_mutability_maximum"></a>

## Function `get_token_mutability_maximum`

return if the token&apos;s maximum is mutable


```move
module 0x3::token {
    public fun get_token_mutability_maximum(config: &token::TokenMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_mutability_maximum(config: &TokenMutabilityConfig): bool {
        config.maximum
    }
}
```


<a id="0x3_token_get_token_mutability_royalty"></a>

## Function `get_token_mutability_royalty`

return if the token royalty is mutable with a token mutability config


```move
module 0x3::token {
    public fun get_token_mutability_royalty(config: &token::TokenMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_mutability_royalty(config: &TokenMutabilityConfig): bool {
        config.royalty
    }
}
```


<a id="0x3_token_get_token_mutability_uri"></a>

## Function `get_token_mutability_uri`

return if the token uri is mutable with a token mutability config


```move
module 0x3::token {
    public fun get_token_mutability_uri(config: &token::TokenMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_mutability_uri(config: &TokenMutabilityConfig): bool {
        config.uri
    }
}
```


<a id="0x3_token_get_token_mutability_description"></a>

## Function `get_token_mutability_description`

return if the token description is mutable with a token mutability config


```move
module 0x3::token {
    public fun get_token_mutability_description(config: &token::TokenMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_mutability_description(config: &TokenMutabilityConfig): bool {
        config.description
    }
}
```


<a id="0x3_token_get_token_mutability_default_properties"></a>

## Function `get_token_mutability_default_properties`

return if the tokendata&apos;s default properties is mutable with a token mutability config


```move
module 0x3::token {
    public fun get_token_mutability_default_properties(config: &token::TokenMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_token_mutability_default_properties(config: &TokenMutabilityConfig): bool {
        config.properties
    }
}
```


<a id="0x3_token_get_collection_mutability_config"></a>

## Function `get_collection_mutability_config`

return the collection mutation setting


```move
module 0x3::token {
    #[view]
    public fun get_collection_mutability_config(creator: address, collection_name: string::String): token::CollectionMutabilityConfig
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_mutability_config(
        creator: address,
        collection_name: String
    ): CollectionMutabilityConfig acquires Collections {
        assert!(exists<Collections>(creator), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_collection_data = &borrow_global<Collections>(creator).collection_data;
        assert!(table::contains(all_collection_data, collection_name), error::not_found(ECOLLECTION_NOT_PUBLISHED));
        table::borrow(all_collection_data, collection_name).mutability_config
    }
}
```


<a id="0x3_token_get_collection_mutability_description"></a>

## Function `get_collection_mutability_description`

return if the collection description is mutable with a collection mutability config


```move
module 0x3::token {
    public fun get_collection_mutability_description(config: &token::CollectionMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_mutability_description(config: &CollectionMutabilityConfig): bool {
        config.description
    }
}
```


<a id="0x3_token_get_collection_mutability_uri"></a>

## Function `get_collection_mutability_uri`

return if the collection uri is mutable with a collection mutability config


```move
module 0x3::token {
    public fun get_collection_mutability_uri(config: &token::CollectionMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_mutability_uri(config: &CollectionMutabilityConfig): bool {
        config.uri
    }
}
```


<a id="0x3_token_get_collection_mutability_maximum"></a>

## Function `get_collection_mutability_maximum`

return if the collection maximum is mutable with collection mutability config


```move
module 0x3::token {
    public fun get_collection_mutability_maximum(config: &token::CollectionMutabilityConfig): bool
}
```


##### Implementation


```move
module 0x3::token {
    public fun get_collection_mutability_maximum(config: &CollectionMutabilityConfig): bool {
        config.maximum
    }
}
```


<a id="0x3_token_destroy_token_data"></a>

## Function `destroy_token_data`



```move
module 0x3::token {
    fun destroy_token_data(token_data: token::TokenData)
}
```


##### Implementation


```move
module 0x3::token {
    fun destroy_token_data(token_data: TokenData) {
        let TokenData {
            maximum: _,
            largest_property_version: _,
            supply: _,
            uri: _,
            royalty: _,
            name: _,
            description: _,
            default_properties: _,
            mutability_config: _,
        } = token_data;
    }
}
```


<a id="0x3_token_destroy_collection_data"></a>

## Function `destroy_collection_data`



```move
module 0x3::token {
    fun destroy_collection_data(collection_data: token::CollectionData)
}
```


##### Implementation


```move
module 0x3::token {
    fun destroy_collection_data(collection_data: CollectionData) {
        let CollectionData {
            description: _,
            name: _,
            uri: _,
            supply: _,
            maximum: _,
            mutability_config: _,
        } = collection_data;
    }
}
```


<a id="0x3_token_withdraw_with_event_internal"></a>

## Function `withdraw_with_event_internal`



```move
module 0x3::token {
    fun withdraw_with_event_internal(account_addr: address, id: token::TokenId, amount: u64): token::Token
}
```


##### Implementation


```move
module 0x3::token {
    fun withdraw_with_event_internal(
        account_addr: address,
        id: TokenId,
        amount: u64,
    ): Token acquires TokenStore {
        // It does not make sense to withdraw 0 tokens.
        assert!(amount > 0, error::invalid_argument(EWITHDRAW_ZERO));
        // Make sure the account has sufficient tokens to withdraw.
        assert!(balance_of(account_addr, id) >= amount, error::invalid_argument(EINSUFFICIENT_BALANCE));

        assert!(
            exists<TokenStore>(account_addr),
            error::not_found(ETOKEN_STORE_NOT_PUBLISHED),
        );

        let token_store = borrow_global_mut<TokenStore>(account_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(Withdraw { id, amount })
        };
        event::emit_event<WithdrawEvent>(
            &mut token_store.withdraw_events,
            WithdrawEvent { id, amount }
        );
        let tokens = &mut borrow_global_mut<TokenStore>(account_addr).tokens;
        assert!(
            table::contains(tokens, id),
            error::not_found(ENO_TOKEN_IN_TOKEN_STORE),
        );
        // balance > amount and amount > 0 indirectly asserted that balance > 0.
        let balance = &mut table::borrow_mut(tokens, id).amount;
        if (*balance > amount) {
            *balance = *balance - amount;
            Token { id, amount, token_properties: property_map::empty() }
        } else {
            table::remove(tokens, id)
        }
    }
}
```


<a id="0x3_token_update_token_property_internal"></a>

## Function `update_token_property_internal`



```move
module 0x3::token {
    fun update_token_property_internal(token_owner: address, token_id: token::TokenId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```


##### Implementation


```move
module 0x3::token {
    fun update_token_property_internal(
        token_owner: address,
        token_id: TokenId,
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>,
    ) acquires TokenStore {
        let tokens = &mut borrow_global_mut<TokenStore>(token_owner).tokens;
        assert!(table::contains(tokens, token_id), error::not_found(ENO_TOKEN_IN_TOKEN_STORE));

        let value = &mut table::borrow_mut(tokens, token_id).token_properties;
        assert_non_standard_reserved_property(&keys);
        property_map::update_property_map(value, keys, values, types);
    }
}
```


<a id="0x3_token_direct_deposit"></a>

## Function `direct_deposit`

Deposit the token balance into the recipients account and emit an event.


```move
module 0x3::token {
    fun direct_deposit(account_addr: address, token: token::Token)
}
```


##### Implementation


```move
module 0x3::token {
    fun direct_deposit(account_addr: address, token: Token) acquires TokenStore {
        assert!(token.amount > 0, error::invalid_argument(ETOKEN_CANNOT_HAVE_ZERO_AMOUNT));
        let token_store = borrow_global_mut<TokenStore>(account_addr);

        if (std::features::module_event_migration_enabled()) {
            event::emit(Deposit { id: token.id, amount: token.amount });
        };
        event::emit_event<DepositEvent>(
            &mut token_store.deposit_events,
            DepositEvent { id: token.id, amount: token.amount },
        );

        assert!(
            exists<TokenStore>(account_addr),
            error::not_found(ETOKEN_STORE_NOT_PUBLISHED),
        );

        if (!table::contains(&token_store.tokens, token.id)) {
            table::add(&mut token_store.tokens, token.id, token);
        } else {
            let recipient_token = table::borrow_mut(&mut token_store.tokens, token.id);
            merge(recipient_token, token);
        };
    }
}
```


<a id="0x3_token_assert_collection_exists"></a>

## Function `assert_collection_exists`



```move
module 0x3::token {
    fun assert_collection_exists(creator_address: address, collection_name: string::String)
}
```


##### Implementation


```move
module 0x3::token {
    fun assert_collection_exists(creator_address: address, collection_name: String) acquires Collections {
        assert!(exists<Collections>(creator_address), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_collection_data = &borrow_global<Collections>(creator_address).collection_data;
        assert!(table::contains(all_collection_data, collection_name), error::not_found(ECOLLECTION_NOT_PUBLISHED));
    }
}
```


<a id="0x3_token_assert_tokendata_exists"></a>

## Function `assert_tokendata_exists`



```move
module 0x3::token {
    fun assert_tokendata_exists(creator: &signer, token_data_id: token::TokenDataId)
}
```


##### Implementation


```move
module 0x3::token {
    fun assert_tokendata_exists(creator: &signer, token_data_id: TokenDataId) acquires Collections {
        let creator_addr = token_data_id.creator;
        assert!(signer::address_of(creator) == creator_addr, error::permission_denied(ENO_MUTATE_CAPABILITY));
        assert!(exists<Collections>(creator_addr), error::not_found(ECOLLECTIONS_NOT_PUBLISHED));
        let all_token_data = &mut borrow_global_mut<Collections>(creator_addr).token_data;
        assert!(table::contains(all_token_data, token_data_id), error::not_found(ETOKEN_DATA_NOT_PUBLISHED));
    }
}
```


<a id="0x3_token_assert_non_standard_reserved_property"></a>

## Function `assert_non_standard_reserved_property`



```move
module 0x3::token {
    fun assert_non_standard_reserved_property(keys: &vector<string::String>)
}
```


##### Implementation


```move
module 0x3::token {
    fun assert_non_standard_reserved_property(keys: &vector<String>) {
        vector::for_each_ref(keys, |key| {
            let key: &String = key;
            let length = string::length(key);
            if (length >= 6) {
                let prefix = string::sub_string(&*key, 0, 6);
                assert!(prefix != string::utf8(b"TOKEN_"), error::permission_denied(EPROPERTY_RESERVED_BY_STANDARD));
            };
        });
    }
}
```


<a id="0x3_token_initialize_token_script"></a>

## Function `initialize_token_script`



```move
module 0x3::token {
    public entry fun initialize_token_script(_account: &signer)
}
```


##### Implementation


```move
module 0x3::token {
    public entry fun initialize_token_script(_account: &signer) {
        abort 0
    }
}
```


<a id="0x3_token_initialize_token"></a>

## Function `initialize_token`



```move
module 0x3::token {
    public fun initialize_token(_account: &signer, _token_id: token::TokenId)
}
```


##### Implementation


```move
module 0x3::token {
    public fun initialize_token(_account: &signer, _token_id: TokenId) {
        abort 0
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x3::token {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_create_collection_script"></a>

### Function `create_collection_script`


```move
module 0x3::token {
    public entry fun create_collection_script(creator: &signer, name: string::String, description: string::String, uri: string::String, maximum: u64, mutate_setting: vector<bool>)
}
```

The length of the name is up to MAX_COLLECTION_NAME_LENGTH;
The length of the uri is up to MAX_URI_LENGTH;


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    include CreateCollectionAbortsIf;
}
```


<a id="@Specification_1_create_token_script"></a>

### Function `create_token_script`


```move
module 0x3::token {
    public entry fun create_token_script(account: &signer, collection: string::String, name: string::String, description: string::String, balance: u64, maximum: u64, uri: string::String, royalty_payee_address: address, royalty_points_denominator: u64, royalty_points_numerator: u64, mutate_setting: vector<bool>, property_keys: vector<string::String>, property_values: vector<vector<u8>>, property_types: vector<string::String>)
}
```

the length of &apos;mutate_setting&apos; should maore than five.
The creator of the TokenDataId is signer.
The token_data_id should exist in the creator&apos;s collections..
The sum of supply and mint Token is less than maximum.


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let addr = signer::address_of(account);
    let token_data_id = spec_create_tokendata(addr, collection, name);
    let creator_addr = token_data_id.creator;
    let all_token_data = global<Collections>(creator_addr).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    aborts_if token_data_id.creator != addr;
    aborts_if !exists<Collections>(creator_addr);
    aborts_if balance <= 0;
    include CreateTokenMutabilityConfigAbortsIf;
    include CreateTokenMutabilityConfigAbortsIf;
}
```



<a id="0x3_token_spec_create_tokendata"></a>


```move
module 0x3::token {
    fun spec_create_tokendata(
       creator: address,
       collection: String,
       name: String): TokenDataId {
       TokenDataId { creator, collection, name }
    }
}
```


<a id="@Specification_1_mint_script"></a>

### Function `mint_script`


```move
module 0x3::token {
    public entry fun mint_script(account: &signer, token_data_address: address, collection: string::String, name: string::String, amount: u64)
}
```

only creator of the tokendata can mint tokens


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let token_data_id = spec_create_token_data_id(
        token_data_address,
        collection,
        name,
    );
    let addr = signer::address_of(account);
    let creator_addr = token_data_id.creator;
    let all_token_data = global<Collections>(creator_addr).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    aborts_if token_data_id.creator != signer::address_of(account);
    include CreateTokenDataIdAbortsIf{
    creator: token_data_address,
    collection: collection,
    name: name
    };
    include MintTokenAbortsIf {
    token_data_id: token_data_id
    };
}
```


<a id="@Specification_1_mutate_token_properties"></a>

### Function `mutate_token_properties`


```move
module 0x3::token {
    public entry fun mutate_token_properties(account: &signer, token_owner: address, creator: address, collection_name: string::String, token_name: string::String, token_property_version: u64, amount: u64, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```

The signer is creator.


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let addr = signer::address_of(account);
    aborts_if addr != creator;
    include CreateTokenDataIdAbortsIf {
        creator: creator,
        collection: collection_name,
        name: token_name
    };
}
```


<a id="@Specification_1_direct_transfer_script"></a>

### Function `direct_transfer_script`


```move
module 0x3::token {
    public entry fun direct_transfer_script(sender: &signer, receiver: &signer, creators_address: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```



```move
module 0x3::token {
    pragma aborts_if_is_partial;
    include CreateTokenDataIdAbortsIf{
        creator: creators_address,
        collection: collection,
        name: name
    };
}
```


<a id="@Specification_1_opt_in_direct_transfer"></a>

### Function `opt_in_direct_transfer`


```move
module 0x3::token {
    public entry fun opt_in_direct_transfer(account: &signer, opt_in: bool)
}
```



```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let addr = signer::address_of(account);
    let account_addr = global<account::Account>(addr);
    aborts_if !exists<TokenStore>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<TokenStore>(addr) && account_addr.guid_creation_num + 4 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<TokenStore>(addr) && account_addr.guid_creation_num + 4 > MAX_U64;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account_addr.guid_creation_num + 9 > account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account_addr.guid_creation_num + 9 > MAX_U64;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
}
```


<a id="@Specification_1_transfer_with_opt_in"></a>

### Function `transfer_with_opt_in`


```move
module 0x3::token {
    public entry fun transfer_with_opt_in(from: &signer, creator: address, collection_name: string::String, token_name: string::String, token_property_version: u64, to: address, amount: u64)
}
```



```move
module 0x3::token {
    pragma aborts_if_is_partial;
    include CreateTokenDataIdAbortsIf{
        creator: creator,
        collection: collection_name,
        name: token_name
    };
}
```


<a id="@Specification_1_burn_by_creator"></a>

### Function `burn_by_creator`


```move
module 0x3::token {
    public entry fun burn_by_creator(creator: &signer, owner: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```



```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let creator_address = signer::address_of(creator);
    let token_id = spec_create_token_id_raw(creator_address, collection, name, property_version);
    let creator_addr = token_id.token_data_id.creator;
    let collections = borrow_global_mut<Collections>(creator_address);
    let token_data = table::spec_get(
        collections.token_data,
        token_id.token_data_id,
    );
    aborts_if amount <= 0;
    aborts_if !exists<Collections>(creator_addr);
    aborts_if !table::spec_contains(collections.token_data, token_id.token_data_id);
    aborts_if !simple_map::spec_contains_key(token_data.default_properties.map, std::string::spec_utf8(BURNABLE_BY_CREATOR));
}
```


<a id="@Specification_1_burn"></a>

### Function `burn`


```move
module 0x3::token {
    public entry fun burn(owner: &signer, creators_address: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```

The token_data_id should exist in token_data.


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let token_id = spec_create_token_id_raw(creators_address, collection, name, property_version);
    let creator_addr = token_id.token_data_id.creator;
    let collections = borrow_global_mut<Collections>(creator_addr);
    let token_data = table::spec_get(
        collections.token_data,
        token_id.token_data_id,
    );
    include CreateTokenDataIdAbortsIf {
    creator: creators_address
    };
    aborts_if amount <= 0;
    aborts_if !exists<Collections>(creator_addr);
    aborts_if !table::spec_contains(collections.token_data, token_id.token_data_id);
    aborts_if !simple_map::spec_contains_key(token_data.default_properties.map, std::string::spec_utf8(BURNABLE_BY_OWNER));
    aborts_if !string::spec_internal_check_utf8(BURNABLE_BY_OWNER);
}
```



<a id="0x3_token_spec_create_token_id_raw"></a>


```move
module 0x3::token {
    fun spec_create_token_id_raw(
       creator: address,
       collection: String,
       name: String,
       property_version: u64,
    ): TokenId {
       let token_data_id = TokenDataId { creator, collection, name };
       TokenId {
           token_data_id,
           property_version
       }
    }
}
```


<a id="@Specification_1_mutate_collection_description"></a>

### Function `mutate_collection_description`


```move
module 0x3::token {
    public fun mutate_collection_description(creator: &signer, collection_name: string::String, description: string::String)
}
```

The description of Collection is mutable.


```move
module 0x3::token {
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let collection_data = table::spec_get(global<Collections>(addr).collection_data, collection_name);
    include AssertCollectionExistsAbortsIf {
        creator_address: addr,
        collection_name: collection_name
    };
    aborts_if !collection_data.mutability_config.description;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_collection_uri"></a>

### Function `mutate_collection_uri`


```move
module 0x3::token {
    public fun mutate_collection_uri(creator: &signer, collection_name: string::String, uri: string::String)
}
```

The uri of Collection is mutable.


```move
module 0x3::token {
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let collection_data = table::spec_get(global<Collections>(addr).collection_data, collection_name);
    aborts_if len(uri.bytes) > MAX_URI_LENGTH;
    include AssertCollectionExistsAbortsIf {
        creator_address: addr,
        collection_name: collection_name
    };
    aborts_if !collection_data.mutability_config.uri;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_collection_maximum"></a>

### Function `mutate_collection_maximum`


```move
module 0x3::token {
    public fun mutate_collection_maximum(creator: &signer, collection_name: string::String, maximum: u64)
}
```

Cannot change maximum from 0 and cannot change maximum to 0.
The maximum should more than suply.
The maxium of Collection is mutable.


```move
module 0x3::token {
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let collection_data = table::spec_get(global<Collections>(addr).collection_data, collection_name);
    include AssertCollectionExistsAbortsIf {
        creator_address: addr,
        collection_name: collection_name
    };
    aborts_if collection_data.maximum == 0 || maximum == 0;
    aborts_if maximum < collection_data.supply;
    aborts_if !collection_data.mutability_config.maximum;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_tokendata_maximum"></a>

### Function `mutate_tokendata_maximum`


```move
module 0x3::token {
    public fun mutate_tokendata_maximum(creator: &signer, token_data_id: token::TokenDataId, maximum: u64)
}
```

Cannot change maximum from 0 and cannot change maximum to 0.
The maximum should more than suply.
The token maximum is mutable


```move
module 0x3::token {
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let all_token_data = global<Collections>(token_data_id.creator).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    include AssertTokendataExistsAbortsIf;
    aborts_if token_data.maximum == 0 || maximum == 0;
    aborts_if maximum < token_data.supply;
    aborts_if !token_data.mutability_config.maximum;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_tokendata_uri"></a>

### Function `mutate_tokendata_uri`


```move
module 0x3::token {
    public fun mutate_tokendata_uri(creator: &signer, token_data_id: token::TokenDataId, uri: string::String)
}
```

The length of uri should less than MAX_URI_LENGTH.
The  creator of token_data_id should exist in Collections.
The token uri is mutable


```move
module 0x3::token {
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let all_token_data = global<Collections>(token_data_id.creator).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    include AssertTokendataExistsAbortsIf;
    aborts_if len(uri.bytes) > MAX_URI_LENGTH;
    aborts_if !token_data.mutability_config.uri;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_tokendata_royalty"></a>

### Function `mutate_tokendata_royalty`


```move
module 0x3::token {
    public fun mutate_tokendata_royalty(creator: &signer, token_data_id: token::TokenDataId, royalty: token::Royalty)
}
```

The token royalty is mutable


```move
module 0x3::token {
    include AssertTokendataExistsAbortsIf;
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let all_token_data = global<Collections>(token_data_id.creator).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    aborts_if !token_data.mutability_config.royalty;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_tokendata_description"></a>

### Function `mutate_tokendata_description`


```move
module 0x3::token {
    public fun mutate_tokendata_description(creator: &signer, token_data_id: token::TokenDataId, description: string::String)
}
```

The token description is mutable


```move
module 0x3::token {
    include AssertTokendataExistsAbortsIf;
    let addr = signer::address_of(creator);
    let account = global<account::Account>(addr);
    let all_token_data = global<Collections>(token_data_id.creator).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    aborts_if !token_data.mutability_config.description;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && !exists<account::Account>(addr);
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<token_event_store::TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
}
```


<a id="@Specification_1_mutate_tokendata_property"></a>

### Function `mutate_tokendata_property`


```move
module 0x3::token {
    public fun mutate_tokendata_property(creator: &signer, token_data_id: token::TokenDataId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```

The property map is mutable


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let all_token_data = global<Collections>(token_data_id.creator).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    include AssertTokendataExistsAbortsIf;
    aborts_if len(keys) != len(values);
    aborts_if len(keys) != len(types);
    aborts_if !token_data.mutability_config.properties;
}
```


<a id="@Specification_1_mutate_one_token"></a>

### Function `mutate_one_token`


```move
module 0x3::token {
    public fun mutate_one_token(account: &signer, token_owner: address, token_id: token::TokenId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>): token::TokenId
}
```

The signer is creator.
The token_data_id should exist in token_data.
The property map is mutable.


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let creator = token_id.token_data_id.creator;
    let addr = signer::address_of(account);
    let all_token_data = global<Collections>(creator).token_data;
    let token_data = table::spec_get(all_token_data, token_id.token_data_id);
    aborts_if addr != creator;
    aborts_if !exists<Collections>(creator);
    aborts_if !table::spec_contains(all_token_data, token_id.token_data_id);
    aborts_if !token_data.mutability_config.properties && !simple_map::spec_contains_key(token_data.default_properties.map, std::string::spec_utf8(TOKEN_PROPERTY_MUTABLE));
}
```


<a id="@Specification_1_create_royalty"></a>

### Function `create_royalty`


```move
module 0x3::token {
    public fun create_royalty(royalty_points_numerator: u64, royalty_points_denominator: u64, payee_address: address): token::Royalty
}
```



```move
module 0x3::token {
    include CreateRoyaltyAbortsIf;
}
```

The royalty_points_numerator should less than royalty_points_denominator.


<a id="0x3_token_CreateRoyaltyAbortsIf"></a>


```move
module 0x3::token {
    schema CreateRoyaltyAbortsIf {
        royalty_points_numerator: u64;
        royalty_points_denominator: u64;
        payee_address: address;
        aborts_if royalty_points_numerator > royalty_points_denominator;
        aborts_if !exists<account::Account>(payee_address);
    }
}
```


<a id="@Specification_1_deposit_token"></a>

### Function `deposit_token`


```move
module 0x3::token {
    public fun deposit_token(account: &signer, token: token::Token)
}
```



```move
module 0x3::token {
    pragma verify = false;
    pragma aborts_if_is_partial;
    let account_addr = signer::address_of(account);
    include !exists<TokenStore>(account_addr) ==> InitializeTokenStore;
    let token_id = token.id;
    let token_amount = token.amount;
    include DirectDepositAbortsIf;
}
```


<a id="@Specification_1_direct_deposit_with_opt_in"></a>

### Function `direct_deposit_with_opt_in`


```move
module 0x3::token {
    public fun direct_deposit_with_opt_in(account_addr: address, token: token::Token)
}
```

The token can direct_transfer.


```move
module 0x3::token {
    let opt_in_transfer = global<TokenStore>(account_addr).direct_transfer;
    aborts_if !exists<TokenStore>(account_addr);
    aborts_if !opt_in_transfer;
    let token_id = token.id;
    let token_amount = token.amount;
    include DirectDepositAbortsIf;
}
```


<a id="@Specification_1_direct_transfer"></a>

### Function `direct_transfer`


```move
module 0x3::token {
    public fun direct_transfer(sender: &signer, receiver: &signer, token_id: token::TokenId, amount: u64)
}
```

Cannot withdraw 0 tokens.
Make sure the account has sufficient tokens to withdraw.


```move
module 0x3::token {
    pragma verify = false;
}
```


<a id="@Specification_1_initialize_token_store"></a>

### Function `initialize_token_store`


```move
module 0x3::token {
    public fun initialize_token_store(account: &signer)
}
```



```move
module 0x3::token {
    include InitializeTokenStore;
}
```



<a id="0x3_token_InitializeTokenStore"></a>


```move
module 0x3::token {
    schema InitializeTokenStore {
        account: signer;
        let addr = signer::address_of(account);
        let account_addr = global<account::Account>(addr);
        aborts_if !exists<TokenStore>(addr) && !exists<account::Account>(addr);
        aborts_if !exists<TokenStore>(addr) && account_addr.guid_creation_num + 4 >= account::MAX_GUID_CREATION_NUM;
        aborts_if !exists<TokenStore>(addr) && account_addr.guid_creation_num + 4 > MAX_U64;
    }
}
```


<a id="@Specification_1_merge"></a>

### Function `merge`


```move
module 0x3::token {
    public fun merge(dst_token: &mut token::Token, source_token: token::Token)
}
```



```move
module 0x3::token {
    aborts_if dst_token.id != source_token.id;
    aborts_if dst_token.amount + source_token.amount > MAX_U64;
}
```


<a id="@Specification_1_split"></a>

### Function `split`


```move
module 0x3::token {
    public fun split(dst_token: &mut token::Token, amount: u64): token::Token
}
```



```move
module 0x3::token {
    aborts_if dst_token.id.property_version != 0;
    aborts_if dst_token.amount <= amount;
    aborts_if amount <= 0;
}
```


<a id="@Specification_1_transfer"></a>

### Function `transfer`


```move
module 0x3::token {
    public fun transfer(from: &signer, id: token::TokenId, to: address, amount: u64)
}
```



```move
module 0x3::token {
    let opt_in_transfer = global<TokenStore>(to).direct_transfer;
    let account_addr = signer::address_of(from);
    aborts_if !opt_in_transfer;
    pragma aborts_if_is_partial;
    include WithdrawWithEventInternalAbortsIf;
}
```


<a id="@Specification_1_withdraw_with_capability"></a>

### Function `withdraw_with_capability`


```move
module 0x3::token {
    public fun withdraw_with_capability(withdraw_proof: token::WithdrawCapability): token::Token
}
```



```move
module 0x3::token {
    let now_seconds = global<timestamp::CurrentTimeMicroseconds>(@aptos_framework).microseconds;
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if now_seconds / timestamp::MICRO_CONVERSION_FACTOR > withdraw_proof.expiration_sec;
    include WithdrawWithEventInternalAbortsIf{
    account_addr: withdraw_proof.token_owner,
    id: withdraw_proof.token_id,
    amount: withdraw_proof.amount};
}
```


<a id="@Specification_1_partial_withdraw_with_capability"></a>

### Function `partial_withdraw_with_capability`


```move
module 0x3::token {
    public fun partial_withdraw_with_capability(withdraw_proof: token::WithdrawCapability, withdraw_amount: u64): (token::Token, option::Option<token::WithdrawCapability>)
}
```



```move
module 0x3::token {
    let now_seconds = global<timestamp::CurrentTimeMicroseconds>(@aptos_framework).microseconds;
    aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    aborts_if now_seconds / timestamp::MICRO_CONVERSION_FACTOR > withdraw_proof.expiration_sec;
    aborts_if withdraw_amount > withdraw_proof.amount;
    include WithdrawWithEventInternalAbortsIf{
        account_addr: withdraw_proof.token_owner,
        id: withdraw_proof.token_id,
        amount: withdraw_amount
    };
}
```


<a id="@Specification_1_withdraw_token"></a>

### Function `withdraw_token`


```move
module 0x3::token {
    public fun withdraw_token(account: &signer, id: token::TokenId, amount: u64): token::Token
}
```

Cannot withdraw 0 tokens.
Make sure the account has sufficient tokens to withdraw.


```move
module 0x3::token {
    let account_addr = signer::address_of(account);
    include WithdrawWithEventInternalAbortsIf;
}
```


<a id="@Specification_1_create_collection"></a>

### Function `create_collection`


```move
module 0x3::token {
    public fun create_collection(creator: &signer, name: string::String, description: string::String, uri: string::String, maximum: u64, mutate_setting: vector<bool>)
}
```

The length of the name is up to MAX_COLLECTION_NAME_LENGTH;
The length of the uri is up to MAX_URI_LENGTH;
The collection_data should not exist before you create it.


```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let account_addr = signer::address_of(creator);
    aborts_if len(name.bytes) > 128;
    aborts_if len(uri.bytes) > 512;
    include CreateCollectionAbortsIf;
}
```



<a id="0x3_token_CreateCollectionAbortsIf"></a>


```move
module 0x3::token {
    schema CreateCollectionAbortsIf {
        creator: signer;
        name: String;
        description: String;
        uri: String;
        maximum: u64;
        mutate_setting: vector<bool>;
        let addr = signer::address_of(creator);
        let account = global<account::Account>(addr);
        let collection = global<Collections>(addr);
        let b = !exists<Collections>(addr);
        let collection_data = global<Collections>(addr).collection_data;
        aborts_if b && !exists<account::Account>(addr);
        aborts_if len(name.bytes) > MAX_COLLECTION_NAME_LENGTH;
        aborts_if len(uri.bytes) > MAX_URI_LENGTH;
        aborts_if b && account.guid_creation_num + 3 >= account::MAX_GUID_CREATION_NUM;
        aborts_if b && account.guid_creation_num + 3 > MAX_U64;
        include CreateCollectionMutabilityConfigAbortsIf;
    }
}
```


<a id="@Specification_1_check_collection_exists"></a>

### Function `check_collection_exists`


```move
module 0x3::token {
    public fun check_collection_exists(creator: address, name: string::String): bool
}
```



```move
module 0x3::token {
    aborts_if !exists<Collections>(creator);
}
```


<a id="@Specification_1_check_tokendata_exists"></a>

### Function `check_tokendata_exists`


```move
module 0x3::token {
    public fun check_tokendata_exists(creator: address, collection_name: string::String, token_name: string::String): bool
}
```

The length of collection should less than MAX_COLLECTION_NAME_LENGTH
The length of name should less than MAX_NFT_NAME_LENGTH


```move
module 0x3::token {
    aborts_if !exists<Collections>(creator);
    include CreateTokenDataIdAbortsIf {
        creator: creator,
        collection: collection_name,
        name: token_name
    };
}
```


<a id="@Specification_1_create_tokendata"></a>

### Function `create_tokendata`


```move
module 0x3::token {
    public fun create_tokendata(account: &signer, collection: string::String, name: string::String, description: string::String, maximum: u64, uri: string::String, royalty_payee_address: address, royalty_points_denominator: u64, royalty_points_numerator: u64, token_mutate_config: token::TokenMutabilityConfig, property_keys: vector<string::String>, property_values: vector<vector<u8>>, property_types: vector<string::String>): token::TokenDataId
}
```

The length of collection should less than MAX_COLLECTION_NAME_LENGTH
The length of name should less than MAX_NFT_NAME_LENGTH


```move
module 0x3::token {
    pragma verify = false;
    pragma aborts_if_is_partial;
    let account_addr = signer::address_of(account);
    let collections = global<Collections>(account_addr);
    let token_data_id = spec_create_token_data_id(account_addr, collection, name);
    let Collection = table::spec_get(collections.collection_data, token_data_id.collection);
    let length = len(property_keys);
    aborts_if len(name.bytes) > MAX_NFT_NAME_LENGTH;
    aborts_if len(collection.bytes) > MAX_COLLECTION_NAME_LENGTH;
    aborts_if len(uri.bytes) > MAX_URI_LENGTH;
    aborts_if royalty_points_numerator > royalty_points_denominator;
    aborts_if !exists<Collections>(account_addr);
    include CreateTokenDataIdAbortsIf {
        creator: account_addr,
        collection: collection,
        name: name
    };
    aborts_if !table::spec_contains(collections.collection_data, collection);
    aborts_if table::spec_contains(collections.token_data, token_data_id);
    aborts_if Collection.maximum > 0 && Collection.supply + 1 > MAX_U64;
    aborts_if Collection.maximum > 0 && Collection.maximum < Collection.supply + 1;
    include CreateRoyaltyAbortsIf {
        payee_address: royalty_payee_address
    };
    aborts_if length > property_map::MAX_PROPERTY_MAP_SIZE;
    aborts_if length != len(property_values);
    aborts_if length != len(property_types);
}
```



<a id="0x3_token_spec_create_token_data_id"></a>


```move
module 0x3::token {
    fun spec_create_token_data_id(
       creator: address,
       collection: String,
       name: String,
    ): TokenDataId {
       TokenDataId { creator, collection, name }
    }
}
```


<a id="@Specification_1_get_collection_supply"></a>

### Function `get_collection_supply`


```move
module 0x3::token {
    public fun get_collection_supply(creator_address: address, collection_name: string::String): option::Option<u64>
}
```



```move
module 0x3::token {
    include AssertCollectionExistsAbortsIf;
}
```


<a id="@Specification_1_get_collection_description"></a>

### Function `get_collection_description`


```move
module 0x3::token {
    public fun get_collection_description(creator_address: address, collection_name: string::String): string::String
}
```



```move
module 0x3::token {
    include AssertCollectionExistsAbortsIf;
}
```


<a id="@Specification_1_get_collection_uri"></a>

### Function `get_collection_uri`


```move
module 0x3::token {
    public fun get_collection_uri(creator_address: address, collection_name: string::String): string::String
}
```



```move
module 0x3::token {
    include AssertCollectionExistsAbortsIf;
}
```


<a id="@Specification_1_get_collection_maximum"></a>

### Function `get_collection_maximum`


```move
module 0x3::token {
    public fun get_collection_maximum(creator_address: address, collection_name: string::String): u64
}
```



```move
module 0x3::token {
    include AssertCollectionExistsAbortsIf;
}
```


<a id="@Specification_1_get_token_supply"></a>

### Function `get_token_supply`


```move
module 0x3::token {
    public fun get_token_supply(creator_address: address, token_data_id: token::TokenDataId): option::Option<u64>
}
```



```move
module 0x3::token {
    aborts_if !exists<Collections>(creator_address);
    let all_token_data = global<Collections>(creator_address).token_data;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_get_tokendata_largest_property_version"></a>

### Function `get_tokendata_largest_property_version`


```move
module 0x3::token {
    public fun get_tokendata_largest_property_version(creator_address: address, token_data_id: token::TokenDataId): u64
}
```



```move
module 0x3::token {
    aborts_if !exists<Collections>(creator_address);
    let all_token_data = global<Collections>(creator_address).token_data;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_create_token_mutability_config"></a>

### Function `create_token_mutability_config`


```move
module 0x3::token {
    public fun create_token_mutability_config(mutate_setting: &vector<bool>): token::TokenMutabilityConfig
}
```

The length of &apos;mutate_setting&apos; should more than five.
The mutate_setting shuold have a value.


```move
module 0x3::token {
    include CreateTokenMutabilityConfigAbortsIf;
}
```



<a id="0x3_token_CreateTokenMutabilityConfigAbortsIf"></a>


```move
module 0x3::token {
    schema CreateTokenMutabilityConfigAbortsIf {
        mutate_setting: vector<bool>;
        aborts_if len(mutate_setting) < 5;
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[TOKEN_MAX_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[TOKEN_URI_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[TOKEN_ROYALTY_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[TOKEN_DESCRIPTION_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[TOKEN_PROPERTY_MUTABLE_IND]);
    }
}
```


<a id="@Specification_1_create_collection_mutability_config"></a>

### Function `create_collection_mutability_config`


```move
module 0x3::token {
    public fun create_collection_mutability_config(mutate_setting: &vector<bool>): token::CollectionMutabilityConfig
}
```



```move
module 0x3::token {
    include CreateCollectionMutabilityConfigAbortsIf;
}
```



<a id="0x3_token_CreateCollectionMutabilityConfigAbortsIf"></a>


```move
module 0x3::token {
    schema CreateCollectionMutabilityConfigAbortsIf {
        mutate_setting: vector<bool>;
        aborts_if len(mutate_setting) < 3;
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[COLLECTION_DESCRIPTION_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[COLLECTION_URI_MUTABLE_IND]);
        aborts_if !vector::spec_contains(mutate_setting, mutate_setting[COLLECTION_MAX_MUTABLE_IND]);
    }
}
```


<a id="@Specification_1_mint_token"></a>

### Function `mint_token`


```move
module 0x3::token {
    public fun mint_token(account: &signer, token_data_id: token::TokenDataId, amount: u64): token::TokenId
}
```

The creator of the TokenDataId is signer.
The token_data_id should exist in the creator&apos;s collections..
The sum of supply and the amount of mint Token is less than maximum.


```move
module 0x3::token {
    pragma verify = false;
}
```



<a id="0x3_token_MintTokenAbortsIf"></a>


```move
module 0x3::token {
    schema MintTokenAbortsIf {
        account: signer;
        token_data_id: TokenDataId;
        amount: u64;
        let addr = signer::address_of(account);
        let creator_addr = token_data_id.creator;
        let all_token_data = global<Collections>(creator_addr).token_data;
        let token_data = table::spec_get(all_token_data, token_data_id);
        aborts_if token_data_id.creator != addr;
        aborts_if !table::spec_contains(all_token_data, token_data_id);
        aborts_if token_data.maximum > 0 && token_data.supply + amount > token_data.maximum;
        aborts_if !exists<Collections>(creator_addr);
        aborts_if amount <= 0;
        include InitializeTokenStore;
        let token_id = create_token_id(token_data_id, 0);
    }
}
```


<a id="@Specification_1_mint_token_to"></a>

### Function `mint_token_to`


```move
module 0x3::token {
    public fun mint_token_to(account: &signer, receiver: address, token_data_id: token::TokenDataId, amount: u64)
}
```



```move
module 0x3::token {
    let addr = signer::address_of(account);
    let opt_in_transfer = global<TokenStore>(receiver).direct_transfer;
    let creator_addr = token_data_id.creator;
    let all_token_data = global<Collections>(creator_addr).token_data;
    let token_data = table::spec_get(all_token_data, token_data_id);
    aborts_if !exists<TokenStore>(receiver);
    aborts_if !opt_in_transfer;
    aborts_if token_data_id.creator != addr;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
    aborts_if token_data.maximum > 0 && token_data.supply + amount > token_data.maximum;
    aborts_if amount <= 0;
    aborts_if !exists<Collections>(creator_addr);
    let token_id = create_token_id(token_data_id, 0);
    include DirectDepositAbortsIf {
        account_addr: receiver,
        token_id: token_id,
        token_amount: amount,
    };
}
```


<a id="@Specification_1_create_token_data_id"></a>

### Function `create_token_data_id`


```move
module 0x3::token {
    public fun create_token_data_id(creator: address, collection: string::String, name: string::String): token::TokenDataId
}
```

The length of collection should less than MAX_COLLECTION_NAME_LENGTH
The length of name should less than MAX_NFT_NAME_LENGTH


```move
module 0x3::token {
    include CreateTokenDataIdAbortsIf;
}
```



<a id="0x3_token_CreateTokenDataIdAbortsIf"></a>


```move
module 0x3::token {
    schema CreateTokenDataIdAbortsIf {
        creator: address;
        collection: String;
        name: String;
        aborts_if len(collection.bytes) > MAX_COLLECTION_NAME_LENGTH;
        aborts_if len(name.bytes) > MAX_NFT_NAME_LENGTH;
    }
}
```


<a id="@Specification_1_create_token_id_raw"></a>

### Function `create_token_id_raw`


```move
module 0x3::token {
    public fun create_token_id_raw(creator: address, collection: string::String, name: string::String, property_version: u64): token::TokenId
}
```

The length of collection should less than MAX_COLLECTION_NAME_LENGTH
The length of name should less than MAX_NFT_NAME_LENGTH


```move
module 0x3::token {
    include CreateTokenDataIdAbortsIf;
}
```



<a id="0x3_token_spec_balance_of"></a>


```move
module 0x3::token {
    fun spec_balance_of(owner: address, id: TokenId): u64 {
       let token_store = borrow_global<TokenStore>(owner);
       if (!exists<TokenStore>(owner)) {
           0
       }
       else if (table::spec_contains(token_store.tokens, id)) {
           table::spec_get(token_store.tokens, id).amount
       } else {
           0
       }
    }
}
```


<a id="@Specification_1_get_royalty"></a>

### Function `get_royalty`


```move
module 0x3::token {
    public fun get_royalty(token_id: token::TokenId): token::Royalty
}
```



```move
module 0x3::token {
    include GetTokendataRoyaltyAbortsIf {
        token_data_id: token_id.token_data_id
    };
}
```


<a id="@Specification_1_get_property_map"></a>

### Function `get_property_map`


```move
module 0x3::token {
    public fun get_property_map(owner: address, token_id: token::TokenId): property_map::PropertyMap
}
```



```move
module 0x3::token {
    let creator_addr = token_id.token_data_id.creator;
    let all_token_data = global<Collections>(creator_addr).token_data;
    aborts_if spec_balance_of(owner, token_id) <= 0;
    aborts_if token_id.property_version == 0 && !table::spec_contains(all_token_data, token_id.token_data_id);
    aborts_if token_id.property_version == 0 && !exists<Collections>(creator_addr);
}
```


<a id="@Specification_1_get_tokendata_maximum"></a>

### Function `get_tokendata_maximum`


```move
module 0x3::token {
    public fun get_tokendata_maximum(token_data_id: token::TokenDataId): u64
}
```



```move
module 0x3::token {
    let creator_address = token_data_id.creator;
    aborts_if !exists<Collections>(creator_address);
    let all_token_data = global<Collections>(creator_address).token_data;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_get_tokendata_uri"></a>

### Function `get_tokendata_uri`


```move
module 0x3::token {
    public fun get_tokendata_uri(creator: address, token_data_id: token::TokenDataId): string::String
}
```



```move
module 0x3::token {
    aborts_if !exists<Collections>(creator);
    let all_token_data = global<Collections>(creator).token_data;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_get_tokendata_description"></a>

### Function `get_tokendata_description`


```move
module 0x3::token {
    public fun get_tokendata_description(token_data_id: token::TokenDataId): string::String
}
```



```move
module 0x3::token {
    let creator_address = token_data_id.creator;
    aborts_if !exists<Collections>(creator_address);
    let all_token_data = global<Collections>(creator_address).token_data;
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_get_tokendata_royalty"></a>

### Function `get_tokendata_royalty`


```move
module 0x3::token {
    public fun get_tokendata_royalty(token_data_id: token::TokenDataId): token::Royalty
}
```



```move
module 0x3::token {
    include GetTokendataRoyaltyAbortsIf;
}
```



<a id="0x3_token_GetTokendataRoyaltyAbortsIf"></a>


```move
module 0x3::token {
    schema GetTokendataRoyaltyAbortsIf {
        token_data_id: TokenDataId;
        let creator_address = token_data_id.creator;
        let all_token_data = global<Collections>(creator_address).token_data;
        aborts_if !exists<Collections>(creator_address);
        aborts_if !table::spec_contains(all_token_data, token_data_id);
    }
}
```


<a id="@Specification_1_get_tokendata_mutability_config"></a>

### Function `get_tokendata_mutability_config`


```move
module 0x3::token {
    public fun get_tokendata_mutability_config(token_data_id: token::TokenDataId): token::TokenMutabilityConfig
}
```



```move
module 0x3::token {
    let creator_addr = token_data_id.creator;
    let all_token_data = global<Collections>(creator_addr).token_data;
    aborts_if !exists<Collections>(creator_addr);
    aborts_if !table::spec_contains(all_token_data, token_data_id);
}
```


<a id="@Specification_1_get_collection_mutability_config"></a>

### Function `get_collection_mutability_config`


```move
module 0x3::token {
    #[view]
    public fun get_collection_mutability_config(creator: address, collection_name: string::String): token::CollectionMutabilityConfig
}
```



```move
module 0x3::token {
    let all_collection_data = global<Collections>(creator).collection_data;
    aborts_if !exists<Collections>(creator);
    aborts_if !table::spec_contains(all_collection_data, collection_name);
}
```


<a id="@Specification_1_withdraw_with_event_internal"></a>

### Function `withdraw_with_event_internal`


```move
module 0x3::token {
    fun withdraw_with_event_internal(account_addr: address, id: token::TokenId, amount: u64): token::Token
}
```



```move
module 0x3::token {
    include WithdrawWithEventInternalAbortsIf;
}
```



<a id="0x3_token_WithdrawWithEventInternalAbortsIf"></a>


```move
module 0x3::token {
    schema WithdrawWithEventInternalAbortsIf {
        account_addr: address;
        id: TokenId;
        amount: u64;
        let tokens = global<TokenStore>(account_addr).tokens;
        aborts_if amount <= 0;
        aborts_if spec_balance_of(account_addr, id) < amount;
        aborts_if !exists<TokenStore>(account_addr);
        aborts_if !table::spec_contains(tokens, id);
    }
}
```


<a id="@Specification_1_update_token_property_internal"></a>

### Function `update_token_property_internal`


```move
module 0x3::token {
    fun update_token_property_internal(token_owner: address, token_id: token::TokenId, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```



```move
module 0x3::token {
    pragma aborts_if_is_partial;
    let tokens = global<TokenStore>(token_owner).tokens;
    aborts_if !exists<TokenStore>(token_owner);
    aborts_if !table::spec_contains(tokens, token_id);
}
```


<a id="@Specification_1_direct_deposit"></a>

### Function `direct_deposit`


```move
module 0x3::token {
    fun direct_deposit(account_addr: address, token: token::Token)
}
```



```move
module 0x3::token {
    let token_id = token.id;
    let token_amount = token.amount;
    include DirectDepositAbortsIf;
}
```



<a id="0x3_token_DirectDepositAbortsIf"></a>


```move
module 0x3::token {
    schema DirectDepositAbortsIf {
        account_addr: address;
        token_id: TokenId;
        token_amount: u64;
        let token_store = global<TokenStore>(account_addr);
        let recipient_token = table::spec_get(token_store.tokens, token_id);
        let b = table::spec_contains(token_store.tokens, token_id);
        aborts_if token_amount <= 0;
        aborts_if !exists<TokenStore>(account_addr);
        aborts_if b && recipient_token.id != token_id;
        aborts_if b && recipient_token.amount + token_amount > MAX_U64;
    }
}
```


<a id="@Specification_1_assert_collection_exists"></a>

### Function `assert_collection_exists`


```move
module 0x3::token {
    fun assert_collection_exists(creator_address: address, collection_name: string::String)
}
```

The collection_name should exist in collection_data of the creator_address&apos;s Collections.


```move
module 0x3::token {
    include AssertCollectionExistsAbortsIf;
}
```



<a id="0x3_token_AssertCollectionExistsAbortsIf"></a>


```move
module 0x3::token {
    schema AssertCollectionExistsAbortsIf {
        creator_address: address;
        collection_name: String;
        let all_collection_data = global<Collections>(creator_address).collection_data;
        aborts_if !exists<Collections>(creator_address);
        aborts_if !table::spec_contains(all_collection_data, collection_name);
    }
}
```


<a id="@Specification_1_assert_tokendata_exists"></a>

### Function `assert_tokendata_exists`


```move
module 0x3::token {
    fun assert_tokendata_exists(creator: &signer, token_data_id: token::TokenDataId)
}
```

The creator of token_data_id should be signer.
The  creator of token_data_id exists in Collections.
The token_data_id is in the all_token_data.


```move
module 0x3::token {
    include AssertTokendataExistsAbortsIf;
}
```



<a id="0x3_token_AssertTokendataExistsAbortsIf"></a>


```move
module 0x3::token {
    schema AssertTokendataExistsAbortsIf {
        creator: signer;
        token_data_id: TokenDataId;
        let creator_addr = token_data_id.creator;
        let addr = signer::address_of(creator);
        aborts_if addr != creator_addr;
        aborts_if !exists<Collections>(creator_addr);
        let all_token_data = global<Collections>(creator_addr).token_data;
        aborts_if !table::spec_contains(all_token_data, token_data_id);
    }
}
```


<a id="@Specification_1_assert_non_standard_reserved_property"></a>

### Function `assert_non_standard_reserved_property`


```move
module 0x3::token {
    fun assert_non_standard_reserved_property(keys: &vector<string::String>)
}
```



```move
module 0x3::token {
    pragma verify = false;
}
```


<a id="@Specification_1_initialize_token_script"></a>

### Function `initialize_token_script`


```move
module 0x3::token {
    public entry fun initialize_token_script(_account: &signer)
}
```

Deprecated function


```move
module 0x3::token {
    pragma verify = false;
}
```


<a id="@Specification_1_initialize_token"></a>

### Function `initialize_token`


```move
module 0x3::token {
    public fun initialize_token(_account: &signer, _token_id: token::TokenId)
}
```

Deprecated function


```move
module 0x3::token {
    pragma verify = false;
}
```
