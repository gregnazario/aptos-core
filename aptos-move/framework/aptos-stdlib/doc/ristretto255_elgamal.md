
<a id="0x1_ristretto255_elgamal"></a>

# Module `0x1::ristretto255_elgamal`

This module implements an ElGamal encryption API, over the Ristretto255 curve, that can be used with the
Bulletproofs module.

An ElGamal *ciphertext* is an encryption of a value <code>v</code> under a basepoint <code>G</code> and public key <code>Y = sk * G</code>, where <code>sk</code>
is the corresponding secret key, is <code>(v * G + r * Y, r * G)</code>, for a random scalar <code>r</code>.

Note that we place the value <code>v</code> "in the exponent" of <code>G</code> so that ciphertexts are additively homomorphic: i.e., so
that <code>Enc_Y(v, r) + Enc_Y(v', r') = Enc_Y(v + v', r + r')</code> where <code>v, v'</code> are plaintext messages, <code>Y</code> is a public key and <code>r, r'</code>
are the randomness of the ciphertexts.


-  [Struct `Ciphertext`](#0x1_ristretto255_elgamal_Ciphertext)
-  [Struct `CompressedCiphertext`](#0x1_ristretto255_elgamal_CompressedCiphertext)
-  [Struct `CompressedPubkey`](#0x1_ristretto255_elgamal_CompressedPubkey)
-  [Function `new_pubkey_from_bytes`](#0x1_ristretto255_elgamal_new_pubkey_from_bytes)
-  [Function `pubkey_to_bytes`](#0x1_ristretto255_elgamal_pubkey_to_bytes)
-  [Function `pubkey_to_point`](#0x1_ristretto255_elgamal_pubkey_to_point)
-  [Function `pubkey_to_compressed_point`](#0x1_ristretto255_elgamal_pubkey_to_compressed_point)
-  [Function `new_ciphertext_from_bytes`](#0x1_ristretto255_elgamal_new_ciphertext_from_bytes)
-  [Function `new_ciphertext_no_randomness`](#0x1_ristretto255_elgamal_new_ciphertext_no_randomness)
-  [Function `ciphertext_from_points`](#0x1_ristretto255_elgamal_ciphertext_from_points)
-  [Function `ciphertext_from_compressed_points`](#0x1_ristretto255_elgamal_ciphertext_from_compressed_points)
-  [Function `ciphertext_to_bytes`](#0x1_ristretto255_elgamal_ciphertext_to_bytes)
-  [Function `ciphertext_into_points`](#0x1_ristretto255_elgamal_ciphertext_into_points)
-  [Function `ciphertext_as_points`](#0x1_ristretto255_elgamal_ciphertext_as_points)
-  [Function `compress_ciphertext`](#0x1_ristretto255_elgamal_compress_ciphertext)
-  [Function `decompress_ciphertext`](#0x1_ristretto255_elgamal_decompress_ciphertext)
-  [Function `ciphertext_add`](#0x1_ristretto255_elgamal_ciphertext_add)
-  [Function `ciphertext_add_assign`](#0x1_ristretto255_elgamal_ciphertext_add_assign)
-  [Function `ciphertext_sub`](#0x1_ristretto255_elgamal_ciphertext_sub)
-  [Function `ciphertext_sub_assign`](#0x1_ristretto255_elgamal_ciphertext_sub_assign)
-  [Function `ciphertext_clone`](#0x1_ristretto255_elgamal_ciphertext_clone)
-  [Function `ciphertext_equals`](#0x1_ristretto255_elgamal_ciphertext_equals)
-  [Function `get_value_component`](#0x1_ristretto255_elgamal_get_value_component)


<pre><code><b>use</b> [../../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [ristretto255.md#0x1_ristretto255](0x1::ristretto255);
<b>use</b> [../../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_ristretto255_elgamal_Ciphertext"></a>

## Struct `Ciphertext`

An ElGamal ciphertext.


<pre><code><b>struct</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>left: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)</code>
</dt>
<dd>

</dd>
<dt>
<code>right: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_ristretto255_elgamal_CompressedCiphertext"></a>

## Struct `CompressedCiphertext`

A compressed ElGamal ciphertext.


<pre><code><b>struct</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>left: [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)</code>
</dt>
<dd>

</dd>
<dt>
<code>right: [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_ristretto255_elgamal_CompressedPubkey"></a>

## Struct `CompressedPubkey`

An ElGamal public key.


<pre><code><b>struct</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>point: [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_ristretto255_elgamal_new_pubkey_from_bytes"></a>

## Function `new_pubkey_from_bytes`

Creates a new public key from a serialized Ristretto255 point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_pubkey_from_bytes](new_pubkey_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](ristretto255_elgamal::CompressedPubkey)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_pubkey_from_bytes](new_pubkey_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): Option&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey)&gt; {
    <b>let</b> point = [ristretto255.md#0x1_ristretto255_new_compressed_point_from_bytes](ristretto255::new_compressed_point_from_bytes)(bytes);
    <b>if</b> (std::option::is_some(&<b>mut</b> point)) {
        <b>let</b> pk = [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey) {
            point: std::option::extract(&<b>mut</b> point)
        };
        std::option::some(pk)
    } <b>else</b> {
        std::option::none&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey)&gt;()
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_pubkey_to_bytes"></a>

## Function `pubkey_to_bytes`

Given an ElGamal public key <code>pubkey</code>, returns the byte representation of that public key.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_bytes](pubkey_to_bytes)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](ristretto255_elgamal::CompressedPubkey)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_bytes](pubkey_to_bytes)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    [ristretto255.md#0x1_ristretto255_compressed_point_to_bytes](ristretto255::compressed_point_to_bytes)(pubkey.point)
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_pubkey_to_point"></a>

## Function `pubkey_to_point`

Given a public key <code>pubkey</code>, returns the underlying <code>RistrettoPoint</code> representing that key.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_point](pubkey_to_point)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](ristretto255_elgamal::CompressedPubkey)): [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_point](pubkey_to_point)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey)): RistrettoPoint {
    [ristretto255.md#0x1_ristretto255_point_decompress](ristretto255::point_decompress)(&pubkey.point)
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_pubkey_to_compressed_point"></a>

## Function `pubkey_to_compressed_point`

Given a public key, returns the underlying <code>CompressedRistretto</code> point representing that key.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_compressed_point](pubkey_to_compressed_point)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](ristretto255_elgamal::CompressedPubkey)): [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_pubkey_to_compressed_point](pubkey_to_compressed_point)(pubkey: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedPubkey](CompressedPubkey)): CompressedRistretto {
    pubkey.point
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_new_ciphertext_from_bytes"></a>

## Function `new_ciphertext_from_bytes`

Creates a new ciphertext from two serialized Ristretto255 points: the first 32 bytes store <code>r * G</code> while the
next 32 bytes store <code>v * G + r * Y</code>, where <code>Y</code> is the public key.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_ciphertext_from_bytes](new_ciphertext_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_ciphertext_from_bytes](new_ciphertext_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): Option&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)&gt; {
    <b>if</b>([../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&bytes) != 64) {
        <b>return</b> std::option::none&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)&gt;()
    };

    <b>let</b> bytes_right = [../../move-stdlib/doc/vector.md#0x1_vector_trim](vector::trim)(&<b>mut</b> bytes, 32);

    <b>let</b> left_point = [ristretto255.md#0x1_ristretto255_new_point_from_bytes](ristretto255::new_point_from_bytes)(bytes);
    <b>let</b> right_point = [ristretto255.md#0x1_ristretto255_new_point_from_bytes](ristretto255::new_point_from_bytes)(bytes_right);

    <b>if</b> (std::option::is_some&lt;RistrettoPoint&gt;(&<b>mut</b> left_point) && std::option::is_some&lt;RistrettoPoint&gt;(&<b>mut</b> right_point)) {
        std::option::some&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)&gt;([ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
            left: std::option::extract&lt;RistrettoPoint&gt;(&<b>mut</b> left_point),
            right: std::option::extract&lt;RistrettoPoint&gt;(&<b>mut</b> right_point)
        })
    } <b>else</b> {
        std::option::none&lt;[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)&gt;()
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_new_ciphertext_no_randomness"></a>

## Function `new_ciphertext_no_randomness`

Creates a new ciphertext <code>(val * G + 0 * Y, 0 * G) = (val * G, 0 * G)</code> where <code>G</code> is the Ristretto255 basepoint
and the randomness is set to zero.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_ciphertext_no_randomness](new_ciphertext_no_randomness)(val: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_new_ciphertext_no_randomness](new_ciphertext_no_randomness)(val: &Scalar): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left: [ristretto255.md#0x1_ristretto255_basepoint_mul](ristretto255::basepoint_mul)(val),
        right: [ristretto255.md#0x1_ristretto255_point_identity](ristretto255::point_identity)(),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_from_points"></a>

## Function `ciphertext_from_points`

Moves a pair of Ristretto points into an ElGamal ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_from_points](ciphertext_from_points)(left: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint), right: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_from_points](ciphertext_from_points)(left: RistrettoPoint, right: RistrettoPoint): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left,
        right,
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_from_compressed_points"></a>

## Function `ciphertext_from_compressed_points`

Moves a pair of <code>CompressedRistretto</code> points into an ElGamal ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_from_compressed_points](ciphertext_from_compressed_points)(left: [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto), right: [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](ristretto255_elgamal::CompressedCiphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_from_compressed_points](ciphertext_from_compressed_points)(left: CompressedRistretto, right: CompressedRistretto): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext) {
        left,
        right,
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_to_bytes"></a>

## Function `ciphertext_to_bytes`

Given a ciphertext <code>ct</code>, serializes that ciphertext into bytes.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_to_bytes](ciphertext_to_bytes)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_to_bytes](ciphertext_to_bytes)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    <b>let</b> bytes_left = [ristretto255.md#0x1_ristretto255_point_to_bytes](ristretto255::point_to_bytes)(&[ristretto255.md#0x1_ristretto255_point_compress](ristretto255::point_compress)(&ct.left));
    <b>let</b> bytes_right = [ristretto255.md#0x1_ristretto255_point_to_bytes](ristretto255::point_to_bytes)(&[ristretto255.md#0x1_ristretto255_point_compress](ristretto255::point_compress)(&ct.right));
    <b>let</b> bytes = [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)&lt;u8&gt;();
    [../../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)&lt;u8&gt;(&<b>mut</b> bytes, bytes_left);
    [../../move-stdlib/doc/vector.md#0x1_vector_append](vector::append)&lt;u8&gt;(&<b>mut</b> bytes, bytes_right);
    bytes
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_into_points"></a>

## Function `ciphertext_into_points`

Moves the ciphertext into a pair of <code>RistrettoPoint</code>'s.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_into_points](ciphertext_into_points)(c: [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): ([ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint), [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_into_points](ciphertext_into_points)(c: [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): (RistrettoPoint, RistrettoPoint) {
    <b>let</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) { left, right } = c;
    (left, right)
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_as_points"></a>

## Function `ciphertext_as_points`

Returns the pair of <code>RistrettoPoint</code>'s representing the ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_as_points](ciphertext_as_points)(c: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): (&[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint), &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_as_points](ciphertext_as_points)(c: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): (&RistrettoPoint, &RistrettoPoint) {
    (&c.left, &c.right)
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_compress_ciphertext"></a>

## Function `compress_ciphertext`

Creates a new compressed ciphertext from a decompressed ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_compress_ciphertext](compress_ciphertext)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](ristretto255_elgamal::CompressedCiphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_compress_ciphertext](compress_ciphertext)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext) {
        left: point_compress(&ct.left),
        right: point_compress(&ct.right),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_decompress_ciphertext"></a>

## Function `decompress_ciphertext`

Creates a new decompressed ciphertext from a compressed ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_decompress_ciphertext](decompress_ciphertext)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](ristretto255_elgamal::CompressedCiphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_decompress_ciphertext](decompress_ciphertext)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_CompressedCiphertext](CompressedCiphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left: [ristretto255.md#0x1_ristretto255_point_decompress](ristretto255::point_decompress)(&ct.left),
        right: [ristretto255.md#0x1_ristretto255_point_decompress](ristretto255::point_decompress)(&ct.right),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_add"></a>

## Function `ciphertext_add`

Homomorphically combines two ciphertexts <code>lhs</code> and <code>rhs</code> as <code>lhs + rhs</code>.
Useful for re-randomizing the ciphertext or updating the committed value.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_add](ciphertext_add)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_add](ciphertext_add)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left: [ristretto255.md#0x1_ristretto255_point_add](ristretto255::point_add)(&lhs.left, &rhs.left),
        right: [ristretto255.md#0x1_ristretto255_point_add](ristretto255::point_add)(&lhs.right, &rhs.right),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_add_assign"></a>

## Function `ciphertext_add_assign`

Like <code>ciphertext_add</code> but assigns <code>lhs = lhs + rhs</code>.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_add_assign](ciphertext_add_assign)(lhs: &<b>mut</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_add_assign](ciphertext_add_assign)(lhs: &<b>mut</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)) {
    [ristretto255.md#0x1_ristretto255_point_add_assign](ristretto255::point_add_assign)(&<b>mut</b> lhs.left, &rhs.left);
    [ristretto255.md#0x1_ristretto255_point_add_assign](ristretto255::point_add_assign)(&<b>mut</b> lhs.right, &rhs.right);
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_sub"></a>

## Function `ciphertext_sub`

Homomorphically combines two ciphertexts <code>lhs</code> and <code>rhs</code> as <code>lhs - rhs</code>.
Useful for re-randomizing the ciphertext or updating the committed value.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_sub](ciphertext_sub)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_sub](ciphertext_sub)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left: [ristretto255.md#0x1_ristretto255_point_sub](ristretto255::point_sub)(&lhs.left, &rhs.left),
        right: [ristretto255.md#0x1_ristretto255_point_sub](ristretto255::point_sub)(&lhs.right, &rhs.right),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_sub_assign"></a>

## Function `ciphertext_sub_assign`

Like <code>ciphertext_add</code> but assigns <code>lhs = lhs - rhs</code>.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_sub_assign](ciphertext_sub_assign)(lhs: &<b>mut</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_sub_assign](ciphertext_sub_assign)(lhs: &<b>mut</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)) {
    [ristretto255.md#0x1_ristretto255_point_sub_assign](ristretto255::point_sub_assign)(&<b>mut</b> lhs.left, &rhs.left);
    [ristretto255.md#0x1_ristretto255_point_sub_assign](ristretto255::point_sub_assign)(&<b>mut</b> lhs.right, &rhs.right);
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_clone"></a>

## Function `ciphertext_clone`

Creates a copy of this ciphertext.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_clone](ciphertext_clone)(c: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_clone](ciphertext_clone)(c: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
    [ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext) {
        left: [ristretto255.md#0x1_ristretto255_point_clone](ristretto255::point_clone)(&c.left),
        right: [ristretto255.md#0x1_ristretto255_point_clone](ristretto255::point_clone)(&c.right),
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_ciphertext_equals"></a>

## Function `ciphertext_equals`

Returns true if the two ciphertexts are identical: i.e., same value and same randomness.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_equals](ciphertext_equals)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_ciphertext_equals](ciphertext_equals)(lhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext), rhs: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): bool {
    [ristretto255.md#0x1_ristretto255_point_equals](ristretto255::point_equals)(&lhs.left, &rhs.left) &&
    [ristretto255.md#0x1_ristretto255_point_equals](ristretto255::point_equals)(&lhs.right, &rhs.right)
}
</code></pre>



</details>

<a id="0x1_ristretto255_elgamal_get_value_component"></a>

## Function `get_value_component`

Returns the <code>RistrettoPoint</code> in the ciphertext which contains the encrypted value in the exponent.


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_get_value_component](get_value_component)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](ristretto255_elgamal::Ciphertext)): &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_elgamal.md#0x1_ristretto255_elgamal_get_value_component](get_value_component)(ct: &[ristretto255_elgamal.md#0x1_ristretto255_elgamal_Ciphertext](Ciphertext)): &RistrettoPoint {
    &ct.left
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
