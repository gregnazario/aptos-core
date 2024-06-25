
<a id="0x1_optional_aggregator"></a>

# Module `0x1::optional_aggregator`

This module provides an interface to aggregate integers either via
aggregator (parallelizable) or via normal integers.


-  [Struct `Integer`](#0x1_optional_aggregator_Integer)
-  [Struct `OptionalAggregator`](#0x1_optional_aggregator_OptionalAggregator)
-  [Constants](#@Constants_0)
-  [Function `new_integer`](#0x1_optional_aggregator_new_integer)
-  [Function `add_integer`](#0x1_optional_aggregator_add_integer)
-  [Function `sub_integer`](#0x1_optional_aggregator_sub_integer)
-  [Function `limit`](#0x1_optional_aggregator_limit)
-  [Function `read_integer`](#0x1_optional_aggregator_read_integer)
-  [Function `destroy_integer`](#0x1_optional_aggregator_destroy_integer)
-  [Function `new`](#0x1_optional_aggregator_new)
-  [Function `switch`](#0x1_optional_aggregator_switch)
-  [Function `switch_and_zero_out`](#0x1_optional_aggregator_switch_and_zero_out)
-  [Function `switch_to_integer_and_zero_out`](#0x1_optional_aggregator_switch_to_integer_and_zero_out)
-  [Function `switch_to_aggregator_and_zero_out`](#0x1_optional_aggregator_switch_to_aggregator_and_zero_out)
-  [Function `destroy`](#0x1_optional_aggregator_destroy)
-  [Function `destroy_optional_aggregator`](#0x1_optional_aggregator_destroy_optional_aggregator)
-  [Function `destroy_optional_integer`](#0x1_optional_aggregator_destroy_optional_integer)
-  [Function `add`](#0x1_optional_aggregator_add)
-  [Function `sub`](#0x1_optional_aggregator_sub)
-  [Function `read`](#0x1_optional_aggregator_read)
-  [Function `is_parallelizable`](#0x1_optional_aggregator_is_parallelizable)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Struct `OptionalAggregator`](#@Specification_1_OptionalAggregator)
    -  [Function `new_integer`](#@Specification_1_new_integer)
    -  [Function `add_integer`](#@Specification_1_add_integer)
    -  [Function `sub_integer`](#@Specification_1_sub_integer)
    -  [Function `limit`](#@Specification_1_limit)
    -  [Function `read_integer`](#@Specification_1_read_integer)
    -  [Function `destroy_integer`](#@Specification_1_destroy_integer)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `switch`](#@Specification_1_switch)
    -  [Function `switch_and_zero_out`](#@Specification_1_switch_and_zero_out)
    -  [Function `switch_to_integer_and_zero_out`](#@Specification_1_switch_to_integer_and_zero_out)
    -  [Function `switch_to_aggregator_and_zero_out`](#@Specification_1_switch_to_aggregator_and_zero_out)
    -  [Function `destroy`](#@Specification_1_destroy)
    -  [Function `destroy_optional_aggregator`](#@Specification_1_destroy_optional_aggregator)
    -  [Function `destroy_optional_integer`](#@Specification_1_destroy_optional_integer)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `sub`](#@Specification_1_sub)
    -  [Function `read`](#@Specification_1_read)


```move
module 0x1::optional_aggregator {
    use 0x1::aggregator;
    use 0x1::aggregator_factory;
    use 0x1::error;
    use 0x1::option;
}
```


<a id="0x1_optional_aggregator_Integer"></a>

## Struct `Integer`

Wrapper around integer with a custom overflow limit. Supports add, subtract and read just like `Aggregator`.


```move
module 0x1::optional_aggregator {
    struct Integer has store
}
```


##### Fields


<dl>
<dt>
`value: u128`
</dt>
<dd>

</dd>
<dt>
`limit: u128`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_optional_aggregator_OptionalAggregator"></a>

## Struct `OptionalAggregator`

Contains either an aggregator or a normal integer, both overflowing on limit.


```move
module 0x1::optional_aggregator {
    struct OptionalAggregator has store
}
```


##### Fields


<dl>
<dt>
`aggregator: option::Option<aggregator::Aggregator>`
</dt>
<dd>

</dd>
<dt>
`integer: option::Option<optional_aggregator::Integer>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_optional_aggregator_EAGGREGATOR_OVERFLOW"></a>

The value of aggregator underflows (goes below zero). Raised by native code.


```move
module 0x1::optional_aggregator {
    const EAGGREGATOR_OVERFLOW: u64 = 1;
}
```


<a id="0x1_optional_aggregator_EAGGREGATOR_UNDERFLOW"></a>

Aggregator feature is not supported. Raised by native code.


```move
module 0x1::optional_aggregator {
    const EAGGREGATOR_UNDERFLOW: u64 = 2;
}
```


<a id="0x1_optional_aggregator_new_integer"></a>

## Function `new_integer`

Creates a new integer which overflows on exceeding a `limit`.


```move
module 0x1::optional_aggregator {
    fun new_integer(limit: u128): optional_aggregator::Integer
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun new_integer(limit: u128): Integer {
        Integer {
            value: 0,
            limit,
        }
    }
}
```


<a id="0x1_optional_aggregator_add_integer"></a>

## Function `add_integer`

Adds `value` to integer. Aborts on overflowing the limit.


```move
module 0x1::optional_aggregator {
    fun add_integer(integer: &mut optional_aggregator::Integer, value: u128)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun add_integer(integer: &mut Integer, value: u128) {
        assert!(
            value <= (integer.limit - integer.value),
            error::out_of_range(EAGGREGATOR_OVERFLOW)
        );
        integer.value = integer.value + value;
    }
}
```


<a id="0x1_optional_aggregator_sub_integer"></a>

## Function `sub_integer`

Subtracts `value` from integer. Aborts on going below zero.


```move
module 0x1::optional_aggregator {
    fun sub_integer(integer: &mut optional_aggregator::Integer, value: u128)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun sub_integer(integer: &mut Integer, value: u128) {
        assert!(value <= integer.value, error::out_of_range(EAGGREGATOR_UNDERFLOW));
        integer.value = integer.value - value;
    }
}
```


<a id="0x1_optional_aggregator_limit"></a>

## Function `limit`

Returns an overflow limit of integer.


```move
module 0x1::optional_aggregator {
    fun limit(integer: &optional_aggregator::Integer): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun limit(integer: &Integer): u128 {
        integer.limit
    }
}
```


<a id="0x1_optional_aggregator_read_integer"></a>

## Function `read_integer`

Returns a value stored in this integer.


```move
module 0x1::optional_aggregator {
    fun read_integer(integer: &optional_aggregator::Integer): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun read_integer(integer: &Integer): u128 {
        integer.value
    }
}
```


<a id="0x1_optional_aggregator_destroy_integer"></a>

## Function `destroy_integer`

Destroys an integer.


```move
module 0x1::optional_aggregator {
    fun destroy_integer(integer: optional_aggregator::Integer)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun destroy_integer(integer: Integer) {
        let Integer { value: _, limit: _ } = integer;
    }
}
```


<a id="0x1_optional_aggregator_new"></a>

## Function `new`

Creates a new optional aggregator.


```move
module 0x1::optional_aggregator {
    public(friend) fun new(limit: u128, parallelizable: bool): optional_aggregator::OptionalAggregator
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public(friend) fun new(limit: u128, parallelizable: bool): OptionalAggregator {
        if (parallelizable) {
            OptionalAggregator {
                aggregator: option::some(aggregator_factory::create_aggregator_internal(limit)),
                integer: option::none(),
            }
        } else {
            OptionalAggregator {
                aggregator: option::none(),
                integer: option::some(new_integer(limit)),
            }
        }
    }
}
```


<a id="0x1_optional_aggregator_switch"></a>

## Function `switch`

Switches between parallelizable and non&#45;parallelizable implementations.


```move
module 0x1::optional_aggregator {
    public fun switch(optional_aggregator: &mut optional_aggregator::OptionalAggregator)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun switch(optional_aggregator: &mut OptionalAggregator) {
        let value = read(optional_aggregator);
        switch_and_zero_out(optional_aggregator);
        add(optional_aggregator, value);
    }
}
```


<a id="0x1_optional_aggregator_switch_and_zero_out"></a>

## Function `switch_and_zero_out`

Switches between parallelizable and non&#45;parallelizable implementations, setting
the value of the new optional aggregator to zero.


```move
module 0x1::optional_aggregator {
    fun switch_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun switch_and_zero_out(optional_aggregator: &mut OptionalAggregator) {
        if (is_parallelizable(optional_aggregator)) {
            switch_to_integer_and_zero_out(optional_aggregator);
        } else {
            switch_to_aggregator_and_zero_out(optional_aggregator);
        }
    }
}
```


<a id="0x1_optional_aggregator_switch_to_integer_and_zero_out"></a>

## Function `switch_to_integer_and_zero_out`

Switches from parallelizable to non&#45;parallelizable implementation, zero&#45;initializing
the value.


```move
module 0x1::optional_aggregator {
    fun switch_to_integer_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun switch_to_integer_and_zero_out(
        optional_aggregator: &mut OptionalAggregator
    ): u128 {
        let aggregator = option::extract(&mut optional_aggregator.aggregator);
        let limit = aggregator::limit(&aggregator);
        aggregator::destroy(aggregator);
        let integer = new_integer(limit);
        option::fill(&mut optional_aggregator.integer, integer);
        limit
    }
}
```


<a id="0x1_optional_aggregator_switch_to_aggregator_and_zero_out"></a>

## Function `switch_to_aggregator_and_zero_out`

Switches from non&#45;parallelizable to parallelizable implementation, zero&#45;initializing
the value.


```move
module 0x1::optional_aggregator {
    fun switch_to_aggregator_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun switch_to_aggregator_and_zero_out(
        optional_aggregator: &mut OptionalAggregator
    ): u128 {
        let integer = option::extract(&mut optional_aggregator.integer);
        let limit = limit(&integer);
        destroy_integer(integer);
        let aggregator = aggregator_factory::create_aggregator_internal(limit);
        option::fill(&mut optional_aggregator.aggregator, aggregator);
        limit
    }
}
```


<a id="0x1_optional_aggregator_destroy"></a>

## Function `destroy`

Destroys optional aggregator.


```move
module 0x1::optional_aggregator {
    public fun destroy(optional_aggregator: optional_aggregator::OptionalAggregator)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun destroy(optional_aggregator: OptionalAggregator) {
        if (is_parallelizable(&optional_aggregator)) {
            destroy_optional_aggregator(optional_aggregator);
        } else {
            destroy_optional_integer(optional_aggregator);
        }
    }
}
```


<a id="0x1_optional_aggregator_destroy_optional_aggregator"></a>

## Function `destroy_optional_aggregator`

Destroys parallelizable optional aggregator and returns its limit.


```move
module 0x1::optional_aggregator {
    fun destroy_optional_aggregator(optional_aggregator: optional_aggregator::OptionalAggregator): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun destroy_optional_aggregator(optional_aggregator: OptionalAggregator): u128 {
        let OptionalAggregator { aggregator, integer } = optional_aggregator;
        let limit = aggregator::limit(option::borrow(&aggregator));
        aggregator::destroy(option::destroy_some(aggregator));
        option::destroy_none(integer);
        limit
    }
}
```


<a id="0x1_optional_aggregator_destroy_optional_integer"></a>

## Function `destroy_optional_integer`

Destroys non&#45;parallelizable optional aggregator and returns its limit.


```move
module 0x1::optional_aggregator {
    fun destroy_optional_integer(optional_aggregator: optional_aggregator::OptionalAggregator): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    fun destroy_optional_integer(optional_aggregator: OptionalAggregator): u128 {
        let OptionalAggregator { aggregator, integer } = optional_aggregator;
        let limit = limit(option::borrow(&integer));
        destroy_integer(option::destroy_some(integer));
        option::destroy_none(aggregator);
        limit
    }
}
```


<a id="0x1_optional_aggregator_add"></a>

## Function `add`

Adds `value` to optional aggregator, aborting on exceeding the `limit`.


```move
module 0x1::optional_aggregator {
    public fun add(optional_aggregator: &mut optional_aggregator::OptionalAggregator, value: u128)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun add(optional_aggregator: &mut OptionalAggregator, value: u128) {
        if (option::is_some(&optional_aggregator.aggregator)) {
            let aggregator = option::borrow_mut(&mut optional_aggregator.aggregator);
            aggregator::add(aggregator, value);
        } else {
            let integer = option::borrow_mut(&mut optional_aggregator.integer);
            add_integer(integer, value);
        }
    }
}
```


<a id="0x1_optional_aggregator_sub"></a>

## Function `sub`

Subtracts `value` from optional aggregator, aborting on going below zero.


```move
module 0x1::optional_aggregator {
    public fun sub(optional_aggregator: &mut optional_aggregator::OptionalAggregator, value: u128)
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun sub(optional_aggregator: &mut OptionalAggregator, value: u128) {
        if (option::is_some(&optional_aggregator.aggregator)) {
            let aggregator = option::borrow_mut(&mut optional_aggregator.aggregator);
            aggregator::sub(aggregator, value);
        } else {
            let integer = option::borrow_mut(&mut optional_aggregator.integer);
            sub_integer(integer, value);
        }
    }
}
```


<a id="0x1_optional_aggregator_read"></a>

## Function `read`

Returns the value stored in optional aggregator.


```move
module 0x1::optional_aggregator {
    public fun read(optional_aggregator: &optional_aggregator::OptionalAggregator): u128
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun read(optional_aggregator: &OptionalAggregator): u128 {
        if (option::is_some(&optional_aggregator.aggregator)) {
            let aggregator = option::borrow(&optional_aggregator.aggregator);
            aggregator::read(aggregator)
        } else {
            let integer = option::borrow(&optional_aggregator.integer);
            read_integer(integer)
        }
    }
}
```


<a id="0x1_optional_aggregator_is_parallelizable"></a>

## Function `is_parallelizable`

Returns true if optional aggregator uses parallelizable implementation.


```move
module 0x1::optional_aggregator {
    public fun is_parallelizable(optional_aggregator: &optional_aggregator::OptionalAggregator): bool
}
```


##### Implementation


```move
module 0x1::optional_aggregator {
    public fun is_parallelizable(optional_aggregator: &OptionalAggregator): bool {
        option::is_some(&optional_aggregator.aggregator)
    }
}
```


<a id="@Specification_1"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>When creating a new integer instance, it guarantees that the limit assigned is a value passed into the function as an argument, and the value field becomes zero.</td>
<td>High</td>
<td>The new_integer function sets the limit field to the argument passed in, and the value field is set to zero.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](new_integer).</td>
</tr>

<tr>
<td>2</td>
<td>For a given integer instance it should always be possible to: (1) return the limit value of the integer resource, (2) return the current value stored in that particular instance, and (3) destroy the integer instance.</td>
<td>Low</td>
<td>The following functions should not abort if the Integer instance exists: limit(), read_integer(), destroy_integer().</td>
<td>Formally verified via: [#high&#45;level&#45;req&#45;2.1](read_integer), [#high&#45;level&#45;req&#45;2.2](limit), and [#high&#45;level&#45;req&#45;2.3](destroy_integer).</td>
</tr>

<tr>
<td>3</td>
<td>Every successful switch must end with the aggregator type changed from non&#45;parallelizable to parallelizable or vice versa.</td>
<td>High</td>
<td>The switch function run, if successful, should always change the aggregator type.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](switch_and_zero_out).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::optional_aggregator {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_OptionalAggregator"></a>

### Struct `OptionalAggregator`


```move
module 0x1::optional_aggregator {
    struct OptionalAggregator has store
}
```


<dl>
<dt>
`aggregator: option::Option<aggregator::Aggregator>`
</dt>
<dd>

</dd>
<dt>
`integer: option::Option<optional_aggregator::Integer>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::optional_aggregator {
    invariant option::is_some(aggregator) <==> option::is_none(integer);
    invariant option::is_some(integer) <==> option::is_none(aggregator);
    invariant option::is_some(integer) ==> option::borrow(integer).value <= option::borrow(integer).limit;
    invariant option::is_some(aggregator) ==> aggregator::spec_aggregator_get_val(option::borrow(aggregator)) <=
        aggregator::spec_get_limit(option::borrow(aggregator));
}
```


<a id="@Specification_1_new_integer"></a>

### Function `new_integer`


```move
module 0x1::optional_aggregator {
    fun new_integer(limit: u128): optional_aggregator::Integer
}
```



```move
module 0x1::optional_aggregator {
    aborts_if false;
    ensures result.limit == limit;
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures result.value == 0;
}
```


<a id="@Specification_1_add_integer"></a>

### Function `add_integer`


```move
module 0x1::optional_aggregator {
    fun add_integer(integer: &mut optional_aggregator::Integer, value: u128)
}
```

Check for overflow.


```move
module 0x1::optional_aggregator {
    aborts_if value > (integer.limit - integer.value);
    aborts_if integer.value + value > MAX_U128;
    ensures integer.value <= integer.limit;
    ensures integer.value == old(integer.value) + value;
}
```


<a id="@Specification_1_sub_integer"></a>

### Function `sub_integer`


```move
module 0x1::optional_aggregator {
    fun sub_integer(integer: &mut optional_aggregator::Integer, value: u128)
}
```



```move
module 0x1::optional_aggregator {
    aborts_if value > integer.value;
    ensures integer.value == old(integer.value) - value;
}
```


<a id="@Specification_1_limit"></a>

### Function `limit`


```move
module 0x1::optional_aggregator {
    fun limit(integer: &optional_aggregator::Integer): u128
}
```



```move
module 0x1::optional_aggregator {
// This enforces ### high&#45;level&#45;req&#45;2.2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_read_integer"></a>

### Function `read_integer`


```move
module 0x1::optional_aggregator {
    fun read_integer(integer: &optional_aggregator::Integer): u128
}
```



```move
module 0x1::optional_aggregator {
// This enforces ### high&#45;level&#45;req&#45;2.1
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_destroy_integer"></a>

### Function `destroy_integer`


```move
module 0x1::optional_aggregator {
    fun destroy_integer(integer: optional_aggregator::Integer)
}
```



```move
module 0x1::optional_aggregator {
// This enforces ### high&#45;level&#45;req&#45;2.3
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_new"></a>

### Function `new`


```move
module 0x1::optional_aggregator {
    public(friend) fun new(limit: u128, parallelizable: bool): optional_aggregator::OptionalAggregator
}
```



```move
module 0x1::optional_aggregator {
    aborts_if parallelizable && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    ensures parallelizable ==> is_parallelizable(result);
    ensures !parallelizable ==> !is_parallelizable(result);
    ensures optional_aggregator_value(result) == 0;
    ensures optional_aggregator_value(result) <= optional_aggregator_limit(result);
}
```


<a id="@Specification_1_switch"></a>

### Function `switch`


```move
module 0x1::optional_aggregator {
    public fun switch(optional_aggregator: &mut optional_aggregator::OptionalAggregator)
}
```



```move
module 0x1::optional_aggregator {
    let vec_ref = optional_aggregator.integer.vec;
    aborts_if is_parallelizable(optional_aggregator) && len(vec_ref) != 0;
    aborts_if !is_parallelizable(optional_aggregator) && len(vec_ref) == 0;
    aborts_if !is_parallelizable(optional_aggregator) && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    ensures optional_aggregator_value(optional_aggregator) == optional_aggregator_value(old(optional_aggregator));
}
```


<a id="@Specification_1_switch_and_zero_out"></a>

### Function `switch_and_zero_out`


```move
module 0x1::optional_aggregator {
    fun switch_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator)
}
```

Option&lt;Integer&gt; does not exist When Option&lt;Aggregator&gt; exists.
Option&lt;Integer&gt; exists when Option&lt;Aggregator&gt; does not exist.
The AggregatorFactory is under the @aptos_framework when Option&lt;Aggregator&gt; does not exist.


```move
module 0x1::optional_aggregator {
    let vec_ref = optional_aggregator.integer.vec;
    aborts_if is_parallelizable(optional_aggregator) && len(vec_ref) != 0;
    aborts_if !is_parallelizable(optional_aggregator) && len(vec_ref) == 0;
    aborts_if !is_parallelizable(optional_aggregator) && !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures is_parallelizable(old(optional_aggregator)) ==> !is_parallelizable(optional_aggregator);
    ensures !is_parallelizable(old(optional_aggregator)) ==> is_parallelizable(optional_aggregator);
    ensures optional_aggregator_value(optional_aggregator) == 0;
}
```


<a id="@Specification_1_switch_to_integer_and_zero_out"></a>

### Function `switch_to_integer_and_zero_out`


```move
module 0x1::optional_aggregator {
    fun switch_to_integer_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator): u128
}
```

The aggregator exists and the integer dosex not exist when Switches from parallelizable to non&#45;parallelizable implementation.


```move
module 0x1::optional_aggregator {
    let limit = aggregator::spec_get_limit(option::borrow(optional_aggregator.aggregator));
    aborts_if len(optional_aggregator.aggregator.vec) == 0;
    aborts_if len(optional_aggregator.integer.vec) != 0;
    ensures !is_parallelizable(optional_aggregator);
    ensures option::borrow(optional_aggregator.integer).limit == limit;
    ensures option::borrow(optional_aggregator.integer).value == 0;
}
```


<a id="@Specification_1_switch_to_aggregator_and_zero_out"></a>

### Function `switch_to_aggregator_and_zero_out`


```move
module 0x1::optional_aggregator {
    fun switch_to_aggregator_and_zero_out(optional_aggregator: &mut optional_aggregator::OptionalAggregator): u128
}
```

The integer exists and the aggregator does not exist when Switches from non&#45;parallelizable to parallelizable implementation.
The AggregatorFactory is under the @aptos_framework.


```move
module 0x1::optional_aggregator {
    let limit = option::borrow(optional_aggregator.integer).limit;
    aborts_if len(optional_aggregator.integer.vec) == 0;
    aborts_if !exists<aggregator_factory::AggregatorFactory>(@aptos_framework);
    aborts_if len(optional_aggregator.aggregator.vec) != 0;
    ensures is_parallelizable(optional_aggregator);
    ensures aggregator::spec_get_limit(option::borrow(optional_aggregator.aggregator)) == limit;
    ensures aggregator::spec_aggregator_get_val(option::borrow(optional_aggregator.aggregator)) == 0;
}
```


<a id="@Specification_1_destroy"></a>

### Function `destroy`


```move
module 0x1::optional_aggregator {
    public fun destroy(optional_aggregator: optional_aggregator::OptionalAggregator)
}
```



```move
module 0x1::optional_aggregator {
    aborts_if is_parallelizable(optional_aggregator) && len(optional_aggregator.integer.vec) != 0;
    aborts_if !is_parallelizable(optional_aggregator) && len(optional_aggregator.integer.vec) == 0;
}
```


<a id="@Specification_1_destroy_optional_aggregator"></a>

### Function `destroy_optional_aggregator`


```move
module 0x1::optional_aggregator {
    fun destroy_optional_aggregator(optional_aggregator: optional_aggregator::OptionalAggregator): u128
}
```

The aggregator exists and the integer does not exist when destroy the aggregator.


```move
module 0x1::optional_aggregator {
    aborts_if len(optional_aggregator.aggregator.vec) == 0;
    aborts_if len(optional_aggregator.integer.vec) != 0;
    ensures result == aggregator::spec_get_limit(option::borrow(optional_aggregator.aggregator));
}
```


<a id="@Specification_1_destroy_optional_integer"></a>

### Function `destroy_optional_integer`


```move
module 0x1::optional_aggregator {
    fun destroy_optional_integer(optional_aggregator: optional_aggregator::OptionalAggregator): u128
}
```

The integer exists and the aggregator does not exist when destroy the integer.


```move
module 0x1::optional_aggregator {
    aborts_if len(optional_aggregator.integer.vec) == 0;
    aborts_if len(optional_aggregator.aggregator.vec) != 0;
    ensures result == option::borrow(optional_aggregator.integer).limit;
}
```



<a id="0x1_optional_aggregator_optional_aggregator_value"></a>


```move
module 0x1::optional_aggregator {
    fun optional_aggregator_value(optional_aggregator: OptionalAggregator): u128 {
       if (is_parallelizable(optional_aggregator)) {
           aggregator::spec_aggregator_get_val(option::borrow(optional_aggregator.aggregator))
       } else {
           option::borrow(optional_aggregator.integer).value
       }
    }
}
```



<a id="0x1_optional_aggregator_optional_aggregator_limit"></a>


```move
module 0x1::optional_aggregator {
    fun optional_aggregator_limit(optional_aggregator: OptionalAggregator): u128 {
       if (is_parallelizable(optional_aggregator)) {
           aggregator::spec_get_limit(option::borrow(optional_aggregator.aggregator))
       } else {
           option::borrow(optional_aggregator.integer).limit
       }
    }
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::optional_aggregator {
    public fun add(optional_aggregator: &mut optional_aggregator::OptionalAggregator, value: u128)
}
```



```move
module 0x1::optional_aggregator {
    include AddAbortsIf;
    ensures ((optional_aggregator_value(optional_aggregator) == optional_aggregator_value(old(optional_aggregator)) + value));
}
```



<a id="0x1_optional_aggregator_AddAbortsIf"></a>


```move
module 0x1::optional_aggregator {
    schema AddAbortsIf {
        optional_aggregator: OptionalAggregator;
        value: u128;
        aborts_if is_parallelizable(optional_aggregator) && (aggregator::spec_aggregator_get_val(option::borrow(optional_aggregator.aggregator))
            + value > aggregator::spec_get_limit(option::borrow(optional_aggregator.aggregator)));
        aborts_if is_parallelizable(optional_aggregator) && (aggregator::spec_aggregator_get_val(option::borrow(optional_aggregator.aggregator))
            + value > MAX_U128);
        aborts_if !is_parallelizable(optional_aggregator) &&
            (option::borrow(optional_aggregator.integer).value + value > MAX_U128);
        aborts_if !is_parallelizable(optional_aggregator) &&
            (value > (option::borrow(optional_aggregator.integer).limit - option::borrow(optional_aggregator.integer).value));
    }
}
```


<a id="@Specification_1_sub"></a>

### Function `sub`


```move
module 0x1::optional_aggregator {
    public fun sub(optional_aggregator: &mut optional_aggregator::OptionalAggregator, value: u128)
}
```



```move
module 0x1::optional_aggregator {
    include SubAbortsIf;
    ensures ((optional_aggregator_value(optional_aggregator) == optional_aggregator_value(old(optional_aggregator)) - value));
}
```



<a id="0x1_optional_aggregator_SubAbortsIf"></a>


```move
module 0x1::optional_aggregator {
    schema SubAbortsIf {
        optional_aggregator: OptionalAggregator;
        value: u128;
        aborts_if is_parallelizable(optional_aggregator) && (aggregator::spec_aggregator_get_val(option::borrow(optional_aggregator.aggregator))
            < value);
        aborts_if !is_parallelizable(optional_aggregator) &&
            (option::borrow(optional_aggregator.integer).value < value);
    }
}
```


<a id="@Specification_1_read"></a>

### Function `read`


```move
module 0x1::optional_aggregator {
    public fun read(optional_aggregator: &optional_aggregator::OptionalAggregator): u128
}
```



```move
module 0x1::optional_aggregator {
    ensures !is_parallelizable(optional_aggregator) ==> result == option::borrow(optional_aggregator.integer).value;
    ensures is_parallelizable(optional_aggregator) ==>
        result == aggregator::spec_read(option::borrow(optional_aggregator.aggregator));
}
```
