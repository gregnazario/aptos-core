
<a id="0x3_property_map"></a>

# Module `0x3::property_map`

PropertyMap is a specialization of SimpleMap for Tokens.
It maps a String key to a PropertyValue that consists of type (string) and value (vector<u8>)
It provides basic on-chain serialization of primitive and string to property value with type information
It also supports deserializing property value to it original type.


-  [Struct `PropertyMap`](#0x3_property_map_PropertyMap)
-  [Struct `PropertyValue`](#0x3_property_map_PropertyValue)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_property_map_new)
-  [Function `new_with_key_and_property_value`](#0x3_property_map_new_with_key_and_property_value)
-  [Function `empty`](#0x3_property_map_empty)
-  [Function `contains_key`](#0x3_property_map_contains_key)
-  [Function `add`](#0x3_property_map_add)
-  [Function `length`](#0x3_property_map_length)
-  [Function `borrow`](#0x3_property_map_borrow)
-  [Function `keys`](#0x3_property_map_keys)
-  [Function `types`](#0x3_property_map_types)
-  [Function `values`](#0x3_property_map_values)
-  [Function `read_string`](#0x3_property_map_read_string)
-  [Function `read_u8`](#0x3_property_map_read_u8)
-  [Function `read_u64`](#0x3_property_map_read_u64)
-  [Function `read_address`](#0x3_property_map_read_address)
-  [Function `read_u128`](#0x3_property_map_read_u128)
-  [Function `read_bool`](#0x3_property_map_read_bool)
-  [Function `borrow_value`](#0x3_property_map_borrow_value)
-  [Function `borrow_type`](#0x3_property_map_borrow_type)
-  [Function `remove`](#0x3_property_map_remove)
-  [Function `update_property_map`](#0x3_property_map_update_property_map)
-  [Function `update_property_value`](#0x3_property_map_update_property_value)
-  [Function `create_property_value_raw`](#0x3_property_map_create_property_value_raw)
-  [Function `create_property_value`](#0x3_property_map_create_property_value)
-  [Specification](#@Specification_1)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `new_with_key_and_property_value`](#@Specification_1_new_with_key_and_property_value)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `contains_key`](#@Specification_1_contains_key)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `keys`](#@Specification_1_keys)
    -  [Function `types`](#@Specification_1_types)
    -  [Function `values`](#@Specification_1_values)
    -  [Function `read_string`](#@Specification_1_read_string)
    -  [Function `read_u8`](#@Specification_1_read_u8)
    -  [Function `read_u64`](#@Specification_1_read_u64)
    -  [Function `read_address`](#@Specification_1_read_address)
    -  [Function `read_u128`](#@Specification_1_read_u128)
    -  [Function `read_bool`](#@Specification_1_read_bool)
    -  [Function `borrow_value`](#@Specification_1_borrow_value)
    -  [Function `borrow_type`](#@Specification_1_borrow_type)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `update_property_map`](#@Specification_1_update_property_map)
    -  [Function `update_property_value`](#@Specification_1_update_property_value)
    -  [Function `create_property_value_raw`](#@Specification_1_create_property_value_raw)
    -  [Function `create_property_value`](#@Specification_1_create_property_value)


<pre><code><b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs](0x1::bcs);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs](0x1::from_bcs);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map](0x1::simple_map);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/type_info.md#0x1_type_info](0x1::type_info);
</code></pre>



<a id="0x3_property_map_PropertyMap"></a>

## Struct `PropertyMap`



<pre><code><b>struct</b> [property_map.md#0x3_property_map_PropertyMap](PropertyMap) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>map: [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_property_map_PropertyValue"></a>

## Struct `PropertyValue`



<pre><code><b>struct</b> [property_map.md#0x3_property_map_PropertyValue](PropertyValue) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x3_property_map_EKEY_AREADY_EXIST_IN_PROPERTY_MAP"></a>

The property key already exists


<pre><code><b>const</b> [property_map.md#0x3_property_map_EKEY_AREADY_EXIST_IN_PROPERTY_MAP](EKEY_AREADY_EXIST_IN_PROPERTY_MAP): u64 = 1;
</code></pre>



<a id="0x3_property_map_EKEY_COUNT_NOT_MATCH_TYPE_COUNT"></a>

Property key and type count don't match


<pre><code><b>const</b> [property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_TYPE_COUNT](EKEY_COUNT_NOT_MATCH_TYPE_COUNT): u64 = 5;
</code></pre>



<a id="0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT"></a>

Property key and value count don't match


<pre><code><b>const</b> [property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT](EKEY_COUNT_NOT_MATCH_VALUE_COUNT): u64 = 4;
</code></pre>



<a id="0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG"></a>

The name (key) of the property is too long


<pre><code><b>const</b> [property_map.md#0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG](EPROPERTY_MAP_NAME_TOO_LONG): u64 = 7;
</code></pre>



<a id="0x3_property_map_EPROPERTY_NOT_EXIST"></a>

The property doesn't exist


<pre><code><b>const</b> [property_map.md#0x3_property_map_EPROPERTY_NOT_EXIST](EPROPERTY_NOT_EXIST): u64 = 3;
</code></pre>



<a id="0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT"></a>

The number of property exceeds the limit


<pre><code><b>const</b> [property_map.md#0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT](EPROPERTY_NUMBER_EXCEED_LIMIT): u64 = 2;
</code></pre>



<a id="0x3_property_map_ETYPE_NOT_MATCH"></a>

Property type doesn't match


<pre><code><b>const</b> [property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH): u64 = 6;
</code></pre>



<a id="0x3_property_map_MAX_PROPERTY_MAP_SIZE"></a>

The maximal number of property that can be stored in property map


<pre><code><b>const</b> [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE): u64 = 1000;
</code></pre>



<a id="0x3_property_map_MAX_PROPERTY_NAME_LENGTH"></a>



<pre><code><b>const</b> [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH): u64 = 128;
</code></pre>



<a id="0x3_property_map_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new](new)(keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;, types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new](new)(
    keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;,
    types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;
): [property_map.md#0x3_property_map_PropertyMap](PropertyMap) {
    <b>let</b> length = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&keys);
    <b>assert</b>!([property_map.md#0x3_property_map_length](length) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT](EPROPERTY_NUMBER_EXCEED_LIMIT)));
    <b>assert</b>!(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&values), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT](EKEY_COUNT_NOT_MATCH_VALUE_COUNT)));
    <b>assert</b>!(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&types), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_TYPE_COUNT](EKEY_COUNT_NOT_MATCH_TYPE_COUNT)));

    <b>let</b> properties = [property_map.md#0x3_property_map_empty](empty)();

    <b>let</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> key = *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&keys, i);
        <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&key) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG](EPROPERTY_MAP_NAME_TOO_LONG)));
        [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_add](simple_map::add)(
            &<b>mut</b> properties.map,
            key,
            [property_map.md#0x3_property_map_PropertyValue](PropertyValue) { value: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&values, i), type: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&types, i) }
        );
        i = i + 1;
    };
    properties
}
</code></pre>



</details>

<a id="0x3_property_map_new_with_key_and_property_value"></a>

## Function `new_with_key_and_property_value`

Create property map directly from key and property value


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new_with_key_and_property_value](new_with_key_and_property_value)(keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new_with_key_and_property_value](new_with_key_and_property_value)(
    keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](PropertyValue)&gt;
): [property_map.md#0x3_property_map_PropertyMap](PropertyMap) {
    <b>let</b> length = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&keys);
    <b>assert</b>!([property_map.md#0x3_property_map_length](length) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT](EPROPERTY_NUMBER_EXCEED_LIMIT)));
    <b>assert</b>!(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&values), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT](EKEY_COUNT_NOT_MATCH_VALUE_COUNT)));

    <b>let</b> properties = [property_map.md#0x3_property_map_empty](empty)();

    <b>let</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> key = *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&keys, i);
        <b>let</b> val = *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&values, i);
        <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&key) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG](EPROPERTY_MAP_NAME_TOO_LONG)));
        [property_map.md#0x3_property_map_add](add)(&<b>mut</b> properties, key, val);
        i = i + 1;
    };
    properties
}
</code></pre>



</details>

<a id="0x3_property_map_empty"></a>

## Function `empty`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_empty](empty)(): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_empty](empty)(): [property_map.md#0x3_property_map_PropertyMap](PropertyMap) {
    [property_map.md#0x3_property_map_PropertyMap](PropertyMap) {
        map: [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_create](simple_map::create)&lt;String, [property_map.md#0x3_property_map_PropertyValue](PropertyValue)&gt;(),
    }
}
</code></pre>



</details>

<a id="0x3_property_map_contains_key"></a>

## Function `contains_key`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_contains_key](contains_key)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_contains_key](contains_key)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): bool {
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_contains_key](simple_map::contains_key)(&map.map, key)
}
</code></pre>



</details>

<a id="0x3_property_map_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_add](add)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_add](add)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: String, value: [property_map.md#0x3_property_map_PropertyValue](PropertyValue)) {
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(&key) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([property_map.md#0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG](EPROPERTY_MAP_NAME_TOO_LONG)));
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_length](simple_map::length)(&map.map) &lt; [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT](EPROPERTY_NUMBER_EXCEED_LIMIT)));
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_add](simple_map::add)(&<b>mut</b> map.map, key, value);
}
</code></pre>



</details>

<a id="0x3_property_map_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_length](length)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_length](length)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap)): u64 {
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_length](simple_map::length)(&map.map)
}
</code></pre>



</details>

<a id="0x3_property_map_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow](borrow)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow](borrow)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): &[property_map.md#0x3_property_map_PropertyValue](PropertyValue) {
    <b>let</b> found = [property_map.md#0x3_property_map_contains_key](contains_key)(map, key);
    <b>assert</b>!(found, [property_map.md#0x3_property_map_EPROPERTY_NOT_EXIST](EPROPERTY_NOT_EXIST));
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_borrow](simple_map::borrow)(&map.map, key)
}
</code></pre>



</details>

<a id="0x3_property_map_keys"></a>

## Function `keys`

Return all the keys in the property map in the order they are added.


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_keys](keys)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_keys](keys)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt; {
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_keys](simple_map::keys)(&map.map)
}
</code></pre>



</details>

<a id="0x3_property_map_types"></a>

## Function `types`

Return the types of all properties in the property map in the order they are added.


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_types](types)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_types](types)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt; {
    [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_map_ref](vector::map_ref)(&[../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_values](simple_map::values)(&map.map), |v| {
        <b>let</b> v: &[property_map.md#0x3_property_map_PropertyValue](PropertyValue) = v;
        v.type
    })
}
</code></pre>



</details>

<a id="0x3_property_map_values"></a>

## Function `values`

Return the values of all properties in the property map in the order they are added.


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_values](values)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_values](values)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt; {
    [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_map_ref](vector::map_ref)(&[../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_values](simple_map::values)(&map.map), |v| {
        <b>let</b> v: &[property_map.md#0x3_property_map_PropertyValue](PropertyValue) = v;
        v.value
    })
}
</code></pre>



</details>

<a id="0x3_property_map_read_string"></a>

## Function `read_string`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_string](read_string)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_string](read_string)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): String {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_string](from_bcs::to_string)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_read_u8"></a>

## Function `read_u8`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u8](read_u8)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u8](read_u8)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): u8 {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u8"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u8](from_bcs::to_u8)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_read_u64"></a>

## Function `read_u64`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u64](read_u64)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u64](read_u64)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): u64 {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u64"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u64](from_bcs::to_u64)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_read_address"></a>

## Function `read_address`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_address](read_address)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_address](read_address)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): <b>address</b> {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"<b>address</b>"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_address](from_bcs::to_address)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_read_u128"></a>

## Function `read_u128`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u128](read_u128)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u128](read_u128)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): u128 {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u128"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_u128](from_bcs::to_u128)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_read_bool"></a>

## Function `read_bool`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_bool](read_bool)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_bool](read_bool)(map: &[property_map.md#0x3_property_map_PropertyMap](PropertyMap), key: &String): bool {
    <b>let</b> prop = [property_map.md#0x3_property_map_borrow](borrow)(map, key);
    <b>assert</b>!(prop.type == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"bool"), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_ETYPE_NOT_MATCH](ETYPE_NOT_MATCH)));
    [../../aptos-framework/../aptos-stdlib/doc/from_bcs.md#0x1_from_bcs_to_bool](from_bcs::to_bool)(prop.value)
}
</code></pre>



</details>

<a id="0x3_property_map_borrow_value"></a>

## Function `borrow_value`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_value](borrow_value)(property: &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_value](borrow_value)(property: &[property_map.md#0x3_property_map_PropertyValue](PropertyValue)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    property.value
}
</code></pre>



</details>

<a id="0x3_property_map_borrow_type"></a>

## Function `borrow_type`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_type](borrow_type)(property: &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_type](borrow_type)(property: &[property_map.md#0x3_property_map_PropertyValue](PropertyValue)): String {
    property.type
}
</code></pre>



</details>

<a id="0x3_property_map_remove"></a>

## Function `remove`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_remove](remove)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_remove](remove)(
    map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](PropertyMap),
    key: &String
): (String, [property_map.md#0x3_property_map_PropertyValue](PropertyValue)) {
    <b>let</b> found = [property_map.md#0x3_property_map_contains_key](contains_key)(map, key);
    <b>assert</b>!(found, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([property_map.md#0x3_property_map_EPROPERTY_NOT_EXIST](EPROPERTY_NOT_EXIST)));
    [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_remove](simple_map::remove)(&<b>mut</b> map.map, key)
}
</code></pre>



</details>

<a id="0x3_property_map_update_property_map"></a>

## Function `update_property_map`

Update the property in the existing property map
Allow updating existing keys' value and add new key-value pairs


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_map](update_property_map)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;, types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_map](update_property_map)(
    map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](PropertyMap),
    keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
    values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;,
    types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;String&gt;,
) {
    <b>let</b> key_len = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&keys);
    <b>let</b> val_len = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&values);
    <b>let</b> typ_len = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&types);
    <b>assert</b>!(key_len == val_len, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT](EKEY_COUNT_NOT_MATCH_VALUE_COUNT)));
    <b>assert</b>!(key_len == typ_len, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([property_map.md#0x3_property_map_EKEY_COUNT_NOT_MATCH_TYPE_COUNT](EKEY_COUNT_NOT_MATCH_TYPE_COUNT)));

    <b>let</b> i = 0;
    <b>while</b> (i &lt; key_len) {
        <b>let</b> key = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&keys, i);
        <b>let</b> prop_val = [property_map.md#0x3_property_map_PropertyValue](PropertyValue) {
            value: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&values, i),
            type: *[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&types, i),
        };
        <b>if</b> ([property_map.md#0x3_property_map_contains_key](contains_key)(map, key)) {
            [property_map.md#0x3_property_map_update_property_value](update_property_value)(map, key, prop_val);
        } <b>else</b> {
            [property_map.md#0x3_property_map_add](add)(map, *key, prop_val);
        };
        i = i + 1;
    }
}
</code></pre>



</details>

<a id="0x3_property_map_update_property_value"></a>

## Function `update_property_value`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_value](update_property_value)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_value](update_property_value)(
    map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](PropertyMap),
    key: &String,
    value: [property_map.md#0x3_property_map_PropertyValue](PropertyValue)
) {
    <b>let</b> property_val = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_borrow_mut](simple_map::borrow_mut)(&<b>mut</b> map.map, key);
    *property_val = value;
}
</code></pre>



</details>

<a id="0x3_property_map_create_property_value_raw"></a>

## Function `create_property_value_raw`



<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value_raw](create_property_value_raw)(value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value_raw](create_property_value_raw)(
    value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;,
    type: String
): [property_map.md#0x3_property_map_PropertyValue](PropertyValue) {
    [property_map.md#0x3_property_map_PropertyValue](PropertyValue) {
        value,
        type,
    }
}
</code></pre>



</details>

<a id="0x3_property_map_create_property_value"></a>

## Function `create_property_value`

create a property value from generic type data


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value](create_property_value)&lt;T: <b>copy</b>&gt;(data: &T): [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value](create_property_value)&lt;T: <b>copy</b>&gt;(data: &T): [property_map.md#0x3_property_map_PropertyValue](PropertyValue) {
    <b>let</b> name = type_name&lt;T&gt;();
    <b>if</b> (
        name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"bool") ||
            name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u8") ||
            name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u64") ||
            name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"u128") ||
            name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"<b>address</b>") ||
            name == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)")
    ) {
        [property_map.md#0x3_property_map_create_property_value_raw](create_property_value_raw)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)&lt;T&gt;(data), name)
    } <b>else</b> {
        [property_map.md#0x3_property_map_create_property_value_raw](create_property_value_raw)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)&lt;T&gt;(data), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8](string::utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;"))
    }
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<pre><code><b>pragma</b> verify = <b>true</b>;
<b>pragma</b> aborts_if_is_strict;
<b>let</b> [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE) = 1000;
<b>let</b> [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH)  = 128;
</code></pre>



<a id="@Specification_1_new"></a>

### Function `new`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new](new)(keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;, types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> length = len(keys);
<b>aborts_if</b> !([property_map.md#0x3_property_map_length](length) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE));
<b>aborts_if</b> !(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(values));
<b>aborts_if</b> !(length == [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(types));
</code></pre>



<a id="@Specification_1_new_with_key_and_property_value"></a>

### Function `new_with_key_and_property_value`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_new_with_key_and_property_value](new_with_key_and_property_value)(keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)&gt;): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> length = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(keys);
<b>aborts_if</b> !([property_map.md#0x3_property_map_length](length) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE));
<b>aborts_if</b> !(length == len(values));
</code></pre>



<a id="@Specification_1_empty"></a>

### Function `empty`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_empty](empty)(): [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_contains_key"></a>

### Function `contains_key`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_contains_key](contains_key)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_add"></a>

### Function `add`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_add](add)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>




<pre><code><b>aborts_if</b> !([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_length](string::length)(key) &lt;= [property_map.md#0x3_property_map_MAX_PROPERTY_NAME_LENGTH](MAX_PROPERTY_NAME_LENGTH));
<b>aborts_if</b> !(![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key));
<b>aborts_if</b> !([../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_len](simple_map::spec_len)(map.map) &lt; [property_map.md#0x3_property_map_MAX_PROPERTY_MAP_SIZE](MAX_PROPERTY_MAP_SIZE));
</code></pre>



<a id="@Specification_1_length"></a>

### Function `length`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_length](length)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): u64
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow](borrow)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>




<pre><code><b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
</code></pre>



<a id="@Specification_1_keys"></a>

### Function `keys`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_keys](keys)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_types"></a>

### Function `types`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_types](types)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_values"></a>

### Function `values`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_values](values)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_read_string"></a>

### Function `read_string`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_string](read_string)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>


Check utf8 for correctness and whether equal
to <code>prop.type</code>


<pre><code><b>pragma</b> aborts_if_is_partial;
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)");
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)");
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;String&gt;(prop.value);
</code></pre>




<a id="0x3_property_map_spec_utf8"></a>


<pre><code><b>fun</b> [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(bytes: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): String {
   String{bytes}
}
</code></pre>



<a id="@Specification_1_read_u8"></a>

### Function `read_u8`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u8](read_u8)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u8
</code></pre>




<pre><code><b>let</b> str = b"u8";
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(str);
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(str);
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;u8&gt;(prop.value);
</code></pre>



<a id="@Specification_1_read_u64"></a>

### Function `read_u64`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u64](read_u64)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u64
</code></pre>




<pre><code><b>let</b> str = b"u64";
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(str);
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(str);
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;u64&gt;(prop.value);
</code></pre>



<a id="@Specification_1_read_address"></a>

### Function `read_address`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_address](read_address)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): <b>address</b>
</code></pre>




<pre><code><b>let</b> str = b"<b>address</b>";
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(str);
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(str);
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;<b>address</b>&gt;(prop.value);
</code></pre>



<a id="@Specification_1_read_u128"></a>

### Function `read_u128`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_u128](read_u128)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): u128
</code></pre>




<pre><code><b>let</b> str = b"u128";
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(str);
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(str);
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;u128&gt;(prop.value);
</code></pre>



<a id="@Specification_1_read_bool"></a>

### Function `read_bool`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_read_bool](read_bool)(map: &[property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): bool
</code></pre>




<pre><code><b>let</b> str = b"bool";
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(str);
<b>let</b> prop = [../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(map.map, key);
<b>aborts_if</b> prop.type != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(str);
<b>aborts_if</b> !aptos_std::from_bcs::deserializable&lt;bool&gt;(prop.value);
</code></pre>



<a id="@Specification_1_borrow_value"></a>

### Function `borrow_value`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_value](borrow_value)(property: &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_borrow_type"></a>

### Function `borrow_type`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_borrow_type](borrow_type)(property: &[property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_remove](remove)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): ([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>




<pre><code><b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
</code></pre>



<a id="@Specification_1_update_property_map"></a>

### Function `update_property_map`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_map](update_property_map)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), keys: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;, values: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;, types: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)&gt;)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> key_len = len(keys);
<b>let</b> val_len = len(values);
<b>let</b> typ_len = len(types);
<b>aborts_if</b> !(key_len == val_len);
<b>aborts_if</b> !(key_len == typ_len);
</code></pre>



<a id="@Specification_1_update_property_value"></a>

### Function `update_property_value`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_update_property_value](update_property_value)(map: &<b>mut</b> [property_map.md#0x3_property_map_PropertyMap](property_map::PropertyMap), key: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), value: [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue))
</code></pre>




<pre><code><b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(map.map, key);
</code></pre>



<a id="@Specification_1_create_property_value_raw"></a>

### Function `create_property_value_raw`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value_raw](create_property_value_raw)(value: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, type: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String)): [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_create_property_value"></a>

### Function `create_property_value`


<pre><code><b>public</b> <b>fun</b> [property_map.md#0x3_property_map_create_property_value](create_property_value)&lt;T: <b>copy</b>&gt;(data: &T): [property_map.md#0x3_property_map_PropertyValue](property_map::PropertyValue)
</code></pre>


Abort according to the code


<pre><code><b>let</b> name = type_name&lt;T&gt;();
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"bool");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"u8");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u8") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"u64");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u8") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u64") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"u128");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u8") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u64") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u128") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"<b>address</b>");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u8") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u64") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u128") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"<b>address</b>") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)");
<b>aborts_if</b> name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"bool") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u8") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u64") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"u128") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"<b>address</b>") &&
    name != [property_map.md#0x3_property_map_spec_utf8](spec_utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](0x1::string::String)") &&
    ![../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_spec_internal_check_utf8](string::spec_internal_check_utf8)(b"[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;");
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
