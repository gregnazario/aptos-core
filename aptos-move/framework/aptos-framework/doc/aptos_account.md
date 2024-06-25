
<a id="0x1_aptos_account"></a>

# Module `0x1::aptos_account`



-  [Resource `DirectTransferConfig`](#0x1_aptos_account_DirectTransferConfig)
-  [Struct `DirectCoinTransferConfigUpdatedEvent`](#0x1_aptos_account_DirectCoinTransferConfigUpdatedEvent)
-  [Struct `DirectCoinTransferConfigUpdated`](#0x1_aptos_account_DirectCoinTransferConfigUpdated)
-  [Constants](#@Constants_0)
-  [Function `create_account`](#0x1_aptos_account_create_account)
-  [Function `batch_transfer`](#0x1_aptos_account_batch_transfer)
-  [Function `transfer`](#0x1_aptos_account_transfer)
-  [Function `batch_transfer_coins`](#0x1_aptos_account_batch_transfer_coins)
-  [Function `transfer_coins`](#0x1_aptos_account_transfer_coins)
-  [Function `deposit_coins`](#0x1_aptos_account_deposit_coins)
-  [Function `assert_account_exists`](#0x1_aptos_account_assert_account_exists)
-  [Function `assert_account_is_registered_for_apt`](#0x1_aptos_account_assert_account_is_registered_for_apt)
-  [Function `set_allow_direct_coin_transfers`](#0x1_aptos_account_set_allow_direct_coin_transfers)
-  [Function `can_receive_direct_coin_transfers`](#0x1_aptos_account_can_receive_direct_coin_transfers)
-  [Function `register_apt`](#0x1_aptos_account_register_apt)
-  [Function `fungible_transfer_only`](#0x1_aptos_account_fungible_transfer_only)
-  [Function `is_fungible_balance_at_least`](#0x1_aptos_account_is_fungible_balance_at_least)
-  [Function `burn_from_fungible_store`](#0x1_aptos_account_burn_from_fungible_store)
-  [Function `ensure_primary_fungible_store_exists`](#0x1_aptos_account_ensure_primary_fungible_store_exists)
-  [Function `primary_fungible_store_address`](#0x1_aptos_account_primary_fungible_store_address)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `create_account`](#@Specification_1_create_account)
    -  [Function `batch_transfer`](#@Specification_1_batch_transfer)
    -  [Function `transfer`](#@Specification_1_transfer)
    -  [Function `batch_transfer_coins`](#@Specification_1_batch_transfer_coins)
    -  [Function `transfer_coins`](#@Specification_1_transfer_coins)
    -  [Function `deposit_coins`](#@Specification_1_deposit_coins)
    -  [Function `assert_account_exists`](#@Specification_1_assert_account_exists)
    -  [Function `assert_account_is_registered_for_apt`](#@Specification_1_assert_account_is_registered_for_apt)
    -  [Function `set_allow_direct_coin_transfers`](#@Specification_1_set_allow_direct_coin_transfers)
    -  [Function `can_receive_direct_coin_transfers`](#@Specification_1_can_receive_direct_coin_transfers)
    -  [Function `register_apt`](#@Specification_1_register_apt)
    -  [Function `fungible_transfer_only`](#@Specification_1_fungible_transfer_only)
    -  [Function `is_fungible_balance_at_least`](#@Specification_1_is_fungible_balance_at_least)
    -  [Function `burn_from_fungible_store`](#@Specification_1_burn_from_fungible_store)


```move
module 0x1::aptos_account {
    use 0x1::account;
    use 0x1::aptos_coin;
    use 0x1::coin;
    use 0x1::create_signer;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::fungible_asset;
    use 0x1::object;
    use 0x1::primary_fungible_store;
    use 0x1::signer;
}
```


<a id="0x1_aptos_account_DirectTransferConfig"></a>

## Resource `DirectTransferConfig`

Configuration for whether an account can receive direct transfers of coins that they have not registered.

By default, this is enabled. Users can opt&#45;out by disabling at any time.


```move
module 0x1::aptos_account {
    struct DirectTransferConfig has key
}
```


##### Fields


<dl>
<dt>
`allow_arbitrary_coin_transfers: bool`
</dt>
<dd>

</dd>
<dt>
`update_coin_transfer_events: event::EventHandle<aptos_account::DirectCoinTransferConfigUpdatedEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_account_DirectCoinTransferConfigUpdatedEvent"></a>

## Struct `DirectCoinTransferConfigUpdatedEvent`

Event emitted when an account&apos;s direct coins transfer config is updated.


```move
module 0x1::aptos_account {
    struct DirectCoinTransferConfigUpdatedEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`new_allow_direct_transfers: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_account_DirectCoinTransferConfigUpdated"></a>

## Struct `DirectCoinTransferConfigUpdated`



```move
module 0x1::aptos_account {
    #[event]
    struct DirectCoinTransferConfigUpdated has drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`new_allow_direct_transfers: bool`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aptos_account_EACCOUNT_DOES_NOT_ACCEPT_DIRECT_COIN_TRANSFERS"></a>

Account opted out of receiving coins that they did not register to receive.


```move
module 0x1::aptos_account {
    const EACCOUNT_DOES_NOT_ACCEPT_DIRECT_COIN_TRANSFERS: u64 = 3;
}
```


<a id="0x1_aptos_account_EACCOUNT_DOES_NOT_ACCEPT_DIRECT_TOKEN_TRANSFERS"></a>

Account opted out of directly receiving NFT tokens.


```move
module 0x1::aptos_account {
    const EACCOUNT_DOES_NOT_ACCEPT_DIRECT_TOKEN_TRANSFERS: u64 = 4;
}
```


<a id="0x1_aptos_account_EACCOUNT_NOT_FOUND"></a>

Account does not exist.


```move
module 0x1::aptos_account {
    const EACCOUNT_NOT_FOUND: u64 = 1;
}
```


<a id="0x1_aptos_account_EACCOUNT_NOT_REGISTERED_FOR_APT"></a>

Account is not registered to receive APT.


```move
module 0x1::aptos_account {
    const EACCOUNT_NOT_REGISTERED_FOR_APT: u64 = 2;
}
```


<a id="0x1_aptos_account_EMISMATCHING_RECIPIENTS_AND_AMOUNTS_LENGTH"></a>

The lengths of the recipients and amounts lists don&apos;t match.


```move
module 0x1::aptos_account {
    const EMISMATCHING_RECIPIENTS_AND_AMOUNTS_LENGTH: u64 = 5;
}
```


<a id="0x1_aptos_account_create_account"></a>

## Function `create_account`

Basic account creation methods.


```move
module 0x1::aptos_account {
    public entry fun create_account(auth_key: address)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun create_account(auth_key: address) {
        let account_signer = account::create_account(auth_key);
        register_apt(&account_signer);
    }
}
```


<a id="0x1_aptos_account_batch_transfer"></a>

## Function `batch_transfer`

Batch version of APT transfer.


```move
module 0x1::aptos_account {
    public entry fun batch_transfer(source: &signer, recipients: vector<address>, amounts: vector<u64>)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun batch_transfer(source: &signer, recipients: vector<address>, amounts: vector<u64>) {
        let recipients_len = vector::length(&recipients);
        assert!(
            recipients_len == vector::length(&amounts),
            error::invalid_argument(EMISMATCHING_RECIPIENTS_AND_AMOUNTS_LENGTH),
        );

        vector::enumerate_ref(&recipients, |i, to| {
            let amount = *vector::borrow(&amounts, i);
            transfer(source, *to, amount);
        });
    }
}
```


<a id="0x1_aptos_account_transfer"></a>

## Function `transfer`

Convenient function to transfer APT to a recipient account that might not exist.
This would create the recipient account first, which also registers it to receive APT, before transferring.


```move
module 0x1::aptos_account {
    public entry fun transfer(source: &signer, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun transfer(source: &signer, to: address, amount: u64) {
        if (!account::exists_at(to)) {
            create_account(to)
        };

        if (features::operations_default_to_fa_apt_store_enabled()) {
            fungible_transfer_only(source, to, amount)
        } else {
            // Resource accounts can be created without registering them to receive APT.
            // This conveniently does the registration if necessary.
            if (!coin::is_account_registered<AptosCoin>(to)) {
                coin::register<AptosCoin>(&create_signer(to));
            };
            coin::transfer<AptosCoin>(source, to, amount)
        }
    }
}
```


<a id="0x1_aptos_account_batch_transfer_coins"></a>

## Function `batch_transfer_coins`

Batch version of transfer_coins.


```move
module 0x1::aptos_account {
    public entry fun batch_transfer_coins<CoinType>(from: &signer, recipients: vector<address>, amounts: vector<u64>)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun batch_transfer_coins<CoinType>(
        from: &signer, recipients: vector<address>, amounts: vector<u64>) acquires DirectTransferConfig {
        let recipients_len = vector::length(&recipients);
        assert!(
            recipients_len == vector::length(&amounts),
            error::invalid_argument(EMISMATCHING_RECIPIENTS_AND_AMOUNTS_LENGTH),
        );

        vector::enumerate_ref(&recipients, |i, to| {
            let amount = *vector::borrow(&amounts, i);
            transfer_coins<CoinType>(from, *to, amount);
        });
    }
}
```


<a id="0x1_aptos_account_transfer_coins"></a>

## Function `transfer_coins`

Convenient function to transfer a custom CoinType to a recipient account that might not exist.
This would create the recipient account first and register it to receive the CoinType, before transferring.


```move
module 0x1::aptos_account {
    public entry fun transfer_coins<CoinType>(from: &signer, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun transfer_coins<CoinType>(from: &signer, to: address, amount: u64) acquires DirectTransferConfig {
        deposit_coins(to, coin::withdraw<CoinType>(from, amount));
    }
}
```


<a id="0x1_aptos_account_deposit_coins"></a>

## Function `deposit_coins`

Convenient function to deposit a custom CoinType into a recipient account that might not exist.
This would create the recipient account first and register it to receive the CoinType, before transferring.


```move
module 0x1::aptos_account {
    public fun deposit_coins<CoinType>(to: address, coins: coin::Coin<CoinType>)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public fun deposit_coins<CoinType>(to: address, coins: Coin<CoinType>) acquires DirectTransferConfig {
        if (!account::exists_at(to)) {
            create_account(to);
            spec {
                assert coin::spec_is_account_registered<AptosCoin>(to);
                assume aptos_std::type_info::type_of<CoinType>() == aptos_std::type_info::type_of<AptosCoin>() ==>
                    coin::spec_is_account_registered<CoinType>(to);
            };
        };
        if (!coin::is_account_registered<CoinType>(to)) {
            assert!(
                can_receive_direct_coin_transfers(to),
                error::permission_denied(EACCOUNT_DOES_NOT_ACCEPT_DIRECT_COIN_TRANSFERS),
            );
            coin::register<CoinType>(&create_signer(to));
        };
        coin::deposit<CoinType>(to, coins)
    }
}
```


<a id="0x1_aptos_account_assert_account_exists"></a>

## Function `assert_account_exists`



```move
module 0x1::aptos_account {
    public fun assert_account_exists(addr: address)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public fun assert_account_exists(addr: address) {
        assert!(account::exists_at(addr), error::not_found(EACCOUNT_NOT_FOUND));
    }
}
```


<a id="0x1_aptos_account_assert_account_is_registered_for_apt"></a>

## Function `assert_account_is_registered_for_apt`



```move
module 0x1::aptos_account {
    public fun assert_account_is_registered_for_apt(addr: address)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public fun assert_account_is_registered_for_apt(addr: address) {
        assert_account_exists(addr);
        assert!(coin::is_account_registered<AptosCoin>(addr), error::not_found(EACCOUNT_NOT_REGISTERED_FOR_APT));
    }
}
```


<a id="0x1_aptos_account_set_allow_direct_coin_transfers"></a>

## Function `set_allow_direct_coin_transfers`

Set whether `account` can receive direct transfers of coins that they have not explicitly registered to receive.


```move
module 0x1::aptos_account {
    public entry fun set_allow_direct_coin_transfers(account: &signer, allow: bool)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public entry fun set_allow_direct_coin_transfers(account: &signer, allow: bool) acquires DirectTransferConfig {
        let addr = signer::address_of(account);
        if (exists<DirectTransferConfig>(addr)) {
            let direct_transfer_config = borrow_global_mut<DirectTransferConfig>(addr);
            // Short-circuit to avoid emitting an event if direct transfer config is not changing.
            if (direct_transfer_config.allow_arbitrary_coin_transfers == allow) {
                return
            };

            direct_transfer_config.allow_arbitrary_coin_transfers = allow;

            if (std::features::module_event_migration_enabled()) {
                emit(DirectCoinTransferConfigUpdated { account: addr, new_allow_direct_transfers: allow });
            };
            emit_event(
                &mut direct_transfer_config.update_coin_transfer_events,
                DirectCoinTransferConfigUpdatedEvent { new_allow_direct_transfers: allow });
        } else {
            let direct_transfer_config = DirectTransferConfig {
                allow_arbitrary_coin_transfers: allow,
                update_coin_transfer_events: new_event_handle<DirectCoinTransferConfigUpdatedEvent>(account),
            };
            if (std::features::module_event_migration_enabled()) {
                emit(DirectCoinTransferConfigUpdated { account: addr, new_allow_direct_transfers: allow });
            };
            emit_event(
                &mut direct_transfer_config.update_coin_transfer_events,
                DirectCoinTransferConfigUpdatedEvent { new_allow_direct_transfers: allow });
            move_to(account, direct_transfer_config);
        };
    }
}
```


<a id="0x1_aptos_account_can_receive_direct_coin_transfers"></a>

## Function `can_receive_direct_coin_transfers`

Return true if `account` can receive direct transfers of coins that they have not explicitly registered to
receive.

By default, this returns true if an account has not explicitly set whether the can receive direct transfers.


```move
module 0x1::aptos_account {
    #[view]
    public fun can_receive_direct_coin_transfers(account: address): bool
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public fun can_receive_direct_coin_transfers(account: address): bool acquires DirectTransferConfig {
        !exists<DirectTransferConfig>(account) ||
            borrow_global<DirectTransferConfig>(account).allow_arbitrary_coin_transfers
    }
}
```


<a id="0x1_aptos_account_register_apt"></a>

## Function `register_apt`



```move
module 0x1::aptos_account {
    public(friend) fun register_apt(account_signer: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public(friend) fun register_apt(account_signer: &signer) {
        if (features::new_accounts_default_to_fa_apt_store_enabled()) {
            ensure_primary_fungible_store_exists(signer::address_of(account_signer));
        } else {
            coin::register<AptosCoin>(account_signer);
        }
    }
}
```


<a id="0x1_aptos_account_fungible_transfer_only"></a>

## Function `fungible_transfer_only`

APT Primary Fungible Store specific specialized functions,
Utilized internally once migration of APT to FungibleAsset is complete.
Convenient function to transfer APT to a recipient account that might not exist.
This would create the recipient APT PFS first, which also registers it to receive APT, before transferring.
TODO: once migration is complete, rename to just &quot;transfer_only&quot; and make it an entry function (for cheapest way
to transfer APT) &#45; if we want to allow APT PFS without account itself


```move
module 0x1::aptos_account {
    fun fungible_transfer_only(source: &signer, to: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    fun fungible_transfer_only(
        source: &signer, to: address, amount: u64
    ) {
        let sender_store = ensure_primary_fungible_store_exists(signer::address_of(source));
        let recipient_store = ensure_primary_fungible_store_exists(to);

        // use internal APIs, as they skip:
        // - owner, frozen and dispatchable checks
        // as APT cannot be frozen or have dispatch, and PFS cannot be transfered
        // (PFS could potentially be burned. regular transfer would permanently unburn the store.
        // Ignoring the check here has the equivalent of unburning, transfers, and then burning again)
        fungible_asset::deposit_internal(recipient_store, fungible_asset::withdraw_internal(sender_store, amount));
    }
}
```


<a id="0x1_aptos_account_is_fungible_balance_at_least"></a>

## Function `is_fungible_balance_at_least`

Is balance from APT Primary FungibleStore at least the given amount


```move
module 0x1::aptos_account {
    public(friend) fun is_fungible_balance_at_least(account: address, amount: u64): bool
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public(friend) fun is_fungible_balance_at_least(account: address, amount: u64): bool {
        let store_addr = primary_fungible_store_address(account);
        fungible_asset::is_address_balance_at_least(store_addr, amount)
    }
}
```


<a id="0x1_aptos_account_burn_from_fungible_store"></a>

## Function `burn_from_fungible_store`

Burn from APT Primary FungibleStore


```move
module 0x1::aptos_account {
    public(friend) fun burn_from_fungible_store(ref: &fungible_asset::BurnRef, account: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::aptos_account {
    public(friend) fun burn_from_fungible_store(
        ref: &BurnRef,
        account: address,
        amount: u64,
    ) {
        // Skip burning if amount is zero. This shouldn't error out as it's called as part of transaction fee burning.
        if (amount != 0) {
            let store_addr = primary_fungible_store_address(account);
            fungible_asset::address_burn_from(ref, store_addr, amount);
        };
    }
}
```


<a id="0x1_aptos_account_ensure_primary_fungible_store_exists"></a>

## Function `ensure_primary_fungible_store_exists`

Ensure that APT Primary FungibleStore exists (and create if it doesn&apos;t)


```move
module 0x1::aptos_account {
    fun ensure_primary_fungible_store_exists(owner: address): address
}
```


##### Implementation


```move
module 0x1::aptos_account {
    inline fun ensure_primary_fungible_store_exists(owner: address): address {
        let store_addr = primary_fungible_store_address(owner);
        if (fungible_asset::store_exists(store_addr)) {
            store_addr
        } else {
            object::object_address(&primary_fungible_store::create_primary_store(owner, object::address_to_object<Metadata>(@aptos_fungible_asset)))
        }
    }
}
```


<a id="0x1_aptos_account_primary_fungible_store_address"></a>

## Function `primary_fungible_store_address`

Address of APT Primary Fungible Store


```move
module 0x1::aptos_account {
    fun primary_fungible_store_address(account: address): address
}
```


##### Implementation


```move
module 0x1::aptos_account {
    inline fun primary_fungible_store_address(account: address): address {
        object::create_user_derived_object_address(account, @aptos_fungible_asset)
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
<td>During the creation of an Aptos account the following rules should hold: (1) the authentication key should be 32 bytes in length, (2) an Aptos account should not already exist for that authentication key, and (3) the address of the authentication key should not be equal to a reserved address (0x0, 0x1, or 0x3).</td>
<td>Critical</td>
<td>The authentication key which is passed in as an argument to create_account should satisfy all necessary conditions.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](CreateAccountAbortsIf).</td>
</tr>

<tr>
<td>2</td>
<td>After creating an Aptos account, the account should become registered to receive AptosCoin.</td>
<td>Critical</td>
<td>The create_account function creates a new account for the particular address and registers AptosCoin.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](create_account).</td>
</tr>

<tr>
<td>3</td>
<td>An account may receive a direct transfer of coins they have not registered for if and only if the transfer of arbitrary coins is enabled. By default the option should always set to be enabled for an account.</td>
<td>Low</td>
<td>Transfers of a coin to an account that has not yet registered for that coin should abort if and only if the allow_arbitrary_coin_transfers flag is explicitly set to false.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](can_receive_direct_coin_transfers).</td>
</tr>

<tr>
<td>4</td>
<td>Setting direct coin transfers may only occur if and only if a direct transfer config is associated with the provided account address.</td>
<td>Low</td>
<td>The set_allow_direct_coin_transfers function ensures the DirectTransferConfig structure exists for the signer.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](set_allow_direct_coin_transfers).</td>
</tr>

<tr>
<td>5</td>
<td>The transfer function should ensure an account is created for the provided destination if one does not exist; then, register AptosCoin for that account if a particular is unregistered before transferring the amount.</td>
<td>Critical</td>
<td>The transfer function checks if the recipient account exists. If the account does not exist, the function creates one and registers the account to AptosCoin if not registered.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](transfer).</td>
</tr>

<tr>
<td>6</td>
<td>Creating an account for the provided destination and registering it for that particular CoinType should be the only way to enable depositing coins, provided the account does not already exist.</td>
<td>Critical</td>
<td>The deposit_coins function verifies if the recipient account exists. If the account does not exist, the function creates one and ensures that the account becomes registered for the specified CointType.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;6](deposit_coins).</td>
</tr>

<tr>
<td>7</td>
<td>When performing a batch transfer of Aptos Coin and/or a batch transfer of a custom coin type, it should ensure that the vector containing destination addresses and the vector containing the corresponding amounts are equal in length.</td>
<td>Low</td>
<td>The batch_transfer and batch_transfer_coins functions verify that the length of the recipient addresses vector matches the length of the amount vector through an assertion.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;7](batch_transfer_coins).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::aptos_account {
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_create_account"></a>

### Function `create_account`


```move
module 0x1::aptos_account {
    public entry fun create_account(auth_key: address)
}
```

Check if the bytes of the auth_key is 32.
The Account does not exist under the auth_key before creating the account.
Limit the address of auth_key is not @vm_reserved / @aptos_framework / @aptos_toke.


```move
module 0x1::aptos_account {
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    pragma aborts_if_is_partial;
    include CreateAccountAbortsIf;
    ensures exists<account::Account>(auth_key);
}
```



<a id="0x1_aptos_account_CreateAccountAbortsIf"></a>


```move
module 0x1::aptos_account {
    schema CreateAccountAbortsIf {
        auth_key: address;
        aborts_if exists<account::Account>(auth_key);
        aborts_if length_judgment(auth_key);
        aborts_if auth_key == @vm_reserved || auth_key == @aptos_framework || auth_key == @aptos_token;
    }
}
```



<a id="0x1_aptos_account_length_judgment"></a>


```move
module 0x1::aptos_account {
    fun length_judgment(auth_key: address): bool {
       use std::bcs;

       let authentication_key = bcs::to_bytes(auth_key);
       len(authentication_key) != 32
    }
}
```


<a id="@Specification_1_batch_transfer"></a>

### Function `batch_transfer`


```move
module 0x1::aptos_account {
    public entry fun batch_transfer(source: &signer, recipients: vector<address>, amounts: vector<u64>)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
    let account_addr_source = signer::address_of(source);
    let coin_store_source = global<coin::CoinStore<AptosCoin>>(account_addr_source);
    let balance_source = coin_store_source.coin.value;
    requires forall i in 0..len(recipients):
        recipients[i] != account_addr_source;
    requires exists i in 0..len(recipients):
        amounts[i] > 0;
    aborts_if len(recipients) != len(amounts);
    aborts_if exists i in 0..len(recipients):
            !account::exists_at(recipients[i]) && length_judgment(recipients[i]);
    aborts_if exists i in 0..len(recipients):
            !account::exists_at(recipients[i]) && (recipients[i] == @vm_reserved || recipients[i] == @aptos_framework || recipients[i] == @aptos_token);
    ensures forall i in 0..len(recipients):
            (!account::exists_at(recipients[i]) ==> !length_judgment(recipients[i])) &&
                (!account::exists_at(recipients[i]) ==> (recipients[i] != @vm_reserved && recipients[i] != @aptos_framework && recipients[i] != @aptos_token));
    aborts_if exists i in 0..len(recipients):
        !exists<coin::CoinStore<AptosCoin>>(account_addr_source);
    aborts_if exists i in 0..len(recipients):
        coin_store_source.frozen;
    aborts_if exists i in 0..len(recipients):
        global<coin::CoinStore<AptosCoin>>(account_addr_source).coin.value < amounts[i];
    aborts_if exists i in 0..len(recipients):
        exists<coin::CoinStore<AptosCoin>>(recipients[i]) && global<coin::CoinStore<AptosCoin>>(recipients[i]).frozen;
    aborts_if exists i in 0..len(recipients):
        account::exists_at(recipients[i]) && !exists<coin::CoinStore<AptosCoin>>(recipients[i]) && global<account::Account>(recipients[i]).guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
    aborts_if exists i in 0..len(recipients):
        account::exists_at(recipients[i]) && !exists<coin::CoinStore<AptosCoin>>(recipients[i]) && global<account::Account>(recipients[i]).guid_creation_num + 2 > MAX_U64;
}
```


<a id="@Specification_1_transfer"></a>

### Function `transfer`


```move
module 0x1::aptos_account {
    public entry fun transfer(source: &signer, to: address, amount: u64)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
    let account_addr_source = signer::address_of(source);
    requires account_addr_source != to;
    include CreateAccountTransferAbortsIf;
    include GuidAbortsIf<AptosCoin>;
    include WithdrawAbortsIf<AptosCoin>{from: source};
    include TransferEnsures<AptosCoin>;
    aborts_if exists<coin::CoinStore<AptosCoin>>(to) && global<coin::CoinStore<AptosCoin>>(to).frozen;
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    ensures exists<aptos_framework::account::Account>(to);
    ensures exists<coin::CoinStore<AptosCoin>>(to);
}
```


<a id="@Specification_1_batch_transfer_coins"></a>

### Function `batch_transfer_coins`


```move
module 0x1::aptos_account {
    public entry fun batch_transfer_coins<CoinType>(from: &signer, recipients: vector<address>, amounts: vector<u64>)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
    let account_addr_source = signer::address_of(from);
    let coin_store_source = global<coin::CoinStore<CoinType>>(account_addr_source);
    let balance_source = coin_store_source.coin.value;
    requires forall i in 0..len(recipients):
        recipients[i] != account_addr_source;
    requires exists i in 0..len(recipients):
        amounts[i] > 0;
// This enforces ### high&#45;level&#45;req&#45;7
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if len(recipients) != len(amounts);
    aborts_if exists i in 0..len(recipients):
            !account::exists_at(recipients[i]) && length_judgment(recipients[i]);
    aborts_if exists i in 0..len(recipients):
            !account::exists_at(recipients[i]) && (recipients[i] == @vm_reserved || recipients[i] == @aptos_framework || recipients[i] == @aptos_token);
    ensures forall i in 0..len(recipients):
            (!account::exists_at(recipients[i]) ==> !length_judgment(recipients[i])) &&
                (!account::exists_at(recipients[i]) ==> (recipients[i] != @vm_reserved && recipients[i] != @aptos_framework && recipients[i] != @aptos_token));
    aborts_if exists i in 0..len(recipients):
        !exists<coin::CoinStore<CoinType>>(account_addr_source);
    aborts_if exists i in 0..len(recipients):
        coin_store_source.frozen;
    aborts_if exists i in 0..len(recipients):
        global<coin::CoinStore<CoinType>>(account_addr_source).coin.value < amounts[i];
    aborts_if exists i in 0..len(recipients):
        exists<coin::CoinStore<CoinType>>(recipients[i]) && global<coin::CoinStore<CoinType>>(recipients[i]).frozen;
    aborts_if exists i in 0..len(recipients):
        account::exists_at(recipients[i]) && !exists<coin::CoinStore<CoinType>>(recipients[i]) && global<account::Account>(recipients[i]).guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
    aborts_if exists i in 0..len(recipients):
        account::exists_at(recipients[i]) && !exists<coin::CoinStore<CoinType>>(recipients[i]) && global<account::Account>(recipients[i]).guid_creation_num + 2 > MAX_U64;
    aborts_if exists i in 0..len(recipients):
        !coin::spec_is_account_registered<CoinType>(recipients[i]) && !type_info::spec_is_struct<CoinType>();
}
```


<a id="@Specification_1_transfer_coins"></a>

### Function `transfer_coins`


```move
module 0x1::aptos_account {
    public entry fun transfer_coins<CoinType>(from: &signer, to: address, amount: u64)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
    let account_addr_source = signer::address_of(from);
    requires account_addr_source != to;
    include CreateAccountTransferAbortsIf;
    include WithdrawAbortsIf<CoinType>;
    include GuidAbortsIf<CoinType>;
    include RegistCoinAbortsIf<CoinType>;
    include TransferEnsures<CoinType>;
    aborts_if exists<coin::CoinStore<CoinType>>(to) && global<coin::CoinStore<CoinType>>(to).frozen;
    ensures exists<aptos_framework::account::Account>(to);
    ensures exists<aptos_framework::coin::CoinStore<CoinType>>(to);
}
```


<a id="@Specification_1_deposit_coins"></a>

### Function `deposit_coins`


```move
module 0x1::aptos_account {
    public fun deposit_coins<CoinType>(to: address, coins: coin::Coin<CoinType>)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
    include CreateAccountTransferAbortsIf;
    include GuidAbortsIf<CoinType>;
    include RegistCoinAbortsIf<CoinType>;
    let if_exist_coin = exists<coin::CoinStore<CoinType>>(to);
    aborts_if if_exist_coin && global<coin::CoinStore<CoinType>>(to).frozen;
// This enforces ### high&#45;level&#45;spec&#45;6
[#high&#45;level&#45;req](high&#45;level requirement 6):
    ensures exists<aptos_framework::account::Account>(to);
    ensures exists<aptos_framework::coin::CoinStore<CoinType>>(to);
    let coin_store_to = global<coin::CoinStore<CoinType>>(to).coin.value;
    let post post_coin_store_to = global<coin::CoinStore<CoinType>>(to).coin.value;
    ensures if_exist_coin ==> post_coin_store_to == coin_store_to + coins.value;
}
```


<a id="@Specification_1_assert_account_exists"></a>

### Function `assert_account_exists`


```move
module 0x1::aptos_account {
    public fun assert_account_exists(addr: address)
}
```



```move
module 0x1::aptos_account {
    aborts_if !account::exists_at(addr);
}
```


<a id="@Specification_1_assert_account_is_registered_for_apt"></a>

### Function `assert_account_is_registered_for_apt`


```move
module 0x1::aptos_account {
    public fun assert_account_is_registered_for_apt(addr: address)
}
```

Check if the address existed.
Check if the AptosCoin under the address existed.


```move
module 0x1::aptos_account {
    pragma aborts_if_is_partial;
    aborts_if !account::exists_at(addr);
    aborts_if !coin::spec_is_account_registered<AptosCoin>(addr);
}
```


<a id="@Specification_1_set_allow_direct_coin_transfers"></a>

### Function `set_allow_direct_coin_transfers`


```move
module 0x1::aptos_account {
    public entry fun set_allow_direct_coin_transfers(account: &signer, allow: bool)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
}
```


<a id="@Specification_1_can_receive_direct_coin_transfers"></a>

### Function `can_receive_direct_coin_transfers`


```move
module 0x1::aptos_account {
    #[view]
    public fun can_receive_direct_coin_transfers(account: address): bool
}
```



```move
module 0x1::aptos_account {
    aborts_if false;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures result == (
        !exists<DirectTransferConfig>(account) ||
            global<DirectTransferConfig>(account).allow_arbitrary_coin_transfers
    );
}
```


<a id="@Specification_1_register_apt"></a>

### Function `register_apt`


```move
module 0x1::aptos_account {
    public(friend) fun register_apt(account_signer: &signer)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
}
```


<a id="@Specification_1_fungible_transfer_only"></a>

### Function `fungible_transfer_only`


```move
module 0x1::aptos_account {
    fun fungible_transfer_only(source: &signer, to: address, amount: u64)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
}
```


<a id="@Specification_1_is_fungible_balance_at_least"></a>

### Function `is_fungible_balance_at_least`


```move
module 0x1::aptos_account {
    public(friend) fun is_fungible_balance_at_least(account: address, amount: u64): bool
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
}
```


<a id="@Specification_1_burn_from_fungible_store"></a>

### Function `burn_from_fungible_store`


```move
module 0x1::aptos_account {
    public(friend) fun burn_from_fungible_store(ref: &fungible_asset::BurnRef, account: address, amount: u64)
}
```



```move
module 0x1::aptos_account {
    pragma verify = false;
}
```



<a id="0x1_aptos_account_CreateAccountTransferAbortsIf"></a>


```move
module 0x1::aptos_account {
    schema CreateAccountTransferAbortsIf {
        to: address;
        aborts_if !account::exists_at(to) && length_judgment(to);
        aborts_if !account::exists_at(to) && (to == @vm_reserved || to == @aptos_framework || to == @aptos_token);
    }
}
```



<a id="0x1_aptos_account_WithdrawAbortsIf"></a>


```move
module 0x1::aptos_account {
    schema WithdrawAbortsIf<CoinType> {
        from: &signer;
        amount: u64;
        let account_addr_source = signer::address_of(from);
        let coin_store_source = global<coin::CoinStore<CoinType>>(account_addr_source);
        let balance_source = coin_store_source.coin.value;
        aborts_if !exists<coin::CoinStore<CoinType>>(account_addr_source);
        aborts_if coin_store_source.frozen;
        aborts_if balance_source < amount;
    }
}
```



<a id="0x1_aptos_account_GuidAbortsIf"></a>


```move
module 0x1::aptos_account {
    schema GuidAbortsIf<CoinType> {
        to: address;
        let acc = global<account::Account>(to);
        aborts_if account::exists_at(to) && !exists<coin::CoinStore<CoinType>>(to) && acc.guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
        aborts_if account::exists_at(to) && !exists<coin::CoinStore<CoinType>>(to) && acc.guid_creation_num + 2 > MAX_U64;
    }
}
```



<a id="0x1_aptos_account_RegistCoinAbortsIf"></a>


```move
module 0x1::aptos_account {
    schema RegistCoinAbortsIf<CoinType> {
        to: address;
        aborts_if !coin::spec_is_account_registered<CoinType>(to) && !type_info::spec_is_struct<CoinType>();
        aborts_if exists<aptos_framework::account::Account>(to);
        aborts_if type_info::type_of<CoinType>() != type_info::type_of<AptosCoin>();
    }
}
```



<a id="0x1_aptos_account_TransferEnsures"></a>


```move
module 0x1::aptos_account {
    schema TransferEnsures<CoinType> {
        to: address;
        account_addr_source: address;
        amount: u64;
        let if_exist_account = exists<account::Account>(to);
        let if_exist_coin = exists<coin::CoinStore<CoinType>>(to);
        let coin_store_to = global<coin::CoinStore<CoinType>>(to);
        let coin_store_source = global<coin::CoinStore<CoinType>>(account_addr_source);
        let post p_coin_store_to = global<coin::CoinStore<CoinType>>(to);
        let post p_coin_store_source = global<coin::CoinStore<CoinType>>(account_addr_source);
        ensures coin_store_source.coin.value - amount == p_coin_store_source.coin.value;
        ensures if_exist_account && if_exist_coin ==> coin_store_to.coin.value + amount == p_coin_store_to.coin.value;
    }
}
```
