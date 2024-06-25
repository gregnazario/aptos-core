
<a id="0x1_resource_account"></a>

# Module `0x1::resource_account`

A resource account is used to manage resources independent of an account managed by a user.
This contains several utilities to make using resource accounts more effective.


<a id="@Resource_Accounts_to_manage_liquidity_pools_0"></a>

### Resource Accounts to manage liquidity pools


A dev wishing to use resource accounts for a liquidity pool, would likely do the following:

1. Create a new account using `resource_account::create_resource_account`. This creates the
account, stores the `signer_cap` within a `resource_account::Container`, and rotates the key to
the current account&apos;s authentication key or a provided authentication key.
2. Define the liquidity pool module&apos;s address to be the same as the resource account.
3. Construct a package&#45;publishing transaction for the resource account using the
authentication key used in step 1.
4. In the liquidity pool module&apos;s `init_module` function, call `retrieve_resource_account_cap`
which will retrieve the `signer_cap` and rotate the resource account&apos;s authentication key to
`0x0`, effectively locking it off.
5. When adding a new coin, the liquidity pool will load the capability and hence the `signer` to
register and store new `LiquidityCoin` resources.

Code snippets to help:

```
fun init_module(resource_account: &amp;signer) &#123;
let dev_address &#61; @DEV_ADDR;
let signer_cap &#61; retrieve_resource_account_cap(resource_account, dev_address);
let lp &#61; LiquidityPoolInfo &#123; signer_cap: signer_cap, ... &#125;;
move_to(resource_account, lp);
&#125;
```

Later on during a coin registration:
```
public fun add_coin&lt;X, Y&gt;(lp: &amp;LP, x: Coin&lt;x&gt;, y: Coin&lt;y&gt;) &#123;
if(!exists&lt;LiquidityCoin&lt;X, Y&gt;(LP::Address(lp), LiquidityCoin&lt;X, Y&gt;)) &#123;
let mint, burn &#61; Coin::initialize&lt;LiquidityCoin&lt;X, Y&gt;&gt;(...);
move_to(&amp;create_signer_with_capability(&amp;lp.cap), LiquidityCoin&lt;X, Y&gt;&#123; mint, burn &#125;);
&#125;
...
&#125;
```

<a id="@Resource_accounts_to_manage_an_account_for_module_publishing_(i.e.,_contract_account)_1"></a>

### Resource accounts to manage an account for module publishing (i.e., contract account)


A dev wishes to have an account dedicated to managing a contract. The contract itself does not
require signer post initialization. The dev could do the following:
1. Create a new account using `resource_account::create_resource_account_and_publish_package`.
This creates the account and publishes the package for that account.
2. At a later point in time, the account creator can move the signer capability to the module.

```
struct MyModuleResource has key &#123;
...
resource_signer_cap: Option&lt;SignerCapability&gt;,
&#125;

public fun provide_signer_capability(resource_signer_cap: SignerCapability) &#123;
let account_addr &#61; account::get_signer_capability_address(resource_signer_cap);
let resource_addr &#61; type_info::account_address(&amp;type_info::type_of&lt;MyModuleResource&gt;());
assert!(account_addr &#61;&#61; resource_addr, EADDRESS_MISMATCH);
let module &#61; borrow_global_mut&lt;MyModuleResource&gt;(account_addr);
module.resource_signer_cap &#61; option::some(resource_signer_cap);
&#125;
```


    -  [Resource Accounts to manage liquidity pools](#@Resource_Accounts_to_manage_liquidity_pools_0)
    -  [Resource accounts to manage an account for module publishing (i.e., contract account)](#@Resource_accounts_to_manage_an_account_for_module_publishing_(i.e.,_contract_account)_1)
-  [Resource `Container`](#0x1_resource_account_Container)
-  [Constants](#@Constants_2)
-  [Function `create_resource_account`](#0x1_resource_account_create_resource_account)
-  [Function `create_resource_account_and_fund`](#0x1_resource_account_create_resource_account_and_fund)
-  [Function `create_resource_account_and_publish_package`](#0x1_resource_account_create_resource_account_and_publish_package)
-  [Function `rotate_account_authentication_key_and_store_capability`](#0x1_resource_account_rotate_account_authentication_key_and_store_capability)
-  [Function `retrieve_resource_account_cap`](#0x1_resource_account_retrieve_resource_account_cap)
-  [Specification](#@Specification_3)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `create_resource_account`](#@Specification_3_create_resource_account)
    -  [Function `create_resource_account_and_fund`](#@Specification_3_create_resource_account_and_fund)
    -  [Function `create_resource_account_and_publish_package`](#@Specification_3_create_resource_account_and_publish_package)
    -  [Function `rotate_account_authentication_key_and_store_capability`](#@Specification_3_rotate_account_authentication_key_and_store_capability)
    -  [Function `retrieve_resource_account_cap`](#@Specification_3_retrieve_resource_account_cap)


```move
module 0x1::resource_account {
    use 0x1::account;
    use 0x1::aptos_coin;
    use 0x1::code;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::signer;
    use 0x1::simple_map;
    use 0x1::vector;
}
```


<a id="0x1_resource_account_Container"></a>

## Resource `Container`



```move
module 0x1::resource_account {
    struct Container has key
}
```


##### Fields


<dl>
<dt>
`store: simple_map::SimpleMap<address, account::SignerCapability>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_2"></a>

## Constants


<a id="0x1_resource_account_ZERO_AUTH_KEY"></a>



```move
module 0x1::resource_account {
    const ZERO_AUTH_KEY: vector<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
}
```


<a id="0x1_resource_account_ECONTAINER_NOT_PUBLISHED"></a>

Container resource not found in account


```move
module 0x1::resource_account {
    const ECONTAINER_NOT_PUBLISHED: u64 = 1;
}
```


<a id="0x1_resource_account_EUNAUTHORIZED_NOT_OWNER"></a>

The resource account was not created by the specified source account


```move
module 0x1::resource_account {
    const EUNAUTHORIZED_NOT_OWNER: u64 = 2;
}
```


<a id="0x1_resource_account_create_resource_account"></a>

## Function `create_resource_account`

Creates a new resource account and rotates the authentication key to either
the optional auth key if it is non&#45;empty (though auth keys are 32&#45;bytes)
or the source accounts current auth key.


```move
module 0x1::resource_account {
    public entry fun create_resource_account(origin: &signer, seed: vector<u8>, optional_auth_key: vector<u8>)
}
```


##### Implementation


```move
module 0x1::resource_account {
    public entry fun create_resource_account(
        origin: &signer,
        seed: vector<u8>,
        optional_auth_key: vector<u8>,
    ) acquires Container {
        let (resource, resource_signer_cap) = account::create_resource_account(origin, seed);
        rotate_account_authentication_key_and_store_capability(
            origin,
            resource,
            resource_signer_cap,
            optional_auth_key,
        );
    }
}
```


<a id="0x1_resource_account_create_resource_account_and_fund"></a>

## Function `create_resource_account_and_fund`

Creates a new resource account, transfer the amount of coins from the origin to the resource
account, and rotates the authentication key to either the optional auth key if it is
non&#45;empty (though auth keys are 32&#45;bytes) or the source accounts current auth key. Note,
this function adds additional resource ownership to the resource account and should only be
used for resource accounts that need access to `Coin<AptosCoin>`.


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_fund(origin: &signer, seed: vector<u8>, optional_auth_key: vector<u8>, fund_amount: u64)
}
```


##### Implementation


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_fund(
        origin: &signer,
        seed: vector<u8>,
        optional_auth_key: vector<u8>,
        fund_amount: u64,
    ) acquires Container {
        let (resource, resource_signer_cap) = account::create_resource_account(origin, seed);
        coin::register<AptosCoin>(&resource);
        coin::transfer<AptosCoin>(origin, signer::address_of(&resource), fund_amount);
        rotate_account_authentication_key_and_store_capability(
            origin,
            resource,
            resource_signer_cap,
            optional_auth_key,
        );
    }
}
```


<a id="0x1_resource_account_create_resource_account_and_publish_package"></a>

## Function `create_resource_account_and_publish_package`

Creates a new resource account, publishes the package under this account transaction under
this account and leaves the signer cap readily available for pickup.


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_publish_package(origin: &signer, seed: vector<u8>, metadata_serialized: vector<u8>, code: vector<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_publish_package(
        origin: &signer,
        seed: vector<u8>,
        metadata_serialized: vector<u8>,
        code: vector<vector<u8>>,
    ) acquires Container {
        let (resource, resource_signer_cap) = account::create_resource_account(origin, seed);
        aptos_framework::code::publish_package_txn(&resource, metadata_serialized, code);
        rotate_account_authentication_key_and_store_capability(
            origin,
            resource,
            resource_signer_cap,
            ZERO_AUTH_KEY,
        );
    }
}
```


<a id="0x1_resource_account_rotate_account_authentication_key_and_store_capability"></a>

## Function `rotate_account_authentication_key_and_store_capability`



```move
module 0x1::resource_account {
    fun rotate_account_authentication_key_and_store_capability(origin: &signer, resource: signer, resource_signer_cap: account::SignerCapability, optional_auth_key: vector<u8>)
}
```


##### Implementation


```move
module 0x1::resource_account {
    fun rotate_account_authentication_key_and_store_capability(
        origin: &signer,
        resource: signer,
        resource_signer_cap: account::SignerCapability,
        optional_auth_key: vector<u8>,
    ) acquires Container {
        let origin_addr = signer::address_of(origin);
        if (!exists<Container>(origin_addr)) {
            move_to(origin, Container { store: simple_map::create() })
        };

        let container = borrow_global_mut<Container>(origin_addr);
        let resource_addr = signer::address_of(&resource);
        simple_map::add(&mut container.store, resource_addr, resource_signer_cap);

        let auth_key = if (vector::is_empty(&optional_auth_key)) {
            account::get_authentication_key(origin_addr)
        } else {
            optional_auth_key
        };
        account::rotate_authentication_key_internal(&resource, auth_key);
    }
}
```


<a id="0x1_resource_account_retrieve_resource_account_cap"></a>

## Function `retrieve_resource_account_cap`

When called by the resource account, it will retrieve the capability associated with that
account and rotate the account&apos;s auth key to 0x0 making the account inaccessible without
the SignerCapability.


```move
module 0x1::resource_account {
    public fun retrieve_resource_account_cap(resource: &signer, source_addr: address): account::SignerCapability
}
```


##### Implementation


```move
module 0x1::resource_account {
    public fun retrieve_resource_account_cap(
        resource: &signer,
        source_addr: address,
    ): account::SignerCapability acquires Container {
        assert!(exists<Container>(source_addr), error::not_found(ECONTAINER_NOT_PUBLISHED));

        let resource_addr = signer::address_of(resource);
        let (resource_signer_cap, empty_container) = {
            let container = borrow_global_mut<Container>(source_addr);
            assert!(
                simple_map::contains_key(&container.store, &resource_addr),
                error::invalid_argument(EUNAUTHORIZED_NOT_OWNER)
            );
            let (_resource_addr, signer_cap) = simple_map::remove(&mut container.store, &resource_addr);
            (signer_cap, simple_map::length(&container.store) == 0)
        };

        if (empty_container) {
            let container = move_from(source_addr);
            let Container { store } = container;
            simple_map::destroy_empty(store);
        };

        account::rotate_authentication_key_internal(resource, ZERO_AUTH_KEY);
        resource_signer_cap
    }
}
```


<a id="@Specification_3"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>The length of the authentication key must be 32 bytes.</td>
<td>Medium</td>
<td>The rotate_authentication_key_internal function ensures that the authentication key passed to it is of 32 bytes.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIf).</td>
</tr>

<tr>
<td>2</td>
<td>The Container structure must exist in the origin account in order to rotate the authentication key of a resource account and to store its signer capability.</td>
<td>High</td>
<td>The rotate_account_authentication_key_and_store_capability function makes sure the Container structure exists under the origin account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](rotate_account_authentication_key_and_store_capability).</td>
</tr>

<tr>
<td>3</td>
<td>The resource account is registered for the Aptos coin.</td>
<td>High</td>
<td>The create_resource_account_and_fund ensures the newly created resource account is registered to receive the AptosCoin.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](create_resource_account_and_fund).</td>
</tr>

<tr>
<td>4</td>
<td>It is not possible to store two capabilities for the same resource address.</td>
<td>Medium</td>
<td>The rotate_account_authentication_key_and_store_capability will abort if the resource signer capability for the given resource address already exists in container.store.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](rotate_account_authentication_key_and_store_capability).</td>
</tr>

<tr>
<td>5</td>
<td>If provided, the optional authentication key is used for key rotation.</td>
<td>Low</td>
<td>The rotate_account_authentication_key_and_store_capability function will use optional_auth_key if it is provided as a parameter.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5](rotate_account_authentication_key_and_store_capability).</td>
</tr>

<tr>
<td>6</td>
<td>The container stores the resource accounts&apos; signer capabilities.</td>
<td>Low</td>
<td>retrieve_resource_account_cap will abort if there is no Container structure assigned to source_addr.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;6](retreive_resource_account_cap).</td>
</tr>

<tr>
<td>7</td>
<td>Resource account may retrieve the signer capability if it was previously added to its container.</td>
<td>High</td>
<td>retrieve_resource_account_cap will abort if the container of source_addr doesn&apos;t store the signer capability for the given resource.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;7](retrieve_resource_account_cap).</td>
</tr>

<tr>
<td>8</td>
<td>Retrieving the last signer capability from the container must result in the container being removed.</td>
<td>Low</td>
<td>retrieve_resource_account_cap will remove the container if the retrieved signer_capability was the last one stored under it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;8](retrieve_resource_account_cap).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::resource_account {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_3_create_resource_account"></a>

### Function `create_resource_account`


```move
module 0x1::resource_account {
    public entry fun create_resource_account(origin: &signer, seed: vector<u8>, optional_auth_key: vector<u8>)
}
```



```move
module 0x1::resource_account {
    let source_addr = signer::address_of(origin);
    let resource_addr = account::spec_create_resource_address(source_addr, seed);
    include RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIfWithoutAccountLimit;
}
```


<a id="@Specification_3_create_resource_account_and_fund"></a>

### Function `create_resource_account_and_fund`


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_fund(origin: &signer, seed: vector<u8>, optional_auth_key: vector<u8>, fund_amount: u64)
}
```



```move
module 0x1::resource_account {
    pragma verify = false;
    let source_addr = signer::address_of(origin);
    let resource_addr = account::spec_create_resource_address(source_addr, seed);
    let coin_store_resource = global<coin::CoinStore<AptosCoin>>(resource_addr);
    include aptos_account::WithdrawAbortsIf<AptosCoin>{from: origin, amount: fund_amount};
    include aptos_account::GuidAbortsIf<AptosCoin>{to: resource_addr};
    include RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIfWithoutAccountLimit;
    aborts_if coin::spec_is_account_registered<AptosCoin>(resource_addr) && coin_store_resource.frozen;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures exists<aptos_framework::coin::CoinStore<AptosCoin>>(resource_addr);
}
```


<a id="@Specification_3_create_resource_account_and_publish_package"></a>

### Function `create_resource_account_and_publish_package`


```move
module 0x1::resource_account {
    public entry fun create_resource_account_and_publish_package(origin: &signer, seed: vector<u8>, metadata_serialized: vector<u8>, code: vector<vector<u8>>)
}
```



```move
module 0x1::resource_account {
    pragma verify = false;
    let source_addr = signer::address_of(origin);
    let resource_addr = account::spec_create_resource_address(source_addr, seed);
    let optional_auth_key = ZERO_AUTH_KEY;
    include RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIfWithoutAccountLimit;
}
```


<a id="@Specification_3_rotate_account_authentication_key_and_store_capability"></a>

### Function `rotate_account_authentication_key_and_store_capability`


```move
module 0x1::resource_account {
    fun rotate_account_authentication_key_and_store_capability(origin: &signer, resource: signer, resource_signer_cap: account::SignerCapability, optional_auth_key: vector<u8>)
}
```



```move
module 0x1::resource_account {
    let resource_addr = signer::address_of(resource);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    include RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIf;
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    ensures exists<Container>(signer::address_of(origin));
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5):
    ensures vector::length(optional_auth_key) != 0 ==>
        global<aptos_framework::account::Account>(resource_addr).authentication_key == optional_auth_key;
}
```



<a id="0x1_resource_account_RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIf"></a>


```move
module 0x1::resource_account {
    schema RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIf {
        origin: signer;
        resource_addr: address;
        optional_auth_key: vector<u8>;
        let source_addr = signer::address_of(origin);
        let container = global<Container>(source_addr);
        let get = len(optional_auth_key) == 0;
        aborts_if get && !exists<Account>(source_addr);
    // This enforces ### high&#45;level&#45;req&#45;4
    [#high&#45;level&#45;req](high&#45;level requirement 4):
        aborts_if exists<Container>(source_addr) && simple_map::spec_contains_key(container.store, resource_addr);
        aborts_if get && !(exists<Account>(resource_addr) && len(global<Account>(source_addr).authentication_key) == 32);
        aborts_if !get && !(exists<Account>(resource_addr) && len(optional_auth_key) == 32);
        ensures simple_map::spec_contains_key(global<Container>(source_addr).store, resource_addr);
        ensures exists<Container>(source_addr);
    }
}
```



<a id="0x1_resource_account_RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIfWithoutAccountLimit"></a>


```move
module 0x1::resource_account {
    schema RotateAccountAuthenticationKeyAndStoreCapabilityAbortsIfWithoutAccountLimit {
        source_addr: address;
        optional_auth_key: vector<u8>;
        resource_addr: address;
        let container = global<Container>(source_addr);
        let get = len(optional_auth_key) == 0;
        let account = global<account::Account>(source_addr);
        requires source_addr != resource_addr;
        aborts_if len(ZERO_AUTH_KEY) != 32;
        include account::exists_at(resource_addr) ==> account::CreateResourceAccountAbortsIf;
        include !account::exists_at(resource_addr) ==> account::CreateAccountAbortsIf {addr: resource_addr};
        aborts_if get && !exists<account::Account>(source_addr);
        aborts_if exists<Container>(source_addr) && simple_map::spec_contains_key(container.store, resource_addr);
        aborts_if get && len(global<account::Account>(source_addr).authentication_key) != 32;
        aborts_if !get && len(optional_auth_key) != 32;
        ensures simple_map::spec_contains_key(global<Container>(source_addr).store, resource_addr);
        ensures exists<Container>(source_addr);
    }
}
```


<a id="@Specification_3_retrieve_resource_account_cap"></a>

### Function `retrieve_resource_account_cap`


```move
module 0x1::resource_account {
    public fun retrieve_resource_account_cap(resource: &signer, source_addr: address): account::SignerCapability
}
```



```move
module 0x1::resource_account {
// This enforces ### high&#45;level&#45;req&#45;6
[#high&#45;level&#45;req](high&#45;level requirement 6):
    aborts_if !exists<Container>(source_addr);
    let resource_addr = signer::address_of(resource);
    let container = global<Container>(source_addr);
// This enforces ### high&#45;level&#45;req&#45;7
[#high&#45;level&#45;req](high&#45;level requirement 7):
    aborts_if !simple_map::spec_contains_key(container.store, resource_addr);
    aborts_if !exists<account::Account>(resource_addr);
// This enforces ### high&#45;level&#45;req&#45;8
[#high&#45;level&#45;req](high&#45;level requirement 8):
    ensures simple_map::spec_contains_key(old(global<Container>(source_addr)).store, resource_addr) &&
        simple_map::spec_len(old(global<Container>(source_addr)).store) == 1 ==> !exists<Container>(source_addr);
    ensures exists<Container>(source_addr) ==> !simple_map::spec_contains_key(global<Container>(source_addr).store, resource_addr);
}
```
