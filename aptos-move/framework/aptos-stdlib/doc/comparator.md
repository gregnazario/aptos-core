
<a id="0x1_comparator"></a>

# Module `0x1::comparator`

Provides a framework for comparing two elements


-  [Struct `Result`](#0x1_comparator_Result)
-  [Constants](#@Constants_0)
-  [Function `is_equal`](#0x1_comparator_is_equal)
-  [Function `is_smaller_than`](#0x1_comparator_is_smaller_than)
-  [Function `is_greater_than`](#0x1_comparator_is_greater_than)
-  [Function `compare`](#0x1_comparator_compare)
-  [Function `compare_u8_vector`](#0x1_comparator_compare_u8_vector)
-  [Specification](#@Specification_1)
    -  [Struct `Result`](#@Specification_1_Result)
    -  [Function `is_equal`](#@Specification_1_is_equal)
    -  [Function `is_smaller_than`](#@Specification_1_is_smaller_than)
    -  [Function `is_greater_than`](#@Specification_1_is_greater_than)
    -  [Function `compare`](#@Specification_1_compare)
    -  [Function `compare_u8_vector`](#@Specification_1_compare_u8_vector)


<pre><code><b>use</b> [../../move-stdlib/doc/bcs.md#0x1_bcs](0x1::bcs);
</code></pre>



<a id="0x1_comparator_Result"></a>

## Struct `Result`



<pre><code><b>struct</b> [comparator.md#0x1_comparator_Result](Result) <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_comparator_EQUAL"></a>



<pre><code><b>const</b> [comparator.md#0x1_comparator_EQUAL](EQUAL): u8 = 0;
</code></pre>



<a id="0x1_comparator_GREATER"></a>



<pre><code><b>const</b> [comparator.md#0x1_comparator_GREATER](GREATER): u8 = 2;
</code></pre>



<a id="0x1_comparator_SMALLER"></a>



<pre><code><b>const</b> [comparator.md#0x1_comparator_SMALLER](SMALLER): u8 = 1;
</code></pre>



<a id="0x1_comparator_is_equal"></a>

## Function `is_equal`



<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_equal](is_equal)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_equal](is_equal)(result: &[comparator.md#0x1_comparator_Result](Result)): bool {
    result.inner == [comparator.md#0x1_comparator_EQUAL](EQUAL)
}
</code></pre>



</details>

<a id="0x1_comparator_is_smaller_than"></a>

## Function `is_smaller_than`



<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_smaller_than](is_smaller_than)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_smaller_than](is_smaller_than)(result: &[comparator.md#0x1_comparator_Result](Result)): bool {
    result.inner == [comparator.md#0x1_comparator_SMALLER](SMALLER)
}
</code></pre>



</details>

<a id="0x1_comparator_is_greater_than"></a>

## Function `is_greater_than`



<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_greater_than](is_greater_than)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_greater_than](is_greater_than)(result: &[comparator.md#0x1_comparator_Result](Result)): bool {
    result.inner == [comparator.md#0x1_comparator_GREATER](GREATER)
}
</code></pre>



</details>

<a id="0x1_comparator_compare"></a>

## Function `compare`



<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare](compare)&lt;T&gt;(left: &T, right: &T): [comparator.md#0x1_comparator_Result](comparator::Result)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare](compare)&lt;T&gt;(left: &T, right: &T): [comparator.md#0x1_comparator_Result](Result) {
    <b>let</b> left_bytes = [../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(left);
    <b>let</b> right_bytes = [../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(right);

    [comparator.md#0x1_comparator_compare_u8_vector](compare_u8_vector)(left_bytes, right_bytes)
}
</code></pre>



</details>

<a id="0x1_comparator_compare_u8_vector"></a>

## Function `compare_u8_vector`



<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare_u8_vector](compare_u8_vector)(left: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, right: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [comparator.md#0x1_comparator_Result](comparator::Result)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare_u8_vector](compare_u8_vector)(left: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, right: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [comparator.md#0x1_comparator_Result](Result) {
    <b>let</b> left_length = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&left);
    <b>let</b> right_length = [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&right);

    <b>let</b> idx = 0;

    <b>while</b> (idx &lt; left_length && idx &lt; right_length) {
        <b>let</b> left_byte = *[../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&left, idx);
        <b>let</b> right_byte = *[../../move-stdlib/doc/vector.md#0x1_vector_borrow](vector::borrow)(&right, idx);

        <b>if</b> (left_byte &lt; right_byte) {
            <b>return</b> [comparator.md#0x1_comparator_Result](Result) { inner: [comparator.md#0x1_comparator_SMALLER](SMALLER) }
        } <b>else</b> <b>if</b> (left_byte &gt; right_byte) {
            <b>return</b> [comparator.md#0x1_comparator_Result](Result) { inner: [comparator.md#0x1_comparator_GREATER](GREATER) }
        };
        idx = idx + 1;
    };

    <b>if</b> (left_length &lt; right_length) {
        [comparator.md#0x1_comparator_Result](Result) { inner: [comparator.md#0x1_comparator_SMALLER](SMALLER) }
    } <b>else</b> <b>if</b> (left_length &gt; right_length) {
        [comparator.md#0x1_comparator_Result](Result) { inner: [comparator.md#0x1_comparator_GREATER](GREATER) }
    } <b>else</b> {
        [comparator.md#0x1_comparator_Result](Result) { inner: [comparator.md#0x1_comparator_EQUAL](EQUAL) }
    }
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_Result"></a>

### Struct `Result`


<pre><code><b>struct</b> [comparator.md#0x1_comparator_Result](Result) <b>has</b> drop
</code></pre>



<dl>
<dt>
<code>inner: u8</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>invariant</b> inner == [comparator.md#0x1_comparator_EQUAL](EQUAL) || inner == [comparator.md#0x1_comparator_SMALLER](SMALLER) || inner == [comparator.md#0x1_comparator_GREATER](GREATER);
</code></pre>



<a id="@Specification_1_is_equal"></a>

### Function `is_equal`


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_equal](is_equal)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>let</b> res = result;
<b>ensures</b> result == (res.inner == [comparator.md#0x1_comparator_EQUAL](EQUAL));
</code></pre>



<a id="@Specification_1_is_smaller_than"></a>

### Function `is_smaller_than`


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_smaller_than](is_smaller_than)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>let</b> res = result;
<b>ensures</b> result == (res.inner == [comparator.md#0x1_comparator_SMALLER](SMALLER));
</code></pre>



<a id="@Specification_1_is_greater_than"></a>

### Function `is_greater_than`


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_is_greater_than](is_greater_than)(result: &[comparator.md#0x1_comparator_Result](comparator::Result)): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>let</b> res = result;
<b>ensures</b> result == (res.inner == [comparator.md#0x1_comparator_GREATER](GREATER));
</code></pre>



<a id="@Specification_1_compare"></a>

### Function `compare`


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare](compare)&lt;T&gt;(left: &T, right: &T): [comparator.md#0x1_comparator_Result](comparator::Result)
</code></pre>




<pre><code><b>let</b> left_bytes = [../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(left);
<b>let</b> right_bytes = [../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(right);
<b>ensures</b> result == [comparator.md#0x1_comparator_spec_compare_u8_vector](spec_compare_u8_vector)(left_bytes, right_bytes);
</code></pre>




<a id="0x1_comparator_spec_compare_u8_vector"></a>


<pre><code><b>fun</b> [comparator.md#0x1_comparator_spec_compare_u8_vector](spec_compare_u8_vector)(left: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, right: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [comparator.md#0x1_comparator_Result](Result);
</code></pre>



<a id="@Specification_1_compare_u8_vector"></a>

### Function `compare_u8_vector`


<pre><code><b>public</b> <b>fun</b> [comparator.md#0x1_comparator_compare_u8_vector](compare_u8_vector)(left: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, right: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [comparator.md#0x1_comparator_Result](comparator::Result)
</code></pre>




<pre><code><b>pragma</b> unroll = 5;
<b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>let</b> left_length = len(left);
<b>let</b> right_length = len(right);
<b>ensures</b> (result.inner == [comparator.md#0x1_comparator_EQUAL](EQUAL)) ==&gt; (
    (left_length == right_length) &&
        (<b>forall</b> i: u64 <b>where</b> i &lt; left_length: left[i] == right[i])
);
<b>ensures</b> (result.inner == [comparator.md#0x1_comparator_SMALLER](SMALLER)) ==&gt; (
    (<b>exists</b> i: u64 <b>where</b> i &lt; left_length:
        (i &lt; right_length) &&
            (left[i] &lt; right[i]) &&
            (<b>forall</b> j: u64 <b>where</b> j &lt; i: left[j] == right[j])
    ) ||
        (left_length &lt; right_length)
);
<b>ensures</b> (result.inner == [comparator.md#0x1_comparator_GREATER](GREATER)) ==&gt; (
    (<b>exists</b> i: u64 <b>where</b> i &lt; left_length:
        (i &lt; right_length) &&
            (left[i] &gt; right[i]) &&
            (<b>forall</b> j: u64 <b>where</b> j &lt; i: left[j] == right[j])
    ) ||
        (left_length &gt; right_length)
);
<b>ensures</b> [abstract] result == [comparator.md#0x1_comparator_spec_compare_u8_vector](spec_compare_u8_vector)(left, right);
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
