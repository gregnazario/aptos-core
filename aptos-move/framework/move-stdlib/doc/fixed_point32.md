
<a id="0x1_fixed_point32"></a>

# Module `0x1::fixed_point32`

Defines a fixed&#45;point numeric type with a 32&#45;bit integer part and
a 32&#45;bit fractional part.


-  [Struct `FixedPoint32`](#0x1_fixed_point32_FixedPoint32)
-  [Constants](#@Constants_0)
-  [Function `multiply_u64`](#0x1_fixed_point32_multiply_u64)
-  [Function `divide_u64`](#0x1_fixed_point32_divide_u64)
-  [Function `create_from_rational`](#0x1_fixed_point32_create_from_rational)
-  [Function `create_from_raw_value`](#0x1_fixed_point32_create_from_raw_value)
-  [Function `get_raw_value`](#0x1_fixed_point32_get_raw_value)
-  [Function `is_zero`](#0x1_fixed_point32_is_zero)
-  [Function `min`](#0x1_fixed_point32_min)
-  [Function `max`](#0x1_fixed_point32_max)
-  [Function `create_from_u64`](#0x1_fixed_point32_create_from_u64)
-  [Function `floor`](#0x1_fixed_point32_floor)
-  [Function `ceil`](#0x1_fixed_point32_ceil)
-  [Function `round`](#0x1_fixed_point32_round)
-  [Specification](#@Specification_1)
    -  [Function `multiply_u64`](#@Specification_1_multiply_u64)
    -  [Function `divide_u64`](#@Specification_1_divide_u64)
    -  [Function `create_from_rational`](#@Specification_1_create_from_rational)
    -  [Function `create_from_raw_value`](#@Specification_1_create_from_raw_value)
    -  [Function `min`](#@Specification_1_min)
    -  [Function `max`](#@Specification_1_max)
    -  [Function `create_from_u64`](#@Specification_1_create_from_u64)
    -  [Function `floor`](#@Specification_1_floor)
    -  [Function `ceil`](#@Specification_1_ceil)
    -  [Function `round`](#@Specification_1_round)


```move
module 0x1::fixed_point32 {
}
```


<a id="0x1_fixed_point32_FixedPoint32"></a>

## Struct `FixedPoint32`

Define a fixed&#45;point numeric type with 32 fractional bits.
This is just a u64 integer but it is wrapped in a struct to
make a unique type. This is a binary representation, so decimal
values may not be exactly representable, but it provides more
than 9 decimal digits of precision both before and after the
decimal point (18 digits total). For comparison, double precision
floating&#45;point has less than 16 decimal digits of precision, so
be careful about using floating&#45;point to convert these values to
decimal.


```move
module 0x1::fixed_point32 {
    struct FixedPoint32 has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`value: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_fixed_point32_MAX_U64"></a>



```move
module 0x1::fixed_point32 {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_fixed_point32_EDENOMINATOR"></a>

The denominator provided was zero


```move
module 0x1::fixed_point32 {
    const EDENOMINATOR: u64 = 65537;
}
```


<a id="0x1_fixed_point32_EDIVISION"></a>

The quotient value would be too large to be held in a `u64`


```move
module 0x1::fixed_point32 {
    const EDIVISION: u64 = 131074;
}
```


<a id="0x1_fixed_point32_EDIVISION_BY_ZERO"></a>

A division by zero was encountered


```move
module 0x1::fixed_point32 {
    const EDIVISION_BY_ZERO: u64 = 65540;
}
```


<a id="0x1_fixed_point32_EMULTIPLICATION"></a>

The multiplied value would be too large to be held in a `u64`


```move
module 0x1::fixed_point32 {
    const EMULTIPLICATION: u64 = 131075;
}
```


<a id="0x1_fixed_point32_ERATIO_OUT_OF_RANGE"></a>

The computed ratio when converting to a `FixedPoint32` would be unrepresentable


```move
module 0x1::fixed_point32 {
    const ERATIO_OUT_OF_RANGE: u64 = 131077;
}
```


<a id="0x1_fixed_point32_multiply_u64"></a>

## Function `multiply_u64`

Multiply a u64 integer by a fixed&#45;point number, truncating any
fractional part of the product. This will abort if the product
overflows.


```move
module 0x1::fixed_point32 {
    public fun multiply_u64(val: u64, multiplier: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun multiply_u64(val: u64, multiplier: FixedPoint32): u64 {
        // The product of two 64 bit values has 128 bits, so perform the
        // multiplication with u128 types and keep the full 128 bit product
        // to avoid losing accuracy.
        let unscaled_product = (val as u128) * (multiplier.value as u128);
        // The unscaled product has 32 fractional bits (from the multiplier)
        // so rescale it by shifting away the low bits.
        let product = unscaled_product >> 32;
        // Check whether the value is too large.
        assert!(product <= MAX_U64, EMULTIPLICATION);
        (product as u64)
    }
}
```


<a id="0x1_fixed_point32_divide_u64"></a>

## Function `divide_u64`

Divide a u64 integer by a fixed&#45;point number, truncating any
fractional part of the quotient. This will abort if the divisor
is zero or if the quotient overflows.


```move
module 0x1::fixed_point32 {
    public fun divide_u64(val: u64, divisor: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun divide_u64(val: u64, divisor: FixedPoint32): u64 {
        // Check for division by zero.
        assert!(divisor.value != 0, EDIVISION_BY_ZERO);
        // First convert to 128 bits and then shift left to
        // add 32 fractional zero bits to the dividend.
        let scaled_value = (val as u128) << 32;
        let quotient = scaled_value / (divisor.value as u128);
        // Check whether the value is too large.
        assert!(quotient <= MAX_U64, EDIVISION);
        // the value may be too large, which will cause the cast to fail
        // with an arithmetic error.
        (quotient as u64)
    }
}
```


<a id="0x1_fixed_point32_create_from_rational"></a>

## Function `create_from_rational`

Create a fixed&#45;point value from a rational number specified by its
numerator and denominator. Calling this function should be preferred
for using `Self::create_from_raw_value` which is also available.
This will abort if the denominator is zero. It will also
abort if the numerator is nonzero and the ratio is not in the range
2^&#45;32 .. 2^32&#45;1. When specifying decimal fractions, be careful about
rounding errors: if you round to display N digits after the decimal
point, you can use a denominator of 10^N to avoid numbers where the
very small imprecision in the binary representation could change the
rounding, e.g., 0.0125 will round down to 0.012 instead of up to 0.013.


```move
module 0x1::fixed_point32 {
    public fun create_from_rational(numerator: u64, denominator: u64): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun create_from_rational(numerator: u64, denominator: u64): FixedPoint32 {
        // If the denominator is zero, this will abort.
        // Scale the numerator to have 64 fractional bits and the denominator
        // to have 32 fractional bits, so that the quotient will have 32
        // fractional bits.
        let scaled_numerator = (numerator as u128) << 64;
        let scaled_denominator = (denominator as u128) << 32;
        assert!(scaled_denominator != 0, EDENOMINATOR);
        let quotient = scaled_numerator / scaled_denominator;
        assert!(quotient != 0 || numerator == 0, ERATIO_OUT_OF_RANGE);
        // Return the quotient as a fixed-point number. We first need to check whether the cast
        // can succeed.
        assert!(quotient <= MAX_U64, ERATIO_OUT_OF_RANGE);
        FixedPoint32 { value: (quotient as u64) }
    }
}
```


<a id="0x1_fixed_point32_create_from_raw_value"></a>

## Function `create_from_raw_value`

Create a fixedpoint value from a raw value.


```move
module 0x1::fixed_point32 {
    public fun create_from_raw_value(value: u64): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun create_from_raw_value(value: u64): FixedPoint32 {
        FixedPoint32 { value }
    }
}
```


<a id="0x1_fixed_point32_get_raw_value"></a>

## Function `get_raw_value`

Accessor for the raw u64 value. Other less common operations, such as
adding or subtracting FixedPoint32 values, can be done using the raw
values directly.


```move
module 0x1::fixed_point32 {
    public fun get_raw_value(num: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun get_raw_value(num: FixedPoint32): u64 {
        num.value
    }
}
```


<a id="0x1_fixed_point32_is_zero"></a>

## Function `is_zero`

Returns true if the ratio is zero.


```move
module 0x1::fixed_point32 {
    public fun is_zero(num: fixed_point32::FixedPoint32): bool
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun is_zero(num: FixedPoint32): bool {
        num.value == 0
    }
}
```


<a id="0x1_fixed_point32_min"></a>

## Function `min`

Returns the smaller of the two FixedPoint32 numbers.


```move
module 0x1::fixed_point32 {
    public fun min(num1: fixed_point32::FixedPoint32, num2: fixed_point32::FixedPoint32): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun min(num1: FixedPoint32, num2: FixedPoint32): FixedPoint32 {
        if (num1.value < num2.value) {
            num1
        } else {
            num2
        }
    }
}
```


<a id="0x1_fixed_point32_max"></a>

## Function `max`

Returns the larger of the two FixedPoint32 numbers.


```move
module 0x1::fixed_point32 {
    public fun max(num1: fixed_point32::FixedPoint32, num2: fixed_point32::FixedPoint32): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun max(num1: FixedPoint32, num2: FixedPoint32): FixedPoint32 {
        if (num1.value > num2.value) {
            num1
        } else {
            num2
        }
    }
}
```


<a id="0x1_fixed_point32_create_from_u64"></a>

## Function `create_from_u64`

Create a fixedpoint value from a u64 value.


```move
module 0x1::fixed_point32 {
    public fun create_from_u64(val: u64): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun create_from_u64(val: u64): FixedPoint32 {
        let value = (val as u128) << 32;
        assert!(value <= MAX_U64, ERATIO_OUT_OF_RANGE);
        FixedPoint32 {value: (value as u64)}
    }
}
```


<a id="0x1_fixed_point32_floor"></a>

## Function `floor`

Returns the largest integer less than or equal to a given number.


```move
module 0x1::fixed_point32 {
    public fun floor(num: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun floor(num: FixedPoint32): u64 {
        num.value >> 32
    }
}
```


<a id="0x1_fixed_point32_ceil"></a>

## Function `ceil`

Rounds up the given FixedPoint32 to the next largest integer.


```move
module 0x1::fixed_point32 {
    public fun ceil(num: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun ceil(num: FixedPoint32): u64 {
        let floored_num = floor(num) << 32;
        if (num.value == floored_num) {
            return floored_num >> 32
        };
        let val = ((floored_num as u128) + (1 << 32));
        (val >> 32 as u64)
    }
}
```


<a id="0x1_fixed_point32_round"></a>

## Function `round`

Returns the value of a FixedPoint32 to the nearest integer.


```move
module 0x1::fixed_point32 {
    public fun round(num: fixed_point32::FixedPoint32): u64
}
```


##### Implementation


```move
module 0x1::fixed_point32 {
    public fun round(num: FixedPoint32): u64 {
        let floored_num = floor(num) << 32;
        let boundary = floored_num + ((1 << 32) / 2);
        if (num.value < boundary) {
            floored_num >> 32
        } else {
            ceil(num)
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification




```move
module 0x1::fixed_point32 {
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_multiply_u64"></a>

### Function `multiply_u64`


```move
module 0x1::fixed_point32 {
    public fun multiply_u64(val: u64, multiplier: fixed_point32::FixedPoint32): u64
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    include MultiplyAbortsIf;
    ensures result == spec_multiply_u64(val, multiplier);
}
```



<a id="0x1_fixed_point32_MultiplyAbortsIf"></a>


```move
module 0x1::fixed_point32 {
    schema MultiplyAbortsIf {
        val: num;
        multiplier: FixedPoint32;
        aborts_if spec_multiply_u64(val, multiplier) > MAX_U64 with EMULTIPLICATION;
    }
}
```



<a id="0x1_fixed_point32_spec_multiply_u64"></a>


```move
module 0x1::fixed_point32 {
    fun spec_multiply_u64(val: num, multiplier: FixedPoint32): num {
       (val * multiplier.value) >> 32
    }
}
```


<a id="@Specification_1_divide_u64"></a>

### Function `divide_u64`


```move
module 0x1::fixed_point32 {
    public fun divide_u64(val: u64, divisor: fixed_point32::FixedPoint32): u64
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    include DivideAbortsIf;
    ensures result == spec_divide_u64(val, divisor);
}
```



<a id="0x1_fixed_point32_DivideAbortsIf"></a>


```move
module 0x1::fixed_point32 {
    schema DivideAbortsIf {
        val: num;
        divisor: FixedPoint32;
        aborts_if divisor.value == 0 with EDIVISION_BY_ZERO;
        aborts_if spec_divide_u64(val, divisor) > MAX_U64 with EDIVISION;
    }
}
```



<a id="0x1_fixed_point32_spec_divide_u64"></a>


```move
module 0x1::fixed_point32 {
    fun spec_divide_u64(val: num, divisor: FixedPoint32): num {
       (val << 32) / divisor.value
    }
}
```


<a id="@Specification_1_create_from_rational"></a>

### Function `create_from_rational`


```move
module 0x1::fixed_point32 {
    public fun create_from_rational(numerator: u64, denominator: u64): fixed_point32::FixedPoint32
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    include CreateFromRationalAbortsIf;
    ensures result == spec_create_from_rational(numerator, denominator);
}
```



<a id="0x1_fixed_point32_CreateFromRationalAbortsIf"></a>


```move
module 0x1::fixed_point32 {
    schema CreateFromRationalAbortsIf {
        numerator: u64;
        denominator: u64;
        let scaled_numerator = (numerator as u128)<< 64;
        let scaled_denominator = (denominator as u128) << 32;
        let quotient = scaled_numerator / scaled_denominator;
        aborts_if scaled_denominator == 0 with EDENOMINATOR;
        aborts_if quotient == 0 && scaled_numerator != 0 with ERATIO_OUT_OF_RANGE;
        aborts_if quotient > MAX_U64 with ERATIO_OUT_OF_RANGE;
    }
}
```



<a id="0x1_fixed_point32_spec_create_from_rational"></a>


```move
module 0x1::fixed_point32 {
    fun spec_create_from_rational(numerator: num, denominator: num): FixedPoint32 {
       FixedPoint32{value: (numerator << 64) / (denominator << 32)}
    }
}
```


<a id="@Specification_1_create_from_raw_value"></a>

### Function `create_from_raw_value`


```move
module 0x1::fixed_point32 {
    public fun create_from_raw_value(value: u64): fixed_point32::FixedPoint32
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    aborts_if false;
    ensures result.value == value;
}
```


<a id="@Specification_1_min"></a>

### Function `min`


```move
module 0x1::fixed_point32 {
    public fun min(num1: fixed_point32::FixedPoint32, num2: fixed_point32::FixedPoint32): fixed_point32::FixedPoint32
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_min(num1, num2);
}
```



<a id="0x1_fixed_point32_spec_min"></a>


```move
module 0x1::fixed_point32 {
    fun spec_min(num1: FixedPoint32, num2: FixedPoint32): FixedPoint32 {
       if (num1.value < num2.value) {
           num1
       } else {
           num2
       }
    }
}
```


<a id="@Specification_1_max"></a>

### Function `max`


```move
module 0x1::fixed_point32 {
    public fun max(num1: fixed_point32::FixedPoint32, num2: fixed_point32::FixedPoint32): fixed_point32::FixedPoint32
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_max(num1, num2);
}
```



<a id="0x1_fixed_point32_spec_max"></a>


```move
module 0x1::fixed_point32 {
    fun spec_max(num1: FixedPoint32, num2: FixedPoint32): FixedPoint32 {
       if (num1.value > num2.value) {
           num1
       } else {
           num2
       }
    }
}
```


<a id="@Specification_1_create_from_u64"></a>

### Function `create_from_u64`


```move
module 0x1::fixed_point32 {
    public fun create_from_u64(val: u64): fixed_point32::FixedPoint32
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    include CreateFromU64AbortsIf;
    ensures result == spec_create_from_u64(val);
}
```



<a id="0x1_fixed_point32_CreateFromU64AbortsIf"></a>


```move
module 0x1::fixed_point32 {
    schema CreateFromU64AbortsIf {
        val: num;
        let scaled_value = (val as u128) << 32;
        aborts_if scaled_value > MAX_U64;
    }
}
```



<a id="0x1_fixed_point32_spec_create_from_u64"></a>


```move
module 0x1::fixed_point32 {
    fun spec_create_from_u64(val: num): FixedPoint32 {
       FixedPoint32 {value: val << 32}
    }
}
```


<a id="@Specification_1_floor"></a>

### Function `floor`


```move
module 0x1::fixed_point32 {
    public fun floor(num: fixed_point32::FixedPoint32): u64
}
```



```move
module 0x1::fixed_point32 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_floor(num);
}
```



<a id="0x1_fixed_point32_spec_floor"></a>


```move
module 0x1::fixed_point32 {
    fun spec_floor(val: FixedPoint32): u64 {
       let fractional = val.value % (1 << 32);
       if (fractional == 0) {
           val.value >> 32
       } else {
           (val.value - fractional) >> 32
       }
    }
}
```


<a id="@Specification_1_ceil"></a>

### Function `ceil`


```move
module 0x1::fixed_point32 {
    public fun ceil(num: fixed_point32::FixedPoint32): u64
}
```



```move
module 0x1::fixed_point32 {
    pragma verify_duration_estimate = 120;
    pragma opaque;
    aborts_if false;
    ensures result == spec_ceil(num);
}
```



<a id="0x1_fixed_point32_spec_ceil"></a>


```move
module 0x1::fixed_point32 {
    fun spec_ceil(val: FixedPoint32): u64 {
       let fractional = val.value % (1 << 32);
       let one = 1 << 32;
       if (fractional == 0) {
           val.value >> 32
       } else {
           (val.value - fractional + one) >> 32
       }
    }
}
```


<a id="@Specification_1_round"></a>

### Function `round`


```move
module 0x1::fixed_point32 {
    public fun round(num: fixed_point32::FixedPoint32): u64
}
```



```move
module 0x1::fixed_point32 {
    pragma verify_duration_estimate = 120;
    pragma opaque;
    aborts_if false;
    ensures result == spec_round(num);
}
```



<a id="0x1_fixed_point32_spec_round"></a>


```move
module 0x1::fixed_point32 {
    fun spec_round(val: FixedPoint32): u64 {
       let fractional = val.value % (1 << 32);
       let boundary = (1 << 32) / 2;
       let one = 1 << 32;
       if (fractional < boundary) {
           (val.value - fractional) >> 32
       } else {
           (val.value - fractional + one) >> 32
       }
    }
}
```
