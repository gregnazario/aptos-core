
<a id="0x1_debug"></a>

# Module `0x1::debug`

Module providing debug functionality.


-  [Constants](#@Constants_0)
-  [Function `print`](#0x1_debug_print)
-  [Function `print_stack_trace`](#0x1_debug_print_stack_trace)
-  [Function `format`](#0x1_debug_format)
-  [Function `native_print`](#0x1_debug_native_print)
-  [Function `native_stack_trace`](#0x1_debug_native_stack_trace)
-  [Specification](#@Specification_1)
    -  [Function `print`](#@Specification_1_print)
    -  [Function `print_stack_trace`](#@Specification_1_print_stack_trace)
    -  [Function `native_print`](#@Specification_1_native_print)
    -  [Function `native_stack_trace`](#@Specification_1_native_stack_trace)


<pre><code><b>use</b> [../../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [string_utils.md#0x1_string_utils](0x1::string_utils);
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_debug_MSG_1"></a>



<pre><code><b>const</b> [debug.md#0x1_debug_MSG_1](MSG_1): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; = [97, 98, 99, 100, 101, 102];
</code></pre>



<a id="0x1_debug_MSG_2"></a>



<pre><code><b>const</b> [debug.md#0x1_debug_MSG_2](MSG_2): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; = [49, 50, 51, 52, 53, 54];
</code></pre>



<a id="0x1_debug_print"></a>

## Function `print`



<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print](print)&lt;T&gt;(x: &T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print](print)&lt;T&gt;(x: &T) {
    [debug.md#0x1_debug_native_print](native_print)([debug.md#0x1_debug_format](format)(x));
}
</code></pre>



</details>

<a id="0x1_debug_print_stack_trace"></a>

## Function `print_stack_trace`



<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print_stack_trace](print_stack_trace)()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print_stack_trace](print_stack_trace)() {
    [debug.md#0x1_debug_native_print](native_print)([debug.md#0x1_debug_native_stack_trace](native_stack_trace)());
}
</code></pre>



</details>

<a id="0x1_debug_format"></a>

## Function `format`



<pre><code><b>fun</b> [debug.md#0x1_debug_format](format)&lt;T&gt;(x: &T): [../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> [debug.md#0x1_debug_format](format)&lt;T&gt;(x: &T): String {
    aptos_std::string_utils::debug_string(x)
}
</code></pre>



</details>

<a id="0x1_debug_native_print"></a>

## Function `native_print`



<pre><code><b>fun</b> [debug.md#0x1_debug_native_print](native_print)(x: [../../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [debug.md#0x1_debug_native_print](native_print)(x: String);
</code></pre>



</details>

<a id="0x1_debug_native_stack_trace"></a>

## Function `native_stack_trace`



<pre><code><b>fun</b> [debug.md#0x1_debug_native_stack_trace](native_stack_trace)(): [../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [debug.md#0x1_debug_native_stack_trace](native_stack_trace)(): String;
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_print"></a>

### Function `print`


<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print](print)&lt;T&gt;(x: &T)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_print_stack_trace"></a>

### Function `print_stack_trace`


<pre><code><b>public</b> <b>fun</b> [debug.md#0x1_debug_print_stack_trace](print_stack_trace)()
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_native_print"></a>

### Function `native_print`


<pre><code><b>fun</b> [debug.md#0x1_debug_native_print](native_print)(x: [../../move-stdlib/doc/string.md#0x1_string_String](string::String))
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_native_stack_trace"></a>

### Function `native_stack_trace`


<pre><code><b>fun</b> [debug.md#0x1_debug_native_stack_trace](native_stack_trace)(): [../../move-stdlib/doc/string.md#0x1_string_String](string::String)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
