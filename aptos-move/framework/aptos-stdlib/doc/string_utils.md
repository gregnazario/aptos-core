
<a id="0x1_string_utils"></a>

# Module `0x1::string_utils`

A module for formatting move values as strings.


-  [Struct `Cons`](#0x1_string_utils_Cons)
-  [Struct `NIL`](#0x1_string_utils_NIL)
-  [Struct `FakeCons`](#0x1_string_utils_FakeCons)
    -  [[test_only]](#@[test_only]_0)
-  [Constants](#@Constants_1)
-  [Function `to_string`](#0x1_string_utils_to_string)
-  [Function `to_string_with_canonical_addresses`](#0x1_string_utils_to_string_with_canonical_addresses)
-  [Function `to_string_with_integer_types`](#0x1_string_utils_to_string_with_integer_types)
-  [Function `debug_string`](#0x1_string_utils_debug_string)
-  [Function `format1`](#0x1_string_utils_format1)
-  [Function `format2`](#0x1_string_utils_format2)
-  [Function `format3`](#0x1_string_utils_format3)
-  [Function `format4`](#0x1_string_utils_format4)
-  [Function `cons`](#0x1_string_utils_cons)
-  [Function `nil`](#0x1_string_utils_nil)
-  [Function `list1`](#0x1_string_utils_list1)
-  [Function `list2`](#0x1_string_utils_list2)
-  [Function `list3`](#0x1_string_utils_list3)
-  [Function `list4`](#0x1_string_utils_list4)
-  [Function `native_format`](#0x1_string_utils_native_format)
-  [Function `native_format_list`](#0x1_string_utils_native_format_list)
-  [Specification](#@Specification_2)
    -  [Function `to_string`](#@Specification_2_to_string)
    -  [Function `to_string_with_canonical_addresses`](#@Specification_2_to_string_with_canonical_addresses)
    -  [Function `to_string_with_integer_types`](#@Specification_2_to_string_with_integer_types)
    -  [Function `debug_string`](#@Specification_2_debug_string)
    -  [Function `format1`](#@Specification_2_format1)
    -  [Function `format2`](#@Specification_2_format2)
    -  [Function `format3`](#@Specification_2_format3)
    -  [Function `format4`](#@Specification_2_format4)
    -  [Function `native_format`](#@Specification_2_native_format)
    -  [Function `native_format_list`](#@Specification_2_native_format_list)


```move
module 0x1::string_utils {
    use 0x1::string;
}
```


<a id="0x1_string_utils_Cons"></a>

## Struct `Cons`



```move
module 0x1::string_utils {
    struct Cons<T, N> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`car: T`
</dt>
<dd>

</dd>
<dt>
`cdr: N`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_string_utils_NIL"></a>

## Struct `NIL`



```move
module 0x1::string_utils {
    struct NIL has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_string_utils_FakeCons"></a>

## Struct `FakeCons`


<a id="@[test_only]_0"></a>

### [test_only]



```move
module 0x1::string_utils {
    struct FakeCons<T, N> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`car: T`
</dt>
<dd>

</dd>
<dt>
`cdr: N`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_1"></a>

## Constants


<a id="0x1_string_utils_EARGS_MISMATCH"></a>

The number of values in the list does not match the number of &quot;&#123;&#125;&quot; in the format string.


```move
module 0x1::string_utils {
    const EARGS_MISMATCH: u64 = 1;
}
```


<a id="0x1_string_utils_EINVALID_FORMAT"></a>

The format string is not valid.


```move
module 0x1::string_utils {
    const EINVALID_FORMAT: u64 = 2;
}
```


<a id="0x1_string_utils_EUNABLE_TO_FORMAT_DELAYED_FIELD"></a>

Formatting is not possible because the value contains delayed fields such as aggregators.


```move
module 0x1::string_utils {
    const EUNABLE_TO_FORMAT_DELAYED_FIELD: u64 = 3;
}
```


<a id="0x1_string_utils_to_string"></a>

## Function `to_string`

Format a move value as a human readable string,
eg. `to_string(&1u64) == "1"`, `to_string(&false) == "false"`, `to_string(&@0x1) == "@0x1"`.
For vectors and structs the format is similar to rust, eg.
`to_string(&cons(1,2)) == "Cons { car: 1, cdr: 2 }"` and `to_string(&vector[1, 2, 3]) == "[ 1, 2, 3 ]"`
For vectors of u8 the output is hex encoded, eg. `to_string(&vector[1u8, 2u8, 3u8]) == "0x010203"`
For std::string::String the output is the string itself including quotes, eg.
`to_string(&std::string::utf8(b"My string")) == "\"My string\""`


```move
module 0x1::string_utils {
    public fun to_string<T>(s: &T): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun to_string<T>(s: &T): String {
        native_format(s, false, false, true, false)
    }
}
```


<a id="0x1_string_utils_to_string_with_canonical_addresses"></a>

## Function `to_string_with_canonical_addresses`

Format addresses as 64 zero&#45;padded hexadecimals.


```move
module 0x1::string_utils {
    public fun to_string_with_canonical_addresses<T>(s: &T): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun to_string_with_canonical_addresses<T>(s: &T): String {
        native_format(s, false, true, true, false)
    }
}
```


<a id="0x1_string_utils_to_string_with_integer_types"></a>

## Function `to_string_with_integer_types`

Format emitting integers with types ie. 6u8 or 128u32.


```move
module 0x1::string_utils {
    public fun to_string_with_integer_types<T>(s: &T): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun to_string_with_integer_types<T>(s: &T): String {
        native_format(s, false, true, true, false)
    }
}
```


<a id="0x1_string_utils_debug_string"></a>

## Function `debug_string`

Format vectors and structs with newlines and indentation.


```move
module 0x1::string_utils {
    public fun debug_string<T>(s: &T): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun debug_string<T>(s: &T): String {
        native_format(s, true, false, false, false)
    }
}
```


<a id="0x1_string_utils_format1"></a>

## Function `format1`

Formatting with a rust&#45;like format string, eg. `format2(&b"a = {}, b = {}", 1, 2) == "a = 1, b = 2"`.


```move
module 0x1::string_utils {
    public fun format1<T0: drop>(fmt: &vector<u8>, a: T0): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun format1<T0: drop>(fmt: &vector<u8>, a: T0): String {
        native_format_list(fmt, &list1(a))
    }
}
```


<a id="0x1_string_utils_format2"></a>

## Function `format2`



```move
module 0x1::string_utils {
    public fun format2<T0: drop, T1: drop>(fmt: &vector<u8>, a: T0, b: T1): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun format2<T0: drop, T1: drop>(fmt: &vector<u8>, a: T0, b: T1): String {
        native_format_list(fmt, &list2(a, b))
    }
}
```


<a id="0x1_string_utils_format3"></a>

## Function `format3`



```move
module 0x1::string_utils {
    public fun format3<T0: drop, T1: drop, T2: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun format3<T0: drop, T1: drop, T2: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2): String {
        native_format_list(fmt, &list3(a, b, c))
    }
}
```


<a id="0x1_string_utils_format4"></a>

## Function `format4`



```move
module 0x1::string_utils {
    public fun format4<T0: drop, T1: drop, T2: drop, T3: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2, d: T3): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    public fun format4<T0: drop, T1: drop, T2: drop, T3: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2, d: T3): String {
        native_format_list(fmt, &list4(a, b, c, d))
    }
}
```


<a id="0x1_string_utils_cons"></a>

## Function `cons`



```move
module 0x1::string_utils {
    fun cons<T, N>(car: T, cdr: N): string_utils::Cons<T, N>
}
```


##### Implementation


```move
module 0x1::string_utils {
    fun cons<T, N>(car: T, cdr: N): Cons<T, N> { Cons { car, cdr } }
}
```


<a id="0x1_string_utils_nil"></a>

## Function `nil`



```move
module 0x1::string_utils {
    fun nil(): string_utils::NIL
}
```


##### Implementation


```move
module 0x1::string_utils {
    fun nil(): NIL { NIL {} }
}
```


<a id="0x1_string_utils_list1"></a>

## Function `list1`



```move
module 0x1::string_utils {
    fun list1<T0>(a: T0): string_utils::Cons<T0, string_utils::NIL>
}
```


##### Implementation


```move
module 0x1::string_utils {
    inline fun list1<T0>(a: T0): Cons<T0, NIL> { cons(a, nil()) }
}
```


<a id="0x1_string_utils_list2"></a>

## Function `list2`



```move
module 0x1::string_utils {
    fun list2<T0, T1>(a: T0, b: T1): string_utils::Cons<T0, string_utils::Cons<T1, string_utils::NIL>>
}
```


##### Implementation


```move
module 0x1::string_utils {
    inline fun list2<T0, T1>(a: T0, b: T1): Cons<T0, Cons<T1, NIL>> { cons(a, list1(b)) }
}
```


<a id="0x1_string_utils_list3"></a>

## Function `list3`



```move
module 0x1::string_utils {
    fun list3<T0, T1, T2>(a: T0, b: T1, c: T2): string_utils::Cons<T0, string_utils::Cons<T1, string_utils::Cons<T2, string_utils::NIL>>>
}
```


##### Implementation


```move
module 0x1::string_utils {
    inline fun list3<T0, T1, T2>(a: T0, b: T1, c: T2): Cons<T0, Cons<T1, Cons<T2, NIL>>> { cons(a, list2(b, c)) }
}
```


<a id="0x1_string_utils_list4"></a>

## Function `list4`



```move
module 0x1::string_utils {
    fun list4<T0, T1, T2, T3>(a: T0, b: T1, c: T2, d: T3): string_utils::Cons<T0, string_utils::Cons<T1, string_utils::Cons<T2, string_utils::Cons<T3, string_utils::NIL>>>>
}
```


##### Implementation


```move
module 0x1::string_utils {
    inline fun list4<T0, T1, T2, T3>(a: T0, b: T1, c: T2, d: T3): Cons<T0, Cons<T1, Cons<T2, Cons<T3, NIL>>>> { cons(a, list3(b, c, d)) }
}
```


<a id="0x1_string_utils_native_format"></a>

## Function `native_format`



```move
module 0x1::string_utils {
    fun native_format<T>(s: &T, type_tag: bool, canonicalize: bool, single_line: bool, include_int_types: bool): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    native fun native_format<T>(s: &T, type_tag: bool, canonicalize: bool, single_line: bool, include_int_types: bool): String;
}
```


<a id="0x1_string_utils_native_format_list"></a>

## Function `native_format_list`



```move
module 0x1::string_utils {
    fun native_format_list<T>(fmt: &vector<u8>, val: &T): string::String
}
```


##### Implementation


```move
module 0x1::string_utils {
    native fun native_format_list<T>(fmt: &vector<u8>, val: &T): String;
}
```


<a id="@Specification_2"></a>

## Specification


<a id="@Specification_2_to_string"></a>

### Function `to_string`


```move
module 0x1::string_utils {
    public fun to_string<T>(s: &T): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if false;
    ensures result == spec_native_format(s, false, false, true, false);
}
```


<a id="@Specification_2_to_string_with_canonical_addresses"></a>

### Function `to_string_with_canonical_addresses`


```move
module 0x1::string_utils {
    public fun to_string_with_canonical_addresses<T>(s: &T): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if false;
    ensures result == spec_native_format(s, false, true, true, false);
}
```


<a id="@Specification_2_to_string_with_integer_types"></a>

### Function `to_string_with_integer_types`


```move
module 0x1::string_utils {
    public fun to_string_with_integer_types<T>(s: &T): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if false;
    ensures result == spec_native_format(s, false, true, true, false);
}
```


<a id="@Specification_2_debug_string"></a>

### Function `debug_string`


```move
module 0x1::string_utils {
    public fun debug_string<T>(s: &T): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if false;
    ensures result == spec_native_format(s, true, false, false, false);
}
```


<a id="@Specification_2_format1"></a>

### Function `format1`


```move
module 0x1::string_utils {
    public fun format1<T0: drop>(fmt: &vector<u8>, a: T0): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if args_mismatch_or_invalid_format(fmt, list1(a));
    ensures result == spec_native_format_list(fmt, list1(a));
}
```


<a id="@Specification_2_format2"></a>

### Function `format2`


```move
module 0x1::string_utils {
    public fun format2<T0: drop, T1: drop>(fmt: &vector<u8>, a: T0, b: T1): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if args_mismatch_or_invalid_format(fmt, list2(a, b));
    ensures result == spec_native_format_list(fmt, list2(a, b));
}
```


<a id="@Specification_2_format3"></a>

### Function `format3`


```move
module 0x1::string_utils {
    public fun format3<T0: drop, T1: drop, T2: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if args_mismatch_or_invalid_format(fmt, list3(a, b, c));
    ensures result == spec_native_format_list(fmt, list3(a, b, c));
}
```


<a id="@Specification_2_format4"></a>

### Function `format4`


```move
module 0x1::string_utils {
    public fun format4<T0: drop, T1: drop, T2: drop, T3: drop>(fmt: &vector<u8>, a: T0, b: T1, c: T2, d: T3): string::String
}
```



```move
module 0x1::string_utils {
    aborts_if args_mismatch_or_invalid_format(fmt, list4(a, b, c, d));
    ensures result == spec_native_format_list(fmt, list4(a, b, c, d));
}
```


<a id="@Specification_2_native_format"></a>

### Function `native_format`


```move
module 0x1::string_utils {
    fun native_format<T>(s: &T, type_tag: bool, canonicalize: bool, single_line: bool, include_int_types: bool): string::String
}
```



```move
module 0x1::string_utils {
    pragma opaque;
    aborts_if false;
    ensures result == spec_native_format(s, type_tag, canonicalize, single_line, include_int_types);
}
```


<a id="@Specification_2_native_format_list"></a>

### Function `native_format_list`


```move
module 0x1::string_utils {
    fun native_format_list<T>(fmt: &vector<u8>, val: &T): string::String
}
```



```move
module 0x1::string_utils {
    pragma opaque;
    aborts_if args_mismatch_or_invalid_format(fmt, val);
    ensures result == spec_native_format_list(fmt, val);
}
```



<a id="0x1_string_utils_spec_native_format"></a>


```move
module 0x1::string_utils {
    fun spec_native_format<T>(s: T, type_tag: bool, canonicalize: bool, single_line: bool, include_int_types: bool): String;
}
```



<a id="0x1_string_utils_spec_native_format_list"></a>


```move
module 0x1::string_utils {
    fun spec_native_format_list<T>(fmt: vector<u8>, val: T): String;
}
```



<a id="0x1_string_utils_args_mismatch_or_invalid_format"></a>


```move
module 0x1::string_utils {
    fun args_mismatch_or_invalid_format<T>(fmt: vector<u8>, val: T): bool;
}
```
