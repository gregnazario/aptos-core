
<a id="0x3_token_transfers"></a>

# Module `0x3::token_transfers`

This module provides the foundation for transferring of Tokens


-  [Resource `PendingClaims`](#0x3_token_transfers_PendingClaims)
-  [Struct `TokenOfferId`](#0x3_token_transfers_TokenOfferId)
-  [Struct `TokenOffer`](#0x3_token_transfers_TokenOffer)
-  [Struct `TokenOfferEvent`](#0x3_token_transfers_TokenOfferEvent)
-  [Struct `TokenCancelOfferEvent`](#0x3_token_transfers_TokenCancelOfferEvent)
-  [Struct `TokenCancelOffer`](#0x3_token_transfers_TokenCancelOffer)
-  [Struct `TokenClaimEvent`](#0x3_token_transfers_TokenClaimEvent)
-  [Struct `TokenClaim`](#0x3_token_transfers_TokenClaim)
-  [Constants](#@Constants_0)
-  [Function `initialize_token_transfers`](#0x3_token_transfers_initialize_token_transfers)
-  [Function `create_token_offer_id`](#0x3_token_transfers_create_token_offer_id)
-  [Function `offer_script`](#0x3_token_transfers_offer_script)
-  [Function `offer`](#0x3_token_transfers_offer)
-  [Function `claim_script`](#0x3_token_transfers_claim_script)
-  [Function `claim`](#0x3_token_transfers_claim)
-  [Function `cancel_offer_script`](#0x3_token_transfers_cancel_offer_script)
-  [Function `cancel_offer`](#0x3_token_transfers_cancel_offer)
-  [Specification](#@Specification_1)
    -  [Function `initialize_token_transfers`](#@Specification_1_initialize_token_transfers)
    -  [Function `create_token_offer_id`](#@Specification_1_create_token_offer_id)
    -  [Function `offer_script`](#@Specification_1_offer_script)
    -  [Function `offer`](#@Specification_1_offer)
    -  [Function `claim_script`](#@Specification_1_claim_script)
    -  [Function `claim`](#@Specification_1_claim)
    -  [Function `cancel_offer_script`](#@Specification_1_cancel_offer_script)
    -  [Function `cancel_offer`](#@Specification_1_cancel_offer)


```move
module 0x3::token_transfers {
    use 0x1::account;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::table;
    use 0x3::token;
}
```


<a id="0x3_token_transfers_PendingClaims"></a>

## Resource `PendingClaims`



```move
module 0x3::token_transfers {
    struct PendingClaims has key
}
```


##### Fields


<dl>
<dt>
`pending_claims: table::Table<token_transfers::TokenOfferId, token::Token>`
</dt>
<dd>

</dd>
<dt>
`offer_events: event::EventHandle<token_transfers::TokenOfferEvent>`
</dt>
<dd>

</dd>
<dt>
`cancel_offer_events: event::EventHandle<token_transfers::TokenCancelOfferEvent>`
</dt>
<dd>

</dd>
<dt>
`claim_events: event::EventHandle<token_transfers::TokenClaimEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenOfferId"></a>

## Struct `TokenOfferId`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenOfferId has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`to_addr: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenOffer"></a>

## Struct `TokenOffer`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenOffer has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenOfferEvent"></a>

## Struct `TokenOfferEvent`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenOfferEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenCancelOfferEvent"></a>

## Struct `TokenCancelOfferEvent`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenCancelOfferEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenCancelOffer"></a>

## Struct `TokenCancelOffer`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenCancelOffer has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenClaimEvent"></a>

## Struct `TokenClaimEvent`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenClaimEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_transfers_TokenClaim"></a>

## Struct `TokenClaim`



```move
module 0x3::token_transfers {
    #[event]
    struct TokenClaim has drop, store
}
```


##### Fields


<dl>
<dt>
`to_address: address`
</dt>
<dd>

</dd>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`amount: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST"></a>

Token offer doesn&apos;t exist


```move
module 0x3::token_transfers {
    const ETOKEN_OFFER_NOT_EXIST: u64 = 1;
}
```


<a id="0x3_token_transfers_initialize_token_transfers"></a>

## Function `initialize_token_transfers`



```move
module 0x3::token_transfers {
    fun initialize_token_transfers(account: &signer)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    fun initialize_token_transfers(account: &signer) {
        move_to(
            account,
            PendingClaims {
                pending_claims: table::new<TokenOfferId, Token>(),
                offer_events: account::new_event_handle<TokenOfferEvent>(account),
                cancel_offer_events: account::new_event_handle<TokenCancelOfferEvent>(account),
                claim_events: account::new_event_handle<TokenClaimEvent>(account),
            }
        )
    }
}
```


<a id="0x3_token_transfers_create_token_offer_id"></a>

## Function `create_token_offer_id`



```move
module 0x3::token_transfers {
    fun create_token_offer_id(to_addr: address, token_id: token::TokenId): token_transfers::TokenOfferId
}
```


##### Implementation


```move
module 0x3::token_transfers {
    fun create_token_offer_id(to_addr: address, token_id: TokenId): TokenOfferId {
        TokenOfferId {
            to_addr,
            token_id
        }
    }
}
```


<a id="0x3_token_transfers_offer_script"></a>

## Function `offer_script`



```move
module 0x3::token_transfers {
    public entry fun offer_script(sender: signer, receiver: address, creator: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public entry fun offer_script(
        sender: signer,
        receiver: address,
        creator: address,
        collection: String,
        name: String,
        property_version: u64,
        amount: u64,
    ) acquires PendingClaims {
        let token_id = token::create_token_id_raw(creator, collection, name, property_version);
        offer(&sender, receiver, token_id, amount);
    }
}
```


<a id="0x3_token_transfers_offer"></a>

## Function `offer`



```move
module 0x3::token_transfers {
    public fun offer(sender: &signer, receiver: address, token_id: token::TokenId, amount: u64)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public fun offer(
        sender: &signer,
        receiver: address,
        token_id: TokenId,
        amount: u64,
    ) acquires PendingClaims {
        let sender_addr = signer::address_of(sender);
        if (!exists<PendingClaims>(sender_addr)) {
            initialize_token_transfers(sender)
        };

        let pending_claims =
            &mut borrow_global_mut<PendingClaims>(sender_addr).pending_claims;
        let token_offer_id = create_token_offer_id(receiver, token_id);
        let token = token::withdraw_token(sender, token_id, amount);
        if (!table::contains(pending_claims, token_offer_id)) {
            table::add(pending_claims, token_offer_id, token);
        } else {
            let dst_token = table::borrow_mut(pending_claims, token_offer_id);
            token::merge(dst_token, token);
        };

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                TokenOffer {
                    to_address: receiver,
                    token_id,
                    amount,
                }
            )
        };
        event::emit_event<TokenOfferEvent>(
            &mut borrow_global_mut<PendingClaims>(sender_addr).offer_events,
            TokenOfferEvent {
                to_address: receiver,
                token_id,
                amount,
            },
        );
    }
}
```


<a id="0x3_token_transfers_claim_script"></a>

## Function `claim_script`



```move
module 0x3::token_transfers {
    public entry fun claim_script(receiver: signer, sender: address, creator: address, collection: string::String, name: string::String, property_version: u64)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public entry fun claim_script(
        receiver: signer,
        sender: address,
        creator: address,
        collection: String,
        name: String,
        property_version: u64,
    ) acquires PendingClaims {
        let token_id = token::create_token_id_raw(creator, collection, name, property_version);
        claim(&receiver, sender, token_id);
    }
}
```


<a id="0x3_token_transfers_claim"></a>

## Function `claim`



```move
module 0x3::token_transfers {
    public fun claim(receiver: &signer, sender: address, token_id: token::TokenId)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public fun claim(
        receiver: &signer,
        sender: address,
        token_id: TokenId,
    ) acquires PendingClaims {
        assert!(exists<PendingClaims>(sender), ETOKEN_OFFER_NOT_EXIST);
        let pending_claims =
            &mut borrow_global_mut<PendingClaims>(sender).pending_claims;
        let token_offer_id = create_token_offer_id(signer::address_of(receiver), token_id);
        assert!(table::contains(pending_claims, token_offer_id), error::not_found(ETOKEN_OFFER_NOT_EXIST));
        let tokens = table::remove(pending_claims, token_offer_id);
        let amount = token::get_token_amount(&tokens);
        token::deposit_token(receiver, tokens);

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                TokenClaim {
                    to_address: signer::address_of(receiver),
                    token_id,
                    amount,
                }
            )
        };
        event::emit_event<TokenClaimEvent>(
            &mut borrow_global_mut<PendingClaims>(sender).claim_events,
            TokenClaimEvent {
                to_address: signer::address_of(receiver),
                token_id,
                amount,
            },
        );
    }
}
```


<a id="0x3_token_transfers_cancel_offer_script"></a>

## Function `cancel_offer_script`



```move
module 0x3::token_transfers {
    public entry fun cancel_offer_script(sender: signer, receiver: address, creator: address, collection: string::String, name: string::String, property_version: u64)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public entry fun cancel_offer_script(
        sender: signer,
        receiver: address,
        creator: address,
        collection: String,
        name: String,
        property_version: u64,
    ) acquires PendingClaims {
        let token_id = token::create_token_id_raw(creator, collection, name, property_version);
        cancel_offer(&sender, receiver, token_id);
    }
}
```


<a id="0x3_token_transfers_cancel_offer"></a>

## Function `cancel_offer`



```move
module 0x3::token_transfers {
    public fun cancel_offer(sender: &signer, receiver: address, token_id: token::TokenId)
}
```


##### Implementation


```move
module 0x3::token_transfers {
    public fun cancel_offer(
        sender: &signer,
        receiver: address,
        token_id: TokenId,
    ) acquires PendingClaims {
        let sender_addr = signer::address_of(sender);
        let token_offer_id = create_token_offer_id(receiver, token_id);
        assert!(exists<PendingClaims>(sender_addr), ETOKEN_OFFER_NOT_EXIST);
        let pending_claims =
            &mut borrow_global_mut<PendingClaims>(sender_addr).pending_claims;
        let token = table::remove(pending_claims, token_offer_id);
        let amount = token::get_token_amount(&token);
        token::deposit_token(sender, token);

        if (std::features::module_event_migration_enabled()) {
            event::emit(
                TokenCancelOffer {
                    to_address: receiver,
                    token_id,
                    amount,
                },
            )
        };
        event::emit_event<TokenCancelOfferEvent>(
            &mut borrow_global_mut<PendingClaims>(sender_addr).cancel_offer_events,
            TokenCancelOfferEvent {
                to_address: receiver,
                token_id,
                amount,
            },
        );
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x3::token_transfers {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize_token_transfers"></a>

### Function `initialize_token_transfers`


```move
module 0x3::token_transfers {
    fun initialize_token_transfers(account: &signer)
}
```



```move
module 0x3::token_transfers {
    include InitializeTokenTransfersAbortsIf;
}
```

Abort according to the code


<a id="0x3_token_transfers_InitializeTokenTransfersAbortsIf"></a>


```move
module 0x3::token_transfers {
    schema InitializeTokenTransfersAbortsIf {
        account: &signer;
        let addr = signer::address_of(account);
        aborts_if exists<PendingClaims>(addr);
        let account = global<Account>(addr);
        aborts_if !exists<Account>(addr);
        aborts_if account.guid_creation_num + 3 >= account::MAX_GUID_CREATION_NUM;
        aborts_if account.guid_creation_num + 3 > MAX_U64;
    }
}
```


<a id="@Specification_1_create_token_offer_id"></a>

### Function `create_token_offer_id`


```move
module 0x3::token_transfers {
    fun create_token_offer_id(to_addr: address, token_id: token::TokenId): token_transfers::TokenOfferId
}
```



```move
module 0x3::token_transfers {
    aborts_if false;
}
```


<a id="@Specification_1_offer_script"></a>

### Function `offer_script`


```move
module 0x3::token_transfers {
    public entry fun offer_script(sender: signer, receiver: address, creator: address, collection: string::String, name: string::String, property_version: u64, amount: u64)
}
```



```move
module 0x3::token_transfers {
    pragma verify = false;
    let token_id = token::create_token_id_raw(creator, collection, name, property_version);
}
```


<a id="@Specification_1_offer"></a>

### Function `offer`


```move
module 0x3::token_transfers {
    public fun offer(sender: &signer, receiver: address, token_id: token::TokenId, amount: u64)
}
```



```move
module 0x3::token_transfers {
    pragma verify = false;
    let sender_addr = signer::address_of(sender);
    include !exists<PendingClaims>(sender_addr) ==> InitializeTokenTransfersAbortsIf{account : sender};
    let pending_claims = global<PendingClaims>(sender_addr).pending_claims;
    let token_offer_id = create_token_offer_id(receiver, token_id);
    let tokens = global<TokenStore>(sender_addr).tokens;
    aborts_if amount <= 0;
    aborts_if token::spec_balance_of(sender_addr, token_id) < amount;
    aborts_if !exists<TokenStore>(sender_addr);
    aborts_if !table::spec_contains(tokens, token_id);
    aborts_if !table::spec_contains(pending_claims, token_offer_id);
    let a = table::spec_contains(pending_claims, token_offer_id);
    let dst_token = table::spec_get(pending_claims, token_offer_id);
    aborts_if dst_token.amount + spce_get(signer::address_of(sender), token_id, amount) > MAX_U64;
}
```

Get the amount from sender token


<a id="0x3_token_transfers_spce_get"></a>


```move
module 0x3::token_transfers {
    fun spce_get(
       account_addr: address,
       id: TokenId,
       amount: u64
    ): u64 {
       use aptos_token::token::{TokenStore};
       use aptos_std::table::{Self};
       let tokens = global<TokenStore>(account_addr).tokens;
       let balance = table::spec_get(tokens, id).amount;
       if (balance > amount) {
           amount
       } else {
           table::spec_get(tokens, id).amount
       }
    }
}
```


<a id="@Specification_1_claim_script"></a>

### Function `claim_script`


```move
module 0x3::token_transfers {
    public entry fun claim_script(receiver: signer, sender: address, creator: address, collection: string::String, name: string::String, property_version: u64)
}
```



```move
module 0x3::token_transfers {
    pragma aborts_if_is_partial;
    let token_id = token::create_token_id_raw(creator, collection, name, property_version);
    aborts_if !exists<PendingClaims>(sender);
    let pending_claims = global<PendingClaims>(sender).pending_claims;
    let token_offer_id = create_token_offer_id(signer::address_of(receiver), token_id);
    aborts_if !table::spec_contains(pending_claims, token_offer_id);
    let tokens = table::spec_get(pending_claims, token_offer_id);
    include token::InitializeTokenStore{account: receiver };
    let account_addr = signer::address_of(receiver);
    let token = tokens;
    let token_store = global<TokenStore>(account_addr);
    let recipient_token = table::spec_get(token_store.tokens, token.id);
    let b = table::spec_contains(token_store.tokens, token.id);
    aborts_if token.amount <= 0;
}
```


<a id="@Specification_1_claim"></a>

### Function `claim`


```move
module 0x3::token_transfers {
    public fun claim(receiver: &signer, sender: address, token_id: token::TokenId)
}
```



```move
module 0x3::token_transfers {
    pragma aborts_if_is_partial;
    aborts_if !exists<PendingClaims>(sender);
    let pending_claims = global<PendingClaims>(sender).pending_claims;
    let token_offer_id = create_token_offer_id(signer::address_of(receiver), token_id);
    aborts_if !table::spec_contains(pending_claims, token_offer_id);
    let tokens = table::spec_get(pending_claims, token_offer_id);
    include token::InitializeTokenStore{account: receiver };
    let account_addr = signer::address_of(receiver);
    let token = tokens;
    let token_store = global<TokenStore>(account_addr);
    let recipient_token = table::spec_get(token_store.tokens, token.id);
    let b = table::spec_contains(token_store.tokens, token.id);
    aborts_if token.amount <= 0;
}
```


<a id="@Specification_1_cancel_offer_script"></a>

### Function `cancel_offer_script`


```move
module 0x3::token_transfers {
    public entry fun cancel_offer_script(sender: signer, receiver: address, creator: address, collection: string::String, name: string::String, property_version: u64)
}
```



```move
module 0x3::token_transfers {
    pragma aborts_if_is_partial;
    let token_id = token::create_token_id_raw(creator, collection, name, property_version);
    let sender_addr = signer::address_of(sender);
    aborts_if !exists<PendingClaims>(sender_addr);
    let pending_claims = global<PendingClaims>(sender_addr).pending_claims;
    let token_offer_id = create_token_offer_id(receiver, token_id);
    aborts_if !table::spec_contains(pending_claims, token_offer_id);
    include token::InitializeTokenStore{account: sender };
    let dst_token = table::spec_get(pending_claims, token_offer_id);
    let account_addr = sender_addr;
    let token = dst_token;
    let token_store = global<TokenStore>(account_addr);
    let recipient_token = table::spec_get(token_store.tokens, token.id);
    let b = table::spec_contains(token_store.tokens, token.id);
    aborts_if token.amount <= 0;
}
```


<a id="@Specification_1_cancel_offer"></a>

### Function `cancel_offer`


```move
module 0x3::token_transfers {
    public fun cancel_offer(sender: &signer, receiver: address, token_id: token::TokenId)
}
```



```move
module 0x3::token_transfers {
    pragma aborts_if_is_partial;
    let sender_addr = signer::address_of(sender);
    aborts_if !exists<PendingClaims>(sender_addr);
    let pending_claims = global<PendingClaims>(sender_addr).pending_claims;
    let token_offer_id = create_token_offer_id(receiver, token_id);
    aborts_if !table::spec_contains(pending_claims, token_offer_id);
    include token::InitializeTokenStore{account: sender };
    let dst_token = table::spec_get(pending_claims, token_offer_id);
    let account_addr = sender_addr;
    let token = dst_token;
    let token_store = global<TokenStore>(account_addr);
    let recipient_token = table::spec_get(token_store.tokens, token.id);
    let b = table::spec_contains(token_store.tokens, token.id);
    aborts_if token.amount <= 0;
}
```
