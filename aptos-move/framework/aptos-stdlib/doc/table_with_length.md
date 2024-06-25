
<a id="0x1_table_with_length"></a>

# Module `0x1::table_with_length`

Extends Table and provides functions such as length and the ability to be destroyed


-  [Struct `TableWithLength`](#0x1_table_with_length_TableWithLength)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_table_with_length_new)
-  [Function `destroy_empty`](#0x1_table_with_length_destroy_empty)
-  [Function `add`](#0x1_table_with_length_add)
-  [Function `borrow`](#0x1_table_with_length_borrow)
-  [Function `borrow_mut`](#0x1_table_with_length_borrow_mut)
-  [Function `length`](#0x1_table_with_length_length)
-  [Function `empty`](#0x1_table_with_length_empty)
-  [Function `borrow_mut_with_default`](#0x1_table_with_length_borrow_mut_with_default)
-  [Function `upsert`](#0x1_table_with_length_upsert)
-  [Function `remove`](#0x1_table_with_length_remove)
-  [Function `contains`](#0x1_table_with_length_contains)
-  [Specification](#@Specification_1)
    -  [Struct `TableWithLength`](#@Specification_1_TableWithLength)
    -  [Function `new`](#@Specification_1_new)
    -  [Function `destroy_empty`](#@Specification_1_destroy_empty)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `length`](#@Specification_1_length)
    -  [Function `empty`](#@Specification_1_empty)
    -  [Function `borrow_mut_with_default`](#@Specification_1_borrow_mut_with_default)
    -  [Function `upsert`](#@Specification_1_upsert)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `contains`](#@Specification_1_contains)


```move
module 0x1::table_with_length {
    use 0x1::error;
    use 0x1::table;
}
```


<a id="0x1_table_with_length_TableWithLength"></a>

## Struct `TableWithLength`

Type of tables


```move
module 0x1::table_with_length {
    struct TableWithLength<K: copy, drop, V> has store
}
```


##### Fields


<dl>
<dt>
`inner: table::Table<K, V>`
</dt>
<dd>

</dd>
<dt>
`length: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_table_with_length_EALREADY_EXISTS"></a>



```move
module 0x1::table_with_length {
    const EALREADY_EXISTS: u64 = 100;
}
```


<a id="0x1_table_with_length_ENOT_EMPTY"></a>



```move
module 0x1::table_with_length {
    const ENOT_EMPTY: u64 = 102;
}
```


<a id="0x1_table_with_length_ENOT_FOUND"></a>



```move
module 0x1::table_with_length {
    const ENOT_FOUND: u64 = 101;
}
```


<a id="0x1_table_with_length_new"></a>

## Function `new`

Create a new Table.


```move
module 0x1::table_with_length {
    public fun new<K: copy, drop, V: store>(): table_with_length::TableWithLength<K, V>
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun new<K: copy + drop, V: store>(): TableWithLength<K, V> {
        TableWithLength {
            inner: table::new<K, V>(),
            length: 0,
        }
    }
}
```


<a id="0x1_table_with_length_destroy_empty"></a>

## Function `destroy_empty`

Destroy a table. The table must be empty to succeed.


```move
module 0x1::table_with_length {
    public fun destroy_empty<K: copy, drop, V>(table: table_with_length::TableWithLength<K, V>)
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun destroy_empty<K: copy + drop, V>(table: TableWithLength<K, V>) {
        assert!(table.length == 0, error::invalid_state(ENOT_EMPTY));
        let TableWithLength { inner, length: _ } = table;
        table::destroy(inner)
    }
}
```


<a id="0x1_table_with_length_add"></a>

## Function `add`

Add a new entry to the table. Aborts if an entry for this
key already exists. The entry itself is not stored in the
table, and cannot be discovered from it.


```move
module 0x1::table_with_length {
    public fun add<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K, val: V)
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun add<K: copy + drop, V>(table: &mut TableWithLength<K, V>, key: K, val: V) {
        table::add(&mut table.inner, key, val);
        table.length = table.length + 1;
    }
}
```


<a id="0x1_table_with_length_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table_with_length {
    public fun borrow<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>, key: K): &V
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun borrow<K: copy + drop, V>(table: &TableWithLength<K, V>, key: K): &V {
        table::borrow(&table.inner, key)
    }
}
```


<a id="0x1_table_with_length_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table_with_length {
    public fun borrow_mut<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K): &mut V
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun borrow_mut<K: copy + drop, V>(table: &mut TableWithLength<K, V>, key: K): &mut V {
        table::borrow_mut(&mut table.inner, key)
    }
}
```


<a id="0x1_table_with_length_length"></a>

## Function `length`

Returns the length of the table, i.e. the number of entries.


```move
module 0x1::table_with_length {
    public fun length<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>): u64
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun length<K: copy + drop, V>(table: &TableWithLength<K, V>): u64 {
        table.length
    }
}
```


<a id="0x1_table_with_length_empty"></a>

## Function `empty`

Returns true if this table is empty.


```move
module 0x1::table_with_length {
    public fun empty<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>): bool
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun empty<K: copy + drop, V>(table: &TableWithLength<K, V>): bool {
        table.length == 0
    }
}
```


<a id="0x1_table_with_length_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which `key` maps to.
Insert the pair (`key`, `default`) first if there is no entry for `key`.


```move
module 0x1::table_with_length {
    public fun borrow_mut_with_default<K: copy, drop, V: drop>(table: &mut table_with_length::TableWithLength<K, V>, key: K, default: V): &mut V
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun borrow_mut_with_default<K: copy + drop, V: drop>(table: &mut TableWithLength<K, V>, key: K, default: V): &mut V {
        if (table::contains(&table.inner, key)) {
            table::borrow_mut(&mut table.inner, key)
        } else {
            table::add(&mut table.inner, key, default);
            table.length = table.length + 1;
            table::borrow_mut(&mut table.inner, key)
        }
    }
}
```


<a id="0x1_table_with_length_upsert"></a>

## Function `upsert`

Insert the pair (`key`, `value`) if there is no entry for `key`.
update the value of the entry for `key` to `value` otherwise


```move
module 0x1::table_with_length {
    public fun upsert<K: copy, drop, V: drop>(table: &mut table_with_length::TableWithLength<K, V>, key: K, value: V)
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun upsert<K: copy + drop, V: drop>(table: &mut TableWithLength<K, V>, key: K, value: V) {
        if (!table::contains(&table.inner, key)) {
            add(table, copy key, value)
        } else {
            let ref = table::borrow_mut(&mut table.inner, key);
            *ref = value;
        };
    }
}
```


<a id="0x1_table_with_length_remove"></a>

## Function `remove`

Remove from `table` and return the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::table_with_length {
    public fun remove<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K): V
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun remove<K: copy + drop, V>(table: &mut TableWithLength<K, V>, key: K): V {
        let val = table::remove(&mut table.inner, key);
        table.length = table.length - 1;
        val
    }
}
```


<a id="0x1_table_with_length_contains"></a>

## Function `contains`

Returns true iff `table` contains an entry for `key`.


```move
module 0x1::table_with_length {
    public fun contains<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>, key: K): bool
}
```


##### Implementation


```move
module 0x1::table_with_length {
    public fun contains<K: copy + drop, V>(table: &TableWithLength<K, V>, key: K): bool {
        table::contains(&table.inner, key)
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_TableWithLength"></a>

### Struct `TableWithLength`


```move
module 0x1::table_with_length {
    struct TableWithLength<K: copy, drop, V> has store
}
```


<dl>
<dt>
`inner: table::Table<K, V>`
</dt>
<dd>

</dd>
<dt>
`length: u64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::table_with_length {
    pragma intrinsic = map,
        map_new = new,
        map_destroy_empty = destroy_empty,
        map_len = length,
        map_is_empty = empty,
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
        map_spec_len = spec_len,
        map_spec_has_key = spec_contains;
}
```


<a id="@Specification_1_new"></a>

### Function `new`


```move
module 0x1::table_with_length {
    public fun new<K: copy, drop, V: store>(): table_with_length::TableWithLength<K, V>
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_destroy_empty"></a>

### Function `destroy_empty`


```move
module 0x1::table_with_length {
    public fun destroy_empty<K: copy, drop, V>(table: table_with_length::TableWithLength<K, V>)
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::table_with_length {
    public fun add<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K, val: V)
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x1::table_with_length {
    public fun borrow<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>, key: K): &V
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


```move
module 0x1::table_with_length {
    public fun borrow_mut<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K): &mut V
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_length"></a>

### Function `length`


```move
module 0x1::table_with_length {
    public fun length<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>): u64
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_empty"></a>

### Function `empty`


```move
module 0x1::table_with_length {
    public fun empty<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>): bool
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_borrow_mut_with_default"></a>

### Function `borrow_mut_with_default`


```move
module 0x1::table_with_length {
    public fun borrow_mut_with_default<K: copy, drop, V: drop>(table: &mut table_with_length::TableWithLength<K, V>, key: K, default: V): &mut V
}
```



```move
module 0x1::table_with_length {
    aborts_if false;
    pragma intrinsic;
}
```


<a id="@Specification_1_upsert"></a>

### Function `upsert`


```move
module 0x1::table_with_length {
    public fun upsert<K: copy, drop, V: drop>(table: &mut table_with_length::TableWithLength<K, V>, key: K, value: V)
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x1::table_with_length {
    public fun remove<K: copy, drop, V>(table: &mut table_with_length::TableWithLength<K, V>, key: K): V
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```


<a id="@Specification_1_contains"></a>

### Function `contains`


```move
module 0x1::table_with_length {
    public fun contains<K: copy, drop, V>(table: &table_with_length::TableWithLength<K, V>, key: K): bool
}
```



```move
module 0x1::table_with_length {
    pragma intrinsic;
}
```



<a id="0x1_table_with_length_spec_len"></a>


```move
module 0x1::table_with_length {
    native fun spec_len<K, V>(t: TableWithLength<K, V>): num;
}
```



<a id="0x1_table_with_length_spec_contains"></a>


```move
module 0x1::table_with_length {
    native fun spec_contains<K, V>(t: TableWithLength<K, V>, k: K): bool;
}
```



<a id="0x1_table_with_length_spec_set"></a>


```move
module 0x1::table_with_length {
    native fun spec_set<K, V>(t: TableWithLength<K, V>, k: K, v: V): TableWithLength<K, V>;
}
```



<a id="0x1_table_with_length_spec_remove"></a>


```move
module 0x1::table_with_length {
    native fun spec_remove<K, V>(t: TableWithLength<K, V>, k: K): TableWithLength<K, V>;
}
```



<a id="0x1_table_with_length_spec_get"></a>


```move
module 0x1::table_with_length {
    native fun spec_get<K, V>(t: TableWithLength<K, V>, k: K): V;
}
```
