
<a id="0x1_acl"></a>

# Module `0x1::acl`

Access control list (acl) module. An acl is a list of account addresses who
have the access permission to a certain object.
This module uses a <code>[vector.md#0x1_vector](vector)</code> to represent the list, but can be refactored to
use a "set" instead when it's available in the language in the future.


-  [Struct `ACL`](#0x1_acl_ACL)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_acl_empty)
-  [Function `add`](#0x1_acl_add)
-  [Function `remove`](#0x1_acl_remove)
-  [Function `contains`](#0x1_acl_contains)
-  [Function `assert_contains`](#0x1_acl_assert_contains)
-  [Specification](#@Specification_1)
    -  [Struct `ACL`](#@Specification_1_ACL)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `assert_contains`](#@Specification_1_assert_contains)


<pre><code><b>use</b> [error.md#0x1_error](0x1::error);
<b>use</b> [vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_acl_ACL"></a>

## Struct `ACL`



<pre><code><b>struct</b> [acl.md#0x1_acl_ACL](ACL) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>list: [vector.md#0x1_vector](vector)&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_acl_ECONTAIN"></a>

The ACL already contains the address.


<pre><code><b>const</b> [acl.md#0x1_acl_ECONTAIN](ECONTAIN): u64 = 0;
</code></pre>



<a id="0x1_acl_ENOT_CONTAIN"></a>

The ACL does not contain the address.


<pre><code><b>const</b> [acl.md#0x1_acl_ENOT_CONTAIN](ENOT_CONTAIN): u64 = 1;
</code></pre>



<a id="0x1_acl_empty"></a>

## Function `empty`

Return an empty ACL.


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_empty](empty)(): [acl.md#0x1_acl_ACL](acl::ACL)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_empty](empty)(): [acl.md#0x1_acl_ACL](ACL) {
    [acl.md#0x1_acl_ACL](ACL){ list: [vector.md#0x1_vector_empty](vector::empty)&lt;<b>address</b>&gt;() }
}
</code></pre>



</details>

<a id="0x1_acl_add"></a>

## Function `add`

Add the address to the ACL.


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_add](add)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_add](add)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](ACL), addr: <b>address</b>) {
    <b>assert</b>!(![vector.md#0x1_vector_contains](vector::contains)(&<b>mut</b> [acl.md#0x1_acl](acl).list, &addr), [error.md#0x1_error_invalid_argument](error::invalid_argument)([acl.md#0x1_acl_ECONTAIN](ECONTAIN)));
    [vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> [acl.md#0x1_acl](acl).list, addr);
}
</code></pre>



</details>

<a id="0x1_acl_remove"></a>

## Function `remove`

Remove the address from the ACL.


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_remove](remove)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_remove](remove)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](ACL), addr: <b>address</b>) {
    <b>let</b> (found, index) = [vector.md#0x1_vector_index_of](vector::index_of)(&<b>mut</b> [acl.md#0x1_acl](acl).list, &addr);
    <b>assert</b>!(found, [error.md#0x1_error_invalid_argument](error::invalid_argument)([acl.md#0x1_acl_ENOT_CONTAIN](ENOT_CONTAIN)));
    [vector.md#0x1_vector_remove](vector::remove)(&<b>mut</b> [acl.md#0x1_acl](acl).list, index);
}
</code></pre>



</details>

<a id="0x1_acl_contains"></a>

## Function `contains`

Return true iff the ACL contains the address.


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_contains](contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_contains](contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](ACL), addr: <b>address</b>): bool {
    [vector.md#0x1_vector_contains](vector::contains)(&[acl.md#0x1_acl](acl).list, &addr)
}
</code></pre>



</details>

<a id="0x1_acl_assert_contains"></a>

## Function `assert_contains`

assert! that the ACL has the address.


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_assert_contains](assert_contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_assert_contains](assert_contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](ACL), addr: <b>address</b>) {
    <b>assert</b>!([acl.md#0x1_acl_contains](contains)([acl.md#0x1_acl](acl), addr), [error.md#0x1_error_invalid_argument](error::invalid_argument)([acl.md#0x1_acl_ENOT_CONTAIN](ENOT_CONTAIN)));
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_ACL"></a>

### Struct `ACL`


<pre><code><b>struct</b> [acl.md#0x1_acl_ACL](ACL) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<dl>
<dt>
<code>list: [vector.md#0x1_vector](vector)&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>invariant</b> <b>forall</b> i in 0..len(list), j in 0..len(list): list[i] == list[j] ==&gt; i == j;
</code></pre>




<a id="0x1_acl_spec_contains"></a>


<pre><code><b>fun</b> [acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl): [acl.md#0x1_acl_ACL](ACL), addr: <b>address</b>): bool {
   <b>exists</b> a in [acl.md#0x1_acl](acl).list: a == addr
}
</code></pre>



<a id="@Specification_1_add"></a>

### Function `add`


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_add](add)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>




<pre><code><b>aborts_if</b> [acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr) <b>with</b> [error.md#0x1_error_INVALID_ARGUMENT](error::INVALID_ARGUMENT);
<b>ensures</b> [acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr);
</code></pre>



<a id="@Specification_1_remove"></a>

### Function `remove`


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_remove](remove)([acl.md#0x1_acl](acl): &<b>mut</b> [acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>




<pre><code><b>aborts_if</b> ![acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr) <b>with</b> [error.md#0x1_error_INVALID_ARGUMENT](error::INVALID_ARGUMENT);
<b>ensures</b> ![acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr);
</code></pre>



<a id="@Specification_1_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_contains](contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>): bool
</code></pre>




<pre><code><b>ensures</b> result == [acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr);
</code></pre>



<a id="@Specification_1_assert_contains"></a>

### Function `assert_contains`


<pre><code><b>public</b> <b>fun</b> [acl.md#0x1_acl_assert_contains](assert_contains)([acl.md#0x1_acl](acl): &[acl.md#0x1_acl_ACL](acl::ACL), addr: <b>address</b>)
</code></pre>




<pre><code><b>aborts_if</b> ![acl.md#0x1_acl_spec_contains](spec_contains)([acl.md#0x1_acl](acl), addr) <b>with</b> [error.md#0x1_error_INVALID_ARGUMENT](error::INVALID_ARGUMENT);
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
