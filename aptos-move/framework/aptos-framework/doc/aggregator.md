
<a id="0x1_aggregator"></a>

# Module `0x1::aggregator`

This module provides an interface for aggregators. Aggregators are similar to
unsigned integers and support addition and subtraction (aborting on underflow
or on overflowing a custom upper limit). The difference from integers is that
aggregators allow to perform both additions and subtractions in parallel across
multiple transactions, enabling parallel execution. For example, if the first
transaction is doing `add(X, 1)` for aggregator resource `X`, and the second
is doing `sub(X,3)`, they can be executed in parallel avoiding a read&#45;modify&#45;write
dependency.
However, reading the aggregator value (i.e. calling `read(X)`) is an expensive
operation and should be avoided as much as possible because it reduces the
parallelism. Moreover, &#42;&#42;aggregators can only be created by Aptos Framework (0x1)
at the moment.&#42;&#42;


-  [Struct `Aggregator`](#0x1_aggregator_Aggregator)
-  [Constants](#@Constants_0)
-  [Function `limit`](#0x1_aggregator_limit)
-  [Function `add`](#0x1_aggregator_add)
-  [Function `sub`](#0x1_aggregator_sub)
-  [Function `read`](#0x1_aggregator_read)
-  [Function `destroy`](#0x1_aggregator_destroy)
-  [Specification](#@Specification_1)
    -  [Struct `Aggregator`](#@Specification_1_Aggregator)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `limit`](#@Specification_1_limit)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `sub`](#@Specification_1_sub)
    -  [Function `read`](#@Specification_1_read)
    -  [Function `destroy`](#@Specification_1_destroy)


```move
module 0x1::aggregator {
}
```


<a id="0x1_aggregator_Aggregator"></a>

## Struct `Aggregator`

Represents an integer which supports parallel additions and subtractions
across multiple transactions. See the module description for more details.


```move
module 0x1::aggregator {
    struct Aggregator has store
}
```


##### Fields


<dl>
<dt>
`handle: address`
</dt>
<dd>

</dd>
<dt>
`key: address`
</dt>
<dd>

</dd>
<dt>
`limit: u128`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aggregator_EAGGREGATOR_OVERFLOW"></a>

The value of aggregator overflows. Raised by native code.


```move
module 0x1::aggregator {
    const EAGGREGATOR_OVERFLOW: u64 = 1;
}
```


<a id="0x1_aggregator_EAGGREGATOR_UNDERFLOW"></a>

The value of aggregator underflows (goes below zero). Raised by native code.


```move
module 0x1::aggregator {
    const EAGGREGATOR_UNDERFLOW: u64 = 2;
}
```


<a id="0x1_aggregator_ENOT_SUPPORTED"></a>

Aggregator feature is not supported. Raised by native code.


```move
module 0x1::aggregator {
    const ENOT_SUPPORTED: u64 = 3;
}
```


<a id="0x1_aggregator_limit"></a>

## Function `limit`

Returns `limit` exceeding which aggregator overflows.


```move
module 0x1::aggregator {
    public fun limit(aggregator: &aggregator::Aggregator): u128
}
```


##### Implementation


```move
module 0x1::aggregator {
    public fun limit(aggregator: &Aggregator): u128 {
        aggregator.limit
    }
}
```


<a id="0x1_aggregator_add"></a>

## Function `add`

Adds `value` to aggregator. Aborts on overflowing the limit.


```move
module 0x1::aggregator {
    public fun add(aggregator: &mut aggregator::Aggregator, value: u128)
}
```


##### Implementation


```move
module 0x1::aggregator {
    public native fun add(aggregator: &mut Aggregator, value: u128);
}
```


<a id="0x1_aggregator_sub"></a>

## Function `sub`

Subtracts `value` from aggregator. Aborts on going below zero.


```move
module 0x1::aggregator {
    public fun sub(aggregator: &mut aggregator::Aggregator, value: u128)
}
```


##### Implementation


```move
module 0x1::aggregator {
    public native fun sub(aggregator: &mut Aggregator, value: u128);
}
```


<a id="0x1_aggregator_read"></a>

## Function `read`

Returns a value stored in this aggregator.


```move
module 0x1::aggregator {
    public fun read(aggregator: &aggregator::Aggregator): u128
}
```


##### Implementation


```move
module 0x1::aggregator {
    public native fun read(aggregator: &Aggregator): u128;
}
```


<a id="0x1_aggregator_destroy"></a>

## Function `destroy`

Destroys an aggregator and removes it from its `AggregatorFactory`.


```move
module 0x1::aggregator {
    public fun destroy(aggregator: aggregator::Aggregator)
}
```


##### Implementation


```move
module 0x1::aggregator {
    public native fun destroy(aggregator: Aggregator);
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_Aggregator"></a>

### Struct `Aggregator`


```move
module 0x1::aggregator {
    struct Aggregator has store
}
```


<dl>
<dt>
`handle: address`
</dt>
<dd>

</dd>
<dt>
`key: address`
</dt>
<dd>

</dd>
<dt>
`limit: u128`
</dt>
<dd>

</dd>
</dl>




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>For a given aggregator, it should always be possible to: Return the limit value of the aggregator. Return the current value stored in the aggregator. Destroy an aggregator, removing it from its AggregatorFactory.</td>
<td>Low</td>
<td>The following functions should not abort if EventHandle exists: limit(), read(), destroy().</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1.1](read), [#high&#45;level&#45;req&#45;1.2](destroy), and [#high&#45;level&#45;req&#45;1.3](limit).</td>
</tr>

<tr>
<td>2</td>
<td>If the value during addition exceeds the limit, an overflow occurs.</td>
<td>High</td>
<td>The native add() function checks the value of the addition to ensure it does not pass the defined limit and results in aggregator overflow.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](add).</td>
</tr>

<tr>
<td>3</td>
<td>Operations over aggregators should be correct.</td>
<td>High</td>
<td>The implementation of the add, sub, read and destroy functions is correct.</td>
<td>The native implementation of the add, sub, read and destroy functions have been manually audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::aggregator {
    pragma intrinsic;
}
```


<a id="@Specification_1_limit"></a>

### Function `limit`


```move
module 0x1::aggregator {
    public fun limit(aggregator: &aggregator::Aggregator): u128
}
```



```move
module 0x1::aggregator {
    pragma opaque;
// This enforces ### high&#45;level&#45;req&#45;1.2
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if false;
    ensures [abstract] result == spec_get_limit(aggregator);
}
```



<a id="0x1_aggregator_spec_read"></a>


```move
module 0x1::aggregator {
    native fun spec_read(aggregator: Aggregator): u128;
}
```



<a id="0x1_aggregator_spec_get_limit"></a>


```move
module 0x1::aggregator {
    native fun spec_get_limit(a: Aggregator): u128;
}
```



<a id="0x1_aggregator_spec_get_handle"></a>


```move
module 0x1::aggregator {
    native fun spec_get_handle(a: Aggregator): u128;
}
```



<a id="0x1_aggregator_spec_get_key"></a>


```move
module 0x1::aggregator {
    native fun spec_get_key(a: Aggregator): u128;
}
```



<a id="0x1_aggregator_spec_aggregator_set_val"></a>


```move
module 0x1::aggregator {
    native fun spec_aggregator_set_val(a: Aggregator, v: u128): Aggregator;
}
```



<a id="0x1_aggregator_spec_aggregator_get_val"></a>


```move
module 0x1::aggregator {
    native fun spec_aggregator_get_val(a: Aggregator): u128;
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::aggregator {
    public fun add(aggregator: &mut aggregator::Aggregator, value: u128)
}
```



```move
module 0x1::aggregator {
    pragma opaque;
    aborts_if spec_aggregator_get_val(aggregator) + value > spec_get_limit(aggregator);
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if spec_aggregator_get_val(aggregator) + value > MAX_U128;
    ensures spec_get_limit(aggregator) == spec_get_limit(old(aggregator));
    ensures aggregator == spec_aggregator_set_val(old(aggregator),
        spec_aggregator_get_val(old(aggregator)) + value);
}
```


<a id="@Specification_1_sub"></a>

### Function `sub`


```move
module 0x1::aggregator {
    public fun sub(aggregator: &mut aggregator::Aggregator, value: u128)
}
```



```move
module 0x1::aggregator {
    pragma opaque;
    aborts_if spec_aggregator_get_val(aggregator) < value;
    ensures spec_get_limit(aggregator) == spec_get_limit(old(aggregator));
    ensures aggregator == spec_aggregator_set_val(old(aggregator),
        spec_aggregator_get_val(old(aggregator)) - value);
}
```


<a id="@Specification_1_read"></a>

### Function `read`


```move
module 0x1::aggregator {
    public fun read(aggregator: &aggregator::Aggregator): u128
}
```



```move
module 0x1::aggregator {
    pragma opaque;
// This enforces ### high&#45;level&#45;req&#45;1.1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if false;
    ensures result == spec_read(aggregator);
    ensures result <= spec_get_limit(aggregator);
}
```


<a id="@Specification_1_destroy"></a>

### Function `destroy`


```move
module 0x1::aggregator {
    public fun destroy(aggregator: aggregator::Aggregator)
}
```



```move
module 0x1::aggregator {
    pragma opaque;
// This enforces ### high&#45;level&#45;req&#45;1.2
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if false;
}
```
