
<a id="0x1_copyable_any"></a>

# Module `0x1::copyable_any`



-  [Struct `Any`](#0x1_copyable_any_Any)
-  [Constants](#@Constants_0)
-  [Function `pack`](#0x1_copyable_any_pack)
-  [Function `unpack`](#0x1_copyable_any_unpack)
-  [Function `type_name`](#0x1_copyable_any_type_name)
-  [Specification](#@Specification_1)
    -  [Function `pack`](#@Specification_1_pack)
    -  [Function `unpack`](#@Specification_1_unpack)
    -  [Function `type_name`](#@Specification_1_type_name)


```move
module 0x1::copyable_any {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::from_bcs;
    use 0x1::string;
    use 0x1::type_info;
}
```


<a id="0x1_copyable_any_Any"></a>

## Struct `Any`

The same as `any::Any` but with the copy ability.


```move
module 0x1::copyable_any {
    struct Any has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`type_name: string::String`
</dt>
<dd>

</dd>
<dt>
`data: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_copyable_any_ETYPE_MISMATCH"></a>

The type provided for `unpack` is not the same as was given for `pack`.


```move
module 0x1::copyable_any {
    const ETYPE_MISMATCH: u64 = 0;
}
```


<a id="0x1_copyable_any_pack"></a>

## Function `pack`

Pack a value into the `Any` representation. Because Any can be stored, dropped, and copied this is
also required from `T`.


```move
module 0x1::copyable_any {
    public fun pack<T: copy, drop, store>(x: T): copyable_any::Any
}
```


##### Implementation


```move
module 0x1::copyable_any {
    public fun pack<T: drop + store + copy>(x: T): Any {
        Any {
            type_name: type_info::type_name<T>(),
            data: bcs::to_bytes(&x)
        }
    }
}
```


<a id="0x1_copyable_any_unpack"></a>

## Function `unpack`

Unpack a value from the `Any` representation. This aborts if the value has not the expected type `T`.


```move
module 0x1::copyable_any {
    public fun unpack<T>(x: copyable_any::Any): T
}
```


##### Implementation


```move
module 0x1::copyable_any {
    public fun unpack<T>(x: Any): T {
        assert!(type_info::type_name<T>() == x.type_name, error::invalid_argument(ETYPE_MISMATCH));
        from_bytes<T>(x.data)
    }
}
```


<a id="0x1_copyable_any_type_name"></a>

## Function `type_name`

Returns the type name of this Any


```move
module 0x1::copyable_any {
    public fun type_name(x: &copyable_any::Any): &string::String
}
```


##### Implementation


```move
module 0x1::copyable_any {
    public fun type_name(x: &Any): &String {
        &x.type_name
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_pack"></a>

### Function `pack`


```move
module 0x1::copyable_any {
    public fun pack<T: copy, drop, store>(x: T): copyable_any::Any
}
```



```move
module 0x1::copyable_any {
    aborts_if false;
    pragma opaque;
    ensures result == Any {
        type_name: type_info::type_name<T>(),
        data: bcs::serialize<T>(x)
    };
    ensures [abstract] from_bcs::deserializable<T>(result.data);
}
```


<a id="@Specification_1_unpack"></a>

### Function `unpack`


```move
module 0x1::copyable_any {
    public fun unpack<T>(x: copyable_any::Any): T
}
```



```move
module 0x1::copyable_any {
    include UnpackAbortsIf<T>;
    ensures result == from_bcs::deserialize<T>(x.data);
}
```



<a id="0x1_copyable_any_UnpackAbortsIf"></a>


```move
module 0x1::copyable_any {
    schema UnpackAbortsIf<T> {
        x: Any;
        aborts_if type_info::type_name<T>() != x.type_name;
        aborts_if !from_bcs::deserializable<T>(x.data);
    }
}
```


<a id="@Specification_1_type_name"></a>

### Function `type_name`


```move
module 0x1::copyable_any {
    public fun type_name(x: &copyable_any::Any): &string::String
}
```



```move
module 0x1::copyable_any {
    aborts_if false;
    ensures result == x.type_name;
}
```
