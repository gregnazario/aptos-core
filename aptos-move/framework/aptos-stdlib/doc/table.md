
<a id="0x1_table"></a>

# Module `0x1::table`

Type of large&#45;scale storage tables.
source: https://github.com/move&#45;language/move/blob/1b6b7513dcc1a5c866f178ca5c1e74beb2ce181e/language/extensions/move&#45;table&#45;extension/sources/Table.move#L1

It implements the Table type which supports individual table items to be represented by
separate global state items. The number of items and a unique handle are tracked on the table
struct itself, while the operations are implemented as native functions. No traversal is provided.


-  [Struct `Table`](#0x1_table_Table)
-  [Resource `Box`](#0x1_table_Box)
-  [Function `new`](#0x1_table_new)
-  [Function `add`](#0x1_table_add)
-  [Function `borrow`](#0x1_table_borrow)
-  [Function `borrow_with_default`](#0x1_table_borrow_with_default)
-  [Function `borrow_mut`](#0x1_table_borrow_mut)
-  [Function `borrow_mut_with_default`](#0x1_table_borrow_mut_with_default)
-  [Function `upsert`](#0x1_table_upsert)
-  [Function `remove`](#0x1_table_remove)
-  [Function `contains`](#0x1_table_contains)
-  [Function `destroy`](#0x1_table_destroy)
-  [Function `new_table_handle`](#0x1_table_new_table_handle)
-  [Function `add_box`](#0x1_table_add_box)
-  [Function `borrow_box`](#0x1_table_borrow_box)
-  [Function `borrow_box_mut`](#0x1_table_borrow_box_mut)
-  [Function `contains_box`](#0x1_table_contains_box)
-  [Function `remove_box`](#0x1_table_remove_box)
-  [Function `destroy_empty_box`](#0x1_table_destroy_empty_box)
-  [Function `drop_unchecked_box`](#0x1_table_drop_unchecked_box)
-  [Specification](#@Specification_0)
    -  [Struct `Table`](#@Specification_0_Table)
    -  [Function `new`](#@Specification_0_new)
    -  [Function `add`](#@Specification_0_add)
    -  [Function `borrow`](#@Specification_0_borrow)
    -  [Function `borrow_mut`](#@Specification_0_borrow_mut)
    -  [Function `borrow_mut_with_default`](#@Specification_0_borrow_mut_with_default)
    -  [Function `upsert`](#@Specification_0_upsert)
    -  [Function `remove`](#@Specification_0_remove)
    -  [Function `contains`](#@Specification_0_contains)
    -  [Function `destroy`](#@Specification_0_destroy)


```move
module 0x1::table {
}
```


<a id="0x1_table_Table"></a>

## Struct `Table`

Type of tables


```move
module 0x1::table {
    struct Table<K: copy, drop, V> has store
}
```


##### Fields


<dl>
<dt>
`handle: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_table_Box"></a>

## Resource `Box`

Wrapper for values. Required for making values appear as resources in the implementation.


```move
module 0x1::table {
    struct Box<V> has drop, store, key
}
```


##### Fields


<dl>
<dt>
`val: V`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_table_new"></a>

## Function `new`

Create a new Table.


```move
module 0x1::table {
    public fun new<K: copy, drop, V: store>(): table::Table<K, V>
}
```


##### Implementation


```move
module 0x1::table {
    public fun new<K: copy + drop, V: store>(): Table<K, V> {
        Table {
            handle: new_table_handle<K, V>(),
        }
    }
}
```


<a id="0x1_table_add"></a>

## Function `add`

Add a new entry to the table. Aborts if an entry for this
key already exists. The entry itself is not stored in the
table, and cannot be discovered from it.


```move
module 0x1::table {
    public fun add<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K, val: V)
}
```


##### Implementation


```move
module 0x1::table {
    public fun add<K: copy + drop, V>(table: &mut Table<K, V>, key: K, val: V) {
        add_box<K, V, Box<V>>(table, key, Box { val })
    }
}
```


<a id="0x1_table_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table {
    public fun borrow<K: copy, drop, V>(table: &table::Table<K, V>, key: K): &V
}
```


##### Implementation


```move
module 0x1::table {
    public fun borrow<K: copy + drop, V>(table: &Table<K, V>, key: K): &V {
        &borrow_box<K, V, Box<V>>(table, key).val
    }
}
```


<a id="0x1_table_borrow_with_default"></a>

## Function `borrow_with_default`

Acquire an immutable reference to the value which `key` maps to.
Returns specified default value if there is no entry for `key`.


```move
module 0x1::table {
    public fun borrow_with_default<K: copy, drop, V>(table: &table::Table<K, V>, key: K, default: &V): &V
}
```


##### Implementation


```move
module 0x1::table {
    public fun borrow_with_default<K: copy + drop, V>(table: &Table<K, V>, key: K, default: &V): &V {
        if (!contains(table, copy key)) {
            default
        } else {
            borrow(table, copy key)
        }
    }
}
```


<a id="0x1_table_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table {
    public fun borrow_mut<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K): &mut V
}
```


##### Implementation


```move
module 0x1::table {
    public fun borrow_mut<K: copy + drop, V>(table: &mut Table<K, V>, key: K): &mut V {
        &mut borrow_box_mut<K, V, Box<V>>(table, key).val
    }
}
```


<a id="0x1_table_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which `key` maps to.
Insert the pair (`key`, `default`) first if there is no entry for `key`.


```move
module 0x1::table {
    public fun borrow_mut_with_default<K: copy, drop, V: drop>(table: &mut table::Table<K, V>, key: K, default: V): &mut V
}
```


##### Implementation


```move
module 0x1::table {
    public fun borrow_mut_with_default<K: copy + drop, V: drop>(table: &mut Table<K, V>, key: K, default: V): &mut V {
        if (!contains(table, copy key)) {
            add(table, copy key, default)
        };
        borrow_mut(table, key)
    }
}
```


<a id="0x1_table_upsert"></a>

## Function `upsert`

Insert the pair (`key`, `value`) if there is no entry for `key`.
update the value of the entry for `key` to `value` otherwise


```move
module 0x1::table {
    public fun upsert<K: copy, drop, V: drop>(table: &mut table::Table<K, V>, key: K, value: V)
}
```


##### Implementation


```move
module 0x1::table {
    public fun upsert<K: copy + drop, V: drop>(table: &mut Table<K, V>, key: K, value: V) {
        if (!contains(table, copy key)) {
            add(table, copy key, value)
        } else {
            let ref = borrow_mut(table, key);
            *ref = value;
        };
    }
}
```


<a id="0x1_table_remove"></a>

## Function `remove`

Remove from `table` and return the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table {
    public fun remove<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K): V
}
```


##### Implementation


```move
module 0x1::table {
    public fun remove<K: copy + drop, V>(table: &mut Table<K, V>, key: K): V {
        let Box { val } = remove_box<K, V, Box<V>>(table, key);
        val
    }
}
```


<a id="0x1_table_contains"></a>

## Function `contains`

Returns true iff `table` contains an entry for `key`.


```move
module 0x1::table {
    public fun contains<K: copy, drop, V>(table: &table::Table<K, V>, key: K): bool
}
```


##### Implementation


```move
module 0x1::table {
    public fun contains<K: copy + drop, V>(table: &Table<K, V>, key: K): bool {
        contains_box<K, V, Box<V>>(table, key)
    }
}
```


<a id="0x1_table_destroy"></a>

## Function `destroy`



```move
module 0x1::table {
    public(friend) fun destroy<K: copy, drop, V>(table: table::Table<K, V>)
}
```


##### Implementation


```move
module 0x1::table {
    public(friend) fun destroy<K: copy + drop, V>(table: Table<K, V>) {
        destroy_empty_box<K, V, Box<V>>(&table);
        drop_unchecked_box<K, V, Box<V>>(table)
    }
}
```


<a id="0x1_table_new_table_handle"></a>

## Function `new_table_handle`



```move
module 0x1::table {
    fun new_table_handle<K, V>(): address
}
```


##### Implementation


```move
module 0x1::table {
    native fun new_table_handle<K, V>(): address;
}
```


<a id="0x1_table_add_box"></a>

## Function `add_box`



```move
module 0x1::table {
    fun add_box<K: copy, drop, V, B>(table: &mut table::Table<K, V>, key: K, val: table::Box<V>)
}
```


##### Implementation


```move
module 0x1::table {
    native fun add_box<K: copy + drop, V, B>(table: &mut Table<K, V>, key: K, val: Box<V>);
}
```


<a id="0x1_table_borrow_box"></a>

## Function `borrow_box`



```move
module 0x1::table {
    fun borrow_box<K: copy, drop, V, B>(table: &table::Table<K, V>, key: K): &table::Box<V>
}
```


##### Implementation


```move
module 0x1::table {
    native fun borrow_box<K: copy + drop, V, B>(table: &Table<K, V>, key: K): &Box<V>;
}
```


<a id="0x1_table_borrow_box_mut"></a>

## Function `borrow_box_mut`



```move
module 0x1::table {
    fun borrow_box_mut<K: copy, drop, V, B>(table: &mut table::Table<K, V>, key: K): &mut table::Box<V>
}
```


##### Implementation


```move
module 0x1::table {
    native fun borrow_box_mut<K: copy + drop, V, B>(table: &mut Table<K, V>, key: K): &mut Box<V>;
}
```


<a id="0x1_table_contains_box"></a>

## Function `contains_box`



```move
module 0x1::table {
    fun contains_box<K: copy, drop, V, B>(table: &table::Table<K, V>, key: K): bool
}
```


##### Implementation


```move
module 0x1::table {
    native fun contains_box<K: copy + drop, V, B>(table: &Table<K, V>, key: K): bool;
}
```


<a id="0x1_table_remove_box"></a>

## Function `remove_box`



```move
module 0x1::table {
    fun remove_box<K: copy, drop, V, B>(table: &mut table::Table<K, V>, key: K): table::Box<V>
}
```


##### Implementation


```move
module 0x1::table {
    native fun remove_box<K: copy + drop, V, B>(table: &mut Table<K, V>, key: K): Box<V>;
}
```


<a id="0x1_table_destroy_empty_box"></a>

## Function `destroy_empty_box`



```move
module 0x1::table {
    fun destroy_empty_box<K: copy, drop, V, B>(table: &table::Table<K, V>)
}
```


##### Implementation


```move
module 0x1::table {
    native fun destroy_empty_box<K: copy + drop, V, B>(table: &Table<K, V>);
}
```


<a id="0x1_table_drop_unchecked_box"></a>

## Function `drop_unchecked_box`



```move
module 0x1::table {
    fun drop_unchecked_box<K: copy, drop, V, B>(table: table::Table<K, V>)
}
```


##### Implementation


```move
module 0x1::table {
    native fun drop_unchecked_box<K: copy + drop, V, B>(table: Table<K, V>);
}
```


<a id="@Specification_0"></a>

## Specification


<a id="@Specification_0_Table"></a>

### Struct `Table`


```move
module 0x1::table {
    struct Table<K: copy, drop, V> has store
}
```


<dl>
<dt>
`handle: address`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::table {
    pragma intrinsic = map,
        map_new = new,
        map_destroy_empty = destroy,
        map_has_key = contains,
        map_add_no_override = add,
        map_add_override_if_exists = upsert,
        map_del_must_exist = remove,
        map_borrow = borrow,
        map_borrow_mut = borrow_mut,
        map_borrow_mut_with_default = borrow_mut_with_default,
        map_spec_get = spec_get,
        map_spec_set = spec_set,
        map_spec_del = spec_remove,
        map_spec_has_key = spec_contains;
}
```


<a id="@Specification_0_new"></a>

### Function `new`


```move
module 0x1::table {
    public fun new<K: copy, drop, V: store>(): table::Table<K, V>
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_add"></a>

### Function `add`


```move
module 0x1::table {
    public fun add<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K, val: V)
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_borrow"></a>

### Function `borrow`


```move
module 0x1::table {
    public fun borrow<K: copy, drop, V>(table: &table::Table<K, V>, key: K): &V
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_borrow_mut"></a>

### Function `borrow_mut`


```move
module 0x1::table {
    public fun borrow_mut<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K): &mut V
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_borrow_mut_with_default"></a>

### Function `borrow_mut_with_default`


```move
module 0x1::table {
    public fun borrow_mut_with_default<K: copy, drop, V: drop>(table: &mut table::Table<K, V>, key: K, default: V): &mut V
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_upsert"></a>

### Function `upsert`


```move
module 0x1::table {
    public fun upsert<K: copy, drop, V: drop>(table: &mut table::Table<K, V>, key: K, value: V)
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_remove"></a>

### Function `remove`


```move
module 0x1::table {
    public fun remove<K: copy, drop, V>(table: &mut table::Table<K, V>, key: K): V
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```


<a id="@Specification_0_contains"></a>

### Function `contains`


```move
module 0x1::table {
    public fun contains<K: copy, drop, V>(table: &table::Table<K, V>, key: K): bool
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```



<a id="0x1_table_spec_contains"></a>


```move
module 0x1::table {
    native fun spec_contains<K, V>(t: Table<K, V>, k: K): bool;
}
```



<a id="0x1_table_spec_remove"></a>


```move
module 0x1::table {
    native fun spec_remove<K, V>(t: Table<K, V>, k: K): Table<K, V>;
}
```



<a id="0x1_table_spec_set"></a>


```move
module 0x1::table {
    native fun spec_set<K, V>(t: Table<K, V>, k: K, v: V): Table<K, V>;
}
```



<a id="0x1_table_spec_get"></a>


```move
module 0x1::table {
    native fun spec_get<K, V>(t: Table<K, V>, k: K): V;
}
```


<a id="@Specification_0_destroy"></a>

### Function `destroy`


```move
module 0x1::table {
    public(friend) fun destroy<K: copy, drop, V>(table: table::Table<K, V>)
}
```



```move
module 0x1::table {
    pragma intrinsic;
}
```
