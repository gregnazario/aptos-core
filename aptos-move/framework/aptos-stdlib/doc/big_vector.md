
<a id="0x1_big_vector"></a>

# Module `0x1::big_vector`



-  [Struct `BigVector`](#0x1_big_vector_BigVector)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_big_vector_empty)
-  [Function `singleton`](#0x1_big_vector_singleton)
-  [Function `destroy_empty`](#0x1_big_vector_destroy_empty)
-  [Function `destroy`](#0x1_big_vector_destroy)
-  [Function `borrow`](#0x1_big_vector_borrow)
-  [Function `borrow_mut`](#0x1_big_vector_borrow_mut)
-  [Function `append`](#0x1_big_vector_append)
-  [Function `push_back`](#0x1_big_vector_push_back)
-  [Function `pop_back`](#0x1_big_vector_pop_back)
-  [Function `remove`](#0x1_big_vector_remove)
-  [Function `swap_remove`](#0x1_big_vector_swap_remove)
-  [Function `swap`](#0x1_big_vector_swap)
-  [Function `reverse`](#0x1_big_vector_reverse)
-  [Function `index_of`](#0x1_big_vector_index_of)
-  [Function `contains`](#0x1_big_vector_contains)
-  [Function `to_vector`](#0x1_big_vector_to_vector)
-  [Function `length`](#0x1_big_vector_length)
-  [Function `is_empty`](#0x1_big_vector_is_empty)
-  [Specification](#@Specification_1)
    -  [Struct `BigVector`](#@Specification_1_BigVector)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `singleton`](#@Specification_1_singleton)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `push_back`](#@Specification_1_push_back)
    -  [Function `pop_back`](#@Specification_1_pop_back)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `swap`](#@Specification_1_swap)
    -  [Function `reverse`](#@Specification_1_reverse)
    -  [Function `index_of`](#@Specification_1_index_of)


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [table_with_length.md#0x1_table_with_length](0x1::table_with_length);
<b>use</b> [../../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_big_vector_BigVector"></a>

## Struct `BigVector`

A scalable vector implementation based on tables where elements are grouped into buckets.
Each bucket has a capacity of <code>bucket_size</code> elements.


<pre><code><b>struct</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>buckets: [table_with_length.md#0x1_table_with_length_TableWithLength](table_with_length::TableWithLength)&lt;u64, [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>end_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>bucket_size: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_big_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


<pre><code><b>const</b> [big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS): u64 = 1;
</code></pre>



<a id="0x1_big_vector_EVECTOR_EMPTY"></a>

Cannot pop back from an empty vector


<pre><code><b>const</b> [big_vector.md#0x1_big_vector_EVECTOR_EMPTY](EVECTOR_EMPTY): u64 = 3;
</code></pre>



<a id="0x1_big_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non-empty vector


<pre><code><b>const</b> [big_vector.md#0x1_big_vector_EVECTOR_NOT_EMPTY](EVECTOR_NOT_EMPTY): u64 = 2;
</code></pre>



<a id="0x1_big_vector_EZERO_BUCKET_SIZE"></a>

bucket_size cannot be 0


<pre><code><b>const</b> [big_vector.md#0x1_big_vector_EZERO_BUCKET_SIZE](EZERO_BUCKET_SIZE): u64 = 4;
</code></pre>



<a id="0x1_big_vector_empty"></a>

## Function `empty`

Regular Vector API
Create an empty vector.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_empty](empty)&lt;T: store&gt;(bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_empty](empty)&lt;T: store&gt;(bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt; {
    <b>assert</b>!(bucket_size &gt; 0, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EZERO_BUCKET_SIZE](EZERO_BUCKET_SIZE)));
    [big_vector.md#0x1_big_vector_BigVector](BigVector) {
        buckets: [table_with_length.md#0x1_table_with_length_new](table_with_length::new)(),
        end_index: 0,
        bucket_size,
    }
}
</code></pre>



</details>

<a id="0x1_big_vector_singleton"></a>

## Function `singleton`

Create a vector of length 1 containing the passed in element.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_singleton](singleton)&lt;T: store&gt;(element: T, bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_singleton](singleton)&lt;T: store&gt;(element: T, bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt; {
    <b>let</b> v = [big_vector.md#0x1_big_vector_empty](empty)(bucket_size);
    [big_vector.md#0x1_big_vector_push_back](push_back)(&<b>mut</b> v, element);
    v
}
</code></pre>



</details>

<a id="0x1_big_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector <code>v</code>.
Aborts if <code>v</code> is not empty.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;) {
    <b>assert</b>!([big_vector.md#0x1_big_vector_is_empty](is_empty)(&v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EVECTOR_NOT_EMPTY](EVECTOR_NOT_EMPTY)));
    <b>let</b> [big_vector.md#0x1_big_vector_BigVector](BigVector) { buckets, end_index: _, bucket_size: _ } = v;
    [table_with_length.md#0x1_table_with_length_destroy_empty](table_with_length::destroy_empty)(buckets);
}
</code></pre>



</details>

<a id="0x1_big_vector_destroy"></a>

## Function `destroy`

Destroy the vector <code>v</code> if T has <code>drop</code>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_destroy](destroy)&lt;T: drop&gt;(v: [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_destroy](destroy)&lt;T: drop&gt;(v: [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;) {
    <b>let</b> [big_vector.md#0x1_big_vector_BigVector](BigVector) { buckets, end_index, bucket_size: _ } = v;
    <b>let</b> i = 0;
    <b>while</b> (end_index &gt; 0) {
        <b>let</b> num_elements = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&[table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> buckets, i));
        end_index = end_index - num_elements;
        i = i + 1;
    };
    [table_with_length.md#0x1_table_with_length_destroy_empty](table_with_length::destroy_empty)(buckets);
}
</code></pre>



</details>

<a id="0x1_big_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>i</code>th element of the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow](borrow)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow](borrow)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64): &T {
    <b>assert</b>!(i &lt; [big_vector.md#0x1_big_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)([table_with_length.md#0x1_table_with_length_borrow](table_with_length::borrow)(&v.buckets, i / v.bucket_size), i % v.bucket_size)
}
</code></pre>



</details>

<a id="0x1_big_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>i</code>th element in the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow_mut](borrow_mut)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow_mut](borrow_mut)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64): &<b>mut</b> T {
    <b>assert</b>!(i &lt; [big_vector.md#0x1_big_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    [../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut](vector::borrow_mut)([table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, i / v.bucket_size), i % v.bucket_size)
}
</code></pre>



</details>

<a id="0x1_big_vector_append"></a>

## Function `append`

Empty and destroy the other vector, and push each of the elements in the other vector onto the lhs vector in the
same order as they occurred in other.
Disclaimer: This function is costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, other: [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, other: [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;) {
    <b>let</b> other_len = [big_vector.md#0x1_big_vector_length](length)(&other);
    <b>let</b> half_other_len = other_len / 2;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; half_other_len) {
        [big_vector.md#0x1_big_vector_push_back](push_back)(lhs, [big_vector.md#0x1_big_vector_swap_remove](swap_remove)(&<b>mut</b> other, i));
        i = i + 1;
    };
    <b>while</b> (i &lt; other_len) {
        [big_vector.md#0x1_big_vector_push_back](push_back)(lhs, [big_vector.md#0x1_big_vector_pop_back](pop_back)(&<b>mut</b> other));
        i = i + 1;
    };
    [big_vector.md#0x1_big_vector_destroy_empty](destroy_empty)(other);
}
</code></pre>



</details>

<a id="0x1_big_vector_push_back"></a>

## Function `push_back`

Add element <code>val</code> to the end of the vector <code>v</code>. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, val: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, val: T) {
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>if</b> (v.end_index == num_buckets * v.bucket_size) {
        [table_with_length.md#0x1_table_with_length_add](table_with_length::add)(&<b>mut</b> v.buckets, num_buckets, [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)());
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)([table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, num_buckets), val);
    } <b>else</b> {
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)([table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, num_buckets - 1), val);
    };
    v.end_index = v.end_index + 1;
}
</code></pre>



</details>

<a id="0x1_big_vector_pop_back"></a>

## Function `pop_back`

Pop an element from the end of vector <code>v</code>. It doesn't shrink the buckets even if they're empty.
Call <code>shrink_to_fit</code> explicity to deallocate empty buckets.
Aborts if <code>v</code> is empty.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;): T {
    <b>assert</b>!(![big_vector.md#0x1_big_vector_is_empty](is_empty)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([big_vector.md#0x1_big_vector_EVECTOR_EMPTY](EVECTOR_EMPTY)));
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>let</b> last_bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, num_buckets - 1);
    <b>let</b> val = [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(last_bucket);
    // Shrink the [table.md#0x1_table](table) <b>if</b> the last [../../move-stdlib/doc/vector.md#0x1_vector](vector) is empty.
    <b>if</b> ([../../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(last_bucket)) {
        <b>move</b> last_bucket;
        [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)([table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> v.buckets, num_buckets - 1));
    };
    v.end_index = v.end_index - 1;
    val
}
</code></pre>



</details>

<a id="0x1_big_vector_remove"></a>

## Function `remove`

Remove the element at index i in the vector v and return the owned value that was previously stored at i in v.
All elements occurring at indices greater than i will be shifted down by 1. Will abort if i is out of bounds.
Disclaimer: This function is costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64): T {
    <b>let</b> len = [big_vector.md#0x1_big_vector_length](length)(v);
    <b>assert</b>!(i &lt; len, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>let</b> cur_bucket_index = i / v.bucket_size + 1;
    <b>let</b> cur_bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, cur_bucket_index - 1);
    <b>let</b> res = [../../move-stdlib/doc/vector.md#0x1_vector_remove](vector::remove)(cur_bucket, i % v.bucket_size);
    v.end_index = v.end_index - 1;
    <b>move</b> cur_bucket;
    <b>while</b> ({
        <b>spec</b> {
            <b>invariant</b> cur_bucket_index &lt;= num_buckets;
            <b>invariant</b> [table_with_length.md#0x1_table_with_length_spec_len](table_with_length::spec_len)(v.buckets) == num_buckets;
        };
        (cur_bucket_index &lt; num_buckets)
    }) {
        // remove one element from the start of current [../../move-stdlib/doc/vector.md#0x1_vector](vector)
        <b>let</b> cur_bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, cur_bucket_index);
        <b>let</b> t = [../../move-stdlib/doc/vector.md#0x1_vector_remove](vector::remove)(cur_bucket, 0);
        <b>move</b> cur_bucket;
        // and put it at the end of the last one
        <b>let</b> prev_bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, cur_bucket_index - 1);
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(prev_bucket, t);
        cur_bucket_index = cur_bucket_index + 1;
    };
    <b>spec</b> {
        <b>assert</b> cur_bucket_index == num_buckets;
    };

    // Shrink the [table.md#0x1_table](table) <b>if</b> the last [../../move-stdlib/doc/vector.md#0x1_vector](vector) is empty.
    <b>let</b> last_bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, num_buckets - 1);
    <b>if</b> ([../../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(last_bucket)) {
        <b>move</b> last_bucket;
        [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)([table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> v.buckets, num_buckets - 1));
    };

    res
}
</code></pre>



</details>

<a id="0x1_big_vector_swap_remove"></a>

## Function `swap_remove`

Swap the <code>i</code>th element of the vector <code>v</code> with the last element and then pop the vector.
This is O(1), but does not preserve ordering of elements in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64): T {
    <b>assert</b>!(i &lt; [big_vector.md#0x1_big_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> last_val = [big_vector.md#0x1_big_vector_pop_back](pop_back)(v);
    // <b>if</b> the requested value is the last one, <b>return</b> it
    <b>if</b> (v.end_index == i) {
        <b>return</b> last_val
    };
    // because the lack of mem::swap, here we swap remove the requested value from the bucket
    // and append the last_val <b>to</b> the bucket then swap the last bucket val back
    <b>let</b> bucket = [table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, i / v.bucket_size);
    <b>let</b> bucket_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(bucket);
    <b>let</b> val = [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(bucket, i % v.bucket_size);
    [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(bucket, last_val);
    [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(bucket, i % v.bucket_size, bucket_len - 1);
    val
}
</code></pre>



</details>

<a id="0x1_big_vector_swap"></a>

## Function `swap`

Swap the elements at the i'th and j'th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap](swap)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap](swap)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64, j: u64) {
    <b>assert</b>!(i &lt; [big_vector.md#0x1_big_vector_length](length)(v) && j &lt; [big_vector.md#0x1_big_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> i_bucket_index = i / v.bucket_size;
    <b>let</b> j_bucket_index = j / v.bucket_size;
    <b>let</b> i_vector_index = i % v.bucket_size;
    <b>let</b> j_vector_index = j % v.bucket_size;
    <b>if</b> (i_bucket_index == j_bucket_index) {
        [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)([table_with_length.md#0x1_table_with_length_borrow_mut](table_with_length::borrow_mut)(&<b>mut</b> v.buckets, i_bucket_index), i_vector_index, j_vector_index);
        <b>return</b>
    };
    // If i and j are in different buckets, take the buckets out first for easy mutation.
    <b>let</b> bucket_i = [table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> v.buckets, i_bucket_index);
    <b>let</b> bucket_j = [table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> v.buckets, j_bucket_index);
    // Get the elements from buckets by calling `swap_remove`.
    <b>let</b> element_i = [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(&<b>mut</b> bucket_i, i_vector_index);
    <b>let</b> element_j = [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(&<b>mut</b> bucket_j, j_vector_index);
    // Swap the elements and push back <b>to</b> the other bucket.
    [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> bucket_i, element_j);
    [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> bucket_j, element_i);
    <b>let</b> last_index_in_bucket_i = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&bucket_i) - 1;
    <b>let</b> last_index_in_bucket_j = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&bucket_j) - 1;
    // Re-position the swapped elements <b>to</b> the right index.
    [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(&<b>mut</b> bucket_i, i_vector_index, last_index_in_bucket_i);
    [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(&<b>mut</b> bucket_j, j_vector_index, last_index_in_bucket_j);
    // Add back the buckets.
    [table_with_length.md#0x1_table_with_length_add](table_with_length::add)(&<b>mut</b> v.buckets, i_bucket_index, bucket_i);
    [table_with_length.md#0x1_table_with_length_add](table_with_length::add)(&<b>mut</b> v.buckets, j_bucket_index, bucket_j);
}
</code></pre>



</details>

<a id="0x1_big_vector_reverse"></a>

## Function `reverse`

Reverse the order of the elements in the vector v in-place.
Disclaimer: This function is costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_reverse](reverse)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_reverse](reverse)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;) {
    <b>let</b> new_buckets = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    <b>let</b> push_bucket = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>let</b> num_buckets_left = num_buckets;

    <b>while</b> (num_buckets_left &gt; 0) {
        <b>let</b> pop_bucket = [table_with_length.md#0x1_table_with_length_remove](table_with_length::remove)(&<b>mut</b> v.buckets, num_buckets_left - 1);
        [../../move-stdlib/doc/vector.md#0x1_vector_for_each_reverse](vector::for_each_reverse)(pop_bucket, |val| {
            [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> push_bucket, val);
            <b>if</b> ([../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&push_bucket) == v.bucket_size) {
                [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> new_buckets, push_bucket);
                push_bucket = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
            };
        });
        num_buckets_left = num_buckets_left - 1;
    };

    <b>if</b> ([../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&push_bucket) &gt; 0) {
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> new_buckets, push_bucket);
    } <b>else</b> {
        [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(push_bucket);
    };

    [../../move-stdlib/doc/vector.md#0x1_vector_reverse](vector::reverse)(&<b>mut</b> new_buckets);
    <b>let</b> i = 0;
    <b>assert</b>!([table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets) == 0, 0);
    <b>while</b> (i &lt; num_buckets) {
        [table_with_length.md#0x1_table_with_length_add](table_with_length::add)(&<b>mut</b> v.buckets, i, [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> new_buckets));
        i = i + 1;
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(new_buckets);
}
</code></pre>



</details>

<a id="0x1_big_vector_index_of"></a>

## Function `index_of`

Return the index of the first occurrence of an element in v that is equal to e. Returns (true, index) if such an
element was found, and (false, 0) otherwise.
Disclaimer: This function is costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_index_of](index_of)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, val: &T): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_index_of](index_of)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, val: &T): (bool, u64) {
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>let</b> bucket_index = 0;
    <b>while</b> (bucket_index &lt; num_buckets) {
        <b>let</b> cur = [table_with_length.md#0x1_table_with_length_borrow](table_with_length::borrow)(&v.buckets, bucket_index);
        <b>let</b> (found, i) = [../../move-stdlib/doc/vector.md#0x1_vector_index_of](vector::index_of)(cur, val);
        <b>if</b> (found) {
            <b>return</b> (<b>true</b>, bucket_index * v.bucket_size + i)
        };
        bucket_index = bucket_index + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a id="0x1_big_vector_contains"></a>

## Function `contains`

Return if an element equal to e exists in the vector v.
Disclaimer: This function is costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_contains](contains)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, val: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_contains](contains)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, val: &T): bool {
    <b>if</b> ([big_vector.md#0x1_big_vector_is_empty](is_empty)(v)) <b>return</b> <b>false</b>;
    <b>let</b> (exist, _) = [big_vector.md#0x1_big_vector_index_of](index_of)(v, val);
    exist
}
</code></pre>



</details>

<a id="0x1_big_vector_to_vector"></a>

## Function `to_vector`

Convert a big vector to a native vector, which is supposed to be called mostly by view functions to get an
atomic view of the whole vector.
Disclaimer: This function may be costly as the big vector may be huge in size. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_to_vector](to_vector)&lt;T: <b>copy</b>&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_to_vector](to_vector)&lt;T: <b>copy</b>&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt; {
    <b>let</b> res = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    <b>let</b> num_buckets = [table_with_length.md#0x1_table_with_length_length](table_with_length::length)(&v.buckets);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; num_buckets) {
        [../../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)(&<b>mut</b> res, *[table_with_length.md#0x1_table_with_length_borrow](table_with_length::borrow)(&v.buckets, i));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a id="0x1_big_vector_length"></a>

## Function `length`

Return the length of the vector.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_length](length)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_length](length)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;): u64 {
    v.end_index
}
</code></pre>



</details>

<a id="0x1_big_vector_is_empty"></a>

## Function `is_empty`

Return <code><b>true</b></code> if the vector <code>v</code> has no elements and <code><b>false</b></code> otherwise.


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_is_empty](is_empty)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_is_empty](is_empty)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;): bool {
    [big_vector.md#0x1_big_vector_length](length)(v) == 0
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_BigVector"></a>

### Struct `BigVector`


<pre><code><b>struct</b> [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt; <b>has</b> store
</code></pre>



<dl>
<dt>
<code>buckets: [table_with_length.md#0x1_table_with_length_TableWithLength](table_with_length::TableWithLength)&lt;u64, [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>end_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>bucket_size: u64</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>invariant</b> bucket_size != 0;
<b>invariant</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0 ==&gt; end_index == 0;
<b>invariant</b> end_index == 0 ==&gt; [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0;
<b>invariant</b> end_index &lt;= [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) * bucket_size;
<b>invariant</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0
    || (<b>forall</b> i in 0..[big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets)-1: len([table_with_length.md#0x1_table_with_length_spec_get](table_with_length::spec_get)(buckets, i)) == bucket_size);
<b>invariant</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0
    || len([table_with_length.md#0x1_table_with_length_spec_get](table_with_length::spec_get)(buckets, [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) -1 )) &lt;= bucket_size;
<b>invariant</b> <b>forall</b> i in 0..[big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets): [big_vector.md#0x1_big_vector_spec_table_contains](spec_table_contains)(buckets, i);
<b>invariant</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == (end_index + bucket_size - 1) / bucket_size;
<b>invariant</b> ([big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0 && end_index == 0)
    || ([big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) != 0 && (([big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) - 1) * bucket_size) + (len([table_with_length.md#0x1_table_with_length_spec_get](table_with_length::spec_get)(buckets, [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) - 1))) == end_index);
<b>invariant</b> <b>forall</b> i: u64 <b>where</b> i &gt;= [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets):  {
    ![big_vector.md#0x1_big_vector_spec_table_contains](spec_table_contains)(buckets, i)
};
<b>invariant</b> <b>forall</b> i: u64 <b>where</b> i &lt; [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets):  {
    [big_vector.md#0x1_big_vector_spec_table_contains](spec_table_contains)(buckets, i)
};
<b>invariant</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) == 0
    || (len([table_with_length.md#0x1_table_with_length_spec_get](table_with_length::spec_get)(buckets, [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(buckets) - 1)) &gt; 0);
</code></pre>



<a id="@Specification_1_empty"></a>

### Function `empty`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_empty](empty)&lt;T: store&gt;(bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;
</code></pre>




<pre><code><b>aborts_if</b> bucket_size == 0;
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(result) == 0;
<b>ensures</b> result.bucket_size == bucket_size;
</code></pre>



<a id="@Specification_1_singleton"></a>

### Function `singleton`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [big_vector.md#0x1_big_vector_singleton](singleton)&lt;T: store&gt;(element: T, bucket_size: u64): [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;
</code></pre>




<pre><code><b>aborts_if</b> bucket_size == 0;
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(result) == 1;
<b>ensures</b> result.bucket_size == bucket_size;
</code></pre>



<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>




<pre><code><b>aborts_if</b> ![big_vector.md#0x1_big_vector_is_empty](is_empty)(v);
</code></pre>



<a id="@Specification_1_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow](borrow)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): &T
</code></pre>




<pre><code><b>aborts_if</b> i &gt;= [big_vector.md#0x1_big_vector_length](length)(v);
<b>ensures</b> result == [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, i);
</code></pre>



<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_borrow_mut](borrow_mut)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): &<b>mut</b> T
</code></pre>




<pre><code><b>aborts_if</b> i &gt;= [big_vector.md#0x1_big_vector_length](length)(v);
<b>ensures</b> result == [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, i);
</code></pre>



<a id="@Specification_1_append"></a>

### Function `append`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, other: [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>



<a id="@Specification_1_push_back"></a>

### Function `push_back`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, val: T)
</code></pre>




<pre><code><b>let</b> num_buckets = [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(v.buckets);
<b>include</b> [big_vector.md#0x1_big_vector_PushbackAbortsIf](PushbackAbortsIf)&lt;T&gt;;
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(v) == [big_vector.md#0x1_big_vector_length](length)(<b>old</b>(v)) + 1;
<b>ensures</b> v.end_index == <b>old</b>(v.end_index) + 1;
<b>ensures</b> [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, v.end_index-1) == val;
<b>ensures</b> <b>forall</b> i in 0..v.end_index-1: [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, i) == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), i);
<b>ensures</b> v.bucket_size == <b>old</b>(v).bucket_size;
</code></pre>




<a id="0x1_big_vector_PushbackAbortsIf"></a>


<pre><code><b>schema</b> [big_vector.md#0x1_big_vector_PushbackAbortsIf](PushbackAbortsIf)&lt;T&gt; {
    v: [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;;
    <b>let</b> num_buckets = [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)(v.buckets);
    <b>aborts_if</b> num_buckets * v.bucket_size &gt; MAX_U64;
    <b>aborts_if</b> v.end_index + 1 &gt; MAX_U64;
}
</code></pre>



<a id="@Specification_1_pop_back"></a>

### Function `pop_back`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;): T
</code></pre>




<pre><code><b>aborts_if</b> [big_vector.md#0x1_big_vector_is_empty](is_empty)(v);
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(v) == [big_vector.md#0x1_big_vector_length](length)(<b>old</b>(v)) - 1;
<b>ensures</b> result == <b>old</b>([big_vector.md#0x1_big_vector_spec_at](spec_at)(v, v.end_index-1));
<b>ensures</b> <b>forall</b> i in 0..v.end_index: [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, i) == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), i);
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): T
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>



<a id="@Specification_1_swap_remove"></a>

### Function `swap_remove`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64): T
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>aborts_if</b> i &gt;= [big_vector.md#0x1_big_vector_length](length)(v);
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(v) == [big_vector.md#0x1_big_vector_length](length)(<b>old</b>(v)) - 1;
<b>ensures</b> result == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), i);
</code></pre>



<a id="@Specification_1_swap"></a>

### Function `swap`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_swap](swap)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, i: u64, j: u64)
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 1000;
<b>aborts_if</b> i &gt;= [big_vector.md#0x1_big_vector_length](length)(v) || j &gt;= [big_vector.md#0x1_big_vector_length](length)(v);
<b>ensures</b> [big_vector.md#0x1_big_vector_length](length)(v) == [big_vector.md#0x1_big_vector_length](length)(<b>old</b>(v));
<b>ensures</b> [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, i) == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), j);
<b>ensures</b> [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, j) == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), i);
<b>ensures</b> <b>forall</b> idx in 0..[big_vector.md#0x1_big_vector_length](length)(v)
    <b>where</b> idx != i && idx != j:
    [big_vector.md#0x1_big_vector_spec_at](spec_at)(v, idx) == [big_vector.md#0x1_big_vector_spec_at](spec_at)(<b>old</b>(v), idx);
</code></pre>



<a id="@Specification_1_reverse"></a>

### Function `reverse`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_reverse](reverse)&lt;T&gt;(v: &<b>mut</b> [big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;)
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>



<a id="@Specification_1_index_of"></a>

### Function `index_of`


<pre><code><b>public</b> <b>fun</b> [big_vector.md#0x1_big_vector_index_of](index_of)&lt;T&gt;(v: &[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;, val: &T): (bool, u64)
</code></pre>




<pre><code><b>pragma</b> verify=<b>false</b>;
</code></pre>




<a id="0x1_big_vector_spec_table_len"></a>


<pre><code><b>fun</b> [big_vector.md#0x1_big_vector_spec_table_len](spec_table_len)&lt;K, V&gt;(t: TableWithLength&lt;K, V&gt;): u64 {
   [table_with_length.md#0x1_table_with_length_spec_len](table_with_length::spec_len)(t)
}
</code></pre>




<a id="0x1_big_vector_spec_table_contains"></a>


<pre><code><b>fun</b> [big_vector.md#0x1_big_vector_spec_table_contains](spec_table_contains)&lt;K, V&gt;(t: TableWithLength&lt;K, V&gt;, k: K): bool {
   [table_with_length.md#0x1_table_with_length_spec_contains](table_with_length::spec_contains)(t, k)
}
</code></pre>




<a id="0x1_big_vector_spec_at"></a>


<pre><code><b>fun</b> [big_vector.md#0x1_big_vector_spec_at](spec_at)&lt;T&gt;(v: [big_vector.md#0x1_big_vector_BigVector](BigVector)&lt;T&gt;, i: u64): T {
   <b>let</b> bucket = i / v.bucket_size;
   <b>let</b> idx = i % v.bucket_size;
   <b>let</b> v = [table_with_length.md#0x1_table_with_length_spec_get](table_with_length::spec_get)(v.buckets, bucket);
   v[idx]
}
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
