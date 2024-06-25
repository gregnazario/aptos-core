
<a id="0x1_math128"></a>

# Module `0x1::math128`

Standard math utilities missing in the Move Language.


-  [Constants](#@Constants_0)
-  [Function `max`](#0x1_math128_max)
-  [Function `min`](#0x1_math128_min)
-  [Function `average`](#0x1_math128_average)
-  [Function `gcd`](#0x1_math128_gcd)
-  [Function `mul_div`](#0x1_math128_mul_div)
-  [Function `clamp`](#0x1_math128_clamp)
-  [Function `pow`](#0x1_math128_pow)
-  [Function `floor_log2`](#0x1_math128_floor_log2)
-  [Function `log2`](#0x1_math128_log2)
-  [Function `log2_64`](#0x1_math128_log2_64)
-  [Function `sqrt`](#0x1_math128_sqrt)
-  [Function `ceil_div`](#0x1_math128_ceil_div)
-  [Specification](#@Specification_1)
    -  [Function `max`](#@Specification_1_max)
    -  [Function `min`](#@Specification_1_min)
    -  [Function `average`](#@Specification_1_average)
    -  [Function `clamp`](#@Specification_1_clamp)
    -  [Function `pow`](#@Specification_1_pow)
    -  [Function `floor_log2`](#@Specification_1_floor_log2)
    -  [Function `sqrt`](#@Specification_1_sqrt)


```move
module 0x1::math128 {
    use 0x1::error;
    use 0x1::fixed_point32;
    use 0x1::fixed_point64;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_math128_EINVALID_ARG_FLOOR_LOG2"></a>

Cannot log2 the value 0


```move
module 0x1::math128 {
    const EINVALID_ARG_FLOOR_LOG2: u64 = 1;
}
```


<a id="0x1_math128_max"></a>

## Function `max`

Return the largest of two numbers.


```move
module 0x1::math128 {
    public fun max(a: u128, b: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun max(a: u128, b: u128): u128 {
        if (a >= b) a else b
    }
}
```


<a id="0x1_math128_min"></a>

## Function `min`

Return the smallest of two numbers.


```move
module 0x1::math128 {
    public fun min(a: u128, b: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun min(a: u128, b: u128): u128 {
        if (a < b) a else b
    }
}
```


<a id="0x1_math128_average"></a>

## Function `average`

Return the average of two.


```move
module 0x1::math128 {
    public fun average(a: u128, b: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun average(a: u128, b: u128): u128 {
        if (a < b) {
            a + (b - a) / 2
        } else {
            b + (a - b) / 2
        }
    }
}
```


<a id="0x1_math128_gcd"></a>

## Function `gcd`

Return greatest common divisor of `a` &amp; `b`, via the Euclidean algorithm.


```move
module 0x1::math128 {
    public fun gcd(a: u128, b: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public inline fun gcd(a: u128, b: u128): u128 {
        let (large, small) = if (a > b) (a, b) else (b, a);
        while (small != 0) {
            let tmp = small;
            small = large % small;
            large = tmp;
        };
        large
    }
}
```


<a id="0x1_math128_mul_div"></a>

## Function `mul_div`

Returns a &#42; b / c going through u256 to prevent intermediate overflow


```move
module 0x1::math128 {
    public fun mul_div(a: u128, b: u128, c: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public inline fun mul_div(a: u128, b: u128, c: u128): u128 {
        // Inline functions cannot take constants, as then every module using it needs the constant
        assert!(c != 0, std::error::invalid_argument(4));
        (((a as u256) * (b as u256) / (c as u256)) as u128)
    }
}
```


<a id="0x1_math128_clamp"></a>

## Function `clamp`

Return x clamped to the interval [lower, upper].


```move
module 0x1::math128 {
    public fun clamp(x: u128, lower: u128, upper: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun clamp(x: u128, lower: u128, upper: u128): u128 {
        min(upper, max(lower, x))
    }
}
```


<a id="0x1_math128_pow"></a>

## Function `pow`

Return the value of n raised to power e


```move
module 0x1::math128 {
    public fun pow(n: u128, e: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun pow(n: u128, e: u128): u128 {
        if (e == 0) {
            1
        } else {
            let p = 1;
            while (e > 1) {
                if (e % 2 == 1) {
                    p = p * n;
                };
                e = e / 2;
                n = n * n;
            };
            p * n
        }
    }
}
```


<a id="0x1_math128_floor_log2"></a>

## Function `floor_log2`

Returns floor(log2(x))


```move
module 0x1::math128 {
    public fun floor_log2(x: u128): u8
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun floor_log2(x: u128): u8 {
        let res = 0;
        assert!(x != 0, std::error::invalid_argument(EINVALID_ARG_FLOOR_LOG2));
        // Effectively the position of the most significant set bit
        let n = 64;
        while (n > 0) {
            if (x >= (1 << n)) {
                x = x >> n;
                res = res + n;
            };
            n = n >> 1;
        };
        res
    }
}
```


<a id="0x1_math128_log2"></a>

## Function `log2`



```move
module 0x1::math128 {
    public fun log2(x: u128): fixed_point32::FixedPoint32
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun log2(x: u128): FixedPoint32 {
        let integer_part = floor_log2(x);
        // Normalize x to [1, 2) in fixed point 32.
        if (x >= 1 << 32) {
            x = x >> (integer_part - 32);
        } else {
            x = x << (32 - integer_part);
        };
        let frac = 0;
        let delta = 1 << 31;
        while (delta != 0) {
            // log x = 1/2 log x^2
            // x in [1, 2)
            x = (x * x) >> 32;
            // x is now in [1, 4)
            // if x in [2, 4) then log x = 1 + log (x / 2)
            if (x >= (2 << 32)) { frac = frac + delta; x = x >> 1; };
            delta = delta >> 1;
        };
        fixed_point32::create_from_raw_value (((integer_part as u64) << 32) + frac)
    }
}
```


<a id="0x1_math128_log2_64"></a>

## Function `log2_64`



```move
module 0x1::math128 {
    public fun log2_64(x: u128): fixed_point64::FixedPoint64
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun log2_64(x: u128): FixedPoint64 {
        let integer_part = floor_log2(x);
        // Normalize x to [1, 2) in fixed point 63. To ensure x is smaller then 1<<64
        if (x >= 1 << 63) {
            x = x >> (integer_part - 63);
        } else {
            x = x << (63 - integer_part);
        };
        let frac = 0;
        let delta = 1 << 63;
        while (delta != 0) {
            // log x = 1/2 log x^2
            // x in [1, 2)
            x = (x * x) >> 63;
            // x is now in [1, 4)
            // if x in [2, 4) then log x = 1 + log (x / 2)
            if (x >= (2 << 63)) { frac = frac + delta; x = x >> 1; };
            delta = delta >> 1;
        };
        fixed_point64::create_from_raw_value (((integer_part as u128) << 64) + frac)
    }
}
```


<a id="0x1_math128_sqrt"></a>

## Function `sqrt`

Returns square root of x, precisely floor(sqrt(x))


```move
module 0x1::math128 {
    public fun sqrt(x: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public fun sqrt(x: u128): u128 {
        if (x == 0) return 0;
        // Note the plus 1 in the expression. Let n = floor_lg2(x) we have x in [2^n, 2^{n+1}) and thus the answer in
        // the half-open interval [2^(n/2), 2^{(n+1)/2}). For even n we can write this as [2^(n/2), sqrt(2) 2^{n/2})
        // for odd n [2^((n+1)/2)/sqrt(2), 2^((n+1)/2). For even n the left end point is integer for odd the right
        // end point is integer. If we choose as our first approximation the integer end point we have as maximum
        // relative error either (sqrt(2) - 1) or (1 - 1/sqrt(2)) both are smaller then 1/2.
        let res = 1 << ((floor_log2(x) + 1) >> 1);
        // We use standard newton-rhapson iteration to improve the initial approximation.
        // The error term evolves as delta_i+1 = delta_i^2 / 2 (quadratic convergence).
        // It turns out that after 5 iterations the delta is smaller than 2^-64 and thus below the treshold.
        res = (res + x / res) >> 1;
        res = (res + x / res) >> 1;
        res = (res + x / res) >> 1;
        res = (res + x / res) >> 1;
        res = (res + x / res) >> 1;
        min(res, x / res)
    }
}
```


<a id="0x1_math128_ceil_div"></a>

## Function `ceil_div`



```move
module 0x1::math128 {
    public fun ceil_div(x: u128, y: u128): u128
}
```


##### Implementation


```move
module 0x1::math128 {
    public inline fun ceil_div(x: u128, y: u128): u128 {
        // ceil_div(x, y) = floor((x + y - 1) / y) = floor((x - 1) / y) + 1
        // (x + y - 1) could spuriously overflow. so we use the later version
        if (x == 0) {
            // Inline functions cannot take constants, as then every module using it needs the constant
            assert!(y != 0, std::error::invalid_argument(4));
            0
        }
        else (x - 1) / y + 1
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_max"></a>

### Function `max`


```move
module 0x1::math128 {
    public fun max(a: u128, b: u128): u128
}
```



```move
module 0x1::math128 {
    aborts_if false;
    ensures a >= b ==> result == a;
    ensures a < b ==> result == b;
}
```


<a id="@Specification_1_min"></a>

### Function `min`


```move
module 0x1::math128 {
    public fun min(a: u128, b: u128): u128
}
```



```move
module 0x1::math128 {
    aborts_if false;
    ensures a < b ==> result == a;
    ensures a >= b ==> result == b;
}
```


<a id="@Specification_1_average"></a>

### Function `average`


```move
module 0x1::math128 {
    public fun average(a: u128, b: u128): u128
}
```



```move
module 0x1::math128 {
    pragma opaque;
    aborts_if false;
    ensures result == (a + b) / 2;
}
```


<a id="@Specification_1_clamp"></a>

### Function `clamp`


```move
module 0x1::math128 {
    public fun clamp(x: u128, lower: u128, upper: u128): u128
}
```



```move
module 0x1::math128 {
    requires (lower <= upper);
    aborts_if false;
    ensures (lower <=x && x <= upper) ==> result == x;
    ensures (x < lower) ==> result == lower;
    ensures (upper < x) ==> result == upper;
}
```


<a id="@Specification_1_pow"></a>

### Function `pow`


```move
module 0x1::math128 {
    public fun pow(n: u128, e: u128): u128
}
```



```move
module 0x1::math128 {
    pragma opaque;
    aborts_if [abstract] spec_pow(n, e) > MAX_U128;
    ensures [abstract] result == spec_pow(n, e);
}
```


<a id="@Specification_1_floor_log2"></a>

### Function `floor_log2`


```move
module 0x1::math128 {
    public fun floor_log2(x: u128): u8
}
```



```move
module 0x1::math128 {
    pragma opaque;
    aborts_if [abstract] x == 0;
    ensures [abstract] spec_pow(2, result) <= x;
    ensures [abstract] x < spec_pow(2, result+1);
}
```


<a id="@Specification_1_sqrt"></a>

### Function `sqrt`


```move
module 0x1::math128 {
    public fun sqrt(x: u128): u128
}
```



```move
module 0x1::math128 {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] x > 0 ==> result * result <= x;
    ensures [abstract] x > 0 ==> x < (result+1) * (result+1);
}
```



<a id="0x1_math128_spec_pow"></a>


```move
module 0x1::math128 {
    fun spec_pow(n: u128, e: u128): u128 {
       if (e == 0) {
           1
       }
       else {
           n * spec_pow(n, e-1)
       }
    }
}
```
