
<a id="0x4_aptos_token"></a>

# Module `0x4::aptos_token`

This defines a minimally viable token for no&#45;code solutions akin to the original token at
0x3::token module.
The key features are:
&#42; Base token and collection features
&#42; Creator definable mutability for tokens
&#42; Creator&#45;based freezing of tokens
&#42; Standard object&#45;based transfer and events
&#42; Metadata property type


-  [Resource `AptosCollection`](#0x4_aptos_token_AptosCollection)
-  [Resource `AptosToken`](#0x4_aptos_token_AptosToken)
-  [Constants](#@Constants_0)
-  [Function `create_collection`](#0x4_aptos_token_create_collection)
-  [Function `create_collection_object`](#0x4_aptos_token_create_collection_object)
-  [Function `mint`](#0x4_aptos_token_mint)
-  [Function `mint_token_object`](#0x4_aptos_token_mint_token_object)
-  [Function `mint_soul_bound`](#0x4_aptos_token_mint_soul_bound)
-  [Function `mint_soul_bound_token_object`](#0x4_aptos_token_mint_soul_bound_token_object)
-  [Function `mint_internal`](#0x4_aptos_token_mint_internal)
-  [Function `borrow`](#0x4_aptos_token_borrow)
-  [Function `are_properties_mutable`](#0x4_aptos_token_are_properties_mutable)
-  [Function `is_burnable`](#0x4_aptos_token_is_burnable)
-  [Function `is_freezable_by_creator`](#0x4_aptos_token_is_freezable_by_creator)
-  [Function `is_mutable_description`](#0x4_aptos_token_is_mutable_description)
-  [Function `is_mutable_name`](#0x4_aptos_token_is_mutable_name)
-  [Function `is_mutable_uri`](#0x4_aptos_token_is_mutable_uri)
-  [Function `authorized_borrow`](#0x4_aptos_token_authorized_borrow)
-  [Function `burn`](#0x4_aptos_token_burn)
-  [Function `freeze_transfer`](#0x4_aptos_token_freeze_transfer)
-  [Function `unfreeze_transfer`](#0x4_aptos_token_unfreeze_transfer)
-  [Function `set_description`](#0x4_aptos_token_set_description)
-  [Function `set_name`](#0x4_aptos_token_set_name)
-  [Function `set_uri`](#0x4_aptos_token_set_uri)
-  [Function `add_property`](#0x4_aptos_token_add_property)
-  [Function `add_typed_property`](#0x4_aptos_token_add_typed_property)
-  [Function `remove_property`](#0x4_aptos_token_remove_property)
-  [Function `update_property`](#0x4_aptos_token_update_property)
-  [Function `update_typed_property`](#0x4_aptos_token_update_typed_property)
-  [Function `collection_object`](#0x4_aptos_token_collection_object)
-  [Function `borrow_collection`](#0x4_aptos_token_borrow_collection)
-  [Function `is_mutable_collection_description`](#0x4_aptos_token_is_mutable_collection_description)
-  [Function `is_mutable_collection_royalty`](#0x4_aptos_token_is_mutable_collection_royalty)
-  [Function `is_mutable_collection_uri`](#0x4_aptos_token_is_mutable_collection_uri)
-  [Function `is_mutable_collection_token_description`](#0x4_aptos_token_is_mutable_collection_token_description)
-  [Function `is_mutable_collection_token_name`](#0x4_aptos_token_is_mutable_collection_token_name)
-  [Function `is_mutable_collection_token_uri`](#0x4_aptos_token_is_mutable_collection_token_uri)
-  [Function `is_mutable_collection_token_properties`](#0x4_aptos_token_is_mutable_collection_token_properties)
-  [Function `are_collection_tokens_burnable`](#0x4_aptos_token_are_collection_tokens_burnable)
-  [Function `are_collection_tokens_freezable`](#0x4_aptos_token_are_collection_tokens_freezable)
-  [Function `authorized_borrow_collection`](#0x4_aptos_token_authorized_borrow_collection)
-  [Function `set_collection_description`](#0x4_aptos_token_set_collection_description)
-  [Function `set_collection_royalties`](#0x4_aptos_token_set_collection_royalties)
-  [Function `set_collection_royalties_call`](#0x4_aptos_token_set_collection_royalties_call)
-  [Function `set_collection_uri`](#0x4_aptos_token_set_collection_uri)


```move
module 0x4::aptos_token {
    use 0x1::error;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x4::collection;
    use 0x4::property_map;
    use 0x4::royalty;
    use 0x4::token;
}
```


<a id="0x4_aptos_token_AptosCollection"></a>

## Resource `AptosCollection`

Storage state for managing the no&#45;code Collection.


```move
module 0x4::aptos_token {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct AptosCollection has key
}
```


##### Fields


<dl>
<dt>
`mutator_ref: option::Option<collection::MutatorRef>`
</dt>
<dd>
 Used to mutate collection fields
</dd>
<dt>
`royalty_mutator_ref: option::Option<royalty::MutatorRef>`
</dt>
<dd>
 Used to mutate royalties
</dd>
<dt>
`mutable_description: bool`
</dt>
<dd>
 Determines if the creator can mutate the collection&apos;s description
</dd>
<dt>
`mutable_uri: bool`
</dt>
<dd>
 Determines if the creator can mutate the collection&apos;s uri
</dd>
<dt>
`mutable_token_description: bool`
</dt>
<dd>
 Determines if the creator can mutate token descriptions
</dd>
<dt>
`mutable_token_name: bool`
</dt>
<dd>
 Determines if the creator can mutate token names
</dd>
<dt>
`mutable_token_properties: bool`
</dt>
<dd>
 Determines if the creator can mutate token properties
</dd>
<dt>
`mutable_token_uri: bool`
</dt>
<dd>
 Determines if the creator can mutate token uris
</dd>
<dt>
`tokens_burnable_by_creator: bool`
</dt>
<dd>
 Determines if the creator can burn tokens
</dd>
<dt>
`tokens_freezable_by_creator: bool`
</dt>
<dd>
 Determines if the creator can freeze tokens
</dd>
</dl>


<a id="0x4_aptos_token_AptosToken"></a>

## Resource `AptosToken`

Storage state for managing the no&#45;code Token.


```move
module 0x4::aptos_token {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct AptosToken has key
}
```


##### Fields


<dl>
<dt>
`burn_ref: option::Option<token::BurnRef>`
</dt>
<dd>
 Used to burn.
</dd>
<dt>
`transfer_ref: option::Option<object::TransferRef>`
</dt>
<dd>
 Used to control freeze.
</dd>
<dt>
`mutator_ref: option::Option<token::MutatorRef>`
</dt>
<dd>
 Used to mutate fields
</dd>
<dt>
`property_mutator_ref: property_map::MutatorRef`
</dt>
<dd>
 Used to mutate properties
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x4_aptos_token_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


```move
module 0x4::aptos_token {
    const ECOLLECTION_DOES_NOT_EXIST: u64 = 1;
}
```


<a id="0x4_aptos_token_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


```move
module 0x4::aptos_token {
    const EFIELD_NOT_MUTABLE: u64 = 4;
}
```


<a id="0x4_aptos_token_ENOT_CREATOR"></a>

The provided signer is not the creator


```move
module 0x4::aptos_token {
    const ENOT_CREATOR: u64 = 3;
}
```


<a id="0x4_aptos_token_ETOKEN_DOES_NOT_EXIST"></a>

The token does not exist


```move
module 0x4::aptos_token {
    const ETOKEN_DOES_NOT_EXIST: u64 = 2;
}
```


<a id="0x4_aptos_token_EPROPERTIES_NOT_MUTABLE"></a>

The property map being mutated is not mutable


```move
module 0x4::aptos_token {
    const EPROPERTIES_NOT_MUTABLE: u64 = 6;
}
```


<a id="0x4_aptos_token_ETOKEN_NOT_BURNABLE"></a>

The token being burned is not burnable


```move
module 0x4::aptos_token {
    const ETOKEN_NOT_BURNABLE: u64 = 5;
}
```


<a id="0x4_aptos_token_create_collection"></a>

## Function `create_collection`

Create a new collection


```move
module 0x4::aptos_token {
    public entry fun create_collection(creator: &signer, description: string::String, max_supply: u64, name: string::String, uri: string::String, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_token_description: bool, mutable_token_name: bool, mutable_token_properties: bool, mutable_token_uri: bool, tokens_burnable_by_creator: bool, tokens_freezable_by_creator: bool, royalty_numerator: u64, royalty_denominator: u64)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun create_collection(
        creator: &signer,
        description: String,
        max_supply: u64,
        name: String,
        uri: String,
        mutable_description: bool,
        mutable_royalty: bool,
        mutable_uri: bool,
        mutable_token_description: bool,
        mutable_token_name: bool,
        mutable_token_properties: bool,
        mutable_token_uri: bool,
        tokens_burnable_by_creator: bool,
        tokens_freezable_by_creator: bool,
        royalty_numerator: u64,
        royalty_denominator: u64,
    ) {
        create_collection_object(
            creator,
            description,
            max_supply,
            name,
            uri,
            mutable_description,
            mutable_royalty,
            mutable_uri,
            mutable_token_description,
            mutable_token_name,
            mutable_token_properties,
            mutable_token_uri,
            tokens_burnable_by_creator,
            tokens_freezable_by_creator,
            royalty_numerator,
            royalty_denominator
        );
    }
}
```


<a id="0x4_aptos_token_create_collection_object"></a>

## Function `create_collection_object`



```move
module 0x4::aptos_token {
    public fun create_collection_object(creator: &signer, description: string::String, max_supply: u64, name: string::String, uri: string::String, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_token_description: bool, mutable_token_name: bool, mutable_token_properties: bool, mutable_token_uri: bool, tokens_burnable_by_creator: bool, tokens_freezable_by_creator: bool, royalty_numerator: u64, royalty_denominator: u64): object::Object<aptos_token::AptosCollection>
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun create_collection_object(
        creator: &signer,
        description: String,
        max_supply: u64,
        name: String,
        uri: String,
        mutable_description: bool,
        mutable_royalty: bool,
        mutable_uri: bool,
        mutable_token_description: bool,
        mutable_token_name: bool,
        mutable_token_properties: bool,
        mutable_token_uri: bool,
        tokens_burnable_by_creator: bool,
        tokens_freezable_by_creator: bool,
        royalty_numerator: u64,
        royalty_denominator: u64,
    ): Object<AptosCollection> {
        let creator_addr = signer::address_of(creator);
        let royalty = royalty::create(royalty_numerator, royalty_denominator, creator_addr);
        let constructor_ref = collection::create_fixed_collection(
            creator,
            description,
            max_supply,
            name,
            option::some(royalty),
            uri,
        );

        let object_signer = object::generate_signer(&constructor_ref);
        let mutator_ref = if (mutable_description || mutable_uri) {
            option::some(collection::generate_mutator_ref(&constructor_ref))
        } else {
            option::none()
        };

        let royalty_mutator_ref = if (mutable_royalty) {
            option::some(royalty::generate_mutator_ref(object::generate_extend_ref(&constructor_ref)))
        } else {
            option::none()
        };

        let aptos_collection = AptosCollection {
            mutator_ref,
            royalty_mutator_ref,
            mutable_description,
            mutable_uri,
            mutable_token_description,
            mutable_token_name,
            mutable_token_properties,
            mutable_token_uri,
            tokens_burnable_by_creator,
            tokens_freezable_by_creator,
        };
        move_to(&object_signer, aptos_collection);
        object::object_from_constructor_ref(&constructor_ref)
    }
}
```


<a id="0x4_aptos_token_mint"></a>

## Function `mint`

With an existing collection, directly mint a viable token into the creators account.


```move
module 0x4::aptos_token {
    public entry fun mint(creator: &signer, collection: string::String, description: string::String, name: string::String, uri: string::String, property_keys: vector<string::String>, property_types: vector<string::String>, property_values: vector<vector<u8>>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun mint(
        creator: &signer,
        collection: String,
        description: String,
        name: String,
        uri: String,
        property_keys: vector<String>,
        property_types: vector<String>,
        property_values: vector<vector<u8>>,
    ) acquires AptosCollection, AptosToken {
        mint_token_object(creator, collection, description, name, uri, property_keys, property_types, property_values);
    }
}
```


<a id="0x4_aptos_token_mint_token_object"></a>

## Function `mint_token_object`

Mint a token into an existing collection, and retrieve the object / address of the token.


```move
module 0x4::aptos_token {
    public fun mint_token_object(creator: &signer, collection: string::String, description: string::String, name: string::String, uri: string::String, property_keys: vector<string::String>, property_types: vector<string::String>, property_values: vector<vector<u8>>): object::Object<aptos_token::AptosToken>
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun mint_token_object(
        creator: &signer,
        collection: String,
        description: String,
        name: String,
        uri: String,
        property_keys: vector<String>,
        property_types: vector<String>,
        property_values: vector<vector<u8>>,
    ): Object<AptosToken> acquires AptosCollection, AptosToken {
        let constructor_ref = mint_internal(
            creator,
            collection,
            description,
            name,
            uri,
            property_keys,
            property_types,
            property_values,
        );

        let collection = collection_object(creator, &collection);

        // If tokens are freezable, add a transfer ref to be able to freeze transfers
        let freezable_by_creator = are_collection_tokens_freezable(collection);
        if (freezable_by_creator) {
            let aptos_token_addr = object::address_from_constructor_ref(&constructor_ref);
            let aptos_token = borrow_global_mut<AptosToken>(aptos_token_addr);
            let transfer_ref = object::generate_transfer_ref(&constructor_ref);
            option::fill(&mut aptos_token.transfer_ref, transfer_ref);
        };

        object::object_from_constructor_ref(&constructor_ref)
    }
}
```


<a id="0x4_aptos_token_mint_soul_bound"></a>

## Function `mint_soul_bound`

With an existing collection, directly mint a soul bound token into the recipient&apos;s account.


```move
module 0x4::aptos_token {
    public entry fun mint_soul_bound(creator: &signer, collection: string::String, description: string::String, name: string::String, uri: string::String, property_keys: vector<string::String>, property_types: vector<string::String>, property_values: vector<vector<u8>>, soul_bound_to: address)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun mint_soul_bound(
        creator: &signer,
        collection: String,
        description: String,
        name: String,
        uri: String,
        property_keys: vector<String>,
        property_types: vector<String>,
        property_values: vector<vector<u8>>,
        soul_bound_to: address,
    ) acquires AptosCollection {
        mint_soul_bound_token_object(
            creator,
            collection,
            description,
            name,
            uri,
            property_keys,
            property_types,
            property_values,
            soul_bound_to
        );
    }
}
```


<a id="0x4_aptos_token_mint_soul_bound_token_object"></a>

## Function `mint_soul_bound_token_object`

With an existing collection, directly mint a soul bound token into the recipient&apos;s account.


```move
module 0x4::aptos_token {
    public fun mint_soul_bound_token_object(creator: &signer, collection: string::String, description: string::String, name: string::String, uri: string::String, property_keys: vector<string::String>, property_types: vector<string::String>, property_values: vector<vector<u8>>, soul_bound_to: address): object::Object<aptos_token::AptosToken>
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun mint_soul_bound_token_object(
        creator: &signer,
        collection: String,
        description: String,
        name: String,
        uri: String,
        property_keys: vector<String>,
        property_types: vector<String>,
        property_values: vector<vector<u8>>,
        soul_bound_to: address,
    ): Object<AptosToken> acquires AptosCollection {
        let constructor_ref = mint_internal(
            creator,
            collection,
            description,
            name,
            uri,
            property_keys,
            property_types,
            property_values,
        );

        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        let linear_transfer_ref = object::generate_linear_transfer_ref(&transfer_ref);
        object::transfer_with_ref(linear_transfer_ref, soul_bound_to);
        object::disable_ungated_transfer(&transfer_ref);

        object::object_from_constructor_ref(&constructor_ref)
    }
}
```


<a id="0x4_aptos_token_mint_internal"></a>

## Function `mint_internal`



```move
module 0x4::aptos_token {
    fun mint_internal(creator: &signer, collection: string::String, description: string::String, name: string::String, uri: string::String, property_keys: vector<string::String>, property_types: vector<string::String>, property_values: vector<vector<u8>>): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::aptos_token {
    fun mint_internal(
        creator: &signer,
        collection: String,
        description: String,
        name: String,
        uri: String,
        property_keys: vector<String>,
        property_types: vector<String>,
        property_values: vector<vector<u8>>,
    ): ConstructorRef acquires AptosCollection {
        let constructor_ref = token::create(creator, collection, description, name, option::none(), uri);

        let object_signer = object::generate_signer(&constructor_ref);

        let collection_obj = collection_object(creator, &collection);
        let collection = borrow_collection(&collection_obj);

        let mutator_ref = if (
            collection.mutable_token_description
                || collection.mutable_token_name
                || collection.mutable_token_uri
        ) {
            option::some(token::generate_mutator_ref(&constructor_ref))
        } else {
            option::none()
        };

        let burn_ref = if (collection.tokens_burnable_by_creator) {
            option::some(token::generate_burn_ref(&constructor_ref))
        } else {
            option::none()
        };

        let aptos_token = AptosToken {
            burn_ref,
            transfer_ref: option::none(),
            mutator_ref,
            property_mutator_ref: property_map::generate_mutator_ref(&constructor_ref),
        };
        move_to(&object_signer, aptos_token);

        let properties = property_map::prepare_input(property_keys, property_types, property_values);
        property_map::init(&constructor_ref, properties);

        constructor_ref
    }
}
```


<a id="0x4_aptos_token_borrow"></a>

## Function `borrow`



```move
module 0x4::aptos_token {
    fun borrow<T: key>(token: &object::Object<T>): &aptos_token::AptosToken
}
```


##### Implementation


```move
module 0x4::aptos_token {
    inline fun borrow<T: key>(token: &Object<T>): &AptosToken {
        let token_address = object::object_address(token);
        assert!(
            exists<AptosToken>(token_address),
            error::not_found(ETOKEN_DOES_NOT_EXIST),
        );
        borrow_global<AptosToken>(token_address)
    }
}
```


<a id="0x4_aptos_token_are_properties_mutable"></a>

## Function `are_properties_mutable`



```move
module 0x4::aptos_token {
    #[view]
    public fun are_properties_mutable<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun are_properties_mutable<T: key>(token: Object<T>): bool acquires AptosCollection {
        let collection = token::collection_object(token);
        borrow_collection(&collection).mutable_token_properties
    }
}
```


<a id="0x4_aptos_token_is_burnable"></a>

## Function `is_burnable`



```move
module 0x4::aptos_token {
    #[view]
    public fun is_burnable<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_burnable<T: key>(token: Object<T>): bool acquires AptosToken {
        option::is_some(&borrow(&token).burn_ref)
    }
}
```


<a id="0x4_aptos_token_is_freezable_by_creator"></a>

## Function `is_freezable_by_creator`



```move
module 0x4::aptos_token {
    #[view]
    public fun is_freezable_by_creator<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_freezable_by_creator<T: key>(token: Object<T>): bool acquires AptosCollection {
        are_collection_tokens_freezable(token::collection_object(token))
    }
}
```


<a id="0x4_aptos_token_is_mutable_description"></a>

## Function `is_mutable_description`



```move
module 0x4::aptos_token {
    #[view]
    public fun is_mutable_description<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_description<T: key>(token: Object<T>): bool acquires AptosCollection {
        is_mutable_collection_token_description(token::collection_object(token))
    }
}
```


<a id="0x4_aptos_token_is_mutable_name"></a>

## Function `is_mutable_name`



```move
module 0x4::aptos_token {
    #[view]
    public fun is_mutable_name<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_name<T: key>(token: Object<T>): bool acquires AptosCollection {
        is_mutable_collection_token_name(token::collection_object(token))
    }
}
```


<a id="0x4_aptos_token_is_mutable_uri"></a>

## Function `is_mutable_uri`



```move
module 0x4::aptos_token {
    #[view]
    public fun is_mutable_uri<T: key>(token: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_uri<T: key>(token: Object<T>): bool acquires AptosCollection {
        is_mutable_collection_token_uri(token::collection_object(token))
    }
}
```


<a id="0x4_aptos_token_authorized_borrow"></a>

## Function `authorized_borrow`



```move
module 0x4::aptos_token {
    fun authorized_borrow<T: key>(token: &object::Object<T>, creator: &signer): &aptos_token::AptosToken
}
```


##### Implementation


```move
module 0x4::aptos_token {
    inline fun authorized_borrow<T: key>(token: &Object<T>, creator: &signer): &AptosToken {
        let token_address = object::object_address(token);
        assert!(
            exists<AptosToken>(token_address),
            error::not_found(ETOKEN_DOES_NOT_EXIST),
        );

        assert!(
            token::creator(*token) == signer::address_of(creator),
            error::permission_denied(ENOT_CREATOR),
        );
        borrow_global<AptosToken>(token_address)
    }
}
```


<a id="0x4_aptos_token_burn"></a>

## Function `burn`



```move
module 0x4::aptos_token {
    public entry fun burn<T: key>(creator: &signer, token: object::Object<T>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun burn<T: key>(creator: &signer, token: Object<T>) acquires AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            option::is_some(&aptos_token.burn_ref),
            error::permission_denied(ETOKEN_NOT_BURNABLE),
        );
        move aptos_token;
        let aptos_token = move_from<AptosToken>(object::object_address(&token));
        let AptosToken {
            burn_ref,
            transfer_ref: _,
            mutator_ref: _,
            property_mutator_ref,
        } = aptos_token;
        property_map::burn(property_mutator_ref);
        token::burn(option::extract(&mut burn_ref));
    }
}
```


<a id="0x4_aptos_token_freeze_transfer"></a>

## Function `freeze_transfer`



```move
module 0x4::aptos_token {
    public entry fun freeze_transfer<T: key>(creator: &signer, token: object::Object<T>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun freeze_transfer<T: key>(creator: &signer, token: Object<T>) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_collection_tokens_freezable(token::collection_object(token))
                && option::is_some(&aptos_token.transfer_ref),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        object::disable_ungated_transfer(option::borrow(&aptos_token.transfer_ref));
    }
}
```


<a id="0x4_aptos_token_unfreeze_transfer"></a>

## Function `unfreeze_transfer`



```move
module 0x4::aptos_token {
    public entry fun unfreeze_transfer<T: key>(creator: &signer, token: object::Object<T>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun unfreeze_transfer<T: key>(
        creator: &signer,
        token: Object<T>
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_collection_tokens_freezable(token::collection_object(token))
                && option::is_some(&aptos_token.transfer_ref),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        object::enable_ungated_transfer(option::borrow(&aptos_token.transfer_ref));
    }
}
```


<a id="0x4_aptos_token_set_description"></a>

## Function `set_description`



```move
module 0x4::aptos_token {
    public entry fun set_description<T: key>(creator: &signer, token: object::Object<T>, description: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun set_description<T: key>(
        creator: &signer,
        token: Object<T>,
        description: String,
    ) acquires AptosCollection, AptosToken {
        assert!(
            is_mutable_description(token),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        let aptos_token = authorized_borrow(&token, creator);
        token::set_description(option::borrow(&aptos_token.mutator_ref), description);
    }
}
```


<a id="0x4_aptos_token_set_name"></a>

## Function `set_name`



```move
module 0x4::aptos_token {
    public entry fun set_name<T: key>(creator: &signer, token: object::Object<T>, name: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun set_name<T: key>(
        creator: &signer,
        token: Object<T>,
        name: String,
    ) acquires AptosCollection, AptosToken {
        assert!(
            is_mutable_name(token),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        let aptos_token = authorized_borrow(&token, creator);
        token::set_name(option::borrow(&aptos_token.mutator_ref), name);
    }
}
```


<a id="0x4_aptos_token_set_uri"></a>

## Function `set_uri`



```move
module 0x4::aptos_token {
    public entry fun set_uri<T: key>(creator: &signer, token: object::Object<T>, uri: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun set_uri<T: key>(
        creator: &signer,
        token: Object<T>,
        uri: String,
    ) acquires AptosCollection, AptosToken {
        assert!(
            is_mutable_uri(token),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        let aptos_token = authorized_borrow(&token, creator);
        token::set_uri(option::borrow(&aptos_token.mutator_ref), uri);
    }
}
```


<a id="0x4_aptos_token_add_property"></a>

## Function `add_property`



```move
module 0x4::aptos_token {
    public entry fun add_property<T: key>(creator: &signer, token: object::Object<T>, key: string::String, type: string::String, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun add_property<T: key>(
        creator: &signer,
        token: Object<T>,
        key: String,
        type: String,
        value: vector<u8>,
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_properties_mutable(token),
            error::permission_denied(EPROPERTIES_NOT_MUTABLE),
        );

        property_map::add(&aptos_token.property_mutator_ref, key, type, value);
    }
}
```


<a id="0x4_aptos_token_add_typed_property"></a>

## Function `add_typed_property`



```move
module 0x4::aptos_token {
    public entry fun add_typed_property<T: key, V: drop>(creator: &signer, token: object::Object<T>, key: string::String, value: V)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun add_typed_property<T: key, V: drop>(
        creator: &signer,
        token: Object<T>,
        key: String,
        value: V,
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_properties_mutable(token),
            error::permission_denied(EPROPERTIES_NOT_MUTABLE),
        );

        property_map::add_typed(&aptos_token.property_mutator_ref, key, value);
    }
}
```


<a id="0x4_aptos_token_remove_property"></a>

## Function `remove_property`



```move
module 0x4::aptos_token {
    public entry fun remove_property<T: key>(creator: &signer, token: object::Object<T>, key: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun remove_property<T: key>(
        creator: &signer,
        token: Object<T>,
        key: String,
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_properties_mutable(token),
            error::permission_denied(EPROPERTIES_NOT_MUTABLE),
        );

        property_map::remove(&aptos_token.property_mutator_ref, &key);
    }
}
```


<a id="0x4_aptos_token_update_property"></a>

## Function `update_property`



```move
module 0x4::aptos_token {
    public entry fun update_property<T: key>(creator: &signer, token: object::Object<T>, key: string::String, type: string::String, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun update_property<T: key>(
        creator: &signer,
        token: Object<T>,
        key: String,
        type: String,
        value: vector<u8>,
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_properties_mutable(token),
            error::permission_denied(EPROPERTIES_NOT_MUTABLE),
        );

        property_map::update(&aptos_token.property_mutator_ref, &key, type, value);
    }
}
```


<a id="0x4_aptos_token_update_typed_property"></a>

## Function `update_typed_property`



```move
module 0x4::aptos_token {
    public entry fun update_typed_property<T: key, V: drop>(creator: &signer, token: object::Object<T>, key: string::String, value: V)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun update_typed_property<T: key, V: drop>(
        creator: &signer,
        token: Object<T>,
        key: String,
        value: V,
    ) acquires AptosCollection, AptosToken {
        let aptos_token = authorized_borrow(&token, creator);
        assert!(
            are_properties_mutable(token),
            error::permission_denied(EPROPERTIES_NOT_MUTABLE),
        );

        property_map::update_typed(&aptos_token.property_mutator_ref, &key, value);
    }
}
```


<a id="0x4_aptos_token_collection_object"></a>

## Function `collection_object`



```move
module 0x4::aptos_token {
    fun collection_object(creator: &signer, name: &string::String): object::Object<aptos_token::AptosCollection>
}
```


##### Implementation


```move
module 0x4::aptos_token {
    inline fun collection_object(creator: &signer, name: &String): Object<AptosCollection> {
        let collection_addr = collection::create_collection_address(&signer::address_of(creator), name);
        object::address_to_object<AptosCollection>(collection_addr)
    }
}
```


<a id="0x4_aptos_token_borrow_collection"></a>

## Function `borrow_collection`



```move
module 0x4::aptos_token {
    fun borrow_collection<T: key>(token: &object::Object<T>): &aptos_token::AptosCollection
}
```


##### Implementation


```move
module 0x4::aptos_token {
    inline fun borrow_collection<T: key>(token: &Object<T>): &AptosCollection {
        let collection_address = object::object_address(token);
        assert!(
            exists<AptosCollection>(collection_address),
            error::not_found(ECOLLECTION_DOES_NOT_EXIST),
        );
        borrow_global<AptosCollection>(collection_address)
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_description"></a>

## Function `is_mutable_collection_description`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_description<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_description<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_description
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_royalty"></a>

## Function `is_mutable_collection_royalty`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_royalty<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_royalty<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        option::is_some(&borrow_collection(&collection).royalty_mutator_ref)
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_uri"></a>

## Function `is_mutable_collection_uri`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_uri<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_uri<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_uri
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_token_description"></a>

## Function `is_mutable_collection_token_description`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_description<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_description<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_token_description
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_token_name"></a>

## Function `is_mutable_collection_token_name`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_name<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_name<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_token_name
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_token_uri"></a>

## Function `is_mutable_collection_token_uri`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_uri<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_uri<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_token_uri
    }
}
```


<a id="0x4_aptos_token_is_mutable_collection_token_properties"></a>

## Function `is_mutable_collection_token_properties`



```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_properties<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun is_mutable_collection_token_properties<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).mutable_token_properties
    }
}
```


<a id="0x4_aptos_token_are_collection_tokens_burnable"></a>

## Function `are_collection_tokens_burnable`



```move
module 0x4::aptos_token {
    public fun are_collection_tokens_burnable<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun are_collection_tokens_burnable<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).tokens_burnable_by_creator
    }
}
```


<a id="0x4_aptos_token_are_collection_tokens_freezable"></a>

## Function `are_collection_tokens_freezable`



```move
module 0x4::aptos_token {
    public fun are_collection_tokens_freezable<T: key>(collection: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun are_collection_tokens_freezable<T: key>(
        collection: Object<T>,
    ): bool acquires AptosCollection {
        borrow_collection(&collection).tokens_freezable_by_creator
    }
}
```


<a id="0x4_aptos_token_authorized_borrow_collection"></a>

## Function `authorized_borrow_collection`



```move
module 0x4::aptos_token {
    fun authorized_borrow_collection<T: key>(collection: &object::Object<T>, creator: &signer): &aptos_token::AptosCollection
}
```


##### Implementation


```move
module 0x4::aptos_token {
    inline fun authorized_borrow_collection<T: key>(collection: &Object<T>, creator: &signer): &AptosCollection {
        let collection_address = object::object_address(collection);
        assert!(
            exists<AptosCollection>(collection_address),
            error::not_found(ECOLLECTION_DOES_NOT_EXIST),
        );
        assert!(
            collection::creator(*collection) == signer::address_of(creator),
            error::permission_denied(ENOT_CREATOR),
        );
        borrow_global<AptosCollection>(collection_address)
    }
}
```


<a id="0x4_aptos_token_set_collection_description"></a>

## Function `set_collection_description`



```move
module 0x4::aptos_token {
    public entry fun set_collection_description<T: key>(creator: &signer, collection: object::Object<T>, description: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun set_collection_description<T: key>(
        creator: &signer,
        collection: Object<T>,
        description: String,
    ) acquires AptosCollection {
        let aptos_collection = authorized_borrow_collection(&collection, creator);
        assert!(
            aptos_collection.mutable_description,
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        collection::set_description(option::borrow(&aptos_collection.mutator_ref), description);
    }
}
```


<a id="0x4_aptos_token_set_collection_royalties"></a>

## Function `set_collection_royalties`



```move
module 0x4::aptos_token {
    public fun set_collection_royalties<T: key>(creator: &signer, collection: object::Object<T>, royalty: royalty::Royalty)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public fun set_collection_royalties<T: key>(
        creator: &signer,
        collection: Object<T>,
        royalty: royalty::Royalty,
    ) acquires AptosCollection {
        let aptos_collection = authorized_borrow_collection(&collection, creator);
        assert!(
            option::is_some(&aptos_collection.royalty_mutator_ref),
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        royalty::update(option::borrow(&aptos_collection.royalty_mutator_ref), royalty);
    }
}
```


<a id="0x4_aptos_token_set_collection_royalties_call"></a>

## Function `set_collection_royalties_call`



```move
module 0x4::aptos_token {
    entry fun set_collection_royalties_call<T: key>(creator: &signer, collection: object::Object<T>, royalty_numerator: u64, royalty_denominator: u64, payee_address: address)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    entry fun set_collection_royalties_call<T: key>(
        creator: &signer,
        collection: Object<T>,
        royalty_numerator: u64,
        royalty_denominator: u64,
        payee_address: address,
    ) acquires AptosCollection {
        let royalty = royalty::create(royalty_numerator, royalty_denominator, payee_address);
        set_collection_royalties(creator, collection, royalty);
    }
}
```


<a id="0x4_aptos_token_set_collection_uri"></a>

## Function `set_collection_uri`



```move
module 0x4::aptos_token {
    public entry fun set_collection_uri<T: key>(creator: &signer, collection: object::Object<T>, uri: string::String)
}
```


##### Implementation


```move
module 0x4::aptos_token {
    public entry fun set_collection_uri<T: key>(
        creator: &signer,
        collection: Object<T>,
        uri: String,
    ) acquires AptosCollection {
        let aptos_collection = authorized_borrow_collection(&collection, creator);
        assert!(
            aptos_collection.mutable_uri,
            error::permission_denied(EFIELD_NOT_MUTABLE),
        );
        collection::set_uri(option::borrow(&aptos_collection.mutator_ref), uri);
    }
}
```
