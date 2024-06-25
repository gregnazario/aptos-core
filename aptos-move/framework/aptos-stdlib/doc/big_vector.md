
<a id="0x1_big_vector"></a>

# Module `0x1::big_vector`



-  [Struct `BigVector`](#0x1_big_vector_BigVector)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_big_vector_empty)
-  [Function `singleton`](#0x1_big_vector_singleton)
-  [Function `destroy_empty`](#0x1_big_vector_destroy_empty)
-  [Function `destroy`](#0x1_big_vector_destroy)
-  [Function `borrow`](#0x1_big_vector_borrow)
-  [Function `borrow_mut`](#0x1_big_vector_borrow_mut)
-  [Function `append`](#0x1_big_vector_append)
-  [Function `push_back`](#0x1_big_vector_push_back)
-  [Function `pop_back`](#0x1_big_vector_pop_back)
-  [Function `remove`](#0x1_big_vector_remove)
-  [Function `swap_remove`](#0x1_big_vector_swap_remove)
-  [Function `swap`](#0x1_big_vector_swap)
-  [Function `reverse`](#0x1_big_vector_reverse)
-  [Function `index_of`](#0x1_big_vector_index_of)
-  [Function `contains`](#0x1_big_vector_contains)
-  [Function `to_vector`](#0x1_big_vector_to_vector)
-  [Function `length`](#0x1_big_vector_length)
-  [Function `is_empty`](#0x1_big_vector_is_empty)
-  [Specification](#@Specification_1)
    -  [Struct `BigVector`](#@Specification_1_BigVector)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `singleton`](#@Specification_1_singleton)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `append`](#@Specification_1_append)
    -  [Function `push_back`](#@Specification_1_push_back)
    -  [Function `pop_back`](#@Specification_1_pop_back)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `swap_remove`](#@Specification_1_swap_remove)
    -  [Function `swap`](#@Specification_1_swap)
    -  [Function `reverse`](#@Specification_1_reverse)
    -  [Function `index_of`](#@Specification_1_index_of)


```move
module 0x1::big_vector {
    use 0x1::error;
    use 0x1::table_with_length;
    use 0x1::vector;
}
```


<a id="0x1_big_vector_BigVector"></a>

## Struct `BigVector`

A scalable vector implementation based on tables where elements are grouped into buckets.
Each bucket has a capacity of `bucket_size` elements.


```move
module 0x1::big_vector {
    struct BigVector<T> has store
}
```


##### Fields


<dl>
<dt>
`buckets: table_with_length::TableWithLength<u64, vector<T>>`
</dt>
<dd>

</dd>
<dt>
`end_index: u64`
</dt>
<dd>

</dd>
<dt>
`bucket_size: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_big_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


```move
module 0x1::big_vector {
    const EINDEX_OUT_OF_BOUNDS: u64 = 1;
}
```


<a id="0x1_big_vector_EVECTOR_EMPTY"></a>

Cannot pop back from an empty vector


```move
module 0x1::big_vector {
    const EVECTOR_EMPTY: u64 = 3;
}
```


<a id="0x1_big_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non&#45;empty vector


```move
module 0x1::big_vector {
    const EVECTOR_NOT_EMPTY: u64 = 2;
}
```


<a id="0x1_big_vector_EZERO_BUCKET_SIZE"></a>

bucket_size cannot be 0


```move
module 0x1::big_vector {
    const EZERO_BUCKET_SIZE: u64 = 4;
}
```


<a id="0x1_big_vector_empty"></a>

## Function `empty`

Regular Vector API
Create an empty vector.


```move
module 0x1::big_vector {
    public(friend) fun empty<T: store>(bucket_size: u64): big_vector::BigVector<T>
}
```


##### Implementation


```move
module 0x1::big_vector {
    public(friend) fun empty<T: store>(bucket_size: u64): BigVector<T> {
        assert!(bucket_size > 0, error::invalid_argument(EZERO_BUCKET_SIZE));
        BigVector {
            buckets: table_with_length::new(),
            end_index: 0,
            bucket_size,
        }
    }
}
```


<a id="0x1_big_vector_singleton"></a>

## Function `singleton`

Create a vector of length 1 containing the passed in element.


```move
module 0x1::big_vector {
    public(friend) fun singleton<T: store>(element: T, bucket_size: u64): big_vector::BigVector<T>
}
```


##### Implementation


```move
module 0x1::big_vector {
    public(friend) fun singleton<T: store>(element: T, bucket_size: u64): BigVector<T> {
        let v = empty(bucket_size);
        push_back(&mut v, element);
        v
    }
}
```


<a id="0x1_big_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector `v`.
Aborts if `v` is not empty.


```move
module 0x1::big_vector {
    public fun destroy_empty<T>(v: big_vector::BigVector<T>)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun destroy_empty<T>(v: BigVector<T>) {
        assert!(is_empty(&v), error::invalid_argument(EVECTOR_NOT_EMPTY));
        let BigVector { buckets, end_index: _, bucket_size: _ } = v;
        table_with_length::destroy_empty(buckets);
    }
}
```


<a id="0x1_big_vector_destroy"></a>

## Function `destroy`

Destroy the vector `v` if T has `drop`


```move
module 0x1::big_vector {
    public fun destroy<T: drop>(v: big_vector::BigVector<T>)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun destroy<T: drop>(v: BigVector<T>) {
        let BigVector { buckets, end_index, bucket_size: _ } = v;
        let i = 0;
        while (end_index > 0) {
            let num_elements = vector::length(&table_with_length::remove(&mut buckets, i));
            end_index = end_index - num_elements;
            i = i + 1;
        };
        table_with_length::destroy_empty(buckets);
    }
}
```


<a id="0x1_big_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the `i`th element of the vector `v`.
Aborts if `i` is out of bounds.


```move
module 0x1::big_vector {
    public fun borrow<T>(v: &big_vector::BigVector<T>, i: u64): &T
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun borrow<T>(v: &BigVector<T>, i: u64): &T {
        assert!(i < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        vector::borrow(table_with_length::borrow(&v.buckets, i / v.bucket_size), i % v.bucket_size)
    }
}
```


<a id="0x1_big_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the `i`th element in the vector `v`.
Aborts if `i` is out of bounds.


```move
module 0x1::big_vector {
    public fun borrow_mut<T>(v: &mut big_vector::BigVector<T>, i: u64): &mut T
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun borrow_mut<T>(v: &mut BigVector<T>, i: u64): &mut T {
        assert!(i < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        vector::borrow_mut(table_with_length::borrow_mut(&mut v.buckets, i / v.bucket_size), i % v.bucket_size)
    }
}
```


<a id="0x1_big_vector_append"></a>

## Function `append`

Empty and destroy the other vector, and push each of the elements in the other vector onto the lhs vector in the
same order as they occurred in other.
Disclaimer: This function is costly. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun append<T: store>(lhs: &mut big_vector::BigVector<T>, other: big_vector::BigVector<T>)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun append<T: store>(lhs: &mut BigVector<T>, other: BigVector<T>) {
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


<a id="0x1_big_vector_push_back"></a>

## Function `push_back`

Add element `val` to the end of the vector `v`. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


```move
module 0x1::big_vector {
    public fun push_back<T: store>(v: &mut big_vector::BigVector<T>, val: T)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun push_back<T: store>(v: &mut BigVector<T>, val: T) {
        let num_buckets = table_with_length::length(&v.buckets);
        if (v.end_index == num_buckets * v.bucket_size) {
            table_with_length::add(&mut v.buckets, num_buckets, vector::empty());
            vector::push_back(table_with_length::borrow_mut(&mut v.buckets, num_buckets), val);
        } else {
            vector::push_back(table_with_length::borrow_mut(&mut v.buckets, num_buckets - 1), val);
        };
        v.end_index = v.end_index + 1;
    }
}
```


<a id="0x1_big_vector_pop_back"></a>

## Function `pop_back`

Pop an element from the end of vector `v`. It doesn&apos;t shrink the buckets even if they&apos;re empty.
Call `shrink_to_fit` explicity to deallocate empty buckets.
Aborts if `v` is empty.


```move
module 0x1::big_vector {
    public fun pop_back<T>(v: &mut big_vector::BigVector<T>): T
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun pop_back<T>(v: &mut BigVector<T>): T {
        assert!(!is_empty(v), error::invalid_state(EVECTOR_EMPTY));
        let num_buckets = table_with_length::length(&v.buckets);
        let last_bucket = table_with_length::borrow_mut(&mut v.buckets, num_buckets - 1);
        let val = vector::pop_back(last_bucket);
        // Shrink the table if the last vector is empty.
        if (vector::is_empty(last_bucket)) {
            move last_bucket;
            vector::destroy_empty(table_with_length::remove(&mut v.buckets, num_buckets - 1));
        };
        v.end_index = v.end_index - 1;
        val
    }
}
```


<a id="0x1_big_vector_remove"></a>

## Function `remove`

Remove the element at index i in the vector v and return the owned value that was previously stored at i in v.
All elements occurring at indices greater than i will be shifted down by 1. Will abort if i is out of bounds.
Disclaimer: This function is costly. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun remove<T>(v: &mut big_vector::BigVector<T>, i: u64): T
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun remove<T>(v: &mut BigVector<T>, i: u64): T {
        let len = length(v);
        assert!(i < len, error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let num_buckets = table_with_length::length(&v.buckets);
        let cur_bucket_index = i / v.bucket_size + 1;
        let cur_bucket = table_with_length::borrow_mut(&mut v.buckets, cur_bucket_index - 1);
        let res = vector::remove(cur_bucket, i % v.bucket_size);
        v.end_index = v.end_index - 1;
        move cur_bucket;
        while ({
            spec {
                invariant cur_bucket_index <= num_buckets;
                invariant table_with_length::spec_len(v.buckets) == num_buckets;
            };
            (cur_bucket_index < num_buckets)
        }) {
            // remove one element from the start of current vector
            let cur_bucket = table_with_length::borrow_mut(&mut v.buckets, cur_bucket_index);
            let t = vector::remove(cur_bucket, 0);
            move cur_bucket;
            // and put it at the end of the last one
            let prev_bucket = table_with_length::borrow_mut(&mut v.buckets, cur_bucket_index - 1);
            vector::push_back(prev_bucket, t);
            cur_bucket_index = cur_bucket_index + 1;
        };
        spec {
            assert cur_bucket_index == num_buckets;
        };

        // Shrink the table if the last vector is empty.
        let last_bucket = table_with_length::borrow_mut(&mut v.buckets, num_buckets - 1);
        if (vector::is_empty(last_bucket)) {
            move last_bucket;
            vector::destroy_empty(table_with_length::remove(&mut v.buckets, num_buckets - 1));
        };

        res
    }
}
```


<a id="0x1_big_vector_swap_remove"></a>

## Function `swap_remove`

Swap the `i`th element of the vector `v` with the last element and then pop the vector.
This is O(1), but does not preserve ordering of elements in the vector.
Aborts if `i` is out of bounds.


```move
module 0x1::big_vector {
    public fun swap_remove<T>(v: &mut big_vector::BigVector<T>, i: u64): T
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun swap_remove<T>(v: &mut BigVector<T>, i: u64): T {
        assert!(i < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let last_val = pop_back(v);
        // if the requested value is the last one, return it
        if (v.end_index == i) {
            return last_val
        };
        // because the lack of mem::swap, here we swap remove the requested value from the bucket
        // and append the last_val to the bucket then swap the last bucket val back
        let bucket = table_with_length::borrow_mut(&mut v.buckets, i / v.bucket_size);
        let bucket_len = vector::length(bucket);
        let val = vector::swap_remove(bucket, i % v.bucket_size);
        vector::push_back(bucket, last_val);
        vector::swap(bucket, i % v.bucket_size, bucket_len - 1);
        val
    }
}
```


<a id="0x1_big_vector_swap"></a>

## Function `swap`

Swap the elements at the i&apos;th and j&apos;th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


```move
module 0x1::big_vector {
    public fun swap<T>(v: &mut big_vector::BigVector<T>, i: u64, j: u64)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun swap<T>(v: &mut BigVector<T>, i: u64, j: u64) {
        assert!(i < length(v) && j < length(v), error::invalid_argument(EINDEX_OUT_OF_BOUNDS));
        let i_bucket_index = i / v.bucket_size;
        let j_bucket_index = j / v.bucket_size;
        let i_vector_index = i % v.bucket_size;
        let j_vector_index = j % v.bucket_size;
        if (i_bucket_index == j_bucket_index) {
            vector::swap(table_with_length::borrow_mut(&mut v.buckets, i_bucket_index), i_vector_index, j_vector_index);
            return
        };
        // If i and j are in different buckets, take the buckets out first for easy mutation.
        let bucket_i = table_with_length::remove(&mut v.buckets, i_bucket_index);
        let bucket_j = table_with_length::remove(&mut v.buckets, j_bucket_index);
        // Get the elements from buckets by calling `swap_remove`.
        let element_i = vector::swap_remove(&mut bucket_i, i_vector_index);
        let element_j = vector::swap_remove(&mut bucket_j, j_vector_index);
        // Swap the elements and push back to the other bucket.
        vector::push_back(&mut bucket_i, element_j);
        vector::push_back(&mut bucket_j, element_i);
        let last_index_in_bucket_i = vector::length(&bucket_i) - 1;
        let last_index_in_bucket_j = vector::length(&bucket_j) - 1;
        // Re-position the swapped elements to the right index.
        vector::swap(&mut bucket_i, i_vector_index, last_index_in_bucket_i);
        vector::swap(&mut bucket_j, j_vector_index, last_index_in_bucket_j);
        // Add back the buckets.
        table_with_length::add(&mut v.buckets, i_bucket_index, bucket_i);
        table_with_length::add(&mut v.buckets, j_bucket_index, bucket_j);
    }
}
```


<a id="0x1_big_vector_reverse"></a>

## Function `reverse`

Reverse the order of the elements in the vector v in&#45;place.
Disclaimer: This function is costly. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun reverse<T>(v: &mut big_vector::BigVector<T>)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun reverse<T>(v: &mut BigVector<T>) {
        let new_buckets = vector[];
        let push_bucket = vector[];
        let num_buckets = table_with_length::length(&v.buckets);
        let num_buckets_left = num_buckets;

        while (num_buckets_left > 0) {
            let pop_bucket = table_with_length::remove(&mut v.buckets, num_buckets_left - 1);
            vector::for_each_reverse(pop_bucket, |val| {
                vector::push_back(&mut push_bucket, val);
                if (vector::length(&push_bucket) == v.bucket_size) {
                    vector::push_back(&mut new_buckets, push_bucket);
                    push_bucket = vector[];
                };
            });
            num_buckets_left = num_buckets_left - 1;
        };

        if (vector::length(&push_bucket) > 0) {
            vector::push_back(&mut new_buckets, push_bucket);
        } else {
            vector::destroy_empty(push_bucket);
        };

        vector::reverse(&mut new_buckets);
        let i = 0;
        assert!(table_with_length::length(&v.buckets) == 0, 0);
        while (i < num_buckets) {
            table_with_length::add(&mut v.buckets, i, vector::pop_back(&mut new_buckets));
            i = i + 1;
        };
        vector::destroy_empty(new_buckets);
    }
}
```


<a id="0x1_big_vector_index_of"></a>

## Function `index_of`

Return the index of the first occurrence of an element in v that is equal to e. Returns (true, index) if such an
element was found, and (false, 0) otherwise.
Disclaimer: This function is costly. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun index_of<T>(v: &big_vector::BigVector<T>, val: &T): (bool, u64)
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun index_of<T>(v: &BigVector<T>, val: &T): (bool, u64) {
        let num_buckets = table_with_length::length(&v.buckets);
        let bucket_index = 0;
        while (bucket_index < num_buckets) {
            let cur = table_with_length::borrow(&v.buckets, bucket_index);
            let (found, i) = vector::index_of(cur, val);
            if (found) {
                return (true, bucket_index * v.bucket_size + i)
            };
            bucket_index = bucket_index + 1;
        };
        (false, 0)
    }
}
```


<a id="0x1_big_vector_contains"></a>

## Function `contains`

Return if an element equal to e exists in the vector v.
Disclaimer: This function is costly. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun contains<T>(v: &big_vector::BigVector<T>, val: &T): bool
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun contains<T>(v: &BigVector<T>, val: &T): bool {
        if (is_empty(v)) return false;
        let (exist, _) = index_of(v, val);
        exist
    }
}
```


<a id="0x1_big_vector_to_vector"></a>

## Function `to_vector`

Convert a big vector to a native vector, which is supposed to be called mostly by view functions to get an
atomic view of the whole vector.
Disclaimer: This function may be costly as the big vector may be huge in size. Use it at your own discretion.


```move
module 0x1::big_vector {
    public fun to_vector<T: copy>(v: &big_vector::BigVector<T>): vector<T>
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun to_vector<T: copy>(v: &BigVector<T>): vector<T> {
        let res = vector[];
        let num_buckets = table_with_length::length(&v.buckets);
        let i = 0;
        while (i < num_buckets) {
            vector::append(&mut res, *table_with_length::borrow(&v.buckets, i));
            i = i + 1;
        };
        res
    }
}
```


<a id="0x1_big_vector_length"></a>

## Function `length`

Return the length of the vector.


```move
module 0x1::big_vector {
    public fun length<T>(v: &big_vector::BigVector<T>): u64
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun length<T>(v: &BigVector<T>): u64 {
        v.end_index
    }
}
```


<a id="0x1_big_vector_is_empty"></a>

## Function `is_empty`

Return `true` if the vector `v` has no elements and `false` otherwise.


```move
module 0x1::big_vector {
    public fun is_empty<T>(v: &big_vector::BigVector<T>): bool
}
```


##### Implementation


```move
module 0x1::big_vector {
    public fun is_empty<T>(v: &BigVector<T>): bool {
        length(v) == 0
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_BigVector"></a>

### Struct `BigVector`


```move
module 0x1::big_vector {
    struct BigVector<T> has store
}
```


<dl>
<dt>
`buckets: table_with_length::TableWithLength<u64, vector<T>>`
</dt>
<dd>

</dd>
<dt>
`end_index: u64`
</dt>
<dd>

</dd>
<dt>
`bucket_size: u64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::big_vector {
    invariant bucket_size != 0;
    invariant spec_table_len(buckets) == 0 ==> end_index == 0;
    invariant end_index == 0 ==> spec_table_len(buckets) == 0;
    invariant end_index <= spec_table_len(buckets) * bucket_size;
    invariant spec_table_len(buckets) == 0
        || (forall i in 0..spec_table_len(buckets)-1: len(table_with_length::spec_get(buckets, i)) == bucket_size);
    invariant spec_table_len(buckets) == 0
        || len(table_with_length::spec_get(buckets, spec_table_len(buckets) -1 )) <= bucket_size;
    invariant forall i in 0..spec_table_len(buckets): spec_table_contains(buckets, i);
    invariant spec_table_len(buckets) == (end_index + bucket_size - 1) / bucket_size;
    invariant (spec_table_len(buckets) == 0 && end_index == 0)
        || (spec_table_len(buckets) != 0 && ((spec_table_len(buckets) - 1) * bucket_size) + (len(table_with_length::spec_get(buckets, spec_table_len(buckets) - 1))) == end_index);
    invariant forall i: u64 where i >= spec_table_len(buckets):  {
        !spec_table_contains(buckets, i)
    };
    invariant forall i: u64 where i < spec_table_len(buckets):  {
        spec_table_contains(buckets, i)
    };
    invariant spec_table_len(buckets) == 0
        || (len(table_with_length::spec_get(buckets, spec_table_len(buckets) - 1)) > 0);
}
```


<a id="@Specification_1_empty"></a>

### Function `empty`


```move
module 0x1::big_vector {
    public(friend) fun empty<T: store>(bucket_size: u64): big_vector::BigVector<T>
}
```



```move
module 0x1::big_vector {
    aborts_if bucket_size == 0;
    ensures length(result) == 0;
    ensures result.bucket_size == bucket_size;
}
```


<a id="@Specification_1_singleton"></a>

### Function `singleton`


```move
module 0x1::big_vector {
    public(friend) fun singleton<T: store>(element: T, bucket_size: u64): big_vector::BigVector<T>
}
```



```move
module 0x1::big_vector {
    aborts_if bucket_size == 0;
    ensures length(result) == 1;
    ensures result.bucket_size == bucket_size;
}
```


<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


```move
module 0x1::big_vector {
    public fun destroy_empty<T>(v: big_vector::BigVector<T>)
}
```



```move
module 0x1::big_vector {
    aborts_if !is_empty(v);
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x1::big_vector {
    public fun borrow<T>(v: &big_vector::BigVector<T>, i: u64): &T
}
```



```move
module 0x1::big_vector {
    aborts_if i >= length(v);
    ensures result == spec_at(v, i);
}
```


<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


```move
module 0x1::big_vector {
    public fun borrow_mut<T>(v: &mut big_vector::BigVector<T>, i: u64): &mut T
}
```



```move
module 0x1::big_vector {
    aborts_if i >= length(v);
    ensures result == spec_at(v, i);
}
```


<a id="@Specification_1_append"></a>

### Function `append`


```move
module 0x1::big_vector {
    public fun append<T: store>(lhs: &mut big_vector::BigVector<T>, other: big_vector::BigVector<T>)
}
```



```move
module 0x1::big_vector {
    pragma verify=false;
}
```


<a id="@Specification_1_push_back"></a>

### Function `push_back`


```move
module 0x1::big_vector {
    public fun push_back<T: store>(v: &mut big_vector::BigVector<T>, val: T)
}
```



```move
module 0x1::big_vector {
    let num_buckets = spec_table_len(v.buckets);
    include PushbackAbortsIf<T>;
    ensures length(v) == length(old(v)) + 1;
    ensures v.end_index == old(v.end_index) + 1;
    ensures spec_at(v, v.end_index-1) == val;
    ensures forall i in 0..v.end_index-1: spec_at(v, i) == spec_at(old(v), i);
    ensures v.bucket_size == old(v).bucket_size;
}
```



<a id="0x1_big_vector_PushbackAbortsIf"></a>


```move
module 0x1::big_vector {
    schema PushbackAbortsIf<T> {
        v: BigVector<T>;
        let num_buckets = spec_table_len(v.buckets);
        aborts_if num_buckets * v.bucket_size > MAX_U64;
        aborts_if v.end_index + 1 > MAX_U64;
    }
}
```


<a id="@Specification_1_pop_back"></a>

### Function `pop_back`


```move
module 0x1::big_vector {
    public fun pop_back<T>(v: &mut big_vector::BigVector<T>): T
}
```



```move
module 0x1::big_vector {
    aborts_if is_empty(v);
    ensures length(v) == length(old(v)) - 1;
    ensures result == old(spec_at(v, v.end_index-1));
    ensures forall i in 0..v.end_index: spec_at(v, i) == spec_at(old(v), i);
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x1::big_vector {
    public fun remove<T>(v: &mut big_vector::BigVector<T>, i: u64): T
}
```



```move
module 0x1::big_vector {
    pragma verify=false;
}
```


<a id="@Specification_1_swap_remove"></a>

### Function `swap_remove`


```move
module 0x1::big_vector {
    public fun swap_remove<T>(v: &mut big_vector::BigVector<T>, i: u64): T
}
```



```move
module 0x1::big_vector {
    pragma verify_duration_estimate = 120;
    aborts_if i >= length(v);
    ensures length(v) == length(old(v)) - 1;
    ensures result == spec_at(old(v), i);
}
```


<a id="@Specification_1_swap"></a>

### Function `swap`


```move
module 0x1::big_vector {
    public fun swap<T>(v: &mut big_vector::BigVector<T>, i: u64, j: u64)
}
```



```move
module 0x1::big_vector {
    pragma verify_duration_estimate = 1000;
    aborts_if i >= length(v) || j >= length(v);
    ensures length(v) == length(old(v));
    ensures spec_at(v, i) == spec_at(old(v), j);
    ensures spec_at(v, j) == spec_at(old(v), i);
    ensures forall idx in 0..length(v)
        where idx != i && idx != j:
        spec_at(v, idx) == spec_at(old(v), idx);
}
```


<a id="@Specification_1_reverse"></a>

### Function `reverse`


```move
module 0x1::big_vector {
    public fun reverse<T>(v: &mut big_vector::BigVector<T>)
}
```



```move
module 0x1::big_vector {
    pragma verify=false;
}
```


<a id="@Specification_1_index_of"></a>

### Function `index_of`


```move
module 0x1::big_vector {
    public fun index_of<T>(v: &big_vector::BigVector<T>, val: &T): (bool, u64)
}
```



```move
module 0x1::big_vector {
    pragma verify=false;
}
```



<a id="0x1_big_vector_spec_table_len"></a>


```move
module 0x1::big_vector {
    fun spec_table_len<K, V>(t: TableWithLength<K, V>): u64 {
       table_with_length::spec_len(t)
    }
}
```



<a id="0x1_big_vector_spec_table_contains"></a>


```move
module 0x1::big_vector {
    fun spec_table_contains<K, V>(t: TableWithLength<K, V>, k: K): bool {
       table_with_length::spec_contains(t, k)
    }
}
```



<a id="0x1_big_vector_spec_at"></a>


```move
module 0x1::big_vector {
    fun spec_at<T>(v: BigVector<T>, i: u64): T {
       let bucket = i / v.bucket_size;
       let idx = i % v.bucket_size;
       let v = table_with_length::spec_get(v.buckets, bucket);
       v[idx]
    }
}
```
