
<a id="0x3_property_map"></a>

# Module `0x3::property_map`

PropertyMap is a specialization of SimpleMap for Tokens.
It maps a String key to a PropertyValue that consists of type (string) and value (vector&lt;u8&gt;)
It provides basic on&#45;chain serialization of primitive and string to property value with type information
It also supports deserializing property value to it original type.


-  [Struct `PropertyMap`](#0x3_property_map_PropertyMap)
-  [Struct `PropertyValue`](#0x3_property_map_PropertyValue)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_property_map_new)
-  [Function `new_with_key_and_property_value`](#0x3_property_map_new_with_key_and_property_value)
-  [Function `empty`](#0x3_property_map_empty)
-  [Function `contains_key`](#0x3_property_map_contains_key)
-  [Function `add`](#0x3_property_map_add)
-  [Function `length`](#0x3_property_map_length)
-  [Function `borrow`](#0x3_property_map_borrow)
-  [Function `keys`](#0x3_property_map_keys)
-  [Function `types`](#0x3_property_map_types)
-  [Function `values`](#0x3_property_map_values)
-  [Function `read_string`](#0x3_property_map_read_string)
-  [Function `read_u8`](#0x3_property_map_read_u8)
-  [Function `read_u64`](#0x3_property_map_read_u64)
-  [Function `read_address`](#0x3_property_map_read_address)
-  [Function `read_u128`](#0x3_property_map_read_u128)
-  [Function `read_bool`](#0x3_property_map_read_bool)
-  [Function `borrow_value`](#0x3_property_map_borrow_value)
-  [Function `borrow_type`](#0x3_property_map_borrow_type)
-  [Function `remove`](#0x3_property_map_remove)
-  [Function `update_property_map`](#0x3_property_map_update_property_map)
-  [Function `update_property_value`](#0x3_property_map_update_property_value)
-  [Function `create_property_value_raw`](#0x3_property_map_create_property_value_raw)
-  [Function `create_property_value`](#0x3_property_map_create_property_value)
-  [Specification](#@Specification_1)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `new_with_key_and_property_value`](#@Specification_1_new_with_key_and_property_value)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `contains_key`](#@Specification_1_contains_key)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `keys`](#@Specification_1_keys)
    -  [Function `types`](#@Specification_1_types)
    -  [Function `values`](#@Specification_1_values)
    -  [Function `read_string`](#@Specification_1_read_string)
    -  [Function `read_u8`](#@Specification_1_read_u8)
    -  [Function `read_u64`](#@Specification_1_read_u64)
    -  [Function `read_address`](#@Specification_1_read_address)
    -  [Function `read_u128`](#@Specification_1_read_u128)
    -  [Function `read_bool`](#@Specification_1_read_bool)
    -  [Function `borrow_value`](#@Specification_1_borrow_value)
    -  [Function `borrow_type`](#@Specification_1_borrow_type)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `update_property_map`](#@Specification_1_update_property_map)
    -  [Function `update_property_value`](#@Specification_1_update_property_value)
    -  [Function `create_property_value_raw`](#@Specification_1_create_property_value_raw)
    -  [Function `create_property_value`](#@Specification_1_create_property_value)


```move
module 0x3::property_map {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::from_bcs;
    use 0x1::simple_map;
    use 0x1::string;
    use 0x1::type_info;
}
```


<a id="0x3_property_map_PropertyMap"></a>

## Struct `PropertyMap`



```move
module 0x3::property_map {
    struct PropertyMap has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`map: simple_map::SimpleMap<string::String, property_map::PropertyValue>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_property_map_PropertyValue"></a>

## Struct `PropertyValue`



```move
module 0x3::property_map {
    struct PropertyValue has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`value: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`type: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x3_property_map_EKEY_AREADY_EXIST_IN_PROPERTY_MAP"></a>

The property key already exists


```move
module 0x3::property_map {
    const EKEY_AREADY_EXIST_IN_PROPERTY_MAP: u64 = 1;
}
```


<a id="0x3_property_map_EKEY_COUNT_NOT_MATCH_TYPE_COUNT"></a>

Property key and type count don&apos;t match


```move
module 0x3::property_map {
    const EKEY_COUNT_NOT_MATCH_TYPE_COUNT: u64 = 5;
}
```


<a id="0x3_property_map_EKEY_COUNT_NOT_MATCH_VALUE_COUNT"></a>

Property key and value count don&apos;t match


```move
module 0x3::property_map {
    const EKEY_COUNT_NOT_MATCH_VALUE_COUNT: u64 = 4;
}
```


<a id="0x3_property_map_EPROPERTY_MAP_NAME_TOO_LONG"></a>

The name (key) of the property is too long


```move
module 0x3::property_map {
    const EPROPERTY_MAP_NAME_TOO_LONG: u64 = 7;
}
```


<a id="0x3_property_map_EPROPERTY_NOT_EXIST"></a>

The property doesn&apos;t exist


```move
module 0x3::property_map {
    const EPROPERTY_NOT_EXIST: u64 = 3;
}
```


<a id="0x3_property_map_EPROPERTY_NUMBER_EXCEED_LIMIT"></a>

The number of property exceeds the limit


```move
module 0x3::property_map {
    const EPROPERTY_NUMBER_EXCEED_LIMIT: u64 = 2;
}
```


<a id="0x3_property_map_ETYPE_NOT_MATCH"></a>

Property type doesn&apos;t match


```move
module 0x3::property_map {
    const ETYPE_NOT_MATCH: u64 = 6;
}
```


<a id="0x3_property_map_MAX_PROPERTY_MAP_SIZE"></a>

The maximal number of property that can be stored in property map


```move
module 0x3::property_map {
    const MAX_PROPERTY_MAP_SIZE: u64 = 1000;
}
```


<a id="0x3_property_map_MAX_PROPERTY_NAME_LENGTH"></a>



```move
module 0x3::property_map {
    const MAX_PROPERTY_NAME_LENGTH: u64 = 128;
}
```


<a id="0x3_property_map_new"></a>

## Function `new`



```move
module 0x3::property_map {
    public fun new(keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>): property_map::PropertyMap
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun new(
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>
    ): PropertyMap {
        let length = vector::length(&keys);
        assert!(length <= MAX_PROPERTY_MAP_SIZE, error::invalid_argument(EPROPERTY_NUMBER_EXCEED_LIMIT));
        assert!(length == vector::length(&values), error::invalid_argument(EKEY_COUNT_NOT_MATCH_VALUE_COUNT));
        assert!(length == vector::length(&types), error::invalid_argument(EKEY_COUNT_NOT_MATCH_TYPE_COUNT));

        let properties = empty();

        let i = 0;
        while (i < length) {
            let key = *vector::borrow(&keys, i);
            assert!(string::length(&key) <= MAX_PROPERTY_NAME_LENGTH, error::invalid_argument(EPROPERTY_MAP_NAME_TOO_LONG));
            simple_map::add(
                &mut properties.map,
                key,
                PropertyValue { value: *vector::borrow(&values, i), type: *vector::borrow(&types, i) }
            );
            i = i + 1;
        };
        properties
    }
}
```


<a id="0x3_property_map_new_with_key_and_property_value"></a>

## Function `new_with_key_and_property_value`

Create property map directly from key and property value


```move
module 0x3::property_map {
    public fun new_with_key_and_property_value(keys: vector<string::String>, values: vector<property_map::PropertyValue>): property_map::PropertyMap
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun new_with_key_and_property_value(
        keys: vector<String>,
        values: vector<PropertyValue>
    ): PropertyMap {
        let length = vector::length(&keys);
        assert!(length <= MAX_PROPERTY_MAP_SIZE, error::invalid_argument(EPROPERTY_NUMBER_EXCEED_LIMIT));
        assert!(length == vector::length(&values), error::invalid_argument(EKEY_COUNT_NOT_MATCH_VALUE_COUNT));

        let properties = empty();

        let i = 0;
        while (i < length) {
            let key = *vector::borrow(&keys, i);
            let val = *vector::borrow(&values, i);
            assert!(string::length(&key) <= MAX_PROPERTY_NAME_LENGTH, error::invalid_argument(EPROPERTY_MAP_NAME_TOO_LONG));
            add(&mut properties, key, val);
            i = i + 1;
        };
        properties
    }
}
```


<a id="0x3_property_map_empty"></a>

## Function `empty`



```move
module 0x3::property_map {
    public fun empty(): property_map::PropertyMap
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun empty(): PropertyMap {
        PropertyMap {
            map: simple_map::create<String, PropertyValue>(),
        }
    }
}
```


<a id="0x3_property_map_contains_key"></a>

## Function `contains_key`



```move
module 0x3::property_map {
    public fun contains_key(map: &property_map::PropertyMap, key: &string::String): bool
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun contains_key(map: &PropertyMap, key: &String): bool {
        simple_map::contains_key(&map.map, key)
    }
}
```


<a id="0x3_property_map_add"></a>

## Function `add`



```move
module 0x3::property_map {
    public fun add(map: &mut property_map::PropertyMap, key: string::String, value: property_map::PropertyValue)
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun add(map: &mut PropertyMap, key: String, value: PropertyValue) {
        assert!(string::length(&key) <= MAX_PROPERTY_NAME_LENGTH, error::invalid_argument(EPROPERTY_MAP_NAME_TOO_LONG));
        assert!(simple_map::length(&map.map) < MAX_PROPERTY_MAP_SIZE, error::invalid_state(EPROPERTY_NUMBER_EXCEED_LIMIT));
        simple_map::add(&mut map.map, key, value);
    }
}
```


<a id="0x3_property_map_length"></a>

## Function `length`



```move
module 0x3::property_map {
    public fun length(map: &property_map::PropertyMap): u64
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun length(map: &PropertyMap): u64 {
        simple_map::length(&map.map)
    }
}
```


<a id="0x3_property_map_borrow"></a>

## Function `borrow`



```move
module 0x3::property_map {
    public fun borrow(map: &property_map::PropertyMap, key: &string::String): &property_map::PropertyValue
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun borrow(map: &PropertyMap, key: &String): &PropertyValue {
        let found = contains_key(map, key);
        assert!(found, EPROPERTY_NOT_EXIST);
        simple_map::borrow(&map.map, key)
    }
}
```


<a id="0x3_property_map_keys"></a>

## Function `keys`

Return all the keys in the property map in the order they are added.


```move
module 0x3::property_map {
    public fun keys(map: &property_map::PropertyMap): vector<string::String>
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun keys(map: &PropertyMap): vector<String> {
        simple_map::keys(&map.map)
    }
}
```


<a id="0x3_property_map_types"></a>

## Function `types`

Return the types of all properties in the property map in the order they are added.


```move
module 0x3::property_map {
    public fun types(map: &property_map::PropertyMap): vector<string::String>
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun types(map: &PropertyMap): vector<String> {
        vector::map_ref(&simple_map::values(&map.map), |v| {
            let v: &PropertyValue = v;
            v.type
        })
    }
}
```


<a id="0x3_property_map_values"></a>

## Function `values`

Return the values of all properties in the property map in the order they are added.


```move
module 0x3::property_map {
    public fun values(map: &property_map::PropertyMap): vector<vector<u8>>
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun values(map: &PropertyMap): vector<vector<u8>> {
        vector::map_ref(&simple_map::values(&map.map), |v| {
            let v: &PropertyValue = v;
            v.value
        })
    }
}
```


<a id="0x3_property_map_read_string"></a>

## Function `read_string`



```move
module 0x3::property_map {
    public fun read_string(map: &property_map::PropertyMap, key: &string::String): string::String
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_string(map: &PropertyMap, key: &String): String {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"0x1::string::String"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_string(prop.value)
    }
}
```


<a id="0x3_property_map_read_u8"></a>

## Function `read_u8`



```move
module 0x3::property_map {
    public fun read_u8(map: &property_map::PropertyMap, key: &string::String): u8
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_u8(map: &PropertyMap, key: &String): u8 {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"u8"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_u8(prop.value)
    }
}
```


<a id="0x3_property_map_read_u64"></a>

## Function `read_u64`



```move
module 0x3::property_map {
    public fun read_u64(map: &property_map::PropertyMap, key: &string::String): u64
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_u64(map: &PropertyMap, key: &String): u64 {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"u64"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_u64(prop.value)
    }
}
```


<a id="0x3_property_map_read_address"></a>

## Function `read_address`



```move
module 0x3::property_map {
    public fun read_address(map: &property_map::PropertyMap, key: &string::String): address
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_address(map: &PropertyMap, key: &String): address {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"address"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_address(prop.value)
    }
}
```


<a id="0x3_property_map_read_u128"></a>

## Function `read_u128`



```move
module 0x3::property_map {
    public fun read_u128(map: &property_map::PropertyMap, key: &string::String): u128
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_u128(map: &PropertyMap, key: &String): u128 {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"u128"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_u128(prop.value)
    }
}
```


<a id="0x3_property_map_read_bool"></a>

## Function `read_bool`



```move
module 0x3::property_map {
    public fun read_bool(map: &property_map::PropertyMap, key: &string::String): bool
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun read_bool(map: &PropertyMap, key: &String): bool {
        let prop = borrow(map, key);
        assert!(prop.type == string::utf8(b"bool"), error::invalid_state(ETYPE_NOT_MATCH));
        from_bcs::to_bool(prop.value)
    }
}
```


<a id="0x3_property_map_borrow_value"></a>

## Function `borrow_value`



```move
module 0x3::property_map {
    public fun borrow_value(property: &property_map::PropertyValue): vector<u8>
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun borrow_value(property: &PropertyValue): vector<u8> {
        property.value
    }
}
```


<a id="0x3_property_map_borrow_type"></a>

## Function `borrow_type`



```move
module 0x3::property_map {
    public fun borrow_type(property: &property_map::PropertyValue): string::String
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun borrow_type(property: &PropertyValue): String {
        property.type
    }
}
```


<a id="0x3_property_map_remove"></a>

## Function `remove`



```move
module 0x3::property_map {
    public fun remove(map: &mut property_map::PropertyMap, key: &string::String): (string::String, property_map::PropertyValue)
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun remove(
        map: &mut PropertyMap,
        key: &String
    ): (String, PropertyValue) {
        let found = contains_key(map, key);
        assert!(found, error::not_found(EPROPERTY_NOT_EXIST));
        simple_map::remove(&mut map.map, key)
    }
}
```


<a id="0x3_property_map_update_property_map"></a>

## Function `update_property_map`

Update the property in the existing property map
Allow updating existing keys&apos; value and add new key&#45;value pairs


```move
module 0x3::property_map {
    public fun update_property_map(map: &mut property_map::PropertyMap, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun update_property_map(
        map: &mut PropertyMap,
        keys: vector<String>,
        values: vector<vector<u8>>,
        types: vector<String>,
    ) {
        let key_len = vector::length(&keys);
        let val_len = vector::length(&values);
        let typ_len = vector::length(&types);
        assert!(key_len == val_len, error::invalid_state(EKEY_COUNT_NOT_MATCH_VALUE_COUNT));
        assert!(key_len == typ_len, error::invalid_state(EKEY_COUNT_NOT_MATCH_TYPE_COUNT));

        let i = 0;
        while (i < key_len) {
            let key = vector::borrow(&keys, i);
            let prop_val = PropertyValue {
                value: *vector::borrow(&values, i),
                type: *vector::borrow(&types, i),
            };
            if (contains_key(map, key)) {
                update_property_value(map, key, prop_val);
            } else {
                add(map, *key, prop_val);
            };
            i = i + 1;
        }
    }
}
```


<a id="0x3_property_map_update_property_value"></a>

## Function `update_property_value`



```move
module 0x3::property_map {
    public fun update_property_value(map: &mut property_map::PropertyMap, key: &string::String, value: property_map::PropertyValue)
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun update_property_value(
        map: &mut PropertyMap,
        key: &String,
        value: PropertyValue
    ) {
        let property_val = simple_map::borrow_mut(&mut map.map, key);
        *property_val = value;
    }
}
```


<a id="0x3_property_map_create_property_value_raw"></a>

## Function `create_property_value_raw`



```move
module 0x3::property_map {
    public fun create_property_value_raw(value: vector<u8>, type: string::String): property_map::PropertyValue
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun create_property_value_raw(
        value: vector<u8>,
        type: String
    ): PropertyValue {
        PropertyValue {
            value,
            type,
        }
    }
}
```


<a id="0x3_property_map_create_property_value"></a>

## Function `create_property_value`

create a property value from generic type data


```move
module 0x3::property_map {
    public fun create_property_value<T: copy>(data: &T): property_map::PropertyValue
}
```


##### Implementation


```move
module 0x3::property_map {
    public fun create_property_value<T: copy>(data: &T): PropertyValue {
        let name = type_name<T>();
        if (
            name == string::utf8(b"bool") ||
                name == string::utf8(b"u8") ||
                name == string::utf8(b"u64") ||
                name == string::utf8(b"u128") ||
                name == string::utf8(b"address") ||
                name == string::utf8(b"0x1::string::String")
        ) {
            create_property_value_raw(bcs::to_bytes<T>(data), name)
        } else {
            create_property_value_raw(bcs::to_bytes<T>(data), string::utf8(b"vector<u8>"))
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x3::property_map {
    pragma verify = true;
    pragma aborts_if_is_strict;
    let MAX_PROPERTY_MAP_SIZE = 1000;
    let MAX_PROPERTY_NAME_LENGTH  = 128;
}
```


<a id="@Specification_1_new"></a>

### Function `new`


```move
module 0x3::property_map {
    public fun new(keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>): property_map::PropertyMap
}
```



```move
module 0x3::property_map {
    pragma aborts_if_is_partial;
    let length = len(keys);
    aborts_if !(length <= MAX_PROPERTY_MAP_SIZE);
    aborts_if !(length == vector::length(values));
    aborts_if !(length == vector::length(types));
}
```


<a id="@Specification_1_new_with_key_and_property_value"></a>

### Function `new_with_key_and_property_value`


```move
module 0x3::property_map {
    public fun new_with_key_and_property_value(keys: vector<string::String>, values: vector<property_map::PropertyValue>): property_map::PropertyMap
}
```



```move
module 0x3::property_map {
    pragma aborts_if_is_partial;
    let length = vector::length(keys);
    aborts_if !(length <= MAX_PROPERTY_MAP_SIZE);
    aborts_if !(length == len(values));
}
```


<a id="@Specification_1_empty"></a>

### Function `empty`


```move
module 0x3::property_map {
    public fun empty(): property_map::PropertyMap
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_contains_key"></a>

### Function `contains_key`


```move
module 0x3::property_map {
    public fun contains_key(map: &property_map::PropertyMap, key: &string::String): bool
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x3::property_map {
    public fun add(map: &mut property_map::PropertyMap, key: string::String, value: property_map::PropertyValue)
}
```



```move
module 0x3::property_map {
    aborts_if !(string::length(key) <= MAX_PROPERTY_NAME_LENGTH);
    aborts_if !(!simple_map::spec_contains_key(map.map, key));
    aborts_if !(simple_map::spec_len(map.map) < MAX_PROPERTY_MAP_SIZE);
}
```


<a id="@Specification_1_length"></a>

### Function `length`


```move
module 0x3::property_map {
    public fun length(map: &property_map::PropertyMap): u64
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x3::property_map {
    public fun borrow(map: &property_map::PropertyMap, key: &string::String): &property_map::PropertyValue
}
```



```move
module 0x3::property_map {
    aborts_if !simple_map::spec_contains_key(map.map, key);
}
```


<a id="@Specification_1_keys"></a>

### Function `keys`


```move
module 0x3::property_map {
    public fun keys(map: &property_map::PropertyMap): vector<string::String>
}
```



```move
module 0x3::property_map {
    pragma verify = false;
}
```


<a id="@Specification_1_types"></a>

### Function `types`


```move
module 0x3::property_map {
    public fun types(map: &property_map::PropertyMap): vector<string::String>
}
```



```move
module 0x3::property_map {
    pragma verify = false;
}
```


<a id="@Specification_1_values"></a>

### Function `values`


```move
module 0x3::property_map {
    public fun values(map: &property_map::PropertyMap): vector<vector<u8>>
}
```



```move
module 0x3::property_map {
    pragma verify = false;
}
```


<a id="@Specification_1_read_string"></a>

### Function `read_string`


```move
module 0x3::property_map {
    public fun read_string(map: &property_map::PropertyMap, key: &string::String): string::String
}
```

Check utf8 for correctness and whether equal
to `prop.type`


```move
module 0x3::property_map {
    pragma aborts_if_is_partial;
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(b"0x1::string::String");
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(b"0x1::string::String");
    aborts_if !aptos_std::from_bcs::deserializable<String>(prop.value);
}
```



<a id="0x3_property_map_spec_utf8"></a>


```move
module 0x3::property_map {
    fun spec_utf8(bytes: vector<u8>): String {
       String{bytes}
    }
}
```


<a id="@Specification_1_read_u8"></a>

### Function `read_u8`


```move
module 0x3::property_map {
    public fun read_u8(map: &property_map::PropertyMap, key: &string::String): u8
}
```



```move
module 0x3::property_map {
    let str = b"u8";
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(str);
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(str);
    aborts_if !aptos_std::from_bcs::deserializable<u8>(prop.value);
}
```


<a id="@Specification_1_read_u64"></a>

### Function `read_u64`


```move
module 0x3::property_map {
    public fun read_u64(map: &property_map::PropertyMap, key: &string::String): u64
}
```



```move
module 0x3::property_map {
    let str = b"u64";
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(str);
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(str);
    aborts_if !aptos_std::from_bcs::deserializable<u64>(prop.value);
}
```


<a id="@Specification_1_read_address"></a>

### Function `read_address`


```move
module 0x3::property_map {
    public fun read_address(map: &property_map::PropertyMap, key: &string::String): address
}
```



```move
module 0x3::property_map {
    let str = b"address";
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(str);
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(str);
    aborts_if !aptos_std::from_bcs::deserializable<address>(prop.value);
}
```


<a id="@Specification_1_read_u128"></a>

### Function `read_u128`


```move
module 0x3::property_map {
    public fun read_u128(map: &property_map::PropertyMap, key: &string::String): u128
}
```



```move
module 0x3::property_map {
    let str = b"u128";
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(str);
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(str);
    aborts_if !aptos_std::from_bcs::deserializable<u128>(prop.value);
}
```


<a id="@Specification_1_read_bool"></a>

### Function `read_bool`


```move
module 0x3::property_map {
    public fun read_bool(map: &property_map::PropertyMap, key: &string::String): bool
}
```



```move
module 0x3::property_map {
    let str = b"bool";
    aborts_if !simple_map::spec_contains_key(map.map, key);
    aborts_if !string::spec_internal_check_utf8(str);
    let prop = simple_map::spec_get(map.map, key);
    aborts_if prop.type != spec_utf8(str);
    aborts_if !aptos_std::from_bcs::deserializable<bool>(prop.value);
}
```


<a id="@Specification_1_borrow_value"></a>

### Function `borrow_value`


```move
module 0x3::property_map {
    public fun borrow_value(property: &property_map::PropertyValue): vector<u8>
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_borrow_type"></a>

### Function `borrow_type`


```move
module 0x3::property_map {
    public fun borrow_type(property: &property_map::PropertyValue): string::String
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x3::property_map {
    public fun remove(map: &mut property_map::PropertyMap, key: &string::String): (string::String, property_map::PropertyValue)
}
```



```move
module 0x3::property_map {
    aborts_if !simple_map::spec_contains_key(map.map, key);
}
```


<a id="@Specification_1_update_property_map"></a>

### Function `update_property_map`


```move
module 0x3::property_map {
    public fun update_property_map(map: &mut property_map::PropertyMap, keys: vector<string::String>, values: vector<vector<u8>>, types: vector<string::String>)
}
```



```move
module 0x3::property_map {
    pragma aborts_if_is_partial;
    let key_len = len(keys);
    let val_len = len(values);
    let typ_len = len(types);
    aborts_if !(key_len == val_len);
    aborts_if !(key_len == typ_len);
}
```


<a id="@Specification_1_update_property_value"></a>

### Function `update_property_value`


```move
module 0x3::property_map {
    public fun update_property_value(map: &mut property_map::PropertyMap, key: &string::String, value: property_map::PropertyValue)
}
```



```move
module 0x3::property_map {
    aborts_if !simple_map::spec_contains_key(map.map, key);
}
```


<a id="@Specification_1_create_property_value_raw"></a>

### Function `create_property_value_raw`


```move
module 0x3::property_map {
    public fun create_property_value_raw(value: vector<u8>, type: string::String): property_map::PropertyValue
}
```



```move
module 0x3::property_map {
    aborts_if false;
}
```


<a id="@Specification_1_create_property_value"></a>

### Function `create_property_value`


```move
module 0x3::property_map {
    public fun create_property_value<T: copy>(data: &T): property_map::PropertyValue
}
```

Abort according to the code


```move
module 0x3::property_map {
    let name = type_name<T>();
    aborts_if !string::spec_internal_check_utf8(b"bool");
    aborts_if name != spec_utf8(b"bool") &&
        !string::spec_internal_check_utf8(b"u8");
    aborts_if name != spec_utf8(b"bool") &&
        name != spec_utf8(b"u8") &&
        !string::spec_internal_check_utf8(b"u64");
    aborts_if name != spec_utf8(b"bool") &&
        name != spec_utf8(b"u8") &&
        name != spec_utf8(b"u64") &&
        !string::spec_internal_check_utf8(b"u128");
    aborts_if name != spec_utf8(b"bool") &&
        name != spec_utf8(b"u8") &&
        name != spec_utf8(b"u64") &&
        name != spec_utf8(b"u128") &&
        !string::spec_internal_check_utf8(b"address");
    aborts_if name != spec_utf8(b"bool") &&
        name != spec_utf8(b"u8") &&
        name != spec_utf8(b"u64") &&
        name != spec_utf8(b"u128") &&
        name != spec_utf8(b"address") &&
        !string::spec_internal_check_utf8(b"0x1::string::String");
    aborts_if name != spec_utf8(b"bool") &&
        name != spec_utf8(b"u8") &&
        name != spec_utf8(b"u64") &&
        name != spec_utf8(b"u128") &&
        name != spec_utf8(b"address") &&
        name != spec_utf8(b"0x1::string::String") &&
        !string::spec_internal_check_utf8(b"vector<u8>");
}
```
