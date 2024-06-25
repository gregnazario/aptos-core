
<a id="0x1_ristretto255_elgamal"></a>

# Module `0x1::ristretto255_elgamal`

This module implements an ElGamal encryption API, over the Ristretto255 curve, that can be used with the
Bulletproofs module.

An ElGamal &#42;ciphertext&#42; is an encryption of a value `v` under a basepoint `G` and public key `Y = sk * G`, where `sk`
is the corresponding secret key, is `(v * G + r * Y, r * G)`, for a random scalar `r`.

Note that we place the value `v` &quot;in the exponent&quot; of `G` so that ciphertexts are additively homomorphic: i.e., so
that `Enc_Y(v, r) + Enc_Y(v', r') = Enc_Y(v + v', r + r')` where `v, v'` are plaintext messages, `Y` is a public key and `r, r'`
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


```move
module 0x1::ristretto255_elgamal {
    use 0x1::option;
    use 0x1::ristretto255;
    use 0x1::vector;
}
```


<a id="0x1_ristretto255_elgamal_Ciphertext"></a>

## Struct `Ciphertext`

An ElGamal ciphertext.


```move
module 0x1::ristretto255_elgamal {
    struct Ciphertext has drop
}
```


##### Fields


<dl>
<dt>
`left: ristretto255::RistrettoPoint`
</dt>
<dd>

</dd>
<dt>
`right: ristretto255::RistrettoPoint`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_ristretto255_elgamal_CompressedCiphertext"></a>

## Struct `CompressedCiphertext`

A compressed ElGamal ciphertext.


```move
module 0x1::ristretto255_elgamal {
    struct CompressedCiphertext has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`left: ristretto255::CompressedRistretto`
</dt>
<dd>

</dd>
<dt>
`right: ristretto255::CompressedRistretto`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_ristretto255_elgamal_CompressedPubkey"></a>

## Struct `CompressedPubkey`

An ElGamal public key.


```move
module 0x1::ristretto255_elgamal {
    struct CompressedPubkey has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`point: ristretto255::CompressedRistretto`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_ristretto255_elgamal_new_pubkey_from_bytes"></a>

## Function `new_pubkey_from_bytes`

Creates a new public key from a serialized Ristretto255 point.


```move
module 0x1::ristretto255_elgamal {
    public fun new_pubkey_from_bytes(bytes: vector<u8>): option::Option<ristretto255_elgamal::CompressedPubkey>
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun new_pubkey_from_bytes(bytes: vector<u8>): Option<CompressedPubkey> {
        let point = ristretto255::new_compressed_point_from_bytes(bytes);
        if (std::option::is_some(&mut point)) {
            let pk = CompressedPubkey {
                point: std::option::extract(&mut point)
            };
            std::option::some(pk)
        } else {
            std::option::none<CompressedPubkey>()
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_pubkey_to_bytes"></a>

## Function `pubkey_to_bytes`

Given an ElGamal public key `pubkey`, returns the byte representation of that public key.


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_bytes(pubkey: &ristretto255_elgamal::CompressedPubkey): vector<u8>
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_bytes(pubkey: &CompressedPubkey): vector<u8> {
        ristretto255::compressed_point_to_bytes(pubkey.point)
    }
}
```


<a id="0x1_ristretto255_elgamal_pubkey_to_point"></a>

## Function `pubkey_to_point`

Given a public key `pubkey`, returns the underlying `RistrettoPoint` representing that key.


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_point(pubkey: &ristretto255_elgamal::CompressedPubkey): ristretto255::RistrettoPoint
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_point(pubkey: &CompressedPubkey): RistrettoPoint {
        ristretto255::point_decompress(&pubkey.point)
    }
}
```


<a id="0x1_ristretto255_elgamal_pubkey_to_compressed_point"></a>

## Function `pubkey_to_compressed_point`

Given a public key, returns the underlying `CompressedRistretto` point representing that key.


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_compressed_point(pubkey: &ristretto255_elgamal::CompressedPubkey): ristretto255::CompressedRistretto
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun pubkey_to_compressed_point(pubkey: &CompressedPubkey): CompressedRistretto {
        pubkey.point
    }
}
```


<a id="0x1_ristretto255_elgamal_new_ciphertext_from_bytes"></a>

## Function `new_ciphertext_from_bytes`

Creates a new ciphertext from two serialized Ristretto255 points: the first 32 bytes store `r * G` while the
next 32 bytes store `v * G + r * Y`, where `Y` is the public key.


```move
module 0x1::ristretto255_elgamal {
    public fun new_ciphertext_from_bytes(bytes: vector<u8>): option::Option<ristretto255_elgamal::Ciphertext>
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun new_ciphertext_from_bytes(bytes: vector<u8>): Option<Ciphertext> {
        if(vector::length(&bytes) != 64) {
            return std::option::none<Ciphertext>()
        };

        let bytes_right = vector::trim(&mut bytes, 32);

        let left_point = ristretto255::new_point_from_bytes(bytes);
        let right_point = ristretto255::new_point_from_bytes(bytes_right);

        if (std::option::is_some<RistrettoPoint>(&mut left_point) && std::option::is_some<RistrettoPoint>(&mut right_point)) {
            std::option::some<Ciphertext>(Ciphertext {
                left: std::option::extract<RistrettoPoint>(&mut left_point),
                right: std::option::extract<RistrettoPoint>(&mut right_point)
            })
        } else {
            std::option::none<Ciphertext>()
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_new_ciphertext_no_randomness"></a>

## Function `new_ciphertext_no_randomness`

Creates a new ciphertext `(val * G + 0 * Y, 0 * G) = (val * G, 0 * G)` where `G` is the Ristretto255 basepoint
and the randomness is set to zero.


```move
module 0x1::ristretto255_elgamal {
    public fun new_ciphertext_no_randomness(val: &ristretto255::Scalar): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun new_ciphertext_no_randomness(val: &Scalar): Ciphertext {
        Ciphertext {
            left: ristretto255::basepoint_mul(val),
            right: ristretto255::point_identity(),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_from_points"></a>

## Function `ciphertext_from_points`

Moves a pair of Ristretto points into an ElGamal ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_from_points(left: ristretto255::RistrettoPoint, right: ristretto255::RistrettoPoint): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_from_points(left: RistrettoPoint, right: RistrettoPoint): Ciphertext {
        Ciphertext {
            left,
            right,
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_from_compressed_points"></a>

## Function `ciphertext_from_compressed_points`

Moves a pair of `CompressedRistretto` points into an ElGamal ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_from_compressed_points(left: ristretto255::CompressedRistretto, right: ristretto255::CompressedRistretto): ristretto255_elgamal::CompressedCiphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_from_compressed_points(left: CompressedRistretto, right: CompressedRistretto): CompressedCiphertext {
        CompressedCiphertext {
            left,
            right,
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_to_bytes"></a>

## Function `ciphertext_to_bytes`

Given a ciphertext `ct`, serializes that ciphertext into bytes.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_to_bytes(ct: &ristretto255_elgamal::Ciphertext): vector<u8>
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_to_bytes(ct: &Ciphertext): vector<u8> {
        let bytes_left = ristretto255::point_to_bytes(&ristretto255::point_compress(&ct.left));
        let bytes_right = ristretto255::point_to_bytes(&ristretto255::point_compress(&ct.right));
        let bytes = vector::empty<u8>();
        vector::append<u8>(&mut bytes, bytes_left);
        vector::append<u8>(&mut bytes, bytes_right);
        bytes
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_into_points"></a>

## Function `ciphertext_into_points`

Moves the ciphertext into a pair of `RistrettoPoint`&apos;s.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_into_points(c: ristretto255_elgamal::Ciphertext): (ristretto255::RistrettoPoint, ristretto255::RistrettoPoint)
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_into_points(c: Ciphertext): (RistrettoPoint, RistrettoPoint) {
        let Ciphertext { left, right } = c;
        (left, right)
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_as_points"></a>

## Function `ciphertext_as_points`

Returns the pair of `RistrettoPoint`&apos;s representing the ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_as_points(c: &ristretto255_elgamal::Ciphertext): (&ristretto255::RistrettoPoint, &ristretto255::RistrettoPoint)
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_as_points(c: &Ciphertext): (&RistrettoPoint, &RistrettoPoint) {
        (&c.left, &c.right)
    }
}
```


<a id="0x1_ristretto255_elgamal_compress_ciphertext"></a>

## Function `compress_ciphertext`

Creates a new compressed ciphertext from a decompressed ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun compress_ciphertext(ct: &ristretto255_elgamal::Ciphertext): ristretto255_elgamal::CompressedCiphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun compress_ciphertext(ct: &Ciphertext): CompressedCiphertext {
        CompressedCiphertext {
            left: point_compress(&ct.left),
            right: point_compress(&ct.right),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_decompress_ciphertext"></a>

## Function `decompress_ciphertext`

Creates a new decompressed ciphertext from a compressed ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun decompress_ciphertext(ct: &ristretto255_elgamal::CompressedCiphertext): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun decompress_ciphertext(ct: &CompressedCiphertext): Ciphertext {
        Ciphertext {
            left: ristretto255::point_decompress(&ct.left),
            right: ristretto255::point_decompress(&ct.right),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_add"></a>

## Function `ciphertext_add`

Homomorphically combines two ciphertexts `lhs` and `rhs` as `lhs + rhs`.
Useful for re&#45;randomizing the ciphertext or updating the committed value.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_add(lhs: &ristretto255_elgamal::Ciphertext, rhs: &ristretto255_elgamal::Ciphertext): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_add(lhs: &Ciphertext, rhs: &Ciphertext): Ciphertext {
        Ciphertext {
            left: ristretto255::point_add(&lhs.left, &rhs.left),
            right: ristretto255::point_add(&lhs.right, &rhs.right),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_add_assign"></a>

## Function `ciphertext_add_assign`

Like `ciphertext_add` but assigns `lhs = lhs + rhs`.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_add_assign(lhs: &mut ristretto255_elgamal::Ciphertext, rhs: &ristretto255_elgamal::Ciphertext)
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_add_assign(lhs: &mut Ciphertext, rhs: &Ciphertext) {
        ristretto255::point_add_assign(&mut lhs.left, &rhs.left);
        ristretto255::point_add_assign(&mut lhs.right, &rhs.right);
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_sub"></a>

## Function `ciphertext_sub`

Homomorphically combines two ciphertexts `lhs` and `rhs` as `lhs - rhs`.
Useful for re&#45;randomizing the ciphertext or updating the committed value.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_sub(lhs: &ristretto255_elgamal::Ciphertext, rhs: &ristretto255_elgamal::Ciphertext): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_sub(lhs: &Ciphertext, rhs: &Ciphertext): Ciphertext {
        Ciphertext {
            left: ristretto255::point_sub(&lhs.left, &rhs.left),
            right: ristretto255::point_sub(&lhs.right, &rhs.right),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_sub_assign"></a>

## Function `ciphertext_sub_assign`

Like `ciphertext_add` but assigns `lhs = lhs - rhs`.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_sub_assign(lhs: &mut ristretto255_elgamal::Ciphertext, rhs: &ristretto255_elgamal::Ciphertext)
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_sub_assign(lhs: &mut Ciphertext, rhs: &Ciphertext) {
        ristretto255::point_sub_assign(&mut lhs.left, &rhs.left);
        ristretto255::point_sub_assign(&mut lhs.right, &rhs.right);
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_clone"></a>

## Function `ciphertext_clone`

Creates a copy of this ciphertext.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_clone(c: &ristretto255_elgamal::Ciphertext): ristretto255_elgamal::Ciphertext
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_clone(c: &Ciphertext): Ciphertext {
        Ciphertext {
            left: ristretto255::point_clone(&c.left),
            right: ristretto255::point_clone(&c.right),
        }
    }
}
```


<a id="0x1_ristretto255_elgamal_ciphertext_equals"></a>

## Function `ciphertext_equals`

Returns true if the two ciphertexts are identical: i.e., same value and same randomness.


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_equals(lhs: &ristretto255_elgamal::Ciphertext, rhs: &ristretto255_elgamal::Ciphertext): bool
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun ciphertext_equals(lhs: &Ciphertext, rhs: &Ciphertext): bool {
        ristretto255::point_equals(&lhs.left, &rhs.left) &&
        ristretto255::point_equals(&lhs.right, &rhs.right)
    }
}
```


<a id="0x1_ristretto255_elgamal_get_value_component"></a>

## Function `get_value_component`

Returns the `RistrettoPoint` in the ciphertext which contains the encrypted value in the exponent.


```move
module 0x1::ristretto255_elgamal {
    public fun get_value_component(ct: &ristretto255_elgamal::Ciphertext): &ristretto255::RistrettoPoint
}
```


##### Implementation


```move
module 0x1::ristretto255_elgamal {
    public fun get_value_component(ct: &Ciphertext): &RistrettoPoint {
        &ct.left
    }
}
```
