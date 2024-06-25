
<a id="0x1_math_fixed64"></a>

# Module `0x1::math_fixed64`

Standard math utilities missing in the Move Language.


-  [Constants](#@Constants_0)
-  [Function `sqrt`](#0x1_math_fixed64_sqrt)
-  [Function `exp`](#0x1_math_fixed64_exp)
-  [Function `log2_plus_64`](#0x1_math_fixed64_log2_plus_64)
-  [Function `ln_plus_32ln2`](#0x1_math_fixed64_ln_plus_32ln2)
-  [Function `pow`](#0x1_math_fixed64_pow)
-  [Function `mul_div`](#0x1_math_fixed64_mul_div)
-  [Function `exp_raw`](#0x1_math_fixed64_exp_raw)
-  [Function `pow_raw`](#0x1_math_fixed64_pow_raw)


```move
module 0x1::math_fixed64 {
    use 0x1::error;
    use 0x1::fixed_point64;
    use 0x1::math128;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_math_fixed64_EOVERFLOW_EXP"></a>

Abort code on overflow


```move
module 0x1::math_fixed64 {
    const EOVERFLOW_EXP: u64 = 1;
}
```


<a id="0x1_math_fixed64_LN2"></a>

Natural log 2 in 32 bit fixed point


```move
module 0x1::math_fixed64 {
    const LN2: u256 = 12786308645202655660;
}
```


<a id="0x1_math_fixed64_sqrt"></a>

## Function `sqrt`

Square root of fixed point number


```move
module 0x1::math_fixed64 {
    public fun sqrt(x: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun sqrt(x: FixedPoint64): FixedPoint64 {
        let y = fixed_point64::get_raw_value(x);
        let z = (math128::sqrt(y) << 32 as u256);
        z = (z + ((y as u256) << 64) / z) >> 1;
        fixed_point64::create_from_raw_value((z as u128))
    }
}
```


<a id="0x1_math_fixed64_exp"></a>

## Function `exp`

Exponent function with a precission of 9 digits.


```move
module 0x1::math_fixed64 {
    public fun exp(x: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun exp(x: FixedPoint64): FixedPoint64 {
        let raw_value = (fixed_point64::get_raw_value(x) as u256);
        fixed_point64::create_from_raw_value((exp_raw(raw_value) as u128))
    }
}
```


<a id="0x1_math_fixed64_log2_plus_64"></a>

## Function `log2_plus_64`

Because log2 is negative for values &lt; 1 we instead return log2(x) &#43; 64 which
is positive for all values of x.


```move
module 0x1::math_fixed64 {
    public fun log2_plus_64(x: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun log2_plus_64(x: FixedPoint64): FixedPoint64 {
        let raw_value = (fixed_point64::get_raw_value(x) as u128);
        math128::log2_64(raw_value)
    }
}
```


<a id="0x1_math_fixed64_ln_plus_32ln2"></a>

## Function `ln_plus_32ln2`



```move
module 0x1::math_fixed64 {
    public fun ln_plus_32ln2(x: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun ln_plus_32ln2(x: FixedPoint64): FixedPoint64 {
        let raw_value = fixed_point64::get_raw_value(x);
        let x = (fixed_point64::get_raw_value(math128::log2_64(raw_value)) as u256);
        fixed_point64::create_from_raw_value(((x * LN2) >> 64 as u128))
    }
}
```


<a id="0x1_math_fixed64_pow"></a>

## Function `pow`

Integer power of a fixed point number


```move
module 0x1::math_fixed64 {
    public fun pow(x: fixed_point64::FixedPoint64, n: u64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun pow(x: FixedPoint64, n: u64): FixedPoint64 {
        let raw_value = (fixed_point64::get_raw_value(x) as u256);
        fixed_point64::create_from_raw_value((pow_raw(raw_value, (n as u128)) as u128))
    }
}
```


<a id="0x1_math_fixed64_mul_div"></a>

## Function `mul_div`

Specialized function for x &#42; y / z that omits intermediate shifting


```move
module 0x1::math_fixed64 {
    public fun mul_div(x: fixed_point64::FixedPoint64, y: fixed_point64::FixedPoint64, z: fixed_point64::FixedPoint64): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    public fun mul_div(x: FixedPoint64, y: FixedPoint64, z: FixedPoint64): FixedPoint64 {
        let a = fixed_point64::get_raw_value(x);
        let b = fixed_point64::get_raw_value(y);
        let c = fixed_point64::get_raw_value(z);
        fixed_point64::create_from_raw_value (math128::mul_div(a, b, c))
    }
}
```


<a id="0x1_math_fixed64_exp_raw"></a>

## Function `exp_raw`



```move
module 0x1::math_fixed64 {
    fun exp_raw(x: u256): u256
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    fun exp_raw(x: u256): u256 {
        // exp(x / 2^64) = 2^(x / (2^64 * ln(2))) = 2^(floor(x / (2^64 * ln(2))) + frac(x / (2^64 * ln(2))))
        let shift_long = x / LN2;
        assert!(shift_long <= 63, std::error::invalid_state(EOVERFLOW_EXP));
        let shift = (shift_long as u8);
        let remainder = x % LN2;
        // At this point we want to calculate 2^(remainder / ln2) << shift
        // ln2 = 580 * 22045359733108027
        let bigfactor = 22045359733108027;
        let exponent = remainder / bigfactor;
        let x = remainder % bigfactor;
        // 2^(remainder / ln2) = (2^(1/580))^exponent * exp(x / 2^64)
        let roottwo = 18468802611690918839;  // fixed point representation of 2^(1/580)
        // 2^(1/580) = roottwo(1 - eps), so the number we seek is roottwo^exponent (1 - eps * exponent)
        let power = pow_raw(roottwo, (exponent as u128));
        let eps_correction = 219071715585908898;
        power = power - ((power * eps_correction * exponent) >> 128);
        // x is fixed point number smaller than bigfactor/2^64 < 0.0011 so we need only 5 tayler steps
        // to get the 15 digits of precission
        let taylor1 = (power * x) >> (64 - shift);
        let taylor2 = (taylor1 * x) >> 64;
        let taylor3 = (taylor2 * x) >> 64;
        let taylor4 = (taylor3 * x) >> 64;
        let taylor5 = (taylor4 * x) >> 64;
        let taylor6 = (taylor5 * x) >> 64;
        (power << shift) + taylor1 + taylor2 / 2 + taylor3 / 6 + taylor4 / 24 + taylor5 / 120 + taylor6 / 720
    }
}
```


<a id="0x1_math_fixed64_pow_raw"></a>

## Function `pow_raw`



```move
module 0x1::math_fixed64 {
    fun pow_raw(x: u256, n: u128): u256
}
```


##### Implementation


```move
module 0x1::math_fixed64 {
    fun pow_raw(x: u256, n: u128): u256 {
        let res: u256 = 1 << 64;
        while (n != 0) {
            if (n & 1 != 0) {
                res = (res * x) >> 64;
            };
            n = n >> 1;
            x = (x * x) >> 64;
        };
        res
    }
}
```
