
<a id="0x4_token"></a>

# Module `0x4::token`

This defines an object&#45;based Token. The key differentiating features from the Aptos standard
token are:
&#42; Decoupled token ownership from token data.
&#42; Explicit data model for token metadata via adjacent resources
&#42; Extensible framework for tokens


-  [Resource `Token`](#0x4_token_Token)
-  [Resource `TokenIdentifiers`](#0x4_token_TokenIdentifiers)
-  [Resource `ConcurrentTokenIdentifiers`](#0x4_token_ConcurrentTokenIdentifiers)
-  [Struct `BurnRef`](#0x4_token_BurnRef)
-  [Struct `MutatorRef`](#0x4_token_MutatorRef)
-  [Struct `MutationEvent`](#0x4_token_MutationEvent)
-  [Struct `Mutation`](#0x4_token_Mutation)
-  [Constants](#@Constants_0)
-  [Function `create_common`](#0x4_token_create_common)
-  [Function `create_common_with_collection`](#0x4_token_create_common_with_collection)
-  [Function `create_token`](#0x4_token_create_token)
-  [Function `create`](#0x4_token_create)
-  [Function `create_numbered_token_object`](#0x4_token_create_numbered_token_object)
-  [Function `create_numbered_token`](#0x4_token_create_numbered_token)
-  [Function `create_named_token_object`](#0x4_token_create_named_token_object)
-  [Function `create_named_token`](#0x4_token_create_named_token)
-  [Function `create_named_token_from_seed`](#0x4_token_create_named_token_from_seed)
-  [Function `create_from_account`](#0x4_token_create_from_account)
-  [Function `create_token_address`](#0x4_token_create_token_address)
-  [Function `create_token_address_with_seed`](#0x4_token_create_token_address_with_seed)
-  [Function `create_token_seed`](#0x4_token_create_token_seed)
-  [Function `create_token_name_with_seed`](#0x4_token_create_token_name_with_seed)
-  [Function `generate_mutator_ref`](#0x4_token_generate_mutator_ref)
-  [Function `generate_burn_ref`](#0x4_token_generate_burn_ref)
-  [Function `address_from_burn_ref`](#0x4_token_address_from_burn_ref)
-  [Function `borrow`](#0x4_token_borrow)
-  [Function `creator`](#0x4_token_creator)
-  [Function `collection_name`](#0x4_token_collection_name)
-  [Function `collection_object`](#0x4_token_collection_object)
-  [Function `description`](#0x4_token_description)
-  [Function `name`](#0x4_token_name)
-  [Function `uri`](#0x4_token_uri)
-  [Function `royalty`](#0x4_token_royalty)
-  [Function `index`](#0x4_token_index)
-  [Function `borrow_mut`](#0x4_token_borrow_mut)
-  [Function `burn`](#0x4_token_burn)
-  [Function `set_description`](#0x4_token_set_description)
-  [Function `set_name`](#0x4_token_set_name)
-  [Function `set_uri`](#0x4_token_set_uri)


```move
module 0x4::token {
    use 0x1::aggregator_v2;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::vector;
    use 0x4::collection;
    use 0x4::royalty;
}
```


<a id="0x4_token_Token"></a>

## Resource `Token`

Represents the common fields to all tokens.


```move
module 0x4::token {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Token has key
}
```


##### Fields


<dl>
<dt>
`collection: object::Object<collection::Collection>`
</dt>
<dd>
 The collection from which this token resides.
</dd>
<dt>
`index: u64`
</dt>
<dd>
 Deprecated in favor of `index` inside TokenIdentifiers.
 Was populated until concurrent_token_v2_enabled feature flag was enabled.

 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
`description: string::String`
</dt>
<dd>
 A brief description of the token.
</dd>
<dt>
`name: string::String`
</dt>
<dd>
 Deprecated in favor of `name` inside TokenIdentifiers.
 Was populated until concurrent_token_v2_enabled feature flag was enabled.

 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: &quot;Aptos Animal #1234&quot;
</dd>
<dt>
`uri: string::String`
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off&#45;chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
<dt>
`mutation_events: event::EventHandle<token::MutationEvent>`
</dt>
<dd>
 Emitted upon any mutation of the token.
</dd>
</dl>


<a id="0x4_token_TokenIdentifiers"></a>

## Resource `TokenIdentifiers`

Represents first addition to the common fields for all tokens
Started being populated once aggregator_v2_api_enabled was enabled.


```move
module 0x4::token {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct TokenIdentifiers has key
}
```


##### Fields


<dl>
<dt>
`index: aggregator_v2::AggregatorSnapshot<u64>`
</dt>
<dd>
 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
`name: aggregator_v2::DerivedStringSnapshot`
</dt>
<dd>
 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: &quot;Aptos Animal #1234&quot;
</dd>
</dl>


<a id="0x4_token_ConcurrentTokenIdentifiers"></a>

## Resource `ConcurrentTokenIdentifiers`



```move
module 0x4::token {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    #[deprecated]
    struct ConcurrentTokenIdentifiers has key
}
```


##### Fields


<dl>
<dt>
`index: aggregator_v2::AggregatorSnapshot<u64>`
</dt>
<dd>

</dd>
<dt>
`name: aggregator_v2::AggregatorSnapshot<string::String>`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_token_BurnRef"></a>

## Struct `BurnRef`

This enables burning an NFT, if possible, it will also delete the object. Note, the data
in inner and self occupies 32&#45;bytes each, rather than have both, this data structure makes
a small optimization to support either and take a fixed amount of 34&#45;bytes.


```move
module 0x4::token {
    struct BurnRef has drop, store
}
```


##### Fields


<dl>
<dt>
`inner: option::Option<object::DeleteRef>`
</dt>
<dd>

</dd>
<dt>
`self: option::Option<address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_token_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


```move
module 0x4::token {
    struct MutatorRef has drop, store
}
```


##### Fields


<dl>
<dt>
`self: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_token_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


```move
module 0x4::token {
    struct MutationEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`mutated_field_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_value: string::String`
</dt>
<dd>

</dd>
<dt>
`new_value: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_token_Mutation"></a>

## Struct `Mutation`



```move
module 0x4::token {
    #[event]
    struct Mutation has drop, store
}
```


##### Fields


<dl>
<dt>
`token_address: address`
</dt>
<dd>

</dd>
<dt>
`mutated_field_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_value: string::String`
</dt>
<dd>

</dd>
<dt>
`new_value: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x4_token_EURI_TOO_LONG"></a>

The URI is over the maximum length


```move
module 0x4::token {
    const EURI_TOO_LONG: u64 = 5;
}
```


<a id="0x4_token_MAX_URI_LENGTH"></a>



```move
module 0x4::token {
    const MAX_URI_LENGTH: u64 = 512;
}
```


<a id="0x4_token_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


```move
module 0x4::token {
    const EDESCRIPTION_TOO_LONG: u64 = 6;
}
```


<a id="0x4_token_MAX_DESCRIPTION_LENGTH"></a>



```move
module 0x4::token {
    const MAX_DESCRIPTION_LENGTH: u64 = 2048;
}
```


<a id="0x4_token_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


```move
module 0x4::token {
    const EFIELD_NOT_MUTABLE: u64 = 3;
}
```


<a id="0x4_token_ENOT_CREATOR"></a>

The provided signer is not the creator


```move
module 0x4::token {
    const ENOT_CREATOR: u64 = 2;
}
```


<a id="0x4_token_ESEED_TOO_LONG"></a>

The seed is over the maximum length


```move
module 0x4::token {
    const ESEED_TOO_LONG: u64 = 7;
}
```


<a id="0x4_token_ETOKEN_DOES_NOT_EXIST"></a>

The token does not exist


```move
module 0x4::token {
    const ETOKEN_DOES_NOT_EXIST: u64 = 1;
}
```


<a id="0x4_token_ETOKEN_NAME_TOO_LONG"></a>

The token name is over the maximum length


```move
module 0x4::token {
    const ETOKEN_NAME_TOO_LONG: u64 = 4;
}
```


<a id="0x4_token_MAX_TOKEN_NAME_LENGTH"></a>



```move
module 0x4::token {
    const MAX_TOKEN_NAME_LENGTH: u64 = 128;
}
```


<a id="0x4_token_MAX_TOKEN_SEED_LENGTH"></a>



```move
module 0x4::token {
    const MAX_TOKEN_SEED_LENGTH: u64 = 128;
}
```


<a id="0x4_token_create_common"></a>

## Function `create_common`



```move
module 0x4::token {
    fun create_common(creator: &signer, constructor_ref: &object::ConstructorRef, collection_name: string::String, description: string::String, name_prefix: string::String, name_with_index_suffix: option::Option<string::String>, royalty: option::Option<royalty::Royalty>, uri: string::String)
}
```


##### Implementation


```move
module 0x4::token {
    inline fun create_common(
        creator: &signer,
        constructor_ref: &ConstructorRef,
        collection_name: String,
        description: String,
        name_prefix: String,
        // If option::some, numbered token is created - i.e. index is appended to the name.
        // If option::none, name_prefix is the full name of the token.
        name_with_index_suffix: Option<String>,
        royalty: Option<Royalty>,
        uri: String,
    ) {
        let creator_address = signer::address_of(creator);
        let collection_addr = collection::create_collection_address(&creator_address, &collection_name);
        let collection = object::address_to_object<Collection>(collection_addr);

        create_common_with_collection(
            creator,
            constructor_ref,
            collection,
            description,
            name_prefix,
            name_with_index_suffix,
            royalty,
            uri
        )
    }
}
```


<a id="0x4_token_create_common_with_collection"></a>

## Function `create_common_with_collection`



```move
module 0x4::token {
    fun create_common_with_collection(creator: &signer, constructor_ref: &object::ConstructorRef, collection: object::Object<collection::Collection>, description: string::String, name_prefix: string::String, name_with_index_suffix: option::Option<string::String>, royalty: option::Option<royalty::Royalty>, uri: string::String)
}
```


##### Implementation


```move
module 0x4::token {
    inline fun create_common_with_collection(
        creator: &signer,
        constructor_ref: &ConstructorRef,
        collection: Object<Collection>,
        description: String,
        name_prefix: String,
        // If option::some, numbered token is created - i.e. index is appended to the name.
        // If option::none, name_prefix is the full name of the token.
        name_with_index_suffix: Option<String>,
        royalty: Option<Royalty>,
        uri: String,
    ) {
        assert!(collection::creator(collection) == signer::address_of(creator), error::unauthenticated(ENOT_CREATOR));

        if (option::is_some(&name_with_index_suffix)) {
            // Be conservative, as we don't know what length the index will be, and assume worst case (20 chars in MAX_U64)
            assert!(
                string::length(&name_prefix) + 20 + string::length(
                    option::borrow(&name_with_index_suffix)
                ) <= MAX_TOKEN_NAME_LENGTH,
                error::out_of_range(ETOKEN_NAME_TOO_LONG)
            );
        } else {
            assert!(string::length(&name_prefix) <= MAX_TOKEN_NAME_LENGTH, error::out_of_range(ETOKEN_NAME_TOO_LONG));
        };
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));

        let object_signer = object::generate_signer(constructor_ref);

        let index = option::destroy_with_default(
            collection::increment_supply(&collection, signer::address_of(&object_signer)),
            aggregator_v2::create_snapshot<u64>(0)
        );

        // If create_numbered_token called us, add index to the name.
        let name = if (option::is_some(&name_with_index_suffix)) {
            aggregator_v2::derive_string_concat(name_prefix, &index, option::extract(&mut name_with_index_suffix))
        } else {
            aggregator_v2::create_derived_string(name_prefix)
        };

        let deprecated_index = 0;
        let deprecated_name = string::utf8(b"");

        let token_concurrent = TokenIdentifiers {
            index,
            name,
        };
        move_to(&object_signer, token_concurrent);

        let token = Token {
            collection,
            index: deprecated_index,
            description,
            name: deprecated_name,
            uri,
            mutation_events: object::new_event_handle(&object_signer),
        };
        move_to(&object_signer, token);

        if (option::is_some(&royalty)) {
            royalty::init(constructor_ref, option::extract(&mut royalty))
        };
    }
}
```


<a id="0x4_token_create_token"></a>

## Function `create_token`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
This takes in the collection object instead of the collection name.
This function must be called if the collection name has been previously changed.


```move
module 0x4::token {
    public fun create_token(creator: &signer, collection: object::Object<collection::Collection>, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_token(
        creator: &signer,
        collection: Object<Collection>,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let constructor_ref = object::create_object(creator_address);
        create_common_with_collection(
            creator,
            &constructor_ref,
            collection,
            description,
            name,
            option::none(),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create"></a>

## Function `create`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.


```move
module 0x4::token {
    public fun create(creator: &signer, collection_name: string::String, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create(
        creator: &signer,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let constructor_ref = object::create_object(creator_address);
        create_common(
            creator,
            &constructor_ref,
            collection_name,
            description,
            name,
            option::none(),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_numbered_token_object"></a>

## Function `create_numbered_token_object`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
The name is created by concatenating the (name_prefix, index, name_suffix).
This function allows creating tokens in parallel, from the same collection,
while providing sequential names.

This takes in the collection object instead of the collection name.
This function must be called if the collection name has been previously changed.


```move
module 0x4::token {
    public fun create_numbered_token_object(creator: &signer, collection: object::Object<collection::Collection>, description: string::String, name_with_index_prefix: string::String, name_with_index_suffix: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_numbered_token_object(
        creator: &signer,
        collection: Object<Collection>,
        description: String,
        name_with_index_prefix: String,
        name_with_index_suffix: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let constructor_ref = object::create_object(creator_address);
        create_common_with_collection(
            creator,
            &constructor_ref,
            collection,
            description,
            name_with_index_prefix,
            option::some(name_with_index_suffix),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_numbered_token"></a>

## Function `create_numbered_token`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
The name is created by concatenating the (name_prefix, index, name_suffix).
This function will allow creating tokens in parallel, from the same collection,
while providing sequential names.


```move
module 0x4::token {
    public fun create_numbered_token(creator: &signer, collection_name: string::String, description: string::String, name_with_index_prefix: string::String, name_with_index_suffix: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_numbered_token(
        creator: &signer,
        collection_name: String,
        description: String,
        name_with_index_prefix: String,
        name_with_index_suffix: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let constructor_ref = object::create_object(creator_address);
        create_common(
            creator,
            &constructor_ref,
            collection_name,
            description,
            name_with_index_prefix,
            option::some(name_with_index_suffix),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_named_token_object"></a>

## Function `create_named_token_object`

Creates a new token object from a token name and returns the ConstructorRef for
additional specialization.
This function must be called if the collection name has been previously changed.


```move
module 0x4::token {
    public fun create_named_token_object(creator: &signer, collection: object::Object<collection::Collection>, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_named_token_object(
        creator: &signer,
        collection: Object<Collection>,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let seed = create_token_seed(&collection::name(collection), &name);
        let constructor_ref = object::create_named_object(creator, seed);
        create_common_with_collection(
            creator,
            &constructor_ref,
            collection,
            description,
            name,
            option::none(),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_named_token"></a>

## Function `create_named_token`

Creates a new token object from a token name and returns the ConstructorRef for
additional specialization.


```move
module 0x4::token {
    public fun create_named_token(creator: &signer, collection_name: string::String, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_named_token(
        creator: &signer,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let seed = create_token_seed(&collection_name, &name);

        let constructor_ref = object::create_named_object(creator, seed);
        create_common(
            creator,
            &constructor_ref,
            collection_name,
            description,
            name,
            option::none(),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_named_token_from_seed"></a>

## Function `create_named_token_from_seed`

Creates a new token object from a token name and seed.
Returns the ConstructorRef for additional specialization.
This function must be called if the collection name has been previously changed.


```move
module 0x4::token {
    public fun create_named_token_from_seed(creator: &signer, collection: object::Object<collection::Collection>, description: string::String, name: string::String, seed: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_named_token_from_seed(
        creator: &signer,
        collection: Object<Collection>,
        description: String,
        name: String,
        seed: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let seed = create_token_name_with_seed(&collection::name(collection), &name, &seed);
        let constructor_ref = object::create_named_object(creator, seed);
        create_common_with_collection(creator, &constructor_ref, collection, description, name, option::none(), royalty, uri);
        constructor_ref
    }
}
```


<a id="0x4_token_create_from_account"></a>

## Function `create_from_account`

DEPRECATED: Use `create` instead for identical behavior.

Creates a new token object from an account GUID and returns the ConstructorRef for
additional specialization.


```move
module 0x4::token {
    #[deprecated]
    public fun create_from_account(creator: &signer, collection_name: string::String, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_from_account(
        creator: &signer,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let constructor_ref = object::create_object_from_account(creator);
        create_common(
            creator,
            &constructor_ref,
            collection_name,
            description,
            name,
            option::none(),
            royalty,
            uri
        );
        constructor_ref
    }
}
```


<a id="0x4_token_create_token_address"></a>

## Function `create_token_address`

Generates the token&apos;s address based upon the creator&apos;s address, the collection&apos;s name and the token&apos;s name.


```move
module 0x4::token {
    public fun create_token_address(creator: &address, collection: &string::String, name: &string::String): address
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_token_address(creator: &address, collection: &String, name: &String): address {
        object::create_object_address(creator, create_token_seed(collection, name))
    }
}
```


<a id="0x4_token_create_token_address_with_seed"></a>

## Function `create_token_address_with_seed`

Generates the token&apos;s address based upon the creator&apos;s address, the collection object and the token&apos;s name and seed.


```move
module 0x4::token {
    #[view]
    public fun create_token_address_with_seed(creator: address, collection: string::String, name: string::String, seed: string::String): address
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_token_address_with_seed(creator: address, collection: String, name: String, seed: String): address {
        let seed = create_token_name_with_seed(&collection, &name, &seed);
        object::create_object_address(&creator, seed)
    }
}
```


<a id="0x4_token_create_token_seed"></a>

## Function `create_token_seed`

Named objects are derived from a seed, the token&apos;s seed is its name appended to the collection&apos;s name.


```move
module 0x4::token {
    public fun create_token_seed(collection: &string::String, name: &string::String): vector<u8>
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_token_seed(collection: &String, name: &String): vector<u8> {
        assert!(string::length(name) <= MAX_TOKEN_NAME_LENGTH, error::out_of_range(ETOKEN_NAME_TOO_LONG));
        let seed = *string::bytes(collection);
        vector::append(&mut seed, b"::");
        vector::append(&mut seed, *string::bytes(name));
        seed
    }
}
```


<a id="0x4_token_create_token_name_with_seed"></a>

## Function `create_token_name_with_seed`



```move
module 0x4::token {
    public fun create_token_name_with_seed(collection: &string::String, name: &string::String, seed: &string::String): vector<u8>
}
```


##### Implementation


```move
module 0x4::token {
    public fun create_token_name_with_seed(collection: &String, name: &String, seed: &String): vector<u8> {
        assert!(string::length(seed) <= MAX_TOKEN_SEED_LENGTH, error::out_of_range(ESEED_TOO_LONG));
        let seeds = create_token_seed(collection, name);
        vector::append(&mut seeds, *string::bytes(seed));
        seeds
    }
}
```


<a id="0x4_token_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


```move
module 0x4::token {
    public fun generate_mutator_ref(ref: &object::ConstructorRef): token::MutatorRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun generate_mutator_ref(ref: &ConstructorRef): MutatorRef {
        let object = object::object_from_constructor_ref<Token>(ref);
        MutatorRef { self: object::object_address(&object) }
    }
}
```


<a id="0x4_token_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a BurnRef, which gates the ability to burn the given token.


```move
module 0x4::token {
    public fun generate_burn_ref(ref: &object::ConstructorRef): token::BurnRef
}
```


##### Implementation


```move
module 0x4::token {
    public fun generate_burn_ref(ref: &ConstructorRef): BurnRef {
        let (inner, self) = if (object::can_generate_delete_ref(ref)) {
            let delete_ref = object::generate_delete_ref(ref);
            (option::some(delete_ref), option::none())
        } else {
            let addr = object::address_from_constructor_ref(ref);
            (option::none(), option::some(addr))
        };
        BurnRef { self, inner }
    }
}
```


<a id="0x4_token_address_from_burn_ref"></a>

## Function `address_from_burn_ref`

Extracts the tokens address from a BurnRef.


```move
module 0x4::token {
    public fun address_from_burn_ref(ref: &token::BurnRef): address
}
```


##### Implementation


```move
module 0x4::token {
    public fun address_from_burn_ref(ref: &BurnRef): address {
        if (option::is_some(&ref.inner)) {
            object::address_from_delete_ref(option::borrow(&ref.inner))
        } else {
            *option::borrow(&ref.self)
        }
    }
}
```


<a id="0x4_token_borrow"></a>

## Function `borrow`



```move
module 0x4::token {
    fun borrow<T: key>(token: &object::Object<T>): &token::Token
}
```


##### Implementation


```move
module 0x4::token {
    inline fun borrow<T: key>(token: &Object<T>): &Token acquires Token {
        let token_address = object::object_address(token);
        assert!(
            exists<Token>(token_address),
            error::not_found(ETOKEN_DOES_NOT_EXIST),
        );
        borrow_global<Token>(token_address)
    }
}
```


<a id="0x4_token_creator"></a>

## Function `creator`



```move
module 0x4::token {
    #[view]
    public fun creator<T: key>(token: object::Object<T>): address
}
```


##### Implementation


```move
module 0x4::token {
    public fun creator<T: key>(token: Object<T>): address acquires Token {
        collection::creator(borrow(&token).collection)
    }
}
```


<a id="0x4_token_collection_name"></a>

## Function `collection_name`



```move
module 0x4::token {
    #[view]
    public fun collection_name<T: key>(token: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::token {
    public fun collection_name<T: key>(token: Object<T>): String acquires Token {
        collection::name(borrow(&token).collection)
    }
}
```


<a id="0x4_token_collection_object"></a>

## Function `collection_object`



```move
module 0x4::token {
    #[view]
    public fun collection_object<T: key>(token: object::Object<T>): object::Object<collection::Collection>
}
```


##### Implementation


```move
module 0x4::token {
    public fun collection_object<T: key>(token: Object<T>): Object<Collection> acquires Token {
        borrow(&token).collection
    }
}
```


<a id="0x4_token_description"></a>

## Function `description`



```move
module 0x4::token {
    #[view]
    public fun description<T: key>(token: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::token {
    public fun description<T: key>(token: Object<T>): String acquires Token {
        borrow(&token).description
    }
}
```


<a id="0x4_token_name"></a>

## Function `name`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


```move
module 0x4::token {
    #[view]
    public fun name<T: key>(token: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::token {
    public fun name<T: key>(token: Object<T>): String acquires Token, TokenIdentifiers {
        let token_address = object::object_address(&token);
        if (exists<TokenIdentifiers>(token_address)) {
            aggregator_v2::read_derived_string(&borrow_global<TokenIdentifiers>(token_address).name)
        } else {
            borrow(&token).name
        }
    }
}
```


<a id="0x4_token_uri"></a>

## Function `uri`



```move
module 0x4::token {
    #[view]
    public fun uri<T: key>(token: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::token {
    public fun uri<T: key>(token: Object<T>): String acquires Token {
        borrow(&token).uri
    }
}
```


<a id="0x4_token_royalty"></a>

## Function `royalty`



```move
module 0x4::token {
    #[view]
    public fun royalty<T: key>(token: object::Object<T>): option::Option<royalty::Royalty>
}
```


##### Implementation


```move
module 0x4::token {
    public fun royalty<T: key>(token: Object<T>): Option<Royalty> acquires Token {
        borrow(&token);
        let royalty = royalty::get(token);
        if (option::is_some(&royalty)) {
            royalty
        } else {
            let creator = creator(token);
            let collection_name = collection_name(token);
            let collection_address = collection::create_collection_address(&creator, &collection_name);
            let collection = object::address_to_object<collection::Collection>(collection_address);
            royalty::get(collection)
        }
    }
}
```


<a id="0x4_token_index"></a>

## Function `index`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


```move
module 0x4::token {
    #[view]
    public fun index<T: key>(token: object::Object<T>): u64
}
```


##### Implementation


```move
module 0x4::token {
    public fun index<T: key>(token: Object<T>): u64 acquires Token, TokenIdentifiers {
        let token_address = object::object_address(&token);
        if (exists<TokenIdentifiers>(token_address)) {
            aggregator_v2::read_snapshot(&borrow_global<TokenIdentifiers>(token_address).index)
        } else {
            borrow(&token).index
        }
    }
}
```


<a id="0x4_token_borrow_mut"></a>

## Function `borrow_mut`



```move
module 0x4::token {
    fun borrow_mut(mutator_ref: &token::MutatorRef): &mut token::Token
}
```


##### Implementation


```move
module 0x4::token {
    inline fun borrow_mut(mutator_ref: &MutatorRef): &mut Token acquires Token {
        assert!(
            exists<Token>(mutator_ref.self),
            error::not_found(ETOKEN_DOES_NOT_EXIST),
        );
        borrow_global_mut<Token>(mutator_ref.self)
    }
}
```


<a id="0x4_token_burn"></a>

## Function `burn`



```move
module 0x4::token {
    public fun burn(burn_ref: token::BurnRef)
}
```


##### Implementation


```move
module 0x4::token {
    public fun burn(burn_ref: BurnRef) acquires Token, TokenIdentifiers {
        let (addr, previous_owner) = if (option::is_some(&burn_ref.inner)) {
            let delete_ref = option::extract(&mut burn_ref.inner);
            let addr = object::address_from_delete_ref(&delete_ref);
            let previous_owner = object::owner(object::address_to_object<Token>(addr));
            object::delete(delete_ref);
            (addr, previous_owner)
        } else {
            let addr = option::extract(&mut burn_ref.self);
            let previous_owner = object::owner(object::address_to_object<Token>(addr));
            (addr, previous_owner)
        };

        if (royalty::exists_at(addr)) {
            royalty::delete(addr)
        };

        let Token {
            collection,
            index: deprecated_index,
            description: _,
            name: _,
            uri: _,
            mutation_events,
        } = move_from<Token>(addr);

        let index = if (exists<TokenIdentifiers>(addr)) {
            let TokenIdentifiers {
                index,
                name: _,
            } = move_from<TokenIdentifiers>(addr);
            aggregator_v2::read_snapshot(&index)
        } else {
            deprecated_index
        };

        event::destroy_handle(mutation_events);
        collection::decrement_supply(&collection, addr, option::some(index), previous_owner);
    }
}
```


<a id="0x4_token_set_description"></a>

## Function `set_description`



```move
module 0x4::token {
    public fun set_description(mutator_ref: &token::MutatorRef, description: string::String)
}
```


##### Implementation


```move
module 0x4::token {
    public fun set_description(mutator_ref: &MutatorRef, description: String) acquires Token {
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));
        let token = borrow_mut(mutator_ref);
        if (std::features::module_event_migration_enabled()) {
            event::emit(Mutation {
                token_address: mutator_ref.self,
                mutated_field_name: string::utf8(b"description"),
                old_value: token.description,
                new_value: description
            })
        };
        event::emit_event(
            &mut token.mutation_events,
            MutationEvent {
                mutated_field_name: string::utf8(b"description"),
                old_value: token.description,
                new_value: description
            },
        );
        token.description = description;
    }
}
```


<a id="0x4_token_set_name"></a>

## Function `set_name`



```move
module 0x4::token {
    public fun set_name(mutator_ref: &token::MutatorRef, name: string::String)
}
```


##### Implementation


```move
module 0x4::token {
    public fun set_name(mutator_ref: &MutatorRef, name: String) acquires Token, TokenIdentifiers {
        assert!(string::length(&name) <= MAX_TOKEN_NAME_LENGTH, error::out_of_range(ETOKEN_NAME_TOO_LONG));

        let token = borrow_mut(mutator_ref);

        let old_name = if (exists<TokenIdentifiers>(mutator_ref.self)) {
            let token_concurrent = borrow_global_mut<TokenIdentifiers>(mutator_ref.self);
            let old_name = aggregator_v2::read_derived_string(&token_concurrent.name);
            token_concurrent.name = aggregator_v2::create_derived_string(name);
            old_name
        } else {
            let old_name = token.name;
            token.name = name;
            old_name
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(Mutation {
                token_address: mutator_ref.self,
                mutated_field_name: string::utf8(b"name"),
                old_value: old_name,
                new_value: name
            })
        };
        event::emit_event(
            &mut token.mutation_events,
            MutationEvent {
                mutated_field_name: string::utf8(b"name"),
                old_value: old_name,
                new_value: name
            },
        );
    }
}
```


<a id="0x4_token_set_uri"></a>

## Function `set_uri`



```move
module 0x4::token {
    public fun set_uri(mutator_ref: &token::MutatorRef, uri: string::String)
}
```


##### Implementation


```move
module 0x4::token {
    public fun set_uri(mutator_ref: &MutatorRef, uri: String) acquires Token {
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        let token = borrow_mut(mutator_ref);
        if (std::features::module_event_migration_enabled()) {
            event::emit(Mutation {
                token_address: mutator_ref.self,
                mutated_field_name: string::utf8(b"uri"),
                old_value: token.uri,
                new_value: uri,
            })
        };
        event::emit_event(
            &mut token.mutation_events,
            MutationEvent {
                mutated_field_name: string::utf8(b"uri"),
                old_value: token.uri,
                new_value: uri,
            },
        );
        token.uri = uri;
    }
}
```
