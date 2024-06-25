
<a id="0x1_any"></a>

# Module `0x1::any`



-  [Struct `Any`](#0x1_any_Any)
-  [Constants](#@Constants_0)
-  [Function `pack`](#0x1_any_pack)
-  [Function `unpack`](#0x1_any_unpack)
-  [Function `type_name`](#0x1_any_type_name)
-  [Specification](#@Specification_1)
    -  [Function `pack`](#@Specification_1_pack)
    -  [Function `unpack`](#@Specification_1_unpack)
    -  [Function `type_name`](#@Specification_1_type_name)


```move
module 0x1::any {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::from_bcs;
    use 0x1::string;
    use 0x1::type_info;
}
```


<a id="0x1_any_Any"></a>

## Struct `Any`

A type which can represent a value of any type. This allows for representation of &apos;unknown&apos; future
values. For example, to define a resource such that it can be later be extended without breaking
changes one can do

```move
struct Resource &#123;
field: Type,
...
extension: Option&lt;Any&gt;
&#125;
```


```move
module 0x1::any {
    struct Any has drop, store
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


<a id="0x1_any_ETYPE_MISMATCH"></a>

The type provided for `unpack` is not the same as was given for `pack`.


```move
module 0x1::any {
    const ETYPE_MISMATCH: u64 = 1;
}
```


<a id="0x1_any_pack"></a>

## Function `pack`

Pack a value into the `Any` representation. Because Any can be stored and dropped, this is
also required from `T`.


```move
module 0x1::any {
    public fun pack<T: drop, store>(x: T): any::Any
}
```


##### Implementation


```move
module 0x1::any {
    public fun pack<T: drop + store>(x: T): Any {
        Any {
            type_name: type_info::type_name<T>(),
            data: to_bytes(&x)
        }
    }
}
```


<a id="0x1_any_unpack"></a>

## Function `unpack`

Unpack a value from the `Any` representation. This aborts if the value has not the expected type `T`.


```move
module 0x1::any {
    public fun unpack<T>(x: any::Any): T
}
```


##### Implementation


```move
module 0x1::any {
    public fun unpack<T>(x: Any): T {
        assert!(type_info::type_name<T>() == x.type_name, error::invalid_argument(ETYPE_MISMATCH));
        from_bytes<T>(x.data)
    }
}
```


<a id="0x1_any_type_name"></a>

## Function `type_name`

Returns the type name of this Any


```move
module 0x1::any {
    public fun type_name(x: &any::Any): &string::String
}
```


##### Implementation


```move
module 0x1::any {
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
module 0x1::any {
    public fun pack<T: drop, store>(x: T): any::Any
}
```



```move
module 0x1::any {
    aborts_if false;
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
module 0x1::any {
    public fun unpack<T>(x: any::Any): T
}
```



```move
module 0x1::any {
    include UnpackAbortsIf<T>;
    ensures result == from_bcs::deserialize<T>(x.data);
}
```



<a id="0x1_any_UnpackAbortsIf"></a>


```move
module 0x1::any {
    schema UnpackAbortsIf<T> {
        x: Any;
        aborts_if type_info::type_name<T>() != x.type_name;
        aborts_if !from_bcs::deserializable<T>(x.data);
    }
}
```



<a id="0x1_any_UnpackRequirement"></a>


```move
module 0x1::any {
    schema UnpackRequirement<T> {
        x: Any;
        requires type_info::type_name<T>() == x.type_name;
        requires from_bcs::deserializable<T>(x.data);
    }
}
```


<a id="@Specification_1_type_name"></a>

### Function `type_name`


```move
module 0x1::any {
    public fun type_name(x: &any::Any): &string::String
}
```



```move
module 0x1::any {
    aborts_if false;
    ensures result == x.type_name;
}
```
