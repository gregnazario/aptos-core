
<a id="0x1_ristretto255_pedersen"></a>

# Module `0x1::ristretto255_pedersen`

This module implements a Pedersen commitment API, over the Ristretto255 curve, that can be used with the
Bulletproofs module.

A Pedersen commitment to a value <code>v</code> under _commitment key_ <code>(g, h)</code> is <code>v * g + r * h</code>, for a random scalar <code>r</code>.


-  [Struct `Commitment`](#0x1_ristretto255_pedersen_Commitment)
-  [Constants](#@Constants_0)
-  [Function `new_commitment_from_bytes`](#0x1_ristretto255_pedersen_new_commitment_from_bytes)
-  [Function `commitment_to_bytes`](#0x1_ristretto255_pedersen_commitment_to_bytes)
-  [Function `commitment_from_point`](#0x1_ristretto255_pedersen_commitment_from_point)
-  [Function `commitment_from_compressed`](#0x1_ristretto255_pedersen_commitment_from_compressed)
-  [Function `new_commitment`](#0x1_ristretto255_pedersen_new_commitment)
-  [Function `new_commitment_with_basepoint`](#0x1_ristretto255_pedersen_new_commitment_with_basepoint)
-  [Function `new_commitment_for_bulletproof`](#0x1_ristretto255_pedersen_new_commitment_for_bulletproof)
-  [Function `commitment_add`](#0x1_ristretto255_pedersen_commitment_add)
-  [Function `commitment_add_assign`](#0x1_ristretto255_pedersen_commitment_add_assign)
-  [Function `commitment_sub`](#0x1_ristretto255_pedersen_commitment_sub)
-  [Function `commitment_sub_assign`](#0x1_ristretto255_pedersen_commitment_sub_assign)
-  [Function `commitment_clone`](#0x1_ristretto255_pedersen_commitment_clone)
-  [Function `commitment_equals`](#0x1_ristretto255_pedersen_commitment_equals)
-  [Function `commitment_as_point`](#0x1_ristretto255_pedersen_commitment_as_point)
-  [Function `commitment_as_compressed_point`](#0x1_ristretto255_pedersen_commitment_as_compressed_point)
-  [Function `commitment_into_point`](#0x1_ristretto255_pedersen_commitment_into_point)
-  [Function `commitment_into_compressed_point`](#0x1_ristretto255_pedersen_commitment_into_compressed_point)
-  [Function `randomness_base_for_bulletproof`](#0x1_ristretto255_pedersen_randomness_base_for_bulletproof)


<pre><code><b>use</b> [../../move-stdlib/doc/option.md#0x1_option](0x1::option);
<b>use</b> [ristretto255.md#0x1_ristretto255](0x1::ristretto255);
</code></pre>



<a id="0x1_ristretto255_pedersen_Commitment"></a>

## Struct `Commitment`

A Pedersen commitment to some value with some randomness.


<pre><code><b>struct</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>point: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_ristretto255_pedersen_BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE"></a>

The default Pedersen randomness base <code>h</code> used in our underlying Bulletproofs library.
This is obtained by hashing the compressed Ristretto255 basepoint using SHA3-512 (not SHA2-512).


<pre><code><b>const</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE](BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; = [140, 146, 64, 180, 86, 169, 230, 220, 101, 195, 119, 161, 4, 141, 116, 95, 148, 160, 140, 219, 127, 68, 203, 205, 123, 70, 243, 64, 72, 135, 17, 52];
</code></pre>



<a id="0x1_ristretto255_pedersen_new_commitment_from_bytes"></a>

## Function `new_commitment_from_bytes`

Creates a new public key from a serialized Ristretto255 point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_from_bytes](new_commitment_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): [../../move-stdlib/doc/option.md#0x1_option_Option](option::Option)&lt;[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_from_bytes](new_commitment_from_bytes)(bytes: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;): Option&lt;[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)&gt; {
    <b>let</b> point = [ristretto255.md#0x1_ristretto255_new_point_from_bytes](ristretto255::new_point_from_bytes)(bytes);
    <b>if</b> (std::option::is_some(&<b>mut</b> point)) {
        <b>let</b> comm = [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
            point: std::option::extract(&<b>mut</b> point)
        };
        std::option::some(comm)
    } <b>else</b> {
        std::option::none&lt;[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)&gt;()
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_to_bytes"></a>

## Function `commitment_to_bytes`

Returns a commitment as a serialized byte array


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_to_bytes](commitment_to_bytes)(comm: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_to_bytes](commitment_to_bytes)(comm: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;u8&gt; {
    [ristretto255.md#0x1_ristretto255_point_to_bytes](ristretto255::point_to_bytes)(&[ristretto255.md#0x1_ristretto255_point_compress](ristretto255::point_compress)(&comm.point))
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_from_point"></a>

## Function `commitment_from_point`

Moves a Ristretto point into a Pedersen commitment.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_from_point](commitment_from_point)(point: [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_from_point](commitment_from_point)(point: RistrettoPoint): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_from_compressed"></a>

## Function `commitment_from_compressed`

Deserializes a commitment from a compressed Ristretto point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_from_compressed](commitment_from_compressed)(point: &[ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_from_compressed](commitment_from_compressed)(point: &CompressedRistretto): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_point_decompress](ristretto255::point_decompress)(point)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_new_commitment"></a>

## Function `new_commitment`

Returns a commitment <code>v * val_base + r * rand_base</code> where <code>(val_base, rand_base)</code> is the commitment key.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment](new_commitment)(v: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar), val_base: &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint), r: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar), rand_base: &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment](new_commitment)(v: &Scalar, val_base: &RistrettoPoint, r: &Scalar, rand_base: &RistrettoPoint): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_double_scalar_mul](ristretto255::double_scalar_mul)(v, val_base, r, rand_base)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_new_commitment_with_basepoint"></a>

## Function `new_commitment_with_basepoint`

Returns a commitment <code>v * G + r * rand_base</code> where <code>G</code> is the Ristretto255 basepoint.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_with_basepoint](new_commitment_with_basepoint)(v: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar), r: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar), rand_base: &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_with_basepoint](new_commitment_with_basepoint)(v: &Scalar, r: &Scalar, rand_base: &RistrettoPoint): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_basepoint_double_mul](ristretto255::basepoint_double_mul)(r, rand_base, v)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_new_commitment_for_bulletproof"></a>

## Function `new_commitment_for_bulletproof`

Returns a commitment <code>v * G + r * H</code> where <code>G</code> is the Ristretto255 basepoint and <code>H</code> is the default randomness
base used in the Bulletproofs library (i.e., <code>[ristretto255_pedersen.md#0x1_ristretto255_pedersen_BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE](BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE)</code>).


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_for_bulletproof](new_commitment_for_bulletproof)(v: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar), r: &[ristretto255.md#0x1_ristretto255_Scalar](ristretto255::Scalar)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_new_commitment_for_bulletproof](new_commitment_for_bulletproof)(v: &Scalar, r: &Scalar): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    <b>let</b> rand_base = [ristretto255.md#0x1_ristretto255_new_point_from_bytes](ristretto255::new_point_from_bytes)([ristretto255_pedersen.md#0x1_ristretto255_pedersen_BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE](BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE));
    <b>let</b> rand_base = std::option::extract(&<b>mut</b> rand_base);

    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_basepoint_double_mul](ristretto255::basepoint_double_mul)(r, &rand_base, v)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_add"></a>

## Function `commitment_add`

Homomorphically combines two commitments <code>lhs</code> and <code>rhs</code> as <code>lhs + rhs</code>.
Useful for re-randomizing the commitment or updating the committed value.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_add](commitment_add)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_add](commitment_add)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_point_add](ristretto255::point_add)(&lhs.point, &rhs.point)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_add_assign"></a>

## Function `commitment_add_assign`

Like <code>commitment_add</code> but assigns <code>lhs = lhs + rhs</code>.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_add_assign](commitment_add_assign)(lhs: &<b>mut</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_add_assign](commitment_add_assign)(lhs: &<b>mut</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)) {
    [ristretto255.md#0x1_ristretto255_point_add_assign](ristretto255::point_add_assign)(&<b>mut</b> lhs.point, &rhs.point);
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_sub"></a>

## Function `commitment_sub`

Homomorphically combines two commitments <code>lhs</code> and <code>rhs</code> as <code>lhs - rhs</code>.
Useful for re-randomizing the commitment or updating the committed value.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_sub](commitment_sub)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_sub](commitment_sub)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_point_sub](ristretto255::point_sub)(&lhs.point, &rhs.point)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_sub_assign"></a>

## Function `commitment_sub_assign`

Like <code>commitment_add</code> but assigns <code>lhs = lhs - rhs</code>.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_sub_assign](commitment_sub_assign)(lhs: &<b>mut</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_sub_assign](commitment_sub_assign)(lhs: &<b>mut</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)) {
    [ristretto255.md#0x1_ristretto255_point_sub_assign](ristretto255::point_sub_assign)(&<b>mut</b> lhs.point, &rhs.point);
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_clone"></a>

## Function `commitment_clone`

Creates a copy of this commitment.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_clone](commitment_clone)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_clone](commitment_clone)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
    [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) {
        point: [ristretto255.md#0x1_ristretto255_point_clone](ristretto255::point_clone)(&c.point)
    }
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_equals"></a>

## Function `commitment_equals`

Returns true if the two commitments are identical: i.e., same value and same randomness.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_equals](commitment_equals)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_equals](commitment_equals)(lhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment), rhs: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): bool {
    [ristretto255.md#0x1_ristretto255_point_equals](ristretto255::point_equals)(&lhs.point, &rhs.point)
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_as_point"></a>

## Function `commitment_as_point`

Returns the underlying elliptic curve point representing the commitment as an in-memory <code>RistrettoPoint</code>.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_as_point](commitment_as_point)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): &[ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_as_point](commitment_as_point)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): &RistrettoPoint {
    &c.point
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_as_compressed_point"></a>

## Function `commitment_as_compressed_point`

Returns the Pedersen commitment as a <code>CompressedRistretto</code> point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_as_compressed_point](commitment_as_compressed_point)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_as_compressed_point](commitment_as_compressed_point)(c: &[ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): CompressedRistretto {
    point_compress(&c.point)
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_into_point"></a>

## Function `commitment_into_point`

Moves the Commitment into a CompressedRistretto point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_into_point](commitment_into_point)(c: [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_into_point](commitment_into_point)(c: [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): RistrettoPoint {
    <b>let</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment) { point } = c;
    point
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_commitment_into_compressed_point"></a>

## Function `commitment_into_compressed_point`

Moves the Commitment into a <code>CompressedRistretto</code> point.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_into_compressed_point](commitment_into_compressed_point)(c: [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](ristretto255_pedersen::Commitment)): [ristretto255.md#0x1_ristretto255_CompressedRistretto](ristretto255::CompressedRistretto)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_commitment_into_compressed_point](commitment_into_compressed_point)(c: [ristretto255_pedersen.md#0x1_ristretto255_pedersen_Commitment](Commitment)): CompressedRistretto {
    point_compress(&c.point)
}
</code></pre>



</details>

<a id="0x1_ristretto255_pedersen_randomness_base_for_bulletproof"></a>

## Function `randomness_base_for_bulletproof`

Returns the randomness base compatible with the Bulletproofs module.

Recal that a Bulletproof range proof attests, in zero-knowledge, that a value <code>v</code> inside a Pedersen commitment
<code>v * g + r * h</code> is sufficiently "small" (e.g., is 32-bits wide). Here, <code>h</code> is referred to as the
"randomness base" of the commitment scheme.

Bulletproof has a default choice for <code>g</code> and <code>h</code> and this function returns the default <code>h</code> as used in the
Bulletproofs Move module.


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_randomness_base_for_bulletproof](randomness_base_for_bulletproof)(): [ristretto255.md#0x1_ristretto255_RistrettoPoint](ristretto255::RistrettoPoint)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [ristretto255_pedersen.md#0x1_ristretto255_pedersen_randomness_base_for_bulletproof](randomness_base_for_bulletproof)(): RistrettoPoint {
    std::option::extract(&<b>mut</b> [ristretto255.md#0x1_ristretto255_new_point_from_bytes](ristretto255::new_point_from_bytes)([ristretto255_pedersen.md#0x1_ristretto255_pedersen_BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE](BULLETPROOF_DEFAULT_PEDERSEN_RAND_BASE)))
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
