
<a id="0x1_vector"></a>

# Module `0x1::vector`

A variable-sized container that can hold any type. Indexing is 0-based, and
vectors are growable. This module has many native functions.
Verification of modules that use this one uses model functions that are implemented
directly in Boogie. The specification language has built-in functions operations such
as <code>singleton_vector</code>. There are some helper functions defined here for specifications in other
modules as well.

>Note: We did not verify most of the
Move functions here because many have loops, requiring loop invariants to prove, and
the return on investment didn't seem worth it for these simple functions.


-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_vector_empty)
-  [Function `length`](#0x1_vector_length)
-  [Function `borrow`](#0x1_vector_borrow)
-  [Function `push_back`](#0x1_vector_push_back)
-  [Function `borrow_mut`](#0x1_vector_borrow_mut)
-  [Function `pop_back`](#0x1_vector_pop_back)
-  [Function `destroy_empty`](#0x1_vector_destroy_empty)
-  [Function `swap`](#0x1_vector_swap)
-  [Function `singleton`](#0x1_vector_singleton)
-  [Function `reverse`](#0x1_vector_reverse)
-  [Function `reverse_slice`](#0x1_vector_reverse_slice)
-  [Function `append`](#0x1_vector_append)
-  [Function `reverse_append`](#0x1_vector_reverse_append)
-  [Function `trim`](#0x1_vector_trim)
-  [Function `trim_reverse`](#0x1_vector_trim_reverse)
-  [Function `is_empty`](#0x1_vector_is_empty)
-  [Function `contains`](#0x1_vector_contains)
-  [Function `index_of`](#0x1_vector_index_of)
-  [Function `find`](#0x1_vector_find)
-  [Function `insert`](#0x1_vector_insert)
-  [Function `remove`](#0x1_vector_remove)
-  [Function `remove_value`](#0x1_vector_remove_value)
-  [Function `swap_remove`](#0x1_vector_swap_remove)
-  [Function `for_each`](#0x1_vector_for_each)
-  [Function `for_each_reverse`](#0x1_vector_for_each_reverse)
-  [Function `for_each_ref`](#0x1_vector_for_each_ref)
-  [Function `zip`](#0x1_vector_zip)
-  [Function `zip_reverse`](#0x1_vector_zip_reverse)
-  [Function `zip_ref`](#0x1_vector_zip_ref)
-  [Function `enumerate_ref`](#0x1_vector_enumerate_ref)
-  [Function `for_each_mut`](#0x1_vector_for_each_mut)
-  [Function `zip_mut`](#0x1_vector_zip_mut)
-  [Function `enumerate_mut`](#0x1_vector_enumerate_mut)
-  [Function `fold`](#0x1_vector_fold)
-  [Function `foldr`](#0x1_vector_foldr)
-  [Function `map_ref`](#0x1_vector_map_ref)
-  [Function `zip_map_ref`](#0x1_vector_zip_map_ref)
-  [Function `map`](#0x1_vector_map)
-  [Function `zip_map`](#0x1_vector_zip_map)
-  [Function `filter`](#0x1_vector_filter)
-  [Function `partition`](#0x1_vector_partition)
-  [Function `rotate`](#0x1_vector_rotate)
-  [Function `rotate_slice`](#0x1_vector_rotate_slice)
-  [Function `stable_partition`](#0x1_vector_stable_partition)
-  [Function `any`](#0x1_vector_any)
-  [Function `all`](#0x1_vector_all)
-  [Function `destroy`](#0x1_vector_destroy)
-  [Function `range`](#0x1_vector_range)
-  [Function `range_with_step`](#0x1_vector_range_with_step)
-  [Function `slice`](#0x1_vector_slice)
-  [Specification](#@Specification_1)
    -  [Helper Functions](#@Helper_Functions_2)
    -  [Function `singleton`](#@Specification_1_singleton)
    -  [Function `reverse`](#@Specification_1_reverse)
    -  [Function `reverse_slice`](#@Specification_1_reverse_slice)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `reverse_append`](#@Specification_1_reverse_append)
    -  [Function `trim`](#@Specification_1_trim)
    -  [Function `trim_reverse`](#@Specification_1_trim_reverse)
    -  [Function `is_empty`](#@Specification_1_is_empty)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `index_of`](#@Specification_1_index_of)
    -  [Function `insert`](#@Specification_1_insert)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `remove_value`](#@Specification_1_remove_value)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `rotate`](#@Specification_1_rotate)
    -  [Function `rotate_slice`](#@Specification_1_rotate_slice)


<pre><code></code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_vector_EINDEX_OUT_OF_BOUNDS"></a>

The index into the vector is out of bounds


<pre><code><b>const</b> [vector.md#0x1_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS): u64 = 131072;
</code></pre>



<a id="0x1_vector_EINVALID_RANGE"></a>

The index into the vector is out of bounds


<pre><code><b>const</b> [vector.md#0x1_vector_EINVALID_RANGE](EINVALID_RANGE): u64 = 131073;
</code></pre>



<a id="0x1_vector_EINVALID_SLICE_RANGE"></a>

The range in <code>slice</code> is invalid.


<pre><code><b>const</b> [vector.md#0x1_vector_EINVALID_SLICE_RANGE](EINVALID_SLICE_RANGE): u64 = 131076;
</code></pre>



<a id="0x1_vector_EINVALID_STEP"></a>

The step provided in <code>range</code> is invalid, must be greater than zero.


<pre><code><b>const</b> [vector.md#0x1_vector_EINVALID_STEP](EINVALID_STEP): u64 = 131075;
</code></pre>



<a id="0x1_vector_EVECTORS_LENGTH_MISMATCH"></a>

The length of the vectors are not equal.


<pre><code><b>const</b> [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH): u64 = 131074;
</code></pre>



<a id="0x1_vector_empty"></a>

## Function `empty`

Create an empty vector.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_empty](empty)&lt;Element&gt;(): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_empty](empty)&lt;Element&gt;(): [vector.md#0x1_vector](vector)&lt;Element&gt;;
</code></pre>



</details>

<a id="0x1_vector_length"></a>

## Function `length`

Return the length of the vector.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_length](length)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_length](length)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;): u64;
</code></pre>



</details>

<a id="0x1_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>i</code>th element of the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_borrow](borrow)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): &Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_borrow](borrow)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): &Element;
</code></pre>



</details>

<a id="0x1_vector_push_back"></a>

## Function `push_back`

Add element <code>e</code> to the end of the vector <code>v</code>.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_push_back](push_back)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, e: Element)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_push_back](push_back)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, e: Element);
</code></pre>



</details>

<a id="0x1_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>i</code>th element in the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_borrow_mut](borrow_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): &<b>mut</b> Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_borrow_mut](borrow_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): &<b>mut</b> Element;
</code></pre>



</details>

<a id="0x1_vector_pop_back"></a>

## Function `pop_back`

Pop an element from the end of vector <code>v</code>.
Aborts if <code>v</code> is empty.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_pop_back](pop_back)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_pop_back](pop_back)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;): Element;
</code></pre>



</details>

<a id="0x1_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector <code>v</code>.
Aborts if <code>v</code> is not empty.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_destroy_empty](destroy_empty)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_destroy_empty](destroy_empty)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;);
</code></pre>



</details>

<a id="0x1_vector_swap"></a>

## Function `swap`

Swaps the elements at the <code>i</code>th and <code>j</code>th indices in the vector <code>v</code>.
Aborts if <code>i</code> or <code>j</code> is out of bounds.


<pre><code>#[bytecode_instruction]
<b>public</b> <b>fun</b> [vector.md#0x1_vector_swap](swap)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> [vector.md#0x1_vector_swap](swap)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64, j: u64);
</code></pre>



</details>

<a id="0x1_vector_singleton"></a>

## Function `singleton`

Return an vector of size one containing element <code>e</code>.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_singleton](singleton)&lt;Element&gt;(e: Element): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_singleton](singleton)&lt;Element&gt;(e: Element): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    <b>let</b> v = [vector.md#0x1_vector_empty](empty)();
    [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> v, e);
    v
}
</code></pre>



</details>

<a id="0x1_vector_reverse"></a>

## Function `reverse`

Reverses the order of the elements in the vector <code>v</code> in place.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse](reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse](reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    [vector.md#0x1_vector_reverse_slice](reverse_slice)(v, 0, len);
}
</code></pre>



</details>

<a id="0x1_vector_reverse_slice"></a>

## Function `reverse_slice`

Reverses the order of the elements [left, right) in the vector <code>v</code> in place.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_slice](reverse_slice)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, left: u64, right: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_slice](reverse_slice)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, left: u64, right: u64) {
    <b>assert</b>!(left &lt;= right, [vector.md#0x1_vector_EINVALID_RANGE](EINVALID_RANGE));
    <b>if</b> (left == right) <b>return</b>;
    right = right - 1;
    <b>while</b> (left &lt; right) {
        [vector.md#0x1_vector_swap](swap)(v, left, right);
        left = left + 1;
        right = right - 1;
    }
}
</code></pre>



</details>

<a id="0x1_vector_append"></a>

## Function `append`

Pushes all of the elements of the <code>other</code> vector into the <code>lhs</code> vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_append](append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_append](append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;) {
    [vector.md#0x1_vector_reverse](reverse)(&<b>mut</b> other);
    [vector.md#0x1_vector_reverse_append](reverse_append)(lhs, other);
}
</code></pre>



</details>

<a id="0x1_vector_reverse_append"></a>

## Function `reverse_append`

Pushes all of the elements of the <code>other</code> vector into the <code>lhs</code> vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_append](reverse_append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_append](reverse_append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(&other);
    <b>while</b> (len &gt; 0) {
        [vector.md#0x1_vector_push_back](push_back)(lhs, [vector.md#0x1_vector_pop_back](pop_back)(&<b>mut</b> other));
        len = len - 1;
    };
    [vector.md#0x1_vector_destroy_empty](destroy_empty)(other);
}
</code></pre>



</details>

<a id="0x1_vector_trim"></a>

## Function `trim`

Trim a vector to a smaller size, returning the evicted elements in order


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim](trim)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim](trim)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    <b>let</b> res = [vector.md#0x1_vector_trim_reverse](trim_reverse)(v, new_len);
    [vector.md#0x1_vector_reverse](reverse)(&<b>mut</b> res);
    res
}
</code></pre>



</details>

<a id="0x1_vector_trim_reverse"></a>

## Function `trim_reverse`

Trim a vector to a smaller size, returning the evicted elements in reverse order


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim_reverse](trim_reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim_reverse](trim_reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>assert</b>!(new_len &lt;= len, [vector.md#0x1_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS));
    <b>let</b> result = [vector.md#0x1_vector_empty](empty)();
    <b>while</b> (new_len &lt; len) {
        [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, [vector.md#0x1_vector_pop_back](pop_back)(v));
        len = len - 1;
    };
    result
}
</code></pre>



</details>

<a id="0x1_vector_is_empty"></a>

## Function `is_empty`

Return <code><b>true</b></code> if the vector <code>v</code> has no elements and <code><b>false</b></code> otherwise.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_is_empty](is_empty)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_is_empty](is_empty)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;): bool {
    [vector.md#0x1_vector_length](length)(v) == 0
}
</code></pre>



</details>

<a id="0x1_vector_contains"></a>

## Function `contains`

Return true if <code>e</code> is in the vector <code>v</code>.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_contains](contains)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_contains](contains)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): bool {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> ([vector.md#0x1_vector_borrow](borrow)(v, i) == e) <b>return</b> <b>true</b>;
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a id="0x1_vector_index_of"></a>

## Function `index_of`

Return <code>(<b>true</b>, i)</code> if <code>e</code> is in the vector <code>v</code> at index <code>i</code>.
Otherwise, returns <code>(<b>false</b>, 0)</code>.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_index_of](index_of)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_index_of](index_of)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): (bool, u64) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> ([vector.md#0x1_vector_borrow](borrow)(v, i) == e) <b>return</b> (<b>true</b>, i);
        i = i + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a id="0x1_vector_find"></a>

## Function `find`

Return <code>(<b>true</b>, i)</code> if there's an element that matches the predicate. If there are multiple elements that match
the predicate, only the index of the first one is returned.
Otherwise, returns <code>(<b>false</b>, 0)</code>.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_find](find)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&Element|bool): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_find](find)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&Element|bool): (bool, u64) {
    <b>let</b> find = <b>false</b>;
    <b>let</b> found_index = 0;
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        // Cannot call <b>return</b> in an inline function so we need <b>to</b> resort <b>to</b> <b>break</b> here.
        <b>if</b> (f([vector.md#0x1_vector_borrow](borrow)(v, i))) {
            find = <b>true</b>;
            found_index = i;
            <b>break</b>
        };
        i = i + 1;
    };
    (find, found_index)
}
</code></pre>



</details>

<a id="0x1_vector_insert"></a>

## Function `insert`

Insert a new element at position 0 <= i <= length, using O(length - i) time.
Aborts if out of bounds.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_insert](insert)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64, e: Element)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_insert](insert)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64, e: Element) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>assert</b>!(i &lt;= len, [vector.md#0x1_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS));
    [vector.md#0x1_vector_push_back](push_back)(v, e);
    <b>while</b> (i &lt; len) {
        [vector.md#0x1_vector_swap](swap)(v, i, len);
        i = i + 1;
    };
}
</code></pre>



</details>

<a id="0x1_vector_remove"></a>

## Function `remove`

Remove the <code>i</code>th element of the vector <code>v</code>, shifting all subsequent elements.
This is O(n) and preserves ordering of elements in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove](remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove](remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    // i out of bounds; <b>abort</b>
    <b>if</b> (i &gt;= len) <b>abort</b> [vector.md#0x1_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS);

    len = len - 1;
    <b>while</b> (i &lt; len) [vector.md#0x1_vector_swap](swap)(v, i, { i = i + 1; i });
    [vector.md#0x1_vector_pop_back](pop_back)(v)
}
</code></pre>



</details>

<a id="0x1_vector_remove_value"></a>

## Function `remove_value`

Remove the first occurrence of a given value in the vector <code>v</code> and return it in a vector, shifting all
subsequent elements.
This is O(n) and preserves ordering of elements in the vector.
This returns an empty vector if the value isn't present in the vector.
Note that this cannot return an option as option uses vector and there'd be a circular dependency between option
and vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove_value](remove_value)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, val: &Element): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove_value](remove_value)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, val: &Element): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    // This doesn't cost a O(2N) run time <b>as</b> index_of scans from left <b>to</b> right and stops when the element is found,
    // <b>while</b> remove would <b>continue</b> from the identified index <b>to</b> the end of the [vector.md#0x1_vector](vector).
    <b>let</b> (found, index) = [vector.md#0x1_vector_index_of](index_of)(v, val);
    <b>if</b> (found) {
        [vector.md#0x1_vector](vector)[[vector.md#0x1_vector_remove](remove)(v, index)]
    } <b>else</b> {
       [vector.md#0x1_vector](vector)[]
    }
}
</code></pre>



</details>

<a id="0x1_vector_swap_remove"></a>

## Function `swap_remove`

Swap the <code>i</code>th element of the vector <code>v</code> with the last element and then pop the vector.
This is O(1), but does not preserve ordering of elements in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_swap_remove](swap_remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_swap_remove](swap_remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element {
    <b>assert</b>!(![vector.md#0x1_vector_is_empty](is_empty)(v), [vector.md#0x1_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS));
    <b>let</b> last_idx = [vector.md#0x1_vector_length](length)(v) - 1;
    [vector.md#0x1_vector_swap](swap)(v, i, last_idx);
    [vector.md#0x1_vector_pop_back](pop_back)(v)
}
</code></pre>



</details>

<a id="0x1_vector_for_each"></a>

## Function `for_each`

Apply the function to each element in the vector, consuming it.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_for_each](for_each)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |Element|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_for_each](for_each)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |Element|) {
    [vector.md#0x1_vector_reverse](reverse)(&<b>mut</b> v); // We need <b>to</b> reverse the [vector.md#0x1_vector](vector) <b>to</b> consume it efficiently
    [vector.md#0x1_vector_for_each_reverse](for_each_reverse)(v, |e| f(e));
}
</code></pre>



</details>

<a id="0x1_vector_for_each_reverse"></a>

## Function `for_each_reverse`

Apply the function to each element in the vector, consuming it.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_for_each_reverse](for_each_reverse)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |Element|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_for_each_reverse](for_each_reverse)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |Element|) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(&v);
    <b>while</b> (len &gt; 0) {
        f([vector.md#0x1_vector_pop_back](pop_back)(&<b>mut</b> v));
        len = len - 1;
    };
    [vector.md#0x1_vector_destroy_empty](destroy_empty)(v)
}
</code></pre>



</details>

<a id="0x1_vector_for_each_ref"></a>

## Function `for_each_ref`

Apply the function to a reference of each element in the vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_for_each_ref](for_each_ref)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&Element|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_for_each_ref](for_each_ref)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&Element|) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        f([vector.md#0x1_vector_borrow](borrow)(v, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_vector_zip"></a>

## Function `zip`

Apply the function to each pair of elements in the two given vectors, consuming them.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip](zip)&lt;Element1, Element2&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(Element1, Element2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip](zip)&lt;Element1, Element2&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |Element1, Element2|) {
    // We need <b>to</b> reverse the vectors <b>to</b> consume it efficiently
    [vector.md#0x1_vector_reverse](reverse)(&<b>mut</b> v1);
    [vector.md#0x1_vector_reverse](reverse)(&<b>mut</b> v2);
    [vector.md#0x1_vector_zip_reverse](zip_reverse)(v1, v2, |e1, e2| f(e1, e2));
}
</code></pre>



</details>

<a id="0x1_vector_zip_reverse"></a>

## Function `zip_reverse`

Apply the function to each pair of elements in the two given vectors in the reverse order, consuming them.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip_reverse](zip_reverse)&lt;Element1, Element2&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(Element1, Element2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip_reverse](zip_reverse)&lt;Element1, Element2&gt;(
    v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;,
    v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;,
    f: |Element1, Element2|,
) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(&v1);
    // We can't <b>use</b> the constant [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == [vector.md#0x1_vector_length](length)(&v2), 0x20002);
    <b>while</b> (len &gt; 0) {
        f([vector.md#0x1_vector_pop_back](pop_back)(&<b>mut</b> v1), [vector.md#0x1_vector_pop_back](pop_back)(&<b>mut</b> v2));
        len = len - 1;
    };
    [vector.md#0x1_vector_destroy_empty](destroy_empty)(v1);
    [vector.md#0x1_vector_destroy_empty](destroy_empty)(v2);
}
</code></pre>



</details>

<a id="0x1_vector_zip_ref"></a>

## Function `zip_ref`

Apply the function to the references of each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip_ref](zip_ref)&lt;Element1, Element2&gt;(v1: &[vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: &[vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(&Element1, &Element2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip_ref](zip_ref)&lt;Element1, Element2&gt;(
    v1: &[vector.md#0x1_vector](vector)&lt;Element1&gt;,
    v2: &[vector.md#0x1_vector](vector)&lt;Element2&gt;,
    f: |&Element1, &Element2|,
) {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v1);
    // We can't <b>use</b> the constant [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == [vector.md#0x1_vector_length](length)(v2), 0x20002);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        f([vector.md#0x1_vector_borrow](borrow)(v1, i), [vector.md#0x1_vector_borrow](borrow)(v2, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_vector_enumerate_ref"></a>

## Function `enumerate_ref`

Apply the function to a reference of each element in the vector with its index.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_enumerate_ref](enumerate_ref)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |(u64, &Element)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_enumerate_ref](enumerate_ref)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |u64, &Element|) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        f(i, [vector.md#0x1_vector_borrow](borrow)(v, i));
        i = i + 1;
    };
}
</code></pre>



</details>

<a id="0x1_vector_for_each_mut"></a>

## Function `for_each_mut`

Apply the function to a mutable reference to each element in the vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_for_each_mut](for_each_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&<b>mut</b> Element|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_for_each_mut](for_each_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&<b>mut</b> Element|) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        f([vector.md#0x1_vector_borrow_mut](borrow_mut)(v, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_vector_zip_mut"></a>

## Function `zip_mut`

Apply the function to mutable references to each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip_mut](zip_mut)&lt;Element1, Element2&gt;(v1: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(&<b>mut</b> Element1, &<b>mut</b> Element2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip_mut](zip_mut)&lt;Element1, Element2&gt;(
    v1: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element1&gt;,
    v2: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element2&gt;,
    f: |&<b>mut</b> Element1, &<b>mut</b> Element2|,
) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v1);
    // We can't <b>use</b> the constant [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == [vector.md#0x1_vector_length](length)(v2), 0x20002);
    <b>while</b> (i &lt; len) {
        f([vector.md#0x1_vector_borrow_mut](borrow_mut)(v1, i), [vector.md#0x1_vector_borrow_mut](borrow_mut)(v2, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_vector_enumerate_mut"></a>

## Function `enumerate_mut`

Apply the function to a mutable reference of each element in the vector with its index.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_enumerate_mut](enumerate_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |(u64, &<b>mut</b> Element)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_enumerate_mut](enumerate_mut)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |u64, &<b>mut</b> Element|) {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        f(i, [vector.md#0x1_vector_borrow_mut](borrow_mut)(v, i));
        i = i + 1;
    };
}
</code></pre>



</details>

<a id="0x1_vector_fold"></a>

## Function `fold`

Fold the function over the elements. For example, <code>[vector.md#0x1_vector_fold](fold)([vector.md#0x1_vector](vector)[1,2,3], 0, f)</code> will execute
<code>f(f(f(0, 1), 2), 3)</code>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_fold](fold)&lt;Accumulator, Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, init: Accumulator, f: |(Accumulator, Element)|Accumulator): Accumulator
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_fold](fold)&lt;Accumulator, Element&gt;(
    v: [vector.md#0x1_vector](vector)&lt;Element&gt;,
    init: Accumulator,
    f: |Accumulator,Element|Accumulator
): Accumulator {
    <b>let</b> accu = init;
    [vector.md#0x1_vector_for_each](for_each)(v, |elem| accu = f(accu, elem));
    accu
}
</code></pre>



</details>

<a id="0x1_vector_foldr"></a>

## Function `foldr`

Fold right like fold above but working right to left. For example, <code>[vector.md#0x1_vector_fold](fold)([vector.md#0x1_vector](vector)[1,2,3], 0, f)</code> will execute
<code>f(1, f(2, f(3, 0)))</code>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_foldr](foldr)&lt;Accumulator, Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, init: Accumulator, f: |(Element, Accumulator)|Accumulator): Accumulator
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_foldr](foldr)&lt;Accumulator, Element&gt;(
    v: [vector.md#0x1_vector](vector)&lt;Element&gt;,
    init: Accumulator,
    f: |Element, Accumulator|Accumulator
): Accumulator {
    <b>let</b> accu = init;
    [vector.md#0x1_vector_for_each_reverse](for_each_reverse)(v, |elem| accu = f(elem, accu));
    accu
}
</code></pre>



</details>

<a id="0x1_vector_map_ref"></a>

## Function `map_ref`

Map the function over the references of the elements of the vector, producing a new vector without modifying the
original vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_map_ref](map_ref)&lt;Element, NewElement&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, f: |&Element|NewElement): [vector.md#0x1_vector](vector)&lt;NewElement&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_map_ref](map_ref)&lt;Element, NewElement&gt;(
    v: &[vector.md#0x1_vector](vector)&lt;Element&gt;,
    f: |&Element|NewElement
): [vector.md#0x1_vector](vector)&lt;NewElement&gt; {
    <b>let</b> result = [vector.md#0x1_vector](vector)&lt;NewElement&gt;[];
    [vector.md#0x1_vector_for_each_ref](for_each_ref)(v, |elem| [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, f(elem)));
    result
}
</code></pre>



</details>

<a id="0x1_vector_zip_map_ref"></a>

## Function `zip_map_ref`

Map the function over the references of the element pairs of two vectors, producing a new vector from the return
values without modifying the original vectors.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip_map_ref](zip_map_ref)&lt;Element1, Element2, NewElement&gt;(v1: &[vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: &[vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(&Element1, &Element2)|NewElement): [vector.md#0x1_vector](vector)&lt;NewElement&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip_map_ref](zip_map_ref)&lt;Element1, Element2, NewElement&gt;(
    v1: &[vector.md#0x1_vector](vector)&lt;Element1&gt;,
    v2: &[vector.md#0x1_vector](vector)&lt;Element2&gt;,
    f: |&Element1, &Element2|NewElement
): [vector.md#0x1_vector](vector)&lt;NewElement&gt; {
    // We can't <b>use</b> the constant [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!([vector.md#0x1_vector_length](length)(v1) == [vector.md#0x1_vector_length](length)(v2), 0x20002);

    <b>let</b> result = [vector.md#0x1_vector](vector)&lt;NewElement&gt;[];
    [vector.md#0x1_vector_zip_ref](zip_ref)(v1, v2, |e1, e2| [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, f(e1, e2)));
    result
}
</code></pre>



</details>

<a id="0x1_vector_map"></a>

## Function `map`

Map the function over the elements of the vector, producing a new vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_map](map)&lt;Element, NewElement&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, f: |Element|NewElement): [vector.md#0x1_vector](vector)&lt;NewElement&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_map](map)&lt;Element, NewElement&gt;(
    v: [vector.md#0x1_vector](vector)&lt;Element&gt;,
    f: |Element|NewElement
): [vector.md#0x1_vector](vector)&lt;NewElement&gt; {
    <b>let</b> result = [vector.md#0x1_vector](vector)&lt;NewElement&gt;[];
    [vector.md#0x1_vector_for_each](for_each)(v, |elem| [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, f(elem)));
    result
}
</code></pre>



</details>

<a id="0x1_vector_zip_map"></a>

## Function `zip_map`

Map the function over the element pairs of the two vectors, producing a new vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_zip_map](zip_map)&lt;Element1, Element2, NewElement&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;, f: |(Element1, Element2)|NewElement): [vector.md#0x1_vector](vector)&lt;NewElement&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_zip_map](zip_map)&lt;Element1, Element2, NewElement&gt;(
    v1: [vector.md#0x1_vector](vector)&lt;Element1&gt;,
    v2: [vector.md#0x1_vector](vector)&lt;Element2&gt;,
    f: |Element1, Element2|NewElement
): [vector.md#0x1_vector](vector)&lt;NewElement&gt; {
    // We can't <b>use</b> the constant [vector.md#0x1_vector_EVECTORS_LENGTH_MISMATCH](EVECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!([vector.md#0x1_vector_length](length)(&v1) == [vector.md#0x1_vector_length](length)(&v2), 0x20002);

    <b>let</b> result = [vector.md#0x1_vector](vector)&lt;NewElement&gt;[];
    [vector.md#0x1_vector_zip](zip)(v1, v2, |e1, e2| [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, f(e1, e2)));
    result
}
</code></pre>



</details>

<a id="0x1_vector_filter"></a>

## Function `filter`

Filter the vector using the boolean function, removing all elements for which <code>p(e)</code> is not true.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_filter](filter)&lt;Element: drop&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, p: |&Element|bool): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_filter](filter)&lt;Element:drop&gt;(
    v: [vector.md#0x1_vector](vector)&lt;Element&gt;,
    p: |&Element|bool
): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    <b>let</b> result = [vector.md#0x1_vector](vector)&lt;Element&gt;[];
    [vector.md#0x1_vector_for_each](for_each)(v, |elem| {
        <b>if</b> (p(&elem)) [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> result, elem);
    });
    result
}
</code></pre>



</details>

<a id="0x1_vector_partition"></a>

## Function `partition`

Partition, sorts all elements for which pred is true to the front.
Preserves the relative order of the elements for which pred is true,
BUT NOT for the elements for which pred is false.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_partition](partition)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, pred: |&Element|bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_partition](partition)&lt;Element&gt;(
    v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;,
    pred: |&Element|bool
): u64 {
    <b>let</b> i = 0;
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> (!pred([vector.md#0x1_vector_borrow](borrow)(v, i))) <b>break</b>;
        i = i + 1;
    };
    <b>let</b> p = i;
    i = i + 1;
    <b>while</b> (i &lt; len) {
        <b>if</b> (pred([vector.md#0x1_vector_borrow](borrow)(v, i))) {
            [vector.md#0x1_vector_swap](swap)(v, p, i);
            p = p + 1;
        };
        i = i + 1;
    };
    p
}
</code></pre>



</details>

<a id="0x1_vector_rotate"></a>

## Function `rotate`

rotate(&mut [1, 2, 3, 4, 5], 2) -> [3, 4, 5, 1, 2] in place, returns the split point
ie. 3 in the example above


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate](rotate)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, rot: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate](rotate)&lt;Element&gt;(
    v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;,
    rot: u64
): u64 {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    [vector.md#0x1_vector_rotate_slice](rotate_slice)(v, 0, rot, len)
}
</code></pre>



</details>

<a id="0x1_vector_rotate_slice"></a>

## Function `rotate_slice`

Same as above but on a sub-slice of an array [left, right) with left <= rot <= right
returns the


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate_slice](rotate_slice)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, left: u64, rot: u64, right: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate_slice](rotate_slice)&lt;Element&gt;(
    v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;,
    left: u64,
    rot: u64,
    right: u64
): u64 {
    [vector.md#0x1_vector_reverse_slice](reverse_slice)(v, left, rot);
    [vector.md#0x1_vector_reverse_slice](reverse_slice)(v, rot, right);
    [vector.md#0x1_vector_reverse_slice](reverse_slice)(v, left, right);
    left + (right - rot)
}
</code></pre>



</details>

<a id="0x1_vector_stable_partition"></a>

## Function `stable_partition`

Partition the array based on a predicate p, this routine is stable and thus
preserves the relative order of the elements in the two partitions.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_stable_partition](stable_partition)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, p: |&Element|bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_stable_partition](stable_partition)&lt;Element&gt;(
    v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;,
    p: |&Element|bool
): u64 {
    <b>let</b> len = [vector.md#0x1_vector_length](length)(v);
    <b>let</b> t = [vector.md#0x1_vector_empty](empty)();
    <b>let</b> f = [vector.md#0x1_vector_empty](empty)();
    <b>while</b> (len &gt; 0) {
        <b>let</b> e = [vector.md#0x1_vector_pop_back](pop_back)(v);
        <b>if</b> (p(&e)) {
            [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> t, e);
        } <b>else</b> {
            [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> f, e);
        };
        len = len - 1;
    };
    <b>let</b> pos = [vector.md#0x1_vector_length](length)(&t);
    [vector.md#0x1_vector_reverse_append](reverse_append)(v, t);
    [vector.md#0x1_vector_reverse_append](reverse_append)(v, f);
    pos
}
</code></pre>



</details>

<a id="0x1_vector_any"></a>

## Function `any`

Return true if any element in the vector satisfies the predicate.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_any](any)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, p: |&Element|bool): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_any](any)&lt;Element&gt;(
    v: &[vector.md#0x1_vector](vector)&lt;Element&gt;,
    p: |&Element|bool
): bool {
    <b>let</b> result = <b>false</b>;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; [vector.md#0x1_vector_length](length)(v)) {
        result = p([vector.md#0x1_vector_borrow](borrow)(v, i));
        <b>if</b> (result) {
            <b>break</b>
        };
        i = i + 1
    };
    result
}
</code></pre>



</details>

<a id="0x1_vector_all"></a>

## Function `all`

Return true if all elements in the vector satisfy the predicate.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_all](all)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, p: |&Element|bool): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_all](all)&lt;Element&gt;(
    v: &[vector.md#0x1_vector](vector)&lt;Element&gt;,
    p: |&Element|bool
): bool {
    <b>let</b> result = <b>true</b>;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; [vector.md#0x1_vector_length](length)(v)) {
        result = p([vector.md#0x1_vector_borrow](borrow)(v, i));
        <b>if</b> (!result) {
            <b>break</b>
        };
        i = i + 1
    };
    result
}
</code></pre>



</details>

<a id="0x1_vector_destroy"></a>

## Function `destroy`

Destroy a vector, just a wrapper around for_each_reverse with a descriptive name
when used in the context of destroying a vector.


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_destroy](destroy)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, d: |Element|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [vector.md#0x1_vector_destroy](destroy)&lt;Element&gt;(
    v: [vector.md#0x1_vector](vector)&lt;Element&gt;,
    d: |Element|
) {
    [vector.md#0x1_vector_for_each_reverse](for_each_reverse)(v, |e| d(e))
}
</code></pre>



</details>

<a id="0x1_vector_range"></a>

## Function `range`



<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_range](range)(start: u64, end: u64): [vector.md#0x1_vector](vector)&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_range](range)(start: u64, end: u64): [vector.md#0x1_vector](vector)&lt;u64&gt; {
    [vector.md#0x1_vector_range_with_step](range_with_step)(start, end, 1)
}
</code></pre>



</details>

<a id="0x1_vector_range_with_step"></a>

## Function `range_with_step`



<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_range_with_step](range_with_step)(start: u64, end: u64, step: u64): [vector.md#0x1_vector](vector)&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_range_with_step](range_with_step)(start: u64, end: u64, step: u64): [vector.md#0x1_vector](vector)&lt;u64&gt; {
    <b>assert</b>!(step &gt; 0, [vector.md#0x1_vector_EINVALID_STEP](EINVALID_STEP));

    <b>let</b> vec = [vector.md#0x1_vector](vector)[];
    <b>while</b> (start &lt; end) {
        [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> vec, start);
        start = start + step;
    };
    vec
}
</code></pre>



</details>

<a id="0x1_vector_slice"></a>

## Function `slice`



<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_slice](slice)&lt;Element: <b>copy</b>&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, start: u64, end: u64): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_slice](slice)&lt;Element: <b>copy</b>&gt;(
    v: &[vector.md#0x1_vector](vector)&lt;Element&gt;,
    start: u64,
    end: u64
): [vector.md#0x1_vector](vector)&lt;Element&gt; {
    <b>assert</b>!(start &lt;= end && end &lt;= [vector.md#0x1_vector_length](length)(v), [vector.md#0x1_vector_EINVALID_SLICE_RANGE](EINVALID_SLICE_RANGE));

    <b>let</b> vec = [vector.md#0x1_vector](vector)[];
    <b>while</b> (start &lt; end) {
        [vector.md#0x1_vector_push_back](push_back)(&<b>mut</b> vec, *[vector.md#0x1_vector_borrow](borrow)(v, start));
        start = start + 1;
    };
    vec
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<a id="@Helper_Functions_2"></a>

### Helper Functions


Check if <code>v1</code> is equal to the result of adding <code>e</code> at the end of <code>v2</code>


<a id="0x1_vector_eq_push_back"></a>


<pre><code><b>fun</b> [vector.md#0x1_vector_eq_push_back](eq_push_back)&lt;Element&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element&gt;, e: Element): bool {
    len(v1) == len(v2) + 1 &&
    v1[len(v1)-1] == e &&
    v1[0..len(v1)-1] == v2[0..len(v2)]
}
</code></pre>


Check if <code>v</code> is equal to the result of concatenating <code>v1</code> and <code>v2</code>


<a id="0x1_vector_eq_append"></a>


<pre><code><b>fun</b> [vector.md#0x1_vector_eq_append](eq_append)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, v1: [vector.md#0x1_vector](vector)&lt;Element&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element&gt;): bool {
    len(v) == len(v1) + len(v2) &&
    v[0..len(v1)] == v1 &&
    v[len(v1)..len(v)] == v2
}
</code></pre>


Check <code>v1</code> is equal to the result of removing the first element of <code>v2</code>


<a id="0x1_vector_eq_pop_front"></a>


<pre><code><b>fun</b> [vector.md#0x1_vector_eq_pop_front](eq_pop_front)&lt;Element&gt;(v1: [vector.md#0x1_vector](vector)&lt;Element&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element&gt;): bool {
    len(v1) + 1 == len(v2) &&
    v1 == v2[1..len(v2)]
}
</code></pre>


Check that <code>v1</code> is equal to the result of removing the element at index <code>i</code> from <code>v2</code>.


<a id="0x1_vector_eq_remove_elem_at_index"></a>


<pre><code><b>fun</b> [vector.md#0x1_vector_eq_remove_elem_at_index](eq_remove_elem_at_index)&lt;Element&gt;(i: u64, v1: [vector.md#0x1_vector](vector)&lt;Element&gt;, v2: [vector.md#0x1_vector](vector)&lt;Element&gt;): bool {
    len(v1) + 1 == len(v2) &&
    v1[0..i] == v2[0..i] &&
    v1[i..len(v1)] == v2[i + 1..len(v2)]
}
</code></pre>


Check if <code>v</code> contains <code>e</code>.


<a id="0x1_vector_spec_contains"></a>


<pre><code><b>fun</b> [vector.md#0x1_vector_spec_contains](spec_contains)&lt;Element&gt;(v: [vector.md#0x1_vector](vector)&lt;Element&gt;, e: Element): bool {
    <b>exists</b> x in v: x == e
}
</code></pre>



<a id="@Specification_1_singleton"></a>

### Function `singleton`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_singleton](singleton)&lt;Element&gt;(e: Element): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == vec(e);
</code></pre>



<a id="@Specification_1_reverse"></a>

### Function `reverse`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse](reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_reverse_slice"></a>

### Function `reverse_slice`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_slice](reverse_slice)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, left: u64, right: u64)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_append"></a>

### Function `append`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_append](append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_reverse_append"></a>

### Function `reverse_append`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_reverse_append](reverse_append)&lt;Element&gt;(lhs: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, other: [vector.md#0x1_vector](vector)&lt;Element&gt;)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_trim"></a>

### Function `trim`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim](trim)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_trim_reverse"></a>

### Function `trim_reverse`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_trim_reverse](trim_reverse)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, new_len: u64): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_is_empty"></a>

### Function `is_empty`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_is_empty](is_empty)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_contains](contains)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): bool
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_index_of"></a>

### Function `index_of`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_index_of](index_of)&lt;Element&gt;(v: &[vector.md#0x1_vector](vector)&lt;Element&gt;, e: &Element): (bool, u64)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_insert"></a>

### Function `insert`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_insert](insert)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64, e: Element)
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove](remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_remove_value"></a>

### Function `remove_value`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_remove_value](remove_value)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, val: &Element): [vector.md#0x1_vector](vector)&lt;Element&gt;
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_swap_remove"></a>

### Function `swap_remove`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_swap_remove](swap_remove)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, i: u64): Element
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_rotate"></a>

### Function `rotate`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate](rotate)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, rot: u64): u64
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>



<a id="@Specification_1_rotate_slice"></a>

### Function `rotate_slice`


<pre><code><b>public</b> <b>fun</b> [vector.md#0x1_vector_rotate_slice](rotate_slice)&lt;Element&gt;(v: &<b>mut</b> [vector.md#0x1_vector](vector)&lt;Element&gt;, left: u64, rot: u64, right: u64): u64
</code></pre>




<pre><code><b>pragma</b> intrinsic = <b>true</b>;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
