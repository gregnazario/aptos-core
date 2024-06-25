
<a id="0x4_token"></a>

# Module `0x4::token`

This defines an object-based Token. The key differentiating features from the Aptos standard
token are:
* Decoupled token ownership from token data.
* Explicit data model for token metadata via adjacent resources
* Extensible framework for tokens


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


<pre><code><b>use</b> [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2](0x1::aggregator_v2);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/doc/event.md#0x1_event](0x1::event);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/features.md#0x1_features](0x1::features);
<b>use</b> [../../aptos-framework/doc/object.md#0x1_object](0x1::object);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](0x1::signer);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
<b>use</b> [collection.md#0x4_collection](0x4::collection);
<b>use</b> [royalty.md#0x4_royalty](0x4::royalty);
</code></pre>



<a id="0x4_token_Token"></a>

## Resource `Token`

Represents the common fields to all tokens.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [token.md#0x4_token_Token](Token) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>[collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;</code>
</dt>
<dd>
 The collection from which this token resides.
</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>
 Deprecated in favor of <code>index</code> inside TokenIdentifiers.
 Was populated until concurrent_token_v2_enabled feature flag was enabled.

 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
<code>description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 A brief description of the token.
</dd>
<dt>
<code>name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 Deprecated in favor of <code>name</code> inside TokenIdentifiers.
 Was populated until concurrent_token_v2_enabled feature flag was enabled.

 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: "Aptos Animal #1234"
</dd>
<dt>
<code>uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
<dt>
<code>mutation_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token.md#0x4_token_MutationEvent](token::MutationEvent)&gt;</code>
</dt>
<dd>
 Emitted upon any mutation of the token.
</dd>
</dl>


</details>

<a id="0x4_token_TokenIdentifiers"></a>

## Resource `TokenIdentifiers`

Represents first addition to the common fields for all tokens
Started being populated once aggregator_v2_api_enabled was enabled.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;u64&gt;</code>
</dt>
<dd>
 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
<code>name: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_DerivedStringSnapshot](aggregator_v2::DerivedStringSnapshot)</code>
</dt>
<dd>
 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: "Aptos Animal #1234"
</dd>
</dl>


</details>

<a id="0x4_token_ConcurrentTokenIdentifiers"></a>

## Resource `ConcurrentTokenIdentifiers`



<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
#[deprecated]
<b>struct</b> [token.md#0x4_token_ConcurrentTokenIdentifiers](ConcurrentTokenIdentifiers) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>name: [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot](aggregator_v2::AggregatorSnapshot)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_token_BurnRef"></a>

## Struct `BurnRef`

This enables burning an NFT, if possible, it will also delete the object. Note, the data
in inner and self occupies 32-bytes each, rather than have both, this data structure makes
a small optimization to support either and take a fixed amount of 34-bytes.


<pre><code><b>struct</b> [token.md#0x4_token_BurnRef](BurnRef) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[../../aptos-framework/doc/object.md#0x1_object_DeleteRef](object::DeleteRef)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>self: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_token_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


<pre><code><b>struct</b> [token.md#0x4_token_MutatorRef](MutatorRef) <b>has</b> drop, store
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

<a id="0x4_token_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code><b>struct</b> [token.md#0x4_token_MutationEvent](MutationEvent) <b>has</b> drop, store
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

<a id="0x4_token_Mutation"></a>

## Struct `Mutation`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token.md#0x4_token_Mutation](Mutation) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>token_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
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

<a id="@Constants_0"></a>

## Constants


<a id="0x4_token_EURI_TOO_LONG"></a>

The URI is over the maximum length


<pre><code><b>const</b> [token.md#0x4_token_EURI_TOO_LONG](EURI_TOO_LONG): u64 = 5;
</code></pre>



<a id="0x4_token_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> [token.md#0x4_token_MAX_URI_LENGTH](MAX_URI_LENGTH): u64 = 512;
</code></pre>



<a id="0x4_token_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


<pre><code><b>const</b> [token.md#0x4_token_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG): u64 = 6;
</code></pre>



<a id="0x4_token_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> [token.md#0x4_token_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH): u64 = 2048;
</code></pre>



<a id="0x4_token_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


<pre><code><b>const</b> [token.md#0x4_token_EFIELD_NOT_MUTABLE](EFIELD_NOT_MUTABLE): u64 = 3;
</code></pre>



<a id="0x4_token_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> [token.md#0x4_token_ENOT_CREATOR](ENOT_CREATOR): u64 = 2;
</code></pre>



<a id="0x4_token_ESEED_TOO_LONG"></a>

The seed is over the maximum length


<pre><code><b>const</b> [token.md#0x4_token_ESEED_TOO_LONG](ESEED_TOO_LONG): u64 = 7;
</code></pre>



<a id="0x4_token_ETOKEN_DOES_NOT_EXIST"></a>

The token does not exist


<pre><code><b>const</b> [token.md#0x4_token_ETOKEN_DOES_NOT_EXIST](ETOKEN_DOES_NOT_EXIST): u64 = 1;
</code></pre>



<a id="0x4_token_ETOKEN_NAME_TOO_LONG"></a>

The token name is over the maximum length


<pre><code><b>const</b> [token.md#0x4_token_ETOKEN_NAME_TOO_LONG](ETOKEN_NAME_TOO_LONG): u64 = 4;
</code></pre>



<a id="0x4_token_MAX_TOKEN_NAME_LENGTH"></a>



<pre><code><b>const</b> [token.md#0x4_token_MAX_TOKEN_NAME_LENGTH](MAX_TOKEN_NAME_LENGTH): u64 = 128;
</code></pre>



<a id="0x4_token_MAX_TOKEN_SEED_LENGTH"></a>



<pre><code><b>const</b> [token.md#0x4_token_MAX_TOKEN_SEED_LENGTH](MAX_TOKEN_SEED_LENGTH): u64 = 128;
</code></pre>



<a id="0x4_token_create_common"></a>

## Function `create_common`



<pre><code><b>fun</b> [token.md#0x4_token_create_common](create_common)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), constructor_ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef), collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_prefix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_suffix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [token.md#0x4_token_create_common](create_common)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    constructor_ref: &ConstructorRef,
    collection_name: String,
    description: String,
    name_prefix: String,
    // If [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some), numbered [token.md#0x4_token](token) is created - i.e. index is appended <b>to</b> the name.
    // If [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none), name_prefix is the full name of the [token.md#0x4_token](token).
    name_with_index_suffix: Option&lt;String&gt;,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
) {
    <b>let</b> creator_address = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> collection_addr = [collection.md#0x4_collection_create_collection_address](collection::create_collection_address)(&creator_address, &collection_name);
    <b>let</b> [collection.md#0x4_collection](collection) = [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)&lt;Collection&gt;(collection_addr);

    [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(
        creator,
        constructor_ref,
        [collection.md#0x4_collection](collection),
        description,
        name_prefix,
        name_with_index_suffix,
        [royalty.md#0x4_royalty](royalty),
        uri
    )
}
</code></pre>



</details>

<a id="0x4_token_create_common_with_collection"></a>

## Function `create_common_with_collection`



<pre><code><b>fun</b> [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), constructor_ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef), [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_prefix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_suffix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    constructor_ref: &ConstructorRef,
    [collection.md#0x4_collection](collection): Object&lt;Collection&gt;,
    description: String,
    name_prefix: String,
    // If [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some), numbered [token.md#0x4_token](token) is created - i.e. index is appended <b>to</b> the name.
    // If [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none), name_prefix is the full name of the [token.md#0x4_token](token).
    name_with_index_suffix: Option&lt;String&gt;,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
) {
    <b>assert</b>!([collection.md#0x4_collection_creator](collection::creator)([collection.md#0x4_collection](collection)) == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_unauthenticated](error::unauthenticated)([token.md#0x4_token_ENOT_CREATOR](ENOT_CREATOR)));

    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&name_with_index_suffix)) {
        // Be conservative, <b>as</b> we don't know what length the index will be, and <b>assume</b> worst case (20 chars in MAX_U64)
        <b>assert</b>!(
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&name_prefix) + 20 + [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(
                [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&name_with_index_suffix)
            ) &lt;= [token.md#0x4_token_MAX_TOKEN_NAME_LENGTH](MAX_TOKEN_NAME_LENGTH),
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_ETOKEN_NAME_TOO_LONG](ETOKEN_NAME_TOO_LONG))
        );
    } <b>else</b> {
        <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&name_prefix) &lt;= [token.md#0x4_token_MAX_TOKEN_NAME_LENGTH](MAX_TOKEN_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_ETOKEN_NAME_TOO_LONG](ETOKEN_NAME_TOO_LONG)));
    };
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&description) &lt;= [token.md#0x4_token_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG)));
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&uri) &lt;= [token.md#0x4_token_MAX_URI_LENGTH](MAX_URI_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_EURI_TOO_LONG](EURI_TOO_LONG)));

    <b>let</b> object_signer = [../../aptos-framework/doc/object.md#0x1_object_generate_signer](object::generate_signer)(constructor_ref);

    <b>let</b> index = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_destroy_with_default](option::destroy_with_default)(
        [collection.md#0x4_collection_increment_supply](collection::increment_supply)(&[collection.md#0x4_collection](collection), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(&object_signer)),
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot](aggregator_v2::create_snapshot)&lt;u64&gt;(0)
    );

    // If create_numbered_token called us, add index <b>to</b> the name.
    <b>let</b> name = <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&name_with_index_suffix)) {
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_derive_string_concat](aggregator_v2::derive_string_concat)(name_prefix, &index, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> name_with_index_suffix))
    } <b>else</b> {
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_derived_string](aggregator_v2::create_derived_string)(name_prefix)
    };

    <b>let</b> deprecated_index = 0;
    <b>let</b> deprecated_name = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"");

    <b>let</b> token_concurrent = [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
        index,
        name,
    };
    <b>move_to</b>(&object_signer, token_concurrent);

    <b>let</b> [token.md#0x4_token](token) = [token.md#0x4_token_Token](Token) {
        [collection.md#0x4_collection](collection),
        index: deprecated_index,
        description,
        name: deprecated_name,
        uri,
        mutation_events: [../../aptos-framework/doc/object.md#0x1_object_new_event_handle](object::new_event_handle)(&object_signer),
    };
    <b>move_to</b>(&object_signer, [token.md#0x4_token](token));

    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&[royalty.md#0x4_royalty](royalty))) {
        [royalty.md#0x4_royalty_init](royalty::init)(constructor_ref, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> [royalty.md#0x4_royalty](royalty)))
    };
}
</code></pre>



</details>

<a id="0x4_token_create_token"></a>

## Function `create_token`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
This takes in the collection object instead of the collection name.
This function must be called if the collection name has been previously changed.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token](create_token)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token](create_token)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    [collection.md#0x4_collection](collection): Object&lt;Collection&gt;,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_object](object::create_object)(creator_address);
    [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(
        creator,
        &constructor_ref,
        [collection.md#0x4_collection](collection),
        description,
        name,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create"></a>

## Function `create`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create](create)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create](create)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection_name: String,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_object](object::create_object)(creator_address);
    [token.md#0x4_token_create_common](create_common)(
        creator,
        &constructor_ref,
        collection_name,
        description,
        name,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_numbered_token_object"></a>

## Function `create_numbered_token_object`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
The name is created by concatenating the (name_prefix, index, name_suffix).
This function allows creating tokens in parallel, from the same collection,
while providing sequential names.

This takes in the collection object instead of the collection name.
This function must be called if the collection name has been previously changed.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_numbered_token_object](create_numbered_token_object)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_prefix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_suffix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_numbered_token_object](create_numbered_token_object)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    [collection.md#0x4_collection](collection): Object&lt;Collection&gt;,
    description: String,
    name_with_index_prefix: String,
    name_with_index_suffix: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_object](object::create_object)(creator_address);
    [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(
        creator,
        &constructor_ref,
        [collection.md#0x4_collection](collection),
        description,
        name_with_index_prefix,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(name_with_index_suffix),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_numbered_token"></a>

## Function `create_numbered_token`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
The name is created by concatenating the (name_prefix, index, name_suffix).
This function will allow creating tokens in parallel, from the same collection,
while providing sequential names.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_numbered_token](create_numbered_token)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_prefix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name_with_index_suffix: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_numbered_token](create_numbered_token)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection_name: String,
    description: String,
    name_with_index_prefix: String,
    name_with_index_suffix: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_object](object::create_object)(creator_address);
    [token.md#0x4_token_create_common](create_common)(
        creator,
        &constructor_ref,
        collection_name,
        description,
        name_with_index_prefix,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(name_with_index_suffix),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_named_token_object"></a>

## Function `create_named_token_object`

Creates a new token object from a token name and returns the ConstructorRef for
additional specialization.
This function must be called if the collection name has been previously changed.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token_object](create_named_token_object)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token_object](create_named_token_object)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    [collection.md#0x4_collection](collection): Object&lt;Collection&gt;,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> seed = [token.md#0x4_token_create_token_seed](create_token_seed)(&[collection.md#0x4_collection_name](collection::name)([collection.md#0x4_collection](collection)), &name);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, seed);
    [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(
        creator,
        &constructor_ref,
        [collection.md#0x4_collection](collection),
        description,
        name,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_named_token"></a>

## Function `create_named_token`

Creates a new token object from a token name and returns the ConstructorRef for
additional specialization.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token](create_named_token)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token](create_named_token)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection_name: String,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> seed = [token.md#0x4_token_create_token_seed](create_token_seed)(&collection_name, &name);

    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, seed);
    [token.md#0x4_token_create_common](create_common)(
        creator,
        &constructor_ref,
        collection_name,
        description,
        name,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_named_token_from_seed"></a>

## Function `create_named_token_from_seed`

Creates a new token object from a token name and seed.
Returns the ConstructorRef for additional specialization.
This function must be called if the collection name has been previously changed.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token_from_seed](create_named_token_from_seed)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [collection.md#0x4_collection](collection): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;, description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), seed: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_named_token_from_seed](create_named_token_from_seed)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    [collection.md#0x4_collection](collection): Object&lt;Collection&gt;,
    description: String,
    name: String,
    seed: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> seed = [token.md#0x4_token_create_token_name_with_seed](create_token_name_with_seed)(&[collection.md#0x4_collection_name](collection::name)([collection.md#0x4_collection](collection)), &name, &seed);
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_named_object](object::create_named_object)(creator, seed);
    [token.md#0x4_token_create_common_with_collection](create_common_with_collection)(creator, &constructor_ref, [collection.md#0x4_collection](collection), description, name, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(), [royalty.md#0x4_royalty](royalty), uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_from_account"></a>

## Function `create_from_account`

DEPRECATED: Use <code>create</code> instead for identical behavior.

Creates a new token object from an account GUID and returns the ConstructorRef for
additional specialization.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [token.md#0x4_token_create_from_account](create_from_account)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [royalty.md#0x4_royalty](royalty): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;, uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_from_account](create_from_account)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection_name: String,
    description: String,
    name: String,
    [royalty.md#0x4_royalty](royalty): Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> constructor_ref = [../../aptos-framework/doc/object.md#0x1_object_create_object_from_account](object::create_object_from_account)(creator);
    [token.md#0x4_token_create_common](create_common)(
        creator,
        &constructor_ref,
        collection_name,
        description,
        name,
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        [royalty.md#0x4_royalty](royalty),
        uri
    );
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_token_address"></a>

## Function `create_token_address`

Generates the token's address based upon the creator's address, the collection's name and the token's name.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_address](create_token_address)(creator: &<b>address</b>, [collection.md#0x4_collection](collection): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_address](create_token_address)(creator: &<b>address</b>, [collection.md#0x4_collection](collection): &String, name: &String): <b>address</b> {
    [../../aptos-framework/doc/object.md#0x1_object_create_object_address](object::create_object_address)(creator, [token.md#0x4_token_create_token_seed](create_token_seed)([collection.md#0x4_collection](collection), name))
}
</code></pre>



</details>

<a id="0x4_token_create_token_address_with_seed"></a>

## Function `create_token_address_with_seed`

Generates the token's address based upon the creator's address, the collection object and the token's name and seed.


<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_create_token_address_with_seed](create_token_address_with_seed)(creator: <b>address</b>, [collection.md#0x4_collection](collection): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), seed: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_address_with_seed](create_token_address_with_seed)(creator: <b>address</b>, [collection.md#0x4_collection](collection): String, name: String, seed: String): <b>address</b> {
    <b>let</b> seed = [token.md#0x4_token_create_token_name_with_seed](create_token_name_with_seed)(&[collection.md#0x4_collection](collection), &name, &seed);
    [../../aptos-framework/doc/object.md#0x1_object_create_object_address](object::create_object_address)(&creator, seed)
}
</code></pre>



</details>

<a id="0x4_token_create_token_seed"></a>

## Function `create_token_seed`

Named objects are derived from a seed, the token's seed is its name appended to the collection's name.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_seed](create_token_seed)([collection.md#0x4_collection](collection): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_seed](create_token_seed)([collection.md#0x4_collection](collection): &String, name: &String): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(name) &lt;= [token.md#0x4_token_MAX_TOKEN_NAME_LENGTH](MAX_TOKEN_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_ETOKEN_NAME_TOO_LONG](ETOKEN_NAME_TOO_LONG)));
    <b>let</b> seed = *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes](string::bytes)([collection.md#0x4_collection](collection));
    [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)(&<b>mut</b> seed, b"::");
    [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)(&<b>mut</b> seed, *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes](string::bytes)(name));
    seed
}
</code></pre>



</details>

<a id="0x4_token_create_token_name_with_seed"></a>

## Function `create_token_name_with_seed`



<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_name_with_seed](create_token_name_with_seed)([collection.md#0x4_collection](collection): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), seed: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_create_token_name_with_seed](create_token_name_with_seed)([collection.md#0x4_collection](collection): &String, name: &String, seed: &String): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(seed) &lt;= [token.md#0x4_token_MAX_TOKEN_SEED_LENGTH](MAX_TOKEN_SEED_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_ESEED_TOO_LONG](ESEED_TOO_LONG)));
    <b>let</b> seeds = [token.md#0x4_token_create_token_seed](create_token_seed)([collection.md#0x4_collection](collection), name);
    [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)(&<b>mut</b> seeds, *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes](string::bytes)(seed));
    seeds
}
</code></pre>



</details>

<a id="0x4_token_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_generate_mutator_ref](generate_mutator_ref)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)): [token.md#0x4_token_MutatorRef](token::MutatorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_generate_mutator_ref](generate_mutator_ref)(ref: &ConstructorRef): [token.md#0x4_token_MutatorRef](MutatorRef) {
    <b>let</b> [../../aptos-framework/doc/object.md#0x1_object](object) = [../../aptos-framework/doc/object.md#0x1_object_object_from_constructor_ref](object::object_from_constructor_ref)&lt;[token.md#0x4_token_Token](Token)&gt;(ref);
    [token.md#0x4_token_MutatorRef](MutatorRef) { self: [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[../../aptos-framework/doc/object.md#0x1_object](object)) }
}
</code></pre>



</details>

<a id="0x4_token_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a BurnRef, which gates the ability to burn the given token.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_generate_burn_ref](generate_burn_ref)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)): [token.md#0x4_token_BurnRef](token::BurnRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_generate_burn_ref](generate_burn_ref)(ref: &ConstructorRef): [token.md#0x4_token_BurnRef](BurnRef) {
    <b>let</b> (inner, self) = <b>if</b> ([../../aptos-framework/doc/object.md#0x1_object_can_generate_delete_ref](object::can_generate_delete_ref)(ref)) {
        <b>let</b> delete_ref = [../../aptos-framework/doc/object.md#0x1_object_generate_delete_ref](object::generate_delete_ref)(ref);
        ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(delete_ref), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)())
    } <b>else</b> {
        <b>let</b> addr = [../../aptos-framework/doc/object.md#0x1_object_address_from_constructor_ref](object::address_from_constructor_ref)(ref);
        ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)(), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(addr))
    };
    [token.md#0x4_token_BurnRef](BurnRef) { self, inner }
}
</code></pre>



</details>

<a id="0x4_token_address_from_burn_ref"></a>

## Function `address_from_burn_ref`

Extracts the tokens address from a BurnRef.


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_address_from_burn_ref](address_from_burn_ref)(ref: &[token.md#0x4_token_BurnRef](token::BurnRef)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_address_from_burn_ref](address_from_burn_ref)(ref: &[token.md#0x4_token_BurnRef](BurnRef)): <b>address</b> {
    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&ref.inner)) {
        [../../aptos-framework/doc/object.md#0x1_object_address_from_delete_ref](object::address_from_delete_ref)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&ref.inner))
    } <b>else</b> {
        *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&ref.self)
    }
}
</code></pre>



</details>

<a id="0x4_token_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> [token.md#0x4_token_borrow](borrow)&lt;T: key&gt;([token.md#0x4_token](token): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): &[token.md#0x4_token_Token](token::Token)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [token.md#0x4_token_borrow](borrow)&lt;T: key&gt;([token.md#0x4_token](token): &Object&lt;T&gt;): &[token.md#0x4_token_Token](Token) <b>acquires</b> [token.md#0x4_token_Token](Token) {
    <b>let</b> token_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([token.md#0x4_token](token));
    <b>assert</b>!(
        <b>exists</b>&lt;[token.md#0x4_token_Token](Token)&gt;(token_address),
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([token.md#0x4_token_ETOKEN_DOES_NOT_EXIST](ETOKEN_DOES_NOT_EXIST)),
    );
    <b>borrow_global</b>&lt;[token.md#0x4_token_Token](Token)&gt;(token_address)
}
</code></pre>



</details>

<a id="0x4_token_creator"></a>

## Function `creator`



<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_creator](creator)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_creator](creator)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): <b>address</b> <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [collection.md#0x4_collection_creator](collection::creator)([token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).[collection.md#0x4_collection](collection))
}
</code></pre>



</details>

<a id="0x4_token_collection_name"></a>

## Function `collection_name`



<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_collection_name](collection_name)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_collection_name](collection_name)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): String <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [collection.md#0x4_collection_name](collection::name)([token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).[collection.md#0x4_collection](collection))
}
</code></pre>



</details>

<a id="0x4_token_collection_object"></a>

## Function `collection_object`



<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_collection_object](collection_object)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_collection_object](collection_object)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): Object&lt;Collection&gt; <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).[collection.md#0x4_collection](collection)
}
</code></pre>



</details>

<a id="0x4_token_description"></a>

## Function `description`



<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_description](description)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_description](description)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): String <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).description
}
</code></pre>



</details>

<a id="0x4_token_name"></a>

## Function `name`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_name](name)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_name](name)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): String <b>acquires</b> [token.md#0x4_token_Token](Token), [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
    <b>let</b> token_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[token.md#0x4_token](token));
    <b>if</b> (<b>exists</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(token_address)) {
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_derived_string](aggregator_v2::read_derived_string)(&<b>borrow_global</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(token_address).name)
    } <b>else</b> {
        [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).name
    }
}
</code></pre>



</details>

<a id="0x4_token_uri"></a>

## Function `uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_uri](uri)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_uri](uri)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): String <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).uri
}
</code></pre>



</details>

<a id="0x4_token_royalty"></a>

## Function `royalty`



<pre><code>#[view]
<b>public</b> <b>fun</b> [royalty.md#0x4_royalty](royalty)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty](royalty)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): Option&lt;Royalty&gt; <b>acquires</b> [token.md#0x4_token_Token](Token) {
    [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token));
    <b>let</b> [royalty.md#0x4_royalty](royalty) = [royalty.md#0x4_royalty_get](royalty::get)([token.md#0x4_token](token));
    <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&[royalty.md#0x4_royalty](royalty))) {
        [royalty.md#0x4_royalty](royalty)
    } <b>else</b> {
        <b>let</b> creator = [token.md#0x4_token_creator](creator)([token.md#0x4_token](token));
        <b>let</b> collection_name = [token.md#0x4_token_collection_name](collection_name)([token.md#0x4_token](token));
        <b>let</b> collection_address = [collection.md#0x4_collection_create_collection_address](collection::create_collection_address)(&creator, &collection_name);
        <b>let</b> [collection.md#0x4_collection](collection) = [../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)&lt;[collection.md#0x4_collection_Collection](collection::Collection)&gt;(collection_address);
        [royalty.md#0x4_royalty_get](royalty::get)([collection.md#0x4_collection](collection))
    }
}
</code></pre>



</details>

<a id="0x4_token_index"></a>

## Function `index`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


<pre><code>#[view]
<b>public</b> <b>fun</b> [token.md#0x4_token_index](index)&lt;T: key&gt;([token.md#0x4_token](token): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_index](index)&lt;T: key&gt;([token.md#0x4_token](token): Object&lt;T&gt;): u64 <b>acquires</b> [token.md#0x4_token_Token](Token), [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
    <b>let</b> token_address = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&[token.md#0x4_token](token));
    <b>if</b> (<b>exists</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(token_address)) {
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_snapshot](aggregator_v2::read_snapshot)(&<b>borrow_global</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(token_address).index)
    } <b>else</b> {
        [token.md#0x4_token_borrow](borrow)(&[token.md#0x4_token](token)).index
    }
}
</code></pre>



</details>

<a id="0x4_token_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> [token.md#0x4_token_borrow_mut](borrow_mut)(mutator_ref: &[token.md#0x4_token_MutatorRef](token::MutatorRef)): &<b>mut</b> [token.md#0x4_token_Token](token::Token)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [token.md#0x4_token_borrow_mut](borrow_mut)(mutator_ref: &[token.md#0x4_token_MutatorRef](MutatorRef)): &<b>mut</b> [token.md#0x4_token_Token](Token) <b>acquires</b> [token.md#0x4_token_Token](Token) {
    <b>assert</b>!(
        <b>exists</b>&lt;[token.md#0x4_token_Token](Token)&gt;(mutator_ref.self),
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([token.md#0x4_token_ETOKEN_DOES_NOT_EXIST](ETOKEN_DOES_NOT_EXIST)),
    );
    <b>borrow_global_mut</b>&lt;[token.md#0x4_token_Token](Token)&gt;(mutator_ref.self)
}
</code></pre>



</details>

<a id="0x4_token_burn"></a>

## Function `burn`



<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_burn](burn)(burn_ref: [token.md#0x4_token_BurnRef](token::BurnRef))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_burn](burn)(burn_ref: [token.md#0x4_token_BurnRef](BurnRef)) <b>acquires</b> [token.md#0x4_token_Token](Token), [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
    <b>let</b> (addr, previous_owner) = <b>if</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&burn_ref.inner)) {
        <b>let</b> delete_ref = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> burn_ref.inner);
        <b>let</b> addr = [../../aptos-framework/doc/object.md#0x1_object_address_from_delete_ref](object::address_from_delete_ref)(&delete_ref);
        <b>let</b> previous_owner = [../../aptos-framework/doc/object.md#0x1_object_owner](object::owner)([../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)&lt;[token.md#0x4_token_Token](Token)&gt;(addr));
        [../../aptos-framework/doc/object.md#0x1_object_delete](object::delete)(delete_ref);
        (addr, previous_owner)
    } <b>else</b> {
        <b>let</b> addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> burn_ref.self);
        <b>let</b> previous_owner = [../../aptos-framework/doc/object.md#0x1_object_owner](object::owner)([../../aptos-framework/doc/object.md#0x1_object_address_to_object](object::address_to_object)&lt;[token.md#0x4_token_Token](Token)&gt;(addr));
        (addr, previous_owner)
    };

    <b>if</b> ([royalty.md#0x4_royalty_exists_at](royalty::exists_at)(addr)) {
        [royalty.md#0x4_royalty_delete](royalty::delete)(addr)
    };

    <b>let</b> [token.md#0x4_token_Token](Token) {
        [collection.md#0x4_collection](collection),
        index: deprecated_index,
        description: _,
        name: _,
        uri: _,
        mutation_events,
    } = <b>move_from</b>&lt;[token.md#0x4_token_Token](Token)&gt;(addr);

    <b>let</b> index = <b>if</b> (<b>exists</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(addr)) {
        <b>let</b> [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
            index,
            name: _,
        } = <b>move_from</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(addr);
        [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_snapshot](aggregator_v2::read_snapshot)(&index)
    } <b>else</b> {
        deprecated_index
    };

    [../../aptos-framework/doc/event.md#0x1_event_destroy_handle](event::destroy_handle)(mutation_events);
    [collection.md#0x4_collection_decrement_supply](collection::decrement_supply)(&[collection.md#0x4_collection](collection), addr, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(index), previous_owner);
}
</code></pre>



</details>

<a id="0x4_token_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_description](set_description)(mutator_ref: &[token.md#0x4_token_MutatorRef](token::MutatorRef), description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_description](set_description)(mutator_ref: &[token.md#0x4_token_MutatorRef](MutatorRef), description: String) <b>acquires</b> [token.md#0x4_token_Token](Token) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&description) &lt;= [token.md#0x4_token_MAX_DESCRIPTION_LENGTH](MAX_DESCRIPTION_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_EDESCRIPTION_TOO_LONG](EDESCRIPTION_TOO_LONG)));
    <b>let</b> [token.md#0x4_token](token) = [token.md#0x4_token_borrow_mut](borrow_mut)(mutator_ref);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([token.md#0x4_token_Mutation](Mutation) {
            token_address: mutator_ref.self,
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"description"),
            old_value: [token.md#0x4_token](token).description,
            new_value: description
        })
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
        &<b>mut</b> [token.md#0x4_token](token).mutation_events,
        [token.md#0x4_token_MutationEvent](MutationEvent) {
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"description"),
            old_value: [token.md#0x4_token](token).description,
            new_value: description
        },
    );
    [token.md#0x4_token](token).description = description;
}
</code></pre>



</details>

<a id="0x4_token_set_name"></a>

## Function `set_name`



<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_name](set_name)(mutator_ref: &[token.md#0x4_token_MutatorRef](token::MutatorRef), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_name](set_name)(mutator_ref: &[token.md#0x4_token_MutatorRef](MutatorRef), name: String) <b>acquires</b> [token.md#0x4_token_Token](Token), [token.md#0x4_token_TokenIdentifiers](TokenIdentifiers) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&name) &lt;= [token.md#0x4_token_MAX_TOKEN_NAME_LENGTH](MAX_TOKEN_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_ETOKEN_NAME_TOO_LONG](ETOKEN_NAME_TOO_LONG)));

    <b>let</b> [token.md#0x4_token](token) = [token.md#0x4_token_borrow_mut](borrow_mut)(mutator_ref);

    <b>let</b> old_name = <b>if</b> (<b>exists</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(mutator_ref.self)) {
        <b>let</b> token_concurrent = <b>borrow_global_mut</b>&lt;[token.md#0x4_token_TokenIdentifiers](TokenIdentifiers)&gt;(mutator_ref.self);
        <b>let</b> old_name = [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_derived_string](aggregator_v2::read_derived_string)(&token_concurrent.name);
        token_concurrent.name = [../../aptos-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_derived_string](aggregator_v2::create_derived_string)(name);
        old_name
    } <b>else</b> {
        <b>let</b> old_name = [token.md#0x4_token](token).name;
        [token.md#0x4_token](token).name = name;
        old_name
    };

    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([token.md#0x4_token_Mutation](Mutation) {
            token_address: mutator_ref.self,
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"name"),
            old_value: old_name,
            new_value: name
        })
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
        &<b>mut</b> [token.md#0x4_token](token).mutation_events,
        [token.md#0x4_token_MutationEvent](MutationEvent) {
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"name"),
            old_value: old_name,
            new_value: name
        },
    );
}
</code></pre>



</details>

<a id="0x4_token_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_uri](set_uri)(mutator_ref: &[token.md#0x4_token_MutatorRef](token::MutatorRef), uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token.md#0x4_token_set_uri](set_uri)(mutator_ref: &[token.md#0x4_token_MutatorRef](MutatorRef), uri: String) <b>acquires</b> [token.md#0x4_token_Token](Token) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&uri) &lt;= [token.md#0x4_token_MAX_URI_LENGTH](MAX_URI_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([token.md#0x4_token_EURI_TOO_LONG](EURI_TOO_LONG)));
    <b>let</b> [token.md#0x4_token](token) = [token.md#0x4_token_borrow_mut](borrow_mut)(mutator_ref);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)([token.md#0x4_token_Mutation](Mutation) {
            token_address: mutator_ref.self,
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"uri"),
            old_value: [token.md#0x4_token](token).uri,
            new_value: uri,
        })
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)(
        &<b>mut</b> [token.md#0x4_token](token).mutation_events,
        [token.md#0x4_token_MutationEvent](MutationEvent) {
            mutated_field_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"uri"),
            old_value: [token.md#0x4_token](token).uri,
            new_value: uri,
        },
    );
    [token.md#0x4_token](token).uri = uri;
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
