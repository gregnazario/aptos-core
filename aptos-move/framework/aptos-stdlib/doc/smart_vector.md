
<a id="0x1_smart_vector"></a>

# Module `0x1::smart_vector`



-  [Struct `SmartVector`](#0x1_smart_vector_SmartVector)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_smart_vector_new)
-  [Function `empty`](#0x1_smart_vector_empty)
-  [Function `empty_with_config`](#0x1_smart_vector_empty_with_config)
-  [Function `singleton`](#0x1_smart_vector_singleton)
-  [Function `destroy_empty`](#0x1_smart_vector_destroy_empty)
-  [Function `destroy`](#0x1_smart_vector_destroy)
-  [Function `clear`](#0x1_smart_vector_clear)
-  [Function `borrow`](#0x1_smart_vector_borrow)
-  [Function `borrow_mut`](#0x1_smart_vector_borrow_mut)
-  [Function `append`](#0x1_smart_vector_append)
-  [Function `add_all`](#0x1_smart_vector_add_all)
-  [Function `to_vector`](#0x1_smart_vector_to_vector)
-  [Function `push_back`](#0x1_smart_vector_push_back)
-  [Function `pop_back`](#0x1_smart_vector_pop_back)
-  [Function `remove`](#0x1_smart_vector_remove)
-  [Function `swap_remove`](#0x1_smart_vector_swap_remove)
-  [Function `swap`](#0x1_smart_vector_swap)
-  [Function `reverse`](#0x1_smart_vector_reverse)
-  [Function `index_of`](#0x1_smart_vector_index_of)
-  [Function `contains`](#0x1_smart_vector_contains)
-  [Function `length`](#0x1_smart_vector_length)
-  [Function `is_empty`](#0x1_smart_vector_is_empty)
-  [Function `for_each`](#0x1_smart_vector_for_each)
-  [Function `for_each_reverse`](#0x1_smart_vector_for_each_reverse)
-  [Function `for_each_ref`](#0x1_smart_vector_for_each_ref)
-  [Function `for_each_mut`](#0x1_smart_vector_for_each_mut)
-  [Function `enumerate_ref`](#0x1_smart_vector_enumerate_ref)
-  [Function `enumerate_mut`](#0x1_smart_vector_enumerate_mut)
-  [Function `fold`](#0x1_smart_vector_fold)
-  [Function `foldr`](#0x1_smart_vector_foldr)
-  [Function `map_ref`](#0x1_smart_vector_map_ref)
-  [Function `map`](#0x1_smart_vector_map)
-  [Function `filter`](#0x1_smart_vector_filter)
-  [Function `zip`](#0x1_smart_vector_zip)
-  [Function `zip_reverse`](#0x1_smart_vector_zip_reverse)
-  [Function `zip_ref`](#0x1_smart_vector_zip_ref)
-  [Function `zip_mut`](#0x1_smart_vector_zip_mut)
-  [Function `zip_map`](#0x1_smart_vector_zip_map)
-  [Function `zip_map_ref`](#0x1_smart_vector_zip_map_ref)
-  [Specification](#@Specification_1)
    -  [Struct `SmartVector`](#@Specification_1_SmartVector)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `empty_with_config`](#@Specification_1_empty_with_config)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `push_back`](#@Specification_1_push_back)
    -  [Function `pop_back`](#@Specification_1_pop_back)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `swap`](#@Specification_1_swap)
    -  [Function `length`](#@Specification_1_length)


<pre><code><b>use</b> [big_vector.md#0x1_big_vector](0x1::big_vector);
<b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [math64.md#0x1_math64](0x1::math64);
<b>use</b> [../../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [type_info.md#0x1_type_info](0x1::type_info);
<b>use</b> [../../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_smart_vector_SmartVector"></a>

## Struct `SmartVector`

A Scalable vector implementation based on tables, Ts are grouped into buckets with <code>bucket_size</code>.
The option wrapping BigVector saves space in the metadata associated with BigVector when smart_vector is
so small that inline_vec vector can hold all the data.


<pre><code><b>struct</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inline_vec: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>big_vec: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>inline_capacity: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bucket_size: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_smart_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


<pre><code><b>const</b> [smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS): u64 = 1;
</code></pre>



<a id="0x1_smart_vector_EVECTOR_EMPTY"></a>

Cannot pop back from an empty vector


<pre><code><b>const</b> [smart_vector.md#0x1_smart_vector_EVECTOR_EMPTY](EVECTOR_EMPTY): u64 = 3;
</code></pre>



<a id="0x1_smart_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non-empty vector


<pre><code><b>const</b> [smart_vector.md#0x1_smart_vector_EVECTOR_NOT_EMPTY](EVECTOR_NOT_EMPTY): u64 = 2;
</code></pre>



<a id="0x1_smart_vector_EZERO_BUCKET_SIZE"></a>

bucket_size cannot be 0


<pre><code><b>const</b> [smart_vector.md#0x1_smart_vector_EZERO_BUCKET_SIZE](EZERO_BUCKET_SIZE): u64 = 4;
</code></pre>



<a id="0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH"></a>

The length of the smart vectors are not equal.


<pre><code><b>const</b> [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH): u64 = 131077;
</code></pre>



<a id="0x1_smart_vector_new"></a>

## Function `new`

Regular Vector API
Create an empty vector using default logic to estimate <code>inline_capacity</code> and <code>bucket_size</code>, which may be
inaccurate.
This is exactly the same as empty() but is more standardized as all other data structures have new().


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_new](new)&lt;T: store&gt;(): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_new](new)&lt;T: store&gt;(): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; {
    [smart_vector.md#0x1_smart_vector_empty](empty)()
}
</code></pre>



</details>

<a id="0x1_smart_vector_empty"></a>

## Function `empty`

Create an empty vector using default logic to estimate <code>inline_capacity</code> and <code>bucket_size</code>, which may be
inaccurate.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty](empty)&lt;T: store&gt;(): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty](empty)&lt;T: store&gt;(): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; {
    [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector) {
        inline_vec: [../../move-stdlib/doc/vector.md#0x1_vector](vector)[],
        big_vec: [../../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        inline_capacity: [../../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        bucket_size: [../../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_empty_with_config"></a>

## Function `empty_with_config`

Create an empty vector with customized config.
When inline_capacity = 0, SmartVector degrades to a wrapper of BigVector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty_with_config](empty_with_config)&lt;T: store&gt;(inline_capacity: u64, bucket_size: u64): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty_with_config](empty_with_config)&lt;T: store&gt;(inline_capacity: u64, bucket_size: u64): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; {
    <b>assert</b>!(bucket_size &gt; 0, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EZERO_BUCKET_SIZE](EZERO_BUCKET_SIZE)));
    [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector) {
        inline_vec: [../../move-stdlib/doc/vector.md#0x1_vector](vector)[],
        big_vec: [../../move-stdlib/doc/option.md#0x1_option_none](option::none)(),
        inline_capacity: [../../move-stdlib/doc/option.md#0x1_option_some](option::some)(inline_capacity),
        bucket_size: [../../move-stdlib/doc/option.md#0x1_option_some](option::some)(bucket_size),
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_singleton"></a>

## Function `singleton`

Create a vector of length 1 containing the passed in T.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_singleton](singleton)&lt;T: store&gt;(element: T): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_singleton](singleton)&lt;T: store&gt;(element: T): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; {
    <b>let</b> v = [smart_vector.md#0x1_smart_vector_empty](empty)();
    [smart_vector.md#0x1_smart_vector_push_back](push_back)(&<b>mut</b> v, element);
    v
}
</code></pre>



</details>

<a id="0x1_smart_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector <code>v</code>.
Aborts if <code>v</code> is not empty.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;) {
    <b>assert</b>!([smart_vector.md#0x1_smart_vector_is_empty](is_empty)(&v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EVECTOR_NOT_EMPTY](EVECTOR_NOT_EMPTY)));
    <b>let</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector) { inline_vec, big_vec, inline_capacity: _, bucket_size: _ } = v;
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(inline_vec);
    [../../move-stdlib/doc/option.md#0x1_option_destroy_none](option::destroy_none)(big_vec);
}
</code></pre>



</details>

<a id="0x1_smart_vector_destroy"></a>

## Function `destroy`

Destroy a vector completely when T has <code>drop</code>.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_destroy](destroy)&lt;T: drop&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_destroy](destroy)&lt;T: drop&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;) {
    [smart_vector.md#0x1_smart_vector_clear](clear)(&<b>mut</b> v);
    [smart_vector.md#0x1_smart_vector_destroy_empty](destroy_empty)(v);
}
</code></pre>



</details>

<a id="0x1_smart_vector_clear"></a>

## Function `clear`

Clear a vector completely when T has <code>drop</code>.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_clear](clear)&lt;T: drop&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_clear](clear)&lt;T: drop&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;) {
    v.inline_vec = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&v.big_vec)) {
        [big_vector.md#0x1_big_vector_destroy](big_vector::destroy)([../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(&<b>mut</b> v.big_vec));
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>i</code>th T of the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_borrow](borrow)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_borrow](borrow)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, i: u64): &T {
    <b>assert</b>!(i &lt; [smart_vector.md#0x1_smart_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>if</b> (i &lt; inline_len) {
        [../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&v.inline_vec, i)
    } <b>else</b> {
        [big_vector.md#0x1_big_vector_borrow](big_vector::borrow)([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.big_vec), i - inline_len)
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>i</code>th T in the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_borrow_mut](borrow_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_borrow_mut](borrow_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, i: u64): &<b>mut</b> T {
    <b>assert</b>!(i &lt; [smart_vector.md#0x1_smart_vector_length](length)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>if</b> (i &lt; inline_len) {
        [../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut](vector::borrow_mut)(&<b>mut</b> v.inline_vec, i)
    } <b>else</b> {
        [big_vector.md#0x1_big_vector_borrow_mut](big_vector::borrow_mut)([../../move-stdlib/doc/option.md#0x1_option_borrow_mut](option::borrow_mut)(&<b>mut</b> v.big_vec), i - inline_len)
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_append"></a>

## Function `append`

Empty and destroy the other vector, and push each of the Ts in the other vector onto the lhs vector in the
same order as they occurred in other.
Disclaimer: This function may be costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, other: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, other: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;) {
    <b>let</b> other_len = [smart_vector.md#0x1_smart_vector_length](length)(&other);
    <b>let</b> half_other_len = other_len / 2;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; half_other_len) {
        [smart_vector.md#0x1_smart_vector_push_back](push_back)(lhs, [smart_vector.md#0x1_smart_vector_swap_remove](swap_remove)(&<b>mut</b> other, i));
        i = i + 1;
    };
    <b>while</b> (i &lt; other_len) {
        [smart_vector.md#0x1_smart_vector_push_back](push_back)(lhs, [smart_vector.md#0x1_smart_vector_pop_back](pop_back)(&<b>mut</b> other));
        i = i + 1;
    };
    [smart_vector.md#0x1_smart_vector_destroy_empty](destroy_empty)(other);
}
</code></pre>



</details>

<a id="0x1_smart_vector_add_all"></a>

## Function `add_all`

Add multiple values to the vector at once.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_add_all](add_all)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, vals: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_add_all](add_all)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, vals: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;) {
    [../../move-stdlib/doc/vector.md#0x1_vector_for_each](vector::for_each)(vals, |val| { [smart_vector.md#0x1_smart_vector_push_back](push_back)(v, val); })
}
</code></pre>



</details>

<a id="0x1_smart_vector_to_vector"></a>

## Function `to_vector`

Convert a smart vector to a native vector, which is supposed to be called mostly by view functions to get an
atomic view of the whole vector.
Disclaimer: This function may be costly as the smart vector may be huge in size. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_to_vector](to_vector)&lt;T: <b>copy</b>, store&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_to_vector](to_vector)&lt;T: store + <b>copy</b>&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt; {
    <b>let</b> res = v.inline_vec;
    <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&v.big_vec)) {
        <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.big_vec);
        [../../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)(&<b>mut</b> res, [big_vector.md#0x1_big_vector_to_vector](big_vector::to_vector)(big_vec));
    };
    res
}
</code></pre>



</details>

<a id="0x1_smart_vector_push_back"></a>

## Function `push_back`

Add T <code>val</code> to the end of the vector <code>v</code>. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, val: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, val: T) {
    <b>let</b> len = [smart_vector.md#0x1_smart_vector_length](length)(v);
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>if</b> (len == inline_len) {
        <b>let</b> bucket_size = <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&v.inline_capacity)) {
            <b>if</b> (len &lt; *[../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.inline_capacity)) {
                [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> v.inline_vec, val);
                <b>return</b>
            };
            *[../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.bucket_size)
        } <b>else</b> {
            <b>let</b> val_size = size_of_val(&val);
            <b>if</b> (val_size * (inline_len + 1) &lt; 150 /* magic number */) {
                [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> v.inline_vec, val);
                <b>return</b>
            };
            <b>let</b> estimated_avg_size = max((size_of_val(&v.inline_vec) + val_size) / (inline_len + 1), 1);
            max(1024 /* free_write_quota */ / estimated_avg_size, 1)
        };
        [../../move-stdlib/doc/option.md#0x1_option_fill](option::fill)(&<b>mut</b> v.big_vec, [big_vector.md#0x1_big_vector_empty](big_vector::empty)(bucket_size));
    };
    [big_vector.md#0x1_big_vector_push_back](big_vector::push_back)([../../move-stdlib/doc/option.md#0x1_option_borrow_mut](option::borrow_mut)(&<b>mut</b> v.big_vec), val);
}
</code></pre>



</details>

<a id="0x1_smart_vector_pop_back"></a>

## Function `pop_back`

Pop an T from the end of vector <code>v</code>. It does shrink the buckets if they're empty.
Aborts if <code>v</code> is empty.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;): T {
    <b>assert</b>!(![smart_vector.md#0x1_smart_vector_is_empty](is_empty)(v), [../../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([smart_vector.md#0x1_smart_vector_EVECTOR_EMPTY](EVECTOR_EMPTY)));
    <b>let</b> big_vec_wrapper = &<b>mut</b> v.big_vec;
    <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(big_vec_wrapper)) {
        <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(big_vec_wrapper);
        <b>let</b> val = [big_vector.md#0x1_big_vector_pop_back](big_vector::pop_back)(&<b>mut</b> big_vec);
        <b>if</b> ([big_vector.md#0x1_big_vector_is_empty](big_vector::is_empty)(&big_vec)) {
            [big_vector.md#0x1_big_vector_destroy_empty](big_vector::destroy_empty)(big_vec)
        } <b>else</b> {
            [../../move-stdlib/doc/option.md#0x1_option_fill](option::fill)(big_vec_wrapper, big_vec);
        };
        val
    } <b>else</b> {
        [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> v.inline_vec)
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_remove"></a>

## Function `remove`

Remove the T at index i in the vector v and return the owned value that was previously stored at i in v.
All Ts occurring at indices greater than i will be shifted down by 1. Will abort if i is out of bounds.
Disclaimer: This function may be costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, i: u64): T {
    <b>let</b> len = [smart_vector.md#0x1_smart_vector_length](length)(v);
    <b>assert</b>!(i &lt; len, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>if</b> (i &lt; inline_len) {
        [../../move-stdlib/doc/vector.md#0x1_vector_remove](vector::remove)(&<b>mut</b> v.inline_vec, i)
    } <b>else</b> {
        <b>let</b> big_vec_wrapper = &<b>mut</b> v.big_vec;
        <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(big_vec_wrapper);
        <b>let</b> val = [big_vector.md#0x1_big_vector_remove](big_vector::remove)(&<b>mut</b> big_vec, i - inline_len);
        <b>if</b> ([big_vector.md#0x1_big_vector_is_empty](big_vector::is_empty)(&big_vec)) {
            [big_vector.md#0x1_big_vector_destroy_empty](big_vector::destroy_empty)(big_vec)
        } <b>else</b> {
            [../../move-stdlib/doc/option.md#0x1_option_fill](option::fill)(big_vec_wrapper, big_vec);
        };
        val
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_swap_remove"></a>

## Function `swap_remove`

Swap the <code>i</code>th T of the vector <code>v</code> with the last T and then pop the vector.
This is O(1), but does not preserve ordering of Ts in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, i: u64): T {
    <b>let</b> len = [smart_vector.md#0x1_smart_vector_length](length)(v);
    <b>assert</b>!(i &lt; len, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>let</b> big_vec_wrapper = &<b>mut</b> v.big_vec;
    <b>let</b> inline_vec = &<b>mut</b> v.inline_vec;
    <b>if</b> (i &gt;= inline_len) {
        <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(big_vec_wrapper);
        <b>let</b> val = [big_vector.md#0x1_big_vector_swap_remove](big_vector::swap_remove)(&<b>mut</b> big_vec, i - inline_len);
        <b>if</b> ([big_vector.md#0x1_big_vector_is_empty](big_vector::is_empty)(&big_vec)) {
            [big_vector.md#0x1_big_vector_destroy_empty](big_vector::destroy_empty)(big_vec)
        } <b>else</b> {
            [../../move-stdlib/doc/option.md#0x1_option_fill](option::fill)(big_vec_wrapper, big_vec);
        };
        val
    } <b>else</b> {
        <b>if</b> (inline_len &lt; len) {
            <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_extract](option::extract)(big_vec_wrapper);
            <b>let</b> last_from_big_vec = [big_vector.md#0x1_big_vector_pop_back](big_vector::pop_back)(&<b>mut</b> big_vec);
            <b>if</b> ([big_vector.md#0x1_big_vector_is_empty](big_vector::is_empty)(&big_vec)) {
                [big_vector.md#0x1_big_vector_destroy_empty](big_vector::destroy_empty)(big_vec)
            } <b>else</b> {
                [../../move-stdlib/doc/option.md#0x1_option_fill](option::fill)(big_vec_wrapper, big_vec);
            };
            [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(inline_vec, last_from_big_vec);
        };
        [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(inline_vec, i)
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_swap"></a>

## Function `swap`

Swap the Ts at the i'th and j'th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap](swap)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap](swap)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, i: u64, j: u64) {
    <b>if</b> (i &gt; j) {
        <b>return</b> [smart_vector.md#0x1_smart_vector_swap](swap)(v, j, i)
    };
    <b>let</b> len = [smart_vector.md#0x1_smart_vector_length](length)(v);
    <b>assert</b>!(j &lt; len, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS](EINDEX_OUT_OF_BOUNDS)));
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>if</b> (i &gt;= inline_len) {
        [big_vector.md#0x1_big_vector_swap](big_vector::swap)([../../move-stdlib/doc/option.md#0x1_option_borrow_mut](option::borrow_mut)(&<b>mut</b> v.big_vec), i - inline_len, j - inline_len);
    } <b>else</b> <b>if</b> (j &lt; inline_len) {
        [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(&<b>mut</b> v.inline_vec, i, j);
    } <b>else</b> {
        <b>let</b> big_vec = [../../move-stdlib/doc/option.md#0x1_option_borrow_mut](option::borrow_mut)(&<b>mut</b> v.big_vec);
        <b>let</b> inline_vec = &<b>mut</b> v.inline_vec;
        <b>let</b> element_i = [../../move-stdlib/doc/vector.md#0x1_vector_swap_remove](vector::swap_remove)(inline_vec, i);
        <b>let</b> element_j = [big_vector.md#0x1_big_vector_swap_remove](big_vector::swap_remove)(big_vec, j - inline_len);
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(inline_vec, element_j);
        [../../move-stdlib/doc/vector.md#0x1_vector_swap](vector::swap)(inline_vec, i, inline_len - 1);
        [big_vector.md#0x1_big_vector_push_back](big_vector::push_back)(big_vec, element_i);
        [big_vector.md#0x1_big_vector_swap](big_vector::swap)(big_vec, j - inline_len, len - inline_len - 1);
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_reverse"></a>

## Function `reverse`

Reverse the order of the Ts in the vector v in-place.
Disclaimer: This function may be costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_reverse](reverse)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_reverse](reverse)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;) {
    <b>let</b> inline_len = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec);
    <b>let</b> i = 0;
    <b>let</b> new_inline_vec = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    // Push the last `inline_len` Ts into a temp [../../move-stdlib/doc/vector.md#0x1_vector](vector).
    <b>while</b> (i &lt; inline_len) {
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> new_inline_vec, [smart_vector.md#0x1_smart_vector_pop_back](pop_back)(v));
        i = i + 1;
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_reverse](vector::reverse)(&<b>mut</b> new_inline_vec);
    // Reverse the [big_vector.md#0x1_big_vector](big_vector) left <b>if</b> <b>exists</b>.
    <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&v.big_vec)) {
        [big_vector.md#0x1_big_vector_reverse](big_vector::reverse)([../../move-stdlib/doc/option.md#0x1_option_borrow_mut](option::borrow_mut)(&<b>mut</b> v.big_vec));
    };
    // Mem::swap the two vectors.
    <b>let</b> temp_vec = [../../move-stdlib/doc/vector.md#0x1_vector](vector)[];
    <b>while</b> (![../../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(&<b>mut</b> v.inline_vec)) {
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> temp_vec, [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> v.inline_vec));
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_reverse](vector::reverse)(&<b>mut</b> temp_vec);
    <b>while</b> (![../../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(&<b>mut</b> new_inline_vec)) {
        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> v.inline_vec, [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> new_inline_vec));
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(new_inline_vec);
    // Push the rest Ts originally left in inline_vector back <b>to</b> the end of the smart [../../move-stdlib/doc/vector.md#0x1_vector](vector).
    <b>while</b> (![../../move-stdlib/doc/vector.md#0x1_vector_is_empty](vector::is_empty)(&<b>mut</b> temp_vec)) {
        [smart_vector.md#0x1_smart_vector_push_back](push_back)(v, [../../move-stdlib/doc/vector.md#0x1_vector_pop_back](vector::pop_back)(&<b>mut</b> temp_vec));
    };
    [../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty](vector::destroy_empty)(temp_vec);
}
</code></pre>



</details>

<a id="0x1_smart_vector_index_of"></a>

## Function `index_of`

Return <code>(<b>true</b>, i)</code> if <code>val</code> is in the vector <code>v</code> at index <code>i</code>.
Otherwise, returns <code>(<b>false</b>, 0)</code>.
Disclaimer: This function may be costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_index_of](index_of)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, val: &T): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_index_of](index_of)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, val: &T): (bool, u64) {
    <b>let</b> (found, i) = [../../move-stdlib/doc/vector.md#0x1_vector_index_of](vector::index_of)(&v.inline_vec, val);
    <b>if</b> (found) {
        (<b>true</b>, i)
    } <b>else</b> <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(&v.big_vec)) {
        <b>let</b> (found, i) = [big_vector.md#0x1_big_vector_index_of](big_vector::index_of)([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.big_vec), val);
        (found, i + [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec))
    } <b>else</b> {
        (<b>false</b>, 0)
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_contains"></a>

## Function `contains`

Return true if <code>val</code> is in the vector <code>v</code>.
Disclaimer: This function may be costly. Use it at your own discretion.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_contains](contains)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, val: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_contains](contains)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, val: &T): bool {
    <b>if</b> ([smart_vector.md#0x1_smart_vector_is_empty](is_empty)(v)) <b>return</b> <b>false</b>;
    <b>let</b> (exist, _) = [smart_vector.md#0x1_smart_vector_index_of](index_of)(v, val);
    exist
}
</code></pre>



</details>

<a id="0x1_smart_vector_length"></a>

## Function `length`

Return the length of the vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_length](length)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_length](length)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;): u64 {
    [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&v.inline_vec) + <b>if</b> ([../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(&v.big_vec)) {
        0
    } <b>else</b> {
        [big_vector.md#0x1_big_vector_length](big_vector::length)([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(&v.big_vec))
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_is_empty"></a>

## Function `is_empty`

Return <code><b>true</b></code> if the vector <code>v</code> has no Ts and <code><b>false</b></code> otherwise.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_is_empty](is_empty)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_is_empty](is_empty)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;): bool {
    [smart_vector.md#0x1_smart_vector_length](length)(v) == 0
}
</code></pre>



</details>

<a id="0x1_smart_vector_for_each"></a>

## Function `for_each`

Apply the function to each T in the vector, consuming it.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each](for_each)&lt;T: store&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |T|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each](for_each)&lt;T: store&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |T|) {
    aptos_std::smart_vector::reverse(&<b>mut</b> v); // We need <b>to</b> reverse the [../../move-stdlib/doc/vector.md#0x1_vector](vector) <b>to</b> consume it efficiently
    aptos_std::smart_vector::for_each_reverse(v, |e| f(e));
}
</code></pre>



</details>

<a id="0x1_smart_vector_for_each_reverse"></a>

## Function `for_each_reverse`

Apply the function to each T in the vector, consuming it.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_reverse](for_each_reverse)&lt;T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |T|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_reverse](for_each_reverse)&lt;T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |T|) {
    <b>let</b> len = aptos_std::smart_vector::length(&v);
    <b>while</b> (len &gt; 0) {
        f(aptos_std::smart_vector::pop_back(&<b>mut</b> v));
        len = len - 1;
    };
    aptos_std::smart_vector::destroy_empty(v)
}
</code></pre>



</details>

<a id="0x1_smart_vector_for_each_ref"></a>

## Function `for_each_ref`

Apply the function to a reference of each T in the vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_ref](for_each_ref)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |&T|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_ref](for_each_ref)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |&T|) {
    <b>let</b> i = 0;
    <b>let</b> len = aptos_std::smart_vector::length(v);
    <b>while</b> (i &lt; len) {
        f(aptos_std::smart_vector::borrow(v, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_for_each_mut"></a>

## Function `for_each_mut`

Apply the function to a mutable reference to each T in the vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_mut](for_each_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |&<b>mut</b> T|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_for_each_mut](for_each_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |&<b>mut</b> T|) {
    <b>let</b> i = 0;
    <b>let</b> len = aptos_std::smart_vector::length(v);
    <b>while</b> (i &lt; len) {
        f(aptos_std::smart_vector::borrow_mut(v, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_enumerate_ref"></a>

## Function `enumerate_ref`

Apply the function to a reference of each T in the vector with its index.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_enumerate_ref](enumerate_ref)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |(u64, &T)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_enumerate_ref](enumerate_ref)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |u64, &T|) {
    <b>let</b> i = 0;
    <b>let</b> len = aptos_std::smart_vector::length(v);
    <b>while</b> (i &lt; len) {
        f(i, aptos_std::smart_vector::borrow(v, i));
        i = i + 1;
    };
}
</code></pre>



</details>

<a id="0x1_smart_vector_enumerate_mut"></a>

## Function `enumerate_mut`

Apply the function to a mutable reference of each T in the vector with its index.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_enumerate_mut](enumerate_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, f: |(u64, &<b>mut</b> T)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_enumerate_mut](enumerate_mut)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;, f: |u64, &<b>mut</b> T|) {
    <b>let</b> i = 0;
    <b>let</b> len = [smart_vector.md#0x1_smart_vector_length](length)(v);
    <b>while</b> (i &lt; len) {
        f(i, [smart_vector.md#0x1_smart_vector_borrow_mut](borrow_mut)(v, i));
        i = i + 1;
    };
}
</code></pre>



</details>

<a id="0x1_smart_vector_fold"></a>

## Function `fold`

Fold the function over the Ts. For example, <code>[smart_vector.md#0x1_smart_vector_fold](fold)([../../move-stdlib/doc/vector.md#0x1_vector](vector)[1,2,3], 0, f)</code> will execute
<code>f(f(f(0, 1), 2), 3)</code>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_fold](fold)&lt;Accumulator, T: store&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, init: Accumulator, f: |(Accumulator, T)|Accumulator): Accumulator
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_fold](fold)&lt;Accumulator, T: store&gt;(
    v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;,
    init: Accumulator,
    f: |Accumulator, T|Accumulator
): Accumulator {
    <b>let</b> accu = init;
    aptos_std::smart_vector::for_each(v, |elem| accu = f(accu, elem));
    accu
}
</code></pre>



</details>

<a id="0x1_smart_vector_foldr"></a>

## Function `foldr`

Fold right like fold above but working right to left. For example, <code>[smart_vector.md#0x1_smart_vector_fold](fold)([../../move-stdlib/doc/vector.md#0x1_vector](vector)[1,2,3], 0, f)</code> will execute
<code>f(1, f(2, f(3, 0)))</code>


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_foldr](foldr)&lt;Accumulator, T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, init: Accumulator, f: |(T, Accumulator)|Accumulator): Accumulator
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_foldr](foldr)&lt;Accumulator, T&gt;(
    v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;,
    init: Accumulator,
    f: |T, Accumulator|Accumulator
): Accumulator {
    <b>let</b> accu = init;
    aptos_std::smart_vector::for_each_reverse(v, |elem| accu = f(elem, accu));
    accu
}
</code></pre>



</details>

<a id="0x1_smart_vector_map_ref"></a>

## Function `map_ref`

Map the function over the references of the Ts of the vector, producing a new vector without modifying the
original vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_map_ref](map_ref)&lt;T1, T2: store&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, f: |&T1|T2): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_map_ref](map_ref)&lt;T1, T2: store&gt;(
    v: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    f: |&T1|T2
): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt; {
    <b>let</b> result = aptos_std::smart_vector::new&lt;T2&gt;();
    aptos_std::smart_vector::for_each_ref(v, |elem| aptos_std::smart_vector::push_back(&<b>mut</b> result, f(elem)));
    result
}
</code></pre>



</details>

<a id="0x1_smart_vector_map"></a>

## Function `map`

Map the function over the Ts of the vector, producing a new vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_map](map)&lt;T1: store, T2: store&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, f: |T1|T2): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_map](map)&lt;T1: store, T2: store&gt;(
    v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    f: |T1|T2
): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt; {
    <b>let</b> result = aptos_std::smart_vector::new&lt;T2&gt;();
    aptos_std::smart_vector::for_each(v, |elem| [smart_vector.md#0x1_smart_vector_push_back](push_back)(&<b>mut</b> result, f(elem)));
    result
}
</code></pre>



</details>

<a id="0x1_smart_vector_filter"></a>

## Function `filter`

Filter the vector using the boolean function, removing all Ts for which <code>p(e)</code> is not true.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_filter](filter)&lt;T: drop, store&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, p: |&T|bool): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_filter](filter)&lt;T: store + drop&gt;(
    v: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt;,
    p: |&T|bool
): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; {
    <b>let</b> result = aptos_std::smart_vector::new&lt;T&gt;();
    aptos_std::smart_vector::for_each(v, |elem| {
        <b>if</b> (p(&elem)) aptos_std::smart_vector::push_back(&<b>mut</b> result, elem);
    });
    result
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip"></a>

## Function `zip`



<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip](zip)&lt;T1: store, T2: store&gt;(v1: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(T1, T2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip](zip)&lt;T1: store, T2: store&gt;(v1: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;, v2: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;, f: |T1, T2|) {
    // We need <b>to</b> reverse the vectors <b>to</b> consume it efficiently
    aptos_std::smart_vector::reverse(&<b>mut</b> v1);
    aptos_std::smart_vector::reverse(&<b>mut</b> v2);
    aptos_std::smart_vector::zip_reverse(v1, v2, |e1, e2| f(e1, e2));
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip_reverse"></a>

## Function `zip_reverse`

Apply the function to each pair of elements in the two given vectors in the reverse order, consuming them.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_reverse](zip_reverse)&lt;T1, T2&gt;(v1: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(T1, T2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_reverse](zip_reverse)&lt;T1, T2&gt;(
    v1: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    v2: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;,
    f: |T1, T2|,
) {
    <b>let</b> len = aptos_std::smart_vector::length(&v1);
    // We can't <b>use</b> the constant [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == aptos_std::smart_vector::length(&v2), 0x20005);
    <b>while</b> (len &gt; 0) {
        f(aptos_std::smart_vector::pop_back(&<b>mut</b> v1), aptos_std::smart_vector::pop_back(&<b>mut</b> v2));
        len = len - 1;
    };
    aptos_std::smart_vector::destroy_empty(v1);
    aptos_std::smart_vector::destroy_empty(v2);
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip_ref"></a>

## Function `zip_ref`

Apply the function to the references of each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_ref](zip_ref)&lt;T1, T2&gt;(v1: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(&T1, &T2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_ref](zip_ref)&lt;T1, T2&gt;(
    v1: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    v2: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;,
    f: |&T1, &T2|,
) {
    <b>let</b> len = aptos_std::smart_vector::length(v1);
    // We can't <b>use</b> the constant [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == aptos_std::smart_vector::length(v2), 0x20005);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        f(aptos_std::smart_vector::borrow(v1, i), aptos_std::smart_vector::borrow(v2, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip_mut"></a>

## Function `zip_mut`

Apply the function to mutable references to each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_mut](zip_mut)&lt;T1, T2&gt;(v1: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(&<b>mut</b> T1, &<b>mut</b> T2)|)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_mut](zip_mut)&lt;T1, T2&gt;(
    v1: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    v2: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;,
    f: |&<b>mut</b> T1, &<b>mut</b> T2|,
) {
    <b>let</b> i = 0;
    <b>let</b> len = aptos_std::smart_vector::length(v1);
    // We can't <b>use</b> the constant [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(len == aptos_std::smart_vector::length(v2), 0x20005);
    <b>while</b> (i &lt; len) {
        f(aptos_std::smart_vector::borrow_mut(v1, i), aptos_std::smart_vector::borrow_mut(v2, i));
        i = i + 1
    }
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip_map"></a>

## Function `zip_map`

Map the function over the element pairs of the two vectors, producing a new vector.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_map](zip_map)&lt;T1: store, T2: store, NewT: store&gt;(v1: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(T1, T2)|NewT): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;NewT&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_map](zip_map)&lt;T1: store, T2: store, NewT: store&gt;(
    v1: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    v2: [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;,
    f: |T1, T2|NewT
): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;NewT&gt; {
    // We can't <b>use</b> the constant [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(aptos_std::smart_vector::length(&v1) == aptos_std::smart_vector::length(&v2), 0x20005);

    <b>let</b> result = aptos_std::smart_vector::new&lt;NewT&gt;();
    aptos_std::smart_vector::zip(v1, v2, |e1, e2| [smart_vector.md#0x1_smart_vector_push_back](push_back)(&<b>mut</b> result, f(e1, e2)));
    result
}
</code></pre>



</details>

<a id="0x1_smart_vector_zip_map_ref"></a>

## Function `zip_map_ref`

Map the function over the references of the element pairs of two vectors, producing a new vector from the return
values without modifying the original vectors.


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_map_ref](zip_map_ref)&lt;T1, T2, NewT: store&gt;(v1: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T1&gt;, v2: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T2&gt;, f: |(&T1, &T2)|NewT): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;NewT&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> inline <b>fun</b> [smart_vector.md#0x1_smart_vector_zip_map_ref](zip_map_ref)&lt;T1, T2, NewT: store&gt;(
    v1: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T1&gt;,
    v2: &[smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T2&gt;,
    f: |&T1, &T2|NewT
): [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;NewT&gt; {
    // We can't <b>use</b> the constant [smart_vector.md#0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH](ESMART_VECTORS_LENGTH_MISMATCH) here <b>as</b> all calling code would then need <b>to</b> define it
    // due <b>to</b> how inline functions work.
    <b>assert</b>!(aptos_std::smart_vector::length(v1) == aptos_std::smart_vector::length(v2), 0x20005);

    <b>let</b> result = aptos_std::smart_vector::new&lt;NewT&gt;();
    aptos_std::smart_vector::zip_ref(v1, v2, |e1, e2| [smart_vector.md#0x1_smart_vector_push_back](push_back)(&<b>mut</b> result, f(e1, e2)));
    result
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_SmartVector"></a>

### Struct `SmartVector`


<pre><code><b>struct</b> [smart_vector.md#0x1_smart_vector_SmartVector](SmartVector)&lt;T&gt; <b>has</b> store
</code></pre>



<dl>
<dt>
<code>inline_vec: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;T&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>big_vec: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[big_vector.md#0x1_big_vector_BigVector](big_vector::BigVector)&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>inline_capacity: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bucket_size: [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;u64&gt;</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>invariant</b> [../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(bucket_size)
    || ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(bucket_size) && [../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(bucket_size) != 0);
<b>invariant</b> [../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(inline_capacity)
    || (len(inline_vec) &lt;= [../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(inline_capacity));
<b>invariant</b> ([../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(inline_capacity) && [../../move-stdlib/doc/option.md#0x1_option_is_none](option::is_none)(bucket_size))
    || ([../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(inline_capacity) && [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(bucket_size));
</code></pre>



<a id="@Specification_1_empty"></a>

### Function `empty`


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty](empty)&lt;T: store&gt;(): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_empty_with_config"></a>

### Function `empty_with_config`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_empty_with_config](empty_with_config)&lt;T: store&gt;(inline_capacity: u64, bucket_size: u64): [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;
</code></pre>




<pre><code><b>aborts_if</b> bucket_size == 0;
</code></pre>



<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_destroy_empty](destroy_empty)&lt;T&gt;(v: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>




<pre><code><b>aborts_if</b> !([smart_vector.md#0x1_smart_vector_is_empty](is_empty)(v));
<b>aborts_if</b> len(v.inline_vec) != 0
    || [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec);
</code></pre>



<a id="@Specification_1_borrow"></a>

### Function `borrow`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_borrow](borrow)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): &T
</code></pre>




<pre><code><b>aborts_if</b> i &gt;= [smart_vector.md#0x1_smart_vector_length](length)(v);
<b>aborts_if</b> [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec) && (
    (len(v.inline_vec) + [big_vector.md#0x1_big_vector_length](big_vector::length)&lt;T&gt;([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(v.big_vec))) &gt; MAX_U64
);
</code></pre>



<a id="@Specification_1_append"></a>

### Function `append`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_append](append)&lt;T: store&gt;(lhs: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, other: [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_push_back"></a>

### Function `push_back`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_push_back](push_back)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, val: T)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_pop_back"></a>

### Function `pop_back`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_pop_back](pop_back)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): T
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>aborts_if</b>  [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec)
    &&
    ([table_with_length.md#0x1_table_with_length_spec_len](table_with_length::spec_len)([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(v.big_vec).buckets) == 0);
<b>aborts_if</b> [smart_vector.md#0x1_smart_vector_is_empty](is_empty)(v);
<b>aborts_if</b> [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec) && (
    (len(v.inline_vec) + [big_vector.md#0x1_big_vector_length](big_vector::length)&lt;T&gt;([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(v.big_vec))) &gt; MAX_U64
);
<b>ensures</b> [smart_vector.md#0x1_smart_vector_length](length)(v) == [smart_vector.md#0x1_smart_vector_length](length)(<b>old</b>(v)) - 1;
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_remove](remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): T
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_swap_remove"></a>

### Function `swap_remove`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap_remove](swap_remove)&lt;T&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64): T
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
<b>aborts_if</b> i &gt;= [smart_vector.md#0x1_smart_vector_length](length)(v);
<b>aborts_if</b> [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec) && (
    (len(v.inline_vec) + [big_vector.md#0x1_big_vector_length](big_vector::length)&lt;T&gt;([../../move-stdlib/doc/option.md#0x1_option_borrow](option::borrow)(v.big_vec))) &gt; MAX_U64
);
<b>ensures</b> [smart_vector.md#0x1_smart_vector_length](length)(v) == [smart_vector.md#0x1_smart_vector_length](length)(<b>old</b>(v)) - 1;
</code></pre>



<a id="@Specification_1_swap"></a>

### Function `swap`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_swap](swap)&lt;T: store&gt;(v: &<b>mut</b> [smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;, i: u64, j: u64)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_length"></a>

### Function `length`


<pre><code><b>public</b> <b>fun</b> [smart_vector.md#0x1_smart_vector_length](length)&lt;T&gt;(v: &[smart_vector.md#0x1_smart_vector_SmartVector](smart_vector::SmartVector)&lt;T&gt;): u64
</code></pre>




<pre><code><b>aborts_if</b> [../../move-stdlib/doc/option.md#0x1_option_is_some](option::is_some)(v.big_vec) && len(v.inline_vec) + [big_vector.md#0x1_big_vector_length](big_vector::length)([../../move-stdlib/doc/option.md#0x1_option_spec_borrow](option::spec_borrow)(v.big_vec)) &gt; MAX_U64;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
