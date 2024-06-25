
<a id="0x1_string"></a>

# Module `0x1::string`

The <code>[string.md#0x1_string](string)</code> module defines the <code>[string.md#0x1_string_String](String)</code> type which represents UTF8 encoded strings.


-  [Struct `String`](#0x1_string_String)
-  [Constants](#@Constants_0)
-  [Function `utf8`](#0x1_string_utf8)
-  [Function `try_utf8`](#0x1_string_try_utf8)
-  [Function `bytes`](#0x1_string_bytes)
-  [Function `is_empty`](#0x1_string_is_empty)
-  [Function `length`](#0x1_string_length)
-  [Function `append`](#0x1_string_append)
-  [Function `append_utf8`](#0x1_string_append_utf8)
-  [Function `insert`](#0x1_string_insert)
-  [Function `sub_string`](#0x1_string_sub_string)
-  [Function `index_of`](#0x1_string_index_of)
-  [Function `internal_check_utf8`](#0x1_string_internal_check_utf8)
-  [Function `internal_is_char_boundary`](#0x1_string_internal_is_char_boundary)
-  [Function `internal_sub_string`](#0x1_string_internal_sub_string)
-  [Function `internal_index_of`](#0x1_string_internal_index_of)
-  [Specification](#@Specification_1)
    -  [Function `internal_check_utf8`](#@Specification_1_internal_check_utf8)
    -  [Function `internal_is_char_boundary`](#@Specification_1_internal_is_char_boundary)
    -  [Function `internal_sub_string`](#@Specification_1_internal_sub_string)
    -  [Function `internal_index_of`](#@Specification_1_internal_index_of)


<pre><code><b>use</b> [option.md#0x1_option](0x1::option);
<b>use</b> [vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_string_String"></a>

## Struct `String`

A <code>[string.md#0x1_string_String](String)</code> holds a sequence of bytes which is guaranteed to be in utf8 format.


<pre><code><b>struct</b> [string.md#0x1_string_String](String) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_string_EINVALID_INDEX"></a>

Index out of range.


<pre><code><b>const</b> [string.md#0x1_string_EINVALID_INDEX](EINVALID_INDEX): u64 = 2;
</code></pre>



<a id="0x1_string_EINVALID_UTF8"></a>

An invalid UTF8 encoding.


<pre><code><b>const</b> [string.md#0x1_string_EINVALID_UTF8](EINVALID_UTF8): u64 = 1;
</code></pre>



<a id="0x1_string_utf8"></a>

## Function `utf8`

Creates a new string from a sequence of bytes. Aborts if the bytes do not represent valid utf8.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_utf8](utf8)(bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;): [string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_utf8](utf8)(bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;): [string.md#0x1_string_String](String) {
    <b>assert</b>!([string.md#0x1_string_internal_check_utf8](internal_check_utf8)(&bytes), [string.md#0x1_string_EINVALID_UTF8](EINVALID_UTF8));
    [string.md#0x1_string_String](String){bytes}
}
</code></pre>



</details>

<a id="0x1_string_try_utf8"></a>

## Function `try_utf8`

Tries to create a new string from a sequence of bytes.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_try_utf8](try_utf8)(bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;): [option.md#0x1_option_Option](option::Option)&lt;[string.md#0x1_string_String](string::String)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_try_utf8](try_utf8)(bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;): Option&lt;[string.md#0x1_string_String](String)&gt; {
    <b>if</b> ([string.md#0x1_string_internal_check_utf8](internal_check_utf8)(&bytes)) {
        [option.md#0x1_option_some](option::some)([string.md#0x1_string_String](String){bytes})
    } <b>else</b> {
        [option.md#0x1_option_none](option::none)()
    }
}
</code></pre>



</details>

<a id="0x1_string_bytes"></a>

## Function `bytes`

Returns a reference to the underlying byte vector.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_bytes](bytes)(s: &[string.md#0x1_string_String](string::String)): &[vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_bytes](bytes)(s: &[string.md#0x1_string_String](String)): &[vector.md#0x1_vector](vector)&lt;u8&gt; {
    &s.bytes
}
</code></pre>



</details>

<a id="0x1_string_is_empty"></a>

## Function `is_empty`

Checks whether this string is empty.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_is_empty](is_empty)(s: &[string.md#0x1_string_String](string::String)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_is_empty](is_empty)(s: &[string.md#0x1_string_String](String)): bool {
    [vector.md#0x1_vector_is_empty](vector::is_empty)(&s.bytes)
}
</code></pre>



</details>

<a id="0x1_string_length"></a>

## Function `length`

Returns the length of this string, in bytes.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_length](length)(s: &[string.md#0x1_string_String](string::String)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_length](length)(s: &[string.md#0x1_string_String](String)): u64 {
    [vector.md#0x1_vector_length](vector::length)(&s.bytes)
}
</code></pre>



</details>

<a id="0x1_string_append"></a>

## Function `append`

Appends a string.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_append](append)(s: &<b>mut</b> [string.md#0x1_string_String](string::String), r: [string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_append](append)(s: &<b>mut</b> [string.md#0x1_string_String](String), r: [string.md#0x1_string_String](String)) {
    [vector.md#0x1_vector_append](vector::append)(&<b>mut</b> s.bytes, r.bytes)
}
</code></pre>



</details>

<a id="0x1_string_append_utf8"></a>

## Function `append_utf8`

Appends bytes which must be in valid utf8 format.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_append_utf8](append_utf8)(s: &<b>mut</b> [string.md#0x1_string_String](string::String), bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_append_utf8](append_utf8)(s: &<b>mut</b> [string.md#0x1_string_String](String), bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;) {
    [string.md#0x1_string_append](append)(s, [string.md#0x1_string_utf8](utf8)(bytes))
}
</code></pre>



</details>

<a id="0x1_string_insert"></a>

## Function `insert`

Insert the other string at the byte index in given string. The index must be at a valid utf8 char
boundary.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_insert](insert)(s: &<b>mut</b> [string.md#0x1_string_String](string::String), at: u64, o: [string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_insert](insert)(s: &<b>mut</b> [string.md#0x1_string_String](String), at: u64, o: [string.md#0x1_string_String](String)) {
    <b>let</b> bytes = &s.bytes;
    <b>assert</b>!(at &lt;= [vector.md#0x1_vector_length](vector::length)(bytes) && [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(bytes, at), [string.md#0x1_string_EINVALID_INDEX](EINVALID_INDEX));
    <b>let</b> l = [string.md#0x1_string_length](length)(s);
    <b>let</b> front = [string.md#0x1_string_sub_string](sub_string)(s, 0, at);
    <b>let</b> end = [string.md#0x1_string_sub_string](sub_string)(s, at, l);
    [string.md#0x1_string_append](append)(&<b>mut</b> front, o);
    [string.md#0x1_string_append](append)(&<b>mut</b> front, end);
    *s = front;
}
</code></pre>



</details>

<a id="0x1_string_sub_string"></a>

## Function `sub_string`

Returns a sub-string using the given byte indices, where <code>i</code> is the first byte position and <code>j</code> is the start
of the first byte not included (or the length of the string). The indices must be at valid utf8 char boundaries,
guaranteeing that the result is valid utf8.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_sub_string](sub_string)(s: &[string.md#0x1_string_String](string::String), i: u64, j: u64): [string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_sub_string](sub_string)(s: &[string.md#0x1_string_String](String), i: u64, j: u64): [string.md#0x1_string_String](String) {
    <b>let</b> bytes = &s.bytes;
    <b>let</b> l = [vector.md#0x1_vector_length](vector::length)(bytes);
    <b>assert</b>!(
        j &lt;= l && i &lt;= j && [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(bytes, i) && [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(bytes, j),
        [string.md#0x1_string_EINVALID_INDEX](EINVALID_INDEX)
    );
    [string.md#0x1_string_String](String) { bytes: [string.md#0x1_string_internal_sub_string](internal_sub_string)(bytes, i, j) }
}
</code></pre>



</details>

<a id="0x1_string_index_of"></a>

## Function `index_of`

Computes the index of the first occurrence of a string. Returns <code>[string.md#0x1_string_length](length)(s)</code> if no occurrence found.


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_index_of](index_of)(s: &[string.md#0x1_string_String](string::String), r: &[string.md#0x1_string_String](string::String)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_index_of](index_of)(s: &[string.md#0x1_string_String](String), r: &[string.md#0x1_string_String](String)): u64 {
    [string.md#0x1_string_internal_index_of](internal_index_of)(&s.bytes, &r.bytes)
}
</code></pre>



</details>

<a id="0x1_string_internal_check_utf8"></a>

## Function `internal_check_utf8`



<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_internal_check_utf8](internal_check_utf8)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> [string.md#0x1_string_internal_check_utf8](internal_check_utf8)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;): bool;
</code></pre>



</details>

<a id="0x1_string_internal_is_char_boundary"></a>

## Function `internal_is_char_boundary`



<pre><code><b>fun</b> [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64): bool;
</code></pre>



</details>

<a id="0x1_string_internal_sub_string"></a>

## Function `internal_sub_string`



<pre><code><b>fun</b> [string.md#0x1_string_internal_sub_string](internal_sub_string)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64, j: u64): [vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [string.md#0x1_string_internal_sub_string](internal_sub_string)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64, j: u64): [vector.md#0x1_vector](vector)&lt;u8&gt;;
</code></pre>



</details>

<a id="0x1_string_internal_index_of"></a>

## Function `internal_index_of`



<pre><code><b>fun</b> [string.md#0x1_string_internal_index_of](internal_index_of)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, r: &[vector.md#0x1_vector](vector)&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [string.md#0x1_string_internal_index_of](internal_index_of)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, r: &[vector.md#0x1_vector](vector)&lt;u8&gt;): u64;
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_internal_check_utf8"></a>

### Function `internal_check_utf8`


<pre><code><b>public</b> <b>fun</b> [string.md#0x1_string_internal_check_utf8](internal_check_utf8)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;): bool
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == [string.md#0x1_string_spec_internal_check_utf8](spec_internal_check_utf8)(v);
</code></pre>



<a id="@Specification_1_internal_is_char_boundary"></a>

### Function `internal_is_char_boundary`


<pre><code><b>fun</b> [string.md#0x1_string_internal_is_char_boundary](internal_is_char_boundary)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64): bool
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == [string.md#0x1_string_spec_internal_is_char_boundary](spec_internal_is_char_boundary)(v, i);
</code></pre>



<a id="@Specification_1_internal_sub_string"></a>

### Function `internal_sub_string`


<pre><code><b>fun</b> [string.md#0x1_string_internal_sub_string](internal_sub_string)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64, j: u64): [vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == [string.md#0x1_string_spec_internal_sub_string](spec_internal_sub_string)(v, i, j);
</code></pre>



<a id="@Specification_1_internal_index_of"></a>

### Function `internal_index_of`


<pre><code><b>fun</b> [string.md#0x1_string_internal_index_of](internal_index_of)(v: &[vector.md#0x1_vector](vector)&lt;u8&gt;, r: &[vector.md#0x1_vector](vector)&lt;u8&gt;): u64
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>false</b>;
<b>ensures</b> [abstract] result == [string.md#0x1_string_spec_internal_index_of](spec_internal_index_of)(v, r);
</code></pre>




<a id="0x1_string_spec_utf8"></a>


<pre><code><b>fun</b> [string.md#0x1_string_spec_utf8](spec_utf8)(bytes: [vector.md#0x1_vector](vector)&lt;u8&gt;): [string.md#0x1_string_String](String) {
   [string.md#0x1_string_String](String){bytes}
}
</code></pre>




<a id="0x1_string_spec_internal_check_utf8"></a>


<pre><code><b>fun</b> [string.md#0x1_string_spec_internal_check_utf8](spec_internal_check_utf8)(v: [vector.md#0x1_vector](vector)&lt;u8&gt;): bool;
<a id="0x1_string_spec_internal_is_char_boundary"></a>
<b>fun</b> [string.md#0x1_string_spec_internal_is_char_boundary](spec_internal_is_char_boundary)(v: [vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64): bool;
<a id="0x1_string_spec_internal_sub_string"></a>
<b>fun</b> [string.md#0x1_string_spec_internal_sub_string](spec_internal_sub_string)(v: [vector.md#0x1_vector](vector)&lt;u8&gt;, i: u64, j: u64): [vector.md#0x1_vector](vector)&lt;u8&gt;;
<a id="0x1_string_spec_internal_index_of"></a>
<b>fun</b> [string.md#0x1_string_spec_internal_index_of](spec_internal_index_of)(v: [vector.md#0x1_vector](vector)&lt;u8&gt;, r: [vector.md#0x1_vector](vector)&lt;u8&gt;): u64;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
