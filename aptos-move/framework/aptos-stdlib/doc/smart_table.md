
<a id="0x1_smart_table"></a>

# Module `0x1::smart_table`

A smart table implementation based on linear hashing. (https://en.wikipedia.org/wiki/Linear_hashing)
Compare to Table, it uses less storage slots but has higher chance of collision, a trade&#45;off between space and time.
Compare to other dynamic hashing implementation, linear hashing splits one bucket a time instead of doubling buckets
when expanding to avoid unexpected gas cost.
SmartTable uses faster hash function SipHash instead of cryptographically secure hash functions like sha3&#45;256 since
it tolerates collisions.


-  [Struct `Entry`](#0x1_smart_table_Entry)
-  [Struct `SmartTable`](#0x1_smart_table_SmartTable)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_smart_table_new)
-  [Function `new_with_config`](#0x1_smart_table_new_with_config)
-  [Function `destroy_empty`](#0x1_smart_table_destroy_empty)
-  [Function `destroy`](#0x1_smart_table_destroy)
-  [Function `clear`](#0x1_smart_table_clear)
-  [Function `add`](#0x1_smart_table_add)
-  [Function `add_all`](#0x1_smart_table_add_all)
-  [Function `unzip_entries`](#0x1_smart_table_unzip_entries)
-  [Function `to_simple_map`](#0x1_smart_table_to_simple_map)
-  [Function `keys`](#0x1_smart_table_keys)
-  [Function `keys_paginated`](#0x1_smart_table_keys_paginated)
-  [Function `split_one_bucket`](#0x1_smart_table_split_one_bucket)
-  [Function `bucket_index`](#0x1_smart_table_bucket_index)
-  [Function `borrow`](#0x1_smart_table_borrow)
-  [Function `borrow_with_default`](#0x1_smart_table_borrow_with_default)
-  [Function `borrow_mut`](#0x1_smart_table_borrow_mut)
-  [Function `borrow_mut_with_default`](#0x1_smart_table_borrow_mut_with_default)
-  [Function `contains`](#0x1_smart_table_contains)
-  [Function `remove`](#0x1_smart_table_remove)
-  [Function `upsert`](#0x1_smart_table_upsert)
-  [Function `length`](#0x1_smart_table_length)
-  [Function `load_factor`](#0x1_smart_table_load_factor)
-  [Function `update_split_load_threshold`](#0x1_smart_table_update_split_load_threshold)
-  [Function `update_target_bucket_size`](#0x1_smart_table_update_target_bucket_size)
-  [Function `for_each_ref`](#0x1_smart_table_for_each_ref)
-  [Function `for_each_mut`](#0x1_smart_table_for_each_mut)
-  [Function `map_ref`](#0x1_smart_table_map_ref)
-  [Function `any`](#0x1_smart_table_any)
-  [Function `borrow_kv`](#0x1_smart_table_borrow_kv)
-  [Function `borrow_kv_mut`](#0x1_smart_table_borrow_kv_mut)
-  [Function `num_buckets`](#0x1_smart_table_num_buckets)
-  [Function `borrow_buckets`](#0x1_smart_table_borrow_buckets)
-  [Function `borrow_buckets_mut`](#0x1_smart_table_borrow_buckets_mut)
-  [Specification](#@Specification_1)
    -  [Struct `SmartTable`](#@Specification_1_SmartTable)
    -  [Function `new_with_config`](#@Specification_1_new_with_config)
    -  [Function `destroy`](#@Specification_1_destroy)
    -  [Function `clear`](#@Specification_1_clear)
    -  [Function `add_all`](#@Specification_1_add_all)
    -  [Function `to_simple_map`](#@Specification_1_to_simple_map)
    -  [Function `keys`](#@Specification_1_keys)
    -  [Function `keys_paginated`](#@Specification_1_keys_paginated)
    -  [Function `split_one_bucket`](#@Specification_1_split_one_bucket)
    -  [Function `bucket_index`](#@Specification_1_bucket_index)
    -  [Function `borrow_with_default`](#@Specification_1_borrow_with_default)
    -  [Function `load_factor`](#@Specification_1_load_factor)
    -  [Function `update_split_load_threshold`](#@Specification_1_update_split_load_threshold)
    -  [Function `update_target_bucket_size`](#@Specification_1_update_target_bucket_size)
    -  [Function `borrow_kv`](#@Specification_1_borrow_kv)
    -  [Function `borrow_kv_mut`](#@Specification_1_borrow_kv_mut)
    -  [Function `num_buckets`](#@Specification_1_num_buckets)
    -  [Function `borrow_buckets`](#@Specification_1_borrow_buckets)
    -  [Function `borrow_buckets_mut`](#@Specification_1_borrow_buckets_mut)


```move
module 0x1::smart_table {
    use 0x1::aptos_hash;
    use 0x1::error;
    use 0x1::math64;
    use 0x1::option;
    use 0x1::simple_map;
    use 0x1::table_with_length;
    use 0x1::type_info;
    use 0x1::vector;
}
```


<a id="0x1_smart_table_Entry"></a>

## Struct `Entry`

SmartTable entry contains both the key and value.


```move
module 0x1::smart_table {
    struct Entry<K, V> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`hash: u64`
</dt>
<dd>

</dd>
<dt>
`key: K`
</dt>
<dd>

</dd>
<dt>
`value: V`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_smart_table_SmartTable"></a>

## Struct `SmartTable`



```move
module 0x1::smart_table {
    struct SmartTable<K, V> has store
}
```


##### Fields


<dl>
<dt>
`buckets: table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>`
</dt>
<dd>

</dd>
<dt>
`num_buckets: u64`
</dt>
<dd>

</dd>
<dt>
`level: u8`
</dt>
<dd>

</dd>
<dt>
`size: u64`
</dt>
<dd>

</dd>
<dt>
`split_load_threshold: u8`
</dt>
<dd>

</dd>
<dt>
`target_bucket_size: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_smart_table_ENOT_EMPTY"></a>

Cannot destroy non&#45;empty hashmap


```move
module 0x1::smart_table {
    const ENOT_EMPTY: u64 = 3;
}
```


<a id="0x1_smart_table_ENOT_FOUND"></a>

Key not found in the smart table


```move
module 0x1::smart_table {
    const ENOT_FOUND: u64 = 1;
}
```


<a id="0x1_smart_table_EALREADY_EXIST"></a>

Key already exists


```move
module 0x1::smart_table {
    const EALREADY_EXIST: u64 = 4;
}
```


<a id="0x1_smart_table_EEXCEED_MAX_BUCKET_SIZE"></a>

Invalid target bucket size.


```move
module 0x1::smart_table {
    const EEXCEED_MAX_BUCKET_SIZE: u64 = 7;
}
```


<a id="0x1_smart_table_EINVALID_BUCKET_INDEX"></a>

Invalid bucket index.


```move
module 0x1::smart_table {
    const EINVALID_BUCKET_INDEX: u64 = 8;
}
```


<a id="0x1_smart_table_EINVALID_LOAD_THRESHOLD_PERCENT"></a>

Invalid load threshold percent to trigger split.


```move
module 0x1::smart_table {
    const EINVALID_LOAD_THRESHOLD_PERCENT: u64 = 5;
}
```


<a id="0x1_smart_table_EINVALID_TARGET_BUCKET_SIZE"></a>

Invalid target bucket size.


```move
module 0x1::smart_table {
    const EINVALID_TARGET_BUCKET_SIZE: u64 = 6;
}
```


<a id="0x1_smart_table_EINVALID_VECTOR_INDEX"></a>

Invalid vector index within a bucket.


```move
module 0x1::smart_table {
    const EINVALID_VECTOR_INDEX: u64 = 9;
}
```


<a id="0x1_smart_table_EZERO_CAPACITY"></a>

Smart table capacity must be larger than 0


```move
module 0x1::smart_table {
    const EZERO_CAPACITY: u64 = 2;
}
```


<a id="0x1_smart_table_new"></a>

## Function `new`

Create an empty SmartTable with default configurations.


```move
module 0x1::smart_table {
    public fun new<K: copy, drop, store, V: store>(): smart_table::SmartTable<K, V>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun new<K: copy + drop + store, V: store>(): SmartTable<K, V> {
        new_with_config<K, V>(0, 0, 0)
    }
}
```


<a id="0x1_smart_table_new_with_config"></a>

## Function `new_with_config`

Create an empty SmartTable with customized configurations.
`num_initial_buckets`: The number of buckets on initialization. 0 means using default value.
`split_load_threshold`: The percent number which once reached, split will be triggered. 0 means using default
value.
`target_bucket_size`: The target number of entries per bucket, though not guaranteed. 0 means not set and will
dynamically assgined by the contract code.


```move
module 0x1::smart_table {
    public fun new_with_config<K: copy, drop, store, V: store>(num_initial_buckets: u64, split_load_threshold: u8, target_bucket_size: u64): smart_table::SmartTable<K, V>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun new_with_config<K: copy + drop + store, V: store>(
        num_initial_buckets: u64,
        split_load_threshold: u8,
        target_bucket_size: u64
    ): SmartTable<K, V> {
        assert!(split_load_threshold <= 100, error::invalid_argument(EINVALID_LOAD_THRESHOLD_PERCENT));
        let buckets = table_with_length::new();
        table_with_length::add(&mut buckets, 0, vector::empty());
        let table = SmartTable {
            buckets,
            num_buckets: 1,
            level: 0,
            size: 0,
            // The default split load threshold is 75%.
            split_load_threshold: if (split_load_threshold == 0) { 75 } else { split_load_threshold },
            target_bucket_size,
        };
        // The default number of initial buckets is 2.
        if (num_initial_buckets == 0) {
            num_initial_buckets = 2;
        };
        while (num_initial_buckets > 1) {
            num_initial_buckets = num_initial_buckets - 1;
            split_one_bucket(&mut table);
        };
        table
    }
}
```


<a id="0x1_smart_table_destroy_empty"></a>

## Function `destroy_empty`

Destroy empty table.
Aborts if it&apos;s not empty.


```move
module 0x1::smart_table {
    public fun destroy_empty<K, V>(table: smart_table::SmartTable<K, V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun destroy_empty<K, V>(table: SmartTable<K, V>) {
        assert!(table.size == 0, error::invalid_argument(ENOT_EMPTY));
        let i = 0;
        while (i < table.num_buckets) {
            vector::destroy_empty(table_with_length::remove(&mut table.buckets, i));
            i = i + 1;
        };
        let SmartTable { buckets, num_buckets: _, level: _, size: _, split_load_threshold: _, target_bucket_size: _ } = table;
        table_with_length::destroy_empty(buckets);
    }
}
```


<a id="0x1_smart_table_destroy"></a>

## Function `destroy`

Destroy a table completely when V has `drop`.


```move
module 0x1::smart_table {
    public fun destroy<K: drop, V: drop>(table: smart_table::SmartTable<K, V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun destroy<K: drop, V: drop>(table: SmartTable<K, V>) {
        clear(&mut table);
        destroy_empty(table);
    }
}
```


<a id="0x1_smart_table_clear"></a>

## Function `clear`

Clear a table completely when T has `drop`.


```move
module 0x1::smart_table {
    public fun clear<K: drop, V: drop>(table: &mut smart_table::SmartTable<K, V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun clear<K: drop, V: drop>(table: &mut SmartTable<K, V>) {
        *table_with_length::borrow_mut(&mut table.buckets, 0) = vector::empty();
        let i = 1;
        while (i < table.num_buckets) {
            table_with_length::remove(&mut table.buckets, i);
            i = i + 1;
        };
        table.num_buckets = 1;
        table.level = 0;
        table.size = 0;
    }
}
```


<a id="0x1_smart_table_add"></a>

## Function `add`

Add (key, value) pair in the hash map, it may grow one bucket if current load factor exceeds the threshold.
Note it may not split the actual overflowed bucket. Instead, it was determined by `num_buckets` and `level`.
For standard linear hash algorithm, it is stored as a variable but `num_buckets` here could be leveraged.
Abort if `key` already exists.
Note: This method may occasionally cost much more gas when triggering bucket split.


```move
module 0x1::smart_table {
    public fun add<K, V>(table: &mut smart_table::SmartTable<K, V>, key: K, value: V)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun add<K, V>(table: &mut SmartTable<K, V>, key: K, value: V) {
        let hash = sip_hash_from_value(&key);
        let index = bucket_index(table.level, table.num_buckets, hash);
        let bucket = table_with_length::borrow_mut(&mut table.buckets, index);
        // We set a per-bucket limit here with a upper bound (10000) that nobody should normally reach.
        assert!(vector::length(bucket) <= 10000, error::permission_denied(EEXCEED_MAX_BUCKET_SIZE));
        assert!(vector::all(bucket, | entry | {
            let e: &Entry<K, V> = entry;
            &e.key != &key
        }), error::invalid_argument(EALREADY_EXIST));
        let e = Entry { hash, key, value };
        if (table.target_bucket_size == 0) {
            let estimated_entry_size = max(size_of_val(&e), 1);
            table.target_bucket_size = max(1024 /* free_write_quota */ / estimated_entry_size, 1);
        };
        vector::push_back(bucket, e);
        table.size = table.size + 1;

        if (load_factor(table) >= (table.split_load_threshold as u64)) {
            split_one_bucket(table);
        }
    }
}
```


<a id="0x1_smart_table_add_all"></a>

## Function `add_all`

Add multiple key/value pairs to the smart table. The keys must not already exist.


```move
module 0x1::smart_table {
    public fun add_all<K, V>(table: &mut smart_table::SmartTable<K, V>, keys: vector<K>, values: vector<V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun add_all<K, V>(table: &mut SmartTable<K, V>, keys: vector<K>, values: vector<V>) {
        vector::zip(keys, values, |key, value| { add(table, key, value); });
    }
}
```


<a id="0x1_smart_table_unzip_entries"></a>

## Function `unzip_entries`



```move
module 0x1::smart_table {
    fun unzip_entries<K: copy, V: copy>(entries: &vector<smart_table::Entry<K, V>>): (vector<K>, vector<V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    inline fun unzip_entries<K: copy, V: copy>(entries: &vector<Entry<K, V>>): (vector<K>, vector<V>) {
        let keys = vector[];
        let values = vector[];
        vector::for_each_ref(entries, |e|{
            let entry: &Entry<K, V> = e;
            vector::push_back(&mut keys, entry.key);
            vector::push_back(&mut values, entry.value);
        });
        (keys, values)
    }
}
```


<a id="0x1_smart_table_to_simple_map"></a>

## Function `to_simple_map`

Convert a smart table to a simple_map, which is supposed to be called mostly by view functions to get an atomic
view of the whole table.
Disclaimer: This function may be costly as the smart table may be huge in size. Use it at your own discretion.


```move
module 0x1::smart_table {
    public fun to_simple_map<K: copy, drop, store, V: copy, store>(table: &smart_table::SmartTable<K, V>): simple_map::SimpleMap<K, V>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun to_simple_map<K: store + copy + drop, V: store + copy>(
        table: &SmartTable<K, V>,
    ): SimpleMap<K, V> {
        let i = 0;
        let res = simple_map::new<K, V>();
        while (i < table.num_buckets) {
            let (keys, values) = unzip_entries(table_with_length::borrow(&table.buckets, i));
            simple_map::add_all(&mut res, keys, values);
            i = i + 1;
        };
        res
    }
}
```


<a id="0x1_smart_table_keys"></a>

## Function `keys`

Get all keys in a smart table.

For a large enough smart table this function will fail due to execution gas limits, and
`keys_paginated` should be used instead.


```move
module 0x1::smart_table {
    public fun keys<K: copy, drop, store, V: copy, store>(table_ref: &smart_table::SmartTable<K, V>): vector<K>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun keys<K: store + copy + drop, V: store + copy>(
        table_ref: &SmartTable<K, V>
    ): vector<K> {
        let (keys, _, _) = keys_paginated(table_ref, 0, 0, length(table_ref));
        keys
    }
}
```


<a id="0x1_smart_table_keys_paginated"></a>

## Function `keys_paginated`

Get keys from a smart table, paginated.

This function can be used to paginate all keys in a large smart table outside of runtime,
e.g. through chained view function calls. The maximum `num_keys_to_get` before hitting gas
limits depends on the data types in the smart table.

When starting pagination, pass `starting_bucket_index` &#61; `starting_vector_index` &#61; 0.

The function will then return a vector of keys, an optional bucket index, and an optional
vector index. The unpacked return indices can then be used as inputs to another pagination
call, which will return a vector of more keys. This process can be repeated until the
returned bucket index and vector index value options are both none, which means that
pagination is complete. For an example, see `test_keys()`.


```move
module 0x1::smart_table {
    public fun keys_paginated<K: copy, drop, store, V: copy, store>(table_ref: &smart_table::SmartTable<K, V>, starting_bucket_index: u64, starting_vector_index: u64, num_keys_to_get: u64): (vector<K>, option::Option<u64>, option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun keys_paginated<K: store + copy + drop, V: store + copy>(
        table_ref: &SmartTable<K, V>,
        starting_bucket_index: u64,
        starting_vector_index: u64,
        num_keys_to_get: u64,
    ): (
        vector<K>,
        Option<u64>,
        Option<u64>,
    ) {
        let num_buckets = table_ref.num_buckets;
        let buckets_ref = &table_ref.buckets;
        assert!(starting_bucket_index < num_buckets, EINVALID_BUCKET_INDEX);
        let bucket_ref = table_with_length::borrow(buckets_ref, starting_bucket_index);
        let bucket_length = vector::length(bucket_ref);
        assert!(
            // In the general case, starting vector index should never be equal to bucket length
            // because then iteration will attempt to borrow a vector element that is out of bounds.
            // However starting vector index can be equal to bucket length in the special case of
            // starting iteration at the beginning of an empty bucket since buckets are never
            // destroyed, only emptied.
            starting_vector_index < bucket_length || starting_vector_index == 0,
            EINVALID_VECTOR_INDEX
        );
        let keys = vector[];
        if (num_keys_to_get == 0) return
            (keys, option::some(starting_bucket_index), option::some(starting_vector_index));
        for (bucket_index in starting_bucket_index..num_buckets) {
            bucket_ref = table_with_length::borrow(buckets_ref, bucket_index);
            bucket_length = vector::length(bucket_ref);
            for (vector_index in starting_vector_index..bucket_length) {
                vector::push_back(&mut keys, vector::borrow(bucket_ref, vector_index).key);
                num_keys_to_get = num_keys_to_get - 1;
                if (num_keys_to_get == 0) {
                    vector_index = vector_index + 1;
                    return if (vector_index == bucket_length) {
                        bucket_index = bucket_index + 1;
                        if (bucket_index < num_buckets) {
                            (keys, option::some(bucket_index), option::some(0))
                        } else {
                            (keys, option::none(), option::none())
                        }
                    } else {
                        (keys, option::some(bucket_index), option::some(vector_index))
                    }
                };
            };
            starting_vector_index = 0; // Start parsing the next bucket at vector index 0.
        };
        (keys, option::none(), option::none())
    }
}
```


<a id="0x1_smart_table_split_one_bucket"></a>

## Function `split_one_bucket`

Decide which is the next bucket to split and split it into two with the elements inside the bucket.


```move
module 0x1::smart_table {
    fun split_one_bucket<K, V>(table: &mut smart_table::SmartTable<K, V>)
}
```


##### Implementation


```move
module 0x1::smart_table {
    fun split_one_bucket<K, V>(table: &mut SmartTable<K, V>) {
        let new_bucket_index = table.num_buckets;
        // the next bucket to split is num_bucket without the most significant bit.
        let to_split = new_bucket_index ^ (1 << table.level);
        table.num_buckets = new_bucket_index + 1;
        // if the whole level is splitted once, bump the level.
        if (to_split + 1 == 1 << table.level) {
            table.level = table.level + 1;
        };
        let old_bucket = table_with_length::borrow_mut(&mut table.buckets, to_split);
        // partition the bucket, [0..p) stays in old bucket, [p..len) goes to new bucket
        let p = vector::partition(old_bucket, |e| {
            let entry: &Entry<K, V> = e; // Explicit type to satisfy compiler
            bucket_index(table.level, table.num_buckets, entry.hash) != new_bucket_index
        });
        let new_bucket = vector::trim_reverse(old_bucket, p);
        table_with_length::add(&mut table.buckets, new_bucket_index, new_bucket);
    }
}
```


<a id="0x1_smart_table_bucket_index"></a>

## Function `bucket_index`

Return the expected bucket index to find the hash.
Basically, it use different base `1 << level` vs `1 << (level + 1)` in modulo operation based on the target
bucket index compared to the index of the next bucket to split.


```move
module 0x1::smart_table {
    fun bucket_index(level: u8, num_buckets: u64, hash: u64): u64
}
```


##### Implementation


```move
module 0x1::smart_table {
    fun bucket_index(level: u8, num_buckets: u64, hash: u64): u64 {
        let index = hash % (1 << (level + 1));
        if (index < num_buckets) {
            // in existing bucket
            index
        } else {
            // in unsplitted bucket
            index % (1 << level)
        }
    }
}
```


<a id="0x1_smart_table_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::smart_table {
    public fun borrow<K: drop, V>(table: &smart_table::SmartTable<K, V>, key: K): &V
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow<K: drop, V>(table: &SmartTable<K, V>, key: K): &V {
        let index = bucket_index(table.level, table.num_buckets, sip_hash_from_value(&key));
        let bucket = table_with_length::borrow(&table.buckets, index);
        let i = 0;
        let len = vector::length(bucket);
        while (i < len) {
            let entry = vector::borrow(bucket, i);
            if (&entry.key == &key) {
                return &entry.value
            };
            i = i + 1;
        };
        abort error::invalid_argument(ENOT_FOUND)
    }
}
```


<a id="0x1_smart_table_borrow_with_default"></a>

## Function `borrow_with_default`

Acquire an immutable reference to the value which `key` maps to.
Returns specified default value if there is no entry for `key`.


```move
module 0x1::smart_table {
    public fun borrow_with_default<K: copy, drop, V>(table: &smart_table::SmartTable<K, V>, key: K, default: &V): &V
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_with_default<K: copy + drop, V>(table: &SmartTable<K, V>, key: K, default: &V): &V {
        if (!contains(table, copy key)) {
            default
        } else {
            borrow(table, copy key)
        }
    }
}
```


<a id="0x1_smart_table_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::smart_table {
    public fun borrow_mut<K: drop, V>(table: &mut smart_table::SmartTable<K, V>, key: K): &mut V
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_mut<K: drop, V>(table: &mut SmartTable<K, V>, key: K): &mut V {
        let index = bucket_index(table.level, table.num_buckets, sip_hash_from_value(&key));
        let bucket = table_with_length::borrow_mut(&mut table.buckets, index);
        let i = 0;
        let len = vector::length(bucket);
        while (i < len) {
            let entry = vector::borrow_mut(bucket, i);
            if (&entry.key == &key) {
                return &mut entry.value
            };
            i = i + 1;
        };
        abort error::invalid_argument(ENOT_FOUND)
    }
}
```


<a id="0x1_smart_table_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which `key` maps to.
Insert the pair (`key`, `default`) first if there is no entry for `key`.


```move
module 0x1::smart_table {
    public fun borrow_mut_with_default<K: copy, drop, V: drop>(table: &mut smart_table::SmartTable<K, V>, key: K, default: V): &mut V
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_mut_with_default<K: copy + drop, V: drop>(
        table: &mut SmartTable<K, V>,
        key: K,
        default: V
    ): &mut V {
        if (!contains(table, copy key)) {
            add(table, copy key, default)
        };
        borrow_mut(table, key)
    }
}
```


<a id="0x1_smart_table_contains"></a>

## Function `contains`

Returns true iff `table` contains an entry for `key`.


```move
module 0x1::smart_table {
    public fun contains<K: drop, V>(table: &smart_table::SmartTable<K, V>, key: K): bool
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun contains<K: drop, V>(table: &SmartTable<K, V>, key: K): bool {
        let hash = sip_hash_from_value(&key);
        let index = bucket_index(table.level, table.num_buckets, hash);
        let bucket = table_with_length::borrow(&table.buckets, index);
        vector::any(bucket, | entry | {
            let e: &Entry<K, V> = entry;
            e.hash == hash && &e.key == &key
        })
    }
}
```


<a id="0x1_smart_table_remove"></a>

## Function `remove`

Remove from `table` and return the value which `key` maps to.
Aborts if there is no entry for `key`.


```move
module 0x1::smart_table {
    public fun remove<K: copy, drop, V>(table: &mut smart_table::SmartTable<K, V>, key: K): V
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun remove<K: copy + drop, V>(table: &mut SmartTable<K, V>, key: K): V {
        let index = bucket_index(table.level, table.num_buckets, sip_hash_from_value(&key));
        let bucket = table_with_length::borrow_mut(&mut table.buckets, index);
        let i = 0;
        let len = vector::length(bucket);
        while (i < len) {
            let entry = vector::borrow(bucket, i);
            if (&entry.key == &key) {
                let Entry { hash: _, key: _, value } = vector::swap_remove(bucket, i);
                table.size = table.size - 1;
                return value
            };
            i = i + 1;
        };
        abort error::invalid_argument(ENOT_FOUND)
    }
}
```


<a id="0x1_smart_table_upsert"></a>

## Function `upsert`

Insert the pair (`key`, `value`) if there is no entry for `key`.
update the value of the entry for `key` to `value` otherwise


```move
module 0x1::smart_table {
    public fun upsert<K: copy, drop, V: drop>(table: &mut smart_table::SmartTable<K, V>, key: K, value: V)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun upsert<K: copy + drop, V: drop>(table: &mut SmartTable<K, V>, key: K, value: V) {
        if (!contains(table, copy key)) {
            add(table, copy key, value)
        } else {
            let ref = borrow_mut(table, key);
            *ref = value;
        };
    }
}
```


<a id="0x1_smart_table_length"></a>

## Function `length`

Returns the length of the table, i.e. the number of entries.


```move
module 0x1::smart_table {
    public fun length<K, V>(table: &smart_table::SmartTable<K, V>): u64
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun length<K, V>(table: &SmartTable<K, V>): u64 {
        table.size
    }
}
```


<a id="0x1_smart_table_load_factor"></a>

## Function `load_factor`

Return the load factor of the hashtable.


```move
module 0x1::smart_table {
    public fun load_factor<K, V>(table: &smart_table::SmartTable<K, V>): u64
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun load_factor<K, V>(table: &SmartTable<K, V>): u64 {
        table.size * 100 / table.num_buckets / table.target_bucket_size
    }
}
```


<a id="0x1_smart_table_update_split_load_threshold"></a>

## Function `update_split_load_threshold`

Update `split_load_threshold`.


```move
module 0x1::smart_table {
    public fun update_split_load_threshold<K, V>(table: &mut smart_table::SmartTable<K, V>, split_load_threshold: u8)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun update_split_load_threshold<K, V>(table: &mut SmartTable<K, V>, split_load_threshold: u8) {
        assert!(
            split_load_threshold <= 100 && split_load_threshold > 0,
            error::invalid_argument(EINVALID_LOAD_THRESHOLD_PERCENT)
        );
        table.split_load_threshold = split_load_threshold;
    }
}
```


<a id="0x1_smart_table_update_target_bucket_size"></a>

## Function `update_target_bucket_size`

Update `target_bucket_size`.


```move
module 0x1::smart_table {
    public fun update_target_bucket_size<K, V>(table: &mut smart_table::SmartTable<K, V>, target_bucket_size: u64)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun update_target_bucket_size<K, V>(table: &mut SmartTable<K, V>, target_bucket_size: u64) {
        assert!(target_bucket_size > 0, error::invalid_argument(EINVALID_TARGET_BUCKET_SIZE));
        table.target_bucket_size = target_bucket_size;
    }
}
```


<a id="0x1_smart_table_for_each_ref"></a>

## Function `for_each_ref`

Apply the function to a reference of each key&#45;value pair in the table.


```move
module 0x1::smart_table {
    public fun for_each_ref<K, V>(table: &smart_table::SmartTable<K, V>, f: |(&K, &V)|)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public inline fun for_each_ref<K, V>(table: &SmartTable<K, V>, f: |&K, &V|) {
        let i = 0;
        while (i < aptos_std::smart_table::num_buckets(table)) {
            vector::for_each_ref(
                aptos_std::table_with_length::borrow(aptos_std::smart_table::borrow_buckets(table), i),
                |elem| {
                    let (key, value) = aptos_std::smart_table::borrow_kv(elem);
                    f(key, value)
                }
            );
            i = i + 1;
        }
    }
}
```


<a id="0x1_smart_table_for_each_mut"></a>

## Function `for_each_mut`

Apply the function to a mutable reference of each key&#45;value pair in the table.


```move
module 0x1::smart_table {
    public fun for_each_mut<K, V>(table: &mut smart_table::SmartTable<K, V>, f: |(&K, &mut V)|)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public inline fun for_each_mut<K, V>(table: &mut SmartTable<K, V>, f: |&K, &mut V|) {
        let i = 0;
        while (i < aptos_std::smart_table::num_buckets(table)) {
            vector::for_each_mut(
                table_with_length::borrow_mut(aptos_std::smart_table::borrow_buckets_mut(table), i),
                |elem| {
                    let (key, value) = aptos_std::smart_table::borrow_kv_mut(elem);
                    f(key, value)
                }
            );
            i = i + 1;
        };
    }
}
```


<a id="0x1_smart_table_map_ref"></a>

## Function `map_ref`

Map the function over the references of key&#45;value pairs in the table without modifying it.


```move
module 0x1::smart_table {
    public fun map_ref<K: copy, drop, store, V1, V2: store>(table: &smart_table::SmartTable<K, V1>, f: |&V1|V2): smart_table::SmartTable<K, V2>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public inline fun map_ref<K: copy + drop + store, V1, V2: store>(
        table: &SmartTable<K, V1>,
        f: |&V1|V2
    ): SmartTable<K, V2> {
        let new_table = new<K, V2>();
        for_each_ref(table, |key, value| add(&mut new_table, *key, f(value)));
        new_table
    }
}
```


<a id="0x1_smart_table_any"></a>

## Function `any`

Return true if any key&#45;value pair in the table satisfies the predicate.


```move
module 0x1::smart_table {
    public fun any<K, V>(table: &smart_table::SmartTable<K, V>, p: |(&K, &V)|bool): bool
}
```


##### Implementation


```move
module 0x1::smart_table {
    public inline fun any<K, V>(
        table: &SmartTable<K, V>,
        p: |&K, &V|bool
    ): bool {
        let found = false;
        let i = 0;
        while (i < aptos_std::smart_table::num_buckets(table)) {
            found = vector::any(table_with_length::borrow(aptos_std::smart_table::borrow_buckets(table), i), |elem| {
                let (key, value) = aptos_std::smart_table::borrow_kv(elem);
                p(key, value)
            });
            if (found) break;
            i = i + 1;
        };
        found
    }
}
```


<a id="0x1_smart_table_borrow_kv"></a>

## Function `borrow_kv`



```move
module 0x1::smart_table {
    public fun borrow_kv<K, V>(e: &smart_table::Entry<K, V>): (&K, &V)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_kv<K, V>(e: &Entry<K, V>): (&K, &V) {
        (&e.key, &e.value)
    }
}
```


<a id="0x1_smart_table_borrow_kv_mut"></a>

## Function `borrow_kv_mut`



```move
module 0x1::smart_table {
    public fun borrow_kv_mut<K, V>(e: &mut smart_table::Entry<K, V>): (&mut K, &mut V)
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_kv_mut<K, V>(e: &mut Entry<K, V>): (&mut K, &mut V) {
        (&mut e.key, &mut e.value)
    }
}
```


<a id="0x1_smart_table_num_buckets"></a>

## Function `num_buckets`



```move
module 0x1::smart_table {
    public fun num_buckets<K, V>(table: &smart_table::SmartTable<K, V>): u64
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun num_buckets<K, V>(table: &SmartTable<K, V>): u64 {
        table.num_buckets
    }
}
```


<a id="0x1_smart_table_borrow_buckets"></a>

## Function `borrow_buckets`



```move
module 0x1::smart_table {
    public fun borrow_buckets<K, V>(table: &smart_table::SmartTable<K, V>): &table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_buckets<K, V>(table: &SmartTable<K, V>): &TableWithLength<u64, vector<Entry<K, V>>> {
        &table.buckets
    }
}
```


<a id="0x1_smart_table_borrow_buckets_mut"></a>

## Function `borrow_buckets_mut`



```move
module 0x1::smart_table {
    public fun borrow_buckets_mut<K, V>(table: &mut smart_table::SmartTable<K, V>): &mut table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>
}
```


##### Implementation


```move
module 0x1::smart_table {
    public fun borrow_buckets_mut<K, V>(table: &mut SmartTable<K, V>): &mut TableWithLength<u64, vector<Entry<K, V>>> {
        &mut table.buckets
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_SmartTable"></a>

### Struct `SmartTable`


```move
module 0x1::smart_table {
    struct SmartTable<K, V> has store
}
```


<dl>
<dt>
`buckets: table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>`
</dt>
<dd>

</dd>
<dt>
`num_buckets: u64`
</dt>
<dd>

</dd>
<dt>
`level: u8`
</dt>
<dd>

</dd>
<dt>
`size: u64`
</dt>
<dd>

</dd>
<dt>
`split_load_threshold: u8`
</dt>
<dd>

</dd>
<dt>
`target_bucket_size: u64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::smart_table {
    pragma intrinsic = map,
        map_new = new,
        map_destroy_empty = destroy_empty,
        map_len = length,
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


<a id="@Specification_1_new_with_config"></a>

### Function `new_with_config`


```move
module 0x1::smart_table {
    public fun new_with_config<K: copy, drop, store, V: store>(num_initial_buckets: u64, split_load_threshold: u8, target_bucket_size: u64): smart_table::SmartTable<K, V>
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_destroy"></a>

### Function `destroy`


```move
module 0x1::smart_table {
    public fun destroy<K: drop, V: drop>(table: smart_table::SmartTable<K, V>)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_clear"></a>

### Function `clear`


```move
module 0x1::smart_table {
    public fun clear<K: drop, V: drop>(table: &mut smart_table::SmartTable<K, V>)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_add_all"></a>

### Function `add_all`


```move
module 0x1::smart_table {
    public fun add_all<K, V>(table: &mut smart_table::SmartTable<K, V>, keys: vector<K>, values: vector<V>)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_to_simple_map"></a>

### Function `to_simple_map`


```move
module 0x1::smart_table {
    public fun to_simple_map<K: copy, drop, store, V: copy, store>(table: &smart_table::SmartTable<K, V>): simple_map::SimpleMap<K, V>
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_keys"></a>

### Function `keys`


```move
module 0x1::smart_table {
    public fun keys<K: copy, drop, store, V: copy, store>(table_ref: &smart_table::SmartTable<K, V>): vector<K>
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_keys_paginated"></a>

### Function `keys_paginated`


```move
module 0x1::smart_table {
    public fun keys_paginated<K: copy, drop, store, V: copy, store>(table_ref: &smart_table::SmartTable<K, V>, starting_bucket_index: u64, starting_vector_index: u64, num_keys_to_get: u64): (vector<K>, option::Option<u64>, option::Option<u64>)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_split_one_bucket"></a>

### Function `split_one_bucket`


```move
module 0x1::smart_table {
    fun split_one_bucket<K, V>(table: &mut smart_table::SmartTable<K, V>)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_bucket_index"></a>

### Function `bucket_index`


```move
module 0x1::smart_table {
    fun bucket_index(level: u8, num_buckets: u64, hash: u64): u64
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_borrow_with_default"></a>

### Function `borrow_with_default`


```move
module 0x1::smart_table {
    public fun borrow_with_default<K: copy, drop, V>(table: &smart_table::SmartTable<K, V>, key: K, default: &V): &V
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_load_factor"></a>

### Function `load_factor`


```move
module 0x1::smart_table {
    public fun load_factor<K, V>(table: &smart_table::SmartTable<K, V>): u64
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_update_split_load_threshold"></a>

### Function `update_split_load_threshold`


```move
module 0x1::smart_table {
    public fun update_split_load_threshold<K, V>(table: &mut smart_table::SmartTable<K, V>, split_load_threshold: u8)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_update_target_bucket_size"></a>

### Function `update_target_bucket_size`


```move
module 0x1::smart_table {
    public fun update_target_bucket_size<K, V>(table: &mut smart_table::SmartTable<K, V>, target_bucket_size: u64)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_borrow_kv"></a>

### Function `borrow_kv`


```move
module 0x1::smart_table {
    public fun borrow_kv<K, V>(e: &smart_table::Entry<K, V>): (&K, &V)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_borrow_kv_mut"></a>

### Function `borrow_kv_mut`


```move
module 0x1::smart_table {
    public fun borrow_kv_mut<K, V>(e: &mut smart_table::Entry<K, V>): (&mut K, &mut V)
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_num_buckets"></a>

### Function `num_buckets`


```move
module 0x1::smart_table {
    public fun num_buckets<K, V>(table: &smart_table::SmartTable<K, V>): u64
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_borrow_buckets"></a>

### Function `borrow_buckets`


```move
module 0x1::smart_table {
    public fun borrow_buckets<K, V>(table: &smart_table::SmartTable<K, V>): &table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```


<a id="@Specification_1_borrow_buckets_mut"></a>

### Function `borrow_buckets_mut`


```move
module 0x1::smart_table {
    public fun borrow_buckets_mut<K, V>(table: &mut smart_table::SmartTable<K, V>): &mut table_with_length::TableWithLength<u64, vector<smart_table::Entry<K, V>>>
}
```



```move
module 0x1::smart_table {
    pragma verify = false;
}
```



<a id="0x1_smart_table_spec_len"></a>


```move
module 0x1::smart_table {
    native fun spec_len<K, V>(t: SmartTable<K, V>): num;
}
```



<a id="0x1_smart_table_spec_contains"></a>


```move
module 0x1::smart_table {
    native fun spec_contains<K, V>(t: SmartTable<K, V>, k: K): bool;
}
```



<a id="0x1_smart_table_spec_set"></a>


```move
module 0x1::smart_table {
    native fun spec_set<K, V>(t: SmartTable<K, V>, k: K, v: V): SmartTable<K, V>;
}
```



<a id="0x1_smart_table_spec_remove"></a>


```move
module 0x1::smart_table {
    native fun spec_remove<K, V>(t: SmartTable<K, V>, k: K): SmartTable<K, V>;
}
```



<a id="0x1_smart_table_spec_get"></a>


```move
module 0x1::smart_table {
    native fun spec_get<K, V>(t: SmartTable<K, V>, k: K): V;
}
```
