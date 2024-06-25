
<a id="0x1_secp256k1"></a>

# Module `0x1::secp256k1`

This module implements ECDSA signatures based on the prime-order secp256k1 ellptic curve (i.e., cofactor is 1).


-  [Struct `ECDSARawPublicKey`](#0x1_secp256k1_ECDSARawPublicKey)
-  [Struct `ECDSASignature`](#0x1_secp256k1_ECDSASignature)
-  [Constants](#@Constants_0)
-  [Function `ecdsa_signature_from_bytes`](#0x1_secp256k1_ecdsa_signature_from_bytes)
-  [Function `ecdsa_raw_public_key_from_64_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes)
-  [Function `ecdsa_raw_public_key_to_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_to_bytes)
-  [Function `ecdsa_signature_to_bytes`](#0x1_secp256k1_ecdsa_signature_to_bytes)
-  [Function `ecdsa_recover`](#0x1_secp256k1_ecdsa_recover)
-  [Function `ecdsa_recover_internal`](#0x1_secp256k1_ecdsa_recover_internal)
-  [Specification](#@Specification_1)
    -  [Function `ecdsa_signature_from_bytes`](#@Specification_1_ecdsa_signature_from_bytes)
    -  [Function `ecdsa_raw_public_key_from_64_bytes`](#@Specification_1_ecdsa_raw_public_key_from_64_bytes)
    -  [Function `ecdsa_raw_public_key_to_bytes`](#@Specification_1_ecdsa_raw_public_key_to_bytes)
    -  [Function `ecdsa_signature_to_bytes`](#@Specification_1_ecdsa_signature_to_bytes)
    -  [Function `ecdsa_recover`](#@Specification_1_ecdsa_recover)
    -  [Function `ecdsa_recover_internal`](#@Specification_1_ecdsa_recover_internal)


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../move-stdlib/doc/option.md#0x1_option](0x1::option);
</code></pre>



<a id="0x1_secp256k1_ECDSARawPublicKey"></a>

## Struct `ECDSARawPublicKey`

A 64-byte ECDSA public key.


<pre><code><b>struct</b> [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_secp256k1_ECDSASignature"></a>

## Struct `ECDSASignature`

A 64-byte ECDSA signature.


<pre><code><b>struct</b> [secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_secp256k1_SIGNATURE_NUM_BYTES"></a>

The size of a secp256k1-based ECDSA signature, in bytes.


<pre><code><b>const</b> [secp256k1.md#0x1_secp256k1_SIGNATURE_NUM_BYTES](SIGNATURE_NUM_BYTES): u64 = 64;
</code></pre>



<a id="0x1_secp256k1_E_DESERIALIZE"></a>

An error occurred while deserializing, for example due to wrong input size.


<pre><code><b>const</b> [secp256k1.md#0x1_secp256k1_E_DESERIALIZE](E_DESERIALIZE): u64 = 1;
</code></pre>



<a id="0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES"></a>

The size of a secp256k1-based ECDSA public key, in bytes.


<pre><code><b>const</b> [secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES](RAW_PUBLIC_KEY_NUM_BYTES): u64 = 64;
</code></pre>



<a id="0x1_secp256k1_ecdsa_signature_from_bytes"></a>

## Function `ecdsa_signature_from_bytes`

Constructs an ECDSASignature struct from the given 64 bytes.


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_from_bytes](ecdsa_signature_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_from_bytes](ecdsa_signature_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature) {
    <b>assert</b>!(std::vector::length(&bytes) == [secp256k1.md#0x1_secp256k1_SIGNATURE_NUM_BYTES](SIGNATURE_NUM_BYTES), std::error::invalid_argument([secp256k1.md#0x1_secp256k1_E_DESERIALIZE](E_DESERIALIZE)));
    [secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature) { bytes }
}
</code></pre>



</details>

<a id="0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes"></a>

## Function `ecdsa_raw_public_key_from_64_bytes`

Constructs an ECDSARawPublicKey struct, given a 64-byte raw representation.


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes](ecdsa_raw_public_key_from_64_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes](ecdsa_raw_public_key_from_64_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey) {
    <b>assert</b>!(std::vector::length(&bytes) == [secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES](RAW_PUBLIC_KEY_NUM_BYTES), std::error::invalid_argument([secp256k1.md#0x1_secp256k1_E_DESERIALIZE](E_DESERIALIZE)));
    [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey) { bytes }
}
</code></pre>



</details>

<a id="0x1_secp256k1_ecdsa_raw_public_key_to_bytes"></a>

## Function `ecdsa_raw_public_key_to_bytes`

Serializes an ECDSARawPublicKey struct to 64-bytes.


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_to_bytes](ecdsa_raw_public_key_to_bytes)(pk: &[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_to_bytes](ecdsa_raw_public_key_to_bytes)(pk: &[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    pk.bytes
}
</code></pre>



</details>

<a id="0x1_secp256k1_ecdsa_signature_to_bytes"></a>

## Function `ecdsa_signature_to_bytes`

Serializes an ECDSASignature struct to 64-bytes.


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_to_bytes](ecdsa_signature_to_bytes)(sig: &[secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_to_bytes](ecdsa_signature_to_bytes)(sig: &[secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    sig.bytes
}
</code></pre>



</details>

<a id="0x1_secp256k1_ecdsa_recover"></a>

## Function `ecdsa_recover`

Recovers the signer's raw (64-byte) public key from a secp256k1 ECDSA <code>signature</code> given the <code>recovery_id</code> and the signed
<code>message</code> (32 byte digest).

Note that an invalid signature, or a signature from a different message, will result in the recovery of an
incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer's
public key (or its hash) is known beforehand.


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover](ecdsa_recover)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: &[secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover](ecdsa_recover)(
    message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;,
    recovery_id: u8,
    signature: &[secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature),
): Option&lt;[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey)&gt; {
    <b>let</b> (pk, success) = [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal](ecdsa_recover_internal)(message, recovery_id, signature.bytes);
    <b>if</b> (success) {
        std::option::some([secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes](ecdsa_raw_public_key_from_64_bytes)(pk))
    } <b>else</b> {
        std::option::none&lt;[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey)&gt;()
    }
}
</code></pre>



</details>

<a id="0x1_secp256k1_ecdsa_recover_internal"></a>

## Function `ecdsa_recover_internal`

Returns <code>(public_key, <b>true</b>)</code> if <code>signature</code> verifies on <code>message</code> under the recovered <code>public_key</code>
and returns <code>([], <b>false</b>)</code> otherwise.


<pre><code><b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal](ecdsa_recover_internal)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal](ecdsa_recover_internal)(
    message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;,
    recovery_id: u8,
    signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, bool);
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_ecdsa_signature_from_bytes"></a>

### Function `ecdsa_signature_from_bytes`


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_from_bytes](ecdsa_signature_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)
</code></pre>




<pre><code><b>aborts_if</b> len(bytes) != [secp256k1.md#0x1_secp256k1_SIGNATURE_NUM_BYTES](SIGNATURE_NUM_BYTES);
<b>ensures</b> result == [secp256k1.md#0x1_secp256k1_ECDSASignature](ECDSASignature) { bytes };
</code></pre>



<a id="@Specification_1_ecdsa_raw_public_key_from_64_bytes"></a>

### Function `ecdsa_raw_public_key_from_64_bytes`


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes](ecdsa_raw_public_key_from_64_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)
</code></pre>




<pre><code><b>aborts_if</b> len(bytes) != [secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES](RAW_PUBLIC_KEY_NUM_BYTES);
<b>ensures</b> result == [secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey) { bytes };
</code></pre>



<a id="@Specification_1_ecdsa_raw_public_key_to_bytes"></a>

### Function `ecdsa_raw_public_key_to_bytes`


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_to_bytes](ecdsa_raw_public_key_to_bytes)(pk: &[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == pk.bytes;
</code></pre>



<a id="@Specification_1_ecdsa_signature_to_bytes"></a>

### Function `ecdsa_signature_to_bytes`


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_signature_to_bytes](ecdsa_signature_to_bytes)(sig: &[secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == sig.bytes;
</code></pre>



<a id="@Specification_1_ecdsa_recover"></a>

### Function `ecdsa_recover`


<pre><code><b>public</b> <b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover](ecdsa_recover)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: &[secp256k1.md#0x1_secp256k1_ECDSASignature](secp256k1::ECDSASignature)): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](secp256k1::ECDSARawPublicKey)&gt;
</code></pre>




<pre><code><b>aborts_if</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal_abort_condition](ecdsa_recover_internal_abort_condition)(message, recovery_id, signature.bytes);
<b>let</b> pk = [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_1](spec_ecdsa_recover_internal_result_1)(message, recovery_id, signature.bytes);
<b>let</b> success = [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_2](spec_ecdsa_recover_internal_result_2)(message, recovery_id, signature.bytes);
<b>ensures</b> success ==&gt; result == std::option::spec_some([secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes](ecdsa_raw_public_key_from_64_bytes)(pk));
<b>ensures</b> !success ==&gt; result == std::option::spec_none&lt;[secp256k1.md#0x1_secp256k1_ECDSARawPublicKey](ECDSARawPublicKey)&gt;();
</code></pre>



<a id="@Specification_1_ecdsa_recover_internal"></a>

### Function `ecdsa_recover_internal`


<pre><code><b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal](ecdsa_recover_internal)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): ([../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, bool)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal_abort_condition](ecdsa_recover_internal_abort_condition)(message, recovery_id, signature);
<b>ensures</b> result_1 == [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_1](spec_ecdsa_recover_internal_result_1)(message, recovery_id, signature);
<b>ensures</b> result_2 == [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_2](spec_ecdsa_recover_internal_result_2)(message, recovery_id, signature);
<b>ensures</b> len(result_1) == <b>if</b> (result_2) { [secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES](RAW_PUBLIC_KEY_NUM_BYTES) } <b>else</b> { 0 };
</code></pre>




<a id="0x1_secp256k1_ecdsa_recover_internal_abort_condition"></a>


<pre><code><b>fun</b> [secp256k1.md#0x1_secp256k1_ecdsa_recover_internal_abort_condition](ecdsa_recover_internal_abort_condition)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): bool;
</code></pre>




<a id="0x1_secp256k1_spec_ecdsa_recover_internal_result_1"></a>


<pre><code><b>fun</b> [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_1](spec_ecdsa_recover_internal_result_1)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;;
</code></pre>




<a id="0x1_secp256k1_spec_ecdsa_recover_internal_result_2"></a>


<pre><code><b>fun</b> [secp256k1.md#0x1_secp256k1_spec_ecdsa_recover_internal_result_2](spec_ecdsa_recover_internal_result_2)(message: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;, recovery_id: u8, signature: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): bool;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
