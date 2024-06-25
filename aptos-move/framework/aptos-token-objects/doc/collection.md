
<a id="0x4_collection"></a>

# Module `0x4::collection`

This defines an object&#45;based Collection. A collection acts as a set organizer for a group of
tokens. This includes aspects such as a general description, project URI, name, and may contain
other useful generalizations across this set of tokens.

Being built upon objects enables collections to be relatively flexible. As core primitives it
supports:
&#42; Common fields: name, uri, description, creator
&#42; MutatorRef leaving mutability configuration to a higher level component
&#42; Addressed by a global identifier of creator&apos;s address and collection name, thus collections
cannot be deleted as a restriction of the object model.
&#42; Optional support for collection&#45;wide royalties
&#42; Optional support for tracking of supply with events on mint or burn

TODO:
&#42; Consider supporting changing the name of the collection with the MutatorRef. This would
require adding the field original_name.
&#42; Consider supporting changing the aspects of supply with the MutatorRef.
&#42; Add aggregator support when added to framework


-  [Resource `Collection`](#0x4_collection_Collection)
-  [Struct `MutatorRef`](#0x4_collection_MutatorRef)
-  [Struct `MutationEvent`](#0x4_collection_MutationEvent)
-  [Struct `Mutation`](#0x4_collection_Mutation)
-  [Resource `FixedSupply`](#0x4_collection_FixedSupply)
-  [Resource `UnlimitedSupply`](#0x4_collection_UnlimitedSupply)
-  [Resource `ConcurrentSupply`](#0x4_collection_ConcurrentSupply)
-  [Struct `BurnEvent`](#0x4_collection_BurnEvent)
-  [Struct `MintEvent`](#0x4_collection_MintEvent)
-  [Struct `Burn`](#0x4_collection_Burn)
-  [Struct `Mint`](#0x4_collection_Mint)
-  [Struct `ConcurrentBurnEvent`](#0x4_collection_ConcurrentBurnEvent)
-  [Struct `ConcurrentMintEvent`](#0x4_collection_ConcurrentMintEvent)
-  [Struct `SetMaxSupply`](#0x4_collection_SetMaxSupply)
-  [Constants](#@Constants_0)
-  [Function `create_fixed_collection`](#0x4_collection_create_fixed_collection)
-  [Function `create_unlimited_collection`](#0x4_collection_create_unlimited_collection)
-  [Function `create_untracked_collection`](#0x4_collection_create_untracked_collection)
-  [Function `create_collection_internal`](#0x4_collection_create_collection_internal)
-  [Function `create_collection_address`](#0x4_collection_create_collection_address)
-  [Function `create_collection_seed`](#0x4_collection_create_collection_seed)
-  [Function `increment_supply`](#0x4_collection_increment_supply)
-  [Function `decrement_supply`](#0x4_collection_decrement_supply)
-  [Function `generate_mutator_ref`](#0x4_collection_generate_mutator_ref)
-  [Function `upgrade_to_concurrent`](#0x4_collection_upgrade_to_concurrent)
-  [Function `check_collection_exists`](#0x4_collection_check_collection_exists)
-  [Function `borrow`](#0x4_collection_borrow)
-  [Function `count`](#0x4_collection_count)
-  [Function `creator`](#0x4_collection_creator)
-  [Function `description`](#0x4_collection_description)
-  [Function `name`](#0x4_collection_name)
-  [Function `uri`](#0x4_collection_uri)
-  [Function `borrow_mut`](#0x4_collection_borrow_mut)
-  [Function `set_name`](#0x4_collection_set_name)
-  [Function `set_description`](#0x4_collection_set_description)
-  [Function `set_uri`](#0x4_collection_set_uri)
-  [Function `set_max_supply`](#0x4_collection_set_max_supply)


```move
module 0x4::collection {
    use 0x1::aggregator_v2;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x4::royalty;
}
```


<a id="0x4_collection_Collection"></a>

## Resource `Collection`

Represents the common fields for a collection.


```move
module 0x4::collection {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Collection has key
}
```


##### Fields


<dl>
<dt>
`creator: address`
</dt>
<dd>
 The creator of this collection.
</dd>
<dt>
`description: string::String`
</dt>
<dd>
 A brief description of the collection.
</dd>
<dt>
`name: string::String`
</dt>
<dd>
 An optional categorization of similar token.
</dd>
<dt>
`uri: string::String`
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off&#45;chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
<dt>
`mutation_events: event::EventHandle<collection::MutationEvent>`
</dt>
<dd>
 Emitted upon any mutation of the collection.
</dd>
</dl>


<a id="0x4_collection_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


```move
module 0x4::collection {
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


<a id="0x4_collection_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


```move
module 0x4::collection {
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
</dl>


<a id="0x4_collection_Mutation"></a>

## Struct `Mutation`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


```move
module 0x4::collection {
    #[event]
    struct Mutation has drop, store
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
`collection: object::Object<collection::Collection>`
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


<a id="0x4_collection_FixedSupply"></a>

## Resource `FixedSupply`

Fixed supply tracker, this is useful for ensuring that a limited number of tokens are minted.
and adding events and supply tracking to a collection.


```move
module 0x4::collection {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct FixedSupply has key
}
```


##### Fields


<dl>
<dt>
`current_supply: u64`
</dt>
<dd>
 Total minted &#45; total burned
</dd>
<dt>
`max_supply: u64`
</dt>
<dd>

</dd>
<dt>
`total_minted: u64`
</dt>
<dd>

</dd>
<dt>
`burn_events: event::EventHandle<collection::BurnEvent>`
</dt>
<dd>
 Emitted upon burning a Token.
</dd>
<dt>
`mint_events: event::EventHandle<collection::MintEvent>`
</dt>
<dd>
 Emitted upon minting an Token.
</dd>
</dl>


<a id="0x4_collection_UnlimitedSupply"></a>

## Resource `UnlimitedSupply`

Unlimited supply tracker, this is useful for adding events and supply tracking to a collection.


```move
module 0x4::collection {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct UnlimitedSupply has key
}
```


##### Fields


<dl>
<dt>
`current_supply: u64`
</dt>
<dd>

</dd>
<dt>
`total_minted: u64`
</dt>
<dd>

</dd>
<dt>
`burn_events: event::EventHandle<collection::BurnEvent>`
</dt>
<dd>
 Emitted upon burning a Token.
</dd>
<dt>
`mint_events: event::EventHandle<collection::MintEvent>`
</dt>
<dd>
 Emitted upon minting an Token.
</dd>
</dl>


<a id="0x4_collection_ConcurrentSupply"></a>

## Resource `ConcurrentSupply`

Supply tracker, useful for tracking amount of issued tokens.
If max_value is not set to U64_MAX, this ensures that a limited number of tokens are minted.


```move
module 0x4::collection {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct ConcurrentSupply has key
}
```


##### Fields


<dl>
<dt>
`current_supply: aggregator_v2::Aggregator<u64>`
</dt>
<dd>
 Total minted &#45; total burned
</dd>
<dt>
`total_minted: aggregator_v2::Aggregator<u64>`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_BurnEvent"></a>

## Struct `BurnEvent`



```move
module 0x4::collection {
    struct BurnEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`index: u64`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_MintEvent"></a>

## Struct `MintEvent`



```move
module 0x4::collection {
    struct MintEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`index: u64`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_Burn"></a>

## Struct `Burn`



```move
module 0x4::collection {
    #[event]
    struct Burn has drop, store
}
```


##### Fields


<dl>
<dt>
`collection: address`
</dt>
<dd>

</dd>
<dt>
`index: u64`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
<dt>
`previous_owner: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_Mint"></a>

## Struct `Mint`



```move
module 0x4::collection {
    #[event]
    struct Mint has drop, store
}
```


##### Fields


<dl>
<dt>
`collection: address`
</dt>
<dd>

</dd>
<dt>
`index: aggregator_v2::AggregatorSnapshot<u64>`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_ConcurrentBurnEvent"></a>

## Struct `ConcurrentBurnEvent`



```move
module 0x4::collection {
    #[event]
    #[deprecated]
    struct ConcurrentBurnEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`collection_addr: address`
</dt>
<dd>

</dd>
<dt>
`index: u64`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_ConcurrentMintEvent"></a>

## Struct `ConcurrentMintEvent`



```move
module 0x4::collection {
    #[event]
    #[deprecated]
    struct ConcurrentMintEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`collection_addr: address`
</dt>
<dd>

</dd>
<dt>
`index: aggregator_v2::AggregatorSnapshot<u64>`
</dt>
<dd>

</dd>
<dt>
`token: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_collection_SetMaxSupply"></a>

## Struct `SetMaxSupply`



```move
module 0x4::collection {
    #[event]
    struct SetMaxSupply has drop, store
}
```


##### Fields


<dl>
<dt>
`collection: object::Object<collection::Collection>`
</dt>
<dd>

</dd>
<dt>
`old_max_supply: u64`
</dt>
<dd>

</dd>
<dt>
`new_max_supply: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x4_collection_MAX_U64"></a>



```move
module 0x4::collection {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x4_collection_EURI_TOO_LONG"></a>

The URI is over the maximum length


```move
module 0x4::collection {
    const EURI_TOO_LONG: u64 = 4;
}
```


<a id="0x4_collection_MAX_URI_LENGTH"></a>



```move
module 0x4::collection {
    const MAX_URI_LENGTH: u64 = 512;
}
```


<a id="0x4_collection_EALREADY_CONCURRENT"></a>

Tried upgrading collection to concurrent, but collection is already concurrent


```move
module 0x4::collection {
    const EALREADY_CONCURRENT: u64 = 8;
}
```


<a id="0x4_collection_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


```move
module 0x4::collection {
    const ECOLLECTION_DOES_NOT_EXIST: u64 = 1;
}
```


<a id="0x4_collection_ECOLLECTION_NAME_TOO_LONG"></a>

The collection name is over the maximum length


```move
module 0x4::collection {
    const ECOLLECTION_NAME_TOO_LONG: u64 = 3;
}
```


<a id="0x4_collection_ECOLLECTION_SUPPLY_EXCEEDED"></a>

The collection has reached its supply and no more tokens can be minted, unless some are burned


```move
module 0x4::collection {
    const ECOLLECTION_SUPPLY_EXCEEDED: u64 = 2;
}
```


<a id="0x4_collection_ECONCURRENT_NOT_ENABLED"></a>

Concurrent feature flag is not yet enabled, so the function cannot be performed


```move
module 0x4::collection {
    const ECONCURRENT_NOT_ENABLED: u64 = 7;
}
```


<a id="0x4_collection_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


```move
module 0x4::collection {
    const EDESCRIPTION_TOO_LONG: u64 = 5;
}
```


<a id="0x4_collection_EINVALID_MAX_SUPPLY"></a>

The new max supply cannot be less than the current supply


```move
module 0x4::collection {
    const EINVALID_MAX_SUPPLY: u64 = 9;
}
```


<a id="0x4_collection_EMAX_SUPPLY_CANNOT_BE_ZERO"></a>

The max supply must be positive


```move
module 0x4::collection {
    const EMAX_SUPPLY_CANNOT_BE_ZERO: u64 = 6;
}
```


<a id="0x4_collection_ENO_MAX_SUPPLY_IN_COLLECTION"></a>

The collection does not have a max supply


```move
module 0x4::collection {
    const ENO_MAX_SUPPLY_IN_COLLECTION: u64 = 10;
}
```


<a id="0x4_collection_MAX_COLLECTION_NAME_LENGTH"></a>



```move
module 0x4::collection {
    const MAX_COLLECTION_NAME_LENGTH: u64 = 128;
}
```


<a id="0x4_collection_MAX_DESCRIPTION_LENGTH"></a>



```move
module 0x4::collection {
    const MAX_DESCRIPTION_LENGTH: u64 = 2048;
}
```


<a id="0x4_collection_create_fixed_collection"></a>

## Function `create_fixed_collection`

Creates a fixed&#45;sized collection, or a collection that supports a fixed amount of tokens.
This is useful to create a guaranteed, limited supply on&#45;chain digital asset. For example,
a collection 1111 vicious vipers. Note, creating restrictions such as upward limits results
in data structures that prevent Aptos from parallelizing mints of this collection type.
Beyond that, it adds supply tracking with events.


```move
module 0x4::collection {
    public fun create_fixed_collection(creator: &signer, description: string::String, max_supply: u64, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::collection {
    public fun create_fixed_collection(
        creator: &signer,
        description: String,
        max_supply: u64,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        assert!(max_supply != 0, error::invalid_argument(EMAX_SUPPLY_CANNOT_BE_ZERO));
        let collection_seed = create_collection_seed(&name);
        let constructor_ref = object::create_named_object(creator, collection_seed);

        let supply = ConcurrentSupply {
            current_supply: aggregator_v2::create_aggregator(max_supply),
            total_minted: aggregator_v2::create_unbounded_aggregator(),
        };

        create_collection_internal(
            creator,
            constructor_ref,
            description,
            name,
            royalty,
            uri,
            option::some(supply),
        )
    }
}
```


<a id="0x4_collection_create_unlimited_collection"></a>

## Function `create_unlimited_collection`

Creates an unlimited collection. This has support for supply tracking but does not limit
the supply of tokens.


```move
module 0x4::collection {
    public fun create_unlimited_collection(creator: &signer, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::collection {
    public fun create_unlimited_collection(
        creator: &signer,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let collection_seed = create_collection_seed(&name);
        let constructor_ref = object::create_named_object(creator, collection_seed);

        let supply = ConcurrentSupply {
            current_supply: aggregator_v2::create_unbounded_aggregator(),
            total_minted: aggregator_v2::create_unbounded_aggregator(),
        };

        create_collection_internal(
            creator,
            constructor_ref,
            description,
            name,
            royalty,
            uri,
            option::some(supply),
        )
    }
}
```


<a id="0x4_collection_create_untracked_collection"></a>

## Function `create_untracked_collection`

Creates an untracked collection, or a collection that supports an arbitrary amount of
tokens. This is useful for mass airdrops that fully leverage Aptos parallelization.
TODO: Hide this until we bring back meaningful way to enforce burns


```move
module 0x4::collection {
    fun create_untracked_collection(creator: &signer, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::collection {
    fun create_untracked_collection(
        creator: &signer,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let collection_seed = create_collection_seed(&name);
        let constructor_ref = object::create_named_object(creator, collection_seed);

        create_collection_internal<FixedSupply>(
            creator,
            constructor_ref,
            description,
            name,
            royalty,
            uri,
            option::none(),
        )
    }
}
```


<a id="0x4_collection_create_collection_internal"></a>

## Function `create_collection_internal`



```move
module 0x4::collection {
    fun create_collection_internal<Supply: key>(creator: &signer, constructor_ref: object::ConstructorRef, description: string::String, name: string::String, royalty: option::Option<royalty::Royalty>, uri: string::String, supply: option::Option<Supply>): object::ConstructorRef
}
```


##### Implementation


```move
module 0x4::collection {
    inline fun create_collection_internal<Supply: key>(
        creator: &signer,
        constructor_ref: ConstructorRef,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
        supply: Option<Supply>,
    ): ConstructorRef {
        assert!(string::length(&name) <= MAX_COLLECTION_NAME_LENGTH, error::out_of_range(ECOLLECTION_NAME_TOO_LONG));
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));

        let object_signer = object::generate_signer(&constructor_ref);

        let collection = Collection {
            creator: signer::address_of(creator),
            description,
            name,
            uri,
            mutation_events: object::new_event_handle(&object_signer),
        };
        move_to(&object_signer, collection);

        if (option::is_some(&supply)) {
            move_to(&object_signer, option::destroy_some(supply))
        } else {
            option::destroy_none(supply)
        };

        if (option::is_some(&royalty)) {
            royalty::init(&constructor_ref, option::extract(&mut royalty))
        };

        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        object::disable_ungated_transfer(&transfer_ref);

        constructor_ref
    }
}
```


<a id="0x4_collection_create_collection_address"></a>

## Function `create_collection_address`

Generates the collections address based upon the creators address and the collection&apos;s name


```move
module 0x4::collection {
    public fun create_collection_address(creator: &address, name: &string::String): address
}
```


##### Implementation


```move
module 0x4::collection {
    public fun create_collection_address(creator: &address, name: &String): address {
        object::create_object_address(creator, create_collection_seed(name))
    }
}
```


<a id="0x4_collection_create_collection_seed"></a>

## Function `create_collection_seed`

Named objects are derived from a seed, the collection&apos;s seed is its name.


```move
module 0x4::collection {
    public fun create_collection_seed(name: &string::String): vector<u8>
}
```


##### Implementation


```move
module 0x4::collection {
    public fun create_collection_seed(name: &String): vector<u8> {
        assert!(string::length(name) <= MAX_COLLECTION_NAME_LENGTH, error::out_of_range(ECOLLECTION_NAME_TOO_LONG));
        *string::bytes(name)
    }
}
```


<a id="0x4_collection_increment_supply"></a>

## Function `increment_supply`

Called by token on mint to increment supply if there&apos;s an appropriate Supply struct.


```move
module 0x4::collection {
    public(friend) fun increment_supply(collection: &object::Object<collection::Collection>, token: address): option::Option<aggregator_v2::AggregatorSnapshot<u64>>
}
```


##### Implementation


```move
module 0x4::collection {
    public(friend) fun increment_supply(
        collection: &Object<Collection>,
        token: address,
    ): Option<AggregatorSnapshot<u64>> acquires FixedSupply, UnlimitedSupply, ConcurrentSupply {
        let collection_addr = object::object_address(collection);
        if (exists<ConcurrentSupply>(collection_addr)) {
            let supply = borrow_global_mut<ConcurrentSupply>(collection_addr);
            assert!(
                aggregator_v2::try_add(&mut supply.current_supply, 1),
                error::out_of_range(ECOLLECTION_SUPPLY_EXCEEDED),
            );
            aggregator_v2::add(&mut supply.total_minted, 1);
            event::emit(
                Mint {
                    collection: collection_addr,
                    index: aggregator_v2::snapshot(&supply.total_minted),
                    token,
                },
            );
            option::some(aggregator_v2::snapshot(&supply.total_minted))
        } else if (exists<FixedSupply>(collection_addr)) {
            let supply = borrow_global_mut<FixedSupply>(collection_addr);
            supply.current_supply = supply.current_supply + 1;
            supply.total_minted = supply.total_minted + 1;
            assert!(
                supply.current_supply <= supply.max_supply,
                error::out_of_range(ECOLLECTION_SUPPLY_EXCEEDED),
            );
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    Mint {
                        collection: collection_addr,
                        index: aggregator_v2::create_snapshot(supply.total_minted),
                        token,
                    },
                );
            };
            event::emit_event(&mut supply.mint_events,
                MintEvent {
                    index: supply.total_minted,
                    token,
                },
            );
            option::some(aggregator_v2::create_snapshot<u64>(supply.total_minted))
        } else if (exists<UnlimitedSupply>(collection_addr)) {
            let supply = borrow_global_mut<UnlimitedSupply>(collection_addr);
            supply.current_supply = supply.current_supply + 1;
            supply.total_minted = supply.total_minted + 1;
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    Mint {
                        collection: collection_addr,
                        index: aggregator_v2::create_snapshot(supply.total_minted),
                        token,
                    },
                );
            };
            event::emit_event(
                &mut supply.mint_events,
                MintEvent {
                    index: supply.total_minted,
                    token,
                },
            );
            option::some(aggregator_v2::create_snapshot<u64>(supply.total_minted))
        } else {
            option::none()
        }
    }
}
```


<a id="0x4_collection_decrement_supply"></a>

## Function `decrement_supply`

Called by token on burn to decrement supply if there&apos;s an appropriate Supply struct.


```move
module 0x4::collection {
    public(friend) fun decrement_supply(collection: &object::Object<collection::Collection>, token: address, index: option::Option<u64>, previous_owner: address)
}
```


##### Implementation


```move
module 0x4::collection {
    public(friend) fun decrement_supply(
        collection: &Object<Collection>,
        token: address,
        index: Option<u64>,
        previous_owner: address,
    ) acquires FixedSupply, UnlimitedSupply, ConcurrentSupply {
        let collection_addr = object::object_address(collection);
        if (exists<ConcurrentSupply>(collection_addr)) {
            let supply = borrow_global_mut<ConcurrentSupply>(collection_addr);
            aggregator_v2::sub(&mut supply.current_supply, 1);

            event::emit(
                Burn {
                    collection: collection_addr,
                    index: *option::borrow(&index),
                    token,
                    previous_owner,
                },
            );
        } else if (exists<FixedSupply>(collection_addr)) {
            let supply = borrow_global_mut<FixedSupply>(collection_addr);
            supply.current_supply = supply.current_supply - 1;
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    Burn {
                        collection: collection_addr,
                        index: *option::borrow(&index),
                        token,
                        previous_owner,
                    },
                );
            };
            event::emit_event(
                &mut supply.burn_events,
                BurnEvent {
                    index: *option::borrow(&index),
                    token,
                },
            );
        } else if (exists<UnlimitedSupply>(collection_addr)) {
            let supply = borrow_global_mut<UnlimitedSupply>(collection_addr);
            supply.current_supply = supply.current_supply - 1;
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    Burn {
                        collection: collection_addr,
                        index: *option::borrow(&index),
                        token,
                        previous_owner,
                    },
                );
            };
            event::emit_event(
                &mut supply.burn_events,
                BurnEvent {
                    index: *option::borrow(&index),
                    token,
                },
            );
        }
    }
}
```


<a id="0x4_collection_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


```move
module 0x4::collection {
    public fun generate_mutator_ref(ref: &object::ConstructorRef): collection::MutatorRef
}
```


##### Implementation


```move
module 0x4::collection {
    public fun generate_mutator_ref(ref: &ConstructorRef): MutatorRef {
        let object = object::object_from_constructor_ref<Collection>(ref);
        MutatorRef { self: object::object_address(&object) }
    }
}
```


<a id="0x4_collection_upgrade_to_concurrent"></a>

## Function `upgrade_to_concurrent`



```move
module 0x4::collection {
    public fun upgrade_to_concurrent(ref: &object::ExtendRef)
}
```


##### Implementation


```move
module 0x4::collection {
    public fun upgrade_to_concurrent(
        ref: &ExtendRef,
    ) acquires FixedSupply, UnlimitedSupply {
        let metadata_object_address = object::address_from_extend_ref(ref);
        let metadata_object_signer = object::generate_signer_for_extending(ref);

        let (supply, current_supply, total_minted, burn_events, mint_events) = if (exists<FixedSupply>(
            metadata_object_address
        )) {
            let FixedSupply {
                current_supply,
                max_supply,
                total_minted,
                burn_events,
                mint_events,
            } = move_from<FixedSupply>(metadata_object_address);

            let supply = ConcurrentSupply {
                current_supply: aggregator_v2::create_aggregator(max_supply),
                total_minted: aggregator_v2::create_unbounded_aggregator(),
            };
            (supply, current_supply, total_minted, burn_events, mint_events)
        } else if (exists<UnlimitedSupply>(metadata_object_address)) {
            let UnlimitedSupply {
                current_supply,
                total_minted,
                burn_events,
                mint_events,
            } = move_from<UnlimitedSupply>(metadata_object_address);

            let supply = ConcurrentSupply {
                current_supply: aggregator_v2::create_unbounded_aggregator(),
                total_minted: aggregator_v2::create_unbounded_aggregator(),
            };
            (supply, current_supply, total_minted, burn_events, mint_events)
        } else {
            // untracked collection is already concurrent, and other variants too.
            abort error::invalid_argument(EALREADY_CONCURRENT)
        };

        // update current state:
        aggregator_v2::add(&mut supply.current_supply, current_supply);
        aggregator_v2::add(&mut supply.total_minted, total_minted);
        move_to(&metadata_object_signer, supply);

        event::destroy_handle(burn_events);
        event::destroy_handle(mint_events);
    }
}
```


<a id="0x4_collection_check_collection_exists"></a>

## Function `check_collection_exists`



```move
module 0x4::collection {
    fun check_collection_exists(addr: address)
}
```


##### Implementation


```move
module 0x4::collection {
    inline fun check_collection_exists(addr: address) {
        assert!(
            exists<Collection>(addr),
            error::not_found(ECOLLECTION_DOES_NOT_EXIST),
        );
    }
}
```


<a id="0x4_collection_borrow"></a>

## Function `borrow`



```move
module 0x4::collection {
    fun borrow<T: key>(collection: &object::Object<T>): &collection::Collection
}
```


##### Implementation


```move
module 0x4::collection {
    inline fun borrow<T: key>(collection: &Object<T>): &Collection {
        let collection_address = object::object_address(collection);
        check_collection_exists(collection_address);
        borrow_global<Collection>(collection_address)
    }
}
```


<a id="0x4_collection_count"></a>

## Function `count`

Provides the count of the current selection if supply tracking is used

Note: Calling this method from transaction that also mints/burns, prevents
it from being parallelized.


```move
module 0x4::collection {
    #[view]
    public fun count<T: key>(collection: object::Object<T>): option::Option<u64>
}
```


##### Implementation


```move
module 0x4::collection {
    public fun count<T: key>(
        collection: Object<T>
    ): Option<u64> acquires FixedSupply, UnlimitedSupply, ConcurrentSupply {
        let collection_address = object::object_address(&collection);
        check_collection_exists(collection_address);

        if (exists<ConcurrentSupply>(collection_address)) {
            let supply = borrow_global_mut<ConcurrentSupply>(collection_address);
            option::some(aggregator_v2::read(&supply.current_supply))
        } else if (exists<FixedSupply>(collection_address)) {
            let supply = borrow_global_mut<FixedSupply>(collection_address);
            option::some(supply.current_supply)
        } else if (exists<UnlimitedSupply>(collection_address)) {
            let supply = borrow_global_mut<UnlimitedSupply>(collection_address);
            option::some(supply.current_supply)
        } else {
            option::none()
        }
    }
}
```


<a id="0x4_collection_creator"></a>

## Function `creator`



```move
module 0x4::collection {
    #[view]
    public fun creator<T: key>(collection: object::Object<T>): address
}
```


##### Implementation


```move
module 0x4::collection {
    public fun creator<T: key>(collection: Object<T>): address acquires Collection {
        borrow(&collection).creator
    }
}
```


<a id="0x4_collection_description"></a>

## Function `description`



```move
module 0x4::collection {
    #[view]
    public fun description<T: key>(collection: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::collection {
    public fun description<T: key>(collection: Object<T>): String acquires Collection {
        borrow(&collection).description
    }
}
```


<a id="0x4_collection_name"></a>

## Function `name`



```move
module 0x4::collection {
    #[view]
    public fun name<T: key>(collection: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::collection {
    public fun name<T: key>(collection: Object<T>): String acquires Collection {
        borrow(&collection).name
    }
}
```


<a id="0x4_collection_uri"></a>

## Function `uri`



```move
module 0x4::collection {
    #[view]
    public fun uri<T: key>(collection: object::Object<T>): string::String
}
```


##### Implementation


```move
module 0x4::collection {
    public fun uri<T: key>(collection: Object<T>): String acquires Collection {
        borrow(&collection).uri
    }
}
```


<a id="0x4_collection_borrow_mut"></a>

## Function `borrow_mut`



```move
module 0x4::collection {
    fun borrow_mut(mutator_ref: &collection::MutatorRef): &mut collection::Collection
}
```


##### Implementation


```move
module 0x4::collection {
    inline fun borrow_mut(mutator_ref: &MutatorRef): &mut Collection {
        check_collection_exists(mutator_ref.self);
        borrow_global_mut<Collection>(mutator_ref.self)
    }
}
```


<a id="0x4_collection_set_name"></a>

## Function `set_name`

Callers of this function must be aware that changing the name will change the calculated
collection&apos;s address when calling `create_collection_address`.
Once the collection has been created, the collection address should be saved for reference and
`create_collection_address` should not be used to derive the collection&apos;s address.

After changing the collection&apos;s name, to create tokens &#45; only call functions that accept the collection object as an argument.


```move
module 0x4::collection {
    public fun set_name(mutator_ref: &collection::MutatorRef, name: string::String)
}
```


##### Implementation


```move
module 0x4::collection {
    public fun set_name(mutator_ref: &MutatorRef, name: String) acquires Collection {
        assert!(string::length(&name) <= MAX_COLLECTION_NAME_LENGTH, error::out_of_range(ECOLLECTION_NAME_TOO_LONG));
        let collection = borrow_mut(mutator_ref);
        event::emit(Mutation {
            mutated_field_name: string::utf8(b"name") ,
            collection: object::address_to_object(mutator_ref.self),
            old_value: collection.name,
            new_value: name,
        });
        collection.name = name;
    }
}
```


<a id="0x4_collection_set_description"></a>

## Function `set_description`



```move
module 0x4::collection {
    public fun set_description(mutator_ref: &collection::MutatorRef, description: string::String)
}
```


##### Implementation


```move
module 0x4::collection {
    public fun set_description(mutator_ref: &MutatorRef, description: String) acquires Collection {
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));
        let collection = borrow_mut(mutator_ref);
        if (std::features::module_event_migration_enabled()) {
            event::emit(Mutation {
                mutated_field_name: string::utf8(b"description"),
                collection: object::address_to_object(mutator_ref.self),
                old_value: collection.description,
                new_value: description,
            });
        };
        collection.description = description;
        event::emit_event(
            &mut collection.mutation_events,
            MutationEvent { mutated_field_name: string::utf8(b"description") },
        );
    }
}
```


<a id="0x4_collection_set_uri"></a>

## Function `set_uri`



```move
module 0x4::collection {
    public fun set_uri(mutator_ref: &collection::MutatorRef, uri: string::String)
}
```


##### Implementation


```move
module 0x4::collection {
    public fun set_uri(mutator_ref: &MutatorRef, uri: String) acquires Collection {
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        let collection = borrow_mut(mutator_ref);
        if (std::features::module_event_migration_enabled()) {
            event::emit(Mutation {
                mutated_field_name: string::utf8(b"uri"),
                collection: object::address_to_object(mutator_ref.self),
                old_value: collection.uri,
                new_value: uri,
            });
        };
        collection.uri = uri;
        event::emit_event(
            &mut collection.mutation_events,
            MutationEvent { mutated_field_name: string::utf8(b"uri") },
        );
    }
}
```


<a id="0x4_collection_set_max_supply"></a>

## Function `set_max_supply`



```move
module 0x4::collection {
    public fun set_max_supply(mutator_ref: &collection::MutatorRef, max_supply: u64)
}
```


##### Implementation


```move
module 0x4::collection {
    public fun set_max_supply(mutator_ref: &MutatorRef, max_supply: u64) acquires ConcurrentSupply, FixedSupply {
        let collection = object::address_to_object<Collection>(mutator_ref.self);
        let collection_address = object::object_address(&collection);
        let old_max_supply;

        if (exists<ConcurrentSupply>(collection_address)) {
            let supply = borrow_global_mut<ConcurrentSupply>(collection_address);
            let current_supply = aggregator_v2::read(&supply.current_supply);
            assert!(
                max_supply >= current_supply,
                error::out_of_range(EINVALID_MAX_SUPPLY),
            );
            old_max_supply = aggregator_v2::max_value(&supply.current_supply);
            supply.current_supply = aggregator_v2::create_aggregator(max_supply);
            aggregator_v2::add(&mut supply.current_supply, current_supply);
        } else if (exists<FixedSupply>(collection_address)) {
            let supply = borrow_global_mut<FixedSupply>(collection_address);
            assert!(
                max_supply >= supply.current_supply,
                error::out_of_range(EINVALID_MAX_SUPPLY),
            );
            old_max_supply = supply.max_supply;
            supply.max_supply = max_supply;
        } else {
            abort error::invalid_argument(ENO_MAX_SUPPLY_IN_COLLECTION)
        };

        event::emit(SetMaxSupply { collection, old_max_supply, new_max_supply: max_supply });
    }
}
```
