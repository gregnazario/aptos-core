
<a id="0x1_from_bcs"></a>

# Module `0x1::from_bcs`

This module provides a number of functions to convert _primitive_ types from their representation in <code>std::bcs</code>
to values. This is the opposite of <code>[../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)</code>. Note that it is not safe to define a generic public <code>from_bytes</code>
function because this can violate implicit struct invariants, therefore only primitive types are offerred. If
a general conversion back-and-force is needed, consider the <code>aptos_std::Any</code> type which preserves invariants.

Example:
```
use std::bcs;
use aptos_std::from_bcs;

assert!(from_bcs::to_address(bcs::to_bytes(&@0xabcdef)) == @0xabcdef, 0);
```


-  [Constants](#@Constants_0)
-  [Function `to_bool`](#0x1_from_bcs_to_bool)
-  [Function `to_u8`](#0x1_from_bcs_to_u8)
-  [Function `to_u16`](#0x1_from_bcs_to_u16)
-  [Function `to_u32`](#0x1_from_bcs_to_u32)
-  [Function `to_u64`](#0x1_from_bcs_to_u64)
-  [Function `to_u128`](#0x1_from_bcs_to_u128)
-  [Function `to_u256`](#0x1_from_bcs_to_u256)
-  [Function `to_address`](#0x1_from_bcs_to_address)
-  [Function `to_bytes`](#0x1_from_bcs_to_bytes)
-  [Function `to_string`](#0x1_from_bcs_to_string)
-  [Function `from_bytes`](#0x1_from_bcs_from_bytes)
-  [Specification](#@Specification_1)
    -  [Function `from_bytes`](#@Specification_1_from_bytes)


<pre><code><b>use</b> [../../move-stdlib/doc/string.md#0x1_string](0x1::string);
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_from_bcs_EINVALID_UTF8"></a>

UTF8 check failed in conversion from bytes to string


<pre><code><b>const</b> [from_bcs.md#0x1_from_bcs_EINVALID_UTF8](EINVALID_UTF8): u64 = 1;
</code></pre>



<a id="0x1_from_bcs_to_bool"></a>

## Function `to_bool`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_bool](to_bool)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_bool](to_bool)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): bool {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;bool&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u8"></a>

## Function `to_u8`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u8](to_u8)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u8](to_u8)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u8 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u8&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u16"></a>

## Function `to_u16`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u16](to_u16)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u16](to_u16)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u16 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u16&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u32"></a>

## Function `to_u32`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u32](to_u32)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u32](to_u32)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u32 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u32&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u64"></a>

## Function `to_u64`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u64](to_u64)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u64](to_u64)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u64 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u64&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u128"></a>

## Function `to_u128`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u128](to_u128)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u128](to_u128)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u128 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u128&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_u256"></a>

## Function `to_u256`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u256](to_u256)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_u256](to_u256)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): u256 {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;u256&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_address"></a>

## Function `to_address`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_address](to_address)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_address](to_address)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): <b>address</b> {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;<b>address</b>&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_bytes"></a>

## Function `to_bytes`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_bytes](to_bytes)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_bytes](to_bytes)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;[../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;&gt;(v)
}
</code></pre>



</details>

<a id="0x1_from_bcs_to_string"></a>

## Function `to_string`



<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_string](to_string)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_to_string](to_string)(v: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): String {
    // To make this safe, we need <b>to</b> evaluate the utf8 <b>invariant</b>.
    <b>let</b> s = [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;String&gt;(v);
    <b>assert</b>!([../../move-stdlib/doc/string.md#0x1_string_internal_check_utf8](string::internal_check_utf8)([../../move-stdlib/doc/string.md#0x1_string_bytes](string::bytes)(&s)), [from_bcs.md#0x1_from_bcs_EINVALID_UTF8](EINVALID_UTF8));
    s
}
</code></pre>



</details>

<a id="0x1_from_bcs_from_bytes"></a>

## Function `from_bytes`

Package private native function to deserialize a type T.

Note that this function does not put any constraint on <code>T</code>. If code uses this function to
deserialize a linear value, its their responsibility that the data they deserialize is
owned.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;T&gt;(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>native</b> <b>fun</b> [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;T&gt;(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): T;
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<a id="0x1_from_bcs_deserialize"></a>


<pre><code><b>fun</b> [from_bcs.md#0x1_from_bcs_deserialize](deserialize)&lt;T&gt;(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): T;
<a id="0x1_from_bcs_deserializable"></a>
<b>fun</b> [from_bcs.md#0x1_from_bcs_deserializable](deserializable)&lt;T&gt;(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): bool;
<b>axiom</b>&lt;T&gt; <b>forall</b> b1: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, b2: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;:
    ( b1 == b2 ==&gt; [from_bcs.md#0x1_from_bcs_deserializable](deserializable)&lt;T&gt;(b1) == [from_bcs.md#0x1_from_bcs_deserializable](deserializable)&lt;T&gt;(b2) );
<b>axiom</b>&lt;T&gt; <b>forall</b> b1: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, b2: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;:
    ( b1 == b2 ==&gt; [from_bcs.md#0x1_from_bcs_deserialize](deserialize)&lt;T&gt;(b1) == [from_bcs.md#0x1_from_bcs_deserialize](deserialize)&lt;T&gt;(b2) );
</code></pre>



<a id="@Specification_1_from_bytes"></a>

### Function `from_bytes`


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [from_bcs.md#0x1_from_bcs_from_bytes](from_bytes)&lt;T&gt;(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): T
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> ![from_bcs.md#0x1_from_bcs_deserializable](deserializable)&lt;T&gt;(bytes);
<b>ensures</b> result == [from_bcs.md#0x1_from_bcs_deserialize](deserialize)&lt;T&gt;(bytes);
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
