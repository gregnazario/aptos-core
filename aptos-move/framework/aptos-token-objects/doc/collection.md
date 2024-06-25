
<a id="0x4_collection"></a>

# Module `0x4::collection`

This defines an object-based Collection. A collection acts as a set organizer for a group of
tokens. This includes aspects such as a general description, project URI, name, and may contain
other useful generalizations across this set of tokens.

Being built upon objects enables collections to be relatively flexible. As core primitives it
supports:
* Common fields: name, uri, description, creator
* MutatorRef leaving mutability configuration to a higher level component
* Addressed by a global identifier of creator's address and collection name, thus collections
cannot be deleted as a restriction of the object model.
* Optional support for collection-wide royalties
* Optional support for tracking of supply with events on mint or burn

TODO:
* Consider supporting changing the name of the collection with the MutatorRef. This would
require adding the field original_name.
* Consider supporting changing the aspects of supply with the MutatorRef.
* Add aggregator support when added to framework


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


<pre><code><b>use</b> [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2](0x1::aggregator_v2);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/doc/event.md#0x1_event](0x1::event);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/features.md#0x1_features](0x1::features);
<b>use</b> [../../aptos-framework/doc/object.md#0x1_object](0x1::object);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](0x1::signer);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [royalty.md#0x4_royalty](0x4::royalty);
</code></pre>



<a id="0x4_collection_Collection"></a>

## Resource `Collection`

Represents the common fields for a collection.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [collection.md#0x4_collection_Collection](Collection) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
 The creator of this collection.
</dd>
<dt>
<code>description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 A brief description of the collection.
</dd>
<dt>
<code>name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 An optional categorization of similar token.
</dd>
<dt>
<code>uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
<dt>
<code>mutation_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[collection.md#0x4_collection_MutationEvent](collection::MutationEvent)&gt;</code>
</dt>
<dd>
 Emitted upon any mutation of the collection.
</dd>
</dl>


</details>

<a id="0x4_collection_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


<pre><code><b>struct</b> [collection.md#0x4_collection_MutatorRef](MutatorRef) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code><b>struct</b> [collection.md#0x4_collection_MutationEvent](MutationEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_Mutation"></a>

## Struct `Mutation`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [collection.md#0x4_collection_Mutation](Mutation) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>old_value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_FixedSupply"></a>

## Resource `FixedSupply`

Fixed supply tracker, this is useful for ensuring that a limited number of tokens are minted.
and adding events and supply tracking to a collection.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [collection.md#0x4_collection_FixedSupply](FixedSupply) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current_supply: u64</code>
</dt>
<dd>
 Total minted - total burned
</dd>
<dt>
<code>max_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_minted: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>burn_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[collection.md#0x4_collection_BurnEvent](collection::BurnEvent)&gt;</code>
</dt>
<dd>
 Emitted upon burning a Token.
</dd>
<dt>
<code>mint_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[collection.md#0x4_collection_MintEvent](collection::MintEvent)&gt;</code>
</dt>
<dd>
 Emitted upon minting an Token.
</dd>
</dl>


</details>

<a id="0x4_collection_UnlimitedSupply"></a>

## Resource `UnlimitedSupply`

Unlimited supply tracker, this is useful for adding events and supply tracking to a collection.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_minted: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>burn_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[collection.md#0x4_collection_BurnEvent](collection::BurnEvent)&gt;</code>
</dt>
<dd>
 Emitted upon burning a Token.
</dd>
<dt>
<code>mint_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[collection.md#0x4_collection_MintEvent](collection::MintEvent)&gt;</code>
</dt>
<dd>
 Emitted upon minting an Token.
</dd>
</dl>


</details>

<a id="0x4_collection_ConcurrentSupply"></a>

## Resource `ConcurrentSupply`

Supply tracker, useful for tracking amount of issued tokens.
If max_value is not set to U64_MAX, this ensures that a limited number of tokens are minted.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current_supply: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_Aggregator](aggregator_v2::Aggregator)&lt;u64&gt;</code>
</dt>
<dd>
 Total minted - total burned
</dd>
<dt>
<code>total_minted: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_Aggregator](aggregator_v2::Aggregator)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_BurnEvent"></a>

## Struct `BurnEvent`



<pre><code><b>struct</b> [collection.md#0x4_collection_BurnEvent](BurnEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_MintEvent"></a>

## Struct `MintEvent`



<pre><code><b>struct</b> [collection.md#0x4_collection_MintEvent](MintEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_Burn"></a>

## Struct `Burn`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [collection.md#0x4_collection_Burn](Burn) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>[collection.md#0x4_collection](collection): <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_Mint"></a>

## Struct `Mint`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [collection.md#0x4_collection_Mint](Mint) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>[collection.md#0x4_collection](collection): <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_ConcurrentBurnEvent"></a>

## Struct `ConcurrentBurnEvent`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
#[deprecated]
<b>struct</b> [collection.md#0x4_collection_ConcurrentBurnEvent](ConcurrentBurnEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>collection_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_ConcurrentMintEvent"></a>

## Struct `ConcurrentMintEvent`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
#[deprecated]
<b>struct</b> [collection.md#0x4_collection_ConcurrentMintEvent](ConcurrentMintEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>collection_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x4_token](token): <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_collection_SetMaxSupply"></a>

## Struct `SetMaxSupply`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [collection.md#0x4_collection_SetMaxSupply](SetMaxSupply) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>[collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>old_max_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_max_supply: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x4_collection_MAX_U64"></a>



<pre><code><b>const</b> [collection.md#0x4_collection_MAX_U64](MAX_U64): u64 = 18446744073709551615;
</code></pre>



<a id="0x4_collection_EURI_TOO_LONG"></a>

The URI is over the maximum length


<pre><code><b>const</b> [collection.md#0x4_collection_EURI_TOO_LONG](EURI_TOO_LONG): u64 = 4;
</code></pre>



<a id="0x4_collection_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> [collection.md#0x4_collection_MAX_URI_LENGTH](MAX_URI_LENGTH): u64 = 512;
</code></pre>



<a id="0x4_collection_EALREADY_CONCURRENT"></a>

Tried upgrading collection to concurrent, but collection is already concurrent


<pre><code><b>const</b> [collection.md#0x4_collection_EALREADY_CONCURRENT](EALREADY_CONCURRENT): u64 = 8;
</code></pre>



<a id="0x4_collection_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


<pre><code><b>const</b> [collection.md#0x4_collection_ECOLLECTION_DOES_NOT_EXIST](ECOLLECTION_DOES_NOT_EXIST): u64 = 1;
</code></pre>



<a id="0x4_collection_ECOLLECTION_NAME_TOO_LONG"></a>

The collection name is over the maximum length


<pre><code><b>const</b> [collection.md#0x4_collection_ECOLLECTION_NAME_TOO_LONG](ECOLLECTION_NAME_TOO_LONG): u64 = 3;
</code></pre>



<a id="0x4_collection_ECOLLECTION_SUPPLY_EXCEEDED"></a>

The collection has reached its supply and no more tokens can be minted, unless some are burned


<pre><code><b>const</b> [collection.md#0x4_collection_ECOLLECTION_SUPPLY_EXCEEDED](ECOLLECTION_SUPPLY_EXCEEDED): u64 = 2;
</code></pre>



<a id="0x4_collection_ECONCURRENT_NOT_ENABLED"></a>

Concurrent feature flag is not yet enabled, so the function cannot be performed


<pre><code><b>const</b> [collection.md#0x4_collection_ECONCURRENT_NOT_ENABLED](ECONCURRENT_NOT_ENABLED): u64 = 7;
</code></pre>



<a id="0x4_collection_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


<pre><code><b>const</b> [collection.md#0x4_collection_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG): u64 = 5;
</code></pre>



<a id="0x4_collection_EINVALID_MAX_SUPPLY"></a>

The new max supply cannot be less than the current supply


<pre><code><b>const</b> [collection.md#0x4_collection_EINVALID_MAX_SUPPLY](EINVALID_MAX_SUPPLY): u64 = 9;
</code></pre>



<a id="0x4_collection_EMAX_SUPPLY_CANNOT_BE_ZERO"></a>

The max supply must be positive


<pre><code><b>const</b> [collection.md#0x4_collection_EMAX_SUPPLY_CANNOT_BE_ZERO](EMAX_SUPPLY_CANNOT_BE_ZERO): u64 = 6;
</code></pre>



<a id="0x4_collection_ENO_MAX_SUPPLY_IN_COLLECTION"></a>

The collection does not have a max supply


<pre><code><b>const</b> [collection.md#0x4_collection_ENO_MAX_SUPPLY_IN_COLLECTION](ENO_MAX_SUPPLY_IN_COLLECTION): u64 = 10;
</code></pre>



<a id="0x4_collection_MAX_COLLECTION_NAME_LENGTH"></a>



<pre><code><b>const</b> [collection.md#0x4_collection_MAX_COLLECTION_NAME_LENGTH](MAX_COLLECTION_NAME_LENGTH): u64 = 128;
</code></pre>



<a id="0x4_collection_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> [collection.md#0x4_collection_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH): u64 = 2048;
</code></pre>



<a id="0x4_collection_create_fixed_collection"></a>

## Function `create_fixed_collection`

Creates a fixed-sized collection, or a collection that supports a fixed amount of tokens.
This is useful to create a guaranteed, limited supply on-chain digital asset. For example,
a collection 1111 vicious vipers. Note, creating restrictions such as upward limits results
in data structures that prevent Aptos from parallelizing mints of this collection type.
Beyond that, it adds supply tracking with events.


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_fixed_collection](create_fixed_collection)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), max_supply: u64, name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_fixed_collection](create_fixed_collection)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    description: String,
    max_supply: u64,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>assert</b>!(max_supply != 0, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([collection.md#0x4_collection_EMAX_SUPPLY_CANNOT_BE_ZERO](EMAX_SUPPLY_CANNOT_BE_ZERO)));
    <b>let</b> collection_seed = [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(&name);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, collection_seed);

    <b>let</b> supply = [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
        current_supply: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_aggregator](aggregator_v2::create_aggregator)(max_supply),
        total_minted: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
    };

    [collection.md#0x4_collection_create_collection_internal](create_collection_internal)(
        creator,
        constructor_ref,
        description,
        name,
        [royalty.md#0x4_royalty](royalty),
        uri,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(supply),
    )
}
</code></pre>



</details>

<a id="0x4_collection_create_unlimited_collection"></a>

## Function `create_unlimited_collection`

Creates an unlimited collection. This has support for supply tracking but does not limit
the supply of tokens.


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_unlimited_collection](create_unlimited_collection)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_unlimited_collection](create_unlimited_collection)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> collection_seed = [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(&name);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, collection_seed);

    <b>let</b> supply = [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
        current_supply: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
        total_minted: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
    };

    [collection.md#0x4_collection_create_collection_internal](create_collection_internal)(
        creator,
        constructor_ref,
        description,
        name,
        [royalty.md#0x4_royalty](royalty),
        uri,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(supply),
    )
}
</code></pre>



</details>

<a id="0x4_collection_create_untracked_collection"></a>

## Function `create_untracked_collection`

Creates an untracked collection, or a collection that supports an arbitrary amount of
tokens. This is useful for mass airdrops that fully leverage Aptos parallelization.
TODO: Hide this until we bring back meaningful way to enforce burns


<pre><code><b>fun</b> [collection.md#0x4_collection_create_untracked_collection](create_untracked_collection)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [collection.md#0x4_collection_create_untracked_collection](create_untracked_collection)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> collection_seed = [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(&name);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, collection_seed);

    [collection.md#0x4_collection_create_collection_internal](create_collection_internal)&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(
        creator,
        constructor_ref,
        description,
        name,
        [royalty.md#0x4_royalty](royalty),
        uri,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
    )
}
</code></pre>



</details>

<a id="0x4_collection_create_collection_internal"></a>

## Function `create_collection_internal`



<pre><code><b>fun</b> [collection.md#0x4_collection_create_collection_internal](create_collection_internal)&lt;Supply: key&gt;(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), constructor_ref: [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), supply: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;Supply&gt;): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [collection.md#0x4_collection_create_collection_internal](create_collection_internal)&lt;Supply: key&gt;(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    constructor_ref: ConstructorRef,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
    supply: Option&lt;Supply&gt;,
): ConstructorRef {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&name) &lt;= [collection.md#0x4_collection_MAX_COLLECTION_NAME_LENGTH](MAX_COLLECTION_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_ECOLLECTION_NAME_TOO_LONG](ECOLLECTION_NAME_TOO_LONG)));
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&uri) &lt;= [collection.md#0x4_collection_MAX_URI_LENGTH](MAX_URI_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EURI_TOO_LONG](EURI_TOO_LONG)));
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&description) &lt;= [collection.md#0x4_collection_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG)));

    <b>let</b> object_signer = [../../aptos-framework/doc/object.md#0x1_object_generate_signer](object::generate_signer)(&constructor_ref);

    <b>let</b> [collection.md#0x4_collection](collection) = [collection.md#0x4_collection_Collection](Collection) {
        creator: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
        description,
        name,
        uri,
        mutation_events: [../../aptos-framework/doc/object.md#0x1_object_new_event_handle](object::new_event_handle)(&object_signer),
    };
    <b>move_to</b>(&object_signer, [collection.md#0x4_collection](collection));

    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&supply)) {
        <b>move_to</b>(&object_signer, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_destroy_some](option::destroy_some)(supply))
    } <b>else</b> {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_destroy_none](option::destroy_none)(supply)
    };

    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&[royalty.md#0x4_royalty](royalty))) {
        [royalty.md#0x4_royalty_init](royalty::init)(&constructor_ref, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> [royalty.md#0x4_royalty](royalty)))
    };

    <b>let</b> transfer_ref = [../../aptos-framework/doc/object.md#0x1_object_generate_transfer_ref](object::generate_transfer_ref)(&constructor_ref);
    [../../aptos-framework/doc/object.md#0x1_object_disable_ungated_transfer](object::disable_ungated_transfer)(&transfer_ref);

    constructor_ref
}
</code></pre>



</details>

<a id="0x4_collection_create_collection_address"></a>

## Function `create_collection_address`

Generates the collections address based upon the creators address and the collection's name


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_collection_address](create_collection_address)(creator: &<b>address</b>, name: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_collection_address](create_collection_address)(creator: &<b>address</b>, name: &String): <b>address</b> {
    [../../aptos-framework/doc/object.md#0x1_object_create_object_address](object::create_object_address)(creator, [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(name))
}
</code></pre>



</details>

<a id="0x4_collection_create_collection_seed"></a>

## Function `create_collection_seed`

Named objects are derived from a seed, the collection's seed is its name.


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(name: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_create_collection_seed](create_collection_seed)(name: &String): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(name) &lt;= [collection.md#0x4_collection_MAX_COLLECTION_NAME_LENGTH](MAX_COLLECTION_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_ECOLLECTION_NAME_TOO_LONG](ECOLLECTION_NAME_TOO_LONG)));
    *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes](string::bytes)(name)
}
</code></pre>



</details>

<a id="0x4_collection_increment_supply"></a>

## Function `increment_supply`

Called by token on mint to increment supply if there's an appropriate Supply struct.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [collection.md#0x4_collection_increment_supply](increment_supply)([collection.md#0x4_collection](collection): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, [token.md#0x4_token](token): <b>address</b>): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;u64&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [collection.md#0x4_collection_increment_supply](increment_supply)(
    [collection.md#0x4_collection](collection): &Object&lt;[collection.md#0x4_collection_Collection](Collection)&gt;,
    [token.md#0x4_token](token): <b>address</b>,
): Option&lt;AggregatorSnapshot&lt;u64&gt;&gt; <b>acquires</b> [collection.md#0x4_collection_FixedSupply](FixedSupply), [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply), [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
    <b>let</b> collection_addr = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([collection.md#0x4_collection](collection));
    <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_addr);
        <b>assert</b>!(
            [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_try_add](aggregator_v2::try_add)(&<b>mut</b> supply.current_supply, 1),
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_ECOLLECTION_SUPPLY_EXCEEDED](ECOLLECTION_SUPPLY_EXCEEDED)),
        );
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_add](aggregator_v2::add)(&<b>mut</b> supply.total_minted, 1);
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [collection.md#0x4_collection_Mint](Mint) {
                [collection.md#0x4_collection](collection): collection_addr,
                index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_snapshot](aggregator_v2::snapshot)(&supply.total_minted),
                [token.md#0x4_token](token),
            },
        );
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)([../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_snapshot](aggregator_v2::snapshot)(&supply.total_minted))
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_addr);
        supply.current_supply = supply.current_supply + 1;
        supply.total_minted = supply.total_minted + 1;
        <b>assert</b>!(
            supply.current_supply &lt;= supply.max_supply,
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_ECOLLECTION_SUPPLY_EXCEEDED](ECOLLECTION_SUPPLY_EXCEEDED)),
        );
        <b>if</b> (std::features::module_event_migration_enabled()) {
            [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
                [collection.md#0x4_collection_Mint](Mint) {
                    [collection.md#0x4_collection](collection): collection_addr,
                    index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot](aggregator_v2::create_snapshot)(supply.total_minted),
                    [token.md#0x4_token](token),
                },
            );
        };
        [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(&<b>mut</b> supply.mint_events,
            [collection.md#0x4_collection_MintEvent](MintEvent) {
                index: supply.total_minted,
                [token.md#0x4_token](token),
            },
        );
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)([../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot](aggregator_v2::create_snapshot)&lt;u64&gt;(supply.total_minted))
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_addr);
        supply.current_supply = supply.current_supply + 1;
        supply.total_minted = supply.total_minted + 1;
        <b>if</b> (std::features::module_event_migration_enabled()) {
            [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
                [collection.md#0x4_collection_Mint](Mint) {
                    [collection.md#0x4_collection](collection): collection_addr,
                    index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot](aggregator_v2::create_snapshot)(supply.total_minted),
                    [token.md#0x4_token](token),
                },
            );
        };
        [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
            &<b>mut</b> supply.mint_events,
            [collection.md#0x4_collection_MintEvent](MintEvent) {
                index: supply.total_minted,
                [token.md#0x4_token](token),
            },
        );
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)([../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot](aggregator_v2::create_snapshot)&lt;u64&gt;(supply.total_minted))
    } <b>else</b> {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)()
    }
}
</code></pre>



</details>

<a id="0x4_collection_decrement_supply"></a>

## Function `decrement_supply`

Called by token on burn to decrement supply if there's an appropriate Supply struct.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [collection.md#0x4_collection_decrement_supply](decrement_supply)([collection.md#0x4_collection](collection): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, [token.md#0x4_token](token): <b>address</b>, index: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;, previous_owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [collection.md#0x4_collection_decrement_supply](decrement_supply)(
    [collection.md#0x4_collection](collection): &Object&lt;[collection.md#0x4_collection_Collection](Collection)&gt;,
    [token.md#0x4_token](token): <b>address</b>,
    index: Option&lt;u64&gt;,
    previous_owner: <b>address</b>,
) <b>acquires</b> [collection.md#0x4_collection_FixedSupply](FixedSupply), [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply), [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
    <b>let</b> collection_addr = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([collection.md#0x4_collection](collection));
    <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_addr);
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_sub](aggregator_v2::sub)(&<b>mut</b> supply.current_supply, 1);

        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [collection.md#0x4_collection_Burn](Burn) {
                [collection.md#0x4_collection](collection): collection_addr,
                index: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&index),
                [token.md#0x4_token](token),
                previous_owner,
            },
        );
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_addr);
        supply.current_supply = supply.current_supply - 1;
        <b>if</b> (std::features::module_event_migration_enabled()) {
            [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
                [collection.md#0x4_collection_Burn](Burn) {
                    [collection.md#0x4_collection](collection): collection_addr,
                    index: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&index),
                    [token.md#0x4_token](token),
                    previous_owner,
                },
            );
        };
        [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
            &<b>mut</b> supply.burn_events,
            [collection.md#0x4_collection_BurnEvent](BurnEvent) {
                index: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&index),
                [token.md#0x4_token](token),
            },
        );
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_addr);
        supply.current_supply = supply.current_supply - 1;
        <b>if</b> (std::features::module_event_migration_enabled()) {
            [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
                [collection.md#0x4_collection_Burn](Burn) {
                    [collection.md#0x4_collection](collection): collection_addr,
                    index: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&index),
                    [token.md#0x4_token](token),
                    previous_owner,
                },
            );
        };
        [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
            &<b>mut</b> supply.burn_events,
            [collection.md#0x4_collection_BurnEvent](BurnEvent) {
                index: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&index),
                [token.md#0x4_token](token),
            },
        );
    }
}
</code></pre>



</details>

<a id="0x4_collection_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_generate_mutator_ref](generate_mutator_ref)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)): [collection.md#0x4_collection_MutatorRef](collection::MutatorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_generate_mutator_ref](generate_mutator_ref)(ref: &ConstructorRef): [collection.md#0x4_collection_MutatorRef](MutatorRef) {
    <b>let</b> [../../aptos-framework/doc/object.md#0x1_object](object) = [../../aptos-framework/doc/object.md#0x1_object_object_from_constructor_ref](object::object_from_constructor_ref)&lt;[collection.md#0x4_collection_Collection](Collection)&gt;(ref);
    [collection.md#0x4_collection_MutatorRef](MutatorRef) { self: [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[../../aptos-framework/doc/object.md#0x1_object](object)) }
}
</code></pre>



</details>

<a id="0x4_collection_upgrade_to_concurrent"></a>

## Function `upgrade_to_concurrent`



<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_upgrade_to_concurrent](upgrade_to_concurrent)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ExtendRef](object::ExtendRef))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_upgrade_to_concurrent](upgrade_to_concurrent)(
    ref: &ExtendRef,
) <b>acquires</b> [collection.md#0x4_collection_FixedSupply](FixedSupply), [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply) {
    <b>let</b> metadata_object_address = [../../aptos-framework/doc/object.md#0x1_object_address_from_extend_ref](object::address_from_extend_ref)(ref);
    <b>let</b> metadata_object_signer = [../../aptos-framework/doc/object.md#0x1_object_generate_signer_for_extending](object::generate_signer_for_extending)(ref);

    <b>let</b> (supply, current_supply, total_minted, burn_events, mint_events) = <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(
        metadata_object_address
    )) {
        <b>let</b> [collection.md#0x4_collection_FixedSupply](FixedSupply) {
            current_supply,
            max_supply,
            total_minted,
            burn_events,
            mint_events,
        } = <b>move_from</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(metadata_object_address);

        <b>let</b> supply = [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
            current_supply: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_aggregator](aggregator_v2::create_aggregator)(max_supply),
            total_minted: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
        };
        (supply, current_supply, total_minted, burn_events, mint_events)
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(metadata_object_address)) {
        <b>let</b> [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply) {
            current_supply,
            total_minted,
            burn_events,
            mint_events,
        } = <b>move_from</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(metadata_object_address);

        <b>let</b> supply = [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
            current_supply: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
            total_minted: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator](aggregator_v2::create_unbounded_aggregator)(),
        };
        (supply, current_supply, total_minted, burn_events, mint_events)
    } <b>else</b> {
        // untracked [collection.md#0x4_collection](collection) is already concurrent, and other variants too.
        <b>abort</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([collection.md#0x4_collection_EALREADY_CONCURRENT](EALREADY_CONCURRENT))
    };

    // <b>update</b> current state:
    [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_add](aggregator_v2::add)(&<b>mut</b> supply.current_supply, current_supply);
    [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_add](aggregator_v2::add)(&<b>mut</b> supply.total_minted, total_minted);
    <b>move_to</b>(&metadata_object_signer, supply);

    [../../aptos-framework/doc/event.md#0x1_event_destroy_handle](event::destroy_handle)(burn_events);
    [../../aptos-framework/doc/event.md#0x1_event_destroy_handle](event::destroy_handle)(mint_events);
}
</code></pre>



</details>

<a id="0x4_collection_check_collection_exists"></a>

## Function `check_collection_exists`



<pre><code><b>fun</b> [collection.md#0x4_collection_check_collection_exists](check_collection_exists)(addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [collection.md#0x4_collection_check_collection_exists](check_collection_exists)(addr: <b>address</b>) {
    <b>assert</b>!(
        <b>exists</b>&lt;[collection.md#0x4_collection_Collection](Collection)&gt;(addr),
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([collection.md#0x4_collection_ECOLLECTION_DOES_NOT_EXIST](ECOLLECTION_DOES_NOT_EXIST)),
    );
}
</code></pre>



</details>

<a id="0x4_collection_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> [collection.md#0x4_collection_borrow](borrow)&lt;T: key&gt;([collection.md#0x4_collection](collection): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): &[collection.md#0x4_collection_Collection](collection::Collection)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [collection.md#0x4_collection_borrow](borrow)&lt;T: key&gt;([collection.md#0x4_collection](collection): &Object&lt;T&gt;): &[collection.md#0x4_collection_Collection](Collection) {
    <b>let</b> collection_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([collection.md#0x4_collection](collection));
    [collection.md#0x4_collection_check_collection_exists](check_collection_exists)(collection_address);
    <b>borrow_global</b>&lt;[collection.md#0x4_collection_Collection](Collection)&gt;(collection_address)
}
</code></pre>



</details>

<a id="0x4_collection_count"></a>

## Function `count`

Provides the count of the current selection if supply tracking is used

Note: Calling this method from transaction that also mints/burns, prevents
it from being parallelized.


<pre><code>#[view]
<b>public</b> <b>fun</b> [collection.md#0x4_collection_count](count)&lt;T: key&gt;([collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_count](count)&lt;T: key&gt;(
    [collection.md#0x4_collection](collection): Object&lt;T&gt;
): Option&lt;u64&gt; <b>acquires</b> [collection.md#0x4_collection_FixedSupply](FixedSupply), [collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply), [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply) {
    <b>let</b> collection_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[collection.md#0x4_collection](collection));
    [collection.md#0x4_collection_check_collection_exists](check_collection_exists)(collection_address);

    <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_address);
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)([../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read](aggregator_v2::read)(&supply.current_supply))
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_address);
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(supply.current_supply)
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_UnlimitedSupply](UnlimitedSupply)&gt;(collection_address);
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(supply.current_supply)
    } <b>else</b> {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)()
    }
}
</code></pre>



</details>

<a id="0x4_collection_creator"></a>

## Function `creator`



<pre><code>#[view]
<b>public</b> <b>fun</b> [collection.md#0x4_collection_creator](creator)&lt;T: key&gt;([collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_creator](creator)&lt;T: key&gt;([collection.md#0x4_collection](collection): Object&lt;T&gt;): <b>address</b> <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    [collection.md#0x4_collection_borrow](borrow)(&[collection.md#0x4_collection](collection)).creator
}
</code></pre>



</details>

<a id="0x4_collection_description"></a>

## Function `description`



<pre><code>#[view]
<b>public</b> <b>fun</b> [collection.md#0x4_collection_description](description)&lt;T: key&gt;([collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_description](description)&lt;T: key&gt;([collection.md#0x4_collection](collection): Object&lt;T&gt;): String <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    [collection.md#0x4_collection_borrow](borrow)(&[collection.md#0x4_collection](collection)).description
}
</code></pre>



</details>

<a id="0x4_collection_name"></a>

## Function `name`



<pre><code>#[view]
<b>public</b> <b>fun</b> [collection.md#0x4_collection_name](name)&lt;T: key&gt;([collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_name](name)&lt;T: key&gt;([collection.md#0x4_collection](collection): Object&lt;T&gt;): String <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    [collection.md#0x4_collection_borrow](borrow)(&[collection.md#0x4_collection](collection)).name
}
</code></pre>



</details>

<a id="0x4_collection_uri"></a>

## Function `uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> [collection.md#0x4_collection_uri](uri)&lt;T: key&gt;([collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_uri](uri)&lt;T: key&gt;([collection.md#0x4_collection](collection): Object&lt;T&gt;): String <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    [collection.md#0x4_collection_borrow](borrow)(&[collection.md#0x4_collection](collection)).uri
}
</code></pre>



</details>

<a id="0x4_collection_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> [collection.md#0x4_collection_borrow_mut](borrow_mut)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](collection::MutatorRef)): &<b>mut</b> [collection.md#0x4_collection_Collection](collection::Collection)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [collection.md#0x4_collection_borrow_mut](borrow_mut)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](MutatorRef)): &<b>mut</b> [collection.md#0x4_collection_Collection](Collection) {
    [collection.md#0x4_collection_check_collection_exists](check_collection_exists)(mutator_ref.self);
    <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_Collection](Collection)&gt;(mutator_ref.self)
}
</code></pre>



</details>

<a id="0x4_collection_set_name"></a>

## Function `set_name`

Callers of this function must be aware that changing the name will change the calculated
collection's address when calling <code>create_collection_address</code>.
Once the collection has been created, the collection address should be saved for reference and
<code>create_collection_address</code> should not be used to derive the collection's address.

After changing the collection's name, to create tokens - only call functions that accept the collection object as an argument.


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_name](set_name)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](collection::MutatorRef), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_name](set_name)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](MutatorRef), name: String) <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&name) &lt;= [collection.md#0x4_collection_MAX_COLLECTION_NAME_LENGTH](MAX_COLLECTION_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_ECOLLECTION_NAME_TOO_LONG](ECOLLECTION_NAME_TOO_LONG)));
    <b>let</b> [collection.md#0x4_collection](collection) = [collection.md#0x4_collection_borrow_mut](borrow_mut)(mutator_ref);
    [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([collection.md#0x4_collection_Mutation](Mutation) {
        mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"name") ,
        [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)(mutator_ref.self),
        old_value: [collection.md#0x4_collection](collection).name,
        new_value: name,
    });
    [collection.md#0x4_collection](collection).name = name;
}
</code></pre>



</details>

<a id="0x4_collection_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_description](set_description)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](collection::MutatorRef), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_description](set_description)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](MutatorRef), description: String) <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&description) &lt;= [collection.md#0x4_collection_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG)));
    <b>let</b> [collection.md#0x4_collection](collection) = [collection.md#0x4_collection_borrow_mut](borrow_mut)(mutator_ref);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([collection.md#0x4_collection_Mutation](Mutation) {
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"description"),
            [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)(mutator_ref.self),
            old_value: [collection.md#0x4_collection](collection).description,
            new_value: description,
        });
    };
    [collection.md#0x4_collection](collection).description = description;
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
        &<b>mut</b> [collection.md#0x4_collection](collection).mutation_events,
        [collection.md#0x4_collection_MutationEvent](MutationEvent) { mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"description") },
    );
}
</code></pre>



</details>

<a id="0x4_collection_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_uri](set_uri)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](collection::MutatorRef), uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_uri](set_uri)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](MutatorRef), uri: String) <b>acquires</b> [collection.md#0x4_collection_Collection](Collection) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&uri) &lt;= [collection.md#0x4_collection_MAX_URI_LENGTH](MAX_URI_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EURI_TOO_LONG](EURI_TOO_LONG)));
    <b>let</b> [collection.md#0x4_collection](collection) = [collection.md#0x4_collection_borrow_mut](borrow_mut)(mutator_ref);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([collection.md#0x4_collection_Mutation](Mutation) {
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"uri"),
            [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)(mutator_ref.self),
            old_value: [collection.md#0x4_collection](collection).uri,
            new_value: uri,
        });
    };
    [collection.md#0x4_collection](collection).uri = uri;
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
        &<b>mut</b> [collection.md#0x4_collection](collection).mutation_events,
        [collection.md#0x4_collection_MutationEvent](MutationEvent) { mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"uri") },
    );
}
</code></pre>



</details>

<a id="0x4_collection_set_max_supply"></a>

## Function `set_max_supply`



<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_max_supply](set_max_supply)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](collection::MutatorRef), max_supply: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [collection.md#0x4_collection_set_max_supply](set_max_supply)(mutator_ref: &[collection.md#0x4_collection_MutatorRef](MutatorRef), max_supply: u64) <b>acquires</b> [collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply), [collection.md#0x4_collection_FixedSupply](FixedSupply) {
    <b>let</b> [collection.md#0x4_collection](collection) = [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)&lt;[collection.md#0x4_collection_Collection](Collection)&gt;(mutator_ref.self);
    <b>let</b> collection_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[collection.md#0x4_collection](collection));
    <b>let</b> old_max_supply;

    <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_ConcurrentSupply](ConcurrentSupply)&gt;(collection_address);
        <b>let</b> current_supply = [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read](aggregator_v2::read)(&supply.current_supply);
        <b>assert</b>!(
            max_supply &gt;= current_supply,
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EINVALID_MAX_SUPPLY](EINVALID_MAX_SUPPLY)),
        );
        old_max_supply = [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_max_value](aggregator_v2::max_value)(&supply.current_supply);
        supply.current_supply = [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_aggregator](aggregator_v2::create_aggregator)(max_supply);
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_add](aggregator_v2::add)(&<b>mut</b> supply.current_supply, current_supply);
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;[collection.md#0x4_collection_FixedSupply](FixedSupply)&gt;(collection_address);
        <b>assert</b>!(
            max_supply &gt;= supply.current_supply,
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([collection.md#0x4_collection_EINVALID_MAX_SUPPLY](EINVALID_MAX_SUPPLY)),
        );
        old_max_supply = supply.max_supply;
        supply.max_supply = max_supply;
    } <b>else</b> {
        <b>abort</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([collection.md#0x4_collection_ENO_MAX_SUPPLY_IN_COLLECTION](ENO_MAX_SUPPLY_IN_COLLECTION))
    };

    [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([collection.md#0x4_collection_SetMaxSupply](SetMaxSupply) { [collection.md#0x4_collection](collection), old_max_supply, new_max_supply: max_supply });
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
