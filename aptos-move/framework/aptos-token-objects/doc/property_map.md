
<a id="0x4_property_map"></a>

# Module `0x4::property_map`

`PropertyMap` provides generic metadata support for `AptosToken`. It is a specialization of
`SimpleMap` that enforces strict typing with minimal storage use by using constant u64 to
represent types and storing values in bcs format.


-  [Resource `PropertyMap`](#0x4_property_map_PropertyMap)
-  [Struct `PropertyValue`](#0x4_property_map_PropertyValue)
-  [Struct `MutatorRef`](#0x4_property_map_MutatorRef)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x4_property_map_init)
-  [Function `extend`](#0x4_property_map_extend)
-  [Function `burn`](#0x4_property_map_burn)
-  [Function `prepare_input`](#0x4_property_map_prepare_input)
-  [Function `to_external_type`](#0x4_property_map_to_external_type)
-  [Function `to_internal_type`](#0x4_property_map_to_internal_type)
-  [Function `type_info_to_internal_type`](#0x4_property_map_type_info_to_internal_type)
-  [Function `validate_type`](#0x4_property_map_validate_type)
-  [Function `generate_mutator_ref`](#0x4_property_map_generate_mutator_ref)
-  [Function `contains_key`](#0x4_property_map_contains_key)
-  [Function `length`](#0x4_property_map_length)
-  [Function `read`](#0x4_property_map_read)
-  [Function `assert_exists`](#0x4_property_map_assert_exists)
-  [Function `read_typed`](#0x4_property_map_read_typed)
-  [Function `read_bool`](#0x4_property_map_read_bool)
-  [Function `read_u8`](#0x4_property_map_read_u8)
-  [Function `read_u16`](#0x4_property_map_read_u16)
-  [Function `read_u32`](#0x4_property_map_read_u32)
-  [Function `read_u64`](#0x4_property_map_read_u64)
-  [Function `read_u128`](#0x4_property_map_read_u128)
-  [Function `read_u256`](#0x4_property_map_read_u256)
-  [Function `read_address`](#0x4_property_map_read_address)
-  [Function `read_bytes`](#0x4_property_map_read_bytes)
-  [Function `read_string`](#0x4_property_map_read_string)
-  [Function `add`](#0x4_property_map_add)
-  [Function `add_typed`](#0x4_property_map_add_typed)
-  [Function `add_internal`](#0x4_property_map_add_internal)
-  [Function `update`](#0x4_property_map_update)
-  [Function `update_typed`](#0x4_property_map_update_typed)
-  [Function `update_internal`](#0x4_property_map_update_internal)
-  [Function `remove`](#0x4_property_map_remove)
-  [Function `assert_end_to_end_input`](#0x4_property_map_assert_end_to_end_input)


```move
module 0x4::property_map {
    use 0x1::bcs;
    use 0x1::error;
    use 0x1::from_bcs;
    use 0x1::object;
    use 0x1::simple_map;
    use 0x1::string;
    use 0x1::type_info;
    use 0x1::vector;
}
```


<a id="0x4_property_map_PropertyMap"></a>

## Resource `PropertyMap`

A Map for typed key to value mapping, the contract using it
should keep track of what keys are what types, and parse them accordingly.


```move
module 0x4::property_map {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct PropertyMap has drop, key
}
```


##### Fields


<dl>
<dt>
`inner: simple_map::SimpleMap<string::String, property_map::PropertyValue>`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_property_map_PropertyValue"></a>

## Struct `PropertyValue`

A typed value for the `PropertyMap` to ensure that typing is always consistent


```move
module 0x4::property_map {
    struct PropertyValue has drop, store
}
```


##### Fields


<dl>
<dt>
`type: u8`
</dt>
<dd>

</dd>
<dt>
`value: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x4_property_map_MutatorRef"></a>

## Struct `MutatorRef`

A mutator ref that allows for mutation of the property map


```move
module 0x4::property_map {
    struct MutatorRef has drop, store
}
```


##### Fields


<dl>
<dt>
`self: address`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x4_property_map_ETYPE_MISMATCH"></a>

Property value does not match expected type


```move
module 0x4::property_map {
    const ETYPE_MISMATCH: u64 = 6;
}
```


<a id="0x4_property_map_ADDRESS"></a>



```move
module 0x4::property_map {
    const ADDRESS: u8 = 7;
}
```


<a id="0x4_property_map_BOOL"></a>



```move
module 0x4::property_map {
    const BOOL: u8 = 0;
}
```


<a id="0x4_property_map_BYTE_VECTOR"></a>



```move
module 0x4::property_map {
    const BYTE_VECTOR: u8 = 8;
}
```


<a id="0x4_property_map_EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP"></a>

The property key already exists


```move
module 0x4::property_map {
    const EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP: u64 = 2;
}
```


<a id="0x4_property_map_EKEY_TYPE_COUNT_MISMATCH"></a>

Property key and type counts do not match


```move
module 0x4::property_map {
    const EKEY_TYPE_COUNT_MISMATCH: u64 = 5;
}
```


<a id="0x4_property_map_EKEY_VALUE_COUNT_MISMATCH"></a>

Property key and value counts do not match


```move
module 0x4::property_map {
    const EKEY_VALUE_COUNT_MISMATCH: u64 = 4;
}
```


<a id="0x4_property_map_EPROPERTY_MAP_DOES_NOT_EXIST"></a>

The property map does not exist


```move
module 0x4::property_map {
    const EPROPERTY_MAP_DOES_NOT_EXIST: u64 = 1;
}
```


<a id="0x4_property_map_EPROPERTY_MAP_KEY_TOO_LONG"></a>

The key of the property is too long


```move
module 0x4::property_map {
    const EPROPERTY_MAP_KEY_TOO_LONG: u64 = 8;
}
```


<a id="0x4_property_map_ETOO_MANY_PROPERTIES"></a>

The number of properties exceeds the maximum


```move
module 0x4::property_map {
    const ETOO_MANY_PROPERTIES: u64 = 3;
}
```


<a id="0x4_property_map_ETYPE_INVALID"></a>

Invalid value type specified


```move
module 0x4::property_map {
    const ETYPE_INVALID: u64 = 7;
}
```


<a id="0x4_property_map_MAX_PROPERTY_MAP_SIZE"></a>

Maximum number of items in a `PropertyMap`


```move
module 0x4::property_map {
    const MAX_PROPERTY_MAP_SIZE: u64 = 1000;
}
```


<a id="0x4_property_map_MAX_PROPERTY_NAME_LENGTH"></a>

Maximum number of characters in a property name


```move
module 0x4::property_map {
    const MAX_PROPERTY_NAME_LENGTH: u64 = 128;
}
```


<a id="0x4_property_map_STRING"></a>



```move
module 0x4::property_map {
    const STRING: u8 = 9;
}
```


<a id="0x4_property_map_U128"></a>



```move
module 0x4::property_map {
    const U128: u8 = 5;
}
```


<a id="0x4_property_map_U16"></a>



```move
module 0x4::property_map {
    const U16: u8 = 2;
}
```


<a id="0x4_property_map_U256"></a>



```move
module 0x4::property_map {
    const U256: u8 = 6;
}
```


<a id="0x4_property_map_U32"></a>



```move
module 0x4::property_map {
    const U32: u8 = 3;
}
```


<a id="0x4_property_map_U64"></a>



```move
module 0x4::property_map {
    const U64: u8 = 4;
}
```


<a id="0x4_property_map_U8"></a>



```move
module 0x4::property_map {
    const U8: u8 = 1;
}
```


<a id="0x4_property_map_init"></a>

## Function `init`



```move
module 0x4::property_map {
    public fun init(ref: &object::ConstructorRef, container: property_map::PropertyMap)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun init(ref: &ConstructorRef, container: PropertyMap) {
        let signer = object::generate_signer(ref);
        move_to(&signer, container);
    }
}
```


<a id="0x4_property_map_extend"></a>

## Function `extend`



```move
module 0x4::property_map {
    public fun extend(ref: &object::ExtendRef, container: property_map::PropertyMap)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun extend(ref: &ExtendRef, container: PropertyMap) {
        let signer = object::generate_signer_for_extending(ref);
        move_to(&signer, container);
    }
}
```


<a id="0x4_property_map_burn"></a>

## Function `burn`

Burns the entire property map


```move
module 0x4::property_map {
    public fun burn(ref: property_map::MutatorRef)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun burn(ref: MutatorRef) acquires PropertyMap {
        move_from<PropertyMap>(ref.self);
    }
}
```


<a id="0x4_property_map_prepare_input"></a>

## Function `prepare_input`

Helper for external entry functions to produce a valid container for property values.


```move
module 0x4::property_map {
    public fun prepare_input(keys: vector<string::String>, types: vector<string::String>, values: vector<vector<u8>>): property_map::PropertyMap
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun prepare_input(
        keys: vector<String>,
        types: vector<String>,
        values: vector<vector<u8>>,
    ): PropertyMap {
        let length = vector::length(&keys);
        assert!(length <= MAX_PROPERTY_MAP_SIZE, error::invalid_argument(ETOO_MANY_PROPERTIES));
        assert!(length == vector::length(&values), error::invalid_argument(EKEY_VALUE_COUNT_MISMATCH));
        assert!(length == vector::length(&types), error::invalid_argument(EKEY_TYPE_COUNT_MISMATCH));

        let container = simple_map::create<String, PropertyValue>();
        while (!vector::is_empty(&keys)) {
            let key = vector::pop_back(&mut keys);
            assert!(
                string::length(&key) <= MAX_PROPERTY_NAME_LENGTH,
                error::invalid_argument(EPROPERTY_MAP_KEY_TOO_LONG),
            );

            let value = vector::pop_back(&mut values);
            let type = vector::pop_back(&mut types);

            let new_type = to_internal_type(type);
            validate_type(new_type, value);

            simple_map::add(&mut container, key, PropertyValue { value, type: new_type });
        };

        PropertyMap { inner: container }
    }
}
```


<a id="0x4_property_map_to_external_type"></a>

## Function `to_external_type`

Maps `String` representation of types from their `u8` representation


```move
module 0x4::property_map {
    fun to_external_type(type: u8): string::String
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun to_external_type(type: u8): String {
        if (type == BOOL) {
            string::utf8(b"bool")
        } else if (type == U8) {
            string::utf8(b"u8")
        } else if (type == U16) {
            string::utf8(b"u16")
        } else if (type == U32) {
            string::utf8(b"u32")
        } else if (type == U64) {
            string::utf8(b"u64")
        } else if (type == U128) {
            string::utf8(b"u128")
        } else if (type == U256) {
            string::utf8(b"u256")
        } else if (type == ADDRESS) {
            string::utf8(b"address")
        } else if (type == BYTE_VECTOR) {
            string::utf8(b"vector<u8>")
        } else if (type == STRING) {
            string::utf8(b"0x1::string::String")
        } else {
            abort (error::invalid_argument(ETYPE_INVALID))
        }
    }
}
```


<a id="0x4_property_map_to_internal_type"></a>

## Function `to_internal_type`

Maps the `String` representation of types to `u8`


```move
module 0x4::property_map {
    fun to_internal_type(type: string::String): u8
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun to_internal_type(type: String): u8 {
        if (type == string::utf8(b"bool")) {
            BOOL
        } else if (type == string::utf8(b"u8")) {
            U8
        } else if (type == string::utf8(b"u16")) {
            U16
        } else if (type == string::utf8(b"u32")) {
            U32
        } else if (type == string::utf8(b"u64")) {
            U64
        } else if (type == string::utf8(b"u128")) {
            U128
        } else if (type == string::utf8(b"u256")) {
            U256
        } else if (type == string::utf8(b"address")) {
            ADDRESS
        } else if (type == string::utf8(b"vector<u8>")) {
            BYTE_VECTOR
        } else if (type == string::utf8(b"0x1::string::String")) {
            STRING
        } else {
            abort (error::invalid_argument(ETYPE_INVALID))
        }
    }
}
```


<a id="0x4_property_map_type_info_to_internal_type"></a>

## Function `type_info_to_internal_type`

Maps Move type to `u8` representation


```move
module 0x4::property_map {
    fun type_info_to_internal_type<T>(): u8
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun type_info_to_internal_type<T>(): u8 {
        let type = type_info::type_name<T>();
        to_internal_type(type)
    }
}
```


<a id="0x4_property_map_validate_type"></a>

## Function `validate_type`

Validates property value type against its expected type


```move
module 0x4::property_map {
    fun validate_type(type: u8, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun validate_type(type: u8, value: vector<u8>) {
        if (type == BOOL) {
            from_bcs::to_bool(value);
        } else if (type == U8) {
            from_bcs::to_u8(value);
        } else if (type == U16) {
            from_bcs::to_u16(value);
        } else if (type == U32) {
            from_bcs::to_u32(value);
        } else if (type == U64) {
            from_bcs::to_u64(value);
        } else if (type == U128) {
            from_bcs::to_u128(value);
        } else if (type == U256) {
            from_bcs::to_u256(value);
        } else if (type == ADDRESS) {
            from_bcs::to_address(value);
        } else if (type == BYTE_VECTOR) {
            // nothing to validate...
        } else if (type == STRING) {
            from_bcs::to_string(value);
        } else {
            abort (error::invalid_argument(ETYPE_MISMATCH))
        };
    }
}
```


<a id="0x4_property_map_generate_mutator_ref"></a>

## Function `generate_mutator_ref`



```move
module 0x4::property_map {
    public fun generate_mutator_ref(ref: &object::ConstructorRef): property_map::MutatorRef
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun generate_mutator_ref(ref: &ConstructorRef): MutatorRef {
        MutatorRef { self: object::address_from_constructor_ref(ref) }
    }
}
```


<a id="0x4_property_map_contains_key"></a>

## Function `contains_key`



```move
module 0x4::property_map {
    public fun contains_key<T: key>(object: &object::Object<T>, key: &string::String): bool
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun contains_key<T: key>(object: &Object<T>, key: &String): bool acquires PropertyMap {
        assert_exists(object::object_address(object));
        let property_map = borrow_global<PropertyMap>(object::object_address(object));
        simple_map::contains_key(&property_map.inner, key)
    }
}
```


<a id="0x4_property_map_length"></a>

## Function `length`



```move
module 0x4::property_map {
    public fun length<T: key>(object: &object::Object<T>): u64
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun length<T: key>(object: &Object<T>): u64 acquires PropertyMap {
        assert_exists(object::object_address(object));
        let property_map = borrow_global<PropertyMap>(object::object_address(object));
        simple_map::length(&property_map.inner)
    }
}
```


<a id="0x4_property_map_read"></a>

## Function `read`

Read the property and get it&apos;s external type in it&apos;s bcs encoded format

The preferred method is to use `read_<type>` where the type is already known.


```move
module 0x4::property_map {
    public fun read<T: key>(object: &object::Object<T>, key: &string::String): (string::String, vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read<T: key>(object: &Object<T>, key: &String): (String, vector<u8>) acquires PropertyMap {
        assert_exists(object::object_address(object));
        let property_map = borrow_global<PropertyMap>(object::object_address(object));
        let property_value = simple_map::borrow(&property_map.inner, key);
        let new_type = to_external_type(property_value.type);
        (new_type, property_value.value)
    }
}
```


<a id="0x4_property_map_assert_exists"></a>

## Function `assert_exists`



```move
module 0x4::property_map {
    fun assert_exists(object: address)
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun assert_exists(object: address) {
        assert!(
            exists<PropertyMap>(object),
            error::not_found(EPROPERTY_MAP_DOES_NOT_EXIST),
        );
    }
}
```


<a id="0x4_property_map_read_typed"></a>

## Function `read_typed`

Read a type and verify that the type is correct


```move
module 0x4::property_map {
    fun read_typed<T: key, V>(object: &object::Object<T>, key: &string::String): vector<u8>
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun read_typed<T: key, V>(object: &Object<T>, key: &String): vector<u8> acquires PropertyMap {
        let (type, value) = read(object, key);
        assert!(
            type == type_info::type_name<V>(),
            error::invalid_argument(ETYPE_MISMATCH),
        );
        value
    }
}
```


<a id="0x4_property_map_read_bool"></a>

## Function `read_bool`



```move
module 0x4::property_map {
    public fun read_bool<T: key>(object: &object::Object<T>, key: &string::String): bool
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_bool<T: key>(object: &Object<T>, key: &String): bool acquires PropertyMap {
        let value = read_typed<T, bool>(object, key);
        from_bcs::to_bool(value)
    }
}
```


<a id="0x4_property_map_read_u8"></a>

## Function `read_u8`



```move
module 0x4::property_map {
    public fun read_u8<T: key>(object: &object::Object<T>, key: &string::String): u8
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u8<T: key>(object: &Object<T>, key: &String): u8 acquires PropertyMap {
        let value = read_typed<T, u8>(object, key);
        from_bcs::to_u8(value)
    }
}
```


<a id="0x4_property_map_read_u16"></a>

## Function `read_u16`



```move
module 0x4::property_map {
    public fun read_u16<T: key>(object: &object::Object<T>, key: &string::String): u16
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u16<T: key>(object: &Object<T>, key: &String): u16 acquires PropertyMap {
        let value = read_typed<T, u16>(object, key);
        from_bcs::to_u16(value)
    }
}
```


<a id="0x4_property_map_read_u32"></a>

## Function `read_u32`



```move
module 0x4::property_map {
    public fun read_u32<T: key>(object: &object::Object<T>, key: &string::String): u32
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u32<T: key>(object: &Object<T>, key: &String): u32 acquires PropertyMap {
        let value = read_typed<T, u32>(object, key);
        from_bcs::to_u32(value)
    }
}
```


<a id="0x4_property_map_read_u64"></a>

## Function `read_u64`



```move
module 0x4::property_map {
    public fun read_u64<T: key>(object: &object::Object<T>, key: &string::String): u64
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u64<T: key>(object: &Object<T>, key: &String): u64 acquires PropertyMap {
        let value = read_typed<T, u64>(object, key);
        from_bcs::to_u64(value)
    }
}
```


<a id="0x4_property_map_read_u128"></a>

## Function `read_u128`



```move
module 0x4::property_map {
    public fun read_u128<T: key>(object: &object::Object<T>, key: &string::String): u128
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u128<T: key>(object: &Object<T>, key: &String): u128 acquires PropertyMap {
        let value = read_typed<T, u128>(object, key);
        from_bcs::to_u128(value)
    }
}
```


<a id="0x4_property_map_read_u256"></a>

## Function `read_u256`



```move
module 0x4::property_map {
    public fun read_u256<T: key>(object: &object::Object<T>, key: &string::String): u256
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_u256<T: key>(object: &Object<T>, key: &String): u256 acquires PropertyMap {
        let value = read_typed<T, u256>(object, key);
        from_bcs::to_u256(value)
    }
}
```


<a id="0x4_property_map_read_address"></a>

## Function `read_address`



```move
module 0x4::property_map {
    public fun read_address<T: key>(object: &object::Object<T>, key: &string::String): address
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_address<T: key>(object: &Object<T>, key: &String): address acquires PropertyMap {
        let value = read_typed<T, address>(object, key);
        from_bcs::to_address(value)
    }
}
```


<a id="0x4_property_map_read_bytes"></a>

## Function `read_bytes`



```move
module 0x4::property_map {
    public fun read_bytes<T: key>(object: &object::Object<T>, key: &string::String): vector<u8>
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_bytes<T: key>(object: &Object<T>, key: &String): vector<u8> acquires PropertyMap {
        let value = read_typed<T, vector<u8>>(object, key);
        from_bcs::to_bytes(value)
    }
}
```


<a id="0x4_property_map_read_string"></a>

## Function `read_string`



```move
module 0x4::property_map {
    public fun read_string<T: key>(object: &object::Object<T>, key: &string::String): string::String
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun read_string<T: key>(object: &Object<T>, key: &String): String acquires PropertyMap {
        let value = read_typed<T, String>(object, key);
        from_bcs::to_string(value)
    }
}
```


<a id="0x4_property_map_add"></a>

## Function `add`

Add a property, already bcs encoded as a `vector<u8>`


```move
module 0x4::property_map {
    public fun add(ref: &property_map::MutatorRef, key: string::String, type: string::String, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun add(ref: &MutatorRef, key: String, type: String, value: vector<u8>) acquires PropertyMap {
        let new_type = to_internal_type(type);
        validate_type(new_type, value);
        add_internal(ref, key, new_type, value);
    }
}
```


<a id="0x4_property_map_add_typed"></a>

## Function `add_typed`

Add a property that isn&apos;t already encoded as a `vector<u8>`


```move
module 0x4::property_map {
    public fun add_typed<T: drop>(ref: &property_map::MutatorRef, key: string::String, value: T)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun add_typed<T: drop>(ref: &MutatorRef, key: String, value: T) acquires PropertyMap {
        let type = type_info_to_internal_type<T>();
        add_internal(ref, key, type, bcs::to_bytes(&value));
    }
}
```


<a id="0x4_property_map_add_internal"></a>

## Function `add_internal`



```move
module 0x4::property_map {
    fun add_internal(ref: &property_map::MutatorRef, key: string::String, type: u8, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun add_internal(ref: &MutatorRef, key: String, type: u8, value: vector<u8>) acquires PropertyMap {
        assert_exists(ref.self);
        let property_map = borrow_global_mut<PropertyMap>(ref.self);
        simple_map::add(&mut property_map.inner, key, PropertyValue { type, value });
    }
}
```


<a id="0x4_property_map_update"></a>

## Function `update`

Updates a property in place already bcs encoded


```move
module 0x4::property_map {
    public fun update(ref: &property_map::MutatorRef, key: &string::String, type: string::String, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun update(ref: &MutatorRef, key: &String, type: String, value: vector<u8>) acquires PropertyMap {
        let new_type = to_internal_type(type);
        validate_type(new_type, value);
        update_internal(ref, key, new_type, value);
    }
}
```


<a id="0x4_property_map_update_typed"></a>

## Function `update_typed`

Updates a property in place that is not already bcs encoded


```move
module 0x4::property_map {
    public fun update_typed<T: drop>(ref: &property_map::MutatorRef, key: &string::String, value: T)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun update_typed<T: drop>(ref: &MutatorRef, key: &String, value: T) acquires PropertyMap {
        let type = type_info_to_internal_type<T>();
        update_internal(ref, key, type, bcs::to_bytes(&value));
    }
}
```


<a id="0x4_property_map_update_internal"></a>

## Function `update_internal`



```move
module 0x4::property_map {
    fun update_internal(ref: &property_map::MutatorRef, key: &string::String, type: u8, value: vector<u8>)
}
```


##### Implementation


```move
module 0x4::property_map {
    inline fun update_internal(ref: &MutatorRef, key: &String, type: u8, value: vector<u8>) acquires PropertyMap {
        assert_exists(ref.self);
        let property_map = borrow_global_mut<PropertyMap>(ref.self);
        let old_value = simple_map::borrow_mut(&mut property_map.inner, key);
        *old_value = PropertyValue { type, value };
    }
}
```


<a id="0x4_property_map_remove"></a>

## Function `remove`

Removes a property from the map, ensuring that it does in fact exist


```move
module 0x4::property_map {
    public fun remove(ref: &property_map::MutatorRef, key: &string::String)
}
```


##### Implementation


```move
module 0x4::property_map {
    public fun remove(ref: &MutatorRef, key: &String) acquires PropertyMap {
        assert_exists(ref.self);
        let property_map = borrow_global_mut<PropertyMap>(ref.self);
        simple_map::remove(&mut property_map.inner, key);
    }
}
```


<a id="0x4_property_map_assert_end_to_end_input"></a>

## Function `assert_end_to_end_input`



```move
module 0x4::property_map {
    fun assert_end_to_end_input(object: object::Object<object::ObjectCore>)
}
```


##### Implementation


```move
module 0x4::property_map {
    fun assert_end_to_end_input(object: Object<ObjectCore>) acquires PropertyMap {
        assert!(read_bool(&object, &string::utf8(b"bool")), 0);
        assert!(read_u8(&object, &string::utf8(b"u8")) == 0x12, 1);
        assert!(read_u16(&object, &string::utf8(b"u16")) == 0x1234, 2);
        assert!(read_u32(&object, &string::utf8(b"u32")) == 0x12345678, 3);
        assert!(read_u64(&object, &string::utf8(b"u64")) == 0x1234567812345678, 4);
        assert!(read_u128(&object, &string::utf8(b"u128")) == 0x12345678123456781234567812345678, 5);
        assert!(
            read_u256(
                &object,
                &string::utf8(b"u256")
            ) == 0x1234567812345678123456781234567812345678123456781234567812345678,
            6
        );
        assert!(read_bytes(&object, &string::utf8(b"vector<u8>")) == vector[0x01], 7);
        assert!(read_string(&object, &string::utf8(b"0x1::string::String")) == string::utf8(b"a"), 8);

        assert!(length(&object) == 9, 9);
    }
}
```
