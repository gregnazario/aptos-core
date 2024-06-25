
<a id="0x4_royalty"></a>

# Module `0x4::royalty`

This defines an object-based Royalty. The royalty can be applied to either a collection or a
token. Applications should read the royalty from the token, as it will read the appropriate
royalty.


-  [Resource `Royalty`](#0x4_royalty_Royalty)
-  [Struct `MutatorRef`](#0x4_royalty_MutatorRef)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x4_royalty_init)
-  [Function `update`](#0x4_royalty_update)
-  [Function `create`](#0x4_royalty_create)
-  [Function `generate_mutator_ref`](#0x4_royalty_generate_mutator_ref)
-  [Function `exists_at`](#0x4_royalty_exists_at)
-  [Function `delete`](#0x4_royalty_delete)
-  [Function `get`](#0x4_royalty_get)
-  [Function `denominator`](#0x4_royalty_denominator)
-  [Function `numerator`](#0x4_royalty_numerator)
-  [Function `payee_address`](#0x4_royalty_payee_address)


<pre><code><b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/doc/object.md#0x1_object](0x1::object);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option](0x1::option);
</code></pre>



<a id="0x4_royalty_Royalty"></a>

## Resource `Royalty`

The royalty of a token within this collection

Royalties are optional for a collection.  Royalty percentage is calculated
by (numerator / denominator) * 100%


<pre><code>#[resource_group_member(#[group = [../../aptos-framework/doc/object.md#0x1_object_ObjectGroup](0x1::object::ObjectGroup)])]
<b>struct</b> [royalty.md#0x4_royalty_Royalty](Royalty) <b>has</b> <b>copy</b>, drop, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>numerator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>denominator: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>payee_address: <b>address</b></code>
</dt>
<dd>
 The recipient of royalty payments. See the <code>shared_account</code> for how to handle multiple
 creators.
</dd>
</dl>


</details>

<a id="0x4_royalty_MutatorRef"></a>

## Struct `MutatorRef`

This enables creating or overwriting a <code>[royalty.md#0x4_royalty_MutatorRef](MutatorRef)</code>.


<pre><code><b>struct</b> [royalty.md#0x4_royalty_MutatorRef](MutatorRef) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: [../../aptos-framework/doc/object.md#0x1_object_ExtendRef](object::ExtendRef)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x4_royalty_EROYALTY_DENOMINATOR_IS_ZERO"></a>

The royalty denominator cannot be 0


<pre><code><b>const</b> [royalty.md#0x4_royalty_EROYALTY_DENOMINATOR_IS_ZERO](EROYALTY_DENOMINATOR_IS_ZERO): u64 = 3;
</code></pre>



<a id="0x4_royalty_EROYALTY_DOES_NOT_EXIST"></a>

Royalty does not exist


<pre><code><b>const</b> [royalty.md#0x4_royalty_EROYALTY_DOES_NOT_EXIST](EROYALTY_DOES_NOT_EXIST): u64 = 1;
</code></pre>



<a id="0x4_royalty_EROYALTY_EXCEEDS_MAXIMUM"></a>

The royalty cannot be greater than 100%


<pre><code><b>const</b> [royalty.md#0x4_royalty_EROYALTY_EXCEEDS_MAXIMUM](EROYALTY_EXCEEDS_MAXIMUM): u64 = 2;
</code></pre>



<a id="0x4_royalty_init"></a>

## Function `init`

Add a royalty, given a ConstructorRef.


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_init](init)(ref: &[../../aptos-framework/doc/object.md#0x1_object_ConstructorRef](object::ConstructorRef), [royalty.md#0x4_royalty](royalty): [royalty.md#0x4_royalty_Royalty](royalty::Royalty))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_init](init)(ref: &ConstructorRef, [royalty.md#0x4_royalty](royalty): [royalty.md#0x4_royalty_Royalty](Royalty)) {
    <b>let</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer) = [../../aptos-framework/doc/object.md#0x1_object_generate_signer](object::generate_signer)(ref);
    <b>move_to</b>(&[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [royalty.md#0x4_royalty](royalty));
}
</code></pre>



</details>

<a id="0x4_royalty_update"></a>

## Function `update`

Set the royalty if it does not exist, replace it otherwise.


<pre><code><b>public</b> <b>fun</b> <b>update</b>(mutator_ref: &[royalty.md#0x4_royalty_MutatorRef](royalty::MutatorRef), [royalty.md#0x4_royalty](royalty): [royalty.md#0x4_royalty_Royalty](royalty::Royalty))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>update</b>(mutator_ref: &[royalty.md#0x4_royalty_MutatorRef](MutatorRef), [royalty.md#0x4_royalty](royalty): [royalty.md#0x4_royalty_Royalty](Royalty)) <b>acquires</b> [royalty.md#0x4_royalty_Royalty](Royalty) {
    <b>let</b> addr = [../../aptos-framework/doc/object.md#0x1_object_address_from_extend_ref](object::address_from_extend_ref)(&mutator_ref.inner);
    <b>if</b> (<b>exists</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(addr)) {
        <b>move_from</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(addr);
    };

    <b>let</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer) = [../../aptos-framework/doc/object.md#0x1_object_generate_signer_for_extending](object::generate_signer_for_extending)(&mutator_ref.inner);
    <b>move_to</b>(&[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), [royalty.md#0x4_royalty](royalty));
}
</code></pre>



</details>

<a id="0x4_royalty_create"></a>

## Function `create`

Creates a new royalty, verifying that it is a valid percentage


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_create](create)(numerator: u64, denominator: u64, payee_address: <b>address</b>): [royalty.md#0x4_royalty_Royalty](royalty::Royalty)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_create](create)(numerator: u64, denominator: u64, payee_address: <b>address</b>): [royalty.md#0x4_royalty_Royalty](Royalty) {
    <b>assert</b>!(denominator != 0, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([royalty.md#0x4_royalty_EROYALTY_DENOMINATOR_IS_ZERO](EROYALTY_DENOMINATOR_IS_ZERO)));
    <b>assert</b>!([royalty.md#0x4_royalty_numerator](numerator) &lt;= denominator, [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range](error::out_of_range)([royalty.md#0x4_royalty_EROYALTY_EXCEEDS_MAXIMUM](EROYALTY_EXCEEDS_MAXIMUM)));

    [royalty.md#0x4_royalty_Royalty](Royalty) { numerator, denominator, payee_address }
}
</code></pre>



</details>

<a id="0x4_royalty_generate_mutator_ref"></a>

## Function `generate_mutator_ref`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_generate_mutator_ref](generate_mutator_ref)(ref: [../../aptos-framework/doc/object.md#0x1_object_ExtendRef](object::ExtendRef)): [royalty.md#0x4_royalty_MutatorRef](royalty::MutatorRef)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_generate_mutator_ref](generate_mutator_ref)(ref: ExtendRef): [royalty.md#0x4_royalty_MutatorRef](MutatorRef) {
    [royalty.md#0x4_royalty_MutatorRef](MutatorRef) { inner: ref }
}
</code></pre>



</details>

<a id="0x4_royalty_exists_at"></a>

## Function `exists_at`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_exists_at](exists_at)(addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_exists_at](exists_at)(addr: <b>address</b>): bool {
    <b>exists</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(addr)
}
</code></pre>



</details>

<a id="0x4_royalty_delete"></a>

## Function `delete`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [royalty.md#0x4_royalty_delete](delete)(addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> [royalty.md#0x4_royalty_delete](delete)(addr: <b>address</b>) <b>acquires</b> [royalty.md#0x4_royalty_Royalty](Royalty) {
    <b>assert</b>!(<b>exists</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(addr), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([royalty.md#0x4_royalty_EROYALTY_DOES_NOT_EXIST](EROYALTY_DOES_NOT_EXIST)));
    <b>move_from</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(addr);
}
</code></pre>



</details>

<a id="0x4_royalty_get"></a>

## Function `get`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_get](get)&lt;T: key&gt;(maybe_royalty: [../../aptos-framework/doc/object.md#0x1_object_Object](object::Object)&lt;T&gt;): [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[royalty.md#0x4_royalty_Royalty](royalty::Royalty)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_get](get)&lt;T: key&gt;(maybe_royalty: Object&lt;T&gt;): Option&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt; <b>acquires</b> [royalty.md#0x4_royalty_Royalty](Royalty) {
    <b>let</b> obj_addr = [../../aptos-framework/doc/object.md#0x1_object_object_address](object::object_address)(&maybe_royalty);
    <b>if</b> (<b>exists</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(obj_addr)) {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_some](option::some)(*<b>borrow_global</b>&lt;[royalty.md#0x4_royalty_Royalty](Royalty)&gt;(obj_addr))
    } <b>else</b> {
        [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/option.md#0x1_option_none](option::none)()
    }
}
</code></pre>



</details>

<a id="0x4_royalty_denominator"></a>

## Function `denominator`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_denominator](denominator)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](royalty::Royalty)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_denominator](denominator)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](Royalty)): u64 {
    [royalty.md#0x4_royalty](royalty).denominator
}
</code></pre>



</details>

<a id="0x4_royalty_numerator"></a>

## Function `numerator`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_numerator](numerator)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](royalty::Royalty)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_numerator](numerator)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](Royalty)): u64 {
    [royalty.md#0x4_royalty](royalty).numerator
}
</code></pre>



</details>

<a id="0x4_royalty_payee_address"></a>

## Function `payee_address`



<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_payee_address](payee_address)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](royalty::Royalty)): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [royalty.md#0x4_royalty_payee_address](payee_address)([royalty.md#0x4_royalty](royalty): &[royalty.md#0x4_royalty_Royalty](Royalty)): <b>address</b> {
    [royalty.md#0x4_royalty](royalty).payee_address
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
