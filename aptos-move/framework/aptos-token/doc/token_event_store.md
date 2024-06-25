
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


<pre><code><b>use</b> [../../aptos-framework/doc/account.md#0x1_account](0x1::account);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/any.md#0x1_any](0x1::any);
<b>use</b> [../../aptos-framework/doc/event.md#0x1_event](0x1::event);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/features.md#0x1_features](0x1::features);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](0x1::signer);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [property_map.md#0x3_property_map](0x3::property_map);
</code></pre>



<a id="0x3_token_event_store_CollectionDescriptionMutateEvent"></a>

## Struct `CollectionDescriptionMutateEvent`

Event emitted when collection description is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionDescriptionMutateEvent](CollectionDescriptionMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_CollectionDescriptionMutate"></a>

## Struct `CollectionDescriptionMutate`

Event emitted when collection description is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionDescriptionMutate](CollectionDescriptionMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_CollectionUriMutateEvent"></a>

## Struct `CollectionUriMutateEvent`

Event emitted when collection uri is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionUriMutateEvent](CollectionUriMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_CollectionUriMutate"></a>

## Struct `CollectionUriMutate`

Event emitted when collection uri is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionUriMutate](CollectionUriMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_CollectionMaxiumMutateEvent"></a>

## Struct `CollectionMaxiumMutateEvent`

Event emitted when the collection maximum is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionMaxiumMutateEvent](CollectionMaxiumMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_maximum: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_maximum: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_CollectionMaxiumMutate"></a>

## Struct `CollectionMaxiumMutate`

Event emitted when the collection maximum is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_CollectionMaxiumMutate](CollectionMaxiumMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection_name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_maximum: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_maximum: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_OptInTransferEvent"></a>

## Struct `OptInTransferEvent`

Event emitted when an user opt-in the direct transfer


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_OptInTransferEvent](OptInTransferEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>opt_in: bool</code>
</dt>
<dd>
 True if the user opt in, false if the user opt-out
</dd>
</dl>


</details>

<a id="0x3_token_event_store_OptInTransfer"></a>

## Struct `OptInTransfer`

Event emitted when an user opt-in the direct transfer


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_OptInTransfer](OptInTransfer) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>opt_in: bool</code>
</dt>
<dd>
 True if the user opt in, false if the user opt-out
</dd>
</dl>


</details>

<a id="0x3_token_event_store_UriMutationEvent"></a>

## Struct `UriMutationEvent`

Event emitted when the tokendata uri mutates


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_UriMutationEvent](UriMutationEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_UriMutation"></a>

## Struct `UriMutation`

Event emitted when the tokendata uri mutates


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_UriMutation](UriMutation) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_DefaultPropertyMutateEvent"></a>

## Struct `DefaultPropertyMutateEvent`

Event emitted when mutating the default the token properties stored at tokendata


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_DefaultPropertyMutateEvent](DefaultPropertyMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>old_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;&gt;</code>
</dt>
<dd>
 we allow upsert so the old values might be none
</dd>
<dt>
<code>new_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_DefaultPropertyMutate"></a>

## Struct `DefaultPropertyMutate`

Event emitted when mutating the default the token properties stored at tokendata


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_DefaultPropertyMutate](DefaultPropertyMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>old_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;&gt;</code>
</dt>
<dd>
 we allow upsert so the old values might be none
</dd>
<dt>
<code>new_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_DescriptionMutateEvent"></a>

## Struct `DescriptionMutateEvent`

Event emitted when the tokendata description is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_DescriptionMutateEvent](DescriptionMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_DescriptionMutate"></a>

## Struct `DescriptionMutate`

Event emitted when the tokendata description is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_DescriptionMutate](DescriptionMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_RoyaltyMutateEvent"></a>

## Struct `RoyaltyMutateEvent`

Event emitted when the token royalty is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_RoyaltyMutateEvent](RoyaltyMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_numerator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_denominator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_payee_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_numerator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_denominator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_payee_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_RoyaltyMutate"></a>

## Struct `RoyaltyMutate`

Event emitted when the token royalty is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_RoyaltyMutate](RoyaltyMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_numerator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_denominator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>old_royalty_payee_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_numerator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_denominator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_royalty_payee_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_MaxiumMutateEvent"></a>

## Struct `MaxiumMutateEvent`

Event emitted when the token maximum is mutated


<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_MaxiumMutateEvent](MaxiumMutateEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_maximum: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_maximum: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_MaximumMutate"></a>

## Struct `MaximumMutate`

Event emitted when the token maximum is mutated


<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_event_store.md#0x3_token_event_store_MaximumMutate](MaximumMutate) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>[token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>old_maximum: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_maximum: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_event_store_TokenEventStoreV1"></a>

## Resource `TokenEventStoreV1`



<pre><code><b>struct</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>collection_uri_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_CollectionUriMutateEvent](token_event_store::CollectionUriMutateEvent)&gt;</code>
</dt>
<dd>
 collection mutation events
</dd>
<dt>
<code>collection_maximum_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_CollectionMaxiumMutateEvent](token_event_store::CollectionMaxiumMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>collection_description_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_CollectionDescriptionMutateEvent](token_event_store::CollectionDescriptionMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>opt_in_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_OptInTransferEvent](token_event_store::OptInTransferEvent)&gt;</code>
</dt>
<dd>
 token transfer opt-in event
</dd>
<dt>
<code>uri_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_UriMutationEvent](token_event_store::UriMutationEvent)&gt;</code>
</dt>
<dd>
 token mutation events
</dd>
<dt>
<code>default_property_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_DefaultPropertyMutateEvent](token_event_store::DefaultPropertyMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>description_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_DescriptionMutateEvent](token_event_store::DescriptionMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>royalty_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_RoyaltyMutateEvent](token_event_store::RoyaltyMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>maximum_mutate_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_event_store.md#0x3_token_event_store_MaxiumMutateEvent](token_event_store::MaxiumMutateEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>extension: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[../../aptos-framework/../aptos-stdlib/doc/any.md#0x1_any_Any](any::Any)&gt;</code>
</dt>
<dd>
 This is for adding new events in future
</dd>
</dl>


</details>

<a id="0x3_token_event_store_initialize_token_event_store"></a>

## Function `initialize_token_event_store`



<pre><code><b>fun</b> [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(acct: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(acct: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer)){
    <b>if</b> (!<b>exists</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(acct))) {
        <b>move_to</b>(acct, [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
            collection_uri_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_CollectionUriMutateEvent](CollectionUriMutateEvent)&gt;(acct),
            collection_maximum_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_CollectionMaxiumMutateEvent](CollectionMaxiumMutateEvent)&gt;(acct),
            collection_description_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_CollectionDescriptionMutateEvent](CollectionDescriptionMutateEvent)&gt;(acct),
            opt_in_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_OptInTransferEvent](OptInTransferEvent)&gt;(acct),
            uri_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_UriMutationEvent](UriMutationEvent)&gt;(acct),
            default_property_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_DefaultPropertyMutateEvent](DefaultPropertyMutateEvent)&gt;(acct),
            description_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_DescriptionMutateEvent](DescriptionMutateEvent)&gt;(acct),
            royalty_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_RoyaltyMutateEvent](RoyaltyMutateEvent)&gt;(acct),
            maximum_mutate_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_event_store.md#0x3_token_event_store_MaxiumMutateEvent](MaxiumMutateEvent)&gt;(acct),
            extension: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)&lt;Any&gt;(),
        });
    };
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_collection_uri_mutate_event"></a>

## Function `emit_collection_uri_mutate_event`

Emit the collection uri mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_uri_mutate_event](emit_collection_uri_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_uri_mutate_event](emit_collection_uri_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: String, old_uri: String, new_uri: String) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_CollectionUriMutateEvent](CollectionUriMutateEvent) {
        creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
        collection_name: collection,
        old_uri,
        new_uri,
    };
    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator));
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_CollectionUriMutate](CollectionUriMutate) {
                creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
                collection_name: collection,
                old_uri,
                new_uri,
            }
        );
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_CollectionUriMutateEvent](CollectionUriMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).collection_uri_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_collection_description_mutate_event"></a>

## Function `emit_collection_description_mutate_event`

Emit the collection description mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_description_mutate_event](emit_collection_description_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_description_mutate_event](emit_collection_description_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: String, old_description: String, new_description: String) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_CollectionDescriptionMutateEvent](CollectionDescriptionMutateEvent) {
        creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
        collection_name: collection,
        old_description,
        new_description,
    };
    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator));
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_CollectionDescriptionMutate](CollectionDescriptionMutate) {
                creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
                collection_name: collection,
                old_description,
                new_description,
            }
        );
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_CollectionDescriptionMutateEvent](CollectionDescriptionMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).collection_description_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_collection_maximum_mutate_event"></a>

## Function `emit_collection_maximum_mutate_event`

Emit the collection maximum mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_maximum_mutate_event](emit_collection_maximum_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_maximum: u64, new_maximum: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_maximum_mutate_event](emit_collection_maximum_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: String, old_maximum: u64, new_maximum: u64) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_CollectionMaxiumMutateEvent](CollectionMaxiumMutateEvent) {
        creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
        collection_name: collection,
        old_maximum,
        new_maximum,
    };
    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator));
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_CollectionMaxiumMutate](CollectionMaxiumMutate) {
                creator_addr: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator),
                collection_name: collection,
                old_maximum,
                new_maximum,
            }
        );
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_CollectionMaxiumMutateEvent](CollectionMaxiumMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).collection_maximum_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_token_opt_in_event"></a>

## Function `emit_token_opt_in_event`

Emit the direct opt-in event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_opt_in_event](emit_token_opt_in_event)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), opt_in: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_opt_in_event](emit_token_opt_in_event)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), opt_in: bool) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> opt_in_event = [token_event_store.md#0x3_token_event_store_OptInTransferEvent](OptInTransferEvent) {
      opt_in,
    };
    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)([../../aptos-framework/doc/account.md#0x1_account](account));
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)([../../aptos-framework/doc/account.md#0x1_account](account)));
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_OptInTransfer](OptInTransfer) {
                account_address: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)([../../aptos-framework/doc/account.md#0x1_account](account)),
                opt_in,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_OptInTransferEvent](OptInTransferEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).opt_in_events,
        opt_in_event,
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_token_uri_mutate_event"></a>

## Function `emit_token_uri_mutate_event`

Emit URI mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_uri_mutate_event](emit_token_uri_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_uri_mutate_event](emit_token_uri_mutate_event)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection: String,
    [token.md#0x3_token](token): String,
    old_uri: String,
    new_uri: String,
) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> creator_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);

    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_UriMutationEvent](UriMutationEvent) {
        creator: creator_addr,
        collection,
        [token.md#0x3_token](token),
        old_uri,
        new_uri,
    };

    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(creator_addr);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_UriMutation](UriMutation) {
                creator: creator_addr,
                collection,
                [token.md#0x3_token](token),
                old_uri,
                new_uri,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_UriMutationEvent](UriMutationEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).uri_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_default_property_mutate_event"></a>

## Function `emit_default_property_mutate_event`

Emit tokendata property map mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_default_property_mutate_event](emit_default_property_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, old_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;&gt;, new_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_default_property_mutate_event](emit_default_property_mutate_event)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection: String,
    [token.md#0x3_token](token): String,
    keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    old_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Option&lt;PropertyValue&gt;&gt;,
    new_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;PropertyValue&gt;,
) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> creator_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);

    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_DefaultPropertyMutateEvent](DefaultPropertyMutateEvent) {
        creator: creator_addr,
        collection,
        [token.md#0x3_token](token),
        keys,
        old_values,
        new_values,
    };

    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(creator_addr);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_DefaultPropertyMutate](DefaultPropertyMutate) {
                creator: creator_addr,
                collection,
                [token.md#0x3_token](token),
                keys,
                old_values,
                new_values,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_DefaultPropertyMutateEvent](DefaultPropertyMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).default_property_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_token_descrition_mutate_event"></a>

## Function `emit_token_descrition_mutate_event`

Emit description mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_descrition_mutate_event](emit_token_descrition_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_descrition_mutate_event](emit_token_descrition_mutate_event)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection: String,
    [token.md#0x3_token](token): String,
    old_description: String,
    new_description: String,
) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> creator_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);

    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_DescriptionMutateEvent](DescriptionMutateEvent) {
        creator: creator_addr,
        collection,
        [token.md#0x3_token](token),
        old_description,
        new_description,
    };

    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(creator_addr);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_DescriptionMutate](DescriptionMutate) {
                creator: creator_addr,
                collection,
                [token.md#0x3_token](token),
                old_description,
                new_description,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_DescriptionMutateEvent](DescriptionMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).description_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_token_royalty_mutate_event"></a>

## Function `emit_token_royalty_mutate_event`

Emit royalty mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_royalty_mutate_event](emit_token_royalty_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_royalty_numerator: u64, old_royalty_denominator: u64, old_royalty_payee_addr: <b>address</b>, new_royalty_numerator: u64, new_royalty_denominator: u64, new_royalty_payee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_royalty_mutate_event](emit_token_royalty_mutate_event)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection: String,
    [token.md#0x3_token](token): String,
    old_royalty_numerator: u64,
    old_royalty_denominator: u64,
    old_royalty_payee_addr: <b>address</b>,
    new_royalty_numerator: u64,
    new_royalty_denominator: u64,
    new_royalty_payee_addr: <b>address</b>,
) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> creator_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_RoyaltyMutateEvent](RoyaltyMutateEvent) {
        creator: creator_addr,
        collection,
        [token.md#0x3_token](token),
        old_royalty_numerator,
        old_royalty_denominator,
        old_royalty_payee_addr,
        new_royalty_numerator,
        new_royalty_denominator,
        new_royalty_payee_addr,
    };

    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) = <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(creator_addr);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_RoyaltyMutate](RoyaltyMutate) {
                creator: creator_addr,
                collection,
                [token.md#0x3_token](token),
                old_royalty_numerator,
                old_royalty_denominator,
                old_royalty_payee_addr,
                new_royalty_numerator,
                new_royalty_denominator,
                new_royalty_payee_addr,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_RoyaltyMutateEvent](RoyaltyMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).royalty_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="0x3_token_event_store_emit_token_maximum_mutate_event"></a>

## Function `emit_token_maximum_mutate_event`

Emit maximum mutation event


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_maximum_mutate_event](emit_token_maximum_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_maximum: u64, new_maximum: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_maximum_mutate_event](emit_token_maximum_mutate_event)(
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    collection: String,
    [token.md#0x3_token](token): String,
    old_maximum: u64,
    new_maximum: u64,
) <b>acquires</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1) {
    <b>let</b> creator_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);

    <b>let</b> [../../aptos-framework/doc/event.md#0x1_event](event) = [token_event_store.md#0x3_token_event_store_MaxiumMutateEvent](MaxiumMutateEvent) {
        creator: creator_addr,
        collection,
        [token.md#0x3_token](token),
        old_maximum,
        new_maximum,
    };

    [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(creator);
    <b>let</b> [token_event_store.md#0x3_token_event_store](token_event_store) =  <b>borrow_global_mut</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(creator_addr);
    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_event_store.md#0x3_token_event_store_MaximumMutate](MaximumMutate) {
                creator: creator_addr,
                collection,
                [token.md#0x3_token](token),
                old_maximum,
                new_maximum,
            });
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_event_store.md#0x3_token_event_store_MaxiumMutateEvent](MaxiumMutateEvent)&gt;(
        &<b>mut</b> [token_event_store.md#0x3_token_event_store](token_event_store).maximum_mutate_events,
        [../../aptos-framework/doc/event.md#0x1_event](event),
    );
}
</code></pre>



</details>

<a id="@Specification_0"></a>

## Specification



<pre><code><b>pragma</b> verify = <b>true</b>;
<b>pragma</b> aborts_if_is_strict;
</code></pre>



<a id="@Specification_0_initialize_token_event_store"></a>

### Function `initialize_token_event_store`


<pre><code><b>fun</b> [token_event_store.md#0x3_token_event_store_initialize_token_event_store](initialize_token_event_store)(acct: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer))
</code></pre>




<pre><code><b>pragma</b> verify = <b>true</b>;
<b>let</b> addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(acct);
<b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf) {creator : acct};
</code></pre>


Adjust the overflow value according to the
number of registered events


<a id="0x3_token_event_store_InitializeTokenEventStoreAbortsIf"></a>


<pre><code><b>schema</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf) {
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer);
    <b>let</b> addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> [../../aptos-framework/doc/account.md#0x1_account](account) = <b>global</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> !<b>exists</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(addr) && !<b>exists</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> !<b>exists</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(addr) && [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 9 &gt;= [../../aptos-framework/doc/account.md#0x1_account_MAX_GUID_CREATION_NUM](account::MAX_GUID_CREATION_NUM);
    <b>aborts_if</b> !<b>exists</b>&lt;[token_event_store.md#0x3_token_event_store_TokenEventStoreV1](TokenEventStoreV1)&gt;(addr) && [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 9 &gt; MAX_U64;
}
</code></pre>




<a id="0x3_token_event_store_TokenEventStoreAbortsIf"></a>


<pre><code><b>schema</b> [token_event_store.md#0x3_token_event_store_TokenEventStoreAbortsIf](TokenEventStoreAbortsIf) {
    creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer);
    <b>let</b> addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(creator);
    <b>let</b> [../../aptos-framework/doc/account.md#0x1_account](account) = <b>global</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> !<b>exists</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 9 &gt;= [../../aptos-framework/doc/account.md#0x1_account_MAX_GUID_CREATION_NUM](account::MAX_GUID_CREATION_NUM);
    <b>aborts_if</b> [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 9 &gt; MAX_U64;
}
</code></pre>



<a id="@Specification_0_emit_collection_uri_mutate_event"></a>

### Function `emit_collection_uri_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_uri_mutate_event](emit_collection_uri_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_collection_description_mutate_event"></a>

### Function `emit_collection_description_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_description_mutate_event](emit_collection_description_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_collection_maximum_mutate_event"></a>

### Function `emit_collection_maximum_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_collection_maximum_mutate_event](emit_collection_maximum_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_maximum: u64, new_maximum: u64)
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_token_opt_in_event"></a>

### Function `emit_token_opt_in_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_opt_in_event](emit_token_opt_in_event)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), opt_in: bool)
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf) {creator : [../../aptos-framework/doc/account.md#0x1_account](account)};
</code></pre>



<a id="@Specification_0_emit_token_uri_mutate_event"></a>

### Function `emit_token_uri_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_uri_mutate_event](emit_token_uri_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_uri: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_default_property_mutate_event"></a>

### Function `emit_default_property_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_default_property_mutate_event](emit_default_property_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, old_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;&gt;, new_values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;)
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_token_descrition_mutate_event"></a>

### Function `emit_token_descrition_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_descrition_mutate_event](emit_token_descrition_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), new_description: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_token_royalty_mutate_event"></a>

### Function `emit_token_royalty_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_royalty_mutate_event](emit_token_royalty_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_royalty_numerator: u64, old_royalty_denominator: u64, old_royalty_payee_addr: <b>address</b>, new_royalty_numerator: u64, new_royalty_denominator: u64, new_royalty_payee_addr: <b>address</b>)
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>



<a id="@Specification_0_emit_token_maximum_mutate_event"></a>

### Function `emit_token_maximum_mutate_event`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [token_event_store.md#0x3_token_event_store_emit_token_maximum_mutate_event](emit_token_maximum_mutate_event)(creator: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [token.md#0x3_token](token): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), old_maximum: u64, new_maximum: u64)
</code></pre>




<pre><code><b>include</b> [token_event_store.md#0x3_token_event_store_InitializeTokenEventStoreAbortsIf](InitializeTokenEventStoreAbortsIf);
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
