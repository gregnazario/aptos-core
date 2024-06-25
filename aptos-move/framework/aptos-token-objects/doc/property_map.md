
<a id="0x4_property_map"></a>

# Module `0x4::property_map`

<code>[property_map.md#0x4_property_map_PropertyMap](PropertyMap)</code> provides generic metadata support for <code>AptosToken</code>. It is a specialization of
<code>SimpleMap</code> that enforces strict typing with minimal storage use by using constant u64 to
represent types and storing values in bcs format.


-  [Resource `PropertyMap`](#0x4_property_map_PropertyMap)
-  [Struct `PropertyValue`](#0x4_property_map_PropertyValue)
-  [Struct `MutatorRef`](#0x4_property_map_MutatorRef)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x4_property_map_init)
-  [Function `extend`](#0x4_property_map_extend)
-  [Function `burn`](#0x4_property_map_burn)
-  [Function `prepare_input`](#0x4_property_map_prepare_input)
-  [Function `to_external_type`](#0x4_property_map_to_external_type)
-  [Function `to_internal_type`](#0x4_property_map_to_internal_type)
-  [Function `type_info_to_internal_type`](#0x4_property_map_type_info_to_internal_type)
-  [Function `validate_type`](#0x4_property_map_validate_type)
-  [Function `generate_mutator_ref`](#0x4_property_map_generate_mutator_ref)
-  [Function `contains_key`](#0x4_property_map_contains_key)
-  [Function `length`](#0x4_property_map_length)
-  [Function `read`](#0x4_property_map_read)
-  [Function `assert_exists`](#0x4_property_map_assert_exists)
-  [Function `read_typed`](#0x4_property_map_read_typed)
-  [Function `read_bool`](#0x4_property_map_read_bool)
-  [Function `read_u8`](#0x4_property_map_read_u8)
-  [Function `read_u16`](#0x4_property_map_read_u16)
-  [Function `read_u32`](#0x4_property_map_read_u32)
-  [Function `read_u64`](#0x4_property_map_read_u64)
-  [Function `read_u128`](#0x4_property_map_read_u128)
-  [Function `read_u256`](#0x4_property_map_read_u256)
-  [Function `read_address`](#0x4_property_map_read_address)
-  [Function `read_bytes`](#0x4_property_map_read_bytes)
-  [Function `read_string`](#0x4_property_map_read_string)
-  [Function `add`](#0x4_property_map_add)
-  [Function `add_typed`](#0x4_property_map_add_typed)
-  [Function `add_internal`](#0x4_property_map_add_internal)
-  [Function `update`](#0x4_property_map_update)
-  [Function `update_typed`](#0x4_property_map_update_typed)
-  [Function `update_internal`](#0x4_property_map_update_internal)
-  [Function `remove`](#0x4_property_map_remove)
-  [Function `assert_end_to_end_input`](#0x4_property_map_assert_end_to_end_input)


<pre><code><b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs](0x1::bcs);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs](0x1::from_bcs);
<b>use</b> [../../aptos-framework/doc/object.md#0x1_object](0x1::object);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map](0x1::simple_map);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/type_info.md#0x1_type_info](0x1::type_info);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x4_property_map_PropertyMap"></a>

## Resource `PropertyMap`

A Map for typed key to value mapping, the contract using it
should keep track of what keys are what types, and parse them accordingly.


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) <b>has</b> drop, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [property_map.md#0x4_property_map_PropertyValue](property_map::PropertyValue)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_property_map_PropertyValue"></a>

## Struct `PropertyValue`

A typed value for the <code>[property_map.md#0x4_property_map_PropertyMap](PropertyMap)</code> to ensure that typing is always consistent


<pre><code><b>struct</b> [property_map.md#0x4_property_map_PropertyValue](PropertyValue) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_property_map_MutatorRef"></a>

## Struct `MutatorRef`

A mutator ref that allows for mutation of the property map


<pre><code><b>struct</b> [property_map.md#0x4_property_map_MutatorRef](MutatorRef) <b>has</b> drop, store
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

<a id="@Constants_0"></a>

## Constants


<a id="0x4_property_map_ETYPE_MISMATCH"></a>

Property value does not match expected type


<pre><code><b>const</b> [property_map.md#0x4_property_map_ETYPE_MISMATCH](ETYPE_MISMATCH): u64 = 6;
</code></pre>



<a id="0x4_property_map_ADDRESS"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_ADDRESS](ADDRESS): u8 = 7;
</code></pre>



<a id="0x4_property_map_BOOL"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_BOOL](BOOL): u8 = 0;
</code></pre>



<a id="0x4_property_map_BYTE_VECTOR"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_BYTE_VECTOR](BYTE_VECTOR): u8 = 8;
</code></pre>



<a id="0x4_property_map_EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP"></a>

The property key already exists


<pre><code><b>const</b> [property_map.md#0x4_property_map_EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP](EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP): u64 = 2;
</code></pre>



<a id="0x4_property_map_EKEY_TYPE_COUNT_MISMATCH"></a>

Property key and type counts do not match


<pre><code><b>const</b> [property_map.md#0x4_property_map_EKEY_TYPE_COUNT_MISMATCH](EKEY_TYPE_COUNT_MISMATCH): u64 = 5;
</code></pre>



<a id="0x4_property_map_EKEY_VALUE_COUNT_MISMATCH"></a>

Property key and value counts do not match


<pre><code><b>const</b> [property_map.md#0x4_property_map_EKEY_VALUE_COUNT_MISMATCH](EKEY_VALUE_COUNT_MISMATCH): u64 = 4;
</code></pre>



<a id="0x4_property_map_EPROPERTY_MAP_DOES_NOT_EXIST"></a>

The property map does not exist


<pre><code><b>const</b> [property_map.md#0x4_property_map_EPROPERTY_MAP_DOES_NOT_EXIST](EPROPERTY_MAP_DOES_NOT_EXIST): u64 = 1;
</code></pre>



<a id="0x4_property_map_EPROPERTY_MAP_KEY_TOO_LONG"></a>

The key of the property is too long


<pre><code><b>const</b> [property_map.md#0x4_property_map_EPROPERTY_MAP_KEY_TOO_LONG](EPROPERTY_MAP_KEY_TOO_LONG): u64 = 8;
</code></pre>



<a id="0x4_property_map_ETOO_MANY_PROPERTIES"></a>

The number of properties exceeds the maximum


<pre><code><b>const</b> [property_map.md#0x4_property_map_ETOO_MANY_PROPERTIES](ETOO_MANY_PROPERTIES): u64 = 3;
</code></pre>



<a id="0x4_property_map_ETYPE_INVALID"></a>

Invalid value type specified


<pre><code><b>const</b> [property_map.md#0x4_property_map_ETYPE_INVALID](ETYPE_INVALID): u64 = 7;
</code></pre>



<a id="0x4_property_map_MAX_PROPERTY_MAP_SIZE"></a>

Maximum number of items in a <code>[property_map.md#0x4_property_map_PropertyMap](PropertyMap)</code>


<pre><code><b>const</b> [property_map.md#0x4_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE): u64 = 1000;
</code></pre>



<a id="0x4_property_map_MAX_PROPERTY_NAME_LENGTH"></a>

Maximum number of characters in a property name


<pre><code><b>const</b> [property_map.md#0x4_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH): u64 = 128;
</code></pre>



<a id="0x4_property_map_STRING"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_STRING](STRING): u8 = 9;
</code></pre>



<a id="0x4_property_map_U128"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U128](U128): u8 = 5;
</code></pre>



<a id="0x4_property_map_U16"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U16](U16): u8 = 2;
</code></pre>



<a id="0x4_property_map_U256"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U256](U256): u8 = 6;
</code></pre>



<a id="0x4_property_map_U32"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U32](U32): u8 = 3;
</code></pre>



<a id="0x4_property_map_U64"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U64](U64): u8 = 4;
</code></pre>



<a id="0x4_property_map_U8"></a>



<pre><code><b>const</b> [property_map.md#0x4_property_map_U8](U8): u8 = 1;
</code></pre>



<a id="0x4_property_map_init"></a>

## Function `init`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_init](init)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef), container: [property_map.md#0x4_property_map_PropertyMap](property_map::PropertyMap))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_init](init)(ref: &ConstructorRef, container: [property_map.md#0x4_property_map_PropertyMap](PropertyMap)) {
    <b>let</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer) = [../../aptos-framework/doc/object.md#0x1_object_generate_signer](object::generate_signer)(ref);
    <b>move_to</b>(&[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), container);
}
</code></pre>



</details>

<a id="0x4_property_map_extend"></a>

## Function `extend`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_extend](extend)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ExtendRef](object::ExtendRef), container: [property_map.md#0x4_property_map_PropertyMap](property_map::PropertyMap))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_extend](extend)(ref: &ExtendRef, container: [property_map.md#0x4_property_map_PropertyMap](PropertyMap)) {
    <b>let</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer) = [../../aptos-framework/doc/object.md#0x1_object_generate_signer_for_extending](object::generate_signer_for_extending)(ref);
    <b>move_to</b>(&[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), container);
}
</code></pre>



</details>

<a id="0x4_property_map_burn"></a>

## Function `burn`

Burns the entire property map


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_burn](burn)(ref: [property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_burn](burn)(ref: [property_map.md#0x4_property_map_MutatorRef](MutatorRef)) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>move_from</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;(ref.self);
}
</code></pre>



</details>

<a id="0x4_property_map_prepare_input"></a>

## Function `prepare_input`

Helper for external entry functions to produce a valid container for property values.


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_prepare_input](prepare_input)(keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;): [property_map.md#0x4_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_prepare_input](prepare_input)(
    keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;,
): [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> length = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&keys);
    <b>assert</b>!([property_map.md#0x4_property_map_length](length) &lt;= [property_map.md#0x4_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_ETOO_MANY_PROPERTIES](ETOO_MANY_PROPERTIES)));
    <b>assert</b>!(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&values), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_EKEY_VALUE_COUNT_MISMATCH](EKEY_VALUE_COUNT_MISMATCH)));
    <b>assert</b>!(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&types), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_EKEY_TYPE_COUNT_MISMATCH](EKEY_TYPE_COUNT_MISMATCH)));

    <b>let</b> container = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_create](simple_map::create)&lt;String, [property_map.md#0x4_property_map_PropertyValue](PropertyValue)&gt;();
    <b>while</b> (![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(&keys)) {
        <b>let</b> key = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> keys);
        <b>assert</b>!(
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&key) &lt;= [property_map.md#0x4_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH),
            [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_EPROPERTY_MAP_KEY_TOO_LONG](EPROPERTY_MAP_KEY_TOO_LONG)),
        );

        <b>let</b> value = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> values);
        <b>let</b> type = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> types);

        <b>let</b> new_type = [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type);
        [property_map.md#0x4_property_map_validate_type](validate_type)(new_type, value);

        [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_add](simple_map::add)(&<b>mut</b> container, key, [property_map.md#0x4_property_map_PropertyValue](PropertyValue) { value, type: new_type });
    };

    [property_map.md#0x4_property_map_PropertyMap](PropertyMap) { inner: container }
}
</code></pre>



</details>

<a id="0x4_property_map_to_external_type"></a>

## Function `to_external_type`

Maps <code>String</code> representation of types from their <code>u8</code> representation


<pre><code><b>fun</b> [property_map.md#0x4_property_map_to_external_type](to_external_type)(type: u8): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_to_external_type](to_external_type)(type: u8): String {
    <b>if</b> (type == [property_map.md#0x4_property_map_BOOL](BOOL)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"bool")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U8](U8)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u8")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U16](U16)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u16")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U32](U32)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u32")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U64](U64)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u64")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U128](U128)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u128")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U256](U256)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u256")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_ADDRESS](ADDRESS)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"<b>address</b>")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_BYTE_VECTOR](BYTE_VECTOR)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;")
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_STRING](STRING)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)")
    } <b>else</b> {
        <b>abort</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_ETYPE_INVALID](ETYPE_INVALID)))
    }
}
</code></pre>



</details>

<a id="0x4_property_map_to_internal_type"></a>

## Function `to_internal_type`

Maps the <code>String</code> representation of types to <code>u8</code>


<pre><code><b>fun</b> [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type: String): u8 {
    <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"bool")) {
        [property_map.md#0x4_property_map_BOOL](BOOL)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u8")) {
        [property_map.md#0x4_property_map_U8](U8)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u16")) {
        [property_map.md#0x4_property_map_U16](U16)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u32")) {
        [property_map.md#0x4_property_map_U32](U32)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u64")) {
        [property_map.md#0x4_property_map_U64](U64)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u128")) {
        [property_map.md#0x4_property_map_U128](U128)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u256")) {
        [property_map.md#0x4_property_map_U256](U256)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"<b>address</b>")) {
        [property_map.md#0x4_property_map_ADDRESS](ADDRESS)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;")) {
        [property_map.md#0x4_property_map_BYTE_VECTOR](BYTE_VECTOR)
    } <b>else</b> <b>if</b> (type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)")) {
        [property_map.md#0x4_property_map_STRING](STRING)
    } <b>else</b> {
        <b>abort</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_ETYPE_INVALID](ETYPE_INVALID)))
    }
}
</code></pre>



</details>

<a id="0x4_property_map_type_info_to_internal_type"></a>

## Function `type_info_to_internal_type`

Maps Move type to <code>u8</code> representation


<pre><code><b>fun</b> [property_map.md#0x4_property_map_type_info_to_internal_type](type_info_to_internal_type)&lt;T&gt;(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_type_info_to_internal_type](type_info_to_internal_type)&lt;T&gt;(): u8 {
    <b>let</b> type = [../../aptos-framework/../aptos-stdlib/doc/type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;T&gt;();
    [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type)
}
</code></pre>



</details>

<a id="0x4_property_map_validate_type"></a>

## Function `validate_type`

Validates property value type against its expected type


<pre><code><b>fun</b> [property_map.md#0x4_property_map_validate_type](validate_type)(type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_validate_type](validate_type)(type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) {
    <b>if</b> (type == [property_map.md#0x4_property_map_BOOL](BOOL)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_bool](from_bcs::to_bool)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U8](U8)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u8](from_bcs::to_u8)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U16](U16)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u16](from_bcs::to_u16)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U32](U32)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u32](from_bcs::to_u32)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U64](U64)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u64](from_bcs::to_u64)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U128](U128)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u128](from_bcs::to_u128)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_U256](U256)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u256](from_bcs::to_u256)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_ADDRESS](ADDRESS)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_address](from_bcs::to_address)(value);
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_BYTE_VECTOR](BYTE_VECTOR)) {
        // nothing <b>to</b> validate...
    } <b>else</b> <b>if</b> (type == [property_map.md#0x4_property_map_STRING](STRING)) {
        [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_string](from_bcs::to_string)(value);
    } <b>else</b> {
        <b>abort</b> ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_ETYPE_MISMATCH](ETYPE_MISMATCH)))
    };
}
</code></pre>



</details>

<a id="0x4_property_map_generate_mutator_ref"></a>

## Function `generate_mutator_ref`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_generate_mutator_ref](generate_mutator_ref)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef)): [property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_generate_mutator_ref](generate_mutator_ref)(ref: &ConstructorRef): [property_map.md#0x4_property_map_MutatorRef](MutatorRef) {
    [property_map.md#0x4_property_map_MutatorRef](MutatorRef) { self: [../../aptos-framework/doc/object.md#0x1_object_address_from_constructor_ref](object::address_from_constructor_ref)(ref) }
}
</code></pre>



</details>

<a id="0x4_property_map_contains_key"></a>

## Function `contains_key`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_contains_key](contains_key)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_contains_key](contains_key)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): bool <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_contains_key](simple_map::contains_key)(&[property_map.md#0x4_property_map](property_map).inner, key)
}
</code></pre>



</details>

<a id="0x4_property_map_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_length](length)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_length](length)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;): u64 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_length](simple_map::length)(&[property_map.md#0x4_property_map](property_map).inner)
}
</code></pre>



</details>

<a id="0x4_property_map_read"></a>

## Function `read`

Read the property and get it's external type in it's bcs encoded format

The preferred method is to use <code>read_&lt;type&gt;</code> where the type is already known.


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read](read)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read](read)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): (String, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;([../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)([../../aptos-framework/doc/object.md#0x1_object](object)));
    <b>let</b> property_value = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_borrow](simple_map::borrow)(&[property_map.md#0x4_property_map](property_map).inner, key);
    <b>let</b> new_type = [property_map.md#0x4_property_map_to_external_type](to_external_type)(property_value.type);
    (new_type, property_value.value)
}
</code></pre>



</details>

<a id="0x4_property_map_assert_exists"></a>

## Function `assert_exists`



<pre><code><b>fun</b> [property_map.md#0x4_property_map_assert_exists](assert_exists)([../../aptos-framework/doc/object.md#0x1_object](object): <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_assert_exists](assert_exists)([../../aptos-framework/doc/object.md#0x1_object](object): <b>address</b>) {
    <b>assert</b>!(
        <b>exists</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;([../../aptos-framework/doc/object.md#0x1_object](object)),
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([property_map.md#0x4_property_map_EPROPERTY_MAP_DOES_NOT_EXIST](EPROPERTY_MAP_DOES_NOT_EXIST)),
    );
}
</code></pre>



</details>

<a id="0x4_property_map_read_typed"></a>

## Function `read_typed`

Read a type and verify that the type is correct


<pre><code><b>fun</b> [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T: key, V&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T: key, V&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> (type, value) = [property_map.md#0x4_property_map_read](read)([../../aptos-framework/doc/object.md#0x1_object](object), key);
    <b>assert</b>!(
        type == [../../aptos-framework/../aptos-stdlib/doc/type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;V&gt;(),
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x4_property_map_ETYPE_MISMATCH](ETYPE_MISMATCH)),
    );
    value
}
</code></pre>



</details>

<a id="0x4_property_map_read_bool"></a>

## Function `read_bool`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_bool](read_bool)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_bool](read_bool)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): bool <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, bool&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_bool](from_bcs::to_bool)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u8"></a>

## Function `read_u8`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u8](read_u8)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u8](read_u8)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u8 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u8&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u8](from_bcs::to_u8)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u16"></a>

## Function `read_u16`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u16](read_u16)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u16](read_u16)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u16 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u16&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u16](from_bcs::to_u16)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u32"></a>

## Function `read_u32`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u32](read_u32)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u32](read_u32)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u32 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u32&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u32](from_bcs::to_u32)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u64"></a>

## Function `read_u64`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u64](read_u64)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u64](read_u64)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u64 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u64&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u64](from_bcs::to_u64)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u128"></a>

## Function `read_u128`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u128](read_u128)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u128](read_u128)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u128 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u128&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u128](from_bcs::to_u128)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_u256"></a>

## Function `read_u256`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u256](read_u256)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_u256](read_u256)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): u256 <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, u256&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u256](from_bcs::to_u256)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_address"></a>

## Function `read_address`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_address](read_address)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_address](read_address)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): <b>address</b> <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, <b>address</b>&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_address](from_bcs::to_address)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_bytes"></a>

## Function `read_bytes`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_bytes](read_bytes)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_bytes](read_bytes)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_bytes](from_bcs::to_bytes)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_read_string"></a>

## Function `read_string`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_string](read_string)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &[../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;, key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_read_string](read_string)&lt;T: key&gt;([../../aptos-framework/doc/object.md#0x1_object](object): &Object&lt;T&gt;, key: &String): String <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> value = [property_map.md#0x4_property_map_read_typed](read_typed)&lt;T, String&gt;([../../aptos-framework/doc/object.md#0x1_object](object), key);
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_string](from_bcs::to_string)(value)
}
</code></pre>



</details>

<a id="0x4_property_map_add"></a>

## Function `add`

Add a property, already bcs encoded as a <code>[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_add](add)(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_add](add)(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: String, type: String, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> new_type = [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type);
    [property_map.md#0x4_property_map_validate_type](validate_type)(new_type, value);
    [property_map.md#0x4_property_map_add_internal](add_internal)(ref, key, new_type, value);
}
</code></pre>



</details>

<a id="0x4_property_map_add_typed"></a>

## Function `add_typed`

Add a property that isn't already encoded as a <code>[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_add_typed](add_typed)&lt;T: drop&gt;(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_add_typed](add_typed)&lt;T: drop&gt;(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: String, value: T) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> type = [property_map.md#0x4_property_map_type_info_to_internal_type](type_info_to_internal_type)&lt;T&gt;();
    [property_map.md#0x4_property_map_add_internal](add_internal)(ref, key, type, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(&value));
}
</code></pre>



</details>

<a id="0x4_property_map_add_internal"></a>

## Function `add_internal`



<pre><code><b>fun</b> [property_map.md#0x4_property_map_add_internal](add_internal)(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_add_internal](add_internal)(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: String, type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)(ref.self);
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global_mut</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;(ref.self);
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_add](simple_map::add)(&<b>mut</b> [property_map.md#0x4_property_map](property_map).inner, key, [property_map.md#0x4_property_map_PropertyValue](PropertyValue) { type, value });
}
</code></pre>



</details>

<a id="0x4_property_map_update"></a>

## Function `update`

Updates a property in place already bcs encoded


<pre><code><b>public</b> <b>fun</b> <b>update</b>(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>update</b>(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: &String, type: String, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> new_type = [property_map.md#0x4_property_map_to_internal_type](to_internal_type)(type);
    [property_map.md#0x4_property_map_validate_type](validate_type)(new_type, value);
    [property_map.md#0x4_property_map_update_internal](update_internal)(ref, key, new_type, value);
}
</code></pre>



</details>

<a id="0x4_property_map_update_typed"></a>

## Function `update_typed`

Updates a property in place that is not already bcs encoded


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_update_typed](update_typed)&lt;T: drop&gt;(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_update_typed](update_typed)&lt;T: drop&gt;(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: &String, value: T) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>let</b> type = [property_map.md#0x4_property_map_type_info_to_internal_type](type_info_to_internal_type)&lt;T&gt;();
    [property_map.md#0x4_property_map_update_internal](update_internal)(ref, key, type, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(&value));
}
</code></pre>



</details>

<a id="0x4_property_map_update_internal"></a>

## Function `update_internal`



<pre><code><b>fun</b> [property_map.md#0x4_property_map_update_internal](update_internal)(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [property_map.md#0x4_property_map_update_internal](update_internal)(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: &String, type: u8, value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)(ref.self);
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global_mut</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;(ref.self);
    <b>let</b> old_value = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_borrow_mut](simple_map::borrow_mut)(&<b>mut</b> [property_map.md#0x4_property_map](property_map).inner, key);
    *old_value = [property_map.md#0x4_property_map_PropertyValue](PropertyValue) { type, value };
}
</code></pre>



</details>

<a id="0x4_property_map_remove"></a>

## Function `remove`

Removes a property from the map, ensuring that it does in fact exist


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_remove](remove)(ref: &[property_map.md#0x4_property_map_MutatorRef](property_map::MutatorRef), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x4_property_map_remove](remove)(ref: &[property_map.md#0x4_property_map_MutatorRef](MutatorRef), key: &String) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x4_property_map_assert_exists](assert_exists)(ref.self);
    <b>let</b> [property_map.md#0x4_property_map](property_map) = <b>borrow_global_mut</b>&lt;[property_map.md#0x4_property_map_PropertyMap](PropertyMap)&gt;(ref.self);
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_remove](simple_map::remove)(&<b>mut</b> [property_map.md#0x4_property_map](property_map).inner, key);
}
</code></pre>



</details>

<a id="0x4_property_map_assert_end_to_end_input"></a>

## Function `assert_end_to_end_input`



<pre><code><b>fun</b> [property_map.md#0x4_property_map_assert_end_to_end_input](assert_end_to_end_input)([../../aptos-framework/doc/object.md#0x1_object](object): [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;[../../aptos-framework/doc/object.md#0x1_object_ObjectCore](object::ObjectCore)&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [property_map.md#0x4_property_map_assert_end_to_end_input](assert_end_to_end_input)([../../aptos-framework/doc/object.md#0x1_object](object): Object&lt;ObjectCore&gt;) <b>acquires</b> [property_map.md#0x4_property_map_PropertyMap](PropertyMap) {
    <b>assert</b>!([property_map.md#0x4_property_map_read_bool](read_bool)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"bool")), 0);
    <b>assert</b>!([property_map.md#0x4_property_map_read_u8](read_u8)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u8")) == 0x12, 1);
    <b>assert</b>!([property_map.md#0x4_property_map_read_u16](read_u16)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u16")) == 0x1234, 2);
    <b>assert</b>!([property_map.md#0x4_property_map_read_u32](read_u32)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u32")) == 0x12345678, 3);
    <b>assert</b>!([property_map.md#0x4_property_map_read_u64](read_u64)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u64")) == 0x1234567812345678, 4);
    <b>assert</b>!([property_map.md#0x4_property_map_read_u128](read_u128)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u128")) == 0x12345678123456781234567812345678, 5);
    <b>assert</b>!(
        [property_map.md#0x4_property_map_read_u256](read_u256)(
            &[../../aptos-framework/doc/object.md#0x1_object](object),
            &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u256")
        ) == 0x1234567812345678123456781234567812345678123456781234567812345678,
        6
    );
    <b>assert</b>!([property_map.md#0x4_property_map_read_bytes](read_bytes)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;")) == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)[0x01], 7);
    <b>assert</b>!([property_map.md#0x4_property_map_read_string](read_string)(&[../../aptos-framework/doc/object.md#0x1_object](object), &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)")) == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"a"), 8);

    <b>assert</b>!([property_map.md#0x4_property_map_length](length)(&[../../aptos-framework/doc/object.md#0x1_object](object)) == 9, 9);
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
