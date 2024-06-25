
<a id="0x3_token_event_store"></a>

# Module `0x3::token_event_store`

This module provides utils to add and emit new token events that are not in token.move


-  [Struct `CollectionDescriptionMutateEvent`](#0x3_token_event_store_CollectionDescriptionMutateEvent)
-  [Struct `CollectionDescriptionMutate`](#0x3_token_event_store_CollectionDescriptionMutate)
-  [Struct `CollectionUriMutateEvent`](#0x3_token_event_store_CollectionUriMutateEvent)
-  [Struct `CollectionUriMutate`](#0x3_token_event_store_CollectionUriMutate)
-  [Struct `CollectionMaxiumMutateEvent`](#0x3_token_event_store_CollectionMaxiumMutateEvent)
-  [Struct `CollectionMaxiumMutate`](#0x3_token_event_store_CollectionMaxiumMutate)
-  [Struct `OptInTransferEvent`](#0x3_token_event_store_OptInTransferEvent)
-  [Struct `OptInTransfer`](#0x3_token_event_store_OptInTransfer)
-  [Struct `UriMutationEvent`](#0x3_token_event_store_UriMutationEvent)
-  [Struct `UriMutation`](#0x3_token_event_store_UriMutation)
-  [Struct `DefaultPropertyMutateEvent`](#0x3_token_event_store_DefaultPropertyMutateEvent)
-  [Struct `DefaultPropertyMutate`](#0x3_token_event_store_DefaultPropertyMutate)
-  [Struct `DescriptionMutateEvent`](#0x3_token_event_store_DescriptionMutateEvent)
-  [Struct `DescriptionMutate`](#0x3_token_event_store_DescriptionMutate)
-  [Struct `RoyaltyMutateEvent`](#0x3_token_event_store_RoyaltyMutateEvent)
-  [Struct `RoyaltyMutate`](#0x3_token_event_store_RoyaltyMutate)
-  [Struct `MaxiumMutateEvent`](#0x3_token_event_store_MaxiumMutateEvent)
-  [Struct `MaximumMutate`](#0x3_token_event_store_MaximumMutate)
-  [Resource `TokenEventStoreV1`](#0x3_token_event_store_TokenEventStoreV1)
-  [Function `initialize_token_event_store`](#0x3_token_event_store_initialize_token_event_store)
-  [Function `emit_collection_uri_mutate_event`](#0x3_token_event_store_emit_collection_uri_mutate_event)
-  [Function `emit_collection_description_mutate_event`](#0x3_token_event_store_emit_collection_description_mutate_event)
-  [Function `emit_collection_maximum_mutate_event`](#0x3_token_event_store_emit_collection_maximum_mutate_event)
-  [Function `emit_token_opt_in_event`](#0x3_token_event_store_emit_token_opt_in_event)
-  [Function `emit_token_uri_mutate_event`](#0x3_token_event_store_emit_token_uri_mutate_event)
-  [Function `emit_default_property_mutate_event`](#0x3_token_event_store_emit_default_property_mutate_event)
-  [Function `emit_token_descrition_mutate_event`](#0x3_token_event_store_emit_token_descrition_mutate_event)
-  [Function `emit_token_royalty_mutate_event`](#0x3_token_event_store_emit_token_royalty_mutate_event)
-  [Function `emit_token_maximum_mutate_event`](#0x3_token_event_store_emit_token_maximum_mutate_event)
-  [Specification](#@Specification_0)
    -  [Function `initialize_token_event_store`](#@Specification_0_initialize_token_event_store)
    -  [Function `emit_collection_uri_mutate_event`](#@Specification_0_emit_collection_uri_mutate_event)
    -  [Function `emit_collection_description_mutate_event`](#@Specification_0_emit_collection_description_mutate_event)
    -  [Function `emit_collection_maximum_mutate_event`](#@Specification_0_emit_collection_maximum_mutate_event)
    -  [Function `emit_token_opt_in_event`](#@Specification_0_emit_token_opt_in_event)
    -  [Function `emit_token_uri_mutate_event`](#@Specification_0_emit_token_uri_mutate_event)
    -  [Function `emit_default_property_mutate_event`](#@Specification_0_emit_default_property_mutate_event)
    -  [Function `emit_token_descrition_mutate_event`](#@Specification_0_emit_token_descrition_mutate_event)
    -  [Function `emit_token_royalty_mutate_event`](#@Specification_0_emit_token_royalty_mutate_event)
    -  [Function `emit_token_maximum_mutate_event`](#@Specification_0_emit_token_maximum_mutate_event)


```move
module 0x3::token_event_store {
    use 0x1::account;
    use 0x1::any;
    use 0x1::event;
    use 0x1::features;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x3::property_map;
}
```


<a id="0x3_token_event_store_CollectionDescriptionMutateEvent"></a>

## Struct `CollectionDescriptionMutateEvent`

Event emitted when collection description is mutated


```move
module 0x3::token_event_store {
    struct CollectionDescriptionMutateEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_description: string::String`
</dt>
<dd>

</dd>
<dt>
`new_description: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_CollectionDescriptionMutate"></a>

## Struct `CollectionDescriptionMutate`

Event emitted when collection description is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct CollectionDescriptionMutate has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_description: string::String`
</dt>
<dd>

</dd>
<dt>
`new_description: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_CollectionUriMutateEvent"></a>

## Struct `CollectionUriMutateEvent`

Event emitted when collection uri is mutated


```move
module 0x3::token_event_store {
    struct CollectionUriMutateEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_uri: string::String`
</dt>
<dd>

</dd>
<dt>
`new_uri: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_CollectionUriMutate"></a>

## Struct `CollectionUriMutate`

Event emitted when collection uri is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct CollectionUriMutate has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_uri: string::String`
</dt>
<dd>

</dd>
<dt>
`new_uri: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_CollectionMaxiumMutateEvent"></a>

## Struct `CollectionMaxiumMutateEvent`

Event emitted when the collection maximum is mutated


```move
module 0x3::token_event_store {
    struct CollectionMaxiumMutateEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_maximum: u64`
</dt>
<dd>

</dd>
<dt>
`new_maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_CollectionMaxiumMutate"></a>

## Struct `CollectionMaxiumMutate`

Event emitted when the collection maximum is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct CollectionMaxiumMutate has drop, store
}
```


##### Fields


<dl>
<dt>
`creator_addr: address`
</dt>
<dd>

</dd>
<dt>
`collection_name: string::String`
</dt>
<dd>

</dd>
<dt>
`old_maximum: u64`
</dt>
<dd>

</dd>
<dt>
`new_maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_OptInTransferEvent"></a>

## Struct `OptInTransferEvent`

Event emitted when an user opt&#45;in the direct transfer


```move
module 0x3::token_event_store {
    struct OptInTransferEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`opt_in: bool`
</dt>
<dd>
 True if the user opt in, false if the user opt&#45;out
</dd>
</dl>


<a id="0x3_token_event_store_OptInTransfer"></a>

## Struct `OptInTransfer`

Event emitted when an user opt&#45;in the direct transfer


```move
module 0x3::token_event_store {
    #[event]
    struct OptInTransfer has drop, store
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
`opt_in: bool`
</dt>
<dd>
 True if the user opt in, false if the user opt&#45;out
</dd>
</dl>


<a id="0x3_token_event_store_UriMutationEvent"></a>

## Struct `UriMutationEvent`

Event emitted when the tokendata uri mutates


```move
module 0x3::token_event_store {
    struct UriMutationEvent has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_uri: string::String`
</dt>
<dd>

</dd>
<dt>
`new_uri: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_UriMutation"></a>

## Struct `UriMutation`

Event emitted when the tokendata uri mutates


```move
module 0x3::token_event_store {
    #[event]
    struct UriMutation has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_uri: string::String`
</dt>
<dd>

</dd>
<dt>
`new_uri: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_DefaultPropertyMutateEvent"></a>

## Struct `DefaultPropertyMutateEvent`

Event emitted when mutating the default the token properties stored at tokendata


```move
module 0x3::token_event_store {
    struct DefaultPropertyMutateEvent has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`old_values: vector<option::Option<property_map::PropertyValue>>`
</dt>
<dd>
 we allow upsert so the old values might be none
</dd>
<dt>
`new_values: vector<property_map::PropertyValue>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_DefaultPropertyMutate"></a>

## Struct `DefaultPropertyMutate`

Event emitted when mutating the default the token properties stored at tokendata


```move
module 0x3::token_event_store {
    #[event]
    struct DefaultPropertyMutate has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`keys: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`old_values: vector<option::Option<property_map::PropertyValue>>`
</dt>
<dd>
 we allow upsert so the old values might be none
</dd>
<dt>
`new_values: vector<property_map::PropertyValue>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_DescriptionMutateEvent"></a>

## Struct `DescriptionMutateEvent`

Event emitted when the tokendata description is mutated


```move
module 0x3::token_event_store {
    struct DescriptionMutateEvent has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_description: string::String`
</dt>
<dd>

</dd>
<dt>
`new_description: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_DescriptionMutate"></a>

## Struct `DescriptionMutate`

Event emitted when the tokendata description is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct DescriptionMutate has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_description: string::String`
</dt>
<dd>

</dd>
<dt>
`new_description: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_RoyaltyMutateEvent"></a>

## Struct `RoyaltyMutateEvent`

Event emitted when the token royalty is mutated


```move
module 0x3::token_event_store {
    struct RoyaltyMutateEvent has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_royalty_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`old_royalty_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`old_royalty_payee_addr: address`
</dt>
<dd>

</dd>
<dt>
`new_royalty_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`new_royalty_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`new_royalty_payee_addr: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_RoyaltyMutate"></a>

## Struct `RoyaltyMutate`

Event emitted when the token royalty is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct RoyaltyMutate has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_royalty_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`old_royalty_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`old_royalty_payee_addr: address`
</dt>
<dd>

</dd>
<dt>
`new_royalty_numerator: u64`
</dt>
<dd>

</dd>
<dt>
`new_royalty_denominator: u64`
</dt>
<dd>

</dd>
<dt>
`new_royalty_payee_addr: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_MaxiumMutateEvent"></a>

## Struct `MaxiumMutateEvent`

Event emitted when the token maximum is mutated


```move
module 0x3::token_event_store {
    struct MaxiumMutateEvent has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_maximum: u64`
</dt>
<dd>

</dd>
<dt>
`new_maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_MaximumMutate"></a>

## Struct `MaximumMutate`

Event emitted when the token maximum is mutated


```move
module 0x3::token_event_store {
    #[event]
    struct MaximumMutate has drop, store
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
`collection: string::String`
</dt>
<dd>

</dd>
<dt>
`token: string::String`
</dt>
<dd>

</dd>
<dt>
`old_maximum: u64`
</dt>
<dd>

</dd>
<dt>
`new_maximum: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_event_store_TokenEventStoreV1"></a>

## Resource `TokenEventStoreV1`



```move
module 0x3::token_event_store {
    struct TokenEventStoreV1 has key
}
```


##### Fields


<dl>
<dt>
`collection_uri_mutate_events: event::EventHandle<token_event_store::CollectionUriMutateEvent>`
</dt>
<dd>
 collection mutation events
</dd>
<dt>
`collection_maximum_mutate_events: event::EventHandle<token_event_store::CollectionMaxiumMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`collection_description_mutate_events: event::EventHandle<token_event_store::CollectionDescriptionMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`opt_in_events: event::EventHandle<token_event_store::OptInTransferEvent>`
</dt>
<dd>
 token transfer opt&#45;in event
</dd>
<dt>
`uri_mutate_events: event::EventHandle<token_event_store::UriMutationEvent>`
</dt>
<dd>
 token mutation events
</dd>
<dt>
`default_property_mutate_events: event::EventHandle<token_event_store::DefaultPropertyMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`description_mutate_events: event::EventHandle<token_event_store::DescriptionMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`royalty_mutate_events: event::EventHandle<token_event_store::RoyaltyMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`maximum_mutate_events: event::EventHandle<token_event_store::MaxiumMutateEvent>`
</dt>
<dd>

</dd>
<dt>
`extension: option::Option<any::Any>`
</dt>
<dd>
 This is for adding new events in future
</dd>
</dl>


<a id="0x3_token_event_store_initialize_token_event_store"></a>

## Function `initialize_token_event_store`



```move
module 0x3::token_event_store {
    fun initialize_token_event_store(acct: &signer)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    fun initialize_token_event_store(acct: &signer){
        if (!exists<TokenEventStoreV1>(signer::address_of(acct))) {
            move_to(acct, TokenEventStoreV1 {
                collection_uri_mutate_events: account::new_event_handle<CollectionUriMutateEvent>(acct),
                collection_maximum_mutate_events: account::new_event_handle<CollectionMaxiumMutateEvent>(acct),
                collection_description_mutate_events: account::new_event_handle<CollectionDescriptionMutateEvent>(acct),
                opt_in_events: account::new_event_handle<OptInTransferEvent>(acct),
                uri_mutate_events: account::new_event_handle<UriMutationEvent>(acct),
                default_property_mutate_events: account::new_event_handle<DefaultPropertyMutateEvent>(acct),
                description_mutate_events: account::new_event_handle<DescriptionMutateEvent>(acct),
                royalty_mutate_events: account::new_event_handle<RoyaltyMutateEvent>(acct),
                maximum_mutate_events: account::new_event_handle<MaxiumMutateEvent>(acct),
                extension: option::none<Any>(),
            });
        };
    }
}
```


<a id="0x3_token_event_store_emit_collection_uri_mutate_event"></a>

## Function `emit_collection_uri_mutate_event`

Emit the collection uri mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_uri_mutate_event(creator: &signer, collection: string::String, old_uri: string::String, new_uri: string::String)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_uri_mutate_event(creator: &signer, collection: String, old_uri: String, new_uri: String) acquires TokenEventStoreV1 {
        let event = CollectionUriMutateEvent {
            creator_addr: signer::address_of(creator),
            collection_name: collection,
            old_uri,
            new_uri,
        };
        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(signer::address_of(creator));
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CollectionUriMutate {
                    creator_addr: signer::address_of(creator),
                    collection_name: collection,
                    old_uri,
                    new_uri,
                }
            );
        };
        event::emit_event<CollectionUriMutateEvent>(
            &mut token_event_store.collection_uri_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_collection_description_mutate_event"></a>

## Function `emit_collection_description_mutate_event`

Emit the collection description mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_description_mutate_event(creator: &signer, collection: string::String, old_description: string::String, new_description: string::String)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_description_mutate_event(creator: &signer, collection: String, old_description: String, new_description: String) acquires TokenEventStoreV1 {
        let event = CollectionDescriptionMutateEvent {
            creator_addr: signer::address_of(creator),
            collection_name: collection,
            old_description,
            new_description,
        };
        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(signer::address_of(creator));
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CollectionDescriptionMutate {
                    creator_addr: signer::address_of(creator),
                    collection_name: collection,
                    old_description,
                    new_description,
                }
            );
        };
        event::emit_event<CollectionDescriptionMutateEvent>(
            &mut token_event_store.collection_description_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_collection_maximum_mutate_event"></a>

## Function `emit_collection_maximum_mutate_event`

Emit the collection maximum mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_maximum_mutate_event(creator: &signer, collection: string::String, old_maximum: u64, new_maximum: u64)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_maximum_mutate_event(creator: &signer, collection: String, old_maximum: u64, new_maximum: u64) acquires TokenEventStoreV1 {
        let event = CollectionMaxiumMutateEvent {
            creator_addr: signer::address_of(creator),
            collection_name: collection,
            old_maximum,
            new_maximum,
        };
        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(signer::address_of(creator));
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                CollectionMaxiumMutate {
                    creator_addr: signer::address_of(creator),
                    collection_name: collection,
                    old_maximum,
                    new_maximum,
                }
            );
        };
        event::emit_event<CollectionMaxiumMutateEvent>(
            &mut token_event_store.collection_maximum_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_token_opt_in_event"></a>

## Function `emit_token_opt_in_event`

Emit the direct opt&#45;in event


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_opt_in_event(account: &signer, opt_in: bool)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_opt_in_event(account: &signer, opt_in: bool) acquires TokenEventStoreV1 {
        let opt_in_event = OptInTransferEvent {
          opt_in,
        };
        initialize_token_event_store(account);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(signer::address_of(account));
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                OptInTransfer {
                    account_address: signer::address_of(account),
                    opt_in,
                });
        };
        event::emit_event<OptInTransferEvent>(
            &mut token_event_store.opt_in_events,
            opt_in_event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_token_uri_mutate_event"></a>

## Function `emit_token_uri_mutate_event`

Emit URI mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_uri_mutate_event(creator: &signer, collection: string::String, token: string::String, old_uri: string::String, new_uri: string::String)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_uri_mutate_event(
        creator: &signer,
        collection: String,
        token: String,
        old_uri: String,
        new_uri: String,
    ) acquires TokenEventStoreV1 {
        let creator_addr = signer::address_of(creator);

        let event = UriMutationEvent {
            creator: creator_addr,
            collection,
            token,
            old_uri,
            new_uri,
        };

        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(creator_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                UriMutation {
                    creator: creator_addr,
                    collection,
                    token,
                    old_uri,
                    new_uri,
                });
        };
        event::emit_event<UriMutationEvent>(
            &mut token_event_store.uri_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_default_property_mutate_event"></a>

## Function `emit_default_property_mutate_event`

Emit tokendata property map mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_default_property_mutate_event(creator: &signer, collection: string::String, token: string::String, keys: vector<string::String>, old_values: vector<option::Option<property_map::PropertyValue>>, new_values: vector<property_map::PropertyValue>)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_default_property_mutate_event(
        creator: &signer,
        collection: String,
        token: String,
        keys: vector<String>,
        old_values: vector<Option<PropertyValue>>,
        new_values: vector<PropertyValue>,
    ) acquires TokenEventStoreV1 {
        let creator_addr = signer::address_of(creator);

        let event = DefaultPropertyMutateEvent {
            creator: creator_addr,
            collection,
            token,
            keys,
            old_values,
            new_values,
        };

        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(creator_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                DefaultPropertyMutate {
                    creator: creator_addr,
                    collection,
                    token,
                    keys,
                    old_values,
                    new_values,
                });
        };
        event::emit_event<DefaultPropertyMutateEvent>(
            &mut token_event_store.default_property_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_token_descrition_mutate_event"></a>

## Function `emit_token_descrition_mutate_event`

Emit description mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_descrition_mutate_event(creator: &signer, collection: string::String, token: string::String, old_description: string::String, new_description: string::String)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_descrition_mutate_event(
        creator: &signer,
        collection: String,
        token: String,
        old_description: String,
        new_description: String,
    ) acquires TokenEventStoreV1 {
        let creator_addr = signer::address_of(creator);

        let event = DescriptionMutateEvent {
            creator: creator_addr,
            collection,
            token,
            old_description,
            new_description,
        };

        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(creator_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                DescriptionMutate {
                    creator: creator_addr,
                    collection,
                    token,
                    old_description,
                    new_description,
                });
        };
        event::emit_event<DescriptionMutateEvent>(
            &mut token_event_store.description_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_token_royalty_mutate_event"></a>

## Function `emit_token_royalty_mutate_event`

Emit royalty mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_royalty_mutate_event(creator: &signer, collection: string::String, token: string::String, old_royalty_numerator: u64, old_royalty_denominator: u64, old_royalty_payee_addr: address, new_royalty_numerator: u64, new_royalty_denominator: u64, new_royalty_payee_addr: address)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_royalty_mutate_event(
        creator: &signer,
        collection: String,
        token: String,
        old_royalty_numerator: u64,
        old_royalty_denominator: u64,
        old_royalty_payee_addr: address,
        new_royalty_numerator: u64,
        new_royalty_denominator: u64,
        new_royalty_payee_addr: address,
    ) acquires TokenEventStoreV1 {
        let creator_addr = signer::address_of(creator);
        let event = RoyaltyMutateEvent {
            creator: creator_addr,
            collection,
            token,
            old_royalty_numerator,
            old_royalty_denominator,
            old_royalty_payee_addr,
            new_royalty_numerator,
            new_royalty_denominator,
            new_royalty_payee_addr,
        };

        initialize_token_event_store(creator);
        let token_event_store = borrow_global_mut<TokenEventStoreV1>(creator_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                RoyaltyMutate {
                    creator: creator_addr,
                    collection,
                    token,
                    old_royalty_numerator,
                    old_royalty_denominator,
                    old_royalty_payee_addr,
                    new_royalty_numerator,
                    new_royalty_denominator,
                    new_royalty_payee_addr,
                });
        };
        event::emit_event<RoyaltyMutateEvent>(
            &mut token_event_store.royalty_mutate_events,
            event,
        );
    }
}
```


<a id="0x3_token_event_store_emit_token_maximum_mutate_event"></a>

## Function `emit_token_maximum_mutate_event`

Emit maximum mutation event


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_maximum_mutate_event(creator: &signer, collection: string::String, token: string::String, old_maximum: u64, new_maximum: u64)
}
```


##### Implementation


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_maximum_mutate_event(
        creator: &signer,
        collection: String,
        token: String,
        old_maximum: u64,
        new_maximum: u64,
    ) acquires TokenEventStoreV1 {
        let creator_addr = signer::address_of(creator);

        let event = MaxiumMutateEvent {
            creator: creator_addr,
            collection,
            token,
            old_maximum,
            new_maximum,
        };

        initialize_token_event_store(creator);
        let token_event_store =  borrow_global_mut<TokenEventStoreV1>(creator_addr);
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                MaximumMutate {
                    creator: creator_addr,
                    collection,
                    token,
                    old_maximum,
                    new_maximum,
                });
        };
        event::emit_event<MaxiumMutateEvent>(
            &mut token_event_store.maximum_mutate_events,
            event,
        );
    }
}
```


<a id="@Specification_0"></a>

## Specification



```move
module 0x3::token_event_store {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_0_initialize_token_event_store"></a>

### Function `initialize_token_event_store`


```move
module 0x3::token_event_store {
    fun initialize_token_event_store(acct: &signer)
}
```



```move
module 0x3::token_event_store {
    pragma verify = true;
    let addr = signer::address_of(acct);
    include InitializeTokenEventStoreAbortsIf {creator : acct};
}
```

Adjust the overflow value according to the
number of registered events


<a id="0x3_token_event_store_InitializeTokenEventStoreAbortsIf"></a>


```move
module 0x3::token_event_store {
    schema InitializeTokenEventStoreAbortsIf {
        creator: &signer;
        let addr = signer::address_of(creator);
        let account = global<Account>(addr);
        aborts_if !exists<TokenEventStoreV1>(addr) && !exists<Account>(addr);
        aborts_if !exists<TokenEventStoreV1>(addr) && account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
        aborts_if !exists<TokenEventStoreV1>(addr) && account.guid_creation_num + 9 > MAX_U64;
    }
}
```



<a id="0x3_token_event_store_TokenEventStoreAbortsIf"></a>


```move
module 0x3::token_event_store {
    schema TokenEventStoreAbortsIf {
        creator: &signer;
        let addr = signer::address_of(creator);
        let account = global<Account>(addr);
        aborts_if !exists<Account>(addr);
        aborts_if account.guid_creation_num + 9 >= account::MAX_GUID_CREATION_NUM;
        aborts_if account.guid_creation_num + 9 > MAX_U64;
    }
}
```


<a id="@Specification_0_emit_collection_uri_mutate_event"></a>

### Function `emit_collection_uri_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_uri_mutate_event(creator: &signer, collection: string::String, old_uri: string::String, new_uri: string::String)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_collection_description_mutate_event"></a>

### Function `emit_collection_description_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_description_mutate_event(creator: &signer, collection: string::String, old_description: string::String, new_description: string::String)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_collection_maximum_mutate_event"></a>

### Function `emit_collection_maximum_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_collection_maximum_mutate_event(creator: &signer, collection: string::String, old_maximum: u64, new_maximum: u64)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_token_opt_in_event"></a>

### Function `emit_token_opt_in_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_opt_in_event(account: &signer, opt_in: bool)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf {creator : account};
}
```


<a id="@Specification_0_emit_token_uri_mutate_event"></a>

### Function `emit_token_uri_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_uri_mutate_event(creator: &signer, collection: string::String, token: string::String, old_uri: string::String, new_uri: string::String)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_default_property_mutate_event"></a>

### Function `emit_default_property_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_default_property_mutate_event(creator: &signer, collection: string::String, token: string::String, keys: vector<string::String>, old_values: vector<option::Option<property_map::PropertyValue>>, new_values: vector<property_map::PropertyValue>)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_token_descrition_mutate_event"></a>

### Function `emit_token_descrition_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_descrition_mutate_event(creator: &signer, collection: string::String, token: string::String, old_description: string::String, new_description: string::String)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_token_royalty_mutate_event"></a>

### Function `emit_token_royalty_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_royalty_mutate_event(creator: &signer, collection: string::String, token: string::String, old_royalty_numerator: u64, old_royalty_denominator: u64, old_royalty_payee_addr: address, new_royalty_numerator: u64, new_royalty_denominator: u64, new_royalty_payee_addr: address)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```


<a id="@Specification_0_emit_token_maximum_mutate_event"></a>

### Function `emit_token_maximum_mutate_event`


```move
module 0x3::token_event_store {
    public(friend) fun emit_token_maximum_mutate_event(creator: &signer, collection: string::String, token: string::String, old_maximum: u64, new_maximum: u64)
}
```



```move
module 0x3::token_event_store {
    include InitializeTokenEventStoreAbortsIf;
}
```
