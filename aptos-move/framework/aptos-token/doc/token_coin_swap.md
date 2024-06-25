
<a id="0x3_token_coin_swap"></a>

# Module `0x3::token_coin_swap`

Deprecated module


-  [Struct `TokenCoinSwap`](#0x3_token_coin_swap_TokenCoinSwap)
-  [Resource `TokenListings`](#0x3_token_coin_swap_TokenListings)
-  [Struct `TokenEscrow`](#0x3_token_coin_swap_TokenEscrow)
-  [Resource `TokenStoreEscrow`](#0x3_token_coin_swap_TokenStoreEscrow)
-  [Struct `TokenListingEvent`](#0x3_token_coin_swap_TokenListingEvent)
-  [Struct `TokenSwapEvent`](#0x3_token_coin_swap_TokenSwapEvent)
-  [Constants](#@Constants_0)
-  [Function `does_listing_exist`](#0x3_token_coin_swap_does_listing_exist)
-  [Function `exchange_coin_for_token`](#0x3_token_coin_swap_exchange_coin_for_token)
-  [Function `list_token_for_swap`](#0x3_token_coin_swap_list_token_for_swap)
-  [Function `initialize_token_listing`](#0x3_token_coin_swap_initialize_token_listing)
-  [Function `initialize_token_store_escrow`](#0x3_token_coin_swap_initialize_token_store_escrow)
-  [Function `deposit_token_to_escrow`](#0x3_token_coin_swap_deposit_token_to_escrow)
-  [Function `withdraw_token_from_escrow_internal`](#0x3_token_coin_swap_withdraw_token_from_escrow_internal)
-  [Function `withdraw_token_from_escrow`](#0x3_token_coin_swap_withdraw_token_from_escrow)
-  [Function `cancel_token_listing`](#0x3_token_coin_swap_cancel_token_listing)
-  [Specification](#@Specification_1)


```move
module 0x3::token_coin_swap {
    use 0x1::error;
    use 0x1::event;
    use 0x1::string;
    use 0x1::table;
    use 0x1::type_info;
    use 0x3::token;
}
```


<a id="0x3_token_coin_swap_TokenCoinSwap"></a>

## Struct `TokenCoinSwap`

TokenCoinSwap records a swap ask for swapping token_amount with CoinType with a minimal price per token


```move
module 0x3::token_coin_swap {
    struct TokenCoinSwap<CoinType> has drop, store
}
```


##### Fields


<dl>
<dt>
`token_amount: u64`
</dt>
<dd>

</dd>
<dt>
`min_price_per_token: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_coin_swap_TokenListings"></a>

## Resource `TokenListings`

The listing of all tokens for swapping stored at token owner&apos;s account


```move
module 0x3::token_coin_swap {
    struct TokenListings<CoinType> has key
}
```


##### Fields


<dl>
<dt>
`listings: table::Table<token::TokenId, token_coin_swap::TokenCoinSwap<CoinType>>`
</dt>
<dd>

</dd>
<dt>
`listing_events: event::EventHandle<token_coin_swap::TokenListingEvent>`
</dt>
<dd>

</dd>
<dt>
`swap_events: event::EventHandle<token_coin_swap::TokenSwapEvent>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_coin_swap_TokenEscrow"></a>

## Struct `TokenEscrow`

TokenEscrow holds the tokens that cannot be withdrawn or transferred


```move
module 0x3::token_coin_swap {
    struct TokenEscrow has store
}
```


##### Fields


<dl>
<dt>
`token: token::Token`
</dt>
<dd>

</dd>
<dt>
`locked_until_secs: u64`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_coin_swap_TokenStoreEscrow"></a>

## Resource `TokenStoreEscrow`

TokenStoreEscrow holds a map of token id to their tokenEscrow


```move
module 0x3::token_coin_swap {
    struct TokenStoreEscrow has key
}
```


##### Fields


<dl>
<dt>
`token_escrows: table::Table<token::TokenId, token_coin_swap::TokenEscrow>`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_coin_swap_TokenListingEvent"></a>

## Struct `TokenListingEvent`



```move
module 0x3::token_coin_swap {
    struct TokenListingEvent has drop, store
}
```


##### Fields


<dl>
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
<dt>
`min_price: u64`
</dt>
<dd>

</dd>
<dt>
`locked_until_secs: u64`
</dt>
<dd>

</dd>
<dt>
`coin_type_info: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="0x3_token_coin_swap_TokenSwapEvent"></a>

## Struct `TokenSwapEvent`



```move
module 0x3::token_coin_swap {
    struct TokenSwapEvent has drop, store
}
```


##### Fields


<dl>
<dt>
`token_id: token::TokenId`
</dt>
<dd>

</dd>
<dt>
`token_buyer: address`
</dt>
<dd>

</dd>
<dt>
`token_amount: u64`
</dt>
<dd>

</dd>
<dt>
`coin_amount: u64`
</dt>
<dd>

</dd>
<dt>
`coin_type_info: type_info::TypeInfo`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x3_token_coin_swap_EDEPRECATED_MODULE"></a>

Deprecated module


```move
module 0x3::token_coin_swap {
    const EDEPRECATED_MODULE: u64 = 8;
}
```


<a id="0x3_token_coin_swap_ENOT_ENOUGH_COIN"></a>

Not enough coin to buy token


```move
module 0x3::token_coin_swap {
    const ENOT_ENOUGH_COIN: u64 = 7;
}
```


<a id="0x3_token_coin_swap_ETOKEN_ALREADY_LISTED"></a>

Token already listed


```move
module 0x3::token_coin_swap {
    const ETOKEN_ALREADY_LISTED: u64 = 1;
}
```


<a id="0x3_token_coin_swap_ETOKEN_AMOUNT_NOT_MATCH"></a>

Token buy amount doesn&apos;t match listing amount


```move
module 0x3::token_coin_swap {
    const ETOKEN_AMOUNT_NOT_MATCH: u64 = 6;
}
```


<a id="0x3_token_coin_swap_ETOKEN_CANNOT_MOVE_OUT_OF_ESCROW_BEFORE_LOCKUP_TIME"></a>

Token cannot be moved out of escrow before the lockup time


```move
module 0x3::token_coin_swap {
    const ETOKEN_CANNOT_MOVE_OUT_OF_ESCROW_BEFORE_LOCKUP_TIME: u64 = 4;
}
```


<a id="0x3_token_coin_swap_ETOKEN_LISTING_NOT_EXIST"></a>

Token listing no longer exists


```move
module 0x3::token_coin_swap {
    const ETOKEN_LISTING_NOT_EXIST: u64 = 2;
}
```


<a id="0x3_token_coin_swap_ETOKEN_MIN_PRICE_NOT_MATCH"></a>

Token buy price doesn&apos;t match listing price


```move
module 0x3::token_coin_swap {
    const ETOKEN_MIN_PRICE_NOT_MATCH: u64 = 5;
}
```


<a id="0x3_token_coin_swap_ETOKEN_NOT_IN_ESCROW"></a>

Token is not in escrow


```move
module 0x3::token_coin_swap {
    const ETOKEN_NOT_IN_ESCROW: u64 = 3;
}
```


<a id="0x3_token_coin_swap_does_listing_exist"></a>

## Function `does_listing_exist`



```move
module 0x3::token_coin_swap {
    public fun does_listing_exist<CoinType>(_token_owner: address, _token_id: token::TokenId): bool
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public fun does_listing_exist<CoinType>(
        _token_owner: address,
        _token_id: TokenId
    ): bool {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_exchange_coin_for_token"></a>

## Function `exchange_coin_for_token`

Coin owner withdraw coin to swap with tokens listed for swapping at the token owner&apos;s address.


```move
module 0x3::token_coin_swap {
    public fun exchange_coin_for_token<CoinType>(_coin_owner: &signer, _coin_amount: u64, _token_owner: address, _creators_address: address, _collection: string::String, _name: string::String, _property_version: u64, _token_amount: u64)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public fun exchange_coin_for_token<CoinType>(
        _coin_owner: &signer,
        _coin_amount: u64,
        _token_owner: address,
        _creators_address: address,
        _collection: String,
        _name: String,
        _property_version: u64,
        _token_amount: u64,
    ) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_list_token_for_swap"></a>

## Function `list_token_for_swap`

Token owner lists their token for swapping


```move
module 0x3::token_coin_swap {
    public entry fun list_token_for_swap<CoinType>(_token_owner: &signer, _creators_address: address, _collection: string::String, _name: string::String, _property_version: u64, _token_amount: u64, _min_coin_per_token: u64, _locked_until_secs: u64)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public entry fun list_token_for_swap<CoinType>(
        _token_owner: &signer,
        _creators_address: address,
        _collection: String,
        _name: String,
        _property_version: u64,
        _token_amount: u64,
        _min_coin_per_token: u64,
        _locked_until_secs: u64
    ) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_initialize_token_listing"></a>

## Function `initialize_token_listing`

Initalize the token listing for a token owner


```move
module 0x3::token_coin_swap {
    fun initialize_token_listing<CoinType>(_token_owner: &signer)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    fun initialize_token_listing<CoinType>(_token_owner: &signer) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_initialize_token_store_escrow"></a>

## Function `initialize_token_store_escrow`

Intialize the token escrow


```move
module 0x3::token_coin_swap {
    fun initialize_token_store_escrow(_token_owner: &signer)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    fun initialize_token_store_escrow(_token_owner: &signer) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_deposit_token_to_escrow"></a>

## Function `deposit_token_to_escrow`

Put the token into escrow that cannot be transferred or withdrawed by the owner.


```move
module 0x3::token_coin_swap {
    public fun deposit_token_to_escrow(_token_owner: &signer, _token_id: token::TokenId, _tokens: token::Token, _locked_until_secs: u64)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public fun deposit_token_to_escrow(
        _token_owner: &signer,
        _token_id: TokenId,
        _tokens: Token,
        _locked_until_secs: u64
    ) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_withdraw_token_from_escrow_internal"></a>

## Function `withdraw_token_from_escrow_internal`

Private function for withdraw tokens from an escrow stored in token owner address


```move
module 0x3::token_coin_swap {
    fun withdraw_token_from_escrow_internal(_token_owner_addr: address, _token_id: token::TokenId, _amount: u64): token::Token
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    fun withdraw_token_from_escrow_internal(
        _token_owner_addr: address,
        _token_id: TokenId,
        _amount: u64
    ): Token {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_withdraw_token_from_escrow"></a>

## Function `withdraw_token_from_escrow`

Withdraw tokens from the token escrow. It needs a signer to authorize


```move
module 0x3::token_coin_swap {
    public fun withdraw_token_from_escrow(_token_owner: &signer, _token_id: token::TokenId, _amount: u64): token::Token
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public fun withdraw_token_from_escrow(
        _token_owner: &signer,
        _token_id: TokenId,
        _amount: u64
    ): Token {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="0x3_token_coin_swap_cancel_token_listing"></a>

## Function `cancel_token_listing`

Cancel token listing for a fixed amount


```move
module 0x3::token_coin_swap {
    public fun cancel_token_listing<CoinType>(_token_owner: &signer, _token_id: token::TokenId, _token_amount: u64)
}
```


##### Implementation


```move
module 0x3::token_coin_swap {
    public fun cancel_token_listing<CoinType>(
        _token_owner: &signer,
        _token_id: TokenId,
        _token_amount: u64,
    ) {
        abort error::invalid_argument(EDEPRECATED_MODULE)
    }
}
```


<a id="@Specification_1"></a>

## Specification



```move
module 0x3::token_coin_swap {
    pragma verify = false;
}
```
