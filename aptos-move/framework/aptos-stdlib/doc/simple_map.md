
<a id="0x1_simple_map"></a>

# Module `0x1::simple_map`

This module provides a solution for unsorted maps, that is it has the properties that
1) Keys point to Values
2) Each Key must be unique
3) A Key can be found within O(N) time
4) The keys are unsorted.
5) Adds and removals take O(N) time


-  [Struct `SimpleMap`](#0x1_simple_map_SimpleMap)
-  [Struct `Element`](#0x1_simple_map_Element)
-  [Constants](#@Constants_0)
-  [Function `length`](#0x1_simple_map_length)
-  [Function `new`](#0x1_simple_map_new)
-  [Function `new_from`](#0x1_simple_map_new_from)
-  [Function `create`](#0x1_simple_map_create)
-  [Function `borrow`](#0x1_simple_map_borrow)
-  [Function `borrow_mut`](#0x1_simple_map_borrow_mut)
-  [Function `contains_key`](#0x1_simple_map_contains_key)
-  [Function `destroy_empty`](#0x1_simple_map_destroy_empty)
-  [Function `add`](#0x1_simple_map_add)
-  [Function `add_all`](#0x1_simple_map_add_all)
-  [Function `upsert`](#0x1_simple_map_upsert)
-  [Function `keys`](#0x1_simple_map_keys)
-  [Function `values`](#0x1_simple_map_values)
-  [Function `to_vec_pair`](#0x1_simple_map_to_vec_pair)
-  [Function `destroy`](#0x1_simple_map_destroy)
-  [Function `remove`](#0x1_simple_map_remove)
-  [Function `find`](#0x1_simple_map_find)
-  [Specification](#@Specification_1)
    -  [Struct `SimpleMap`](#@Specification_1_SimpleMap)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `new_from`](#@Specification_1_new_from)
    -  [Function `create`](#@Specification_1_create)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `contains_key`](#@Specification_1_contains_key)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `add_all`](#@Specification_1_add_all)
    -  [Function `upsert`](#@Specification_1_upsert)
    -  [Function `keys`](#@Specification_1_keys)
    -  [Function `values`](#@Specification_1_values)
    -  [Function `to_vec_pair`](#@Specification_1_to_vec_pair)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `find`](#@Specification_1_find)


```move
module 0x1::simple_map {
    use 0x1::error;
    use 0x1::option;
    use 0x1::vector;
}
```


<a id="0x1_simple_map_SimpleMap"></a>

## Struct `SimpleMap`



```move
module 0x1::simple_map {
    struct SimpleMap<Key, Value> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`data: vector<simple_map::Element<Key, Value>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_simple_map_Element"></a>

## Struct `Element`



```move
module 0x1::simple_map {
    struct Element<Key, Value> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`key: Key`
</dt>
<dd>

</dd>
<dt>
`value: Value`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_simple_map_EKEY_ALREADY_EXISTS"></a>

Map key already exists


```move
module 0x1::simple_map {
    const EKEY_ALREADY_EXISTS: u64 = 1;
}
```


<a id="0x1_simple_map_EKEY_NOT_FOUND"></a>

Map key is not found


```move
module 0x1::simple_map {
    const EKEY_NOT_FOUND: u64 = 2;
}
```


<a id="0x1_simple_map_length"></a>

## Function `length`



```move
module 0x1::simple_map {
    public fun length<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>): u64
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun length<Key: store, Value: store>(map: &SimpleMap<Key, Value>): u64 {
        vector::length(&map.data)
    }
}
```


<a id="0x1_simple_map_new"></a>

## Function `new`

Create an empty SimpleMap.


```move
module 0x1::simple_map {
    public fun new<Key: store, Value: store>(): simple_map::SimpleMap<Key, Value>
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun new<Key: store, Value: store>(): SimpleMap<Key, Value> {
        SimpleMap {
            data: vector::empty(),
        }
    }
}
```


<a id="0x1_simple_map_new_from"></a>

## Function `new_from`

Create a SimpleMap from a vector of keys and values. The keys must be unique.


```move
module 0x1::simple_map {
    public fun new_from<Key: store, Value: store>(keys: vector<Key>, values: vector<Value>): simple_map::SimpleMap<Key, Value>
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun new_from<Key: store, Value: store>(
        keys: vector<Key>,
        values: vector<Value>,
    ): SimpleMap<Key, Value> {
        let map = new();
        add_all(&mut map, keys, values);
        map
    }
}
```


<a id="0x1_simple_map_create"></a>

## Function `create`

Create an empty SimpleMap.
This function is deprecated, use `new` instead.


```move
module 0x1::simple_map {
    #[deprecated]
    public fun create<Key: store, Value: store>(): simple_map::SimpleMap<Key, Value>
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun create<Key: store, Value: store>(): SimpleMap<Key, Value> {
        new()
    }
}
```


<a id="0x1_simple_map_borrow"></a>

## Function `borrow`



```move
module 0x1::simple_map {
    public fun borrow<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): &Value
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun borrow<Key: store, Value: store>(
        map: &SimpleMap<Key, Value>,
        key: &Key,
    ): &Value {
        let maybe_idx = find(map, key);
        assert!(option::is_some(&maybe_idx), error::invalid_argument(EKEY_NOT_FOUND));
        let idx = option::extract(&mut maybe_idx);
        &vector::borrow(&map.data, idx).value
    }
}
```


<a id="0x1_simple_map_borrow_mut"></a>

## Function `borrow_mut`



```move
module 0x1::simple_map {
    public fun borrow_mut<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: &Key): &mut Value
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun borrow_mut<Key: store, Value: store>(
        map: &mut SimpleMap<Key, Value>,
        key: &Key,
    ): &mut Value {
        let maybe_idx = find(map, key);
        assert!(option::is_some(&maybe_idx), error::invalid_argument(EKEY_NOT_FOUND));
        let idx = option::extract(&mut maybe_idx);
        &mut vector::borrow_mut(&mut map.data, idx).value
    }
}
```


<a id="0x1_simple_map_contains_key"></a>

## Function `contains_key`



```move
module 0x1::simple_map {
    public fun contains_key<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): bool
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun contains_key<Key: store, Value: store>(
        map: &SimpleMap<Key, Value>,
        key: &Key,
    ): bool {
        let maybe_idx = find(map, key);
        option::is_some(&maybe_idx)
    }
}
```


<a id="0x1_simple_map_destroy_empty"></a>

## Function `destroy_empty`



```move
module 0x1::simple_map {
    public fun destroy_empty<Key: store, Value: store>(map: simple_map::SimpleMap<Key, Value>)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun destroy_empty<Key: store, Value: store>(map: SimpleMap<Key, Value>) {
        let SimpleMap { data } = map;
        vector::destroy_empty(data);
    }
}
```


<a id="0x1_simple_map_add"></a>

## Function `add`

Add a key/value pair to the map. The key must not already exist.


```move
module 0x1::simple_map {
    public fun add<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: Key, value: Value)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun add<Key: store, Value: store>(
        map: &mut SimpleMap<Key, Value>,
        key: Key,
        value: Value,
    ) {
        let maybe_idx = find(map, &key);
        assert!(option::is_none(&maybe_idx), error::invalid_argument(EKEY_ALREADY_EXISTS));

        vector::push_back(&mut map.data, Element { key, value });
    }
}
```


<a id="0x1_simple_map_add_all"></a>

## Function `add_all`

Add multiple key/value pairs to the map. The keys must not already exist.


```move
module 0x1::simple_map {
    public fun add_all<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, keys: vector<Key>, values: vector<Value>)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun add_all<Key: store, Value: store>(
        map: &mut SimpleMap<Key, Value>,
        keys: vector<Key>,
        values: vector<Value>,
    ) {
        vector::zip(keys, values, |key, value| {
            add(map, key, value);
        });
    }
}
```


<a id="0x1_simple_map_upsert"></a>

## Function `upsert`

Insert key/value pair or update an existing key to a new value


```move
module 0x1::simple_map {
    public fun upsert<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: Key, value: Value): (option::Option<Key>, option::Option<Value>)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun upsert<Key: store, Value: store>(
        map: &mut SimpleMap<Key, Value>,
        key: Key,
        value: Value
    ): (std::option::Option<Key>, std::option::Option<Value>) {
        let data = &mut map.data;
        let len = vector::length(data);
        let i = 0;
        while (i < len) {
            let element = vector::borrow(data, i);
            if (&element.key == &key) {
                vector::push_back(data, Element { key, value });
                vector::swap(data, i, len);
                let Element { key, value } = vector::pop_back(data);
                return (std::option::some(key), std::option::some(value))
            };
            i = i + 1;
        };
        vector::push_back(&mut map.data, Element { key, value });
        (std::option::none(), std::option::none())
    }
}
```


<a id="0x1_simple_map_keys"></a>

## Function `keys`

Return all keys in the map. This requires keys to be copyable.


```move
module 0x1::simple_map {
    public fun keys<Key: copy, Value>(map: &simple_map::SimpleMap<Key, Value>): vector<Key>
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun keys<Key: copy, Value>(map: &SimpleMap<Key, Value>): vector<Key> {
        vector::map_ref(&map.data, |e| {
            let e: &Element<Key, Value> = e;
            e.key
        })
    }
}
```


<a id="0x1_simple_map_values"></a>

## Function `values`

Return all values in the map. This requires values to be copyable.


```move
module 0x1::simple_map {
    public fun values<Key, Value: copy>(map: &simple_map::SimpleMap<Key, Value>): vector<Value>
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun values<Key, Value: copy>(map: &SimpleMap<Key, Value>): vector<Value> {
        vector::map_ref(&map.data, |e| {
            let e: &Element<Key, Value> = e;
            e.value
        })
    }
}
```


<a id="0x1_simple_map_to_vec_pair"></a>

## Function `to_vec_pair`

Transform the map into two vectors with the keys and values respectively
Primarily used to destroy a map


```move
module 0x1::simple_map {
    public fun to_vec_pair<Key: store, Value: store>(map: simple_map::SimpleMap<Key, Value>): (vector<Key>, vector<Value>)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun to_vec_pair<Key: store, Value: store>(
        map: SimpleMap<Key, Value>): (vector<Key>, vector<Value>) {
        let keys: vector<Key> = vector::empty();
        let values: vector<Value> = vector::empty();
        let SimpleMap { data } = map;
        vector::for_each(data, |e| {
            let Element { key, value } = e;
            vector::push_back(&mut keys, key);
            vector::push_back(&mut values, value);
        });
        (keys, values)
    }
}
```


<a id="0x1_simple_map_destroy"></a>

## Function `destroy`

For maps that cannot be dropped this is a utility to destroy them
using lambdas to destroy the individual keys and values.


```move
module 0x1::simple_map {
    public fun destroy<Key: store, Value: store>(map: simple_map::SimpleMap<Key, Value>, dk: |Key|, dv: |Value|)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public inline fun destroy<Key: store, Value: store>(
        map: SimpleMap<Key, Value>,
        dk: |Key|,
        dv: |Value|
    ) {
        let (keys, values) = to_vec_pair(map);
        vector::destroy(keys, |_k| dk(_k));
        vector::destroy(values, |_v| dv(_v));
    }
}
```


<a id="0x1_simple_map_remove"></a>

## Function `remove`

Remove a key/value pair from the map. The key must exist.


```move
module 0x1::simple_map {
    public fun remove<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: &Key): (Key, Value)
}
```


##### Implementation


```move
module 0x1::simple_map {
    public fun remove<Key: store, Value: store>(
        map: &mut SimpleMap<Key, Value>,
        key: &Key,
    ): (Key, Value) {
        let maybe_idx = find(map, key);
        assert!(option::is_some(&maybe_idx), error::invalid_argument(EKEY_NOT_FOUND));
        let placement = option::extract(&mut maybe_idx);
        let Element { key, value } = vector::swap_remove(&mut map.data, placement);
        (key, value)
    }
}
```


<a id="0x1_simple_map_find"></a>

## Function `find`



```move
module 0x1::simple_map {
    fun find<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): option::Option<u64>
}
```


##### Implementation


```move
module 0x1::simple_map {
    fun find<Key: store, Value: store>(
        map: &SimpleMap<Key, Value>,
        key: &Key,
    ): option::Option<u64> {
        let leng = vector::length(&map.data);
        let i = 0;
        while (i < leng) {
            let element = vector::borrow(&map.data, i);
            if (&element.key == key) {
                return option::some(i)
            };
            i = i + 1;
        };
        option::none<u64>()
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_SimpleMap"></a>

### Struct `SimpleMap`


```move
module 0x1::simple_map {
    struct SimpleMap<Key, Value> has copy, drop, store
}
```


<dl>
<dt>
`data: vector<simple_map::Element<Key, Value>>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::simple_map {
    pragma intrinsic = map,
        map_new = create,
        map_len = length,
        map_destroy_empty = destroy_empty,
        map_has_key = contains_key,
        map_add_no_override = add,
        map_del_return_key = remove,
        map_borrow = borrow,
        map_borrow_mut = borrow_mut,
        map_spec_get = spec_get,
        map_spec_set = spec_set,
        map_spec_del = spec_remove,
        map_spec_len = spec_len,
        map_spec_has_key = spec_contains_key;
}
```


<a id="@Specification_1_length"></a>

### Function `length`


```move
module 0x1::simple_map {
    public fun length<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>): u64
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_new"></a>

### Function `new`


```move
module 0x1::simple_map {
    public fun new<Key: store, Value: store>(): simple_map::SimpleMap<Key, Value>
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] spec_len(result) == 0;
    ensures [abstract] forall k: Key: !spec_contains_key(result, k);
}
```


<a id="@Specification_1_new_from"></a>

### Function `new_from`


```move
module 0x1::simple_map {
    public fun new_from<Key: store, Value: store>(keys: vector<Key>, values: vector<Value>): simple_map::SimpleMap<Key, Value>
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] spec_len(result) == len(keys);
    ensures [abstract] forall k: Key: spec_contains_key(result, k) <==> vector::spec_contains(keys, k);
    ensures [abstract] forall i in 0..len(keys):
        spec_get(result, vector::borrow(keys, i)) == vector::borrow(values, i);
}
```


<a id="@Specification_1_create"></a>

### Function `create`


```move
module 0x1::simple_map {
    #[deprecated]
    public fun create<Key: store, Value: store>(): simple_map::SimpleMap<Key, Value>
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x1::simple_map {
    public fun borrow<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): &Value
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


```move
module 0x1::simple_map {
    public fun borrow_mut<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: &Key): &mut Value
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_contains_key"></a>

### Function `contains_key`


```move
module 0x1::simple_map {
    public fun contains_key<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): bool
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


```move
module 0x1::simple_map {
    public fun destroy_empty<Key: store, Value: store>(map: simple_map::SimpleMap<Key, Value>)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::simple_map {
    public fun add<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: Key, value: Value)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_add_all"></a>

### Function `add_all`


```move
module 0x1::simple_map {
    public fun add_all<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, keys: vector<Key>, values: vector<Value>)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_upsert"></a>

### Function `upsert`


```move
module 0x1::simple_map {
    public fun upsert<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: Key, value: Value): (option::Option<Key>, option::Option<Value>)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] !spec_contains_key(old(map), key) ==> option::is_none(result_1);
    ensures [abstract] !spec_contains_key(old(map), key) ==> option::is_none(result_2);
    ensures [abstract] spec_contains_key(map, key);
    ensures [abstract] spec_get(map, key) == value;
    ensures [abstract] spec_contains_key(old(map), key) ==> ((option::is_some(result_1)) && (option::spec_borrow(result_1) == key));
    ensures [abstract] spec_contains_key(old(map), key) ==> ((option::is_some(result_2)) && (option::spec_borrow(result_2) == spec_get(old(map), key)));
}
```



<a id="0x1_simple_map_spec_len"></a>


```move
module 0x1::simple_map {
    native fun spec_len<K, V>(t: SimpleMap<K, V>): num;
}
```



<a id="0x1_simple_map_spec_contains_key"></a>


```move
module 0x1::simple_map {
    native fun spec_contains_key<K, V>(t: SimpleMap<K, V>, k: K): bool;
}
```



<a id="0x1_simple_map_spec_set"></a>


```move
module 0x1::simple_map {
    native fun spec_set<K, V>(t: SimpleMap<K, V>, k: K, v: V): SimpleMap<K, V>;
}
```



<a id="0x1_simple_map_spec_remove"></a>


```move
module 0x1::simple_map {
    native fun spec_remove<K, V>(t: SimpleMap<K, V>, k: K): SimpleMap<K, V>;
}
```



<a id="0x1_simple_map_spec_get"></a>


```move
module 0x1::simple_map {
    native fun spec_get<K, V>(t: SimpleMap<K, V>, k: K): V;
}
```


<a id="@Specification_1_keys"></a>

### Function `keys`


```move
module 0x1::simple_map {
    public fun keys<Key: copy, Value>(map: &simple_map::SimpleMap<Key, Value>): vector<Key>
}
```



```move
module 0x1::simple_map {
    pragma verify=false;
}
```


<a id="@Specification_1_values"></a>

### Function `values`


```move
module 0x1::simple_map {
    public fun values<Key, Value: copy>(map: &simple_map::SimpleMap<Key, Value>): vector<Value>
}
```



```move
module 0x1::simple_map {
    pragma verify=false;
}
```


<a id="@Specification_1_to_vec_pair"></a>

### Function `to_vec_pair`


```move
module 0x1::simple_map {
    public fun to_vec_pair<Key: store, Value: store>(map: simple_map::SimpleMap<Key, Value>): (vector<Key>, vector<Value>)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
    pragma opaque;
    ensures [abstract]
        forall k: Key: vector::spec_contains(result_1, k) <==>
            spec_contains_key(map, k);
    ensures [abstract] forall i in 0..len(result_1):
        spec_get(map, vector::borrow(result_1, i)) == vector::borrow(result_2, i);
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x1::simple_map {
    public fun remove<Key: store, Value: store>(map: &mut simple_map::SimpleMap<Key, Value>, key: &Key): (Key, Value)
}
```



```move
module 0x1::simple_map {
    pragma intrinsic;
}
```


<a id="@Specification_1_find"></a>

### Function `find`


```move
module 0x1::simple_map {
    fun find<Key: store, Value: store>(map: &simple_map::SimpleMap<Key, Value>, key: &Key): option::Option<u64>
}
```



```move
module 0x1::simple_map {
    pragma verify=false;
}
```
