
<a id="0x1_smart_vector"></a>

# Module `0x1::smart_vector`



-  [Struct `SmartVector`](#0x1_smart_vector_SmartVector)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_smart_vector_new)
-  [Function `empty`](#0x1_smart_vector_empty)
-  [Function `empty_with_config`](#0x1_smart_vector_empty_with_config)
-  [Function `singleton`](#0x1_smart_vector_singleton)
-  [Function `destroy_empty`](#0x1_smart_vector_destroy_empty)
-  [Function `destroy`](#0x1_smart_vector_destroy)
-  [Function `clear`](#0x1_smart_vector_clear)
-  [Function `borrow`](#0x1_smart_vector_borrow)
-  [Function `borrow_mut`](#0x1_smart_vector_borrow_mut)
-  [Function `append`](#0x1_smart_vector_append)
-  [Function `add_all`](#0x1_smart_vector_add_all)
-  [Function `to_vector`](#0x1_smart_vector_to_vector)
-  [Function `push_back`](#0x1_smart_vector_push_back)
-  [Function `pop_back`](#0x1_smart_vector_pop_back)
-  [Function `remove`](#0x1_smart_vector_remove)
-  [Function `swap_remove`](#0x1_smart_vector_swap_remove)
-  [Function `swap`](#0x1_smart_vector_swap)
-  [Function `reverse`](#0x1_smart_vector_reverse)
-  [Function `index_of`](#0x1_smart_vector_index_of)
-  [Function `contains`](#0x1_smart_vector_contains)
-  [Function `length`](#0x1_smart_vector_length)
-  [Function `is_empty`](#0x1_smart_vector_is_empty)
-  [Function `for_each`](#0x1_smart_vector_for_each)
-  [Function `for_each_reverse`](#0x1_smart_vector_for_each_reverse)
-  [Function `for_each_ref`](#0x1_smart_vector_for_each_ref)
-  [Function `for_each_mut`](#0x1_smart_vector_for_each_mut)
-  [Function `enumerate_ref`](#0x1_smart_vector_enumerate_ref)
-  [Function `enumerate_mut`](#0x1_smart_vector_enumerate_mut)
-  [Function `fold`](#0x1_smart_vector_fold)
-  [Function `foldr`](#0x1_smart_vector_foldr)
-  [Function `map_ref`](#0x1_smart_vector_map_ref)
-  [Function `map`](#0x1_smart_vector_map)
-  [Function `filter`](#0x1_smart_vector_filter)
-  [Function `zip`](#0x1_smart_vector_zip)
-  [Function `zip_reverse`](#0x1_smart_vector_zip_reverse)
-  [Function `zip_ref`](#0x1_smart_vector_zip_ref)
-  [Function `zip_mut`](#0x1_smart_vector_zip_mut)
-  [Function `zip_map`](#0x1_smart_vector_zip_map)
-  [Function `zip_map_ref`](#0x1_smart_vector_zip_map_ref)
-  [Specification](#@Specification_1)
    -  [Struct `SmartVector`](#@Specification_1_SmartVector)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `empty_with_config`](#@Specification_1_empty_with_config)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `push_back`](#@Specification_1_push_back)
    -  [Function `pop_back`](#@Specification_1_pop_back)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `swap`](#@Specification_1_swap)
    -  [Function `length`](#@Specification_1_length)


```move
module 0x1::smart_vector {
    use 0x1::big_vector;
    use 0x1::error;
    use 0x1::math64;
    use 0x1::option;
    use 0x1::type_info;
    use 0x1::vector;
}
```


<a id="0x1_smart_vector_SmartVector"></a>

## Struct `SmartVector`

A Scalable vector implementation based on tables, Ts are grouped into buckets with `bucket_size`.
The option wrapping BigVector saves space in the metadata associated with BigVector when smart_vector is
so small that inline_vec vector can hold all the data.


```move
module 0x1::smart_vector {
    struct SmartVector<T> has store
}
```


##### Fields


<dl>
<dt>
`inline_vec: vector<T>`
</dt>
<dd>

</dd>
<dt>
`big_vec: option::Option<big_vector::BigVector<T>>`
</dt>
<dd>

</dd>
<dt>
`inline_capacity: option::Option<u64>`
</dt>
<dd>

</dd>
<dt>
`bucket_size: option::Option<u64>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_smart_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


```move
module 0x1::smart_vector {
    const EINDEX_OUT_OF_BOUNDS: u64 = 1;
}
```


<a id="0x1_smart_vector_EVECTOR_EMPTY"></a>

Cannot pop back from an empty vector


```move
module 0x1::smart_vector {
    const EVECTOR_EMPTY: u64 = 3;
}
```


<a id="0x1_smart_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non&#45;empty vector


```move
module 0x1::smart_vector {
    const EVECTOR_NOT_EMPTY: u64 = 2;
}
```


<a id="0x1_smart_vector_EZERO_BUCKET_SIZE"></a>

bucket_size cannot be 0


```move
module 0x1::smart_vector {
    const EZERO_BUCKET_SIZE: u64 = 4;
}
```


<a id="0x1_smart_vector_ESMART_VECTORS_LENGTH_MISMATCH"></a>

The length of the smart vectors are not equal.


```move
module 0x1::smart_vector {
    const ESMART_VECTORS_LENGTH_MISMATCH: u64 = 131077;
}
```


<a id="0x1_smart_vector_new"></a>

## Function `new`

Regular Vector API
Create an empty vector using default logic to estimate `inline_capacity` and `bucket_size`, which may be
inaccurate.
This is exactly the same as empty() but is more standardized as all other data structures have new().


```move
module 0x1::smart_vector {
    public fun new<T: store>(): smart_vector::SmartVector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun new<T: store>(): SmartVector<T> {
        empty()
    }
}
```


<a id="0x1_smart_vector_empty"></a>

## Function `empty`

Create an empty vector using default logic to estimate `inline_capacity` and `bucket_size`, which may be
inaccurate.


```move
module 0x1::smart_vector {
    #[deprecated]
    public fun empty<T: store>(): smart_vector::SmartVector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun empty<T: store>(): SmartVector<T> {
        SmartVector {
            inline_vec: vector[],
            big_vec: option::none(),
            inline_capacity: option::none(),
            bucket_size: option::none(),
        }
    }
}
```


<a id="0x1_smart_vector_empty_with_config"></a>

## Function `empty_with_config`

Create an empty vector with customized config.
When inline_capacity &#61; 0, SmartVector degrades to a wrapper of BigVector.


```move
module 0x1::smart_vector {
    public fun empty_with_config<T: store>(inline_capacity: u64, bucket_size: u64): smart_vector::SmartVector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun empty_with_config<T: store>(inline_capacity: u64, bucket_size: u64): SmartVector<T> {
        assert!(bucket_size > 0, error::invalid_argument(EZERO_BUCKET_SIZE));
        SmartVector {
            inline_vec: vector[],
            big_vec: option::none(),
            inline_capacity: option::some(inline_capacity),
            bucket_size: option::some(bucket_size),
        }
    }
}
```


<a id="0x1_smart_vector_singleton"></a>

## Function `singleton`

Create a vector of length 1 containing the passed in T.


```move
module 0x1::smart_vector {
    public fun singleton<T: store>(element: T): smart_vector::SmartVector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun singleton<T: store>(element: T): SmartVector<T> {
        let v = empty();
        push_back(&mut v, element);
        v
    }
}
```


<a id="0x1_smart_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector `v`.
Aborts if `v` is not empty.


```move
module 0x1::smart_vector {
    public fun destroy_empty<T>(v: smart_vector::SmartVector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun destroy_empty<T>(v: SmartVector<T>) {
        assert!(is_empty(&v), error::invalid_argument(EVECTOR_NOT_EMPTY));
        let SmartVector { inline_vec, big_vec, inline_capacity: _, bucket_size: _ } = v;
        vector::destroy_empty(inline_vec);
        option::destroy_none(big_vec);
    }
}
```


<a id="0x1_smart_vector_destroy"></a>

## Function `destroy`

Destroy a vector completely when T has `drop`.


```move
module 0x1::smart_vector {
    public fun destroy<T: drop>(v: smart_vector::SmartVector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun destroy<T: drop>(v: SmartVector<T>) {
        clear(&mut v);
        destroy_empty(v);
    }
}
```


<a id="0x1_smart_vector_clear"></a>

## Function `clear`

Clear a vector completely when T has `drop`.


```move
module 0x1::smart_vector {
    public fun clear<T: drop>(v: &mut smart_vector::SmartVector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun clear<T: drop>(v: &mut SmartVector<T>) {
        v.inline_vec = vector[];
        if (option::is_some(&v.big_vec)) {
            big_vector::destroy(option::extract(&mut v.big_vec));
        }
    }
}
```


<a id="0x1_smart_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the `i`th T of the vector `v`.
Aborts if `i` is out of bounds.


```move
module 0x1::smart_vector {
    public fun borrow<T>(v: &smart_vector::SmartVector<T>, i: u64): &T
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun borrow<T>(v: &SmartVector<T>, i: u64): &T {
        assert!(i < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let inline_len = vector::length(&v.inline_vec);
        if (i < inline_len) {
            vector::borrow(&v.inline_vec, i)
        } else {
            big_vector::borrow(option::borrow(&v.big_vec), i - inline_len)
        }
    }
}
```


<a id="0x1_smart_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the `i`th T in the vector `v`.
Aborts if `i` is out of bounds.


```move
module 0x1::smart_vector {
    public fun borrow_mut<T>(v: &mut smart_vector::SmartVector<T>, i: u64): &mut T
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun borrow_mut<T>(v: &mut SmartVector<T>, i: u64): &mut T {
        assert!(i < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let inline_len = vector::length(&v.inline_vec);
        if (i < inline_len) {
            vector::borrow_mut(&mut v.inline_vec, i)
        } else {
            big_vector::borrow_mut(option::borrow_mut(&mut v.big_vec), i - inline_len)
        }
    }
}
```


<a id="0x1_smart_vector_append"></a>

## Function `append`

Empty and destroy the other vector, and push each of the Ts in the other vector onto the lhs vector in the
same order as they occurred in other.
Disclaimer: This function may be costly. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun append<T: store>(lhs: &mut smart_vector::SmartVector<T>, other: smart_vector::SmartVector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun append<T: store>(lhs: &mut SmartVector<T>, other: SmartVector<T>) {
        let other_len = length(&other);
        let half_other_len = other_len / 2;
        let i = 0;
        while (i < half_other_len) {
            push_back(lhs, swap_remove(&mut other, i));
            i = i + 1;
        };
        while (i < other_len) {
            push_back(lhs, pop_back(&mut other));
            i = i + 1;
        };
        destroy_empty(other);
    }
}
```


<a id="0x1_smart_vector_add_all"></a>

## Function `add_all`

Add multiple values to the vector at once.


```move
module 0x1::smart_vector {
    public fun add_all<T: store>(v: &mut smart_vector::SmartVector<T>, vals: vector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun add_all<T: store>(v: &mut SmartVector<T>, vals: vector<T>) {
        vector::for_each(vals, |val| { push_back(v, val); })
    }
}
```


<a id="0x1_smart_vector_to_vector"></a>

## Function `to_vector`

Convert a smart vector to a native vector, which is supposed to be called mostly by view functions to get an
atomic view of the whole vector.
Disclaimer: This function may be costly as the smart vector may be huge in size. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun to_vector<T: copy, store>(v: &smart_vector::SmartVector<T>): vector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun to_vector<T: store + copy>(v: &SmartVector<T>): vector<T> {
        let res = v.inline_vec;
        if (option::is_some(&v.big_vec)) {
            let big_vec = option::borrow(&v.big_vec);
            vector::append(&mut res, big_vector::to_vector(big_vec));
        };
        res
    }
}
```


<a id="0x1_smart_vector_push_back"></a>

## Function `push_back`

Add T `val` to the end of the vector `v`. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


```move
module 0x1::smart_vector {
    public fun push_back<T: store>(v: &mut smart_vector::SmartVector<T>, val: T)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun push_back<T: store>(v: &mut SmartVector<T>, val: T) {
        let len = length(v);
        let inline_len = vector::length(&v.inline_vec);
        if (len == inline_len) {
            let bucket_size = if (option::is_some(&v.inline_capacity)) {
                if (len < *option::borrow(&v.inline_capacity)) {
                    vector::push_back(&mut v.inline_vec, val);
                    return
                };
                *option::borrow(&v.bucket_size)
            } else {
                let val_size = size_of_val(&val);
                if (val_size * (inline_len + 1) < 150 /* magic number */) {
                    vector::push_back(&mut v.inline_vec, val);
                    return
                };
                let estimated_avg_size = max((size_of_val(&v.inline_vec) + val_size) / (inline_len + 1), 1);
                max(1024 /* free_write_quota */ / estimated_avg_size, 1)
            };
            option::fill(&mut v.big_vec, big_vector::empty(bucket_size));
        };
        big_vector::push_back(option::borrow_mut(&mut v.big_vec), val);
    }
}
```


<a id="0x1_smart_vector_pop_back"></a>

## Function `pop_back`

Pop an T from the end of vector `v`. It does shrink the buckets if they&apos;re empty.
Aborts if `v` is empty.


```move
module 0x1::smart_vector {
    public fun pop_back<T>(v: &mut smart_vector::SmartVector<T>): T
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun pop_back<T>(v: &mut SmartVector<T>): T {
        assert!(!is_empty(v), error::invalid_state(EVECTOR_EMPTY));
        let big_vec_wrapper = &mut v.big_vec;
        if (option::is_some(big_vec_wrapper)) {
            let big_vec = option::extract(big_vec_wrapper);
            let val = big_vector::pop_back(&mut big_vec);
            if (big_vector::is_empty(&big_vec)) {
                big_vector::destroy_empty(big_vec)
            } else {
                option::fill(big_vec_wrapper, big_vec);
            };
            val
        } else {
            vector::pop_back(&mut v.inline_vec)
        }
    }
}
```


<a id="0x1_smart_vector_remove"></a>

## Function `remove`

Remove the T at index i in the vector v and return the owned value that was previously stored at i in v.
All Ts occurring at indices greater than i will be shifted down by 1. Will abort if i is out of bounds.
Disclaimer: This function may be costly. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun remove<T>(v: &mut smart_vector::SmartVector<T>, i: u64): T
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun remove<T>(v: &mut SmartVector<T>, i: u64): T {
        let len = length(v);
        assert!(i < len, error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let inline_len = vector::length(&v.inline_vec);
        if (i < inline_len) {
            vector::remove(&mut v.inline_vec, i)
        } else {
            let big_vec_wrapper = &mut v.big_vec;
            let big_vec = option::extract(big_vec_wrapper);
            let val = big_vector::remove(&mut big_vec, i - inline_len);
            if (big_vector::is_empty(&big_vec)) {
                big_vector::destroy_empty(big_vec)
            } else {
                option::fill(big_vec_wrapper, big_vec);
            };
            val
        }
    }
}
```


<a id="0x1_smart_vector_swap_remove"></a>

## Function `swap_remove`

Swap the `i`th T of the vector `v` with the last T and then pop the vector.
This is O(1), but does not preserve ordering of Ts in the vector.
Aborts if `i` is out of bounds.


```move
module 0x1::smart_vector {
    public fun swap_remove<T>(v: &mut smart_vector::SmartVector<T>, i: u64): T
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun swap_remove<T>(v: &mut SmartVector<T>, i: u64): T {
        let len = length(v);
        assert!(i < len, error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let inline_len = vector::length(&v.inline_vec);
        let big_vec_wrapper = &mut v.big_vec;
        let inline_vec = &mut v.inline_vec;
        if (i >= inline_len) {
            let big_vec = option::extract(big_vec_wrapper);
            let val = big_vector::swap_remove(&mut big_vec, i - inline_len);
            if (big_vector::is_empty(&big_vec)) {
                big_vector::destroy_empty(big_vec)
            } else {
                option::fill(big_vec_wrapper, big_vec);
            };
            val
        } else {
            if (inline_len < len) {
                let big_vec = option::extract(big_vec_wrapper);
                let last_from_big_vec = big_vector::pop_back(&mut big_vec);
                if (big_vector::is_empty(&big_vec)) {
                    big_vector::destroy_empty(big_vec)
                } else {
                    option::fill(big_vec_wrapper, big_vec);
                };
                vector::push_back(inline_vec, last_from_big_vec);
            };
            vector::swap_remove(inline_vec, i)
        }
    }
}
```


<a id="0x1_smart_vector_swap"></a>

## Function `swap`

Swap the Ts at the i&apos;th and j&apos;th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


```move
module 0x1::smart_vector {
    public fun swap<T: store>(v: &mut smart_vector::SmartVector<T>, i: u64, j: u64)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun swap<T: store>(v: &mut SmartVector<T>, i: u64, j: u64) {
        if (i > j) {
            return swap(v, j, i)
        };
        let len = length(v);
        assert!(j < len, error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let inline_len = vector::length(&v.inline_vec);
        if (i >= inline_len) {
            big_vector::swap(option::borrow_mut(&mut v.big_vec), i - inline_len, j - inline_len);
        } else if (j < inline_len) {
            vector::swap(&mut v.inline_vec, i, j);
        } else {
            let big_vec = option::borrow_mut(&mut v.big_vec);
            let inline_vec = &mut v.inline_vec;
            let element_i = vector::swap_remove(inline_vec, i);
            let element_j = big_vector::swap_remove(big_vec, j - inline_len);
            vector::push_back(inline_vec, element_j);
            vector::swap(inline_vec, i, inline_len - 1);
            big_vector::push_back(big_vec, element_i);
            big_vector::swap(big_vec, j - inline_len, len - inline_len - 1);
        }
    }
}
```


<a id="0x1_smart_vector_reverse"></a>

## Function `reverse`

Reverse the order of the Ts in the vector v in&#45;place.
Disclaimer: This function may be costly. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun reverse<T: store>(v: &mut smart_vector::SmartVector<T>)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun reverse<T: store>(v: &mut SmartVector<T>) {
        let inline_len = vector::length(&v.inline_vec);
        let i = 0;
        let new_inline_vec = vector[];
        // Push the last `inline_len` Ts into a temp vector.
        while (i < inline_len) {
            vector::push_back(&mut new_inline_vec, pop_back(v));
            i = i + 1;
        };
        vector::reverse(&mut new_inline_vec);
        // Reverse the big_vector left if exists.
        if (option::is_some(&v.big_vec)) {
            big_vector::reverse(option::borrow_mut(&mut v.big_vec));
        };
        // Mem::swap the two vectors.
        let temp_vec = vector[];
        while (!vector::is_empty(&mut v.inline_vec)) {
            vector::push_back(&mut temp_vec, vector::pop_back(&mut v.inline_vec));
        };
        vector::reverse(&mut temp_vec);
        while (!vector::is_empty(&mut new_inline_vec)) {
            vector::push_back(&mut v.inline_vec, vector::pop_back(&mut new_inline_vec));
        };
        vector::destroy_empty(new_inline_vec);
        // Push the rest Ts originally left in inline_vector back to the end of the smart vector.
        while (!vector::is_empty(&mut temp_vec)) {
            push_back(v, vector::pop_back(&mut temp_vec));
        };
        vector::destroy_empty(temp_vec);
    }
}
```


<a id="0x1_smart_vector_index_of"></a>

## Function `index_of`

Return `(true, i)` if `val` is in the vector `v` at index `i`.
Otherwise, returns `(false, 0)`.
Disclaimer: This function may be costly. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun index_of<T>(v: &smart_vector::SmartVector<T>, val: &T): (bool, u64)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun index_of<T>(v: &SmartVector<T>, val: &T): (bool, u64) {
        let (found, i) = vector::index_of(&v.inline_vec, val);
        if (found) {
            (true, i)
        } else if (option::is_some(&v.big_vec)) {
            let (found, i) = big_vector::index_of(option::borrow(&v.big_vec), val);
            (found, i + vector::length(&v.inline_vec))
        } else {
            (false, 0)
        }
    }
}
```


<a id="0x1_smart_vector_contains"></a>

## Function `contains`

Return true if `val` is in the vector `v`.
Disclaimer: This function may be costly. Use it at your own discretion.


```move
module 0x1::smart_vector {
    public fun contains<T>(v: &smart_vector::SmartVector<T>, val: &T): bool
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun contains<T>(v: &SmartVector<T>, val: &T): bool {
        if (is_empty(v)) return false;
        let (exist, _) = index_of(v, val);
        exist
    }
}
```


<a id="0x1_smart_vector_length"></a>

## Function `length`

Return the length of the vector.


```move
module 0x1::smart_vector {
    public fun length<T>(v: &smart_vector::SmartVector<T>): u64
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun length<T>(v: &SmartVector<T>): u64 {
        vector::length(&v.inline_vec) + if (option::is_none(&v.big_vec)) {
            0
        } else {
            big_vector::length(option::borrow(&v.big_vec))
        }
    }
}
```


<a id="0x1_smart_vector_is_empty"></a>

## Function `is_empty`

Return `true` if the vector `v` has no Ts and `false` otherwise.


```move
module 0x1::smart_vector {
    public fun is_empty<T>(v: &smart_vector::SmartVector<T>): bool
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public fun is_empty<T>(v: &SmartVector<T>): bool {
        length(v) == 0
    }
}
```


<a id="0x1_smart_vector_for_each"></a>

## Function `for_each`

Apply the function to each T in the vector, consuming it.


```move
module 0x1::smart_vector {
    public fun for_each<T: store>(v: smart_vector::SmartVector<T>, f: |T|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun for_each<T: store>(v: SmartVector<T>, f: |T|) {
        aptos_std::smart_vector::reverse(&mut v); // We need to reverse the vector to consume it efficiently
        aptos_std::smart_vector::for_each_reverse(v, |e| f(e));
    }
}
```


<a id="0x1_smart_vector_for_each_reverse"></a>

## Function `for_each_reverse`

Apply the function to each T in the vector, consuming it.


```move
module 0x1::smart_vector {
    public fun for_each_reverse<T>(v: smart_vector::SmartVector<T>, f: |T|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun for_each_reverse<T>(v: SmartVector<T>, f: |T|) {
        let len = aptos_std::smart_vector::length(&v);
        while (len > 0) {
            f(aptos_std::smart_vector::pop_back(&mut v));
            len = len - 1;
        };
        aptos_std::smart_vector::destroy_empty(v)
    }
}
```


<a id="0x1_smart_vector_for_each_ref"></a>

## Function `for_each_ref`

Apply the function to a reference of each T in the vector.


```move
module 0x1::smart_vector {
    public fun for_each_ref<T>(v: &smart_vector::SmartVector<T>, f: |&T|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun for_each_ref<T>(v: &SmartVector<T>, f: |&T|) {
        let i = 0;
        let len = aptos_std::smart_vector::length(v);
        while (i < len) {
            f(aptos_std::smart_vector::borrow(v, i));
            i = i + 1
        }
    }
}
```


<a id="0x1_smart_vector_for_each_mut"></a>

## Function `for_each_mut`

Apply the function to a mutable reference to each T in the vector.


```move
module 0x1::smart_vector {
    public fun for_each_mut<T>(v: &mut smart_vector::SmartVector<T>, f: |&mut T|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun for_each_mut<T>(v: &mut SmartVector<T>, f: |&mut T|) {
        let i = 0;
        let len = aptos_std::smart_vector::length(v);
        while (i < len) {
            f(aptos_std::smart_vector::borrow_mut(v, i));
            i = i + 1
        }
    }
}
```


<a id="0x1_smart_vector_enumerate_ref"></a>

## Function `enumerate_ref`

Apply the function to a reference of each T in the vector with its index.


```move
module 0x1::smart_vector {
    public fun enumerate_ref<T>(v: &smart_vector::SmartVector<T>, f: |(u64, &T)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun enumerate_ref<T>(v: &SmartVector<T>, f: |u64, &T|) {
        let i = 0;
        let len = aptos_std::smart_vector::length(v);
        while (i < len) {
            f(i, aptos_std::smart_vector::borrow(v, i));
            i = i + 1;
        };
    }
}
```


<a id="0x1_smart_vector_enumerate_mut"></a>

## Function `enumerate_mut`

Apply the function to a mutable reference of each T in the vector with its index.


```move
module 0x1::smart_vector {
    public fun enumerate_mut<T>(v: &mut smart_vector::SmartVector<T>, f: |(u64, &mut T)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun enumerate_mut<T>(v: &mut SmartVector<T>, f: |u64, &mut T|) {
        let i = 0;
        let len = length(v);
        while (i < len) {
            f(i, borrow_mut(v, i));
            i = i + 1;
        };
    }
}
```


<a id="0x1_smart_vector_fold"></a>

## Function `fold`

Fold the function over the Ts. For example, `fold(vector[1,2,3], 0, f)` will execute
`f(f(f(0, 1), 2), 3)`


```move
module 0x1::smart_vector {
    public fun fold<Accumulator, T: store>(v: smart_vector::SmartVector<T>, init: Accumulator, f: |(Accumulator, T)|Accumulator): Accumulator
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun fold<Accumulator, T: store>(
        v: SmartVector<T>,
        init: Accumulator,
        f: |Accumulator, T|Accumulator
    ): Accumulator {
        let accu = init;
        aptos_std::smart_vector::for_each(v, |elem| accu = f(accu, elem));
        accu
    }
}
```


<a id="0x1_smart_vector_foldr"></a>

## Function `foldr`

Fold right like fold above but working right to left. For example, `fold(vector[1,2,3], 0, f)` will execute
`f(1, f(2, f(3, 0)))`


```move
module 0x1::smart_vector {
    public fun foldr<Accumulator, T>(v: smart_vector::SmartVector<T>, init: Accumulator, f: |(T, Accumulator)|Accumulator): Accumulator
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun foldr<Accumulator, T>(
        v: SmartVector<T>,
        init: Accumulator,
        f: |T, Accumulator|Accumulator
    ): Accumulator {
        let accu = init;
        aptos_std::smart_vector::for_each_reverse(v, |elem| accu = f(elem, accu));
        accu
    }
}
```


<a id="0x1_smart_vector_map_ref"></a>

## Function `map_ref`

Map the function over the references of the Ts of the vector, producing a new vector without modifying the
original vector.


```move
module 0x1::smart_vector {
    public fun map_ref<T1, T2: store>(v: &smart_vector::SmartVector<T1>, f: |&T1|T2): smart_vector::SmartVector<T2>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun map_ref<T1, T2: store>(
        v: &SmartVector<T1>,
        f: |&T1|T2
    ): SmartVector<T2> {
        let result = aptos_std::smart_vector::new<T2>();
        aptos_std::smart_vector::for_each_ref(v, |elem| aptos_std::smart_vector::push_back(&mut result, f(elem)));
        result
    }
}
```


<a id="0x1_smart_vector_map"></a>

## Function `map`

Map the function over the Ts of the vector, producing a new vector.


```move
module 0x1::smart_vector {
    public fun map<T1: store, T2: store>(v: smart_vector::SmartVector<T1>, f: |T1|T2): smart_vector::SmartVector<T2>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun map<T1: store, T2: store>(
        v: SmartVector<T1>,
        f: |T1|T2
    ): SmartVector<T2> {
        let result = aptos_std::smart_vector::new<T2>();
        aptos_std::smart_vector::for_each(v, |elem| push_back(&mut result, f(elem)));
        result
    }
}
```


<a id="0x1_smart_vector_filter"></a>

## Function `filter`

Filter the vector using the boolean function, removing all Ts for which `p(e)` is not true.


```move
module 0x1::smart_vector {
    public fun filter<T: drop, store>(v: smart_vector::SmartVector<T>, p: |&T|bool): smart_vector::SmartVector<T>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun filter<T: store + drop>(
        v: SmartVector<T>,
        p: |&T|bool
    ): SmartVector<T> {
        let result = aptos_std::smart_vector::new<T>();
        aptos_std::smart_vector::for_each(v, |elem| {
            if (p(&elem)) aptos_std::smart_vector::push_back(&mut result, elem);
        });
        result
    }
}
```


<a id="0x1_smart_vector_zip"></a>

## Function `zip`



```move
module 0x1::smart_vector {
    public fun zip<T1: store, T2: store>(v1: smart_vector::SmartVector<T1>, v2: smart_vector::SmartVector<T2>, f: |(T1, T2)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip<T1: store, T2: store>(v1: SmartVector<T1>, v2: SmartVector<T2>, f: |T1, T2|) {
        // We need to reverse the vectors to consume it efficiently
        aptos_std::smart_vector::reverse(&mut v1);
        aptos_std::smart_vector::reverse(&mut v2);
        aptos_std::smart_vector::zip_reverse(v1, v2, |e1, e2| f(e1, e2));
    }
}
```


<a id="0x1_smart_vector_zip_reverse"></a>

## Function `zip_reverse`

Apply the function to each pair of elements in the two given vectors in the reverse order, consuming them.
This errors out if the vectors are not of the same length.


```move
module 0x1::smart_vector {
    public fun zip_reverse<T1, T2>(v1: smart_vector::SmartVector<T1>, v2: smart_vector::SmartVector<T2>, f: |(T1, T2)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip_reverse<T1, T2>(
        v1: SmartVector<T1>,
        v2: SmartVector<T2>,
        f: |T1, T2|,
    ) {
        let len = aptos_std::smart_vector::length(&v1);
        // We can't use the constant ESMART_VECTORS_LENGTH_MISMATCH here as all calling code would then need to define it
        // due to how inline functions work.
        assert!(len == aptos_std::smart_vector::length(&v2), 0x20005);
        while (len > 0) {
            f(aptos_std::smart_vector::pop_back(&mut v1), aptos_std::smart_vector::pop_back(&mut v2));
            len = len - 1;
        };
        aptos_std::smart_vector::destroy_empty(v1);
        aptos_std::smart_vector::destroy_empty(v2);
    }
}
```


<a id="0x1_smart_vector_zip_ref"></a>

## Function `zip_ref`

Apply the function to the references of each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


```move
module 0x1::smart_vector {
    public fun zip_ref<T1, T2>(v1: &smart_vector::SmartVector<T1>, v2: &smart_vector::SmartVector<T2>, f: |(&T1, &T2)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip_ref<T1, T2>(
        v1: &SmartVector<T1>,
        v2: &SmartVector<T2>,
        f: |&T1, &T2|,
    ) {
        let len = aptos_std::smart_vector::length(v1);
        // We can't use the constant ESMART_VECTORS_LENGTH_MISMATCH here as all calling code would then need to define it
        // due to how inline functions work.
        assert!(len == aptos_std::smart_vector::length(v2), 0x20005);
        let i = 0;
        while (i < len) {
            f(aptos_std::smart_vector::borrow(v1, i), aptos_std::smart_vector::borrow(v2, i));
            i = i + 1
        }
    }
}
```


<a id="0x1_smart_vector_zip_mut"></a>

## Function `zip_mut`

Apply the function to mutable references to each pair of elements in the two given vectors.
This errors out if the vectors are not of the same length.


```move
module 0x1::smart_vector {
    public fun zip_mut<T1, T2>(v1: &mut smart_vector::SmartVector<T1>, v2: &mut smart_vector::SmartVector<T2>, f: |(&mut T1, &mut T2)|)
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip_mut<T1, T2>(
        v1: &mut SmartVector<T1>,
        v2: &mut SmartVector<T2>,
        f: |&mut T1, &mut T2|,
    ) {
        let i = 0;
        let len = aptos_std::smart_vector::length(v1);
        // We can't use the constant ESMART_VECTORS_LENGTH_MISMATCH here as all calling code would then need to define it
        // due to how inline functions work.
        assert!(len == aptos_std::smart_vector::length(v2), 0x20005);
        while (i < len) {
            f(aptos_std::smart_vector::borrow_mut(v1, i), aptos_std::smart_vector::borrow_mut(v2, i));
            i = i + 1
        }
    }
}
```


<a id="0x1_smart_vector_zip_map"></a>

## Function `zip_map`

Map the function over the element pairs of the two vectors, producing a new vector.


```move
module 0x1::smart_vector {
    public fun zip_map<T1: store, T2: store, NewT: store>(v1: smart_vector::SmartVector<T1>, v2: smart_vector::SmartVector<T2>, f: |(T1, T2)|NewT): smart_vector::SmartVector<NewT>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip_map<T1: store, T2: store, NewT: store>(
        v1: SmartVector<T1>,
        v2: SmartVector<T2>,
        f: |T1, T2|NewT
    ): SmartVector<NewT> {
        // We can't use the constant ESMART_VECTORS_LENGTH_MISMATCH here as all calling code would then need to define it
        // due to how inline functions work.
        assert!(aptos_std::smart_vector::length(&v1) == aptos_std::smart_vector::length(&v2), 0x20005);

        let result = aptos_std::smart_vector::new<NewT>();
        aptos_std::smart_vector::zip(v1, v2, |e1, e2| push_back(&mut result, f(e1, e2)));
        result
    }
}
```


<a id="0x1_smart_vector_zip_map_ref"></a>

## Function `zip_map_ref`

Map the function over the references of the element pairs of two vectors, producing a new vector from the return
values without modifying the original vectors.


```move
module 0x1::smart_vector {
    public fun zip_map_ref<T1, T2, NewT: store>(v1: &smart_vector::SmartVector<T1>, v2: &smart_vector::SmartVector<T2>, f: |(&T1, &T2)|NewT): smart_vector::SmartVector<NewT>
}
```


##### Implementation


```move
module 0x1::smart_vector {
    public inline fun zip_map_ref<T1, T2, NewT: store>(
        v1: &SmartVector<T1>,
        v2: &SmartVector<T2>,
        f: |&T1, &T2|NewT
    ): SmartVector<NewT> {
        // We can't use the constant ESMART_VECTORS_LENGTH_MISMATCH here as all calling code would then need to define it
        // due to how inline functions work.
        assert!(aptos_std::smart_vector::length(v1) == aptos_std::smart_vector::length(v2), 0x20005);

        let result = aptos_std::smart_vector::new<NewT>();
        aptos_std::smart_vector::zip_ref(v1, v2, |e1, e2| push_back(&mut result, f(e1, e2)));
        result
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_SmartVector"></a>

### Struct `SmartVector`


```move
module 0x1::smart_vector {
    struct SmartVector<T> has store
}
```


<dl>
<dt>
`inline_vec: vector<T>`
</dt>
<dd>

</dd>
<dt>
`big_vec: option::Option<big_vector::BigVector<T>>`
</dt>
<dd>

</dd>
<dt>
`inline_capacity: option::Option<u64>`
</dt>
<dd>

</dd>
<dt>
`bucket_size: option::Option<u64>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::smart_vector {
    invariant option::is_none(bucket_size)
        || (option::is_some(bucket_size) && option::borrow(bucket_size) != 0);
    invariant option::is_none(inline_capacity)
        || (len(inline_vec) <= option::borrow(inline_capacity));
    invariant (option::is_none(inline_capacity) && option::is_none(bucket_size))
        || (option::is_some(inline_capacity) && option::is_some(bucket_size));
}
```


<a id="@Specification_1_empty"></a>

### Function `empty`


```move
module 0x1::smart_vector {
    #[deprecated]
    public fun empty<T: store>(): smart_vector::SmartVector<T>
}
```



```move
module 0x1::smart_vector {
    aborts_if false;
}
```


<a id="@Specification_1_empty_with_config"></a>

### Function `empty_with_config`


```move
module 0x1::smart_vector {
    public fun empty_with_config<T: store>(inline_capacity: u64, bucket_size: u64): smart_vector::SmartVector<T>
}
```



```move
module 0x1::smart_vector {
    aborts_if bucket_size == 0;
}
```


<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


```move
module 0x1::smart_vector {
    public fun destroy_empty<T>(v: smart_vector::SmartVector<T>)
}
```



```move
module 0x1::smart_vector {
    aborts_if !(is_empty(v));
    aborts_if len(v.inline_vec) != 0
        || option::is_some(v.big_vec);
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x1::smart_vector {
    public fun borrow<T>(v: &smart_vector::SmartVector<T>, i: u64): &T
}
```



```move
module 0x1::smart_vector {
    aborts_if i >= length(v);
    aborts_if option::is_some(v.big_vec) && (
        (len(v.inline_vec) + big_vector::length<T>(option::borrow(v.big_vec))) > MAX_U64
    );
}
```


<a id="@Specification_1_append"></a>

### Function `append`


```move
module 0x1::smart_vector {
    public fun append<T: store>(lhs: &mut smart_vector::SmartVector<T>, other: smart_vector::SmartVector<T>)
}
```



```move
module 0x1::smart_vector {
    pragma verify = false;
}
```


<a id="@Specification_1_push_back"></a>

### Function `push_back`


```move
module 0x1::smart_vector {
    public fun push_back<T: store>(v: &mut smart_vector::SmartVector<T>, val: T)
}
```



```move
module 0x1::smart_vector {
    pragma verify = false;
}
```


<a id="@Specification_1_pop_back"></a>

### Function `pop_back`


```move
module 0x1::smart_vector {
    public fun pop_back<T>(v: &mut smart_vector::SmartVector<T>): T
}
```



```move
module 0x1::smart_vector {
    pragma verify_duration_estimate = 120;
    aborts_if  option::is_some(v.big_vec)
        &&
        (table_with_length::spec_len(option::borrow(v.big_vec).buckets) == 0);
    aborts_if is_empty(v);
    aborts_if option::is_some(v.big_vec) && (
        (len(v.inline_vec) + big_vector::length<T>(option::borrow(v.big_vec))) > MAX_U64
    );
    ensures length(v) == length(old(v)) - 1;
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x1::smart_vector {
    public fun remove<T>(v: &mut smart_vector::SmartVector<T>, i: u64): T
}
```



```move
module 0x1::smart_vector {
    pragma verify = false;
}
```


<a id="@Specification_1_swap_remove"></a>

### Function `swap_remove`


```move
module 0x1::smart_vector {
    public fun swap_remove<T>(v: &mut smart_vector::SmartVector<T>, i: u64): T
}
```



```move
module 0x1::smart_vector {
    pragma verify = false;
    aborts_if i >= length(v);
    aborts_if option::is_some(v.big_vec) && (
        (len(v.inline_vec) + big_vector::length<T>(option::borrow(v.big_vec))) > MAX_U64
    );
    ensures length(v) == length(old(v)) - 1;
}
```


<a id="@Specification_1_swap"></a>

### Function `swap`


```move
module 0x1::smart_vector {
    public fun swap<T: store>(v: &mut smart_vector::SmartVector<T>, i: u64, j: u64)
}
```



```move
module 0x1::smart_vector {
    pragma verify = false;
}
```


<a id="@Specification_1_length"></a>

### Function `length`


```move
module 0x1::smart_vector {
    public fun length<T>(v: &smart_vector::SmartVector<T>): u64
}
```



```move
module 0x1::smart_vector {
    aborts_if option::is_some(v.big_vec) && len(v.inline_vec) + big_vector::length(option::spec_borrow(v.big_vec)) > MAX_U64;
}
```
