
<a id="0x1_pool_u64_unbound"></a>

# Module `0x1::pool_u64_unbound`


Simple module for tracking and calculating shares of a pool of coins. The shares are worth more as the total coins in
the pool increases. New shareholder can buy more shares or redeem their existing shares.

Example flow:
1. Pool start outs empty.
2. Shareholder A buys in with 1000 coins. A will receive 1000 shares in the pool. Pool now has 1000 total coins and
1000 total shares.
3. Pool appreciates in value from rewards and now has 2000 coins. A&apos;s 1000 shares are now worth 2000 coins.
4. Shareholder B now buys in with 1000 coins. Since before the buy in, each existing share is worth 2 coins, B will
receive 500 shares in exchange for 1000 coins. Pool now has 1500 shares and 3000 coins.
5. Pool appreciates in value from rewards and now has 6000 coins.
6. A redeems 500 shares. Each share is worth 6000 / 1500 &#61; 4. A receives 2000 coins. Pool has 4000 coins and 1000
shares left.


-  [Struct `Pool`](#0x1_pool_u64_unbound_Pool)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_pool_u64_unbound_new)
-  [Function `create`](#0x1_pool_u64_unbound_create)
-  [Function `create_with_scaling_factor`](#0x1_pool_u64_unbound_create_with_scaling_factor)
-  [Function `destroy_empty`](#0x1_pool_u64_unbound_destroy_empty)
-  [Function `total_coins`](#0x1_pool_u64_unbound_total_coins)
-  [Function `total_shares`](#0x1_pool_u64_unbound_total_shares)
-  [Function `contains`](#0x1_pool_u64_unbound_contains)
-  [Function `shares`](#0x1_pool_u64_unbound_shares)
-  [Function `balance`](#0x1_pool_u64_unbound_balance)
-  [Function `shareholders_count`](#0x1_pool_u64_unbound_shareholders_count)
-  [Function `update_total_coins`](#0x1_pool_u64_unbound_update_total_coins)
-  [Function `buy_in`](#0x1_pool_u64_unbound_buy_in)
-  [Function `add_shares`](#0x1_pool_u64_unbound_add_shares)
-  [Function `redeem_shares`](#0x1_pool_u64_unbound_redeem_shares)
-  [Function `transfer_shares`](#0x1_pool_u64_unbound_transfer_shares)
-  [Function `deduct_shares`](#0x1_pool_u64_unbound_deduct_shares)
-  [Function `amount_to_shares`](#0x1_pool_u64_unbound_amount_to_shares)
-  [Function `amount_to_shares_with_total_coins`](#0x1_pool_u64_unbound_amount_to_shares_with_total_coins)
-  [Function `shares_to_amount`](#0x1_pool_u64_unbound_shares_to_amount)
-  [Function `shares_to_amount_with_total_coins`](#0x1_pool_u64_unbound_shares_to_amount_with_total_coins)
-  [Function `shares_to_amount_with_total_stats`](#0x1_pool_u64_unbound_shares_to_amount_with_total_stats)
-  [Function `multiply_then_divide`](#0x1_pool_u64_unbound_multiply_then_divide)
-  [Function `to_u128`](#0x1_pool_u64_unbound_to_u128)
-  [Function `to_u256`](#0x1_pool_u64_unbound_to_u256)
-  [Specification](#@Specification_1)
    -  [Struct `Pool`](#@Specification_1_Pool)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `shares`](#@Specification_1_shares)
    -  [Function `balance`](#@Specification_1_balance)
    -  [Function `buy_in`](#@Specification_1_buy_in)
    -  [Function `add_shares`](#@Specification_1_add_shares)
    -  [Function `redeem_shares`](#@Specification_1_redeem_shares)
    -  [Function `transfer_shares`](#@Specification_1_transfer_shares)
    -  [Function `deduct_shares`](#@Specification_1_deduct_shares)
    -  [Function `amount_to_shares_with_total_coins`](#@Specification_1_amount_to_shares_with_total_coins)
    -  [Function `shares_to_amount_with_total_coins`](#@Specification_1_shares_to_amount_with_total_coins)
    -  [Function `multiply_then_divide`](#@Specification_1_multiply_then_divide)
    -  [Function `to_u128`](#@Specification_1_to_u128)
    -  [Function `to_u256`](#@Specification_1_to_u256)


```move
module 0x1::pool_u64_unbound {
    use 0x1::error;
    use 0x1::table_with_length;
}
```


<a id="0x1_pool_u64_unbound_Pool"></a>

## Struct `Pool`



```move
module 0x1::pool_u64_unbound {
    struct Pool has store
}
```


##### Fields


<dl>
<dt>
`total_coins: u64`
</dt>
<dd>

</dd>
<dt>
`total_shares: u128`
</dt>
<dd>

</dd>
<dt>
`shares: table_with_length::TableWithLength<address, u128>`
</dt>
<dd>

</dd>
<dt>
`scaling_factor: u64`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_pool_u64_unbound_MAX_U64"></a>



```move
module 0x1::pool_u64_unbound {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x1_pool_u64_unbound_MAX_U128"></a>



```move
module 0x1::pool_u64_unbound {
    const MAX_U128: u128 = 340282366920938463463374607431768211455;
}
```


<a id="0x1_pool_u64_unbound_EINSUFFICIENT_SHARES"></a>

Cannot redeem more shares than the shareholder has in the pool.


```move
module 0x1::pool_u64_unbound {
    const EINSUFFICIENT_SHARES: u64 = 4;
}
```


<a id="0x1_pool_u64_unbound_EPOOL_IS_NOT_EMPTY"></a>

Cannot destroy non&#45;empty pool.


```move
module 0x1::pool_u64_unbound {
    const EPOOL_IS_NOT_EMPTY: u64 = 3;
}
```


<a id="0x1_pool_u64_unbound_EPOOL_TOTAL_COINS_OVERFLOW"></a>

Pool&apos;s total coins cannot exceed u64.max.


```move
module 0x1::pool_u64_unbound {
    const EPOOL_TOTAL_COINS_OVERFLOW: u64 = 6;
}
```


<a id="0x1_pool_u64_unbound_EPOOL_TOTAL_SHARES_OVERFLOW"></a>

Pool&apos;s total shares cannot exceed u64.max.


```move
module 0x1::pool_u64_unbound {
    const EPOOL_TOTAL_SHARES_OVERFLOW: u64 = 7;
}
```


<a id="0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND"></a>

Shareholder not present in pool.


```move
module 0x1::pool_u64_unbound {
    const ESHAREHOLDER_NOT_FOUND: u64 = 1;
}
```


<a id="0x1_pool_u64_unbound_ESHAREHOLDER_SHARES_OVERFLOW"></a>

Shareholder cannot have more than u64.max shares.


```move
module 0x1::pool_u64_unbound {
    const ESHAREHOLDER_SHARES_OVERFLOW: u64 = 5;
}
```


<a id="0x1_pool_u64_unbound_ETOO_MANY_SHAREHOLDERS"></a>

There are too many shareholders in the pool.


```move
module 0x1::pool_u64_unbound {
    const ETOO_MANY_SHAREHOLDERS: u64 = 2;
}
```


<a id="0x1_pool_u64_unbound_new"></a>

## Function `new`

Create a new pool.


```move
module 0x1::pool_u64_unbound {
    public fun new(): pool_u64_unbound::Pool
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun new(): Pool {
        // Default to a scaling factor of 1 (effectively no scaling).
        create_with_scaling_factor(1)
    }
}
```


<a id="0x1_pool_u64_unbound_create"></a>

## Function `create`

Deprecated. Use `new` instead.
Create a new pool.


```move
module 0x1::pool_u64_unbound {
    #[deprecated]
    public fun create(): pool_u64_unbound::Pool
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun create(): Pool {
        new()
    }
}
```


<a id="0x1_pool_u64_unbound_create_with_scaling_factor"></a>

## Function `create_with_scaling_factor`

Create a new pool with custom `scaling_factor`.


```move
module 0x1::pool_u64_unbound {
    public fun create_with_scaling_factor(scaling_factor: u64): pool_u64_unbound::Pool
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun create_with_scaling_factor(scaling_factor: u64): Pool {
        Pool {
            total_coins: 0,
            total_shares: 0,
            shares: table::new<address, u128>(),
            scaling_factor,
        }
    }
}
```


<a id="0x1_pool_u64_unbound_destroy_empty"></a>

## Function `destroy_empty`

Destroy an empty pool. This will fail if the pool has any balance of coins.


```move
module 0x1::pool_u64_unbound {
    public fun destroy_empty(pool: pool_u64_unbound::Pool)
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun destroy_empty(pool: Pool) {
        assert!(pool.total_coins == 0, error::invalid_state(EPOOL_IS_NOT_EMPTY));
        let Pool {
            total_coins: _,
            total_shares: _,
            shares,
            scaling_factor: _,
        } = pool;
        table::destroy_empty<address, u128>(shares);
    }
}
```


<a id="0x1_pool_u64_unbound_total_coins"></a>

## Function `total_coins`

Return `pool`&apos;s total balance of coins.


```move
module 0x1::pool_u64_unbound {
    public fun total_coins(pool: &pool_u64_unbound::Pool): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun total_coins(pool: &Pool): u64 {
        pool.total_coins
    }
}
```


<a id="0x1_pool_u64_unbound_total_shares"></a>

## Function `total_shares`

Return the total number of shares across all shareholders in `pool`.


```move
module 0x1::pool_u64_unbound {
    public fun total_shares(pool: &pool_u64_unbound::Pool): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun total_shares(pool: &Pool): u128 {
        pool.total_shares
    }
}
```


<a id="0x1_pool_u64_unbound_contains"></a>

## Function `contains`

Return true if `shareholder` is in `pool`.


```move
module 0x1::pool_u64_unbound {
    public fun contains(pool: &pool_u64_unbound::Pool, shareholder: address): bool
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun contains(pool: &Pool, shareholder: address): bool {
        table::contains(&pool.shares, shareholder)
    }
}
```


<a id="0x1_pool_u64_unbound_shares"></a>

## Function `shares`

Return the number of shares of `stakeholder` in `pool`.


```move
module 0x1::pool_u64_unbound {
    public fun shares(pool: &pool_u64_unbound::Pool, shareholder: address): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun shares(pool: &Pool, shareholder: address): u128 {
        if (contains(pool, shareholder)) {
            *table::borrow(&pool.shares, shareholder)
        } else {
            0
        }
    }
}
```


<a id="0x1_pool_u64_unbound_balance"></a>

## Function `balance`

Return the balance in coins of `shareholder` in `pool.`


```move
module 0x1::pool_u64_unbound {
    public fun balance(pool: &pool_u64_unbound::Pool, shareholder: address): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun balance(pool: &Pool, shareholder: address): u64 {
        let num_shares = shares(pool, shareholder);
        shares_to_amount(pool, num_shares)
    }
}
```


<a id="0x1_pool_u64_unbound_shareholders_count"></a>

## Function `shareholders_count`

Return the number of shareholders in `pool`.


```move
module 0x1::pool_u64_unbound {
    public fun shareholders_count(pool: &pool_u64_unbound::Pool): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun shareholders_count(pool: &Pool): u64 {
        table::length(&pool.shares)
    }
}
```


<a id="0x1_pool_u64_unbound_update_total_coins"></a>

## Function `update_total_coins`

Update `pool`&apos;s total balance of coins.


```move
module 0x1::pool_u64_unbound {
    public fun update_total_coins(pool: &mut pool_u64_unbound::Pool, new_total_coins: u64)
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun update_total_coins(pool: &mut Pool, new_total_coins: u64) {
        pool.total_coins = new_total_coins;
    }
}
```


<a id="0x1_pool_u64_unbound_buy_in"></a>

## Function `buy_in`

Allow an existing or new shareholder to add their coins to the pool in exchange for new shares.


```move
module 0x1::pool_u64_unbound {
    public fun buy_in(pool: &mut pool_u64_unbound::Pool, shareholder: address, coins_amount: u64): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun buy_in(pool: &mut Pool, shareholder: address, coins_amount: u64): u128 {
        if (coins_amount == 0) return 0;

        let new_shares = amount_to_shares(pool, coins_amount);
        assert!(MAX_U64 - pool.total_coins >= coins_amount, error::invalid_argument(EPOOL_TOTAL_COINS_OVERFLOW));
        assert!(MAX_U128 - pool.total_shares >= new_shares, error::invalid_argument(EPOOL_TOTAL_SHARES_OVERFLOW));

        pool.total_coins = pool.total_coins + coins_amount;
        pool.total_shares = pool.total_shares + new_shares;
        add_shares(pool, shareholder, new_shares);
        new_shares
    }
}
```


<a id="0x1_pool_u64_unbound_add_shares"></a>

## Function `add_shares`

Add the number of shares directly for `shareholder` in `pool`.
This would dilute other shareholders if the pool&apos;s balance of coins didn&apos;t change.


```move
module 0x1::pool_u64_unbound {
    fun add_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, new_shares: u128): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    fun add_shares(pool: &mut Pool, shareholder: address, new_shares: u128): u128 {
        if (contains(pool, shareholder)) {
            let existing_shares = table::borrow_mut(&mut pool.shares, shareholder);
            let current_shares = *existing_shares;
            assert!(MAX_U128 - current_shares >= new_shares, error::invalid_argument(ESHAREHOLDER_SHARES_OVERFLOW));

            *existing_shares = current_shares + new_shares;
            *existing_shares
        } else if (new_shares > 0) {
            table::add(&mut pool.shares, shareholder, new_shares);
            new_shares
        } else {
            new_shares
        }
    }
}
```


<a id="0x1_pool_u64_unbound_redeem_shares"></a>

## Function `redeem_shares`

Allow `shareholder` to redeem their shares in `pool` for coins.


```move
module 0x1::pool_u64_unbound {
    public fun redeem_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, shares_to_redeem: u128): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun redeem_shares(pool: &mut Pool, shareholder: address, shares_to_redeem: u128): u64 {
        assert!(contains(pool, shareholder), error::invalid_argument(ESHAREHOLDER_NOT_FOUND));
        assert!(shares(pool, shareholder) >= shares_to_redeem, error::invalid_argument(EINSUFFICIENT_SHARES));

        if (shares_to_redeem == 0) return 0;

        let redeemed_coins = shares_to_amount(pool, shares_to_redeem);
        pool.total_coins = pool.total_coins - redeemed_coins;
        pool.total_shares = pool.total_shares - shares_to_redeem;
        deduct_shares(pool, shareholder, shares_to_redeem);

        redeemed_coins
    }
}
```


<a id="0x1_pool_u64_unbound_transfer_shares"></a>

## Function `transfer_shares`

Transfer shares from `shareholder_1` to `shareholder_2`.


```move
module 0x1::pool_u64_unbound {
    public fun transfer_shares(pool: &mut pool_u64_unbound::Pool, shareholder_1: address, shareholder_2: address, shares_to_transfer: u128)
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun transfer_shares(
        pool: &mut Pool,
        shareholder_1: address,
        shareholder_2: address,
        shares_to_transfer: u128,
    ) {
        assert!(contains(pool, shareholder_1), error::invalid_argument(ESHAREHOLDER_NOT_FOUND));
        assert!(shares(pool, shareholder_1) >= shares_to_transfer, error::invalid_argument(EINSUFFICIENT_SHARES));
        if (shares_to_transfer == 0) return;

        deduct_shares(pool, shareholder_1, shares_to_transfer);
        add_shares(pool, shareholder_2, shares_to_transfer);
    }
}
```


<a id="0x1_pool_u64_unbound_deduct_shares"></a>

## Function `deduct_shares`

Directly deduct `shareholder`&apos;s number of shares in `pool` and return the number of remaining shares.


```move
module 0x1::pool_u64_unbound {
    fun deduct_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, num_shares: u128): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    fun deduct_shares(pool: &mut Pool, shareholder: address, num_shares: u128): u128 {
        assert!(contains(pool, shareholder), error::invalid_argument(ESHAREHOLDER_NOT_FOUND));
        assert!(shares(pool, shareholder) >= num_shares, error::invalid_argument(EINSUFFICIENT_SHARES));

        let existing_shares = table::borrow_mut(&mut pool.shares, shareholder);
        *existing_shares = *existing_shares - num_shares;

        // Remove the shareholder completely if they have no shares left.
        let remaining_shares = *existing_shares;
        if (remaining_shares == 0) {
            table::remove(&mut pool.shares, shareholder);
        };

        remaining_shares
    }
}
```


<a id="0x1_pool_u64_unbound_amount_to_shares"></a>

## Function `amount_to_shares`

Return the number of new shares `coins_amount` can buy in `pool`.
`amount` needs to big enough to avoid rounding number.


```move
module 0x1::pool_u64_unbound {
    public fun amount_to_shares(pool: &pool_u64_unbound::Pool, coins_amount: u64): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun amount_to_shares(pool: &Pool, coins_amount: u64): u128 {
        amount_to_shares_with_total_coins(pool, coins_amount, pool.total_coins)
    }
}
```


<a id="0x1_pool_u64_unbound_amount_to_shares_with_total_coins"></a>

## Function `amount_to_shares_with_total_coins`

Return the number of new shares `coins_amount` can buy in `pool` with a custom total coins number.
`amount` needs to big enough to avoid rounding number.


```move
module 0x1::pool_u64_unbound {
    public fun amount_to_shares_with_total_coins(pool: &pool_u64_unbound::Pool, coins_amount: u64, total_coins: u64): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun amount_to_shares_with_total_coins(pool: &Pool, coins_amount: u64, total_coins: u64): u128 {
        // No shares yet so amount is worth the same number of shares.
        if (pool.total_coins == 0 || pool.total_shares == 0) {
            // Multiply by scaling factor to minimize rounding errors during internal calculations for buy ins/redeems.
            // This can overflow but scaling factor is expected to be chosen carefully so this would not overflow.
            to_u128(coins_amount) * to_u128(pool.scaling_factor)
        } else {
            // Shares price = total_coins / total existing shares.
            // New number of shares = new_amount / shares_price = new_amount * existing_shares / total_amount.
            // We rearrange the calc and do multiplication first to avoid rounding errors.
            multiply_then_divide(pool, to_u128(coins_amount), pool.total_shares, to_u128(total_coins))
        }
    }
}
```


<a id="0x1_pool_u64_unbound_shares_to_amount"></a>

## Function `shares_to_amount`

Return the number of coins `shares` are worth in `pool`.
`shares` needs to big enough to avoid rounding number.


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount(pool: &pool_u64_unbound::Pool, shares: u128): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount(pool: &Pool, shares: u128): u64 {
        shares_to_amount_with_total_coins(pool, shares, pool.total_coins)
    }
}
```


<a id="0x1_pool_u64_unbound_shares_to_amount_with_total_coins"></a>

## Function `shares_to_amount_with_total_coins`

Return the number of coins `shares` are worth in `pool` with a custom total coins number.
`shares` needs to big enough to avoid rounding number.


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount_with_total_coins(pool: &pool_u64_unbound::Pool, shares: u128, total_coins: u64): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount_with_total_coins(pool: &Pool, shares: u128, total_coins: u64): u64 {
        // No shares or coins yet so shares are worthless.
        if (pool.total_coins == 0 || pool.total_shares == 0) {
            0
        } else {
            // Shares price = total_coins / total existing shares.
            // Shares worth = shares * shares price = shares * total_coins / total existing shares.
            // We rearrange the calc and do multiplication first to avoid rounding errors.
            (multiply_then_divide(pool, shares, to_u128(total_coins), pool.total_shares) as u64)
        }
    }
}
```


<a id="0x1_pool_u64_unbound_shares_to_amount_with_total_stats"></a>

## Function `shares_to_amount_with_total_stats`

Return the number of coins `shares` are worth in `pool` with custom total coins and shares numbers.


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount_with_total_stats(pool: &pool_u64_unbound::Pool, shares: u128, total_coins: u64, total_shares: u128): u64
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount_with_total_stats(
        pool: &Pool,
        shares: u128,
        total_coins: u64,
        total_shares: u128,
    ): u64 {
        if (pool.total_coins == 0 || total_shares == 0) {
            0
        } else {
            (multiply_then_divide(pool, shares, to_u128(total_coins), total_shares) as u64)
        }
    }
}
```


<a id="0x1_pool_u64_unbound_multiply_then_divide"></a>

## Function `multiply_then_divide`



```move
module 0x1::pool_u64_unbound {
    public fun multiply_then_divide(_pool: &pool_u64_unbound::Pool, x: u128, y: u128, z: u128): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    public fun multiply_then_divide(_pool: &Pool, x: u128, y: u128, z: u128): u128 {
        let result = (to_u256(x) * to_u256(y)) / to_u256(z);
        (result as u128)
    }
}
```


<a id="0x1_pool_u64_unbound_to_u128"></a>

## Function `to_u128`



```move
module 0x1::pool_u64_unbound {
    fun to_u128(num: u64): u128
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    fun to_u128(num: u64): u128 {
        (num as u128)
    }
}
```


<a id="0x1_pool_u64_unbound_to_u256"></a>

## Function `to_u256`



```move
module 0x1::pool_u64_unbound {
    fun to_u256(num: u128): u256
}
```


##### Implementation


```move
module 0x1::pool_u64_unbound {
    fun to_u256(num: u128): u256 {
        (num as u256)
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_Pool"></a>

### Struct `Pool`


```move
module 0x1::pool_u64_unbound {
    struct Pool has store
}
```


<dl>
<dt>
`total_coins: u64`
</dt>
<dd>

</dd>
<dt>
`total_shares: u128`
</dt>
<dd>

</dd>
<dt>
`shares: table_with_length::TableWithLength<address, u128>`
</dt>
<dd>

</dd>
<dt>
`scaling_factor: u64`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::pool_u64_unbound {
    invariant forall addr: address:
        table::spec_contains(shares, addr) ==> (table::spec_get(shares, addr) > 0);
}
```



<a id="0x1_pool_u64_unbound_spec_contains"></a>


```move
module 0x1::pool_u64_unbound {
    fun spec_contains(pool: Pool, shareholder: address): bool {
       table::spec_contains(pool.shares, shareholder)
    }
}
```


<a id="@Specification_1_contains"></a>

### Function `contains`


```move
module 0x1::pool_u64_unbound {
    public fun contains(pool: &pool_u64_unbound::Pool, shareholder: address): bool
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if false;
    ensures result == spec_contains(pool, shareholder);
}
```



<a id="0x1_pool_u64_unbound_spec_shares"></a>


```move
module 0x1::pool_u64_unbound {
    fun spec_shares(pool: Pool, shareholder: address): u64 {
       if (spec_contains(pool, shareholder)) {
           table::spec_get(pool.shares, shareholder)
       }
       else {
           0
       }
    }
}
```


<a id="@Specification_1_shares"></a>

### Function `shares`


```move
module 0x1::pool_u64_unbound {
    public fun shares(pool: &pool_u64_unbound::Pool, shareholder: address): u128
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if false;
    ensures result == spec_shares(pool, shareholder);
}
```


<a id="@Specification_1_balance"></a>

### Function `balance`


```move
module 0x1::pool_u64_unbound {
    public fun balance(pool: &pool_u64_unbound::Pool, shareholder: address): u64
}
```



```move
module 0x1::pool_u64_unbound {
    let shares = spec_shares(pool, shareholder);
    let total_coins = pool.total_coins;
    aborts_if pool.total_coins > 0 && pool.total_shares > 0 && (shares * total_coins) / pool.total_shares > MAX_U64;
    ensures result == spec_shares_to_amount_with_total_coins(pool, shares, total_coins);
}
```


<a id="@Specification_1_buy_in"></a>

### Function `buy_in`


```move
module 0x1::pool_u64_unbound {
    public fun buy_in(pool: &mut pool_u64_unbound::Pool, shareholder: address, coins_amount: u64): u128
}
```



```move
module 0x1::pool_u64_unbound {
    let new_shares = spec_amount_to_shares_with_total_coins(pool, coins_amount, pool.total_coins);
    aborts_if pool.total_coins + coins_amount > MAX_U64;
    aborts_if pool.total_shares + new_shares > MAX_U128;
    include coins_amount > 0 ==> AddSharesAbortsIf { new_shares: new_shares };
    include coins_amount > 0 ==> AddSharesEnsures { new_shares: new_shares };
    ensures pool.total_coins == old(pool.total_coins) + coins_amount;
    ensures pool.total_shares == old(pool.total_shares) + new_shares;
    ensures result == new_shares;
}
```


<a id="@Specification_1_add_shares"></a>

### Function `add_shares`


```move
module 0x1::pool_u64_unbound {
    fun add_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, new_shares: u128): u128
}
```



```move
module 0x1::pool_u64_unbound {
    include AddSharesAbortsIf;
    include AddSharesEnsures;
    let key_exists = table::spec_contains(pool.shares, shareholder);
    ensures result == if (key_exists) { table::spec_get(pool.shares, shareholder) }
    else { new_shares };
}
```



<a id="0x1_pool_u64_unbound_AddSharesAbortsIf"></a>


```move
module 0x1::pool_u64_unbound {
    schema AddSharesAbortsIf {
        pool: Pool;
        shareholder: address;
        new_shares: u64;
        let key_exists = table::spec_contains(pool.shares, shareholder);
        let current_shares = table::spec_get(pool.shares, shareholder);
        aborts_if key_exists && current_shares + new_shares > MAX_U128;
    }
}
```



<a id="0x1_pool_u64_unbound_AddSharesEnsures"></a>


```move
module 0x1::pool_u64_unbound {
    schema AddSharesEnsures {
        pool: Pool;
        shareholder: address;
        new_shares: u64;
        let key_exists = table::spec_contains(pool.shares, shareholder);
        let current_shares = table::spec_get(pool.shares, shareholder);
        ensures key_exists ==>
            pool.shares == table::spec_set(old(pool.shares), shareholder, current_shares + new_shares);
        ensures (!key_exists && new_shares > 0) ==>
            pool.shares == table::spec_set(old(pool.shares), shareholder, new_shares);
    }
}
```



<a id="0x1_pool_u64_unbound_spec_amount_to_shares_with_total_coins"></a>


```move
module 0x1::pool_u64_unbound {
    fun spec_amount_to_shares_with_total_coins(pool: Pool, coins_amount: u64, total_coins: u64): u128 {
       if (pool.total_coins == 0 || pool.total_shares == 0) {
           coins_amount * pool.scaling_factor
       }
       else {
           (coins_amount * pool.total_shares) / total_coins
       }
    }
}
```


<a id="@Specification_1_redeem_shares"></a>

### Function `redeem_shares`


```move
module 0x1::pool_u64_unbound {
    public fun redeem_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, shares_to_redeem: u128): u64
}
```



```move
module 0x1::pool_u64_unbound {
    let redeemed_coins = spec_shares_to_amount_with_total_coins(pool, shares_to_redeem, pool.total_coins);
    aborts_if !spec_contains(pool, shareholder);
    aborts_if spec_shares(pool, shareholder) < shares_to_redeem;
    aborts_if pool.total_coins < redeemed_coins;
    aborts_if pool.total_shares < shares_to_redeem;
    ensures pool.total_coins == old(pool.total_coins) - redeemed_coins;
    ensures pool.total_shares == old(pool.total_shares) - shares_to_redeem;
    include shares_to_redeem > 0 ==> DeductSharesEnsures { num_shares: shares_to_redeem };
    ensures result == redeemed_coins;
}
```


<a id="@Specification_1_transfer_shares"></a>

### Function `transfer_shares`


```move
module 0x1::pool_u64_unbound {
    public fun transfer_shares(pool: &mut pool_u64_unbound::Pool, shareholder_1: address, shareholder_2: address, shares_to_transfer: u128)
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if (shareholder_1 != shareholder_2) && shares_to_transfer > 0 && spec_contains(pool, shareholder_2) &&
        (spec_shares(pool, shareholder_2) + shares_to_transfer > MAX_U128);
    aborts_if !spec_contains(pool, shareholder_1);
    aborts_if spec_shares(pool, shareholder_1) < shares_to_transfer;
    ensures shareholder_1 == shareholder_2 ==> spec_shares(old(pool), shareholder_1) == spec_shares(pool, shareholder_1);
    ensures ((shareholder_1 != shareholder_2) && (spec_shares(old(pool), shareholder_1) == shares_to_transfer)) ==>
        !spec_contains(pool, shareholder_1);
    ensures (shareholder_1 != shareholder_2 && shares_to_transfer > 0) ==>
        (spec_contains(pool, shareholder_2));
    ensures (shareholder_1 != shareholder_2 && shares_to_transfer > 0 && !spec_contains(old(pool), shareholder_2)) ==>
        (spec_contains(pool, shareholder_2) && spec_shares(pool, shareholder_2) == shares_to_transfer);
    ensures (shareholder_1 != shareholder_2 && shares_to_transfer > 0 && spec_contains(old(pool), shareholder_2)) ==>
        (spec_contains(pool, shareholder_2) && spec_shares(pool, shareholder_2) == spec_shares(old(pool), shareholder_2) + shares_to_transfer);
    ensures ((shareholder_1 != shareholder_2) && (spec_shares(old(pool), shareholder_1) > shares_to_transfer)) ==>
        (spec_contains(pool, shareholder_1) && (spec_shares(pool, shareholder_1) == spec_shares(old(pool), shareholder_1) - shares_to_transfer));
}
```


<a id="@Specification_1_deduct_shares"></a>

### Function `deduct_shares`


```move
module 0x1::pool_u64_unbound {
    fun deduct_shares(pool: &mut pool_u64_unbound::Pool, shareholder: address, num_shares: u128): u128
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if !spec_contains(pool, shareholder);
    aborts_if spec_shares(pool, shareholder) < num_shares;
    include DeductSharesEnsures;
    let remaining_shares = table::spec_get(pool.shares, shareholder) - num_shares;
    ensures remaining_shares > 0 ==> result == table::spec_get(pool.shares, shareholder);
    ensures remaining_shares == 0 ==> result == 0;
}
```



<a id="0x1_pool_u64_unbound_DeductSharesEnsures"></a>


```move
module 0x1::pool_u64_unbound {
    schema DeductSharesEnsures {
        pool: Pool;
        shareholder: address;
        num_shares: u64;
        let remaining_shares = table::spec_get(pool.shares, shareholder) - num_shares;
        ensures remaining_shares > 0 ==> table::spec_get(pool.shares, shareholder) == remaining_shares;
        ensures remaining_shares == 0 ==> !table::spec_contains(pool.shares, shareholder);
    }
}
```


<a id="@Specification_1_amount_to_shares_with_total_coins"></a>

### Function `amount_to_shares_with_total_coins`


```move
module 0x1::pool_u64_unbound {
    public fun amount_to_shares_with_total_coins(pool: &pool_u64_unbound::Pool, coins_amount: u64, total_coins: u64): u128
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if pool.total_coins > 0 && pool.total_shares > 0
        && (coins_amount * pool.total_shares) / total_coins > MAX_U128;
    aborts_if (pool.total_coins == 0 || pool.total_shares == 0)
        && coins_amount * pool.scaling_factor > MAX_U128;
    aborts_if pool.total_coins > 0 && pool.total_shares > 0 && total_coins == 0;
    ensures result == spec_amount_to_shares_with_total_coins(pool, coins_amount, total_coins);
}
```


<a id="@Specification_1_shares_to_amount_with_total_coins"></a>

### Function `shares_to_amount_with_total_coins`


```move
module 0x1::pool_u64_unbound {
    public fun shares_to_amount_with_total_coins(pool: &pool_u64_unbound::Pool, shares: u128, total_coins: u64): u64
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if pool.total_coins > 0 && pool.total_shares > 0
        && (shares * total_coins) / pool.total_shares > MAX_U64;
    ensures result == spec_shares_to_amount_with_total_coins(pool, shares, total_coins);
}
```



<a id="0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins"></a>


```move
module 0x1::pool_u64_unbound {
    fun spec_shares_to_amount_with_total_coins(pool: Pool, shares: u128, total_coins: u64): u64 {
       if (pool.total_coins == 0 || pool.total_shares == 0) {
           0
       }
       else {
           (shares * total_coins) / pool.total_shares
       }
    }
}
```


<a id="@Specification_1_multiply_then_divide"></a>

### Function `multiply_then_divide`


```move
module 0x1::pool_u64_unbound {
    public fun multiply_then_divide(_pool: &pool_u64_unbound::Pool, x: u128, y: u128, z: u128): u128
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if z == 0;
    aborts_if (x * y) / z > MAX_U128;
    ensures result == (x * y) / z;
}
```


<a id="@Specification_1_to_u128"></a>

### Function `to_u128`


```move
module 0x1::pool_u64_unbound {
    fun to_u128(num: u64): u128
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if false;
    ensures result == num;
}
```


<a id="@Specification_1_to_u256"></a>

### Function `to_u256`


```move
module 0x1::pool_u64_unbound {
    fun to_u256(num: u128): u256
}
```



```move
module 0x1::pool_u64_unbound {
    aborts_if false;
    ensures result == num;
}
```
