
<a id="0x1_math_fixed"></a>

# Module `0x1::math_fixed`

Standard math utilities missing in the Move Language.


-  [Constants](#@Constants_0)
-  [Function `sqrt`](#0x1_math_fixed_sqrt)
-  [Function `exp`](#0x1_math_fixed_exp)
-  [Function `log2_plus_32`](#0x1_math_fixed_log2_plus_32)
-  [Function `ln_plus_32ln2`](#0x1_math_fixed_ln_plus_32ln2)
-  [Function `pow`](#0x1_math_fixed_pow)
-  [Function `mul_div`](#0x1_math_fixed_mul_div)
-  [Function `exp_raw`](#0x1_math_fixed_exp_raw)
-  [Function `pow_raw`](#0x1_math_fixed_pow_raw)


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32](0x1::fixed_point32);
<b>use</b> [math128.md#0x1_math128](0x1::math128);
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_math_fixed_EOVERFLOW_EXP"></a>

Abort code on overflow


<pre><code><b>const</b> [math_fixed.md#0x1_math_fixed_EOVERFLOW_EXP](EOVERFLOW_EXP): u64 = 1;
</code></pre>



<a id="0x1_math_fixed_LN2"></a>

Natural log 2 in 32 bit fixed point


<pre><code><b>const</b> [math_fixed.md#0x1_math_fixed_LN2](LN2): u128 = 2977044472;
</code></pre>



<a id="0x1_math_fixed_LN2_X_32"></a>



<pre><code><b>const</b> [math_fixed.md#0x1_math_fixed_LN2_X_32](LN2_X_32): u64 = 95265423104;
</code></pre>



<a id="0x1_math_fixed_sqrt"></a>

## Function `sqrt`

Square root of fixed point number


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_sqrt](sqrt)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_sqrt](sqrt)(x: FixedPoint32): FixedPoint32 {
    <b>let</b> y = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x) <b>as</b> u128);
    [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value](fixed_point32::create_from_raw_value)(([math128.md#0x1_math128_sqrt](math128::sqrt)(y &lt;&lt; 32) <b>as</b> u64))
}
</code></pre>



</details>

<a id="0x1_math_fixed_exp"></a>

## Function `exp`

Exponent function with a precission of 9 digits.


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_exp](exp)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_exp](exp)(x: FixedPoint32): FixedPoint32 {
    <b>let</b> raw_value = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x) <b>as</b> u128);
    [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value](fixed_point32::create_from_raw_value)(([math_fixed.md#0x1_math_fixed_exp_raw](exp_raw)(raw_value) <b>as</b> u64))
}
</code></pre>



</details>

<a id="0x1_math_fixed_log2_plus_32"></a>

## Function `log2_plus_32`

Because log2 is negative for values < 1 we instead return log2(x) + 32 which
is positive for all values of x.


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_log2_plus_32](log2_plus_32)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_log2_plus_32](log2_plus_32)(x: FixedPoint32): FixedPoint32 {
    <b>let</b> raw_value = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x) <b>as</b> u128);
    [math128.md#0x1_math128_log2](math128::log2)(raw_value)
}
</code></pre>



</details>

<a id="0x1_math_fixed_ln_plus_32ln2"></a>

## Function `ln_plus_32ln2`



<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_ln_plus_32ln2](ln_plus_32ln2)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_ln_plus_32ln2](ln_plus_32ln2)(x: FixedPoint32): FixedPoint32 {
    <b>let</b> raw_value = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x) <b>as</b> u128);
    <b>let</b> x = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)([math128.md#0x1_math128_log2](math128::log2)(raw_value)) <b>as</b> u128);
    [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value](fixed_point32::create_from_raw_value)((x * [math_fixed.md#0x1_math_fixed_LN2](LN2) &gt;&gt; 32 <b>as</b> u64))
}
</code></pre>



</details>

<a id="0x1_math_fixed_pow"></a>

## Function `pow`

Integer power of a fixed point number


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_pow](pow)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), n: u64): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_pow](pow)(x: FixedPoint32, n: u64): FixedPoint32 {
    <b>let</b> raw_value = ([../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x) <b>as</b> u128);
    [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value](fixed_point32::create_from_raw_value)(([math_fixed.md#0x1_math_fixed_pow_raw](pow_raw)(raw_value, (n <b>as</b> u128)) <b>as</b> u64))
}
</code></pre>



</details>

<a id="0x1_math_fixed_mul_div"></a>

## Function `mul_div`

Specialized function for x * y / z that omits intermediate shifting


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_mul_div](mul_div)(x: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), y: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32), z: [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)): [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32](fixed_point32::FixedPoint32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [math_fixed.md#0x1_math_fixed_mul_div](mul_div)(x: FixedPoint32, y: FixedPoint32, z: FixedPoint32): FixedPoint32 {
    <b>let</b> a = [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(x);
    <b>let</b> b = [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(y);
    <b>let</b> c = [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_get_raw_value](fixed_point32::get_raw_value)(z);
    [../../move-stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value](fixed_point32::create_from_raw_value) ([math64.md#0x1_math64_mul_div](math64::mul_div)(a, b, c))
}
</code></pre>



</details>

<a id="0x1_math_fixed_exp_raw"></a>

## Function `exp_raw`



<pre><code><b>fun</b> [math_fixed.md#0x1_math_fixed_exp_raw](exp_raw)(x: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [math_fixed.md#0x1_math_fixed_exp_raw](exp_raw)(x: u128): u128 {
    // [math_fixed.md#0x1_math_fixed_exp](exp)(x / 2^32) = 2^(x / (2^32 * ln(2))) = 2^(floor(x / (2^32 * ln(2))) + frac(x / (2^32 * ln(2))))
    <b>let</b> shift_long = x / [math_fixed.md#0x1_math_fixed_LN2](LN2);
    <b>assert</b>!(shift_long &lt;= 31, std::error::invalid_state([math_fixed.md#0x1_math_fixed_EOVERFLOW_EXP](EOVERFLOW_EXP)));
    <b>let</b> shift = (shift_long <b>as</b> u8);
    <b>let</b> remainder = x % [math_fixed.md#0x1_math_fixed_LN2](LN2);
    // At this point we want <b>to</b> calculate 2^(remainder / ln2) &lt;&lt; shift
    // ln2 = 595528 * 4999 which means
    <b>let</b> bigfactor = 595528;
    <b>let</b> exponent = remainder / bigfactor;
    <b>let</b> x = remainder % bigfactor;
    // 2^(remainder / ln2) = (2^(1/4999))^exponent * [math_fixed.md#0x1_math_fixed_exp](exp)(x / 2^32)
    <b>let</b> roottwo = 4295562865;  // fixed point representation of 2^(1/4999)
    // This <b>has</b> an [../../move-stdlib/doc/error.md#0x1_error](error) of 5000 / 4 10^9 roughly 6 digits of precission
    <b>let</b> power = [math_fixed.md#0x1_math_fixed_pow_raw](pow_raw)(roottwo, exponent);
    <b>let</b> eps_correction = 1241009291;
    power = power + ((power * eps_correction * exponent) &gt;&gt; 64);
    // x is fixed point number smaller than 595528/2^32 &lt; 0.00014 so we need only 2 tayler steps
    // <b>to</b> get the 6 digits of precission
    <b>let</b> taylor1 = (power * x) &gt;&gt; (32 - shift);
    <b>let</b> taylor2 = (taylor1 * x) &gt;&gt; 32;
    <b>let</b> taylor3 = (taylor2 * x) &gt;&gt; 32;
    (power &lt;&lt; shift) + taylor1 + taylor2 / 2 + taylor3 / 6
}
</code></pre>



</details>

<a id="0x1_math_fixed_pow_raw"></a>

## Function `pow_raw`



<pre><code><b>fun</b> [math_fixed.md#0x1_math_fixed_pow_raw](pow_raw)(x: u128, n: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [math_fixed.md#0x1_math_fixed_pow_raw](pow_raw)(x: u128, n: u128): u128 {
    <b>let</b> res: u256 = 1 &lt;&lt; 64;
    x = x &lt;&lt; 32;
    <b>while</b> (n != 0) {
        <b>if</b> (n & 1 != 0) {
            res = (res * (x <b>as</b> u256)) &gt;&gt; 64;
        };
        n = n &gt;&gt; 1;
        x = ((((x <b>as</b> u256) * (x <b>as</b> u256)) &gt;&gt; 64) <b>as</b> u128);
    };
    ((res &gt;&gt; 32) <b>as</b> u128)
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
