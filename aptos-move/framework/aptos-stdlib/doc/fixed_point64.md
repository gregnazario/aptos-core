
<a id="0x1_fixed_point64"></a>

# Module `0x1::fixed_point64`

Defines a fixed&#45;point numeric type with a 64&#45;bit integer part and
a 64&#45;bit fractional part.


-  [Struct `FixedPoint64`](#0x1_fixed_point64_FixedPoint64)
-  [Constants](#@Constants_0)
-  [Function `sub`](#0x1_fixed_point64_sub)
-  [Function `add`](#0x1_fixed_point64_add)
-  [Function `multiply_u128`](#0x1_fixed_point64_multiply_u128)
-  [Function `divide_u128`](#0x1_fixed_point64_divide_u128)
-  [Function `create_from_rational`](#0x1_fixed_point64_create_from_rational)
-  [Function `create_from_raw_value`](#0x1_fixed_point64_create_from_raw_value)
-  [Function `get_raw_value`](#0x1_fixed_point64_get_raw_value)
-  [Function `is_zero`](#0x1_fixed_point64_is_zero)
-  [Function `min`](#0x1_fixed_point64_min)
-  [Function `max`](#0x1_fixed_point64_max)
-  [Function `less_or_equal`](#0x1_fixed_point64_less_or_equal)
-  [Function `less`](#0x1_fixed_point64_less)
-  [Function `greater_or_equal`](#0x1_fixed_point64_greater_or_equal)
-  [Function `greater`](#0x1_fixed_point64_greater)
-  [Function `equal`](#0x1_fixed_point64_equal)
-  [Function `almost_equal`](#0x1_fixed_point64_almost_equal)
-  [Function `create_from_u128`](#0x1_fixed_point64_create_from_u128)
-  [Function `floor`](#0x1_fixed_point64_floor)
-  [Function `ceil`](#0x1_fixed_point64_ceil)
-  [Function `round`](#0x1_fixed_point64_round)
-  [Specification](#@Specification_1)
    -  [Function `sub`](#@Specification_1_sub)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `multiply_u128`](#@Specification_1_multiply_u128)
    -  [Function `divide_u128`](#@Specification_1_divide_u128)
    -  [Function `create_from_rational`](#@Specification_1_create_from_rational)
    -  [Function `create_from_raw_value`](#@Specification_1_create_from_raw_value)
    -  [Function `min`](#@Specification_1_min)
    -  [Function `max`](#@Specification_1_max)
    -  [Function `less_or_equal`](#@Specification_1_less_or_equal)
    -  [Function `less`](#@Specification_1_less)
    -  [Function `greater_or_equal`](#@Specification_1_greater_or_equal)
    -  [Function `greater`](#@Specification_1_greater)
    -  [Function `equal`](#@Specification_1_equal)
    -  [Function `almost_equal`](#@Specification_1_almost_equal)
    -  [Function `create_from_u128`](#@Specification_1_create_from_u128)
    -  [Function `floor`](#@Specification_1_floor)
    -  [Function `ceil`](#@Specification_1_ceil)
    -  [Function `round`](#@Specification_1_round)


```move
module 0x1::fixed_point64 {
}
```


<a id="0x1_fixed_point64_FixedPoint64"></a>

## Struct `FixedPoint64`

Define a fixed&#45;point numeric type with 64 fractional bits.
This is just a u128 integer but it is wrapped in a struct to
make a unique type. This is a binary representation, so decimal
values may not be exactly representable, but it provides more
than 9 decimal digits of precision both before and after the
decimal point (18 digits total). For comparison, double precision
floating&#45;point has less than 16 decimal digits of precision, so
be careful about using floating&#45;point to convert these values to
decimal.


```move
module 0x1::fixed_point64 {
    struct FixedPoint64 has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`value: u128`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_fixed_point64_MAX_U128"></a>



```move
module 0x1::fixed_point64 {
    const MAX_U128: u256 = 340282366920938463463374607431768211455;
}
```


<a id="0x1_fixed_point64_EDENOMINATOR"></a>

The denominator provided was zero


```move
module 0x1::fixed_point64 {
    const EDENOMINATOR: u64 = 65537;
}
```


<a id="0x1_fixed_point64_EDIVISION"></a>

The quotient value would be too large to be held in a `u128`


```move
module 0x1::fixed_point64 {
    const EDIVISION: u64 = 131074;
}
```


<a id="0x1_fixed_point64_EDIVISION_BY_ZERO"></a>

A division by zero was encountered


```move
module 0x1::fixed_point64 {
    const EDIVISION_BY_ZERO: u64 = 65540;
}
```


<a id="0x1_fixed_point64_EMULTIPLICATION"></a>

The multiplied value would be too large to be held in a `u128`


```move
module 0x1::fixed_point64 {
    const EMULTIPLICATION: u64 = 131075;
}
```


<a id="0x1_fixed_point64_ENEGATIVE_RESULT"></a>

Abort code on calculation result is negative.


```move
module 0x1::fixed_point64 {
    const ENEGATIVE_RESULT: u64 = 65542;
}
```


<a id="0x1_fixed_point64_ERATIO_OUT_OF_RANGE"></a>

The computed ratio when converting to a `FixedPoint64` would be unrepresentable


```move
module 0x1::fixed_point64 {
    const ERATIO_OUT_OF_RANGE: u64 = 131077;
}
```


<a id="0x1_fixed_point64_sub"></a>

## Function `sub`

Returns x &#45; y. x must be not less than y.


```move
module 0x1::fixed_point64 {
    public fun sub(x: fixed_point64::FixedPoint64, y: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun sub(x: FixedPoint64, y: FixedPoint64): FixedPoint64 {
        let x_raw = get_raw_value(x);
        let y_raw = get_raw_value(y);
        assert!(x_raw >= y_raw, ENEGATIVE_RESULT);
        create_from_raw_value(x_raw - y_raw)
    }
}
```


<a id="0x1_fixed_point64_add"></a>

## Function `add`

Returns x &#43; y. The result cannot be greater than MAX_U128.


```move
module 0x1::fixed_point64 {
    public fun add(x: fixed_point64::FixedPoint64, y: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun add(x: FixedPoint64, y: FixedPoint64): FixedPoint64 {
        let x_raw = get_raw_value(x);
        let y_raw = get_raw_value(y);
        let result = (x_raw as u256) + (y_raw as u256);
        assert!(result <= MAX_U128, ERATIO_OUT_OF_RANGE);
        create_from_raw_value((result as u128))
    }
}
```


<a id="0x1_fixed_point64_multiply_u128"></a>

## Function `multiply_u128`

Multiply a u128 integer by a fixed&#45;point number, truncating any
fractional part of the product. This will abort if the product
overflows.


```move
module 0x1::fixed_point64 {
    public fun multiply_u128(val: u128, multiplier: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun multiply_u128(val: u128, multiplier: FixedPoint64): u128 {
        // The product of two 128 bit values has 256 bits, so perform the
        // multiplication with u256 types and keep the full 256 bit product
        // to avoid losing accuracy.
        let unscaled_product = (val as u256) * (multiplier.value as u256);
        // The unscaled product has 64 fractional bits (from the multiplier)
        // so rescale it by shifting away the low bits.
        let product = unscaled_product >> 64;
        // Check whether the value is too large.
        assert!(product <= MAX_U128, EMULTIPLICATION);
        (product as u128)
    }
}
```


<a id="0x1_fixed_point64_divide_u128"></a>

## Function `divide_u128`

Divide a u128 integer by a fixed&#45;point number, truncating any
fractional part of the quotient. This will abort if the divisor
is zero or if the quotient overflows.


```move
module 0x1::fixed_point64 {
    public fun divide_u128(val: u128, divisor: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun divide_u128(val: u128, divisor: FixedPoint64): u128 {
        // Check for division by zero.
        assert!(divisor.value != 0, EDIVISION_BY_ZERO);
        // First convert to 256 bits and then shift left to
        // add 64 fractional zero bits to the dividend.
        let scaled_value = (val as u256) << 64;
        let quotient = scaled_value / (divisor.value as u256);
        // Check whether the value is too large.
        assert!(quotient <= MAX_U128, EDIVISION);
        // the value may be too large, which will cause the cast to fail
        // with an arithmetic error.
        (quotient as u128)
    }
}
```


<a id="0x1_fixed_point64_create_from_rational"></a>

## Function `create_from_rational`

Create a fixed&#45;point value from a rational number specified by its
numerator and denominator. Calling this function should be preferred
for using `Self::create_from_raw_value` which is also available.
This will abort if the denominator is zero. It will also
abort if the numerator is nonzero and the ratio is not in the range
2^&#45;64 .. 2^64&#45;1. When specifying decimal fractions, be careful about
rounding errors: if you round to display N digits after the decimal
point, you can use a denominator of 10^N to avoid numbers where the
very small imprecision in the binary representation could change the
rounding, e.g., 0.0125 will round down to 0.012 instead of up to 0.013.


```move
module 0x1::fixed_point64 {
    public fun create_from_rational(numerator: u128, denominator: u128): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun create_from_rational(numerator: u128, denominator: u128): FixedPoint64 {
        // If the denominator is zero, this will abort.
        // Scale the numerator to have 64 fractional bits, so that the quotient will have 64
        // fractional bits.
        let scaled_numerator = (numerator as u256) << 64;
        assert!(denominator != 0, EDENOMINATOR);
        let quotient = scaled_numerator / (denominator as u256);
        assert!(quotient != 0 || numerator == 0, ERATIO_OUT_OF_RANGE);
        // Return the quotient as a fixed-point number. We first need to check whether the cast
        // can succeed.
        assert!(quotient <= MAX_U128, ERATIO_OUT_OF_RANGE);
        FixedPoint64 { value: (quotient as u128) }
    }
}
```


<a id="0x1_fixed_point64_create_from_raw_value"></a>

## Function `create_from_raw_value`

Create a fixedpoint value from a raw value.


```move
module 0x1::fixed_point64 {
    public fun create_from_raw_value(value: u128): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun create_from_raw_value(value: u128): FixedPoint64 {
        FixedPoint64 { value }
    }
}
```


<a id="0x1_fixed_point64_get_raw_value"></a>

## Function `get_raw_value`

Accessor for the raw u128 value. Other less common operations, such as
adding or subtracting FixedPoint64 values, can be done using the raw
values directly.


```move
module 0x1::fixed_point64 {
    public fun get_raw_value(num: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun get_raw_value(num: FixedPoint64): u128 {
        num.value
    }
}
```


<a id="0x1_fixed_point64_is_zero"></a>

## Function `is_zero`

Returns true if the ratio is zero.


```move
module 0x1::fixed_point64 {
    public fun is_zero(num: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun is_zero(num: FixedPoint64): bool {
        num.value == 0
    }
}
```


<a id="0x1_fixed_point64_min"></a>

## Function `min`

Returns the smaller of the two FixedPoint64 numbers.


```move
module 0x1::fixed_point64 {
    public fun min(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun min(num1: FixedPoint64, num2: FixedPoint64): FixedPoint64 {
        if (num1.value < num2.value) {
            num1
        } else {
            num2
        }
    }
}
```


<a id="0x1_fixed_point64_max"></a>

## Function `max`

Returns the larger of the two FixedPoint64 numbers.


```move
module 0x1::fixed_point64 {
    public fun max(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun max(num1: FixedPoint64, num2: FixedPoint64): FixedPoint64 {
        if (num1.value > num2.value) {
            num1
        } else {
            num2
        }
    }
}
```


<a id="0x1_fixed_point64_less_or_equal"></a>

## Function `less_or_equal`

Returns true if num1 &lt;&#61; num2


```move
module 0x1::fixed_point64 {
    public fun less_or_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun less_or_equal(num1: FixedPoint64, num2: FixedPoint64): bool {
        num1.value <= num2.value
    }
}
```


<a id="0x1_fixed_point64_less"></a>

## Function `less`

Returns true if num1 &lt; num2


```move
module 0x1::fixed_point64 {
    public fun less(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun less(num1: FixedPoint64, num2: FixedPoint64): bool {
        num1.value < num2.value
    }
}
```


<a id="0x1_fixed_point64_greater_or_equal"></a>

## Function `greater_or_equal`

Returns true if num1 &gt;&#61; num2


```move
module 0x1::fixed_point64 {
    public fun greater_or_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun greater_or_equal(num1: FixedPoint64, num2: FixedPoint64): bool {
        num1.value >= num2.value
    }
}
```


<a id="0x1_fixed_point64_greater"></a>

## Function `greater`

Returns true if num1 &gt; num2


```move
module 0x1::fixed_point64 {
    public fun greater(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun greater(num1: FixedPoint64, num2: FixedPoint64): bool {
        num1.value > num2.value
    }
}
```


<a id="0x1_fixed_point64_equal"></a>

## Function `equal`

Returns true if num1 &#61; num2


```move
module 0x1::fixed_point64 {
    public fun equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun equal(num1: FixedPoint64, num2: FixedPoint64): bool {
        num1.value == num2.value
    }
}
```


<a id="0x1_fixed_point64_almost_equal"></a>

## Function `almost_equal`

Returns true if num1 almost equals to num2, which means abs(num1&#45;num2) &lt;&#61; precision


```move
module 0x1::fixed_point64 {
    public fun almost_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64, precision: fixed_point64::FixedPoint64): bool
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun almost_equal(num1: FixedPoint64, num2: FixedPoint64, precision: FixedPoint64): bool {
        if (num1.value > num2.value) {
            (num1.value - num2.value <= precision.value)
        } else {
            (num2.value - num1.value <= precision.value)
        }
    }
}
```


<a id="0x1_fixed_point64_create_from_u128"></a>

## Function `create_from_u128`

Create a fixedpoint value from a u128 value.


```move
module 0x1::fixed_point64 {
    public fun create_from_u128(val: u128): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun create_from_u128(val: u128): FixedPoint64 {
        let value = (val as u256) << 64;
        assert!(value <= MAX_U128, ERATIO_OUT_OF_RANGE);
        FixedPoint64 {value: (value as u128)}
    }
}
```


<a id="0x1_fixed_point64_floor"></a>

## Function `floor`

Returns the largest integer less than or equal to a given number.


```move
module 0x1::fixed_point64 {
    public fun floor(num: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun floor(num: FixedPoint64): u128 {
        num.value >> 64
    }
}
```


<a id="0x1_fixed_point64_ceil"></a>

## Function `ceil`

Rounds up the given FixedPoint64 to the next largest integer.


```move
module 0x1::fixed_point64 {
    public fun ceil(num: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun ceil(num: FixedPoint64): u128 {
        let floored_num = floor(num) << 64;
        if (num.value == floored_num) {
            return floored_num >> 64
        };
        let val = ((floored_num as u256) + (1 << 64));
        (val >> 64 as u128)
    }
}
```


<a id="0x1_fixed_point64_round"></a>

## Function `round`

Returns the value of a FixedPoint64 to the nearest integer.


```move
module 0x1::fixed_point64 {
    public fun round(num: fixed_point64::FixedPoint64): u128
}
```


##### Implementation


```move
module 0x1::fixed_point64 {
    public fun round(num: FixedPoint64): u128 {
        let floored_num = floor(num) << 64;
        let boundary = floored_num + ((1 << 64) / 2);
        if (num.value < boundary) {
            floored_num >> 64
        } else {
            ceil(num)
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification




```move
module 0x1::fixed_point64 {
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_sub"></a>

### Function `sub`


```move
module 0x1::fixed_point64 {
    public fun sub(x: fixed_point64::FixedPoint64, y: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if x.value < y.value with ENEGATIVE_RESULT;
    ensures result.value == x.value - y.value;
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::fixed_point64 {
    public fun add(x: fixed_point64::FixedPoint64, y: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if (x.value as u256) + (y.value as u256) > MAX_U128 with ERATIO_OUT_OF_RANGE;
    ensures result.value == x.value + y.value;
}
```


<a id="@Specification_1_multiply_u128"></a>

### Function `multiply_u128`


```move
module 0x1::fixed_point64 {
    public fun multiply_u128(val: u128, multiplier: fixed_point64::FixedPoint64): u128
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    include MultiplyAbortsIf;
    ensures result == spec_multiply_u128(val, multiplier);
}
```



<a id="0x1_fixed_point64_MultiplyAbortsIf"></a>


```move
module 0x1::fixed_point64 {
    schema MultiplyAbortsIf {
        val: num;
        multiplier: FixedPoint64;
        aborts_if spec_multiply_u128(val, multiplier) > MAX_U128 with EMULTIPLICATION;
    }
}
```



<a id="0x1_fixed_point64_spec_multiply_u128"></a>


```move
module 0x1::fixed_point64 {
    fun spec_multiply_u128(val: num, multiplier: FixedPoint64): num {
       (val * multiplier.value) >> 64
    }
}
```


<a id="@Specification_1_divide_u128"></a>

### Function `divide_u128`


```move
module 0x1::fixed_point64 {
    public fun divide_u128(val: u128, divisor: fixed_point64::FixedPoint64): u128
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    include DivideAbortsIf;
    ensures result == spec_divide_u128(val, divisor);
}
```



<a id="0x1_fixed_point64_DivideAbortsIf"></a>


```move
module 0x1::fixed_point64 {
    schema DivideAbortsIf {
        val: num;
        divisor: FixedPoint64;
        aborts_if divisor.value == 0 with EDIVISION_BY_ZERO;
        aborts_if spec_divide_u128(val, divisor) > MAX_U128 with EDIVISION;
    }
}
```



<a id="0x1_fixed_point64_spec_divide_u128"></a>


```move
module 0x1::fixed_point64 {
    fun spec_divide_u128(val: num, divisor: FixedPoint64): num {
       (val << 64) / divisor.value
    }
}
```


<a id="@Specification_1_create_from_rational"></a>

### Function `create_from_rational`


```move
module 0x1::fixed_point64 {
    public fun create_from_rational(numerator: u128, denominator: u128): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    pragma verify_duration_estimate = 1000;
    include CreateFromRationalAbortsIf;
    ensures result == spec_create_from_rational(numerator, denominator);
}
```



<a id="0x1_fixed_point64_CreateFromRationalAbortsIf"></a>


```move
module 0x1::fixed_point64 {
    schema CreateFromRationalAbortsIf {
        numerator: u128;
        denominator: u128;
        let scaled_numerator = (numerator as u256)<< 64;
        let scaled_denominator = (denominator as u256);
        let quotient = scaled_numerator / scaled_denominator;
        aborts_if scaled_denominator == 0 with EDENOMINATOR;
        aborts_if quotient == 0 && scaled_numerator != 0 with ERATIO_OUT_OF_RANGE;
        aborts_if quotient > MAX_U128 with ERATIO_OUT_OF_RANGE;
    }
}
```



<a id="0x1_fixed_point64_spec_create_from_rational"></a>


```move
module 0x1::fixed_point64 {
    fun spec_create_from_rational(numerator: num, denominator: num): FixedPoint64 {
       FixedPoint64{value: (numerator << 128) / (denominator << 64)}
    }
}
```


<a id="@Specification_1_create_from_raw_value"></a>

### Function `create_from_raw_value`


```move
module 0x1::fixed_point64 {
    public fun create_from_raw_value(value: u128): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result.value == value;
}
```


<a id="@Specification_1_min"></a>

### Function `min`


```move
module 0x1::fixed_point64 {
    public fun min(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_min(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_min"></a>


```move
module 0x1::fixed_point64 {
    fun spec_min(num1: FixedPoint64, num2: FixedPoint64): FixedPoint64 {
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
module 0x1::fixed_point64 {
    public fun max(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_max(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_max"></a>


```move
module 0x1::fixed_point64 {
    fun spec_max(num1: FixedPoint64, num2: FixedPoint64): FixedPoint64 {
       if (num1.value > num2.value) {
           num1
       } else {
           num2
       }
    }
}
```


<a id="@Specification_1_less_or_equal"></a>

### Function `less_or_equal`


```move
module 0x1::fixed_point64 {
    public fun less_or_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_less_or_equal(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_less_or_equal"></a>


```move
module 0x1::fixed_point64 {
    fun spec_less_or_equal(num1: FixedPoint64, num2: FixedPoint64): bool {
       num1.value <= num2.value
    }
}
```


<a id="@Specification_1_less"></a>

### Function `less`


```move
module 0x1::fixed_point64 {
    public fun less(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_less(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_less"></a>


```move
module 0x1::fixed_point64 {
    fun spec_less(num1: FixedPoint64, num2: FixedPoint64): bool {
       num1.value < num2.value
    }
}
```


<a id="@Specification_1_greater_or_equal"></a>

### Function `greater_or_equal`


```move
module 0x1::fixed_point64 {
    public fun greater_or_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_greater_or_equal(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_greater_or_equal"></a>


```move
module 0x1::fixed_point64 {
    fun spec_greater_or_equal(num1: FixedPoint64, num2: FixedPoint64): bool {
       num1.value >= num2.value
    }
}
```


<a id="@Specification_1_greater"></a>

### Function `greater`


```move
module 0x1::fixed_point64 {
    public fun greater(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_greater(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_greater"></a>


```move
module 0x1::fixed_point64 {
    fun spec_greater(num1: FixedPoint64, num2: FixedPoint64): bool {
       num1.value > num2.value
    }
}
```


<a id="@Specification_1_equal"></a>

### Function `equal`


```move
module 0x1::fixed_point64 {
    public fun equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_equal(num1, num2);
}
```



<a id="0x1_fixed_point64_spec_equal"></a>


```move
module 0x1::fixed_point64 {
    fun spec_equal(num1: FixedPoint64, num2: FixedPoint64): bool {
       num1.value == num2.value
    }
}
```


<a id="@Specification_1_almost_equal"></a>

### Function `almost_equal`


```move
module 0x1::fixed_point64 {
    public fun almost_equal(num1: fixed_point64::FixedPoint64, num2: fixed_point64::FixedPoint64, precision: fixed_point64::FixedPoint64): bool
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_almost_equal(num1, num2, precision);
}
```



<a id="0x1_fixed_point64_spec_almost_equal"></a>


```move
module 0x1::fixed_point64 {
    fun spec_almost_equal(num1: FixedPoint64, num2: FixedPoint64, precision: FixedPoint64): bool {
       if (num1.value > num2.value) {
           (num1.value - num2.value <= precision.value)
       } else {
           (num2.value - num1.value <= precision.value)
       }
    }
}
```


<a id="@Specification_1_create_from_u128"></a>

### Function `create_from_u128`


```move
module 0x1::fixed_point64 {
    public fun create_from_u128(val: u128): fixed_point64::FixedPoint64
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    include CreateFromU64AbortsIf;
    ensures result == spec_create_from_u128(val);
}
```



<a id="0x1_fixed_point64_CreateFromU64AbortsIf"></a>


```move
module 0x1::fixed_point64 {
    schema CreateFromU64AbortsIf {
        val: num;
        let scaled_value = (val as u256) << 64;
        aborts_if scaled_value > MAX_U128;
    }
}
```



<a id="0x1_fixed_point64_spec_create_from_u128"></a>


```move
module 0x1::fixed_point64 {
    fun spec_create_from_u128(val: num): FixedPoint64 {
       FixedPoint64 {value: val << 64}
    }
}
```


<a id="@Specification_1_floor"></a>

### Function `floor`


```move
module 0x1::fixed_point64 {
    public fun floor(num: fixed_point64::FixedPoint64): u128
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_floor(num);
}
```



<a id="0x1_fixed_point64_spec_floor"></a>


```move
module 0x1::fixed_point64 {
    fun spec_floor(val: FixedPoint64): u128 {
       let fractional = val.value % (1 << 64);
       if (fractional == 0) {
           val.value >> 64
       } else {
           (val.value - fractional) >> 64
       }
    }
}
```


<a id="@Specification_1_ceil"></a>

### Function `ceil`


```move
module 0x1::fixed_point64 {
    public fun ceil(num: fixed_point64::FixedPoint64): u128
}
```



```move
module 0x1::fixed_point64 {
    pragma verify_duration_estimate = 1000;
    pragma opaque;
    aborts_if false;
    ensures result == spec_ceil(num);
}
```



<a id="0x1_fixed_point64_spec_ceil"></a>


```move
module 0x1::fixed_point64 {
    fun spec_ceil(val: FixedPoint64): u128 {
       let fractional = val.value % (1 << 64);
       let one = 1 << 64;
       if (fractional == 0) {
           val.value >> 64
       } else {
           (val.value - fractional + one) >> 64
       }
    }
}
```


<a id="@Specification_1_round"></a>

### Function `round`


```move
module 0x1::fixed_point64 {
    public fun round(num: fixed_point64::FixedPoint64): u128
}
```



```move
module 0x1::fixed_point64 {
    pragma opaque;
    aborts_if false;
    ensures result == spec_round(num);
}
```



<a id="0x1_fixed_point64_spec_round"></a>


```move
module 0x1::fixed_point64 {
    fun spec_round(val: FixedPoint64): u128 {
       let fractional = val.value % (1 << 64);
       let boundary = (1 << 64) / 2;
       let one = 1 << 64;
       if (fractional < boundary) {
           (val.value - fractional) >> 64
       } else {
           (val.value - fractional + one) >> 64
       }
    }
}
```
