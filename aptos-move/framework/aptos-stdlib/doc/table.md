
<a id="0x1_table"></a>

# Module `0x1::table`

Type of large-scale storage tables.
source: https://github.com/move-language/move/blob/1b6b7513dcc1a5c866f178ca5c1e74beb2ce181e/language/extensions/move-table-extension/sources/Table.move#L1

It implements the Table type which supports individual table items to be represented by
separate global state items. The number of items and a unique handle are tracked on the table
struct itself, while the operations are implemented as native functions. No traversal is provided.


-  [Struct `Table`](#0x1_table_Table)
-  [Resource `Box`](#0x1_table_Box)
-  [Function `new`](#0x1_table_new)
-  [Function `add`](#0x1_table_add)
-  [Function `borrow`](#0x1_table_borrow)
-  [Function `borrow_with_default`](#0x1_table_borrow_with_default)
-  [Function `borrow_mut`](#0x1_table_borrow_mut)
-  [Function `borrow_mut_with_default`](#0x1_table_borrow_mut_with_default)
-  [Function `upsert`](#0x1_table_upsert)
-  [Function `remove`](#0x1_table_remove)
-  [Function `contains`](#0x1_table_contains)
-  [Function `destroy`](#0x1_table_destroy)
-  [Function `new_table_handle`](#0x1_table_new_table_handle)
-  [Function `add_box`](#0x1_table_add_box)
-  [Function `borrow_box`](#0x1_table_borrow_box)
-  [Function `borrow_box_mut`](#0x1_table_borrow_box_mut)
-  [Function `contains_box`](#0x1_table_contains_box)
-  [Function `remove_box`](#0x1_table_remove_box)
-  [Function `destroy_empty_box`](#0x1_table_destroy_empty_box)
-  [Function `drop_unchecked_box`](#0x1_table_drop_unchecked_box)
-  [Specification](#@Specification_0)
    -  [Struct `Table`](#@Specification_0_Table)
    -  [Function `new`](#@Specification_0_new)
    -  [Function `add`](#@Specification_0_add)
    -  [Function `borrow`](#@Specification_0_borrow)
    -  [Function `borrow_mut`](#@Specification_0_borrow_mut)
    -  [Function `borrow_mut_with_default`](#@Specification_0_borrow_mut_with_default)
    -  [Function `upsert`](#@Specification_0_upsert)
    -  [Function `remove`](#@Specification_0_remove)
    -  [Function `contains`](#@Specification_0_contains)
    -  [Function `destroy`](#@Specification_0_destroy)


<pre><code></code></pre>



<a id="0x1_table_Table"></a>

## Struct `Table`

Type of tables


<pre><code><b>struct</b> [table.md#0x1_table_Table](Table)&lt;K: <b>copy</b>, drop, V&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>handle: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_table_Box"></a>

## Resource `Box`

Wrapper for values. Required for making values appear as resources in the implementation.


<pre><code><b>struct</b> [table.md#0x1_table_Box](Box)&lt;V&gt; <b>has</b> drop, store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>val: V</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_table_new"></a>

## Function `new`

Create a new Table.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_new](new)&lt;K: <b>copy</b>, drop, V: store&gt;(): [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_new](new)&lt;K: <b>copy</b> + drop, V: store&gt;(): [table.md#0x1_table_Table](Table)&lt;K, V&gt; {
    [table.md#0x1_table_Table](Table) {
        handle: [table.md#0x1_table_new_table_handle](new_table_handle)&lt;K, V&gt;(),
    }
}
</code></pre>



</details>

<a id="0x1_table_add"></a>

## Function `add`

Add a new entry to the table. Aborts if an entry for this
key already exists. The entry itself is not stored in the
table, and cannot be discovered from it.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_add](add)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, val: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_add](add)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K, val: V) {
    [table.md#0x1_table_add_box](add_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table), key, [table.md#0x1_table_Box](Box) { val })
}
</code></pre>



</details>

<a id="0x1_table_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow](borrow)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow](borrow)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): &V {
    &[table.md#0x1_table_borrow_box](borrow_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table), key).val
}
</code></pre>



</details>

<a id="0x1_table_borrow_with_default"></a>

## Function `borrow_with_default`

Acquire an immutable reference to the value which <code>key</code> maps to.
Returns specified default value if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_with_default](borrow_with_default)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, default: &V): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_with_default](borrow_with_default)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K, default: &V): &V {
    <b>if</b> (![table.md#0x1_table_contains](contains)([table.md#0x1_table](table), <b>copy</b> key)) {
        default
    } <b>else</b> {
        [table.md#0x1_table_borrow](borrow)([table.md#0x1_table](table), <b>copy</b> key)
    }
}
</code></pre>



</details>

<a id="0x1_table_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut](borrow_mut)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut](borrow_mut)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): &<b>mut</b> V {
    &<b>mut</b> [table.md#0x1_table_borrow_box_mut](borrow_box_mut)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table), key).val
}
</code></pre>



</details>

<a id="0x1_table_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which <code>key</code> maps to.
Insert the pair (<code>key</code>, <code>default</code>) first if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut_with_default](borrow_mut_with_default)&lt;K: <b>copy</b>, drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, default: V): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut_with_default](borrow_mut_with_default)&lt;K: <b>copy</b> + drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K, default: V): &<b>mut</b> V {
    <b>if</b> (![table.md#0x1_table_contains](contains)([table.md#0x1_table](table), <b>copy</b> key)) {
        [table.md#0x1_table_add](add)([table.md#0x1_table](table), <b>copy</b> key, default)
    };
    [table.md#0x1_table_borrow_mut](borrow_mut)([table.md#0x1_table](table), key)
}
</code></pre>



</details>

<a id="0x1_table_upsert"></a>

## Function `upsert`

Insert the pair (<code>key</code>, <code>value</code>) if there is no entry for <code>key</code>.
update the value of the entry for <code>key</code> to <code>value</code> otherwise


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_upsert](upsert)&lt;K: <b>copy</b>, drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_upsert](upsert)&lt;K: <b>copy</b> + drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K, value: V) {
    <b>if</b> (![table.md#0x1_table_contains](contains)([table.md#0x1_table](table), <b>copy</b> key)) {
        [table.md#0x1_table_add](add)([table.md#0x1_table](table), <b>copy</b> key, value)
    } <b>else</b> {
        <b>let</b> ref = [table.md#0x1_table_borrow_mut](borrow_mut)([table.md#0x1_table](table), key);
        *ref = value;
    };
}
</code></pre>



</details>

<a id="0x1_table_remove"></a>

## Function `remove`

Remove from <code>[table.md#0x1_table](table)</code> and return the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_remove](remove)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_remove](remove)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): V {
    <b>let</b> [table.md#0x1_table_Box](Box) { val } = [table.md#0x1_table_remove_box](remove_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table), key);
    val
}
</code></pre>



</details>

<a id="0x1_table_contains"></a>

## Function `contains`

Returns true iff <code>[table.md#0x1_table](table)</code> contains an entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_contains](contains)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_contains](contains)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): bool {
    [table.md#0x1_table_contains_box](contains_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table), key)
}
</code></pre>



</details>

<a id="0x1_table_destroy"></a>

## Function `destroy`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [table.md#0x1_table_destroy](destroy)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [table.md#0x1_table_destroy](destroy)&lt;K: <b>copy</b> + drop, V&gt;([table.md#0x1_table](table): [table.md#0x1_table_Table](Table)&lt;K, V&gt;) {
    [table.md#0x1_table_destroy_empty_box](destroy_empty_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;(&[table.md#0x1_table](table));
    [table.md#0x1_table_drop_unchecked_box](drop_unchecked_box)&lt;K, V, [table.md#0x1_table_Box](Box)&lt;V&gt;&gt;([table.md#0x1_table](table))
}
</code></pre>



</details>

<a id="0x1_table_new_table_handle"></a>

## Function `new_table_handle`



<pre><code><b>fun</b> [table.md#0x1_table_new_table_handle](new_table_handle)&lt;K, V&gt;(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_new_table_handle](new_table_handle)&lt;K, V&gt;(): <b>address</b>;
</code></pre>



</details>

<a id="0x1_table_add_box"></a>

## Function `add_box`



<pre><code><b>fun</b> [table.md#0x1_table_add_box](add_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, val: [table.md#0x1_table_Box](table::Box)&lt;V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_add_box](add_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K, val: [table.md#0x1_table_Box](Box)&lt;V&gt;);
</code></pre>



</details>

<a id="0x1_table_borrow_box"></a>

## Function `borrow_box`



<pre><code><b>fun</b> [table.md#0x1_table_borrow_box](borrow_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &[table.md#0x1_table_Box](table::Box)&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_borrow_box](borrow_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): &[table.md#0x1_table_Box](Box)&lt;V&gt;;
</code></pre>



</details>

<a id="0x1_table_borrow_box_mut"></a>

## Function `borrow_box_mut`



<pre><code><b>fun</b> [table.md#0x1_table_borrow_box_mut](borrow_box_mut)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &<b>mut</b> [table.md#0x1_table_Box](table::Box)&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_borrow_box_mut](borrow_box_mut)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): &<b>mut</b> [table.md#0x1_table_Box](Box)&lt;V&gt;;
</code></pre>



</details>

<a id="0x1_table_contains_box"></a>

## Function `contains_box`



<pre><code><b>fun</b> [table.md#0x1_table_contains_box](contains_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_contains_box](contains_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): bool;
</code></pre>



</details>

<a id="0x1_table_remove_box"></a>

## Function `remove_box`



<pre><code><b>fun</b> [table.md#0x1_table_remove_box](remove_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): [table.md#0x1_table_Box](table::Box)&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_remove_box](remove_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](Table)&lt;K, V&gt;, key: K): [table.md#0x1_table_Box](Box)&lt;V&gt;;
</code></pre>



</details>

<a id="0x1_table_destroy_empty_box"></a>

## Function `destroy_empty_box`



<pre><code><b>fun</b> [table.md#0x1_table_destroy_empty_box](destroy_empty_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_destroy_empty_box](destroy_empty_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](Table)&lt;K, V&gt;);
</code></pre>



</details>

<a id="0x1_table_drop_unchecked_box"></a>

## Function `drop_unchecked_box`



<pre><code><b>fun</b> [table.md#0x1_table_drop_unchecked_box](drop_unchecked_box)&lt;K: <b>copy</b>, drop, V, B&gt;([table.md#0x1_table](table): [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_drop_unchecked_box](drop_unchecked_box)&lt;K: <b>copy</b> + drop, V, B&gt;([table.md#0x1_table](table): [table.md#0x1_table_Table](Table)&lt;K, V&gt;);
</code></pre>



</details>

<a id="@Specification_0"></a>

## Specification


<a id="@Specification_0_Table"></a>

### Struct `Table`


<pre><code><b>struct</b> [table.md#0x1_table_Table](Table)&lt;K: <b>copy</b>, drop, V&gt; <b>has</b> store
</code></pre>



<dl>
<dt>
<code>handle: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>pragma</b> intrinsic = map,
    map_new = new,
    map_destroy_empty = destroy,
    map_has_key = contains,
    map_add_no_override = add,
    map_add_override_if_exists = upsert,
    map_del_must_exist = remove,
    map_borrow = borrow,
    map_borrow_mut = borrow_mut,
    map_borrow_mut_with_default = borrow_mut_with_default,
    map_spec_get = spec_get,
    map_spec_set = spec_set,
    map_spec_del = spec_remove,
    map_spec_has_key = spec_contains;
</code></pre>



<a id="@Specification_0_new"></a>

### Function `new`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_new](new)&lt;K: <b>copy</b>, drop, V: store&gt;(): [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_add"></a>

### Function `add`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_add](add)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, val: V)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow](borrow)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &V
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_borrow_mut"></a>

### Function `borrow_mut`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut](borrow_mut)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): &<b>mut</b> V
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_borrow_mut_with_default"></a>

### Function `borrow_mut_with_default`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_borrow_mut_with_default](borrow_mut_with_default)&lt;K: <b>copy</b>, drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, default: V): &<b>mut</b> V
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_upsert"></a>

### Function `upsert`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_upsert](upsert)&lt;K: <b>copy</b>, drop, V: drop&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K, value: V)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_remove](remove)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &<b>mut</b> [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): V
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>



<a id="@Specification_0_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> [table.md#0x1_table_contains](contains)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): &[table.md#0x1_table_Table](table::Table)&lt;K, V&gt;, key: K): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>




<a id="0x1_table_spec_contains"></a>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_spec_contains](spec_contains)&lt;K, V&gt;(t: [table.md#0x1_table_Table](Table)&lt;K, V&gt;, k: K): bool;
</code></pre>




<a id="0x1_table_spec_remove"></a>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_spec_remove](spec_remove)&lt;K, V&gt;(t: [table.md#0x1_table_Table](Table)&lt;K, V&gt;, k: K): [table.md#0x1_table_Table](Table)&lt;K, V&gt;;
</code></pre>




<a id="0x1_table_spec_set"></a>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_spec_set](spec_set)&lt;K, V&gt;(t: [table.md#0x1_table_Table](Table)&lt;K, V&gt;, k: K, v: V): [table.md#0x1_table_Table](Table)&lt;K, V&gt;;
</code></pre>




<a id="0x1_table_spec_get"></a>


<pre><code><b>native</b> <b>fun</b> [table.md#0x1_table_spec_get](spec_get)&lt;K, V&gt;(t: [table.md#0x1_table_Table](Table)&lt;K, V&gt;, k: K): V;
</code></pre>



<a id="@Specification_0_destroy"></a>

### Function `destroy`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [table.md#0x1_table_destroy](destroy)&lt;K: <b>copy</b>, drop, V&gt;([table.md#0x1_table](table): [table.md#0x1_table_Table](table::Table)&lt;K, V&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
