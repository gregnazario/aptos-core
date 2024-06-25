
<a id="0x1_object"></a>

# Module `0x1::object`

This defines the Move object model with the following properties:
&#45; Simplified storage interface that supports a heterogeneous collection of resources to be
stored together. This enables data types to share a common core data layer (e.g., tokens),
while having richer extensions (e.g., concert ticket, sword).
&#45; Globally accessible data and ownership model that enables creators and developers to dictate
the application and lifetime of data.
&#45; Extensible programming model that supports individualization of user applications that
leverage the core framework including tokens.
&#45; Support emitting events directly, thus improving discoverability of events associated with
objects.
&#45; Considerate of the underlying system by leveraging resource groups for gas efficiency,
avoiding costly deserialization and serialization costs, and supporting deletability.

TODO:
&#42; There is no means to borrow an object or a reference to an object. We are exploring how to
make it so that a reference to a global object can be returned from a function.


-  [Resource `ObjectCore`](#0x1_object_ObjectCore)
-  [Resource `TombStone`](#0x1_object_TombStone)
-  [Resource `Untransferable`](#0x1_object_Untransferable)
-  [Struct `ObjectGroup`](#0x1_object_ObjectGroup)
-  [Struct `Object`](#0x1_object_Object)
-  [Struct `ConstructorRef`](#0x1_object_ConstructorRef)
-  [Struct `DeleteRef`](#0x1_object_DeleteRef)
-  [Struct `ExtendRef`](#0x1_object_ExtendRef)
-  [Struct `TransferRef`](#0x1_object_TransferRef)
-  [Struct `LinearTransferRef`](#0x1_object_LinearTransferRef)
-  [Struct `DeriveRef`](#0x1_object_DeriveRef)
-  [Struct `TransferEvent`](#0x1_object_TransferEvent)
-  [Struct `Transfer`](#0x1_object_Transfer)
-  [Constants](#@Constants_0)
-  [Function `is_untransferable`](#0x1_object_is_untransferable)
-  [Function `is_burnt`](#0x1_object_is_burnt)
-  [Function `address_to_object`](#0x1_object_address_to_object)
-  [Function `is_object`](#0x1_object_is_object)
-  [Function `object_exists`](#0x1_object_object_exists)
-  [Function `create_object_address`](#0x1_object_create_object_address)
-  [Function `create_user_derived_object_address_impl`](#0x1_object_create_user_derived_object_address_impl)
-  [Function `create_user_derived_object_address`](#0x1_object_create_user_derived_object_address)
-  [Function `create_guid_object_address`](#0x1_object_create_guid_object_address)
-  [Function `exists_at`](#0x1_object_exists_at)
-  [Function `object_address`](#0x1_object_object_address)
-  [Function `convert`](#0x1_object_convert)
-  [Function `create_named_object`](#0x1_object_create_named_object)
-  [Function `create_user_derived_object`](#0x1_object_create_user_derived_object)
-  [Function `create_object`](#0x1_object_create_object)
-  [Function `create_sticky_object`](#0x1_object_create_sticky_object)
-  [Function `create_sticky_object_at_address`](#0x1_object_create_sticky_object_at_address)
-  [Function `create_object_from_account`](#0x1_object_create_object_from_account)
-  [Function `create_object_from_object`](#0x1_object_create_object_from_object)
-  [Function `create_object_from_guid`](#0x1_object_create_object_from_guid)
-  [Function `create_object_internal`](#0x1_object_create_object_internal)
-  [Function `generate_delete_ref`](#0x1_object_generate_delete_ref)
-  [Function `generate_extend_ref`](#0x1_object_generate_extend_ref)
-  [Function `generate_transfer_ref`](#0x1_object_generate_transfer_ref)
-  [Function `generate_derive_ref`](#0x1_object_generate_derive_ref)
-  [Function `generate_signer`](#0x1_object_generate_signer)
-  [Function `address_from_constructor_ref`](#0x1_object_address_from_constructor_ref)
-  [Function `object_from_constructor_ref`](#0x1_object_object_from_constructor_ref)
-  [Function `can_generate_delete_ref`](#0x1_object_can_generate_delete_ref)
-  [Function `create_guid`](#0x1_object_create_guid)
-  [Function `new_event_handle`](#0x1_object_new_event_handle)
-  [Function `address_from_delete_ref`](#0x1_object_address_from_delete_ref)
-  [Function `object_from_delete_ref`](#0x1_object_object_from_delete_ref)
-  [Function `delete`](#0x1_object_delete)
-  [Function `generate_signer_for_extending`](#0x1_object_generate_signer_for_extending)
-  [Function `address_from_extend_ref`](#0x1_object_address_from_extend_ref)
-  [Function `disable_ungated_transfer`](#0x1_object_disable_ungated_transfer)
-  [Function `set_untransferable`](#0x1_object_set_untransferable)
-  [Function `enable_ungated_transfer`](#0x1_object_enable_ungated_transfer)
-  [Function `generate_linear_transfer_ref`](#0x1_object_generate_linear_transfer_ref)
-  [Function `transfer_with_ref`](#0x1_object_transfer_with_ref)
-  [Function `transfer_call`](#0x1_object_transfer_call)
-  [Function `transfer`](#0x1_object_transfer)
-  [Function `transfer_raw`](#0x1_object_transfer_raw)
-  [Function `transfer_raw_inner`](#0x1_object_transfer_raw_inner)
-  [Function `transfer_to_object`](#0x1_object_transfer_to_object)
-  [Function `verify_ungated_and_descendant`](#0x1_object_verify_ungated_and_descendant)
-  [Function `burn`](#0x1_object_burn)
-  [Function `unburn`](#0x1_object_unburn)
-  [Function `ungated_transfer_allowed`](#0x1_object_ungated_transfer_allowed)
-  [Function `owner`](#0x1_object_owner)
-  [Function `is_owner`](#0x1_object_is_owner)
-  [Function `owns`](#0x1_object_owns)
-  [Function `root_owner`](#0x1_object_root_owner)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `address_to_object`](#@Specification_1_address_to_object)
    -  [Function `create_object_address`](#@Specification_1_create_object_address)
    -  [Function `create_user_derived_object_address_impl`](#@Specification_1_create_user_derived_object_address_impl)
    -  [Function `create_user_derived_object_address`](#@Specification_1_create_user_derived_object_address)
    -  [Function `create_guid_object_address`](#@Specification_1_create_guid_object_address)
    -  [Function `exists_at`](#@Specification_1_exists_at)
    -  [Function `object_address`](#@Specification_1_object_address)
    -  [Function `convert`](#@Specification_1_convert)
    -  [Function `create_named_object`](#@Specification_1_create_named_object)
    -  [Function `create_user_derived_object`](#@Specification_1_create_user_derived_object)
    -  [Function `create_object`](#@Specification_1_create_object)
    -  [Function `create_sticky_object`](#@Specification_1_create_sticky_object)
    -  [Function `create_sticky_object_at_address`](#@Specification_1_create_sticky_object_at_address)
    -  [Function `create_object_from_account`](#@Specification_1_create_object_from_account)
    -  [Function `create_object_from_object`](#@Specification_1_create_object_from_object)
    -  [Function `create_object_from_guid`](#@Specification_1_create_object_from_guid)
    -  [Function `create_object_internal`](#@Specification_1_create_object_internal)
    -  [Function `generate_delete_ref`](#@Specification_1_generate_delete_ref)
    -  [Function `generate_transfer_ref`](#@Specification_1_generate_transfer_ref)
    -  [Function `object_from_constructor_ref`](#@Specification_1_object_from_constructor_ref)
    -  [Function `create_guid`](#@Specification_1_create_guid)
    -  [Function `new_event_handle`](#@Specification_1_new_event_handle)
    -  [Function `object_from_delete_ref`](#@Specification_1_object_from_delete_ref)
    -  [Function `delete`](#@Specification_1_delete)
    -  [Function `disable_ungated_transfer`](#@Specification_1_disable_ungated_transfer)
    -  [Function `set_untransferable`](#@Specification_1_set_untransferable)
    -  [Function `enable_ungated_transfer`](#@Specification_1_enable_ungated_transfer)
    -  [Function `generate_linear_transfer_ref`](#@Specification_1_generate_linear_transfer_ref)
    -  [Function `transfer_with_ref`](#@Specification_1_transfer_with_ref)
    -  [Function `transfer_call`](#@Specification_1_transfer_call)
    -  [Function `transfer`](#@Specification_1_transfer)
    -  [Function `transfer_raw`](#@Specification_1_transfer_raw)
    -  [Function `transfer_to_object`](#@Specification_1_transfer_to_object)
    -  [Function `verify_ungated_and_descendant`](#@Specification_1_verify_ungated_and_descendant)
    -  [Function `burn`](#@Specification_1_burn)
    -  [Function `unburn`](#@Specification_1_unburn)
    -  [Function `ungated_transfer_allowed`](#@Specification_1_ungated_transfer_allowed)
    -  [Function `owner`](#@Specification_1_owner)
    -  [Function `is_owner`](#@Specification_1_is_owner)
    -  [Function `owns`](#@Specification_1_owns)
    -  [Function `root_owner`](#@Specification_1_root_owner)


```move
module 0x1::object {
    use 0x1::account;
    use 0x1::bcs;
    use 0x1::create_signer;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::from_bcs;
    use 0x1::guid;
    use 0x1::hash;
    use 0x1::signer;
    use 0x1::transaction_context;
    use 0x1::vector;
}
```


<a id="0x1_object_ObjectCore"></a>

## Resource `ObjectCore`

The core of the object model that defines ownership, transferability, and events.


```move
module 0x1::object {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct ObjectCore has key
}
```


##### Fields


<dl>
<dt>
`guid_creation_num: u64`
</dt>
<dd>
 Used by guid to guarantee globally unique objects and create event streams
</dd>
<dt>
`owner: address`
</dt>
<dd>
 The address (object or account) that owns this object
</dd>
<dt>
`allow_ungated_transfer: bool`
</dt>
<dd>
 Object transferring is a common operation, this allows for disabling and enabling
 transfers bypassing the use of a TransferRef.
</dd>
<dt>
`transfer_events: event::EventHandle<object::TransferEvent>`
</dt>
<dd>
 Emitted events upon transferring of ownership.
</dd>
</dl>


<a id="0x1_object_TombStone"></a>

## Resource `TombStone`

This is added to objects that are burnt (ownership transferred to BURN_ADDRESS).


```move
module 0x1::object {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct TombStone has key
}
```


##### Fields


<dl>
<dt>
`original_owner: address`
</dt>
<dd>
 Track the previous owner before the object is burnt so they can reclaim later if so desired.
</dd>
</dl>


<a id="0x1_object_Untransferable"></a>

## Resource `Untransferable`

The existence of this renders all `TransferRef`s irrelevant. The object cannot be moved.


```move
module 0x1::object {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct Untransferable has key
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_object_ObjectGroup"></a>

## Struct `ObjectGroup`

A shared resource group for storing object resources together in storage.


```move
module 0x1::object {
    #[resource_group(#[scope = global])]
    struct ObjectGroup
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_object_Object"></a>

## Struct `Object`

A pointer to an object &#45;&#45; these can only provide guarantees based upon the underlying data
type, that is the validity of T existing at an address is something that cannot be verified
by any other module than the module that defined T. Similarly, the module that defines T
can remove it from storage at any point in time.


```move
module 0x1::object {
    struct Object<T> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`inner: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_object_ConstructorRef"></a>

## Struct `ConstructorRef`

This is a one time ability given to the creator to configure the object as necessary


```move
module 0x1::object {
    struct ConstructorRef has drop
}
```


##### Fields


<dl>
<dt>
`self: address`
</dt>
<dd>

</dd>
<dt>
`can_delete: bool`
</dt>
<dd>
 True if the object can be deleted. Named objects are not deletable.
</dd>
</dl>


<a id="0x1_object_DeleteRef"></a>

## Struct `DeleteRef`

Used to remove an object from storage.


```move
module 0x1::object {
    struct DeleteRef has drop, store
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


<a id="0x1_object_ExtendRef"></a>

## Struct `ExtendRef`

Used to create events or move additional resources into object storage.


```move
module 0x1::object {
    struct ExtendRef has drop, store
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


<a id="0x1_object_TransferRef"></a>

## Struct `TransferRef`

Used to create LinearTransferRef, hence ownership transfer.


```move
module 0x1::object {
    struct TransferRef has drop, store
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


<a id="0x1_object_LinearTransferRef"></a>

## Struct `LinearTransferRef`

Used to perform transfers. This locks transferring ability to a single time use bound to
the current owner.


```move
module 0x1::object {
    struct LinearTransferRef has drop
}
```


##### Fields


<dl>
<dt>
`self: address`
</dt>
<dd>

</dd>
<dt>
`owner: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_object_DeriveRef"></a>

## Struct `DeriveRef`

Used to create derived objects from a given objects.


```move
module 0x1::object {
    struct DeriveRef has drop, store
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


<a id="0x1_object_TransferEvent"></a>

## Struct `TransferEvent`

Emitted whenever the object&apos;s owner field is changed.


```move
module 0x1::object {
    struct TransferEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`object: address`
</dt>
<dd>

</dd>
<dt>
`from: address`
</dt>
<dd>

</dd>
<dt>
`to: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_object_Transfer"></a>

## Struct `Transfer`

Emitted whenever the object&apos;s owner field is changed.


```move
module 0x1::object {
    #[event]
    struct Transfer has drop, store
}
```


##### Fields


<dl>
<dt>
`object: address`
</dt>
<dd>

</dd>
<dt>
`from: address`
</dt>
<dd>

</dd>
<dt>
`to: address`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_object_BURN_ADDRESS"></a>

Address where unwanted objects can be forcefully transferred to.


```move
module 0x1::object {
    const BURN_ADDRESS: address = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
}
```


<a id="0x1_object_DERIVE_AUID_ADDRESS_SCHEME"></a>

generate_unique_address uses this for domain separation within its native implementation


```move
module 0x1::object {
    const DERIVE_AUID_ADDRESS_SCHEME: u8 = 251;
}
```


<a id="0x1_object_ECANNOT_DELETE"></a>

The object does not allow for deletion


```move
module 0x1::object {
    const ECANNOT_DELETE: u64 = 5;
}
```


<a id="0x1_object_EMAXIMUM_NESTING"></a>

Exceeds maximum nesting for an object transfer.


```move
module 0x1::object {
    const EMAXIMUM_NESTING: u64 = 6;
}
```


<a id="0x1_object_ENOT_OBJECT_OWNER"></a>

The caller does not have ownership permissions


```move
module 0x1::object {
    const ENOT_OBJECT_OWNER: u64 = 4;
}
```


<a id="0x1_object_ENO_UNGATED_TRANSFERS"></a>

The object does not have ungated transfers enabled


```move
module 0x1::object {
    const ENO_UNGATED_TRANSFERS: u64 = 3;
}
```


<a id="0x1_object_EOBJECT_DOES_NOT_EXIST"></a>

An object does not exist at this address


```move
module 0x1::object {
    const EOBJECT_DOES_NOT_EXIST: u64 = 2;
}
```


<a id="0x1_object_EOBJECT_EXISTS"></a>

An object already exists at this address


```move
module 0x1::object {
    const EOBJECT_EXISTS: u64 = 1;
}
```


<a id="0x1_object_EOBJECT_NOT_BURNT"></a>

Cannot reclaim objects that weren&apos;t burnt.


```move
module 0x1::object {
    const EOBJECT_NOT_BURNT: u64 = 8;
}
```


<a id="0x1_object_EOBJECT_NOT_TRANSFERRABLE"></a>

Object is untransferable any operations that might result in a transfer are disallowed.


```move
module 0x1::object {
    const EOBJECT_NOT_TRANSFERRABLE: u64 = 9;
}
```


<a id="0x1_object_ERESOURCE_DOES_NOT_EXIST"></a>

The resource is not stored at the specified address.


```move
module 0x1::object {
    const ERESOURCE_DOES_NOT_EXIST: u64 = 7;
}
```


<a id="0x1_object_INIT_GUID_CREATION_NUM"></a>

Explicitly separate the GUID space between Object and Account to prevent accidental overlap.


```move
module 0x1::object {
    const INIT_GUID_CREATION_NUM: u64 = 1125899906842624;
}
```


<a id="0x1_object_MAXIMUM_OBJECT_NESTING"></a>

Maximum nesting from one object to another. That is objects can technically have infinte
nesting, but any checks such as transfer will only be evaluated this deep.


```move
module 0x1::object {
    const MAXIMUM_OBJECT_NESTING: u8 = 8;
}
```


<a id="0x1_object_OBJECT_DERIVED_SCHEME"></a>

Scheme identifier used to generate an object&apos;s address `obj_addr` as derived from another object.
The object&apos;s address is generated as:
```
obj_addr &#61; sha3_256(account addr &#124; derived from object&apos;s address &#124; 0xFC)
```

This 0xFC constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


```move
module 0x1::object {
    const OBJECT_DERIVED_SCHEME: u8 = 252;
}
```


<a id="0x1_object_OBJECT_FROM_GUID_ADDRESS_SCHEME"></a>

Scheme identifier used to generate an object&apos;s address `obj_addr` via a fresh GUID generated by the creator at
`source_addr`. The object&apos;s address is generated as:
```
obj_addr &#61; sha3_256(guid &#124; 0xFD)
```
where `guid = account::create_guid(create_signer(source_addr))`

This 0xFD constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


```move
module 0x1::object {
    const OBJECT_FROM_GUID_ADDRESS_SCHEME: u8 = 253;
}
```


<a id="0x1_object_OBJECT_FROM_SEED_ADDRESS_SCHEME"></a>

Scheme identifier used to generate an object&apos;s address `obj_addr` from the creator&apos;s `source_addr` and a `seed` as:
obj_addr &#61; sha3_256(source_addr &#124; seed &#124; 0xFE).

This 0xFE constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


```move
module 0x1::object {
    const OBJECT_FROM_SEED_ADDRESS_SCHEME: u8 = 254;
}
```


<a id="0x1_object_is_untransferable"></a>

## Function `is_untransferable`



```move
module 0x1::object {
    #[view]
    public fun is_untransferable<T: key>(object: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun is_untransferable<T: key>(object: Object<T>): bool {
        exists<Untransferable>(object.inner)
    }
}
```


<a id="0x1_object_is_burnt"></a>

## Function `is_burnt`



```move
module 0x1::object {
    #[view]
    public fun is_burnt<T: key>(object: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun is_burnt<T: key>(object: Object<T>): bool {
        exists<TombStone>(object.inner)
    }
}
```


<a id="0x1_object_address_to_object"></a>

## Function `address_to_object`

Produces an ObjectId from the given address. This is not verified.


```move
module 0x1::object {
    public fun address_to_object<T: key>(object: address): object::Object<T>
}
```


##### Implementation


```move
module 0x1::object {
    public fun address_to_object<T: key>(object: address): Object<T> {
        assert!(exists<ObjectCore>(object), error::not_found(EOBJECT_DOES_NOT_EXIST));
        assert!(exists_at<T>(object), error::not_found(ERESOURCE_DOES_NOT_EXIST));
        Object<T> { inner: object }
    }
}
```


<a id="0x1_object_is_object"></a>

## Function `is_object`

Returns true if there exists an object or the remnants of an object.


```move
module 0x1::object {
    public fun is_object(object: address): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun is_object(object: address): bool {
        exists<ObjectCore>(object)
    }
}
```


<a id="0x1_object_object_exists"></a>

## Function `object_exists`

Returns true if there exists an object with resource T.


```move
module 0x1::object {
    public fun object_exists<T: key>(object: address): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun object_exists<T: key>(object: address): bool {
        exists<ObjectCore>(object) && exists_at<T>(object)
    }
}
```


<a id="0x1_object_create_object_address"></a>

## Function `create_object_address`

Derives an object address from source material: sha3_256([creator address &#124; seed &#124; 0xFE]).


```move
module 0x1::object {
    public fun create_object_address(source: &address, seed: vector<u8>): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_object_address(source: &address, seed: vector<u8>): address {
        let bytes = bcs::to_bytes(source);
        vector::append(&mut bytes, seed);
        vector::push_back(&mut bytes, OBJECT_FROM_SEED_ADDRESS_SCHEME);
        from_bcs::to_address(hash::sha3_256(bytes))
    }
}
```


<a id="0x1_object_create_user_derived_object_address_impl"></a>

## Function `create_user_derived_object_address_impl`



```move
module 0x1::object {
    fun create_user_derived_object_address_impl(source: address, derive_from: address): address
}
```


##### Implementation


```move
module 0x1::object {
    native fun create_user_derived_object_address_impl(source: address, derive_from: address): address;
}
```


<a id="0x1_object_create_user_derived_object_address"></a>

## Function `create_user_derived_object_address`

Derives an object address from the source address and an object: sha3_256([source &#124; object addr &#124; 0xFC]).


```move
module 0x1::object {
    public fun create_user_derived_object_address(source: address, derive_from: address): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_user_derived_object_address(source: address, derive_from: address): address {
        if (std::features::object_native_derived_address_enabled()) {
            create_user_derived_object_address_impl(source, derive_from)
        } else {
            let bytes = bcs::to_bytes(&source);
            vector::append(&mut bytes, bcs::to_bytes(&derive_from));
            vector::push_back(&mut bytes, OBJECT_DERIVED_SCHEME);
            from_bcs::to_address(hash::sha3_256(bytes))
        }
    }
}
```


<a id="0x1_object_create_guid_object_address"></a>

## Function `create_guid_object_address`

Derives an object from an Account GUID.


```move
module 0x1::object {
    public fun create_guid_object_address(source: address, creation_num: u64): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_guid_object_address(source: address, creation_num: u64): address {
        let id = guid::create_id(source, creation_num);
        let bytes = bcs::to_bytes(&id);
        vector::push_back(&mut bytes, OBJECT_FROM_GUID_ADDRESS_SCHEME);
        from_bcs::to_address(hash::sha3_256(bytes))
    }
}
```


<a id="0x1_object_exists_at"></a>

## Function `exists_at`



```move
module 0x1::object {
    fun exists_at<T: key>(object: address): bool
}
```


##### Implementation


```move
module 0x1::object {
    native fun exists_at<T: key>(object: address): bool;
}
```


<a id="0x1_object_object_address"></a>

## Function `object_address`

Returns the address of within an ObjectId.


```move
module 0x1::object {
    public fun object_address<T: key>(object: &object::Object<T>): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun object_address<T: key>(object: &Object<T>): address {
        object.inner
    }
}
```


<a id="0x1_object_convert"></a>

## Function `convert`

Convert Object&lt;X&gt; to Object&lt;Y&gt;.


```move
module 0x1::object {
    public fun convert<X: key, Y: key>(object: object::Object<X>): object::Object<Y>
}
```


##### Implementation


```move
module 0x1::object {
    public fun convert<X: key, Y: key>(object: Object<X>): Object<Y> {
        address_to_object<Y>(object.inner)
    }
}
```


<a id="0x1_object_create_named_object"></a>

## Function `create_named_object`

Create a new named object and return the ConstructorRef. Named objects can be queried globally
by knowing the user generated seed used to create them. Named objects cannot be deleted.


```move
module 0x1::object {
    public fun create_named_object(creator: &signer, seed: vector<u8>): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_named_object(creator: &signer, seed: vector<u8>): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let obj_addr = create_object_address(&creator_address, seed);
        create_object_internal(creator_address, obj_addr, false)
    }
}
```


<a id="0x1_object_create_user_derived_object"></a>

## Function `create_user_derived_object`

Create a new object whose address is derived based on the creator account address and another object.
Derivde objects, similar to named objects, cannot be deleted.


```move
module 0x1::object {
    public(friend) fun create_user_derived_object(creator_address: address, derive_ref: &object::DeriveRef): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public(friend) fun create_user_derived_object(creator_address: address, derive_ref: &DeriveRef): ConstructorRef {
        let obj_addr = create_user_derived_object_address(creator_address, derive_ref.self);
        create_object_internal(creator_address, obj_addr, false)
    }
}
```


<a id="0x1_object_create_object"></a>

## Function `create_object`

Create a new object by generating a random unique address based on transaction hash.
The unique address is computed sha3_256([transaction hash &#124; auid counter &#124; 0xFB]).
The created object is deletable as we can guarantee the same unique address can
never be regenerated with future txs.


```move
module 0x1::object {
    public fun create_object(owner_address: address): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_object(owner_address: address): ConstructorRef {
        let unique_address = transaction_context::generate_auid_address();
        create_object_internal(owner_address, unique_address, true)
    }
}
```


<a id="0x1_object_create_sticky_object"></a>

## Function `create_sticky_object`

Same as `create_object` except the object to be created will be undeletable.


```move
module 0x1::object {
    public fun create_sticky_object(owner_address: address): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_sticky_object(owner_address: address): ConstructorRef {
        let unique_address = transaction_context::generate_auid_address();
        create_object_internal(owner_address, unique_address, false)
    }
}
```


<a id="0x1_object_create_sticky_object_at_address"></a>

## Function `create_sticky_object_at_address`

Create a sticky object at a specific address. Only used by aptos_framework::coin.


```move
module 0x1::object {
    public(friend) fun create_sticky_object_at_address(owner_address: address, object_address: address): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public(friend) fun create_sticky_object_at_address(
        owner_address: address,
        object_address: address,
    ): ConstructorRef {
        create_object_internal(owner_address, object_address, false)
    }
}
```


<a id="0x1_object_create_object_from_account"></a>

## Function `create_object_from_account`

Use `create_object` instead.
Create a new object from a GUID generated by an account.
As the GUID creation internally increments a counter, two transactions that executes
`create_object_from_account` function for the same creator run sequentially.
Therefore, using `create_object` method for creating objects is preferrable as it
doesn&apos;t have the same bottlenecks.


```move
module 0x1::object {
    #[deprecated]
    public fun create_object_from_account(creator: &signer): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_object_from_account(creator: &signer): ConstructorRef {
        let guid = account::create_guid(creator);
        create_object_from_guid(signer::address_of(creator), guid)
    }
}
```


<a id="0x1_object_create_object_from_object"></a>

## Function `create_object_from_object`

Use `create_object` instead.
Create a new object from a GUID generated by an object.
As the GUID creation internally increments a counter, two transactions that executes
`create_object_from_object` function for the same creator run sequentially.
Therefore, using `create_object` method for creating objects is preferrable as it
doesn&apos;t have the same bottlenecks.


```move
module 0x1::object {
    #[deprecated]
    public fun create_object_from_object(creator: &signer): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_object_from_object(creator: &signer): ConstructorRef acquires ObjectCore {
        let guid = create_guid(creator);
        create_object_from_guid(signer::address_of(creator), guid)
    }
}
```


<a id="0x1_object_create_object_from_guid"></a>

## Function `create_object_from_guid`



```move
module 0x1::object {
    fun create_object_from_guid(creator_address: address, guid: guid::GUID): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    fun create_object_from_guid(creator_address: address, guid: guid::GUID): ConstructorRef {
        let bytes = bcs::to_bytes(&guid);
        vector::push_back(&mut bytes, OBJECT_FROM_GUID_ADDRESS_SCHEME);
        let obj_addr = from_bcs::to_address(hash::sha3_256(bytes));
        create_object_internal(creator_address, obj_addr, true)
    }
}
```


<a id="0x1_object_create_object_internal"></a>

## Function `create_object_internal`



```move
module 0x1::object {
    fun create_object_internal(creator_address: address, object: address, can_delete: bool): object::ConstructorRef
}
```


##### Implementation


```move
module 0x1::object {
    fun create_object_internal(
        creator_address: address,
        object: address,
        can_delete: bool,
    ): ConstructorRef {
        assert!(!exists<ObjectCore>(object), error::already_exists(EOBJECT_EXISTS));

        let object_signer = create_signer(object);
        let guid_creation_num = INIT_GUID_CREATION_NUM;
        let transfer_events_guid = guid::create(object, &mut guid_creation_num);

        move_to(
            &object_signer,
            ObjectCore {
                guid_creation_num,
                owner: creator_address,
                allow_ungated_transfer: true,
                transfer_events: event::new_event_handle(transfer_events_guid),
            },
        );
        ConstructorRef { self: object, can_delete }
    }
}
```


<a id="0x1_object_generate_delete_ref"></a>

## Function `generate_delete_ref`

Generates the DeleteRef, which can be used to remove ObjectCore from global storage.


```move
module 0x1::object {
    public fun generate_delete_ref(ref: &object::ConstructorRef): object::DeleteRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_delete_ref(ref: &ConstructorRef): DeleteRef {
        assert!(ref.can_delete, error::permission_denied(ECANNOT_DELETE));
        DeleteRef { self: ref.self }
    }
}
```


<a id="0x1_object_generate_extend_ref"></a>

## Function `generate_extend_ref`

Generates the ExtendRef, which can be used to add new events and resources to the object.


```move
module 0x1::object {
    public fun generate_extend_ref(ref: &object::ConstructorRef): object::ExtendRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_extend_ref(ref: &ConstructorRef): ExtendRef {
        ExtendRef { self: ref.self }
    }
}
```


<a id="0x1_object_generate_transfer_ref"></a>

## Function `generate_transfer_ref`

Generates the TransferRef, which can be used to manage object transfers.


```move
module 0x1::object {
    public fun generate_transfer_ref(ref: &object::ConstructorRef): object::TransferRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_transfer_ref(ref: &ConstructorRef): TransferRef {
        assert!(!exists<Untransferable>(ref.self), error::permission_denied(EOBJECT_NOT_TRANSFERRABLE));
        TransferRef { self: ref.self }
    }
}
```


<a id="0x1_object_generate_derive_ref"></a>

## Function `generate_derive_ref`

Generates the DeriveRef, which can be used to create determnistic derived objects from the current object.


```move
module 0x1::object {
    public fun generate_derive_ref(ref: &object::ConstructorRef): object::DeriveRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_derive_ref(ref: &ConstructorRef): DeriveRef {
        DeriveRef { self: ref.self }
    }
}
```


<a id="0x1_object_generate_signer"></a>

## Function `generate_signer`

Create a signer for the ConstructorRef


```move
module 0x1::object {
    public fun generate_signer(ref: &object::ConstructorRef): signer
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_signer(ref: &ConstructorRef): signer {
        create_signer(ref.self)
    }
}
```


<a id="0x1_object_address_from_constructor_ref"></a>

## Function `address_from_constructor_ref`

Returns the address associated with the constructor


```move
module 0x1::object {
    public fun address_from_constructor_ref(ref: &object::ConstructorRef): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun address_from_constructor_ref(ref: &ConstructorRef): address {
        ref.self
    }
}
```


<a id="0x1_object_object_from_constructor_ref"></a>

## Function `object_from_constructor_ref`

Returns an Object&lt;T&gt; from within a ConstructorRef


```move
module 0x1::object {
    public fun object_from_constructor_ref<T: key>(ref: &object::ConstructorRef): object::Object<T>
}
```


##### Implementation


```move
module 0x1::object {
    public fun object_from_constructor_ref<T: key>(ref: &ConstructorRef): Object<T> {
        address_to_object<T>(ref.self)
    }
}
```


<a id="0x1_object_can_generate_delete_ref"></a>

## Function `can_generate_delete_ref`

Returns whether or not the ConstructorRef can be used to create DeleteRef


```move
module 0x1::object {
    public fun can_generate_delete_ref(ref: &object::ConstructorRef): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun can_generate_delete_ref(ref: &ConstructorRef): bool {
        ref.can_delete
    }
}
```


<a id="0x1_object_create_guid"></a>

## Function `create_guid`

Create a guid for the object, typically used for events


```move
module 0x1::object {
    public fun create_guid(object: &signer): guid::GUID
}
```


##### Implementation


```move
module 0x1::object {
    public fun create_guid(object: &signer): guid::GUID acquires ObjectCore {
        let addr = signer::address_of(object);
        let object_data = borrow_global_mut<ObjectCore>(addr);
        guid::create(addr, &mut object_data.guid_creation_num)
    }
}
```


<a id="0x1_object_new_event_handle"></a>

## Function `new_event_handle`

Generate a new event handle.


```move
module 0x1::object {
    public fun new_event_handle<T: drop, store>(object: &signer): event::EventHandle<T>
}
```


##### Implementation


```move
module 0x1::object {
    public fun new_event_handle<T: drop + store>(
        object: &signer,
    ): event::EventHandle<T> acquires ObjectCore {
        event::new_event_handle(create_guid(object))
    }
}
```


<a id="0x1_object_address_from_delete_ref"></a>

## Function `address_from_delete_ref`

Returns the address associated with the constructor


```move
module 0x1::object {
    public fun address_from_delete_ref(ref: &object::DeleteRef): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun address_from_delete_ref(ref: &DeleteRef): address {
        ref.self
    }
}
```


<a id="0x1_object_object_from_delete_ref"></a>

## Function `object_from_delete_ref`

Returns an Object&lt;T&gt; from within a DeleteRef.


```move
module 0x1::object {
    public fun object_from_delete_ref<T: key>(ref: &object::DeleteRef): object::Object<T>
}
```


##### Implementation


```move
module 0x1::object {
    public fun object_from_delete_ref<T: key>(ref: &DeleteRef): Object<T> {
        address_to_object<T>(ref.self)
    }
}
```


<a id="0x1_object_delete"></a>

## Function `delete`

Removes from the specified Object from global storage.


```move
module 0x1::object {
    public fun delete(ref: object::DeleteRef)
}
```


##### Implementation


```move
module 0x1::object {
    public fun delete(ref: DeleteRef) acquires Untransferable, ObjectCore {
        let object_core = move_from<ObjectCore>(ref.self);
        let ObjectCore {
            guid_creation_num: _,
            owner: _,
            allow_ungated_transfer: _,
            transfer_events,
        } = object_core;

        if (exists<Untransferable>(ref.self)) {
          let Untransferable {} = move_from<Untransferable>(ref.self);
        };

        event::destroy_handle(transfer_events);
    }
}
```


<a id="0x1_object_generate_signer_for_extending"></a>

## Function `generate_signer_for_extending`

Create a signer for the ExtendRef


```move
module 0x1::object {
    public fun generate_signer_for_extending(ref: &object::ExtendRef): signer
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_signer_for_extending(ref: &ExtendRef): signer {
        create_signer(ref.self)
    }
}
```


<a id="0x1_object_address_from_extend_ref"></a>

## Function `address_from_extend_ref`

Returns an address from within a ExtendRef.


```move
module 0x1::object {
    public fun address_from_extend_ref(ref: &object::ExtendRef): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun address_from_extend_ref(ref: &ExtendRef): address {
        ref.self
    }
}
```


<a id="0x1_object_disable_ungated_transfer"></a>

## Function `disable_ungated_transfer`

Disable direct transfer, transfers can only be triggered via a TransferRef


```move
module 0x1::object {
    public fun disable_ungated_transfer(ref: &object::TransferRef)
}
```


##### Implementation


```move
module 0x1::object {
    public fun disable_ungated_transfer(ref: &TransferRef) acquires ObjectCore {
        let object = borrow_global_mut<ObjectCore>(ref.self);
        object.allow_ungated_transfer = false;
    }
}
```


<a id="0x1_object_set_untransferable"></a>

## Function `set_untransferable`

Prevent moving of the object


```move
module 0x1::object {
    public fun set_untransferable(ref: &object::ConstructorRef)
}
```


##### Implementation


```move
module 0x1::object {
    public fun set_untransferable(ref: &ConstructorRef) acquires ObjectCore {
        let object = borrow_global_mut<ObjectCore>(ref.self);
        object.allow_ungated_transfer = false;
        let object_signer = generate_signer(ref);
        move_to(&object_signer, Untransferable {});
    }
}
```


<a id="0x1_object_enable_ungated_transfer"></a>

## Function `enable_ungated_transfer`

Enable direct transfer.


```move
module 0x1::object {
    public fun enable_ungated_transfer(ref: &object::TransferRef)
}
```


##### Implementation


```move
module 0x1::object {
    public fun enable_ungated_transfer(ref: &TransferRef) acquires ObjectCore {
        assert!(!exists<Untransferable>(ref.self), error::permission_denied(EOBJECT_NOT_TRANSFERRABLE));
        let object = borrow_global_mut<ObjectCore>(ref.self);
        object.allow_ungated_transfer = true;
    }
}
```


<a id="0x1_object_generate_linear_transfer_ref"></a>

## Function `generate_linear_transfer_ref`

Create a LinearTransferRef for a one&#45;time transfer. This requires that the owner at the
time of generation is the owner at the time of transferring.


```move
module 0x1::object {
    public fun generate_linear_transfer_ref(ref: &object::TransferRef): object::LinearTransferRef
}
```


##### Implementation


```move
module 0x1::object {
    public fun generate_linear_transfer_ref(ref: &TransferRef): LinearTransferRef acquires ObjectCore {
        assert!(!exists<Untransferable>(ref.self), error::permission_denied(EOBJECT_NOT_TRANSFERRABLE));
        let owner = owner(Object<ObjectCore> { inner: ref.self });
        LinearTransferRef {
            self: ref.self,
            owner,
        }
    }
}
```


<a id="0x1_object_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer to the destination address using a LinearTransferRef.


```move
module 0x1::object {
    public fun transfer_with_ref(ref: object::LinearTransferRef, to: address)
}
```


##### Implementation


```move
module 0x1::object {
    public fun transfer_with_ref(ref: LinearTransferRef, to: address) acquires ObjectCore, TombStone {
        assert!(!exists<Untransferable>(ref.self), error::permission_denied(EOBJECT_NOT_TRANSFERRABLE));

        // Undo soft burn if present as we don't want the original owner to be able to reclaim by calling unburn later.
        if (exists<TombStone>(ref.self)) {
            let TombStone { original_owner: _ } = move_from<TombStone>(ref.self);
        };

        let object = borrow_global_mut<ObjectCore>(ref.self);
        assert!(
            object.owner == ref.owner,
            error::permission_denied(ENOT_OBJECT_OWNER),
        );
        if (std::features::module_event_migration_enabled()) {
            event::emit(
                Transfer {
                    object: ref.self,
                    from: object.owner,
                    to,
                },
            );
        };
        event::emit_event(
            &mut object.transfer_events,
            TransferEvent {
                object: ref.self,
                from: object.owner,
                to,
            },
        );
        object.owner = to;
    }
}
```


<a id="0x1_object_transfer_call"></a>

## Function `transfer_call`

Entry function that can be used to transfer, if allow_ungated_transfer is set true.


```move
module 0x1::object {
    public entry fun transfer_call(owner: &signer, object: address, to: address)
}
```


##### Implementation


```move
module 0x1::object {
    public entry fun transfer_call(
        owner: &signer,
        object: address,
        to: address,
    ) acquires ObjectCore {
        transfer_raw(owner, object, to)
    }
}
```


<a id="0x1_object_transfer"></a>

## Function `transfer`

Transfers ownership of the object (and all associated resources) at the specified address
for Object&lt;T&gt; to the &quot;to&quot; address.


```move
module 0x1::object {
    public entry fun transfer<T: key>(owner: &signer, object: object::Object<T>, to: address)
}
```


##### Implementation


```move
module 0x1::object {
    public entry fun transfer<T: key>(
        owner: &signer,
        object: Object<T>,
        to: address,
    ) acquires ObjectCore {
        transfer_raw(owner, object.inner, to)
    }
}
```


<a id="0x1_object_transfer_raw"></a>

## Function `transfer_raw`

Attempts to transfer using addresses only. Transfers the given object if
allow_ungated_transfer is set true. Note, that this allows the owner of a nested object to
transfer that object, so long as allow_ungated_transfer is enabled at each stage in the
hierarchy.


```move
module 0x1::object {
    public fun transfer_raw(owner: &signer, object: address, to: address)
}
```


##### Implementation


```move
module 0x1::object {
    public fun transfer_raw(
        owner: &signer,
        object: address,
        to: address,
    ) acquires ObjectCore {
        let owner_address = signer::address_of(owner);
        verify_ungated_and_descendant(owner_address, object);
        transfer_raw_inner(object, to);
    }
}
```


<a id="0x1_object_transfer_raw_inner"></a>

## Function `transfer_raw_inner`



```move
module 0x1::object {
    fun transfer_raw_inner(object: address, to: address)
}
```


##### Implementation


```move
module 0x1::object {
    inline fun transfer_raw_inner(object: address, to: address) acquires ObjectCore {
        let object_core = borrow_global_mut<ObjectCore>(object);
        if (object_core.owner != to) {
            if (std::features::module_event_migration_enabled()) {
                event::emit(
                    Transfer {
                        object,
                        from: object_core.owner,
                        to,
                    },
                );
            };
            event::emit_event(
                &mut object_core.transfer_events,
                TransferEvent {
                    object,
                    from: object_core.owner,
                    to,
                },
            );
            object_core.owner = to;
        };
    }
}
```


<a id="0x1_object_transfer_to_object"></a>

## Function `transfer_to_object`

Transfer the given object to another object. See `transfer` for more information.


```move
module 0x1::object {
    public entry fun transfer_to_object<O: key, T: key>(owner: &signer, object: object::Object<O>, to: object::Object<T>)
}
```


##### Implementation


```move
module 0x1::object {
    public entry fun transfer_to_object<O: key, T: key>(
        owner: &signer,
        object: Object<O>,
        to: Object<T>,
    ) acquires ObjectCore {
        transfer(owner, object, to.inner)
    }
}
```


<a id="0x1_object_verify_ungated_and_descendant"></a>

## Function `verify_ungated_and_descendant`

This checks that the destination address is eventually owned by the owner and that each
object between the two allows for ungated transfers. Note, this is limited to a depth of 8
objects may have cyclic dependencies.


```move
module 0x1::object {
    fun verify_ungated_and_descendant(owner: address, destination: address)
}
```


##### Implementation


```move
module 0x1::object {
    fun verify_ungated_and_descendant(owner: address, destination: address) acquires ObjectCore {
        let current_address = destination;
        assert!(
            exists<ObjectCore>(current_address),
            error::not_found(EOBJECT_DOES_NOT_EXIST),
        );

        let object = borrow_global<ObjectCore>(current_address);
        assert!(
            object.allow_ungated_transfer,
            error::permission_denied(ENO_UNGATED_TRANSFERS),
        );

        let current_address = object.owner;
        let count = 0;
        while (owner != current_address) {
            count = count + 1;
            assert!(count < MAXIMUM_OBJECT_NESTING, error::out_of_range(EMAXIMUM_NESTING));
            // At this point, the first object exists and so the more likely case is that the
            // object's owner is not an object. So we return a more sensible error.
            assert!(
                exists<ObjectCore>(current_address),
                error::permission_denied(ENOT_OBJECT_OWNER),
            );
            let object = borrow_global<ObjectCore>(current_address);
            assert!(
                object.allow_ungated_transfer,
                error::permission_denied(ENO_UNGATED_TRANSFERS),
            );
            current_address = object.owner;
        };
    }
}
```


<a id="0x1_object_burn"></a>

## Function `burn`

Forcefully transfer an unwanted object to BURN_ADDRESS, ignoring whether ungated_transfer is allowed.
This only works for objects directly owned and for simplicity does not apply to indirectly owned objects.
Original owners can reclaim burnt objects any time in the future by calling unburn.


```move
module 0x1::object {
    public entry fun burn<T: key>(owner: &signer, object: object::Object<T>)
}
```


##### Implementation


```move
module 0x1::object {
    public entry fun burn<T: key>(owner: &signer, object: Object<T>) acquires ObjectCore {
        let original_owner = signer::address_of(owner);
        assert!(is_owner(object, original_owner), error::permission_denied(ENOT_OBJECT_OWNER));
        let object_addr = object.inner;
        move_to(&create_signer(object_addr), TombStone { original_owner });
        transfer_raw_inner(object_addr, BURN_ADDRESS);
    }
}
```


<a id="0x1_object_unburn"></a>

## Function `unburn`

Allow origin owners to reclaim any objects they previous burnt.


```move
module 0x1::object {
    public entry fun unburn<T: key>(original_owner: &signer, object: object::Object<T>)
}
```


##### Implementation


```move
module 0x1::object {
    public entry fun unburn<T: key>(
        original_owner: &signer,
        object: Object<T>,
    ) acquires TombStone, ObjectCore {
        let object_addr = object.inner;
        assert!(exists<TombStone>(object_addr), error::invalid_argument(EOBJECT_NOT_BURNT));

        let TombStone { original_owner: original_owner_addr } = move_from<TombStone>(object_addr);
        assert!(original_owner_addr == signer::address_of(original_owner), error::permission_denied(ENOT_OBJECT_OWNER));
        transfer_raw_inner(object_addr, original_owner_addr);
    }
}
```


<a id="0x1_object_ungated_transfer_allowed"></a>

## Function `ungated_transfer_allowed`

Accessors
Return true if ungated transfer is allowed.


```move
module 0x1::object {
    public fun ungated_transfer_allowed<T: key>(object: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun ungated_transfer_allowed<T: key>(object: Object<T>): bool acquires ObjectCore {
        assert!(
            exists<ObjectCore>(object.inner),
            error::not_found(EOBJECT_DOES_NOT_EXIST),
        );
        borrow_global<ObjectCore>(object.inner).allow_ungated_transfer
    }
}
```


<a id="0x1_object_owner"></a>

## Function `owner`

Return the current owner.


```move
module 0x1::object {
    public fun owner<T: key>(object: object::Object<T>): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun owner<T: key>(object: Object<T>): address acquires ObjectCore {
        assert!(
            exists<ObjectCore>(object.inner),
            error::not_found(EOBJECT_DOES_NOT_EXIST),
        );
        borrow_global<ObjectCore>(object.inner).owner
    }
}
```


<a id="0x1_object_is_owner"></a>

## Function `is_owner`

Return true if the provided address is the current owner.


```move
module 0x1::object {
    public fun is_owner<T: key>(object: object::Object<T>, owner: address): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun is_owner<T: key>(object: Object<T>, owner: address): bool acquires ObjectCore {
        owner(object) == owner
    }
}
```


<a id="0x1_object_owns"></a>

## Function `owns`

Return true if the provided address has indirect or direct ownership of the provided object.


```move
module 0x1::object {
    public fun owns<T: key>(object: object::Object<T>, owner: address): bool
}
```


##### Implementation


```move
module 0x1::object {
    public fun owns<T: key>(object: Object<T>, owner: address): bool acquires ObjectCore {
        let current_address = object_address(&object);
        if (current_address == owner) {
            return true
        };

        assert!(
            exists<ObjectCore>(current_address),
            error::not_found(EOBJECT_DOES_NOT_EXIST),
        );

        let object = borrow_global<ObjectCore>(current_address);
        let current_address = object.owner;

        let count = 0;
        while (owner != current_address) {
            count = count + 1;
            assert!(count < MAXIMUM_OBJECT_NESTING, error::out_of_range(EMAXIMUM_NESTING));
            if (!exists<ObjectCore>(current_address)) {
                return false
            };

            let object = borrow_global<ObjectCore>(current_address);
            current_address = object.owner;
        };
        true
    }
}
```


<a id="0x1_object_root_owner"></a>

## Function `root_owner`

Returns the root owner of an object. As objects support nested ownership, it can be useful
to determine the identity of the starting point of ownership.


```move
module 0x1::object {
    public fun root_owner<T: key>(object: object::Object<T>): address
}
```


##### Implementation


```move
module 0x1::object {
    public fun root_owner<T: key>(object: Object<T>): address acquires ObjectCore {
        let obj_owner = owner(object);
        while (is_object(obj_owner)) {
            obj_owner = owner(address_to_object<ObjectCore>(obj_owner));
        };
        obj_owner
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
<td>It&apos;s not possible to create an object twice on the same address.</td>
<td>Critical</td>
<td>The create_object_internal function includes an assertion to ensure that the object being created does not already exist at the specified address.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](create_object_internal).</td>
</tr>

<tr>
<td>2</td>
<td>Only its owner may transfer an object.</td>
<td>Critical</td>
<td>The transfer function mandates that the transaction be signed by the owner&apos;s address, ensuring that only the rightful owner may initiate the object transfer.</td>
<td>Audited that it aborts if anyone other than the owner attempts to transfer.</td>
</tr>

<tr>
<td>3</td>
<td>The indirect owner of an object may transfer the object.</td>
<td>Medium</td>
<td>The owns function evaluates to true when the given address possesses either direct or indirect ownership of the specified object.</td>
<td>Audited that it aborts if address transferring is not indirect owner.</td>
</tr>

<tr>
<td>4</td>
<td>Objects may never change the address which houses them.</td>
<td>Low</td>
<td>After creating an object, transfers to another owner may occur. However, the address which stores the object may not be changed.</td>
<td>This is implied by [#high&#45;level&#45;req](high&#45;level requirement 1).</td>
</tr>

<tr>
<td>5</td>
<td>If an ungated transfer is disabled on an object in an indirect ownership chain, a transfer should not occur.</td>
<td>Medium</td>
<td>Calling disable_ungated_transfer disables direct transfer, and only TransferRef may trigger transfers. The transfer_with_ref function is called.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](transfer_with_ref).</td>
</tr>

<tr>
<td>6</td>
<td>Object addresses must not overlap with other addresses in different domains.</td>
<td>Critical</td>
<td>The current addressing scheme with suffixes does not conflict with any existing addresses, such as resource accounts. The GUID space is explicitly separated to ensure this doesn&apos;t happen.</td>
<td>This is true by construction if one correctly ensures the usage of INIT_GUID_CREATION_NUM during the creation of GUID.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::object {
    pragma aborts_if_is_strict;
}
```



<a id="0x1_object_spec_exists_at"></a>


```move
module 0x1::object {
    fun spec_exists_at<T: key>(object: address): bool;
}
```


<a id="@Specification_1_address_to_object"></a>

### Function `address_to_object`


```move
module 0x1::object {
    public fun address_to_object<T: key>(object: address): object::Object<T>
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(object);
    aborts_if !spec_exists_at<T>(object);
    ensures result == Object<T> { inner: object };
}
```


<a id="@Specification_1_create_object_address"></a>

### Function `create_object_address`


```move
module 0x1::object {
    public fun create_object_address(source: &address, seed: vector<u8>): address
}
```



```move
module 0x1::object {
    pragma opaque;
    pragma aborts_if_is_strict = false;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_create_object_address(source, seed);
}
```



<a id="0x1_object_spec_create_user_derived_object_address_impl"></a>


```move
module 0x1::object {
    fun spec_create_user_derived_object_address_impl(source: address, derive_from: address): address;
}
```


<a id="@Specification_1_create_user_derived_object_address_impl"></a>

### Function `create_user_derived_object_address_impl`


```move
module 0x1::object {
    fun create_user_derived_object_address_impl(source: address, derive_from: address): address
}
```



```move
module 0x1::object {
    pragma opaque;
    ensures [abstract] result == spec_create_user_derived_object_address_impl(source, derive_from);
}
```


<a id="@Specification_1_create_user_derived_object_address"></a>

### Function `create_user_derived_object_address`


```move
module 0x1::object {
    public fun create_user_derived_object_address(source: address, derive_from: address): address
}
```



```move
module 0x1::object {
    pragma opaque;
    pragma aborts_if_is_strict = false;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_create_user_derived_object_address(source, derive_from);
}
```


<a id="@Specification_1_create_guid_object_address"></a>

### Function `create_guid_object_address`


```move
module 0x1::object {
    public fun create_guid_object_address(source: address, creation_num: u64): address
}
```



```move
module 0x1::object {
    pragma opaque;
    pragma aborts_if_is_strict = false;
    aborts_if [abstract] false;
    ensures [abstract] result == spec_create_guid_object_address(source, creation_num);
}
```


<a id="@Specification_1_exists_at"></a>

### Function `exists_at`


```move
module 0x1::object {
    fun exists_at<T: key>(object: address): bool
}
```



```move
module 0x1::object {
    pragma opaque;
    ensures [abstract] result == spec_exists_at<T>(object);
}
```


<a id="@Specification_1_object_address"></a>

### Function `object_address`


```move
module 0x1::object {
    public fun object_address<T: key>(object: &object::Object<T>): address
}
```



```move
module 0x1::object {
    aborts_if false;
    ensures result == object.inner;
}
```


<a id="@Specification_1_convert"></a>

### Function `convert`


```move
module 0x1::object {
    public fun convert<X: key, Y: key>(object: object::Object<X>): object::Object<Y>
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(object.inner);
    aborts_if !spec_exists_at<Y>(object.inner);
    ensures result == Object<Y> { inner: object.inner };
}
```


<a id="@Specification_1_create_named_object"></a>

### Function `create_named_object`


```move
module 0x1::object {
    public fun create_named_object(creator: &signer, seed: vector<u8>): object::ConstructorRef
}
```



```move
module 0x1::object {
    let creator_address = signer::address_of(creator);
    let obj_addr = spec_create_object_address(creator_address, seed);
    aborts_if exists<ObjectCore>(obj_addr);
    ensures exists<ObjectCore>(obj_addr);
    ensures global<ObjectCore>(obj_addr) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: creator_address,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: obj_addr,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: obj_addr, can_delete: false };
}
```


<a id="@Specification_1_create_user_derived_object"></a>

### Function `create_user_derived_object`


```move
module 0x1::object {
    public(friend) fun create_user_derived_object(creator_address: address, derive_ref: &object::DeriveRef): object::ConstructorRef
}
```



```move
module 0x1::object {
    let obj_addr = spec_create_user_derived_object_address(creator_address, derive_ref.self);
    aborts_if exists<ObjectCore>(obj_addr);
    ensures exists<ObjectCore>(obj_addr);
    ensures global<ObjectCore>(obj_addr) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: creator_address,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: obj_addr,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: obj_addr, can_delete: false };
}
```


<a id="@Specification_1_create_object"></a>

### Function `create_object`


```move
module 0x1::object {
    public fun create_object(owner_address: address): object::ConstructorRef
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let unique_address = transaction_context::spec_generate_unique_address();
    aborts_if exists<ObjectCore>(unique_address);
    ensures exists<ObjectCore>(unique_address);
    ensures global<ObjectCore>(unique_address) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: owner_address,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: unique_address,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: unique_address, can_delete: true };
}
```


<a id="@Specification_1_create_sticky_object"></a>

### Function `create_sticky_object`


```move
module 0x1::object {
    public fun create_sticky_object(owner_address: address): object::ConstructorRef
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let unique_address = transaction_context::spec_generate_unique_address();
    aborts_if exists<ObjectCore>(unique_address);
    ensures exists<ObjectCore>(unique_address);
    ensures global<ObjectCore>(unique_address) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: owner_address,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: unique_address,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: unique_address, can_delete: false };
}
```


<a id="@Specification_1_create_sticky_object_at_address"></a>

### Function `create_sticky_object_at_address`


```move
module 0x1::object {
    public(friend) fun create_sticky_object_at_address(owner_address: address, object_address: address): object::ConstructorRef
}
```



```move
module 0x1::object {
    pragma verify = false;
}
```


<a id="@Specification_1_create_object_from_account"></a>

### Function `create_object_from_account`


```move
module 0x1::object {
    #[deprecated]
    public fun create_object_from_account(creator: &signer): object::ConstructorRef
}
```



```move
module 0x1::object {
    aborts_if !exists<account::Account>(signer::address_of(creator));
    let object_data = global<account::Account>(signer::address_of(creator));
    aborts_if object_data.guid_creation_num + 1 > MAX_U64;
    aborts_if object_data.guid_creation_num + 1 >= account::MAX_GUID_CREATION_NUM;
    let creation_num = object_data.guid_creation_num;
    let addr = signer::address_of(creator);
    let guid = guid::GUID {
        id: guid::ID {
            creation_num,
            addr,
        }
    };
    let bytes_spec = bcs::to_bytes(guid);
    let bytes = concat(bytes_spec, vec<u8>(OBJECT_FROM_GUID_ADDRESS_SCHEME));
    let hash_bytes = hash::sha3_256(bytes);
    let obj_addr = from_bcs::deserialize<address>(hash_bytes);
    aborts_if exists<ObjectCore>(obj_addr);
    aborts_if !from_bcs::deserializable<address>(hash_bytes);
    ensures global<account::Account>(addr).guid_creation_num == old(
        global<account::Account>(addr)
    ).guid_creation_num + 1;
    ensures exists<ObjectCore>(obj_addr);
    ensures global<ObjectCore>(obj_addr) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: addr,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: obj_addr,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: obj_addr, can_delete: true };
}
```


<a id="@Specification_1_create_object_from_object"></a>

### Function `create_object_from_object`


```move
module 0x1::object {
    #[deprecated]
    public fun create_object_from_object(creator: &signer): object::ConstructorRef
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(signer::address_of(creator));
    let object_data = global<ObjectCore>(signer::address_of(creator));
    aborts_if object_data.guid_creation_num + 1 > MAX_U64;
    let creation_num = object_data.guid_creation_num;
    let addr = signer::address_of(creator);
    let guid = guid::GUID {
        id: guid::ID {
            creation_num,
            addr,
        }
    };
    let bytes_spec = bcs::to_bytes(guid);
    let bytes = concat(bytes_spec, vec<u8>(OBJECT_FROM_GUID_ADDRESS_SCHEME));
    let hash_bytes = hash::sha3_256(bytes);
    let obj_addr = from_bcs::deserialize<address>(hash_bytes);
    aborts_if exists<ObjectCore>(obj_addr);
    aborts_if !from_bcs::deserializable<address>(hash_bytes);
    ensures global<ObjectCore>(addr).guid_creation_num == old(global<ObjectCore>(addr)).guid_creation_num + 1;
    ensures exists<ObjectCore>(obj_addr);
    ensures global<ObjectCore>(obj_addr) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: addr,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: obj_addr,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: obj_addr, can_delete: true };
}
```


<a id="@Specification_1_create_object_from_guid"></a>

### Function `create_object_from_guid`


```move
module 0x1::object {
    fun create_object_from_guid(creator_address: address, guid: guid::GUID): object::ConstructorRef
}
```



```move
module 0x1::object {
    let bytes_spec = bcs::to_bytes(guid);
    let bytes = concat(bytes_spec, vec<u8>(OBJECT_FROM_GUID_ADDRESS_SCHEME));
    let hash_bytes = hash::sha3_256(bytes);
    let obj_addr = from_bcs::deserialize<address>(hash_bytes);
    aborts_if exists<ObjectCore>(obj_addr);
    aborts_if !from_bcs::deserializable<address>(hash_bytes);
    ensures exists<ObjectCore>(obj_addr);
    ensures global<ObjectCore>(obj_addr) == ObjectCore {
        guid_creation_num: INIT_GUID_CREATION_NUM + 1,
        owner: creator_address,
        allow_ungated_transfer: true,
        transfer_events: event::EventHandle {
            counter: 0,
            guid: guid::GUID {
                id: guid::ID {
                    creation_num: INIT_GUID_CREATION_NUM,
                    addr: obj_addr,
                }
            }
        }
    };
    ensures result == ConstructorRef { self: obj_addr, can_delete: true };
}
```


<a id="@Specification_1_create_object_internal"></a>

### Function `create_object_internal`


```move
module 0x1::object {
    fun create_object_internal(creator_address: address, object: address, can_delete: bool): object::ConstructorRef
}
```



```move
module 0x1::object {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    aborts_if exists<ObjectCore>(object);
    ensures exists<ObjectCore>(object);
    ensures global<ObjectCore>(object).guid_creation_num == INIT_GUID_CREATION_NUM + 1;
    ensures result == ConstructorRef { self: object, can_delete };
}
```


<a id="@Specification_1_generate_delete_ref"></a>

### Function `generate_delete_ref`


```move
module 0x1::object {
    public fun generate_delete_ref(ref: &object::ConstructorRef): object::DeleteRef
}
```



```move
module 0x1::object {
    aborts_if !ref.can_delete;
    ensures result == DeleteRef { self: ref.self };
}
```


<a id="@Specification_1_generate_transfer_ref"></a>

### Function `generate_transfer_ref`


```move
module 0x1::object {
    public fun generate_transfer_ref(ref: &object::ConstructorRef): object::TransferRef
}
```



```move
module 0x1::object {
    aborts_if exists<Untransferable>(ref.self);
    ensures result == TransferRef {
        self: ref.self,
    };
}
```


<a id="@Specification_1_object_from_constructor_ref"></a>

### Function `object_from_constructor_ref`


```move
module 0x1::object {
    public fun object_from_constructor_ref<T: key>(ref: &object::ConstructorRef): object::Object<T>
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(ref.self);
    aborts_if !spec_exists_at<T>(ref.self);
    ensures result == Object<T> { inner: ref.self };
}
```


<a id="@Specification_1_create_guid"></a>

### Function `create_guid`


```move
module 0x1::object {
    public fun create_guid(object: &signer): guid::GUID
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(signer::address_of(object));
    let object_data = global<ObjectCore>(signer::address_of(object));
    aborts_if object_data.guid_creation_num + 1 > MAX_U64;
    ensures result == guid::GUID {
        id: guid::ID {
            creation_num: object_data.guid_creation_num,
            addr: signer::address_of(object)
        }
    };
}
```


<a id="@Specification_1_new_event_handle"></a>

### Function `new_event_handle`


```move
module 0x1::object {
    public fun new_event_handle<T: drop, store>(object: &signer): event::EventHandle<T>
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(signer::address_of(object));
    let object_data = global<ObjectCore>(signer::address_of(object));
    aborts_if object_data.guid_creation_num + 1 > MAX_U64;
    let guid = guid::GUID {
        id: guid::ID {
            creation_num: object_data.guid_creation_num,
            addr: signer::address_of(object)
        }
    };
    ensures result == event::EventHandle<T> {
        counter: 0,
        guid,
    };
}
```


<a id="@Specification_1_object_from_delete_ref"></a>

### Function `object_from_delete_ref`


```move
module 0x1::object {
    public fun object_from_delete_ref<T: key>(ref: &object::DeleteRef): object::Object<T>
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(ref.self);
    aborts_if !spec_exists_at<T>(ref.self);
    ensures result == Object<T> { inner: ref.self };
}
```


<a id="@Specification_1_delete"></a>

### Function `delete`


```move
module 0x1::object {
    public fun delete(ref: object::DeleteRef)
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(ref.self);
    ensures !exists<ObjectCore>(ref.self);
}
```


<a id="@Specification_1_disable_ungated_transfer"></a>

### Function `disable_ungated_transfer`


```move
module 0x1::object {
    public fun disable_ungated_transfer(ref: &object::TransferRef)
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(ref.self);
    ensures global<ObjectCore>(ref.self).allow_ungated_transfer == false;
}
```


<a id="@Specification_1_set_untransferable"></a>

### Function `set_untransferable`


```move
module 0x1::object {
    public fun set_untransferable(ref: &object::ConstructorRef)
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(ref.self);
    aborts_if exists<Untransferable>(ref.self);
    ensures exists<Untransferable>(ref.self);
    ensures global<ObjectCore>(ref.self).allow_ungated_transfer == false;
}
```


<a id="@Specification_1_enable_ungated_transfer"></a>

### Function `enable_ungated_transfer`


```move
module 0x1::object {
    public fun enable_ungated_transfer(ref: &object::TransferRef)
}
```



```move
module 0x1::object {
    aborts_if exists<Untransferable>(ref.self);
    aborts_if !exists<ObjectCore>(ref.self);
    ensures global<ObjectCore>(ref.self).allow_ungated_transfer == true;
}
```


<a id="@Specification_1_generate_linear_transfer_ref"></a>

### Function `generate_linear_transfer_ref`


```move
module 0x1::object {
    public fun generate_linear_transfer_ref(ref: &object::TransferRef): object::LinearTransferRef
}
```



```move
module 0x1::object {
    aborts_if exists<Untransferable>(ref.self);
    aborts_if !exists<ObjectCore>(ref.self);
    let owner = global<ObjectCore>(ref.self).owner;
    ensures result == LinearTransferRef {
        self: ref.self,
        owner,
    };
}
```


<a id="@Specification_1_transfer_with_ref"></a>

### Function `transfer_with_ref`


```move
module 0x1::object {
    public fun transfer_with_ref(ref: object::LinearTransferRef, to: address)
}
```



```move
module 0x1::object {
    aborts_if exists<Untransferable>(ref.self);
    let object = global<ObjectCore>(ref.self);
    aborts_if !exists<ObjectCore>(ref.self);
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if object.owner != ref.owner;
    ensures global<ObjectCore>(ref.self).owner == to;
}
```


<a id="@Specification_1_transfer_call"></a>

### Function `transfer_call`


```move
module 0x1::object {
    public entry fun transfer_call(owner: &signer, object: address, to: address)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let owner_address = signer::address_of(owner);
    aborts_if !exists<ObjectCore>(object);
    aborts_if !global<ObjectCore>(object).allow_ungated_transfer;
}
```


<a id="@Specification_1_transfer"></a>

### Function `transfer`


```move
module 0x1::object {
    public entry fun transfer<T: key>(owner: &signer, object: object::Object<T>, to: address)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let owner_address = signer::address_of(owner);
    let object_address = object.inner;
    aborts_if !exists<ObjectCore>(object_address);
    aborts_if !global<ObjectCore>(object_address).allow_ungated_transfer;
}
```


<a id="@Specification_1_transfer_raw"></a>

### Function `transfer_raw`


```move
module 0x1::object {
    public fun transfer_raw(owner: &signer, object: address, to: address)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let owner_address = signer::address_of(owner);
    aborts_if !exists<ObjectCore>(object);
    aborts_if !global<ObjectCore>(object).allow_ungated_transfer;
}
```


<a id="@Specification_1_transfer_to_object"></a>

### Function `transfer_to_object`


```move
module 0x1::object {
    public entry fun transfer_to_object<O: key, T: key>(owner: &signer, object: object::Object<O>, to: object::Object<T>)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let owner_address = signer::address_of(owner);
    let object_address = object.inner;
    aborts_if !exists<ObjectCore>(object_address);
    aborts_if !global<ObjectCore>(object_address).allow_ungated_transfer;
}
```


<a id="@Specification_1_verify_ungated_and_descendant"></a>

### Function `verify_ungated_and_descendant`


```move
module 0x1::object {
    fun verify_ungated_and_descendant(owner: address, destination: address)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    pragma unroll = MAXIMUM_OBJECT_NESTING;
    aborts_if !exists<ObjectCore>(destination);
    aborts_if !global<ObjectCore>(destination).allow_ungated_transfer;
}
```


<a id="@Specification_1_burn"></a>

### Function `burn`


```move
module 0x1::object {
    public entry fun burn<T: key>(owner: &signer, object: object::Object<T>)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let object_address = object.inner;
    aborts_if !exists<ObjectCore>(object_address);
    aborts_if owner(object) != signer::address_of(owner);
    aborts_if is_burnt(object);
}
```


<a id="@Specification_1_unburn"></a>

### Function `unburn`


```move
module 0x1::object {
    public entry fun unburn<T: key>(original_owner: &signer, object: object::Object<T>)
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let object_address = object.inner;
    aborts_if !exists<ObjectCore>(object_address);
    aborts_if !is_burnt(object);
    let tomb_stone = borrow_global<TombStone>(object_address);
    aborts_if tomb_stone.original_owner != signer::address_of(original_owner);
}
```


<a id="@Specification_1_ungated_transfer_allowed"></a>

### Function `ungated_transfer_allowed`


```move
module 0x1::object {
    public fun ungated_transfer_allowed<T: key>(object: object::Object<T>): bool
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(object.inner);
    ensures result == global<ObjectCore>(object.inner).allow_ungated_transfer;
}
```


<a id="@Specification_1_owner"></a>

### Function `owner`


```move
module 0x1::object {
    public fun owner<T: key>(object: object::Object<T>): address
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(object.inner);
    ensures result == global<ObjectCore>(object.inner).owner;
}
```


<a id="@Specification_1_is_owner"></a>

### Function `is_owner`


```move
module 0x1::object {
    public fun is_owner<T: key>(object: object::Object<T>, owner: address): bool
}
```



```move
module 0x1::object {
    aborts_if !exists<ObjectCore>(object.inner);
    ensures result == (global<ObjectCore>(object.inner).owner == owner);
}
```


<a id="@Specification_1_owns"></a>

### Function `owns`


```move
module 0x1::object {
    public fun owns<T: key>(object: object::Object<T>, owner: address): bool
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
    let current_address_0 = object.inner;
    let object_0 = global<ObjectCore>(current_address_0);
    let current_address = object_0.owner;
    aborts_if object.inner != owner && !exists<ObjectCore>(object.inner);
    ensures current_address_0 == owner ==> result == true;
}
```


<a id="@Specification_1_root_owner"></a>

### Function `root_owner`


```move
module 0x1::object {
    public fun root_owner<T: key>(object: object::Object<T>): address
}
```



```move
module 0x1::object {
    pragma aborts_if_is_partial;
}
```



<a id="0x1_object_spec_create_object_address"></a>


```move
module 0x1::object {
    fun spec_create_object_address(source: address, seed: vector<u8>): address;
}
```



<a id="0x1_object_spec_create_user_derived_object_address"></a>


```move
module 0x1::object {
    fun spec_create_user_derived_object_address(source: address, derive_from: address): address;
}
```



<a id="0x1_object_spec_create_guid_object_address"></a>


```move
module 0x1::object {
    fun spec_create_guid_object_address(source: address, creation_num: u64): address;
}
```
