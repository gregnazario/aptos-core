
<a id="0x1_bit_vector"></a>

# Module `0x1::bit_vector`



-  [Struct `BitVector`](#0x1_bit_vector_BitVector)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_bit_vector_new)
-  [Function `set`](#0x1_bit_vector_set)
-  [Function `unset`](#0x1_bit_vector_unset)
-  [Function `shift_left`](#0x1_bit_vector_shift_left)
-  [Function `is_index_set`](#0x1_bit_vector_is_index_set)
-  [Function `length`](#0x1_bit_vector_length)
-  [Function `longest_set_sequence_starting_at`](#0x1_bit_vector_longest_set_sequence_starting_at)
-  [Function `shift_left_for_verification_only`](#0x1_bit_vector_shift_left_for_verification_only)
-  [Specification](#@Specification_1)
    -  [Struct `BitVector`](#@Specification_1_BitVector)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `set`](#@Specification_1_set)
    -  [Function `unset`](#@Specification_1_unset)
    -  [Function `shift_left`](#@Specification_1_shift_left)
    -  [Function `is_index_set`](#@Specification_1_is_index_set)
    -  [Function `longest_set_sequence_starting_at`](#@Specification_1_longest_set_sequence_starting_at)
    -  [Function `shift_left_for_verification_only`](#@Specification_1_shift_left_for_verification_only)


```move
module 0x1::bit_vector {
}
```


<a id="0x1_bit_vector_BitVector"></a>

## Struct `BitVector`



```move
module 0x1::bit_vector {
    struct BitVector has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`length: u64`
</dt>
<dd>

</dd>
<dt>
`bit_field: vector<bool>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_bit_vector_EINDEX"></a>

The provided index is out of bounds


```move
module 0x1::bit_vector {
    const EINDEX: u64 = 131072;
}
```


<a id="0x1_bit_vector_ELENGTH"></a>

An invalid length of bitvector was given


```move
module 0x1::bit_vector {
    const ELENGTH: u64 = 131073;
}
```


<a id="0x1_bit_vector_MAX_SIZE"></a>

The maximum allowed bitvector size


```move
module 0x1::bit_vector {
    const MAX_SIZE: u64 = 1024;
}
```


<a id="0x1_bit_vector_WORD_SIZE"></a>



```move
module 0x1::bit_vector {
    const WORD_SIZE: u64 = 1;
}
```


<a id="0x1_bit_vector_new"></a>

## Function `new`



```move
module 0x1::bit_vector {
    public fun new(length: u64): bit_vector::BitVector
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun new(length: u64): BitVector {
        assert!(length > 0, ELENGTH);
        assert!(length < MAX_SIZE, ELENGTH);
        let counter = 0;
        let bit_field = vector::empty();
        while ({spec {
            invariant counter <= length;
            invariant len(bit_field) == counter;
        };
            (counter < length)}) {
            vector::push_back(&mut bit_field, false);
            counter = counter + 1;
        };
        spec {
            assert counter == length;
            assert len(bit_field) == length;
        };

        BitVector {
            length,
            bit_field,
        }
    }
}
```


<a id="0x1_bit_vector_set"></a>

## Function `set`

Set the bit at `bit_index` in the `bitvector` regardless of its previous state.


```move
module 0x1::bit_vector {
    public fun set(bitvector: &mut bit_vector::BitVector, bit_index: u64)
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun set(bitvector: &mut BitVector, bit_index: u64) {
        assert!(bit_index < vector::length(&bitvector.bit_field), EINDEX);
        let x = vector::borrow_mut(&mut bitvector.bit_field, bit_index);
        *x = true;
    }
}
```


<a id="0x1_bit_vector_unset"></a>

## Function `unset`

Unset the bit at `bit_index` in the `bitvector` regardless of its previous state.


```move
module 0x1::bit_vector {
    public fun unset(bitvector: &mut bit_vector::BitVector, bit_index: u64)
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun unset(bitvector: &mut BitVector, bit_index: u64) {
        assert!(bit_index < vector::length(&bitvector.bit_field), EINDEX);
        let x = vector::borrow_mut(&mut bitvector.bit_field, bit_index);
        *x = false;
    }
}
```


<a id="0x1_bit_vector_shift_left"></a>

## Function `shift_left`

Shift the `bitvector` left by `amount`. If `amount` is greater than the
bitvector&apos;s length the bitvector will be zeroed out.


```move
module 0x1::bit_vector {
    public fun shift_left(bitvector: &mut bit_vector::BitVector, amount: u64)
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun shift_left(bitvector: &mut BitVector, amount: u64) {
        if (amount >= bitvector.length) {
            vector::for_each_mut(&mut bitvector.bit_field, |elem| {
                *elem = false;
            });
        } else {
            let i = amount;

            while (i < bitvector.length) {
                if (is_index_set(bitvector, i)) set(bitvector, i - amount)
                else unset(bitvector, i - amount);
                i = i + 1;
            };

            i = bitvector.length - amount;

            while (i < bitvector.length) {
                unset(bitvector, i);
                i = i + 1;
            };
        }
    }
}
```


<a id="0x1_bit_vector_is_index_set"></a>

## Function `is_index_set`

Return the value of the bit at `bit_index` in the `bitvector`. `true`
represents &quot;1&quot; and `false` represents a 0


```move
module 0x1::bit_vector {
    public fun is_index_set(bitvector: &bit_vector::BitVector, bit_index: u64): bool
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun is_index_set(bitvector: &BitVector, bit_index: u64): bool {
        assert!(bit_index < vector::length(&bitvector.bit_field), EINDEX);
        *vector::borrow(&bitvector.bit_field, bit_index)
    }
}
```


<a id="0x1_bit_vector_length"></a>

## Function `length`

Return the length (number of usable bits) of this bitvector


```move
module 0x1::bit_vector {
    public fun length(bitvector: &bit_vector::BitVector): u64
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun length(bitvector: &BitVector): u64 {
        vector::length(&bitvector.bit_field)
    }
}
```


<a id="0x1_bit_vector_longest_set_sequence_starting_at"></a>

## Function `longest_set_sequence_starting_at`

Returns the length of the longest sequence of set bits starting at (and
including) `start_index` in the `bitvector`. If there is no such
sequence, then `0` is returned.


```move
module 0x1::bit_vector {
    public fun longest_set_sequence_starting_at(bitvector: &bit_vector::BitVector, start_index: u64): u64
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun longest_set_sequence_starting_at(bitvector: &BitVector, start_index: u64): u64 {
        assert!(start_index < bitvector.length, EINDEX);
        let index = start_index;

        // Find the greatest index in the vector such that all indices less than it are set.
        while ({
            spec {
                invariant index >= start_index;
                invariant index == start_index || is_index_set(bitvector, index - 1);
                invariant index == start_index || index - 1 < vector::length(bitvector.bit_field);
                invariant forall j in start_index..index: is_index_set(bitvector, j);
                invariant forall j in start_index..index: j < vector::length(bitvector.bit_field);
            };
            index < bitvector.length
        }) {
            if (!is_index_set(bitvector, index)) break;
            index = index + 1;
        };

        index - start_index
    }
}
```


<a id="0x1_bit_vector_shift_left_for_verification_only"></a>

## Function `shift_left_for_verification_only`



```move
module 0x1::bit_vector {
    #[verify_only]
    public fun shift_left_for_verification_only(bitvector: &mut bit_vector::BitVector, amount: u64)
}
```


##### Implementation


```move
module 0x1::bit_vector {
    public fun shift_left_for_verification_only(bitvector: &mut BitVector, amount: u64) {
        if (amount >= bitvector.length) {
            let len = vector::length(&bitvector.bit_field);
            let i = 0;
            while ({
                spec {
                    invariant len == bitvector.length;
                    invariant forall k in 0..i: !bitvector.bit_field[k];
                    invariant forall k in i..bitvector.length: bitvector.bit_field[k] == old(bitvector).bit_field[k];
                };
                i < len
            }) {
                let elem = vector::borrow_mut(&mut bitvector.bit_field, i);
                *elem = false;
                i = i + 1;
            };
        } else {
            let i = amount;

            while ({
                spec {
                    invariant i >= amount;
                    invariant bitvector.length == old(bitvector).length;
                    invariant forall j in amount..i: old(bitvector).bit_field[j] == bitvector.bit_field[j - amount];
                    invariant forall j in (i-amount)..bitvector.length : old(bitvector).bit_field[j] == bitvector.bit_field[j];
                    invariant forall k in 0..i-amount: bitvector.bit_field[k] == old(bitvector).bit_field[k + amount];
                };
                i < bitvector.length
            }) {
                if (is_index_set(bitvector, i)) set(bitvector, i - amount)
                else unset(bitvector, i - amount);
                i = i + 1;
            };


            i = bitvector.length - amount;

            while ({
                spec {
                    invariant forall j in bitvector.length - amount..i: !bitvector.bit_field[j];
                    invariant forall k in 0..bitvector.length - amount: bitvector.bit_field[k] == old(bitvector).bit_field[k + amount];
                    invariant i >= bitvector.length - amount;
                };
                i < bitvector.length
            }) {
                unset(bitvector, i);
                i = i + 1;
            }
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_BitVector"></a>

### Struct `BitVector`


```move
module 0x1::bit_vector {
    struct BitVector has copy, drop, store
}
```


<dl>
<dt>
`length: u64`
</dt>
<dd>

</dd>
<dt>
`bit_field: vector<bool>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::bit_vector {
    invariant length == len(bit_field);
}
```


<a id="@Specification_1_new"></a>

### Function `new`


```move
module 0x1::bit_vector {
    public fun new(length: u64): bit_vector::BitVector
}
```



```move
module 0x1::bit_vector {
    include NewAbortsIf;
    ensures result.length == length;
    ensures len(result.bit_field) == length;
}
```



<a id="0x1_bit_vector_NewAbortsIf"></a>


```move
module 0x1::bit_vector {
    schema NewAbortsIf {
        length: u64;
        aborts_if length <= 0 with ELENGTH;
        aborts_if length >= MAX_SIZE with ELENGTH;
    }
}
```


<a id="@Specification_1_set"></a>

### Function `set`


```move
module 0x1::bit_vector {
    public fun set(bitvector: &mut bit_vector::BitVector, bit_index: u64)
}
```



```move
module 0x1::bit_vector {
    include SetAbortsIf;
    ensures bitvector.bit_field[bit_index];
}
```



<a id="0x1_bit_vector_SetAbortsIf"></a>


```move
module 0x1::bit_vector {
    schema SetAbortsIf {
        bitvector: BitVector;
        bit_index: u64;
        aborts_if bit_index >= length(bitvector) with EINDEX;
    }
}
```


<a id="@Specification_1_unset"></a>

### Function `unset`


```move
module 0x1::bit_vector {
    public fun unset(bitvector: &mut bit_vector::BitVector, bit_index: u64)
}
```



```move
module 0x1::bit_vector {
    include UnsetAbortsIf;
    ensures !bitvector.bit_field[bit_index];
}
```



<a id="0x1_bit_vector_UnsetAbortsIf"></a>


```move
module 0x1::bit_vector {
    schema UnsetAbortsIf {
        bitvector: BitVector;
        bit_index: u64;
        aborts_if bit_index >= length(bitvector) with EINDEX;
    }
}
```


<a id="@Specification_1_shift_left"></a>

### Function `shift_left`


```move
module 0x1::bit_vector {
    public fun shift_left(bitvector: &mut bit_vector::BitVector, amount: u64)
}
```



```move
module 0x1::bit_vector {
    pragma verify = false;
}
```


<a id="@Specification_1_is_index_set"></a>

### Function `is_index_set`


```move
module 0x1::bit_vector {
    public fun is_index_set(bitvector: &bit_vector::BitVector, bit_index: u64): bool
}
```



```move
module 0x1::bit_vector {
    include IsIndexSetAbortsIf;
    ensures result == bitvector.bit_field[bit_index];
}
```



<a id="0x1_bit_vector_IsIndexSetAbortsIf"></a>


```move
module 0x1::bit_vector {
    schema IsIndexSetAbortsIf {
        bitvector: BitVector;
        bit_index: u64;
        aborts_if bit_index >= length(bitvector) with EINDEX;
    }
}
```



<a id="0x1_bit_vector_spec_is_index_set"></a>


```move
module 0x1::bit_vector {
    fun spec_is_index_set(bitvector: BitVector, bit_index: u64): bool {
       if (bit_index >= length(bitvector)) {
           false
       } else {
           bitvector.bit_field[bit_index]
       }
    }
}
```


<a id="@Specification_1_longest_set_sequence_starting_at"></a>

### Function `longest_set_sequence_starting_at`


```move
module 0x1::bit_vector {
    public fun longest_set_sequence_starting_at(bitvector: &bit_vector::BitVector, start_index: u64): u64
}
```



```move
module 0x1::bit_vector {
    aborts_if start_index >= bitvector.length;
    ensures forall i in start_index..result: is_index_set(bitvector, i);
}
```


<a id="@Specification_1_shift_left_for_verification_only"></a>

### Function `shift_left_for_verification_only`


```move
module 0x1::bit_vector {
    #[verify_only]
    public fun shift_left_for_verification_only(bitvector: &mut bit_vector::BitVector, amount: u64)
}
```



```move
module 0x1::bit_vector {
    aborts_if false;
    ensures amount >= bitvector.length ==> (forall k in 0..bitvector.length: !bitvector.bit_field[k]);
    ensures amount < bitvector.length ==>
        (forall i in bitvector.length - amount..bitvector.length: !bitvector.bit_field[i]);
    ensures amount < bitvector.length ==>
        (forall i in 0..bitvector.length - amount: bitvector.bit_field[i] == old(bitvector).bit_field[i + amount]);
}
```
