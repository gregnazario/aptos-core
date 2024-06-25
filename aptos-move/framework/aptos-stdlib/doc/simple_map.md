
<a id="0x1_simple_map"></a>

# Module `0x1::simple_map`

This module provides a solution for unsorted maps, that is it has the properties that
1) Keys point to Values
2) Each Key must be unique
3) A Key can be found within O(N) time
4) The keys are unsorted.
5) Adds and removals take O(N) time


-  [Struct `SimpleMap`](#0x1_simple_map_SimpleMap)
-  [Struct `Element`](#0x1_simple_map_Element)
-  [Constants](#@Constants_0)
-  [Function `length`](#0x1_simple_map_length)
-  [Function `new`](#0x1_simple_map_new)
-  [Function `new_from`](#0x1_simple_map_new_from)
-  [Function `create`](#0x1_simple_map_create)
-  [Function `borrow`](#0x1_simple_map_borrow)
-  [Function `borrow_mut`](#0x1_simple_map_borrow_mut)
-  [Function `contains_key`](#0x1_simple_map_contains_key)
-  [Function `destroy_empty`](#0x1_simple_map_destroy_empty)
-  [Function `add`](#0x1_simple_map_add)
-  [Function `add_all`](#0x1_simple_map_add_all)
-  [Function `upsert`](#0x1_simple_map_upsert)
-  [Function `keys`](#0x1_simple_map_keys)
-  [Function `values`](#0x1_simple_map_values)
-  [Function `to_vec_pair`](#0x1_simple_map_to_vec_pair)
-  [Function `destroy`](#0x1_simple_map_destroy)
-  [Function `remove`](#0x1_simple_map_remove)
-  [Function `find`](#0x1_simple_map_find)
-  [Specification](#@Specification_1)
    -  [Struct `SimpleMap`](#@Specification_1_SimpleMap)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `new_from`](#@Specification_1_new_from)
    -  [Function `create`](#@Specification_1_create)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `contains_key`](#@Specification_1_contains_key)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `add_all`](#@Specification_1_add_all)
    -  [Function `upsert`](#@Specification_1_upsert)
    -  [Function `keys`](#@Specification_1_keys)
    -  [Function `values`](#@Specification_1_values)
    -  [Function `to_vec_pair`](#@Specification_1_to_vec_pair)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `find`](#@Specification_1_find)


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [../../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_simple_map_SimpleMap"></a>

## Struct `SimpleMap`



<pre><code><b>struct</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>data: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[simple_map.md#0x1_simple_map_Element](simple_map::Element)&lt;Key, Value&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_simple_map_Element"></a>

## Struct `Element`



<pre><code><b>struct</b> [simple_map.md#0x1_simple_map_Element](Element)&lt;Key, Value&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>key: Key</code>
</dt>
<dd>

</dd>
<dt>
<code>value: Value</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_simple_map_EKEY_ALREADY_EXISTS"></a>

Map key already exists


<pre><code><b>const</b> [simple_map.md#0x1_simple_map_EKEY_ALREADY_EXISTS](EKEY_ALREADY_EXISTS): u64 = 1;
</code></pre>



<a id="0x1_simple_map_EKEY_NOT_FOUND"></a>

Map key is not found


<pre><code><b>const</b> [simple_map.md#0x1_simple_map_EKEY_NOT_FOUND](EKEY_NOT_FOUND): u64 = 2;
</code></pre>



<a id="0x1_simple_map_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_length](length)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_length](length)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;): u64 {
    [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&map.data)
}
</code></pre>



</details>

<a id="0x1_simple_map_new"></a>

## Function `new`

Create an empty SimpleMap.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new](new)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new](new)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt; {
    [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap) {
        data: [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)(),
    }
}
</code></pre>



</details>

<a id="0x1_simple_map_new_from"></a>

## Function `new_from`

Create a SimpleMap from a vector of keys and values. The keys must be unique.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new_from](new_from)&lt;Key: store, Value: store&gt;(keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new_from](new_from)&lt;Key: store, Value: store&gt;(
    keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;,
    values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;,
): [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt; {
    <b>let</b> map = [simple_map.md#0x1_simple_map_new](new)();
    [simple_map.md#0x1_simple_map_add_all](add_all)(&<b>mut</b> map, keys, values);
    map
}
</code></pre>



</details>

<a id="0x1_simple_map_create"></a>

## Function `create`

Create an empty SimpleMap.
This function is deprecated, use <code>new</code> instead.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_create](create)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_create](create)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt; {
    [simple_map.md#0x1_simple_map_new](new)()
}
</code></pre>



</details>

<a id="0x1_simple_map_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow](borrow)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): &Value
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow](borrow)&lt;Key: store, Value: store&gt;(
    map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: &Key,
): &Value {
    <b>let</b> maybe_idx = [simple_map.md#0x1_simple_map_find](find)(map, key);
    <b>assert</b>!([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&maybe_idx), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([simple_map.md#0x1_simple_map_EKEY_NOT_FOUND](EKEY_NOT_FOUND)));
    <b>let</b> idx = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> maybe_idx);
    &[../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&map.data, idx).value
}
</code></pre>



</details>

<a id="0x1_simple_map_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow_mut](borrow_mut)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): &<b>mut</b> Value
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow_mut](borrow_mut)&lt;Key: store, Value: store&gt;(
    map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: &Key,
): &<b>mut</b> Value {
    <b>let</b> maybe_idx = [simple_map.md#0x1_simple_map_find](find)(map, key);
    <b>assert</b>!([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&maybe_idx), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([simple_map.md#0x1_simple_map_EKEY_NOT_FOUND](EKEY_NOT_FOUND)));
    <b>let</b> idx = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> maybe_idx);
    &<b>mut</b> [../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut](vector::borrow_mut)(&<b>mut</b> map.data, idx).value
}
</code></pre>



</details>

<a id="0x1_simple_map_contains_key"></a>

## Function `contains_key`



<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_contains_key](contains_key)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_contains_key](contains_key)&lt;Key: store, Value: store&gt;(
    map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: &Key,
): bool {
    <b>let</b> maybe_idx = [simple_map.md#0x1_simple_map_find](find)(map, key);
    [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&maybe_idx)
}
</code></pre>



</details>

<a id="0x1_simple_map_destroy_empty"></a>

## Function `destroy_empty`



<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_destroy_empty](destroy_empty)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_destroy_empty](destroy_empty)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;) {
    <b>let</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap) { data } = map;
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(data);
}
</code></pre>



</details>

<a id="0x1_simple_map_add"></a>

## Function `add`

Add a key/value pair to the map. The key must not already exist.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add](add)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: Key, value: Value)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add](add)&lt;Key: store, Value: store&gt;(
    map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: Key,
    value: Value,
) {
    <b>let</b> maybe_idx = [simple_map.md#0x1_simple_map_find](find)(map, &key);
    <b>assert</b>!([../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(&maybe_idx), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([simple_map.md#0x1_simple_map_EKEY_ALREADY_EXISTS](EKEY_ALREADY_EXISTS)));

    [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> map.data, [simple_map.md#0x1_simple_map_Element](Element) { key, value });
}
</code></pre>



</details>

<a id="0x1_simple_map_add_all"></a>

## Function `add_all`

Add multiple key/value pairs to the map. The keys must not already exist.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add_all](add_all)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add_all](add_all)&lt;Key: store, Value: store&gt;(
    map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;,
    values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;,
) {
    [../../move-stdlib/doc/vector.md#0x1_vector_zip](vector::zip)(keys, values, |key, value| {
        [simple_map.md#0x1_simple_map_add](add)(map, key, value);
    });
}
</code></pre>



</details>

<a id="0x1_simple_map_upsert"></a>

## Function `upsert`

Insert key/value pair or update an existing key to a new value


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_upsert](upsert)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: Key, value: Value): ([../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;Key&gt;, [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;Value&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_upsert](upsert)&lt;Key: store, Value: store&gt;(
    map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: Key,
    value: Value
): (std::option::Option&lt;Key&gt;, std::option::Option&lt;Value&gt;) {
    <b>let</b> data = &<b>mut</b> map.data;
    <b>let</b> len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(data);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> element = [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(data, i);
        <b>if</b> (&element.key == &key) {
            [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(data, [simple_map.md#0x1_simple_map_Element](Element) { key, value });
            [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(data, i, len);
            <b>let</b> [simple_map.md#0x1_simple_map_Element](Element) { key, value } = [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(data);
            <b>return</b> (std::option::some(key), std::option::some(value))
        };
        i = i + 1;
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> map.data, [simple_map.md#0x1_simple_map_Element](Element) { key, value });
    (std::option::none(), std::option::none())
}
</code></pre>



</details>

<a id="0x1_simple_map_keys"></a>

## Function `keys`

Return all keys in the map. This requires keys to be copyable.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_keys](keys)&lt;Key: <b>copy</b>, Value&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_keys](keys)&lt;Key: <b>copy</b>, Value&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt; {
    [../../move-stdlib/doc/vector.md#0x1_vector_map_ref](vector::map_ref)(&map.data, |e| {
        <b>let</b> e: &[simple_map.md#0x1_simple_map_Element](Element)&lt;Key, Value&gt; = e;
        e.key
    })
}
</code></pre>



</details>

<a id="0x1_simple_map_values"></a>

## Function `values`

Return all values in the map. This requires values to be copyable.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_values](values)&lt;Key, Value: <b>copy</b>&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_values](values)&lt;Key, Value: <b>copy</b>&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt; {
    [../../move-stdlib/doc/vector.md#0x1_vector_map_ref](vector::map_ref)(&map.data, |e| {
        <b>let</b> e: &[simple_map.md#0x1_simple_map_Element](Element)&lt;Key, Value&gt; = e;
        e.value
    })
}
</code></pre>



</details>

<a id="0x1_simple_map_to_vec_pair"></a>

## Function `to_vec_pair`

Transform the map into two vectors with the keys and values respectively
Primarily used to destroy a map


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_to_vec_pair](to_vec_pair)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_to_vec_pair](to_vec_pair)&lt;Key: store, Value: store&gt;(
    map: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;) {
    <b>let</b> keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt; = [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)();
    <b>let</b> values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt; = [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)();
    <b>let</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap) { data } = map;
    [../../move-stdlib/doc/vector.md#0x1_vector_for_each](vector::for_each)(data, |e| {
        <b>let</b> [simple_map.md#0x1_simple_map_Element](Element) { key, value } = e;
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> keys, key);
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> values, value);
    });
    (keys, values)
}
</code></pre>



</details>

<a id="0x1_simple_map_destroy"></a>

## Function `destroy`

For maps that cannot be dropped this is a utility to destroy them
using lambdas to destroy the individual keys and values.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_destroy](destroy)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, dk: |Key|, dv: |Value|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [simple_map.md#0x1_simple_map_destroy](destroy)&lt;Key: store, Value: store&gt;(
    map: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    dk: |Key|,
    dv: |Value|
) {
    <b>let</b> (keys, values) = [simple_map.md#0x1_simple_map_to_vec_pair](to_vec_pair)(map);
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy](vector::destroy)(keys, |_k| dk(_k));
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy](vector::destroy)(values, |_v| dv(_v));
}
</code></pre>



</details>

<a id="0x1_simple_map_remove"></a>

## Function `remove`

Remove a key/value pair from the map. The key must exist.


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_remove](remove)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): (Key, Value)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_remove](remove)&lt;Key: store, Value: store&gt;(
    map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: &Key,
): (Key, Value) {
    <b>let</b> maybe_idx = [simple_map.md#0x1_simple_map_find](find)(map, key);
    <b>assert</b>!([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&maybe_idx), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([simple_map.md#0x1_simple_map_EKEY_NOT_FOUND](EKEY_NOT_FOUND)));
    <b>let</b> placement = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> maybe_idx);
    <b>let</b> [simple_map.md#0x1_simple_map_Element](Element) { key, value } = [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(&<b>mut</b> map.data, placement);
    (key, value)
}
</code></pre>



</details>

<a id="0x1_simple_map_find"></a>

## Function `find`



<pre><code><b>fun</b> [simple_map.md#0x1_simple_map_find](find)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [simple_map.md#0x1_simple_map_find](find)&lt;Key: store, Value: store&gt;(
    map: &[simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt;,
    key: &Key,
): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt; {
    <b>let</b> leng = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&map.data);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; leng) {
        <b>let</b> element = [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&map.data, i);
        <b>if</b> (&element.key == key) {
            <b>return</b> [../../move-stdlib/doc/option.md#0x1_option_some](option::some)(i)
        };
        i = i + 1;
    };
    [../../move-stdlib/doc/option.md#0x1_option_none](option::none)&lt;u64&gt;()
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_SimpleMap"></a>

### Struct `SimpleMap`


<pre><code><b>struct</b> [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;Key, Value&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<dl>
<dt>
<code>data: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;[simple_map.md#0x1_simple_map_Element](simple_map::Element)&lt;Key, Value&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>pragma</b> intrinsic = map,
    map_new = create,
    map_len = length,
    map_destroy_empty = destroy_empty,
    map_has_key = contains_key,
    map_add_no_override = add,
    map_del_return_key = remove,
    map_borrow = borrow,
    map_borrow_mut = borrow_mut,
    map_spec_get = spec_get,
    map_spec_set = spec_set,
    map_spec_del = spec_remove,
    map_spec_len = spec_len,
    map_spec_has_key = spec_contains_key;
</code></pre>



<a id="@Specification_1_length"></a>

### Function `length`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_length](length)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): u64
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_new"></a>

### Function `new`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new](new)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic;
<b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_len](spec_len)(result) == 0;
<b>ensures</b> [abstract] <b>forall</b> k: Key: ![simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(result, k);
</code></pre>



<a id="@Specification_1_new_from"></a>

### Function `new_from`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_new_from](new_from)&lt;Key: store, Value: store&gt;(keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic;
<b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_len](spec_len)(result) == len(keys);
<b>ensures</b> [abstract] <b>forall</b> k: Key: [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(result, k) &lt;==&gt; [../../move-stdlib/doc/vector.md#0x1_vector_spec_contains](vector::spec_contains)(keys, k);
<b>ensures</b> [abstract] <b>forall</b> i in 0..len(keys):
    [simple_map.md#0x1_simple_map_spec_get](spec_get)(result, [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(keys, i)) == [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(values, i);
</code></pre>



<a id="@Specification_1_create"></a>

### Function `create`


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_create](create)&lt;Key: store, Value: store&gt;(): [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow](borrow)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): &Value
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_borrow_mut](borrow_mut)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): &<b>mut</b> Value
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_contains_key"></a>

### Function `contains_key`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_contains_key](contains_key)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_destroy_empty](destroy_empty)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_add"></a>

### Function `add`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add](add)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: Key, value: Value)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_add_all"></a>

### Function `add_all`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_add_all](add_all)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, keys: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, values: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_upsert"></a>

### Function `upsert`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_upsert](upsert)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: Key, value: Value): ([../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;Key&gt;, [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;Value&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
<b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] ![simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(<b>old</b>(map), key) ==&gt; [../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(result_1);
<b>ensures</b> [abstract] ![simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(<b>old</b>(map), key) ==&gt; [../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(result_2);
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(map, key);
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_get](spec_get)(map, key) == value;
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(<b>old</b>(map), key) ==&gt; (([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(result_1)) && ([../../move-stdlib/doc/option.md#0x1_option_spec_borrow](option::spec_borrow)(result_1) == key));
<b>ensures</b> [abstract] [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(<b>old</b>(map), key) ==&gt; (([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(result_2)) && ([../../move-stdlib/doc/option.md#0x1_option_spec_borrow](option::spec_borrow)(result_2) == [simple_map.md#0x1_simple_map_spec_get](spec_get)(<b>old</b>(map), key)));
</code></pre>




<a id="0x1_simple_map_spec_len"></a>


<pre><code><b>native</b> <b>fun</b> [simple_map.md#0x1_simple_map_spec_len](spec_len)&lt;K, V&gt;(t: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;): num;
</code></pre>




<a id="0x1_simple_map_spec_contains_key"></a>


<pre><code><b>native</b> <b>fun</b> [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)&lt;K, V&gt;(t: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;, k: K): bool;
</code></pre>




<a id="0x1_simple_map_spec_set"></a>


<pre><code><b>native</b> <b>fun</b> [simple_map.md#0x1_simple_map_spec_set](spec_set)&lt;K, V&gt;(t: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;, k: K, v: V): [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;;
</code></pre>




<a id="0x1_simple_map_spec_remove"></a>


<pre><code><b>native</b> <b>fun</b> [simple_map.md#0x1_simple_map_spec_remove](spec_remove)&lt;K, V&gt;(t: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;, k: K): [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;;
</code></pre>




<a id="0x1_simple_map_spec_get"></a>


<pre><code><b>native</b> <b>fun</b> [simple_map.md#0x1_simple_map_spec_get](spec_get)&lt;K, V&gt;(t: [simple_map.md#0x1_simple_map_SimpleMap](SimpleMap)&lt;K, V&gt;, k: K): V;
</code></pre>



<a id="@Specification_1_keys"></a>

### Function `keys`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_keys](keys)&lt;Key: <b>copy</b>, Value&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>



<a id="@Specification_1_values"></a>

### Function `values`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_values](values)&lt;Key, Value: <b>copy</b>&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>



<a id="@Specification_1_to_vec_pair"></a>

### Function `to_vec_pair`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_to_vec_pair](to_vec_pair)&lt;Key: store, Value: store&gt;(map: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Key&gt;, [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;Value&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
<b>pragma</b> opaque;
<b>ensures</b> [abstract]
    <b>forall</b> k: Key: [../../move-stdlib/doc/vector.md#0x1_vector_spec_contains](vector::spec_contains)(result_1, k) &lt;==&gt;
        [simple_map.md#0x1_simple_map_spec_contains_key](spec_contains_key)(map, k);
<b>ensures</b> [abstract] <b>forall</b> i in 0..len(result_1):
    [simple_map.md#0x1_simple_map_spec_get](spec_get)(map, [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(result_1, i)) == [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(result_2, i);
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [simple_map.md#0x1_simple_map_remove](remove)&lt;Key: store, Value: store&gt;(map: &<b>mut</b> [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): (Key, Value)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_1_find"></a>

### Function `find`


<pre><code><b>fun</b> [simple_map.md#0x1_simple_map_find](find)&lt;Key: store, Value: store&gt;(map: &[simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;Key, Value&gt;, key: &Key): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
