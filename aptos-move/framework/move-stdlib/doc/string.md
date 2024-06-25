
<a id="0x1_string"></a>

# Module `0x1::string`

The `string` module defines the `String` type which represents UTF8 encoded strings.


-  [Struct `String`](#0x1_string_String)
-  [Constants](#@Constants_0)
-  [Function `utf8`](#0x1_string_utf8)
-  [Function `try_utf8`](#0x1_string_try_utf8)
-  [Function `bytes`](#0x1_string_bytes)
-  [Function `is_empty`](#0x1_string_is_empty)
-  [Function `length`](#0x1_string_length)
-  [Function `append`](#0x1_string_append)
-  [Function `append_utf8`](#0x1_string_append_utf8)
-  [Function `insert`](#0x1_string_insert)
-  [Function `sub_string`](#0x1_string_sub_string)
-  [Function `index_of`](#0x1_string_index_of)
-  [Function `internal_check_utf8`](#0x1_string_internal_check_utf8)
-  [Function `internal_is_char_boundary`](#0x1_string_internal_is_char_boundary)
-  [Function `internal_sub_string`](#0x1_string_internal_sub_string)
-  [Function `internal_index_of`](#0x1_string_internal_index_of)
-  [Specification](#@Specification_1)
    -  [Function `internal_check_utf8`](#@Specification_1_internal_check_utf8)
    -  [Function `internal_is_char_boundary`](#@Specification_1_internal_is_char_boundary)
    -  [Function `internal_sub_string`](#@Specification_1_internal_sub_string)
    -  [Function `internal_index_of`](#@Specification_1_internal_index_of)


```move
module 0x1::string {
    use 0x1::option;
    use 0x1::vector;
}
```


<a id="0x1_string_String"></a>

## Struct `String`

A `String` holds a sequence of bytes which is guaranteed to be in utf8 format.


```move
module 0x1::string {
    struct String has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`bytes: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_string_EINVALID_INDEX"></a>

Index out of range.


```move
module 0x1::string {
    const EINVALID_INDEX: u64 = 2;
}
```


<a id="0x1_string_EINVALID_UTF8"></a>

An invalid UTF8 encoding.


```move
module 0x1::string {
    const EINVALID_UTF8: u64 = 1;
}
```


<a id="0x1_string_utf8"></a>

## Function `utf8`

Creates a new string from a sequence of bytes. Aborts if the bytes do not represent valid utf8.


```move
module 0x1::string {
    public fun utf8(bytes: vector<u8>): string::String
}
```


##### Implementation


```move
module 0x1::string {
    public fun utf8(bytes: vector<u8>): String {
        assert!(internal_check_utf8(&bytes), EINVALID_UTF8);
        String{bytes}
    }
}
```


<a id="0x1_string_try_utf8"></a>

## Function `try_utf8`

Tries to create a new string from a sequence of bytes.


```move
module 0x1::string {
    public fun try_utf8(bytes: vector<u8>): option::Option<string::String>
}
```


##### Implementation


```move
module 0x1::string {
    public fun try_utf8(bytes: vector<u8>): Option<String> {
        if (internal_check_utf8(&bytes)) {
            option::some(String{bytes})
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_string_bytes"></a>

## Function `bytes`

Returns a reference to the underlying byte vector.


```move
module 0x1::string {
    public fun bytes(s: &string::String): &vector<u8>
}
```


##### Implementation


```move
module 0x1::string {
    public fun bytes(s: &String): &vector<u8> {
        &s.bytes
    }
}
```


<a id="0x1_string_is_empty"></a>

## Function `is_empty`

Checks whether this string is empty.


```move
module 0x1::string {
    public fun is_empty(s: &string::String): bool
}
```


##### Implementation


```move
module 0x1::string {
    public fun is_empty(s: &String): bool {
        vector::is_empty(&s.bytes)
    }
}
```


<a id="0x1_string_length"></a>

## Function `length`

Returns the length of this string, in bytes.


```move
module 0x1::string {
    public fun length(s: &string::String): u64
}
```


##### Implementation


```move
module 0x1::string {
    public fun length(s: &String): u64 {
        vector::length(&s.bytes)
    }
}
```


<a id="0x1_string_append"></a>

## Function `append`

Appends a string.


```move
module 0x1::string {
    public fun append(s: &mut string::String, r: string::String)
}
```


##### Implementation


```move
module 0x1::string {
    public fun append(s: &mut String, r: String) {
        vector::append(&mut s.bytes, r.bytes)
    }
}
```


<a id="0x1_string_append_utf8"></a>

## Function `append_utf8`

Appends bytes which must be in valid utf8 format.


```move
module 0x1::string {
    public fun append_utf8(s: &mut string::String, bytes: vector<u8>)
}
```


##### Implementation


```move
module 0x1::string {
    public fun append_utf8(s: &mut String, bytes: vector<u8>) {
        append(s, utf8(bytes))
    }
}
```


<a id="0x1_string_insert"></a>

## Function `insert`

Insert the other string at the byte index in given string. The index must be at a valid utf8 char
boundary.


```move
module 0x1::string {
    public fun insert(s: &mut string::String, at: u64, o: string::String)
}
```


##### Implementation


```move
module 0x1::string {
    public fun insert(s: &mut String, at: u64, o: String) {
        let bytes = &s.bytes;
        assert!(at <= vector::length(bytes) && internal_is_char_boundary(bytes, at), EINVALID_INDEX);
        let l = length(s);
        let front = sub_string(s, 0, at);
        let end = sub_string(s, at, l);
        append(&mut front, o);
        append(&mut front, end);
        *s = front;
    }
}
```


<a id="0x1_string_sub_string"></a>

## Function `sub_string`

Returns a sub&#45;string using the given byte indices, where `i` is the first byte position and `j` is the start
of the first byte not included (or the length of the string). The indices must be at valid utf8 char boundaries,
guaranteeing that the result is valid utf8.


```move
module 0x1::string {
    public fun sub_string(s: &string::String, i: u64, j: u64): string::String
}
```


##### Implementation


```move
module 0x1::string {
    public fun sub_string(s: &String, i: u64, j: u64): String {
        let bytes = &s.bytes;
        let l = vector::length(bytes);
        assert!(
            j <= l && i <= j && internal_is_char_boundary(bytes, i) && internal_is_char_boundary(bytes, j),
            EINVALID_INDEX
        );
        String { bytes: internal_sub_string(bytes, i, j) }
    }
}
```


<a id="0x1_string_index_of"></a>

## Function `index_of`

Computes the index of the first occurrence of a string. Returns `length(s)` if no occurrence found.


```move
module 0x1::string {
    public fun index_of(s: &string::String, r: &string::String): u64
}
```


##### Implementation


```move
module 0x1::string {
    public fun index_of(s: &String, r: &String): u64 {
        internal_index_of(&s.bytes, &r.bytes)
    }
}
```


<a id="0x1_string_internal_check_utf8"></a>

## Function `internal_check_utf8`



```move
module 0x1::string {
    public fun internal_check_utf8(v: &vector<u8>): bool
}
```


##### Implementation


```move
module 0x1::string {
    public native fun internal_check_utf8(v: &vector<u8>): bool;
}
```


<a id="0x1_string_internal_is_char_boundary"></a>

## Function `internal_is_char_boundary`



```move
module 0x1::string {
    fun internal_is_char_boundary(v: &vector<u8>, i: u64): bool
}
```


##### Implementation


```move
module 0x1::string {
    native fun internal_is_char_boundary(v: &vector<u8>, i: u64): bool;
}
```


<a id="0x1_string_internal_sub_string"></a>

## Function `internal_sub_string`



```move
module 0x1::string {
    fun internal_sub_string(v: &vector<u8>, i: u64, j: u64): vector<u8>
}
```


##### Implementation


```move
module 0x1::string {
    native fun internal_sub_string(v: &vector<u8>, i: u64, j: u64): vector<u8>;
}
```


<a id="0x1_string_internal_index_of"></a>

## Function `internal_index_of`



```move
module 0x1::string {
    fun internal_index_of(v: &vector<u8>, r: &vector<u8>): u64
}
```


##### Implementation


```move
module 0x1::string {
    native fun internal_index_of(v: &vector<u8>, r: &vector<u8>): u64;
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_internal_check_utf8"></a>

### Function `internal_check_utf8`


```move
module 0x1::string {
    public fun internal_check_utf8(v: &vector<u8>): bool
}
```



```move
module 0x1::string {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_internal_check_utf8(v);
}
```


<a id="@Specification_1_internal_is_char_boundary"></a>

### Function `internal_is_char_boundary`


```move
module 0x1::string {
    fun internal_is_char_boundary(v: &vector<u8>, i: u64): bool
}
```



```move
module 0x1::string {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_internal_is_char_boundary(v, i);
}
```


<a id="@Specification_1_internal_sub_string"></a>

### Function `internal_sub_string`


```move
module 0x1::string {
    fun internal_sub_string(v: &vector<u8>, i: u64, j: u64): vector<u8>
}
```



```move
module 0x1::string {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_internal_sub_string(v, i, j);
}
```


<a id="@Specification_1_internal_index_of"></a>

### Function `internal_index_of`


```move
module 0x1::string {
    fun internal_index_of(v: &vector<u8>, r: &vector<u8>): u64
}
```



```move
module 0x1::string {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_internal_index_of(v, r);
}
```



<a id="0x1_string_spec_utf8"></a>


```move
module 0x1::string {
    fun spec_utf8(bytes: vector<u8>): String {
       String{bytes}
    }
}
```



<a id="0x1_string_spec_internal_check_utf8"></a>


```move
module 0x1::string {
    fun spec_internal_check_utf8(v: vector<u8>): bool;
<a id="0x1_string_spec_internal_is_char_boundary"></a>
    fun spec_internal_is_char_boundary(v: vector<u8>, i: u64): bool;
<a id="0x1_string_spec_internal_sub_string"></a>
    fun spec_internal_sub_string(v: vector<u8>, i: u64, j: u64): vector<u8>;
<a id="0x1_string_spec_internal_index_of"></a>
    fun spec_internal_index_of(v: vector<u8>, r: vector<u8>): u64;
}
```
