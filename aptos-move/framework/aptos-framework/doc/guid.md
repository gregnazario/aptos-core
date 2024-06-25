
<a id="0x1_guid"></a>

# Module `0x1::guid`

A module for generating globally unique identifiers


-  [Struct `GUID`](#0x1_guid_GUID)
-  [Struct `ID`](#0x1_guid_ID)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x1_guid_create)
-  [Function `create_id`](#0x1_guid_create_id)
-  [Function `id`](#0x1_guid_id)
-  [Function `creator_address`](#0x1_guid_creator_address)
-  [Function `id_creator_address`](#0x1_guid_id_creator_address)
-  [Function `creation_num`](#0x1_guid_creation_num)
-  [Function `id_creation_num`](#0x1_guid_id_creation_num)
-  [Function `eq_id`](#0x1_guid_eq_id)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `create`](#@Specification_1_create)
    -  [Function `create_id`](#@Specification_1_create_id)
    -  [Function `id`](#@Specification_1_id)
    -  [Function `creator_address`](#@Specification_1_creator_address)
    -  [Function `id_creator_address`](#@Specification_1_id_creator_address)
    -  [Function `creation_num`](#@Specification_1_creation_num)
    -  [Function `id_creation_num`](#@Specification_1_id_creation_num)
    -  [Function `eq_id`](#@Specification_1_eq_id)


```move
module 0x1::guid {
}
```


<a id="0x1_guid_GUID"></a>

## Struct `GUID`

A globally unique identifier derived from the sender&apos;s address and a counter


```move
module 0x1::guid {
    struct GUID has drop, store
}
```


##### Fields


<dl>
<dt>
`id: guid::ID`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_guid_ID"></a>

## Struct `ID`

A non&#45;privileged identifier that can be freely created by anyone. Useful for looking up GUID&apos;s.


```move
module 0x1::guid {
    struct ID has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`creation_num: u64`
</dt>
<dd>
 If creation_num is `i`, this is the `i+1`th GUID created by `addr`
</dd>
<dt>
`addr: address`
</dt>
<dd>
 Address that created the GUID
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_guid_EGUID_GENERATOR_NOT_PUBLISHED"></a>

GUID generator must be published ahead of first usage of `create_with_capability` function.


```move
module 0x1::guid {
    const EGUID_GENERATOR_NOT_PUBLISHED: u64 = 0;
}
```


<a id="0x1_guid_create"></a>

## Function `create`

Create and return a new GUID from a trusted module.


```move
module 0x1::guid {
    public(friend) fun create(addr: address, creation_num_ref: &mut u64): guid::GUID
}
```


##### Implementation


```move
module 0x1::guid {
    public(friend) fun create(addr: address, creation_num_ref: &mut u64): GUID {
        let creation_num = *creation_num_ref;
        *creation_num_ref = creation_num + 1;
        GUID {
            id: ID {
                creation_num,
                addr,
            }
        }
    }
}
```


<a id="0x1_guid_create_id"></a>

## Function `create_id`

Create a non&#45;privileged id from `addr` and `creation_num`


```move
module 0x1::guid {
    public fun create_id(addr: address, creation_num: u64): guid::ID
}
```


##### Implementation


```move
module 0x1::guid {
    public fun create_id(addr: address, creation_num: u64): ID {
        ID { creation_num, addr }
    }
}
```


<a id="0x1_guid_id"></a>

## Function `id`

Get the non&#45;privileged ID associated with a GUID


```move
module 0x1::guid {
    public fun id(guid: &guid::GUID): guid::ID
}
```


##### Implementation


```move
module 0x1::guid {
    public fun id(guid: &GUID): ID {
        guid.id
    }
}
```


<a id="0x1_guid_creator_address"></a>

## Function `creator_address`

Return the account address that created the GUID


```move
module 0x1::guid {
    public fun creator_address(guid: &guid::GUID): address
}
```


##### Implementation


```move
module 0x1::guid {
    public fun creator_address(guid: &GUID): address {
        guid.id.addr
    }
}
```


<a id="0x1_guid_id_creator_address"></a>

## Function `id_creator_address`

Return the account address that created the guid::ID


```move
module 0x1::guid {
    public fun id_creator_address(id: &guid::ID): address
}
```


##### Implementation


```move
module 0x1::guid {
    public fun id_creator_address(id: &ID): address {
        id.addr
    }
}
```


<a id="0x1_guid_creation_num"></a>

## Function `creation_num`

Return the creation number associated with the GUID


```move
module 0x1::guid {
    public fun creation_num(guid: &guid::GUID): u64
}
```


##### Implementation


```move
module 0x1::guid {
    public fun creation_num(guid: &GUID): u64 {
        guid.id.creation_num
    }
}
```


<a id="0x1_guid_id_creation_num"></a>

## Function `id_creation_num`

Return the creation number associated with the guid::ID


```move
module 0x1::guid {
    public fun id_creation_num(id: &guid::ID): u64
}
```


##### Implementation


```move
module 0x1::guid {
    public fun id_creation_num(id: &ID): u64 {
        id.creation_num
    }
}
```


<a id="0x1_guid_eq_id"></a>

## Function `eq_id`

Return true if the GUID&apos;s ID is `id`


```move
module 0x1::guid {
    public fun eq_id(guid: &guid::GUID, id: &guid::ID): bool
}
```


##### Implementation


```move
module 0x1::guid {
    public fun eq_id(guid: &GUID, id: &ID): bool {
        &guid.id == id
    }
}
```


<a id="@Specification_1"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>The creation of GUID constructs a unique GUID by combining an address with an incremented creation number.</td>
<td>Low</td>
<td>The create function generates a new GUID by combining an address with an incremented creation number, effectively creating a unique identifier.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;1](create).</td>
</tr>

<tr>
<td>2</td>
<td>The operations on GUID and ID, such as construction, field access, and equality comparison, should not abort.</td>
<td>Low</td>
<td>The following functions will never abort: (1) create_id, (2) id, (3) creator_address, (4) id_creator_address, (5) creation_num, (6) id_creation_num, and (7) eq_id.</td>
<td>Verified via [#high&#45;level&#45;req&#45;2.1](create_id), [#high&#45;level&#45;req&#45;2.2](id), [#high&#45;level&#45;req&#45;2.3](creator_address), [#high&#45;level&#45;req&#45;2.4](id_creator_address), [#high&#45;level&#45;req&#45;2.5](creation_num), [#high&#45;level&#45;req&#45;2.6](id_creation_num), and [#high&#45;level&#45;req&#45;2.7](eq_id).</td>
</tr>

<tr>
<td>3</td>
<td>The creation number should increment by 1 with each new creation.</td>
<td>Low</td>
<td>An account can only own up to MAX_U64 resources. Not incrementing the guid_creation_num constantly could lead to shrinking the space for new resources.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;3](create).</td>
</tr>

<tr>
<td>4</td>
<td>The creation number and address of an ID / GUID must be immutable.</td>
<td>Medium</td>
<td>The addr and creation_num values are meant to be constant and never updated as they are unique and used for identification.</td>
<td>Audited: This is enforced through missing functionality to update the creation_num or addr.</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::guid {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_create"></a>

### Function `create`


```move
module 0x1::guid {
    public(friend) fun create(addr: address, creation_num_ref: &mut u64): guid::GUID
}
```



```move
module 0x1::guid {
    aborts_if creation_num_ref + 1 > MAX_U64;
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures result.id.creation_num == old(creation_num_ref);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures creation_num_ref == old(creation_num_ref) + 1;
}
```


<a id="@Specification_1_create_id"></a>

### Function `create_id`


```move
module 0x1::guid {
    public fun create_id(addr: address, creation_num: u64): guid::ID
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.1
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_id"></a>

### Function `id`


```move
module 0x1::guid {
    public fun id(guid: &guid::GUID): guid::ID
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_creator_address"></a>

### Function `creator_address`


```move
module 0x1::guid {
    public fun creator_address(guid: &guid::GUID): address
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.3
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_id_creator_address"></a>

### Function `id_creator_address`


```move
module 0x1::guid {
    public fun id_creator_address(id: &guid::ID): address
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.4
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_creation_num"></a>

### Function `creation_num`


```move
module 0x1::guid {
    public fun creation_num(guid: &guid::GUID): u64
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.5
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_id_creation_num"></a>

### Function `id_creation_num`


```move
module 0x1::guid {
    public fun id_creation_num(id: &guid::ID): u64
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.6
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_eq_id"></a>

### Function `eq_id`


```move
module 0x1::guid {
    public fun eq_id(guid: &guid::GUID, id: &guid::ID): bool
}
```



```move
module 0x1::guid {
// This enforces ### high&#45;level&#45;req&#45;2.7
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```
