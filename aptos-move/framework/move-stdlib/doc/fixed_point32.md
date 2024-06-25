
<a id="0x1_fixed_point32"></a>

# Module `0x1::fixed_point32`

Defines a fixed-point numeric type with a 32-bit integer part and
a 32-bit fractional part.


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


<pre><code></code></pre>



<a id="0x1_fixed_point32_FixedPoint32"></a>

## Struct `FixedPoint32`

Define a fixed-point numeric type with 32 fractional bits.
This is just a u64 integer but it is wrapped in a struct to
make a unique type. This is a binary representation, so decimal
values may not be exactly representable, but it provides more
than 9 decimal digits of precision both before and after the
decimal point (18 digits total). For comparison, double precision
floating-point has less than 16 decimal digits of precision, so
be careful about using floating-point to convert these values to
decimal.


<pre><code><b>struct</b> [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_fixed_point32_MAX_U64"></a>



<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64): u128 = 18446744073709551615;
</code></pre>



<a id="0x1_fixed_point32_EDENOMINATOR"></a>

The denominator provided was zero


<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_EDENOMINATOR](EDENOMINATOR): u64 = 65537;
</code></pre>



<a id="0x1_fixed_point32_EDIVISION"></a>

The quotient value would be too large to be held in a <code>u64</code>


<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_EDIVISION](EDIVISION): u64 = 131074;
</code></pre>



<a id="0x1_fixed_point32_EDIVISION_BY_ZERO"></a>

A division by zero was encountered


<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_EDIVISION_BY_ZERO](EDIVISION_BY_ZERO): u64 = 65540;
</code></pre>



<a id="0x1_fixed_point32_EMULTIPLICATION"></a>

The multiplied value would be too large to be held in a <code>u64</code>


<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_EMULTIPLICATION](EMULTIPLICATION): u64 = 131075;
</code></pre>



<a id="0x1_fixed_point32_ERATIO_OUT_OF_RANGE"></a>

The computed ratio when converting to a <code>[fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)</code> would be unrepresentable


<pre><code><b>const</b> [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE): u64 = 131077;
</code></pre>



<a id="0x1_fixed_point32_multiply_u64"></a>

## Function `multiply_u64`

Multiply a u64 integer by a fixed-point number, truncating any
fractional part of the product. This will abort if the product
overflows.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_multiply_u64](multiply_u64)(val: u64, multiplier: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_multiply_u64](multiply_u64)(val: u64, multiplier: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    // The product of two 64 bit values <b>has</b> 128 bits, so perform the
    // multiplication <b>with</b> u128 types and keep the full 128 bit product
    // <b>to</b> avoid losing accuracy.
    <b>let</b> unscaled_product = (val <b>as</b> u128) * (multiplier.value <b>as</b> u128);
    // The unscaled product <b>has</b> 32 fractional bits (from the multiplier)
    // so rescale it by shifting away the low bits.
    <b>let</b> product = unscaled_product &gt;&gt; 32;
    // Check whether the value is too large.
    <b>assert</b>!(product &lt;= [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64), [fixed_point32.md#0x1_fixed_point32_EMULTIPLICATION](EMULTIPLICATION));
    (product <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_fixed_point32_divide_u64"></a>

## Function `divide_u64`

Divide a u64 integer by a fixed-point number, truncating any
fractional part of the quotient. This will abort if the divisor
is zero or if the quotient overflows.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_divide_u64](divide_u64)(val: u64, divisor: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_divide_u64](divide_u64)(val: u64, divisor: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    // Check for division by zero.
    <b>assert</b>!(divisor.value != 0, [fixed_point32.md#0x1_fixed_point32_EDIVISION_BY_ZERO](EDIVISION_BY_ZERO));
    // First convert <b>to</b> 128 bits and then shift left <b>to</b>
    // add 32 fractional zero bits <b>to</b> the dividend.
    <b>let</b> scaled_value = (val <b>as</b> u128) &lt;&lt; 32;
    <b>let</b> quotient = scaled_value / (divisor.value <b>as</b> u128);
    // Check whether the value is too large.
    <b>assert</b>!(quotient &lt;= [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64), [fixed_point32.md#0x1_fixed_point32_EDIVISION](EDIVISION));
    // the value may be too large, which will cause the cast <b>to</b> fail
    // <b>with</b> an arithmetic [error.md#0x1_error](error).
    (quotient <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_fixed_point32_create_from_rational"></a>

## Function `create_from_rational`

Create a fixed-point value from a rational number specified by its
numerator and denominator. Calling this function should be preferred
for using <code>[fixed_point32.md#0x1_fixed_point32_create_from_raw_value](Self::create_from_raw_value)</code> which is also available.
This will abort if the denominator is zero. It will also
abort if the numerator is nonzero and the ratio is not in the range
2^-32 .. 2^32-1. When specifying decimal fractions, be careful about
rounding errors: if you round to display N digits after the decimal
point, you can use a denominator of 10^N to avoid numbers where the
very small imprecision in the binary representation could change the
rounding, e.g., 0.0125 will round down to 0.012 instead of up to 0.013.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_rational](create_from_rational)(numerator: u64, denominator: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_rational](create_from_rational)(numerator: u64, denominator: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
    // If the denominator is zero, this will <b>abort</b>.
    // Scale the numerator <b>to</b> have 64 fractional bits and the denominator
    // <b>to</b> have 32 fractional bits, so that the quotient will have 32
    // fractional bits.
    <b>let</b> scaled_numerator = (numerator <b>as</b> u128) &lt;&lt; 64;
    <b>let</b> scaled_denominator = (denominator <b>as</b> u128) &lt;&lt; 32;
    <b>assert</b>!(scaled_denominator != 0, [fixed_point32.md#0x1_fixed_point32_EDENOMINATOR](EDENOMINATOR));
    <b>let</b> quotient = scaled_numerator / scaled_denominator;
    <b>assert</b>!(quotient != 0 || numerator == 0, [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE));
    // Return the quotient <b>as</b> a fixed-point number. We first need <b>to</b> check whether the cast
    // can succeed.
    <b>assert</b>!(quotient &lt;= [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64), [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE));
    [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) { value: (quotient <b>as</b> u64) }
}
</code></pre>



</details>

<a id="0x1_fixed_point32_create_from_raw_value"></a>

## Function `create_from_raw_value`

Create a fixedpoint value from a raw value.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_raw_value](create_from_raw_value)(value: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_raw_value](create_from_raw_value)(value: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
    [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) { value }
}
</code></pre>



</details>

<a id="0x1_fixed_point32_get_raw_value"></a>

## Function `get_raw_value`

Accessor for the raw u64 value. Other less common operations, such as
adding or subtracting FixedPoint32 values, can be done using the raw
values directly.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_get_raw_value](get_raw_value)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_get_raw_value](get_raw_value)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    num.value
}
</code></pre>



</details>

<a id="0x1_fixed_point32_is_zero"></a>

## Function `is_zero`

Returns true if the ratio is zero.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_is_zero](is_zero)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_is_zero](is_zero)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): bool {
    num.value == 0
}
</code></pre>



</details>

<a id="0x1_fixed_point32_min"></a>

## Function `min`

Returns the smaller of the two FixedPoint32 numbers.


<pre><code><b>public</b> <b>fun</b> <b>min</b>(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>min</b>(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
    <b>if</b> (num1.value &lt; num2.value) {
        num1
    } <b>else</b> {
        num2
    }
}
</code></pre>



</details>

<a id="0x1_fixed_point32_max"></a>

## Function `max`

Returns the larger of the two FixedPoint32 numbers.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_max](max)(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_max](max)(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
    <b>if</b> (num1.value &gt; num2.value) {
        num1
    } <b>else</b> {
        num2
    }
}
</code></pre>



</details>

<a id="0x1_fixed_point32_create_from_u64"></a>

## Function `create_from_u64`

Create a fixedpoint value from a u64 value.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_u64](create_from_u64)(val: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_u64](create_from_u64)(val: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
    <b>let</b> value = (val <b>as</b> u128) &lt;&lt; 32;
    <b>assert</b>!(value &lt;= [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64), [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE));
    [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {value: (value <b>as</b> u64)}
}
</code></pre>



</details>

<a id="0x1_fixed_point32_floor"></a>

## Function `floor`

Returns the largest integer less than or equal to a given number.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_floor](floor)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_floor](floor)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    num.value &gt;&gt; 32
}
</code></pre>



</details>

<a id="0x1_fixed_point32_ceil"></a>

## Function `ceil`

Rounds up the given FixedPoint32 to the next largest integer.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_ceil](ceil)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_ceil](ceil)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    <b>let</b> floored_num = [fixed_point32.md#0x1_fixed_point32_floor](floor)(num) &lt;&lt; 32;
    <b>if</b> (num.value == floored_num) {
        <b>return</b> floored_num &gt;&gt; 32
    };
    <b>let</b> val = ((floored_num <b>as</b> u128) + (1 &lt;&lt; 32));
    (val &gt;&gt; 32 <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_fixed_point32_round"></a>

## Function `round`

Returns the value of a FixedPoint32 to the nearest integer.


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_round](round)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_round](round)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
    <b>let</b> floored_num = [fixed_point32.md#0x1_fixed_point32_floor](floor)(num) &lt;&lt; 32;
    <b>let</b> boundary = floored_num + ((1 &lt;&lt; 32) / 2);
    <b>if</b> (num.value &lt; boundary) {
        floored_num &gt;&gt; 32
    } <b>else</b> {
        [fixed_point32.md#0x1_fixed_point32_ceil](ceil)(num)
    }
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification




<pre><code><b>pragma</b> aborts_if_is_strict;
</code></pre>



<a id="@Specification_1_multiply_u64"></a>

### Function `multiply_u64`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_multiply_u64](multiply_u64)(val: u64, multiplier: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>include</b> [fixed_point32.md#0x1_fixed_point32_MultiplyAbortsIf](MultiplyAbortsIf);
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_multiply_u64](spec_multiply_u64)(val, multiplier);
</code></pre>




<a id="0x1_fixed_point32_MultiplyAbortsIf"></a>


<pre><code><b>schema</b> [fixed_point32.md#0x1_fixed_point32_MultiplyAbortsIf](MultiplyAbortsIf) {
    val: num;
    multiplier: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32);
    <b>aborts_if</b> [fixed_point32.md#0x1_fixed_point32_spec_multiply_u64](spec_multiply_u64)(val, multiplier) &gt; [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64) <b>with</b> [fixed_point32.md#0x1_fixed_point32_EMULTIPLICATION](EMULTIPLICATION);
}
</code></pre>




<a id="0x1_fixed_point32_spec_multiply_u64"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_multiply_u64](spec_multiply_u64)(val: num, multiplier: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): num {
   (val * multiplier.value) &gt;&gt; 32
}
</code></pre>



<a id="@Specification_1_divide_u64"></a>

### Function `divide_u64`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_divide_u64](divide_u64)(val: u64, divisor: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>include</b> [fixed_point32.md#0x1_fixed_point32_DivideAbortsIf](DivideAbortsIf);
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_divide_u64](spec_divide_u64)(val, divisor);
</code></pre>




<a id="0x1_fixed_point32_DivideAbortsIf"></a>


<pre><code><b>schema</b> [fixed_point32.md#0x1_fixed_point32_DivideAbortsIf](DivideAbortsIf) {
    val: num;
    divisor: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32);
    <b>aborts_if</b> divisor.value == 0 <b>with</b> [fixed_point32.md#0x1_fixed_point32_EDIVISION_BY_ZERO](EDIVISION_BY_ZERO);
    <b>aborts_if</b> [fixed_point32.md#0x1_fixed_point32_spec_divide_u64](spec_divide_u64)(val, divisor) &gt; [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64) <b>with</b> [fixed_point32.md#0x1_fixed_point32_EDIVISION](EDIVISION);
}
</code></pre>




<a id="0x1_fixed_point32_spec_divide_u64"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_divide_u64](spec_divide_u64)(val: num, divisor: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): num {
   (val &lt;&lt; 32) / divisor.value
}
</code></pre>



<a id="@Specification_1_create_from_rational"></a>

### Function `create_from_rational`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_rational](create_from_rational)(numerator: u64, denominator: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>include</b> [fixed_point32.md#0x1_fixed_point32_CreateFromRationalAbortsIf](CreateFromRationalAbortsIf);
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_create_from_rational](spec_create_from_rational)(numerator, denominator);
</code></pre>




<a id="0x1_fixed_point32_CreateFromRationalAbortsIf"></a>


<pre><code><b>schema</b> [fixed_point32.md#0x1_fixed_point32_CreateFromRationalAbortsIf](CreateFromRationalAbortsIf) {
    numerator: u64;
    denominator: u64;
    <b>let</b> scaled_numerator = (numerator <b>as</b> u128)&lt;&lt; 64;
    <b>let</b> scaled_denominator = (denominator <b>as</b> u128) &lt;&lt; 32;
    <b>let</b> quotient = scaled_numerator / scaled_denominator;
    <b>aborts_if</b> scaled_denominator == 0 <b>with</b> [fixed_point32.md#0x1_fixed_point32_EDENOMINATOR](EDENOMINATOR);
    <b>aborts_if</b> quotient == 0 && scaled_numerator != 0 <b>with</b> [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE);
    <b>aborts_if</b> quotient &gt; [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64) <b>with</b> [fixed_point32.md#0x1_fixed_point32_ERATIO_OUT_OF_RANGE](ERATIO_OUT_OF_RANGE);
}
</code></pre>




<a id="0x1_fixed_point32_spec_create_from_rational"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_create_from_rational](spec_create_from_rational)(numerator: num, denominator: num): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
   [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32){value: (numerator &lt;&lt; 64) / (denominator &lt;&lt; 32)}
}
</code></pre>



<a id="@Specification_1_create_from_raw_value"></a>

### Function `create_from_raw_value`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_raw_value](create_from_raw_value)(value: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result.value == value;
</code></pre>



<a id="@Specification_1_min"></a>

### Function `min`


<pre><code><b>public</b> <b>fun</b> <b>min</b>(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_min](spec_min)(num1, num2);
</code></pre>




<a id="0x1_fixed_point32_spec_min"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_min](spec_min)(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
   <b>if</b> (num1.value &lt; num2.value) {
       num1
   } <b>else</b> {
       num2
   }
}
</code></pre>



<a id="@Specification_1_max"></a>

### Function `max`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_max](max)(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_max](spec_max)(num1, num2);
</code></pre>




<a id="0x1_fixed_point32_spec_max"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_max](spec_max)(num1: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32), num2: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
   <b>if</b> (num1.value &gt; num2.value) {
       num1
   } <b>else</b> {
       num2
   }
}
</code></pre>



<a id="@Specification_1_create_from_u64"></a>

### Function `create_from_u64`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_create_from_u64](create_from_u64)(val: u64): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>include</b> [fixed_point32.md#0x1_fixed_point32_CreateFromU64AbortsIf](CreateFromU64AbortsIf);
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_create_from_u64](spec_create_from_u64)(val);
</code></pre>




<a id="0x1_fixed_point32_CreateFromU64AbortsIf"></a>


<pre><code><b>schema</b> [fixed_point32.md#0x1_fixed_point32_CreateFromU64AbortsIf](CreateFromU64AbortsIf) {
    val: num;
    <b>let</b> scaled_value = (val <b>as</b> u128) &lt;&lt; 32;
    <b>aborts_if</b> scaled_value &gt; [fixed_point32.md#0x1_fixed_point32_MAX_U64](MAX_U64);
}
</code></pre>




<a id="0x1_fixed_point32_spec_create_from_u64"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_create_from_u64](spec_create_from_u64)(val: num): [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {
   [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32) {value: val &lt;&lt; 32}
}
</code></pre>



<a id="@Specification_1_floor"></a>

### Function `floor`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_floor](floor)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>




<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_floor](spec_floor)(num);
</code></pre>




<a id="0x1_fixed_point32_spec_floor"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_floor](spec_floor)(val: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
   <b>let</b> fractional = val.value % (1 &lt;&lt; 32);
   <b>if</b> (fractional == 0) {
       val.value &gt;&gt; 32
   } <b>else</b> {
       (val.value - fractional) &gt;&gt; 32
   }
}
</code></pre>



<a id="@Specification_1_ceil"></a>

### Function `ceil`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_ceil](ceil)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_ceil](spec_ceil)(num);
</code></pre>




<a id="0x1_fixed_point32_spec_ceil"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_ceil](spec_ceil)(val: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
   <b>let</b> fractional = val.value % (1 &lt;&lt; 32);
   <b>let</b> one = 1 &lt;&lt; 32;
   <b>if</b> (fractional == 0) {
       val.value &gt;&gt; 32
   } <b>else</b> {
       (val.value - fractional + one) &gt;&gt; 32
   }
}
</code></pre>



<a id="@Specification_1_round"></a>

### Function `round`


<pre><code><b>public</b> <b>fun</b> [fixed_point32.md#0x1_fixed_point32_round](round)(num: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): u64
</code></pre>




<pre><code><b>pragma</b> verify_duration_estimate = 120;
<b>pragma</b> opaque;
<b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [fixed_point32.md#0x1_fixed_point32_spec_round](spec_round)(num);
</code></pre>




<a id="0x1_fixed_point32_spec_round"></a>


<pre><code><b>fun</b> [fixed_point32.md#0x1_fixed_point32_spec_round](spec_round)(val: [fixed_point32.md#0x1_fixed_point32_FixedPoint32](FixedPoint32)): u64 {
   <b>let</b> fractional = val.value % (1 &lt;&lt; 32);
   <b>let</b> boundary = (1 &lt;&lt; 32) / 2;
   <b>let</b> one = 1 &lt;&lt; 32;
   <b>if</b> (fractional &lt; boundary) {
       (val.value - fractional) &gt;&gt; 32
   } <b>else</b> {
       (val.value - fractional + one) &gt;&gt; 32
   }
}
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
