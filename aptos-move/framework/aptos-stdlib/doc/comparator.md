
<a id="0x1_comparator"></a>

# Module `0x1::comparator`

Provides a framework for comparing two elements


-  [Struct `Result`](#0x1_comparator_Result)
-  [Constants](#@Constants_0)
-  [Function `is_equal`](#0x1_comparator_is_equal)
-  [Function `is_smaller_than`](#0x1_comparator_is_smaller_than)
-  [Function `is_greater_than`](#0x1_comparator_is_greater_than)
-  [Function `compare`](#0x1_comparator_compare)
-  [Function `compare_u8_vector`](#0x1_comparator_compare_u8_vector)
-  [Specification](#@Specification_1)
    -  [Struct `Result`](#@Specification_1_Result)
    -  [Function `is_equal`](#@Specification_1_is_equal)
    -  [Function `is_smaller_than`](#@Specification_1_is_smaller_than)
    -  [Function `is_greater_than`](#@Specification_1_is_greater_than)
    -  [Function `compare`](#@Specification_1_compare)
    -  [Function `compare_u8_vector`](#@Specification_1_compare_u8_vector)


```move
module 0x1::comparator {
    use 0x1::bcs;
}
```


<a id="0x1_comparator_Result"></a>

## Struct `Result`



```move
module 0x1::comparator {
    struct Result has drop
}
```


##### Fields


<dl>
<dt>
`inner: u8`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_comparator_EQUAL"></a>



```move
module 0x1::comparator {
    const EQUAL: u8 = 0;
}
```


<a id="0x1_comparator_GREATER"></a>



```move
module 0x1::comparator {
    const GREATER: u8 = 2;
}
```


<a id="0x1_comparator_SMALLER"></a>



```move
module 0x1::comparator {
    const SMALLER: u8 = 1;
}
```


<a id="0x1_comparator_is_equal"></a>

## Function `is_equal`



```move
module 0x1::comparator {
    public fun is_equal(result: &comparator::Result): bool
}
```


##### Implementation


```move
module 0x1::comparator {
    public fun is_equal(result: &Result): bool {
        result.inner == EQUAL
    }
}
```


<a id="0x1_comparator_is_smaller_than"></a>

## Function `is_smaller_than`



```move
module 0x1::comparator {
    public fun is_smaller_than(result: &comparator::Result): bool
}
```


##### Implementation


```move
module 0x1::comparator {
    public fun is_smaller_than(result: &Result): bool {
        result.inner == SMALLER
    }
}
```


<a id="0x1_comparator_is_greater_than"></a>

## Function `is_greater_than`



```move
module 0x1::comparator {
    public fun is_greater_than(result: &comparator::Result): bool
}
```


##### Implementation


```move
module 0x1::comparator {
    public fun is_greater_than(result: &Result): bool {
        result.inner == GREATER
    }
}
```


<a id="0x1_comparator_compare"></a>

## Function `compare`



```move
module 0x1::comparator {
    public fun compare<T>(left: &T, right: &T): comparator::Result
}
```


##### Implementation


```move
module 0x1::comparator {
    public fun compare<T>(left: &T, right: &T): Result {
        let left_bytes = bcs::to_bytes(left);
        let right_bytes = bcs::to_bytes(right);

        compare_u8_vector(left_bytes, right_bytes)
    }
}
```


<a id="0x1_comparator_compare_u8_vector"></a>

## Function `compare_u8_vector`



```move
module 0x1::comparator {
    public fun compare_u8_vector(left: vector<u8>, right: vector<u8>): comparator::Result
}
```


##### Implementation


```move
module 0x1::comparator {
    public fun compare_u8_vector(left: vector<u8>, right: vector<u8>): Result {
        let left_length = vector::length(&left);
        let right_length = vector::length(&right);

        let idx = 0;

        while (idx < left_length && idx < right_length) {
            let left_byte = *vector::borrow(&left, idx);
            let right_byte = *vector::borrow(&right, idx);

            if (left_byte < right_byte) {
                return Result { inner: SMALLER }
            } else if (left_byte > right_byte) {
                return Result { inner: GREATER }
            };
            idx = idx + 1;
        };

        if (left_length < right_length) {
            Result { inner: SMALLER }
        } else if (left_length > right_length) {
            Result { inner: GREATER }
        } else {
            Result { inner: EQUAL }
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_Result"></a>

### Struct `Result`


```move
module 0x1::comparator {
    struct Result has drop
}
```


<dl>
<dt>
`inner: u8`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::comparator {
    invariant inner == EQUAL || inner == SMALLER || inner == GREATER;
}
```


<a id="@Specification_1_is_equal"></a>

### Function `is_equal`


```move
module 0x1::comparator {
    public fun is_equal(result: &comparator::Result): bool
}
```



```move
module 0x1::comparator {
    aborts_if false;
    let res = result;
    ensures result == (res.inner == EQUAL);
}
```


<a id="@Specification_1_is_smaller_than"></a>

### Function `is_smaller_than`


```move
module 0x1::comparator {
    public fun is_smaller_than(result: &comparator::Result): bool
}
```



```move
module 0x1::comparator {
    aborts_if false;
    let res = result;
    ensures result == (res.inner == SMALLER);
}
```


<a id="@Specification_1_is_greater_than"></a>

### Function `is_greater_than`


```move
module 0x1::comparator {
    public fun is_greater_than(result: &comparator::Result): bool
}
```



```move
module 0x1::comparator {
    aborts_if false;
    let res = result;
    ensures result == (res.inner == GREATER);
}
```


<a id="@Specification_1_compare"></a>

### Function `compare`


```move
module 0x1::comparator {
    public fun compare<T>(left: &T, right: &T): comparator::Result
}
```



```move
module 0x1::comparator {
    let left_bytes = bcs::to_bytes(left);
    let right_bytes = bcs::to_bytes(right);
    ensures result == spec_compare_u8_vector(left_bytes, right_bytes);
}
```



<a id="0x1_comparator_spec_compare_u8_vector"></a>


```move
module 0x1::comparator {
    fun spec_compare_u8_vector(left: vector<u8>, right: vector<u8>): Result;
}
```


<a id="@Specification_1_compare_u8_vector"></a>

### Function `compare_u8_vector`


```move
module 0x1::comparator {
    public fun compare_u8_vector(left: vector<u8>, right: vector<u8>): comparator::Result
}
```



```move
module 0x1::comparator {
    pragma unroll = 5;
    pragma opaque;
    aborts_if false;
    let left_length = len(left);
    let right_length = len(right);
    ensures (result.inner == EQUAL) ==> (
        (left_length == right_length) &&
            (forall i: u64 where i < left_length: left[i] == right[i])
    );
    ensures (result.inner == SMALLER) ==> (
        (exists i: u64 where i < left_length:
            (i < right_length) &&
                (left[i] < right[i]) &&
                (forall j: u64 where j < i: left[j] == right[j])
        ) ||
            (left_length < right_length)
    );
    ensures (result.inner == GREATER) ==> (
        (exists i: u64 where i < left_length:
            (i < right_length) &&
                (left[i] > right[i]) &&
                (forall j: u64 where j < i: left[j] == right[j])
        ) ||
            (left_length > right_length)
    );
    ensures [abstract] result == spec_compare_u8_vector(left, right);
}
```
