
<a id="0x1_primary_fungible_store"></a>

# Module `0x1::primary_fungible_store`

This module provides a way for creators of fungible assets to enable support for creating primary (deterministic)
stores for their users. This is useful for assets that are meant to be used as a currency, as it allows users to
easily create a store for their account and deposit/withdraw/transfer fungible assets to/from it.

The transfer flow works as below:
1. The sender calls `transfer` on the fungible asset metadata object to transfer `amount` of fungible asset to
`recipient`.
2. The fungible asset metadata object calls `ensure_primary_store_exists` to ensure that both the sender&apos;s and the
recipient&apos;s primary stores exist. If either doesn&apos;t, it will be created.
3. The fungible asset metadata object calls `withdraw` on the sender&apos;s primary store to withdraw `amount` of
fungible asset from it. This emits a withdraw event.
4. The fungible asset metadata object calls `deposit` on the recipient&apos;s primary store to deposit `amount` of
fungible asset to it. This emits an deposit event.


-  [Resource `DeriveRefPod`](#0x1_primary_fungible_store_DeriveRefPod)
-  [Function `create_primary_store_enabled_fungible_asset`](#0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset)
-  [Function `ensure_primary_store_exists`](#0x1_primary_fungible_store_ensure_primary_store_exists)
-  [Function `create_primary_store`](#0x1_primary_fungible_store_create_primary_store)
-  [Function `primary_store_address`](#0x1_primary_fungible_store_primary_store_address)
-  [Function `primary_store`](#0x1_primary_fungible_store_primary_store)
-  [Function `primary_store_exists`](#0x1_primary_fungible_store_primary_store_exists)
-  [Function `primary_store_address_inlined`](#0x1_primary_fungible_store_primary_store_address_inlined)
-  [Function `primary_store_inlined`](#0x1_primary_fungible_store_primary_store_inlined)
-  [Function `primary_store_exists_inlined`](#0x1_primary_fungible_store_primary_store_exists_inlined)
-  [Function `balance`](#0x1_primary_fungible_store_balance)
-  [Function `is_balance_at_least`](#0x1_primary_fungible_store_is_balance_at_least)
-  [Function `is_frozen`](#0x1_primary_fungible_store_is_frozen)
-  [Function `withdraw`](#0x1_primary_fungible_store_withdraw)
-  [Function `deposit`](#0x1_primary_fungible_store_deposit)
-  [Function `force_deposit`](#0x1_primary_fungible_store_force_deposit)
-  [Function `transfer`](#0x1_primary_fungible_store_transfer)
-  [Function `transfer_assert_minimum_deposit`](#0x1_primary_fungible_store_transfer_assert_minimum_deposit)
-  [Function `mint`](#0x1_primary_fungible_store_mint)
-  [Function `burn`](#0x1_primary_fungible_store_burn)
-  [Function `set_frozen_flag`](#0x1_primary_fungible_store_set_frozen_flag)
-  [Function `withdraw_with_ref`](#0x1_primary_fungible_store_withdraw_with_ref)
-  [Function `deposit_with_ref`](#0x1_primary_fungible_store_deposit_with_ref)
-  [Function `transfer_with_ref`](#0x1_primary_fungible_store_transfer_with_ref)
-  [Function `may_be_unburn`](#0x1_primary_fungible_store_may_be_unburn)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)


```move
module 0x1::primary_fungible_store {
    use 0x1::dispatchable_fungible_asset;
    use 0x1::fungible_asset;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
}
```


<a id="0x1_primary_fungible_store_DeriveRefPod"></a>

## Resource `DeriveRefPod`

A resource that holds the derive ref for the fungible asset metadata object. This is used to create primary
stores for users with deterministic addresses so that users can easily deposit/withdraw/transfer fungible
assets.


```move
module 0x1::primary_fungible_store {
    #[resource_group_member(#[group = 0x1::object::ObjectGroup])]
    struct DeriveRefPod has key
}
```


##### Fields


<dl>
<dt>
`metadata_derive_ref: object::DeriveRef`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset"></a>

## Function `create_primary_store_enabled_fungible_asset`

Create a fungible asset with primary store support. When users transfer fungible assets to each other, their
primary stores will be created automatically if they don&apos;t exist. Primary stores have deterministic addresses
so that users can easily deposit/withdraw/transfer fungible assets.


```move
module 0x1::primary_fungible_store {
    public fun create_primary_store_enabled_fungible_asset(constructor_ref: &object::ConstructorRef, maximum_supply: option::Option<u128>, name: string::String, symbol: string::String, decimals: u8, icon_uri: string::String, project_uri: string::String)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun create_primary_store_enabled_fungible_asset(
        constructor_ref: &ConstructorRef,
        maximum_supply: Option<u128>,
        name: String,
        symbol: String,
        decimals: u8,
        icon_uri: String,
        project_uri: String,
    ) {
        fungible_asset::add_fungibility(
            constructor_ref,
            maximum_supply,
            name,
            symbol,
            decimals,
            icon_uri,
            project_uri,
        );
        let metadata_obj = &object::generate_signer(constructor_ref);
        move_to(metadata_obj, DeriveRefPod {
            metadata_derive_ref: object::generate_derive_ref(constructor_ref),
        });
    }
}
```


<a id="0x1_primary_fungible_store_ensure_primary_store_exists"></a>

## Function `ensure_primary_store_exists`

Ensure that the primary store object for the given address exists. If it doesn&apos;t, create it.


```move
module 0x1::primary_fungible_store {
    public fun ensure_primary_store_exists<T: key>(owner: address, metadata: object::Object<T>): object::Object<fungible_asset::FungibleStore>
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun ensure_primary_store_exists<T: key>(
        owner: address,
        metadata: Object<T>,
    ): Object<FungibleStore> acquires DeriveRefPod {
        let store_addr = primary_store_address(owner, metadata);
        if (fungible_asset::store_exists(store_addr)) {
            object::address_to_object(store_addr)
        } else {
            create_primary_store(owner, metadata)
        }
    }
}
```


<a id="0x1_primary_fungible_store_create_primary_store"></a>

## Function `create_primary_store`

Create a primary store object to hold fungible asset for the given address.


```move
module 0x1::primary_fungible_store {
    public fun create_primary_store<T: key>(owner_addr: address, metadata: object::Object<T>): object::Object<fungible_asset::FungibleStore>
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun create_primary_store<T: key>(
        owner_addr: address,
        metadata: Object<T>,
    ): Object<FungibleStore> acquires DeriveRefPod {
        let metadata_addr = object::object_address(&metadata);
        object::address_to_object<Metadata>(metadata_addr);
        let derive_ref = &borrow_global<DeriveRefPod>(metadata_addr).metadata_derive_ref;
        let constructor_ref = &object::create_user_derived_object(owner_addr, derive_ref);
        // Disable ungated transfer as deterministic stores shouldn't be transferrable.
        let transfer_ref = &object::generate_transfer_ref(constructor_ref);
        object::disable_ungated_transfer(transfer_ref);

        fungible_asset::create_store(constructor_ref, metadata)
    }
}
```


<a id="0x1_primary_fungible_store_primary_store_address"></a>

## Function `primary_store_address`

Get the address of the primary store for the given account.


```move
module 0x1::primary_fungible_store {
    #[view]
    public fun primary_store_address<T: key>(owner: address, metadata: object::Object<T>): address
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun primary_store_address<T: key>(owner: address, metadata: Object<T>): address {
        let metadata_addr = object::object_address(&metadata);
        object::create_user_derived_object_address(owner, metadata_addr)
    }
}
```


<a id="0x1_primary_fungible_store_primary_store"></a>

## Function `primary_store`

Get the primary store object for the given account.


```move
module 0x1::primary_fungible_store {
    #[view]
    public fun primary_store<T: key>(owner: address, metadata: object::Object<T>): object::Object<fungible_asset::FungibleStore>
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun primary_store<T: key>(owner: address, metadata: Object<T>): Object<FungibleStore> {
        let store = primary_store_address(owner, metadata);
        object::address_to_object<FungibleStore>(store)
    }
}
```


<a id="0x1_primary_fungible_store_primary_store_exists"></a>

## Function `primary_store_exists`

Return whether the given account&apos;s primary store exists.


```move
module 0x1::primary_fungible_store {
    #[view]
    public fun primary_store_exists<T: key>(account: address, metadata: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun primary_store_exists<T: key>(account: address, metadata: Object<T>): bool {
        fungible_asset::store_exists(primary_store_address(account, metadata))
    }
}
```


<a id="0x1_primary_fungible_store_primary_store_address_inlined"></a>

## Function `primary_store_address_inlined`

Get the address of the primary store for the given account.
Use instead of the corresponding view functions for dispatchable hooks to avoid circular dependencies of modules.


```move
module 0x1::primary_fungible_store {
    public fun primary_store_address_inlined<T: key>(owner: address, metadata: object::Object<T>): address
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public inline fun primary_store_address_inlined<T: key>(owner: address, metadata: Object<T>): address {
        let metadata_addr = object::object_address(&metadata);
        object::create_user_derived_object_address(owner, metadata_addr)
    }
}
```


<a id="0x1_primary_fungible_store_primary_store_inlined"></a>

## Function `primary_store_inlined`

Get the primary store object for the given account.
Use instead of the corresponding view functions for dispatchable hooks to avoid circular dependencies of modules.


```move
module 0x1::primary_fungible_store {
    public fun primary_store_inlined<T: key>(owner: address, metadata: object::Object<T>): object::Object<fungible_asset::FungibleStore>
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public inline fun primary_store_inlined<T: key>(owner: address, metadata: Object<T>): Object<FungibleStore> {
        let store = primary_store_address_inlined(owner, metadata);
        object::address_to_object(store)
    }
}
```


<a id="0x1_primary_fungible_store_primary_store_exists_inlined"></a>

## Function `primary_store_exists_inlined`

Return whether the given account&apos;s primary store exists.
Use instead of the corresponding view functions for dispatchable hooks to avoid circular dependencies of modules.


```move
module 0x1::primary_fungible_store {
    public fun primary_store_exists_inlined<T: key>(account: address, metadata: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public inline fun primary_store_exists_inlined<T: key>(account: address, metadata: Object<T>): bool {
        fungible_asset::store_exists(primary_store_address_inlined(account, metadata))
    }
}
```


<a id="0x1_primary_fungible_store_balance"></a>

## Function `balance`

Get the balance of `account`&apos;s primary store.


```move
module 0x1::primary_fungible_store {
    #[view]
    public fun balance<T: key>(account: address, metadata: object::Object<T>): u64
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun balance<T: key>(account: address, metadata: Object<T>): u64 {
        if (primary_store_exists(account, metadata)) {
            fungible_asset::balance(primary_store(account, metadata))
        } else {
            0
        }
    }
}
```


<a id="0x1_primary_fungible_store_is_balance_at_least"></a>

## Function `is_balance_at_least`



```move
module 0x1::primary_fungible_store {
    #[view]
    public fun is_balance_at_least<T: key>(account: address, metadata: object::Object<T>, amount: u64): bool
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun is_balance_at_least<T: key>(account: address, metadata: Object<T>, amount: u64): bool {
        if (primary_store_exists(account, metadata)) {
            fungible_asset::is_balance_at_least(primary_store(account, metadata), amount)
        } else {
            amount == 0
        }
    }
}
```


<a id="0x1_primary_fungible_store_is_frozen"></a>

## Function `is_frozen`

Return whether the given account&apos;s primary store is frozen.


```move
module 0x1::primary_fungible_store {
    #[view]
    public fun is_frozen<T: key>(account: address, metadata: object::Object<T>): bool
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun is_frozen<T: key>(account: address, metadata: Object<T>): bool {
        if (primary_store_exists(account, metadata)) {
            fungible_asset::is_frozen(primary_store(account, metadata))
        } else {
            false
        }
    }
}
```


<a id="0x1_primary_fungible_store_withdraw"></a>

## Function `withdraw`

Withdraw `amount` of fungible asset from the given account&apos;s primary store.


```move
module 0x1::primary_fungible_store {
    public fun withdraw<T: key>(owner: &signer, metadata: object::Object<T>, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun withdraw<T: key>(owner: &signer, metadata: Object<T>, amount: u64): FungibleAsset acquires DeriveRefPod {
        let store = ensure_primary_store_exists(signer::address_of(owner), metadata);
        // Check if the store object has been burnt or not. If so, unburn it first.
        may_be_unburn(owner, store);
        dispatchable_fungible_asset::withdraw(owner, store, amount)
    }
}
```


<a id="0x1_primary_fungible_store_deposit"></a>

## Function `deposit`

Deposit fungible asset `fa` to the given account&apos;s primary store.


```move
module 0x1::primary_fungible_store {
    public fun deposit(owner: address, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun deposit(owner: address, fa: FungibleAsset) acquires DeriveRefPod {
        let metadata = fungible_asset::asset_metadata(&fa);
        let store = ensure_primary_store_exists(owner, metadata);
        dispatchable_fungible_asset::deposit(store, fa);
    }
}
```


<a id="0x1_primary_fungible_store_force_deposit"></a>

## Function `force_deposit`

Deposit fungible asset `fa` to the given account&apos;s primary store.


```move
module 0x1::primary_fungible_store {
    public(friend) fun force_deposit(owner: address, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public(friend) fun force_deposit(owner: address, fa: FungibleAsset) acquires DeriveRefPod {
        let metadata = fungible_asset::asset_metadata(&fa);
        let store = ensure_primary_store_exists(owner, metadata);
        fungible_asset::deposit_internal(object::object_address(&store), fa);
    }
}
```


<a id="0x1_primary_fungible_store_transfer"></a>

## Function `transfer`

Transfer `amount` of fungible asset from sender&apos;s primary store to receiver&apos;s primary store.


```move
module 0x1::primary_fungible_store {
    public entry fun transfer<T: key>(sender: &signer, metadata: object::Object<T>, recipient: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public entry fun transfer<T: key>(
        sender: &signer,
        metadata: Object<T>,
        recipient: address,
        amount: u64,
    ) acquires DeriveRefPod {
        let sender_store = ensure_primary_store_exists(signer::address_of(sender), metadata);
        // Check if the sender store object has been burnt or not. If so, unburn it first.
        may_be_unburn(sender, sender_store);
        let recipient_store = ensure_primary_store_exists(recipient, metadata);
        dispatchable_fungible_asset::transfer(sender, sender_store, recipient_store, amount);
    }
}
```


<a id="0x1_primary_fungible_store_transfer_assert_minimum_deposit"></a>

## Function `transfer_assert_minimum_deposit`

Transfer `amount` of fungible asset from sender&apos;s primary store to receiver&apos;s primary store.
Use the minimum deposit assertion api to make sure receipient will receive a minimum amount of fund.


```move
module 0x1::primary_fungible_store {
    public entry fun transfer_assert_minimum_deposit<T: key>(sender: &signer, metadata: object::Object<T>, recipient: address, amount: u64, expected: u64)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public entry fun transfer_assert_minimum_deposit<T: key>(
        sender: &signer,
        metadata: Object<T>,
        recipient: address,
        amount: u64,
        expected: u64,
    ) acquires DeriveRefPod {
        let sender_store = ensure_primary_store_exists(signer::address_of(sender), metadata);
        // Check if the sender store object has been burnt or not. If so, unburn it first.
        may_be_unburn(sender, sender_store);
        let recipient_store = ensure_primary_store_exists(recipient, metadata);
        dispatchable_fungible_asset::transfer_assert_minimum_deposit(
            sender,
            sender_store,
            recipient_store,
            amount,
            expected
        );
    }
}
```


<a id="0x1_primary_fungible_store_mint"></a>

## Function `mint`

Mint to the primary store of `owner`.


```move
module 0x1::primary_fungible_store {
    public fun mint(mint_ref: &fungible_asset::MintRef, owner: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun mint(mint_ref: &MintRef, owner: address, amount: u64) acquires DeriveRefPod {
        let primary_store = ensure_primary_store_exists(owner, fungible_asset::mint_ref_metadata(mint_ref));
        fungible_asset::mint_to(mint_ref, primary_store, amount);
    }
}
```


<a id="0x1_primary_fungible_store_burn"></a>

## Function `burn`

Burn from the primary store of `owner`.


```move
module 0x1::primary_fungible_store {
    public fun burn(burn_ref: &fungible_asset::BurnRef, owner: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun burn(burn_ref: &BurnRef, owner: address, amount: u64) {
        let primary_store = primary_store(owner, fungible_asset::burn_ref_metadata(burn_ref));
        fungible_asset::burn_from(burn_ref, primary_store, amount);
    }
}
```


<a id="0x1_primary_fungible_store_set_frozen_flag"></a>

## Function `set_frozen_flag`

Freeze/Unfreeze the primary store of `owner`.


```move
module 0x1::primary_fungible_store {
    public fun set_frozen_flag(transfer_ref: &fungible_asset::TransferRef, owner: address, frozen: bool)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun set_frozen_flag(transfer_ref: &TransferRef, owner: address, frozen: bool) acquires DeriveRefPod {
        let primary_store = ensure_primary_store_exists(owner, fungible_asset::transfer_ref_metadata(transfer_ref));
        fungible_asset::set_frozen_flag(transfer_ref, primary_store, frozen);
    }
}
```


<a id="0x1_primary_fungible_store_withdraw_with_ref"></a>

## Function `withdraw_with_ref`

Withdraw from the primary store of `owner` ignoring frozen flag.


```move
module 0x1::primary_fungible_store {
    public fun withdraw_with_ref(transfer_ref: &fungible_asset::TransferRef, owner: address, amount: u64): fungible_asset::FungibleAsset
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun withdraw_with_ref(transfer_ref: &TransferRef, owner: address, amount: u64): FungibleAsset {
        let from_primary_store = primary_store(owner, fungible_asset::transfer_ref_metadata(transfer_ref));
        fungible_asset::withdraw_with_ref(transfer_ref, from_primary_store, amount)
    }
}
```


<a id="0x1_primary_fungible_store_deposit_with_ref"></a>

## Function `deposit_with_ref`

Deposit from the primary store of `owner` ignoring frozen flag.


```move
module 0x1::primary_fungible_store {
    public fun deposit_with_ref(transfer_ref: &fungible_asset::TransferRef, owner: address, fa: fungible_asset::FungibleAsset)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun deposit_with_ref(transfer_ref: &TransferRef, owner: address, fa: FungibleAsset) acquires DeriveRefPod {
        let from_primary_store = ensure_primary_store_exists(
            owner,
            fungible_asset::transfer_ref_metadata(transfer_ref)
        );
        fungible_asset::deposit_with_ref(transfer_ref, from_primary_store, fa);
    }
}
```


<a id="0x1_primary_fungible_store_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer `amount` of FA from the primary store of `from` to that of `to` ignoring frozen flag.


```move
module 0x1::primary_fungible_store {
    public fun transfer_with_ref(transfer_ref: &fungible_asset::TransferRef, from: address, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    public fun transfer_with_ref(
        transfer_ref: &TransferRef,
        from: address,
        to: address,
        amount: u64
    ) acquires DeriveRefPod {
        let from_primary_store = primary_store(from, fungible_asset::transfer_ref_metadata(transfer_ref));
        let to_primary_store = ensure_primary_store_exists(to, fungible_asset::transfer_ref_metadata(transfer_ref));
        fungible_asset::transfer_with_ref(transfer_ref, from_primary_store, to_primary_store, amount);
    }
}
```


<a id="0x1_primary_fungible_store_may_be_unburn"></a>

## Function `may_be_unburn`



```move
module 0x1::primary_fungible_store {
    fun may_be_unburn(owner: &signer, store: object::Object<fungible_asset::FungibleStore>)
}
```


##### Implementation


```move
module 0x1::primary_fungible_store {
    fun may_be_unburn(owner: &signer, store: Object<FungibleStore>) {
        if (object::is_burnt(store)) {
            object::unburn(owner, store);
        };
    }
}
```


<a id="@Specification_0"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>Creating a fungible asset with primary store support should initiate a derived reference and store it under the metadata object.</td>
<td>Medium</td>
<td>The function create_primary_store_enabled_fungible_asset makes an existing object, fungible, via the fungible_asset::add_fungibility function and initializes the DeriveRefPod resource by generating a DeriveRef for the object and then stores it under the object address.</td>
<td>Audited that the DeriveRefPod has been properly initialized and stored under the metadata object.</td>
</tr>

<tr>
<td>2</td>
<td>Fetching and creating a primary fungible store of an asset should only succeed if the object supports primary store.</td>
<td>Low</td>
<td>The function create_primary_store is used to create a primary store by borrowing the DeriveRef resource from the object. In case the resource does not exist, creation will fail. The function ensure_primary_store_exists is used to fetch the primary store if it exists, otherwise it will create one via the create_primary function.</td>
<td>Audited that it aborts if the DeriveRefPod doesn&apos;t exist. Audited that it aborts if the FungibleStore resource exists already under the object address.</td>
</tr>

<tr>
<td>3</td>
<td>It should be possible to create a primary store to hold a fungible asset.</td>
<td>Medium</td>
<td>The function create_primary_store borrows the DeriveRef resource from DeriveRefPod and then creates the store which is returned.</td>
<td>Audited that it returns the newly created FungibleStore.</td>
</tr>

<tr>
<td>4</td>
<td>Fetching the balance or the frozen status of a primary store should never abort.</td>
<td>Low</td>
<td>The function balance returns the balance of the store, if the store exists, otherwise it returns 0. The function is_frozen returns the frozen flag of the fungible store, if the store exists, otherwise it returns false.</td>
<td>Audited that the balance function returns the balance of the FungibleStore. Audited that the is_frozen function returns the frozen status of the FungibleStore resource. Audited that it never aborts.</td>
</tr>

<tr>
<td>5</td>
<td>The ability to withdraw, deposit, transfer, mint and burn should only be available for assets with primary store support.</td>
<td>Medium</td>
<td>The primary store is fetched before performing either of withdraw, deposit, transfer, mint, burn operation. If the FungibleStore resource doesn&apos;t exist the operation will fail.</td>
<td>Audited that it aborts if the primary store FungibleStore doesn&apos;t exist.</td>
</tr>

<tr>
<td>6</td>
<td>The action of depositing a fungible asset of the same type as the store should never fail if the store is not frozen.</td>
<td>Medium</td>
<td>The function deposit fetches the owner&apos;s store, if it doesn&apos;t exist it will be created, and then deposits the fungible asset to it. The function deposit_with_ref fetches the owner&apos;s store, if it doesn&apos;t exist it will be created, and then deposit the fungible asset via the fungible_asset::deposit_with_ref function. Depositing fails if the metadata of the FungibleStore and FungibleAsset differs.</td>
<td>Audited that it aborts if the store is frozen (deposit). Audited that the balance of the store is increased by the deposit amount (deposit, deposit_with_ref). Audited that it aborts if the metadata of the store and the asset differs (deposit, deposit_with_ref).</td>
</tr>

<tr>
<td>7</td>
<td>Withdrawing should only be allowed to the owner of an existing store with sufficient balance.</td>
<td>Critical</td>
<td>The withdraw function fetches the owner&apos;s store via the primary_store function and then calls fungible_asset::withdraw which validates the owner of the store, checks the frozen status and the balance of the store. The withdraw_with_ref function fetches the store of the owner via primary_store function and calls the fungible_asset::withdraw_with_ref which validates transfer_ref&apos;s metadata with the withdrawing stores metadata, and the balance of the store.</td>
<td>Audited that it aborts if the owner doesn&apos;t own the store (withdraw). Audited that it aborts if the store is frozen (withdraw). Audited that it aborts if the transfer ref&apos;s metadata doesn&apos;t match the withdrawing store&apos;s metadata (withdraw_with_ref). Audited that it aborts if the store doesn&apos;t have sufficient balance. Audited that the store is not burned. Audited that the balance of the store is decreased by the amount withdrawn.</td>
</tr>

<tr>
<td>8</td>
<td>Only the fungible store owner is allowed to unburn a burned store.</td>
<td>High</td>
<td>The function may_be_unburn checks if the store is burned and then proceeds to call object::unburn which ensures that the owner of the object matches the address of the signer.</td>
<td>Audited that the store is unburned successfully.</td>
</tr>

<tr>
<td>9</td>
<td>Only the owner of a primary store can transfer its balance to any recipient&apos;s primary store.</td>
<td>High</td>
<td>The function transfer fetches sender and recipient&apos;s primary stores, if the sender&apos;s store is burned it unburns the store and calls the fungile_asset::transfer to proceed with the transfer, which first withdraws the assets from the sender&apos;s store and then deposits to the recipient&apos;s store. The function transfer_with_ref fetches the sender&apos;s and recipient&apos;s stores and calls the fungible_asset::transfer_with_ref function which withdraws the asset with the ref from the sender and deposits the asset to the recipient with the ref.</td>
<td>Audited the deposit and withdraw (transfer). Audited the deposit_with_ref and withdraw_with_ref (transfer_with_ref). Audited that the store balance of the sender is decreased by the specified amount and its added to the recipients store. (transfer, transfer_with_ref) Audited that the sender&apos;s store is not burned (transfer).</td>
</tr>

<tr>
<td>10</td>
<td>Minting an amount of assets to an unfrozen store is only allowed with a valid mint reference.</td>
<td>High</td>
<td>The mint function fetches the primary store and calls the fungible_asset::mint_to, which mints with MintRef&apos;s metadata which internally validates the amount and the increases the total supply of the asset. And the minted asset is deposited to the provided store by validating that the store is unfrozen and the store&apos;s metadata is the same as the depositing asset&apos;s metadata.</td>
<td>Audited that it aborts if the amount is equal to 0. Audited that it aborts if the store is frozen. Audited that it aborts if the mint_ref&apos;s metadata is not the same as the store&apos;s metadata. Audited that the asset&apos;s total supply is increased by the amount minted. Audited that the balance of the store is increased by the minted amount.</td>
</tr>

<tr>
<td>11</td>
<td>Burning an amount of assets from an existing unfrozen store is only allowed with a valid burn reference.</td>
<td>High</td>
<td>The burn function fetches the primary store and calls the fungible_asset::burn_from function which withdraws the amount from the store while enforcing that the store has enough balance and burns the withdrawn asset after validating the asset&apos;s metadata and the BurnRef&apos;s metadata followed by decreasing the supply of the asset.</td>
<td>Audited that it aborts if the metadata of the store is not same as the BurnRef&apos;s metadata. Audited that it aborts if the burning amount is 0. Audited that it aborts if the store doesn&apos;t have enough balance. Audited that it aborts if the asset&apos;s metadata is not same as the BurnRef&apos;s metadata. Audited that the total supply of the asset is decreased. Audited that the store&apos;s balance is reduced by the amount burned.</td>
</tr>

<tr>
<td>12</td>
<td>Setting the frozen flag of a store is only allowed with a valid reference.</td>
<td>High</td>
<td>The function set_frozen_flag fetches the primary store and calls fungible_asset::set_frozen_flag which validates the TransferRef&apos;s metadata with the store&apos;s metadata and then updates the frozen flag.</td>
<td>Audited that it aborts if the store&apos;s metadata is not same as the TransferRef&apos;s metadata. Audited that the status of the frozen flag is updated correctly.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::primary_fungible_store {
    pragma verify = false;
}
```



<a id="0x1_primary_fungible_store_spec_primary_store_exists"></a>


```move
module 0x1::primary_fungible_store {
    fun spec_primary_store_exists<T: key>(account: address, metadata: Object<T>): bool {
       fungible_asset::store_exists(spec_primary_store_address(account, metadata))
    }
}
```



<a id="0x1_primary_fungible_store_spec_primary_store_address"></a>


```move
module 0x1::primary_fungible_store {
    fun spec_primary_store_address<T: key>(owner: address, metadata: Object<T>): address {
       let metadata_addr = object::object_address(metadata);
       object::spec_create_user_derived_object_address(owner, metadata_addr)
    }
}
```
