
<a id="0x1_copyable_any"></a>

# Module `0x1::copyable_any`



-  [Struct `Any`](#0x1_copyable_any_Any)
-  [Constants](#@Constants_0)
-  [Function `pack`](#0x1_copyable_any_pack)
-  [Function `unpack`](#0x1_copyable_any_unpack)
-  [Function `type_name`](#0x1_copyable_any_type_name)
-  [Specification](#@Specification_1)
    -  [Function `pack`](#@Specification_1_pack)
    -  [Function `unpack`](#@Specification_1_unpack)
    -  [Function `type_name`](#@Specification_1_type_name)


<pre><code><b>use</b> [../../move-stdlib/doc/bcs.md#0x1_bcs](0x1::bcs);
<b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [from_bcs.md#0x1_from_bcs](0x1::from_bcs);
<b>use</b> [../../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [type_info.md#0x1_type_info](0x1::type_info);
</code></pre>



<a id="0x1_copyable_any_Any"></a>

## Struct `Any`

The same as <code>[any.md#0x1_any_Any](any::Any)</code> but with the copy ability.


<pre><code><b>struct</b> [copyable_any.md#0x1_copyable_any_Any](Any) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type_name: [../../move-stdlib/doc/string.md#0x1_string_String](string::String)</code>
</dt>
<dd>

</dd>
<dt>
<code>data: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_copyable_any_ETYPE_MISMATCH"></a>

The type provided for <code>unpack</code> is not the same as was given for <code>pack</code>.


<pre><code><b>const</b> [copyable_any.md#0x1_copyable_any_ETYPE_MISMATCH](ETYPE_MISMATCH): u64 = 0;
</code></pre>



<a id="0x1_copyable_any_pack"></a>

## Function `pack`

Pack a value into the <code>[copyable_any.md#0x1_copyable_any_Any](Any)</code> representation. Because Any can be stored, dropped, and copied this is
also required from <code>T</code>.


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_pack](pack)&lt;T: <b>copy</b>, drop, store&gt;(x: T): [copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_pack](pack)&lt;T: drop + store + <b>copy</b>&gt;(x: T): [copyable_any.md#0x1_copyable_any_Any](Any) {
    [copyable_any.md#0x1_copyable_any_Any](Any) {
        type_name: [type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;T&gt;(),
        data: [../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes](bcs::to_bytes)(&x)
    }
}
</code></pre>



</details>

<a id="0x1_copyable_any_unpack"></a>

## Function `unpack`

Unpack a value from the <code>[copyable_any.md#0x1_copyable_any_Any](Any)</code> representation. This aborts if the value has not the expected type <code>T</code>.


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_unpack](unpack)&lt;T&gt;(x: [copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_unpack](unpack)&lt;T&gt;(x: [copyable_any.md#0x1_copyable_any_Any](Any)): T {
    <b>assert</b>!([type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;T&gt;() == x.type_name, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([copyable_any.md#0x1_copyable_any_ETYPE_MISMATCH](ETYPE_MISMATCH)));
    from_bytes&lt;T&gt;(x.data)
}
</code></pre>



</details>

<a id="0x1_copyable_any_type_name"></a>

## Function `type_name`

Returns the type name of this Any


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_type_name](type_name)(x: &[copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)): &[../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_type_name](type_name)(x: &[copyable_any.md#0x1_copyable_any_Any](Any)): &String {
    &x.type_name
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_pack"></a>

### Function `pack`


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_pack](pack)&lt;T: <b>copy</b>, drop, store&gt;(x: T): [copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>pragma</b> opaque;
<b>ensures</b> result == [copyable_any.md#0x1_copyable_any_Any](Any) {
    type_name: [type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;T&gt;(),
    data: [../../move-stdlib/doc/bcs.md#0x1_bcs_serialize](bcs::serialize)&lt;T&gt;(x)
};
<b>ensures</b> [abstract] [from_bcs.md#0x1_from_bcs_deserializable](from_bcs::deserializable)&lt;T&gt;(result.data);
</code></pre>



<a id="@Specification_1_unpack"></a>

### Function `unpack`


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_unpack](unpack)&lt;T&gt;(x: [copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)): T
</code></pre>




<pre><code><b>include</b> [copyable_any.md#0x1_copyable_any_UnpackAbortsIf](UnpackAbortsIf)&lt;T&gt;;
<b>ensures</b> result == [from_bcs.md#0x1_from_bcs_deserialize](from_bcs::deserialize)&lt;T&gt;(x.data);
</code></pre>




<a id="0x1_copyable_any_UnpackAbortsIf"></a>


<pre><code><b>schema</b> [copyable_any.md#0x1_copyable_any_UnpackAbortsIf](UnpackAbortsIf)&lt;T&gt; {
    x: [copyable_any.md#0x1_copyable_any_Any](Any);
    <b>aborts_if</b> [type_info.md#0x1_type_info_type_name](type_info::type_name)&lt;T&gt;() != x.type_name;
    <b>aborts_if</b> ![from_bcs.md#0x1_from_bcs_deserializable](from_bcs::deserializable)&lt;T&gt;(x.data);
}
</code></pre>



<a id="@Specification_1_type_name"></a>

### Function `type_name`


<pre><code><b>public</b> <b>fun</b> [copyable_any.md#0x1_copyable_any_type_name](type_name)(x: &[copyable_any.md#0x1_copyable_any_Any](copyable_any::Any)): &[../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == x.type_name;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
