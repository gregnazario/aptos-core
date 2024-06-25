
<a id="0x1_managed_coin"></a>

# Module `0x1::managed_coin`

ManagedCoin is built to make a simple walkthrough of the Coins module.
It contains scripts you will need to initialize, mint, burn, transfer coins.
By utilizing this current module, a developer can create his own coin and care less about mint and burn capabilities,


-  [Resource `Capabilities`](#0x1_managed_coin_Capabilities)
-  [Constants](#@Constants_0)
-  [Function `burn`](#0x1_managed_coin_burn)
-  [Function `initialize`](#0x1_managed_coin_initialize)
-  [Function `mint`](#0x1_managed_coin_mint)
-  [Function `register`](#0x1_managed_coin_register)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `burn`](#@Specification_1_burn)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `mint`](#@Specification_1_mint)
    -  [Function `register`](#@Specification_1_register)


```move
module 0x1::managed_coin {
    use 0x1::coin;
    use 0x1::error;
    use 0x1::signer;
    use 0x1::string;
}
```


<a id="0x1_managed_coin_Capabilities"></a>

## Resource `Capabilities`

Capabilities resource storing mint and burn capabilities.
The resource is stored on the account that initialized coin `CoinType`.


```move
module 0x1::managed_coin {
    struct Capabilities<CoinType> has key
}
```


##### Fields


<dl>
<dt>
`burn_cap: coin::BurnCapability<CoinType>`
</dt>
<dd>

</dd>
<dt>
`freeze_cap: coin::FreezeCapability<CoinType>`
</dt>
<dd>

</dd>
<dt>
`mint_cap: coin::MintCapability<CoinType>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_managed_coin_ENO_CAPABILITIES"></a>

Account has no capabilities (burn/mint).


```move
module 0x1::managed_coin {
    const ENO_CAPABILITIES: u64 = 1;
}
```


<a id="0x1_managed_coin_burn"></a>

## Function `burn`

Withdraw an `amount` of coin `CoinType` from `account` and burn it.


```move
module 0x1::managed_coin {
    public entry fun burn<CoinType>(account: &signer, amount: u64)
}
```


##### Implementation


```move
module 0x1::managed_coin {
    public entry fun burn<CoinType>(
        account: &signer,
        amount: u64,
    ) acquires Capabilities {
        let account_addr = signer::address_of(account);

        assert!(
            exists<Capabilities<CoinType>>(account_addr),
            error::not_found(ENO_CAPABILITIES),
        );

        let capabilities = borrow_global<Capabilities<CoinType>>(account_addr);

        let to_burn = coin::withdraw<CoinType>(account, amount);
        coin::burn(to_burn, &capabilities.burn_cap);
    }
}
```


<a id="0x1_managed_coin_initialize"></a>

## Function `initialize`

Initialize new coin `CoinType` in Aptos Blockchain.
Mint and Burn Capabilities will be stored under `account` in `Capabilities` resource.


```move
module 0x1::managed_coin {
    public entry fun initialize<CoinType>(account: &signer, name: vector<u8>, symbol: vector<u8>, decimals: u8, monitor_supply: bool)
}
```


##### Implementation


```move
module 0x1::managed_coin {
    public entry fun initialize<CoinType>(
        account: &signer,
        name: vector<u8>,
        symbol: vector<u8>,
        decimals: u8,
        monitor_supply: bool,
    ) {
        let (burn_cap, freeze_cap, mint_cap) = coin::initialize<CoinType>(
            account,
            string::utf8(name),
            string::utf8(symbol),
            decimals,
            monitor_supply,
        );

        move_to(account, Capabilities<CoinType> {
            burn_cap,
            freeze_cap,
            mint_cap,
        });
    }
}
```


<a id="0x1_managed_coin_mint"></a>

## Function `mint`

Create new coins `CoinType` and deposit them into dst_addr&apos;s account.


```move
module 0x1::managed_coin {
    public entry fun mint<CoinType>(account: &signer, dst_addr: address, amount: u64)
}
```


##### Implementation


```move
module 0x1::managed_coin {
    public entry fun mint<CoinType>(
        account: &signer,
        dst_addr: address,
        amount: u64,
    ) acquires Capabilities {
        let account_addr = signer::address_of(account);

        assert!(
            exists<Capabilities<CoinType>>(account_addr),
            error::not_found(ENO_CAPABILITIES),
        );

        let capabilities = borrow_global<Capabilities<CoinType>>(account_addr);
        let coins_minted = coin::mint(amount, &capabilities.mint_cap);
        coin::deposit(dst_addr, coins_minted);
    }
}
```


<a id="0x1_managed_coin_register"></a>

## Function `register`

Creating a resource that stores balance of `CoinType` on user&apos;s account, withdraw and deposit event handlers.
Required if user wants to start accepting deposits of `CoinType` in his account.


```move
module 0x1::managed_coin {
    public entry fun register<CoinType>(account: &signer)
}
```


##### Implementation


```move
module 0x1::managed_coin {
    public entry fun register<CoinType>(account: &signer) {
        coin::register<CoinType>(account);
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
<td>The initializing account should hold the capabilities to operate the coin.</td>
<td>Critical</td>
<td>The capabilities are stored under the initializing account under the Capabilities resource, which is distinct for a distinct type of coin.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;1](initialize).</td>
</tr>

<tr>
<td>2</td>
<td>A new coin should be properly initialized.</td>
<td>High</td>
<td>In the initialize function, a new coin is initialized via the coin module with the specified properties.</td>
<td>Enforced via [coin.md#high&#45;level&#45;req&#45;2](initialize_internal).</td>
</tr>

<tr>
<td>3</td>
<td>Minting/Burning should only be done by the account who hold the valid capabilities.</td>
<td>High</td>
<td>The mint and burn capabilities are moved under the initializing account and retrieved, while minting/burning</td>
<td>Enforced via: [#high&#45;level&#45;req&#45;3.1](initialize), [#high&#45;level&#45;req&#45;3.2](burn), [#high&#45;level&#45;req&#45;3.3](mint).</td>
</tr>

<tr>
<td>4</td>
<td>If the total supply of coins is being monitored, burn and mint operations will appropriately adjust the total supply.</td>
<td>High</td>
<td>The coin::burn and coin::mint functions, when tracking the supply, adjusts the total coin supply accordingly.</td>
<td>Enforced via [coin.md#high&#45;level&#45;req&#45;4](TotalSupplyNoChange).</td>
</tr>

<tr>
<td>5</td>
<td>Before burning coins, exact amount of coins are withdrawn.</td>
<td>High</td>
<td>After utilizing the coin::withdraw function to withdraw coins, they are then burned, and the function ensures the precise return of the initially specified coin amount.</td>
<td>Enforced via [coin.md#high&#45;level&#45;req&#45;5](burn_from).</td>
</tr>

<tr>
<td>6</td>
<td>Minted coins are deposited to the provided destination address.</td>
<td>High</td>
<td>After the coins are minted via coin::mint they are deposited into the coinstore of the destination address.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;6](mint).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::managed_coin {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_burn"></a>

### Function `burn`


```move
module 0x1::managed_coin {
    public entry fun burn<CoinType>(account: &signer, amount: u64)
}
```



```move
module 0x1::managed_coin {
    pragma verify = false;
    let account_addr = signer::address_of(account);
    aborts_if !exists<Capabilities<CoinType>>(account_addr);
    let coin_store = global<coin::CoinStore<CoinType>>(account_addr);
    let balance = coin_store.coin.value;
// This enforces ### high&#45;level&#45;req&#45;3.2
[#high&#45;level&#45;req](high&#45;level requirement 3) and ### high&#45;level&#45;req&#45;4.1
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if !exists<coin::CoinStore<CoinType>>(account_addr);
    aborts_if coin_store.frozen;
    aborts_if balance < amount;
    let addr =  type_info::type_of<CoinType>().account_address;
    let maybe_supply = global<coin::CoinInfo<CoinType>>(addr).supply;
    aborts_if amount == 0;
    aborts_if !exists<coin::CoinInfo<CoinType>>(addr);
    include coin::CoinSubAbortsIf<CoinType> { amount:amount };
    ensures coin::supply<CoinType> == old(coin::supply<CoinType>) - amount;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::managed_coin {
    public entry fun initialize<CoinType>(account: &signer, name: vector<u8>, symbol: vector<u8>, decimals: u8, monitor_supply: bool)
}
```

Make sure `name` and `symbol` are legal length.
Only the creator of `CoinType` can initialize.
The &apos;name&apos; and &apos;symbol&apos; should be valid utf8 bytes
The Capabilities&lt;CoinType&gt; should not be under the signer before creating;
The Capabilities&lt;CoinType&gt; should be under the signer after creating;


```move
module 0x1::managed_coin {
    include coin::InitializeInternalSchema<CoinType>;
    aborts_if !string::spec_internal_check_utf8(name);
    aborts_if !string::spec_internal_check_utf8(symbol);
    aborts_if exists<Capabilities<CoinType>>(signer::address_of(account));
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1) and ### high&#45;level&#45;req&#45;3.1
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures exists<Capabilities<CoinType>>(signer::address_of(account));
}
```


<a id="@Specification_1_mint"></a>

### Function `mint`


```move
module 0x1::managed_coin {
    public entry fun mint<CoinType>(account: &signer, dst_addr: address, amount: u64)
}
```

The Capabilities&lt;CoinType&gt; should not exist in the signer address.
The `dst_addr` should not be frozen.


```move
module 0x1::managed_coin {
    pragma verify = false;
    let account_addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;3.3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if !exists<Capabilities<CoinType>>(account_addr);
    let addr = type_info::type_of<CoinType>().account_address;
    aborts_if (amount != 0) && !exists<coin::CoinInfo<CoinType>>(addr);
    let coin_store = global<coin::CoinStore<CoinType>>(dst_addr);
    aborts_if !exists<coin::CoinStore<CoinType>>(dst_addr);
    aborts_if coin_store.frozen;
    include coin::CoinAddAbortsIf<CoinType>;
    ensures coin::supply<CoinType> == old(coin::supply<CoinType>) + amount;
// This enforces ### high&#45;level&#45;req&#45;6
[#high&#45;level&#45;req](high&#45;level requirement 6):
    ensures global<coin::CoinStore<CoinType>>(dst_addr).coin.value == old(global<coin::CoinStore<CoinType>>(dst_addr)).coin.value + amount;
}
```


<a id="@Specification_1_register"></a>

### Function `register`


```move
module 0x1::managed_coin {
    public entry fun register<CoinType>(account: &signer)
}
```

An account can only be registered once.
Updating `Account.guid_creation_num` will not overflow.


```move
module 0x1::managed_coin {
    pragma verify = false;
    let account_addr = signer::address_of(account);
    let acc = global<account::Account>(account_addr);
    aborts_if !exists<coin::CoinStore<CoinType>>(account_addr) && acc.guid_creation_num + 2 >= account::MAX_GUID_CREATION_NUM;
    aborts_if !exists<coin::CoinStore<CoinType>>(account_addr) && acc.guid_creation_num + 2 > MAX_U64;
    aborts_if !exists<coin::CoinStore<CoinType>>(account_addr) && !exists<account::Account>(account_addr);
    aborts_if !exists<coin::CoinStore<CoinType>>(account_addr) && !type_info::spec_is_struct<CoinType>();
    ensures exists<coin::CoinStore<CoinType>>(account_addr);
}
```
