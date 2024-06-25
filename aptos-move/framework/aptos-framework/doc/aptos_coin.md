
<a id="0x1_aptos_coin"></a>

# Module `0x1::aptos_coin`

This module defines a minimal and generic Coin and Balance.
modified from https://github.com/move&#45;language/move/tree/main/language/documentation/tutorial


-  [Resource `AptosCoin`](#0x1_aptos_coin_AptosCoin)
-  [Resource `MintCapStore`](#0x1_aptos_coin_MintCapStore)
-  [Struct `DelegatedMintCapability`](#0x1_aptos_coin_DelegatedMintCapability)
-  [Resource `Delegations`](#0x1_aptos_coin_Delegations)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_aptos_coin_initialize)
-  [Function `has_mint_capability`](#0x1_aptos_coin_has_mint_capability)
-  [Function `destroy_mint_cap`](#0x1_aptos_coin_destroy_mint_cap)
-  [Function `configure_accounts_for_test`](#0x1_aptos_coin_configure_accounts_for_test)
-  [Function `mint`](#0x1_aptos_coin_mint)
-  [Function `delegate_mint_capability`](#0x1_aptos_coin_delegate_mint_capability)
-  [Function `claim_mint_capability`](#0x1_aptos_coin_claim_mint_capability)
-  [Function `find_delegation`](#0x1_aptos_coin_find_delegation)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `destroy_mint_cap`](#@Specification_1_destroy_mint_cap)
    -  [Function `configure_accounts_for_test`](#@Specification_1_configure_accounts_for_test)
    -  [Function `mint`](#@Specification_1_mint)
    -  [Function `delegate_mint_capability`](#@Specification_1_delegate_mint_capability)
    -  [Function `claim_mint_capability`](#@Specification_1_claim_mint_capability)
    -  [Function `find_delegation`](#@Specification_1_find_delegation)


```move
module 0x1::aptos_coin {
    use 0x1::coin;
    use 0x1::error;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::vector;
}
```


<a id="0x1_aptos_coin_AptosCoin"></a>

## Resource `AptosCoin`



```move
module 0x1::aptos_coin {
    struct AptosCoin has key
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


<a id="0x1_aptos_coin_MintCapStore"></a>

## Resource `MintCapStore`



```move
module 0x1::aptos_coin {
    struct MintCapStore has key
}
```


##### Fields


<dl>
<dt>
`mint_cap: coin::MintCapability<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_coin_DelegatedMintCapability"></a>

## Struct `DelegatedMintCapability`

Delegation token created by delegator and can be claimed by the delegatee as MintCapability.


```move
module 0x1::aptos_coin {
    struct DelegatedMintCapability has store
}
```


##### Fields


<dl>
<dt>
`to: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_aptos_coin_Delegations"></a>

## Resource `Delegations`

The container stores the current pending delegations.


```move
module 0x1::aptos_coin {
    struct Delegations has key
}
```


##### Fields


<dl>
<dt>
`inner: vector<aptos_coin::DelegatedMintCapability>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aptos_coin_EALREADY_DELEGATED"></a>

Mint capability has already been delegated to this specified address


```move
module 0x1::aptos_coin {
    const EALREADY_DELEGATED: u64 = 2;
}
```


<a id="0x1_aptos_coin_EDELEGATION_NOT_FOUND"></a>

Cannot find delegation of mint capability to this account


```move
module 0x1::aptos_coin {
    const EDELEGATION_NOT_FOUND: u64 = 3;
}
```


<a id="0x1_aptos_coin_ENO_CAPABILITIES"></a>

Account does not have mint capability


```move
module 0x1::aptos_coin {
    const ENO_CAPABILITIES: u64 = 1;
}
```


<a id="0x1_aptos_coin_initialize"></a>

## Function `initialize`

Can only called during genesis to initialize the Aptos coin.


```move
module 0x1::aptos_coin {
    public(friend) fun initialize(aptos_framework: &signer): (coin::BurnCapability<aptos_coin::AptosCoin>, coin::MintCapability<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public(friend) fun initialize(aptos_framework: &signer): (BurnCapability<AptosCoin>, MintCapability<AptosCoin>) {
        system_addresses::assert_aptos_framework(aptos_framework);

        let (burn_cap, freeze_cap, mint_cap) = coin::initialize_with_parallelizable_supply<AptosCoin>(
            aptos_framework,
            string::utf8(b"Aptos Coin"),
            string::utf8(b"APT"),
            8, // decimals
            true, // monitor_supply
        );

        // Aptos framework needs mint cap to mint coins to initial validators. This will be revoked once the validators
        // have been initialized.
        move_to(aptos_framework, MintCapStore { mint_cap });

        coin::destroy_freeze_cap(freeze_cap);
        (burn_cap, mint_cap)
    }
}
```


<a id="0x1_aptos_coin_has_mint_capability"></a>

## Function `has_mint_capability`



```move
module 0x1::aptos_coin {
    public fun has_mint_capability(account: &signer): bool
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public fun has_mint_capability(account: &signer): bool {
        exists<MintCapStore>(signer::address_of(account))
    }
}
```


<a id="0x1_aptos_coin_destroy_mint_cap"></a>

## Function `destroy_mint_cap`

Only called during genesis to destroy the aptos framework account&apos;s mint capability once all initial validators
and accounts have been initialized during genesis.


```move
module 0x1::aptos_coin {
    public(friend) fun destroy_mint_cap(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public(friend) fun destroy_mint_cap(aptos_framework: &signer) acquires MintCapStore {
        system_addresses::assert_aptos_framework(aptos_framework);
        let MintCapStore { mint_cap } = move_from<MintCapStore>(@aptos_framework);
        coin::destroy_mint_cap(mint_cap);
    }
}
```


<a id="0x1_aptos_coin_configure_accounts_for_test"></a>

## Function `configure_accounts_for_test`

Can only be called during genesis for tests to grant mint capability to aptos framework and core resources
accounts.
Expects account and APT store to be registered before calling.


```move
module 0x1::aptos_coin {
    public(friend) fun configure_accounts_for_test(aptos_framework: &signer, core_resources: &signer, mint_cap: coin::MintCapability<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public(friend) fun configure_accounts_for_test(
        aptos_framework: &signer,
        core_resources: &signer,
        mint_cap: MintCapability<AptosCoin>,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        // Mint the core resource account AptosCoin for gas so it can execute system transactions.
        let coins = coin::mint<AptosCoin>(
            18446744073709551615,
            &mint_cap,
        );
        coin::deposit<AptosCoin>(signer::address_of(core_resources), coins);

        move_to(core_resources, MintCapStore { mint_cap });
        move_to(core_resources, Delegations { inner: vector::empty() });
    }
}
```


<a id="0x1_aptos_coin_mint"></a>

## Function `mint`

Only callable in tests and testnets where the core resources account exists.
Create new coins and deposit them into dst_addr&apos;s account.


```move
module 0x1::aptos_coin {
    public entry fun mint(account: &signer, dst_addr: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public entry fun mint(
        account: &signer,
        dst_addr: address,
        amount: u64,
    ) acquires MintCapStore {
        let account_addr = signer::address_of(account);

        assert!(
            exists<MintCapStore>(account_addr),
            error::not_found(ENO_CAPABILITIES),
        );

        let mint_cap = &borrow_global<MintCapStore>(account_addr).mint_cap;
        let coins_minted = coin::mint<AptosCoin>(amount, mint_cap);
        coin::deposit<AptosCoin>(dst_addr, coins_minted);
    }
}
```


<a id="0x1_aptos_coin_delegate_mint_capability"></a>

## Function `delegate_mint_capability`

Only callable in tests and testnets where the core resources account exists.
Create delegated token for the address so the account could claim MintCapability later.


```move
module 0x1::aptos_coin {
    public entry fun delegate_mint_capability(account: signer, to: address)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public entry fun delegate_mint_capability(account: signer, to: address) acquires Delegations {
        system_addresses::assert_core_resource(&account);
        let delegations = &mut borrow_global_mut<Delegations>(@core_resources).inner;
        vector::for_each_ref(delegations, |element| {
            let element: &DelegatedMintCapability = element;
            assert!(element.to != to, error::invalid_argument(EALREADY_DELEGATED));
        });
        vector::push_back(delegations, DelegatedMintCapability { to });
    }
}
```


<a id="0x1_aptos_coin_claim_mint_capability"></a>

## Function `claim_mint_capability`

Only callable in tests and testnets where the core resources account exists.
Claim the delegated mint capability and destroy the delegated token.


```move
module 0x1::aptos_coin {
    public entry fun claim_mint_capability(account: &signer)
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    public entry fun claim_mint_capability(account: &signer) acquires Delegations, MintCapStore {
        let maybe_index = find_delegation(signer::address_of(account));
        assert!(option::is_some(&maybe_index), EDELEGATION_NOT_FOUND);
        let idx = *option::borrow(&maybe_index);
        let delegations = &mut borrow_global_mut<Delegations>(@core_resources).inner;
        let DelegatedMintCapability { to: _ } = vector::swap_remove(delegations, idx);

        // Make a copy of mint cap and give it to the specified account.
        let mint_cap = borrow_global<MintCapStore>(@core_resources).mint_cap;
        move_to(account, MintCapStore { mint_cap });
    }
}
```


<a id="0x1_aptos_coin_find_delegation"></a>

## Function `find_delegation`



```move
module 0x1::aptos_coin {
    fun find_delegation(addr: address): option::Option<u64>
}
```


##### Implementation


```move
module 0x1::aptos_coin {
    fun find_delegation(addr: address): Option<u64> acquires Delegations {
        let delegations = &borrow_global<Delegations>(@core_resources).inner;
        let i = 0;
        let len = vector::length(delegations);
        let index = option::none();
        while (i < len) {
            let element = vector::borrow(delegations, i);
            if (element.to == addr) {
                index = option::some(i);
                break
            };
            i = i + 1;
        };
        index
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
<td>The native token, APT, must be initialized during genesis.</td>
<td>Medium</td>
<td>The initialize function is only called once, during genesis.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>The APT coin may only be created exactly once.</td>
<td>Medium</td>
<td>The initialization function may only be called once.</td>
<td>Enforced through the [https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/aptos&#45;move/framework/aptos&#45;framework/sources/coin.move](coin) module, which has been audited.</td>
</tr>

<tr>
<td>4</td>
<td>Any type of operation on the APT coin should fail if the user has not registered for the coin.</td>
<td>Medium</td>
<td>Coin operations may succeed only on valid user coin registration.</td>
<td>Enforced through the [https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/aptos&#45;move/framework/aptos&#45;framework/sources/coin.move](coin) module, which has been audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::aptos_coin {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::aptos_coin {
    public(friend) fun initialize(aptos_framework: &signer): (coin::BurnCapability<aptos_coin::AptosCoin>, coin::MintCapability<aptos_coin::AptosCoin>)
}
```



```move
module 0x1::aptos_coin {
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if !string::spec_internal_check_utf8(b"Aptos Coin");
    aborts_if !string::spec_internal_check_utf8(b"APT");
    aborts_if exists<MintCapStore>(addr);
    aborts_if exists<coin::CoinInfo<AptosCoin>>(addr);
    aborts_if !exists<aggregator_factory::AggregatorFactory>(addr);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<MintCapStore>(addr);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures global<MintCapStore>(addr).mint_cap ==  MintCapability<AptosCoin> {};
    ensures exists<coin::CoinInfo<AptosCoin>>(addr);
    ensures result_1 == BurnCapability<AptosCoin> {};
    ensures result_2 == MintCapability<AptosCoin> {};
}
```


<a id="@Specification_1_destroy_mint_cap"></a>

### Function `destroy_mint_cap`


```move
module 0x1::aptos_coin {
    public(friend) fun destroy_mint_cap(aptos_framework: &signer)
}
```



```move
module 0x1::aptos_coin {
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if !exists<MintCapStore>(@aptos_framework);
}
```


<a id="@Specification_1_configure_accounts_for_test"></a>

### Function `configure_accounts_for_test`


```move
module 0x1::aptos_coin {
    public(friend) fun configure_accounts_for_test(aptos_framework: &signer, core_resources: &signer, mint_cap: coin::MintCapability<aptos_coin::AptosCoin>)
}
```



```move
module 0x1::aptos_coin {
    pragma verify = false;
}
```


<a id="@Specification_1_mint"></a>

### Function `mint`


```move
module 0x1::aptos_coin {
    public entry fun mint(account: &signer, dst_addr: address, amount: u64)
}
```



```move
module 0x1::aptos_coin {
    pragma verify = false;
}
```


<a id="@Specification_1_delegate_mint_capability"></a>

### Function `delegate_mint_capability`


```move
module 0x1::aptos_coin {
    public entry fun delegate_mint_capability(account: signer, to: address)
}
```



```move
module 0x1::aptos_coin {
    pragma verify = false;
}
```


<a id="@Specification_1_claim_mint_capability"></a>

### Function `claim_mint_capability`


```move
module 0x1::aptos_coin {
    public entry fun claim_mint_capability(account: &signer)
}
```



```move
module 0x1::aptos_coin {
    pragma verify = false;
}
```


<a id="@Specification_1_find_delegation"></a>

### Function `find_delegation`


```move
module 0x1::aptos_coin {
    fun find_delegation(addr: address): option::Option<u64>
}
```



```move
module 0x1::aptos_coin {
    aborts_if !exists<Delegations>(@core_resources);
}
```



<a id="0x1_aptos_coin_ExistsAptosCoin"></a>


```move
module 0x1::aptos_coin {
    schema ExistsAptosCoin {
        requires exists<coin::CoinInfo<AptosCoin>>(@aptos_framework);
    }
}
```
