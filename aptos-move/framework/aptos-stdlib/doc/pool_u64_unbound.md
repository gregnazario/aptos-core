
<a id="0x1_pool_u64_unbound"></a>

# Module `0x1::pool_u64_unbound`


Simple module for tracking and calculating shares of a pool of coins. The shares are worth more as the total coins in
the pool increases. New shareholder can buy more shares or redeem their existing shares.

Example flow:
1. Pool start outs empty.
2. Shareholder A buys in with 1000 coins. A will receive 1000 shares in the pool. Pool now has 1000 total coins and
1000 total shares.
3. Pool appreciates in value from rewards and now has 2000 coins. A's 1000 shares are now worth 2000 coins.
4. Shareholder B now buys in with 1000 coins. Since before the buy in, each existing share is worth 2 coins, B will
receive 500 shares in exchange for 1000 coins. Pool now has 1500 shares and 3000 coins.
5. Pool appreciates in value from rewards and now has 6000 coins.
6. A redeems 500 shares. Each share is worth 6000 / 1500 = 4. A receives 2000 coins. Pool has 4000 coins and 1000
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


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [table_with_length.md#0x1_table_with_length](0x1::table_with_length);
</code></pre>



<a id="0x1_pool_u64_unbound_Pool"></a>

## Struct `Pool`



<pre><code><b>struct</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>total_coins: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_shares: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>shares: [table_with_length.md#0x1_table_with_length_TableWithLength](table_with_length::TableWithLength)&lt;<b>address</b>, u128&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>scaling_factor: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_pool_u64_unbound_MAX_U64"></a>



<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U64](MAX_U64): u64 = 18446744073709551615;
</code></pre>



<a id="0x1_pool_u64_unbound_MAX_U128"></a>



<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128): u128 = 340282366920938463463374607431768211455;
</code></pre>



<a id="0x1_pool_u64_unbound_EINSUFFICIENT_SHARES"></a>

Cannot redeem more shares than the shareholder has in the pool.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES): u64 = 4;
</code></pre>



<a id="0x1_pool_u64_unbound_EPOOL_IS_NOT_EMPTY"></a>

Cannot destroy non-empty pool.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_IS_NOT_EMPTY](EPOOL_IS_NOT_EMPTY): u64 = 3;
</code></pre>



<a id="0x1_pool_u64_unbound_EPOOL_TOTAL_COINS_OVERFLOW"></a>

Pool's total coins cannot exceed u64.max.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_TOTAL_COINS_OVERFLOW](EPOOL_TOTAL_COINS_OVERFLOW): u64 = 6;
</code></pre>



<a id="0x1_pool_u64_unbound_EPOOL_TOTAL_SHARES_OVERFLOW"></a>

Pool's total shares cannot exceed u64.max.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_TOTAL_SHARES_OVERFLOW](EPOOL_TOTAL_SHARES_OVERFLOW): u64 = 7;
</code></pre>



<a id="0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND"></a>

Shareholder not present in pool.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND): u64 = 1;
</code></pre>



<a id="0x1_pool_u64_unbound_ESHAREHOLDER_SHARES_OVERFLOW"></a>

Shareholder cannot have more than u64.max shares.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_SHARES_OVERFLOW](ESHAREHOLDER_SHARES_OVERFLOW): u64 = 5;
</code></pre>



<a id="0x1_pool_u64_unbound_ETOO_MANY_SHAREHOLDERS"></a>

There are too many shareholders in the pool.


<pre><code><b>const</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_ETOO_MANY_SHAREHOLDERS](ETOO_MANY_SHAREHOLDERS): u64 = 2;
</code></pre>



<a id="0x1_pool_u64_unbound_new"></a>

## Function `new`

Create a new pool.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_new](new)(): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_new](new)(): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) {
    // Default <b>to</b> a scaling factor of 1 (effectively no scaling).
    [pool_u64_unbound.md#0x1_pool_u64_unbound_create_with_scaling_factor](create_with_scaling_factor)(1)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_create"></a>

## Function `create`

Deprecated. Use <code>new</code> instead.
Create a new pool.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_create](create)(): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_create](create)(): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) {
    [pool_u64_unbound.md#0x1_pool_u64_unbound_new](new)()
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_create_with_scaling_factor"></a>

## Function `create_with_scaling_factor`

Create a new pool with custom <code>scaling_factor</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_create_with_scaling_factor](create_with_scaling_factor)(scaling_factor: u64): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_create_with_scaling_factor](create_with_scaling_factor)(scaling_factor: u64): [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) {
    [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) {
        total_coins: 0,
        total_shares: 0,
        shares: [table.md#0x1_table_new](table::new)&lt;<b>address</b>, u128&gt;(),
        scaling_factor,
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_destroy_empty"></a>

## Function `destroy_empty`

Destroy an empty pool. This will fail if the pool has any balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_destroy_empty](destroy_empty)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_destroy_empty](destroy_empty)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool)) {
    <b>assert</b>!(pool.total_coins == 0, [../../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_IS_NOT_EMPTY](EPOOL_IS_NOT_EMPTY)));
    <b>let</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) {
        total_coins: _,
        total_shares: _,
        shares,
        scaling_factor: _,
    } = pool;
    table::destroy_empty&lt;<b>address</b>, u128&gt;(shares);
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_total_coins"></a>

## Function `total_coins`

Return <code>pool</code>'s total balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_total_coins](total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_total_coins](total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool)): u64 {
    pool.total_coins
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_total_shares"></a>

## Function `total_shares`

Return the total number of shares across all shareholders in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_total_shares](total_shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_total_shares](total_shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool)): u128 {
    pool.total_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_contains"></a>

## Function `contains`

Return true if <code>shareholder</code> is in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>): bool {
    [table.md#0x1_table_contains](table::contains)(&pool.shares, shareholder)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_shares"></a>

## Function `shares`

Return the number of shares of <code>stakeholder</code> in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>): u128 {
    <b>if</b> ([pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool, shareholder)) {
        *[table.md#0x1_table_borrow](table::borrow)(&pool.shares, shareholder)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_balance"></a>

## Function `balance`

Return the balance in coins of <code>shareholder</code> in <code>pool.</code>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_balance](balance)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_balance](balance)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>): u64 {
    <b>let</b> num_shares = [pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool, shareholder);
    [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount](shares_to_amount)(pool, num_shares)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_shareholders_count"></a>

## Function `shareholders_count`

Return the number of shareholders in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shareholders_count](shareholders_count)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shareholders_count](shareholders_count)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool)): u64 {
    table::length(&pool.shares)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_update_total_coins"></a>

## Function `update_total_coins`

Update <code>pool</code>'s total balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_update_total_coins](update_total_coins)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), new_total_coins: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_update_total_coins](update_total_coins)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), new_total_coins: u64) {
    pool.total_coins = new_total_coins;
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_buy_in"></a>

## Function `buy_in`

Allow an existing or new shareholder to add their coins to the pool in exchange for new shares.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, coins_amount: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>, coins_amount: u64): u128 {
    <b>if</b> (coins_amount == 0) <b>return</b> 0;

    <b>let</b> new_shares = [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares](amount_to_shares)(pool, coins_amount);
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U64](MAX_U64) - pool.total_coins &gt;= coins_amount, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_TOTAL_COINS_OVERFLOW](EPOOL_TOTAL_COINS_OVERFLOW)));
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128) - pool.total_shares &gt;= new_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_EPOOL_TOTAL_SHARES_OVERFLOW](EPOOL_TOTAL_SHARES_OVERFLOW)));

    pool.total_coins = pool.total_coins + coins_amount;
    pool.total_shares = pool.total_shares + new_shares;
    [pool_u64_unbound.md#0x1_pool_u64_unbound_add_shares](add_shares)(pool, shareholder, new_shares);
    new_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_add_shares"></a>

## Function `add_shares`

Add the number of shares directly for <code>shareholder</code> in <code>pool</code>.
This would dilute other shareholders if the pool's balance of coins didn't change.


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, new_shares: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>, new_shares: u128): u128 {
    <b>if</b> ([pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool, shareholder)) {
        <b>let</b> existing_shares = [table.md#0x1_table_borrow_mut](table::borrow_mut)(&<b>mut</b> pool.shares, shareholder);
        <b>let</b> current_shares = *existing_shares;
        <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128) - current_shares &gt;= new_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_SHARES_OVERFLOW](ESHAREHOLDER_SHARES_OVERFLOW)));

        *existing_shares = current_shares + new_shares;
        *existing_shares
    } <b>else</b> <b>if</b> (new_shares &gt; 0) {
        [table.md#0x1_table_add](table::add)(&<b>mut</b> pool.shares, shareholder, new_shares);
        new_shares
    } <b>else</b> {
        new_shares
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_redeem_shares"></a>

## Function `redeem_shares`

Allow <code>shareholder</code> to redeem their shares in <code>pool</code> for coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, shares_to_redeem: u128): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>, shares_to_redeem: u128): u64 {
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool, shareholder), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool, shareholder) &gt;= shares_to_redeem, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));

    <b>if</b> (shares_to_redeem == 0) <b>return</b> 0;

    <b>let</b> redeemed_coins = [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount](shares_to_amount)(pool, shares_to_redeem);
    pool.total_coins = pool.total_coins - redeemed_coins;
    pool.total_shares = pool.total_shares - shares_to_redeem;
    [pool_u64_unbound.md#0x1_pool_u64_unbound_deduct_shares](deduct_shares)(pool, shareholder, shares_to_redeem);

    redeemed_coins
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_transfer_shares"></a>

## Function `transfer_shares`

Transfer shares from <code>shareholder_1</code> to <code>shareholder_2</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_transfer_shares](transfer_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder_1: <b>address</b>, shareholder_2: <b>address</b>, shares_to_transfer: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_transfer_shares](transfer_shares)(
    pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool),
    shareholder_1: <b>address</b>,
    shareholder_2: <b>address</b>,
    shares_to_transfer: u128,
) {
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool, shareholder_1), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool, shareholder_1) &gt;= shares_to_transfer, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));
    <b>if</b> (shares_to_transfer == 0) <b>return</b>;

    [pool_u64_unbound.md#0x1_pool_u64_unbound_deduct_shares](deduct_shares)(pool, shareholder_1, shares_to_transfer);
    [pool_u64_unbound.md#0x1_pool_u64_unbound_add_shares](add_shares)(pool, shareholder_2, shares_to_transfer);
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_deduct_shares"></a>

## Function `deduct_shares`

Directly deduct <code>shareholder</code>'s number of shares in <code>pool</code> and return the number of remaining shares.


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, num_shares: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>, num_shares: u128): u128 {
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool, shareholder), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool, shareholder) &gt;= num_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64_unbound.md#0x1_pool_u64_unbound_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));

    <b>let</b> existing_shares = [table.md#0x1_table_borrow_mut](table::borrow_mut)(&<b>mut</b> pool.shares, shareholder);
    *existing_shares = *existing_shares - num_shares;

    // Remove the shareholder completely <b>if</b> they have no shares left.
    <b>let</b> remaining_shares = *existing_shares;
    <b>if</b> (remaining_shares == 0) {
        [table.md#0x1_table_remove](table::remove)(&<b>mut</b> pool.shares, shareholder);
    };

    remaining_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_amount_to_shares"></a>

## Function `amount_to_shares`

Return the number of new shares <code>coins_amount</code> can buy in <code>pool</code>.
<code>amount</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares](amount_to_shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), coins_amount: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares](amount_to_shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), coins_amount: u64): u128 {
    [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool, coins_amount, pool.total_coins)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_amount_to_shares_with_total_coins"></a>

## Function `amount_to_shares_with_total_coins`

Return the number of new shares <code>coins_amount</code> can buy in <code>pool</code> with a custom total coins number.
<code>amount</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), coins_amount: u64, total_coins: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), coins_amount: u64, total_coins: u64): u128 {
    // No shares yet so amount is worth the same number of shares.
    <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
        // Multiply by scaling factor <b>to</b> minimize rounding errors during <b>internal</b> calculations for buy ins/redeems.
        // This can overflow but scaling factor is expected <b>to</b> be chosen carefully so this would not overflow.
        [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(coins_amount) * [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(pool.scaling_factor)
    } <b>else</b> {
        // Shares price = total_coins / total existing shares.
        // New number of shares = new_amount / shares_price = new_amount * existing_shares / total_amount.
        // We rearrange the calc and do multiplication first <b>to</b> avoid rounding errors.
        [pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(pool, [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(coins_amount), pool.total_shares, [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(total_coins))
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_shares_to_amount"></a>

## Function `shares_to_amount`

Return the number of coins <code>shares</code> are worth in <code>pool</code>.
<code>shares</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount](shares_to_amount)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shares: u128): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount](shares_to_amount)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shares: u128): u64 {
    [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool, shares, pool.total_coins)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_shares_to_amount_with_total_coins"></a>

## Function `shares_to_amount_with_total_coins`

Return the number of coins <code>shares</code> are worth in <code>pool</code> with a custom total coins number.
<code>shares</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shares: u128, total_coins: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shares: u128, total_coins: u64): u64 {
    // No shares or coins yet so shares are worthless.
    <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
        0
    } <b>else</b> {
        // Shares price = total_coins / total existing shares.
        // Shares worth = shares * shares price = shares * total_coins / total existing shares.
        // We rearrange the calc and do multiplication first <b>to</b> avoid rounding errors.
        ([pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(pool, shares, [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(total_coins), pool.total_shares) <b>as</b> u64)
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_shares_to_amount_with_total_stats"></a>

## Function `shares_to_amount_with_total_stats`

Return the number of coins <code>shares</code> are worth in <code>pool</code> with custom total coins and shares numbers.


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_stats](shares_to_amount_with_total_stats)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shares: u128, total_coins: u64, total_shares: u128): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_stats](shares_to_amount_with_total_stats)(
    pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool),
    shares: u128,
    total_coins: u64,
    total_shares: u128,
): u64 {
    <b>if</b> (pool.total_coins == 0 || total_shares == 0) {
        0
    } <b>else</b> {
        ([pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(pool, shares, [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(total_coins), total_shares) <b>as</b> u64)
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_multiply_then_divide"></a>

## Function `multiply_then_divide`



<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), x: u128, y: u128, z: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), x: u128, y: u128, z: u128): u128 {
    <b>let</b> result = ([pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(x) * [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(y)) / [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(z);
    (result <b>as</b> u128)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_to_u128"></a>

## Function `to_u128`



<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(num: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(num: u64): u128 {
    (num <b>as</b> u128)
}
</code></pre>



</details>

<a id="0x1_pool_u64_unbound_to_u256"></a>

## Function `to_u256`



<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(num: u128): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(num: u128): u256 {
    (num <b>as</b> u256)
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_Pool"></a>

### Struct `Pool`


<pre><code><b>struct</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool) <b>has</b> store
</code></pre>



<dl>
<dt>
<code>total_coins: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_shares: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>shares: [table_with_length.md#0x1_table_with_length_TableWithLength](table_with_length::TableWithLength)&lt;<b>address</b>, u128&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>scaling_factor: u64</code>
</dt>
<dd>

</dd>
</dl>



<pre><code><b>invariant</b> <b>forall</b> addr: <b>address</b>:
    [table.md#0x1_table_spec_contains](table::spec_contains)(shares, addr) ==&gt; ([table.md#0x1_table_spec_get](table::spec_get)(shares, addr) &gt; 0);
</code></pre>




<a id="0x1_pool_u64_unbound_spec_contains"></a>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>): bool {
   [table.md#0x1_table_spec_contains](table::spec_contains)(pool.shares, shareholder)
}
</code></pre>



<a id="@Specification_1_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_contains](contains)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder);
</code></pre>




<a id="0x1_pool_u64_unbound_spec_shares"></a>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shareholder: <b>address</b>): u64 {
   <b>if</b> ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder)) {
       [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder)
   }
   <b>else</b> {
       0
   }
}
</code></pre>



<a id="@Specification_1_shares"></a>

### Function `shares`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares](shares)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): u128
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder);
</code></pre>



<a id="@Specification_1_balance"></a>

### Function `balance`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_balance](balance)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>): u64
</code></pre>




<pre><code><b>let</b> shares = [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder);
<b>let</b> total_coins = pool.total_coins;
<b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0 && (shares * total_coins) / pool.total_shares &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U64](MAX_U64);
<b>ensures</b> result == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares, total_coins);
</code></pre>



<a id="@Specification_1_buy_in"></a>

### Function `buy_in`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, coins_amount: u64): u128
</code></pre>




<pre><code><b>let</b> new_shares = [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool, coins_amount, pool.total_coins);
<b>aborts_if</b> pool.total_coins + coins_amount &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U64](MAX_U64);
<b>aborts_if</b> pool.total_shares + new_shares &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128);
<b>include</b> coins_amount &gt; 0 ==&gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesAbortsIf](AddSharesAbortsIf) { new_shares: new_shares };
<b>include</b> coins_amount &gt; 0 ==&gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesEnsures](AddSharesEnsures) { new_shares: new_shares };
<b>ensures</b> pool.total_coins == <b>old</b>(pool.total_coins) + coins_amount;
<b>ensures</b> pool.total_shares == <b>old</b>(pool.total_shares) + new_shares;
<b>ensures</b> result == new_shares;
</code></pre>



<a id="@Specification_1_add_shares"></a>

### Function `add_shares`


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, new_shares: u128): u128
</code></pre>




<pre><code><b>include</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesAbortsIf](AddSharesAbortsIf);
<b>include</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesEnsures](AddSharesEnsures);
<b>let</b> key_exists = [table.md#0x1_table_spec_contains](table::spec_contains)(pool.shares, shareholder);
<b>ensures</b> result == <b>if</b> (key_exists) { [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder) }
<b>else</b> { new_shares };
</code></pre>




<a id="0x1_pool_u64_unbound_AddSharesAbortsIf"></a>


<pre><code><b>schema</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesAbortsIf](AddSharesAbortsIf) {
    pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool);
    shareholder: <b>address</b>;
    new_shares: u64;
    <b>let</b> key_exists = [table.md#0x1_table_spec_contains](table::spec_contains)(pool.shares, shareholder);
    <b>let</b> current_shares = [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder);
    <b>aborts_if</b> key_exists && current_shares + new_shares &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128);
}
</code></pre>




<a id="0x1_pool_u64_unbound_AddSharesEnsures"></a>


<pre><code><b>schema</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_AddSharesEnsures](AddSharesEnsures) {
    pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool);
    shareholder: <b>address</b>;
    new_shares: u64;
    <b>let</b> key_exists = [table.md#0x1_table_spec_contains](table::spec_contains)(pool.shares, shareholder);
    <b>let</b> current_shares = [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder);
    <b>ensures</b> key_exists ==&gt;
        pool.shares == [table.md#0x1_table_spec_set](table::spec_set)(<b>old</b>(pool.shares), shareholder, current_shares + new_shares);
    <b>ensures</b> (!key_exists && new_shares &gt; 0) ==&gt;
        pool.shares == [table.md#0x1_table_spec_set](table::spec_set)(<b>old</b>(pool.shares), shareholder, new_shares);
}
</code></pre>




<a id="0x1_pool_u64_unbound_spec_amount_to_shares_with_total_coins"></a>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), coins_amount: u64, total_coins: u64): u128 {
   <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
       coins_amount * pool.scaling_factor
   }
   <b>else</b> {
       (coins_amount * pool.total_shares) / total_coins
   }
}
</code></pre>



<a id="@Specification_1_redeem_shares"></a>

### Function `redeem_shares`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, shares_to_redeem: u128): u64
</code></pre>




<pre><code><b>let</b> redeemed_coins = [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares_to_redeem, pool.total_coins);
<b>aborts_if</b> ![pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder);
<b>aborts_if</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder) &lt; shares_to_redeem;
<b>aborts_if</b> pool.[pool_u64_unbound.md#0x1_pool_u64_unbound_total_coins](total_coins) &lt; redeemed_coins;
<b>aborts_if</b> pool.[pool_u64_unbound.md#0x1_pool_u64_unbound_total_shares](total_shares) &lt; shares_to_redeem;
<b>ensures</b> pool.total_coins == <b>old</b>(pool.total_coins) - redeemed_coins;
<b>ensures</b> pool.total_shares == <b>old</b>(pool.total_shares) - shares_to_redeem;
<b>include</b> shares_to_redeem &gt; 0 ==&gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_DeductSharesEnsures](DeductSharesEnsures) { num_shares: shares_to_redeem };
<b>ensures</b> result == redeemed_coins;
</code></pre>



<a id="@Specification_1_transfer_shares"></a>

### Function `transfer_shares`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_transfer_shares](transfer_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder_1: <b>address</b>, shareholder_2: <b>address</b>, shares_to_transfer: u128)
</code></pre>




<pre><code><b>aborts_if</b> (shareholder_1 != shareholder_2) && shares_to_transfer &gt; 0 && [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_2) &&
    ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_2) + shares_to_transfer &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128));
<b>aborts_if</b> ![pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_1);
<b>aborts_if</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_1) &lt; shares_to_transfer;
<b>ensures</b> shareholder_1 == shareholder_2 ==&gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(<b>old</b>(pool), shareholder_1) == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_1);
<b>ensures</b> ((shareholder_1 != shareholder_2) && ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(<b>old</b>(pool), shareholder_1) == shares_to_transfer)) ==&gt;
    ![pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_1);
<b>ensures</b> (shareholder_1 != shareholder_2 && shares_to_transfer &gt; 0) ==&gt;
    ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_2));
<b>ensures</b> (shareholder_1 != shareholder_2 && shares_to_transfer &gt; 0 && ![pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(<b>old</b>(pool), shareholder_2)) ==&gt;
    ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_2) && [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_2) == shares_to_transfer);
<b>ensures</b> (shareholder_1 != shareholder_2 && shares_to_transfer &gt; 0 && [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(<b>old</b>(pool), shareholder_2)) ==&gt;
    ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_2) && [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_2) == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(<b>old</b>(pool), shareholder_2) + shares_to_transfer);
<b>ensures</b> ((shareholder_1 != shareholder_2) && ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(<b>old</b>(pool), shareholder_1) &gt; shares_to_transfer)) ==&gt;
    ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder_1) && ([pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder_1) == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(<b>old</b>(pool), shareholder_1) - shares_to_transfer));
</code></pre>



<a id="@Specification_1_deduct_shares"></a>

### Function `deduct_shares`


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shareholder: <b>address</b>, num_shares: u128): u128
</code></pre>




<pre><code><b>aborts_if</b> ![pool_u64_unbound.md#0x1_pool_u64_unbound_spec_contains](spec_contains)(pool, shareholder);
<b>aborts_if</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares](spec_shares)(pool, shareholder) &lt; num_shares;
<b>include</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_DeductSharesEnsures](DeductSharesEnsures);
<b>let</b> remaining_shares = [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder) - num_shares;
<b>ensures</b> remaining_shares &gt; 0 ==&gt; result == [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder);
<b>ensures</b> remaining_shares == 0 ==&gt; result == 0;
</code></pre>




<a id="0x1_pool_u64_unbound_DeductSharesEnsures"></a>


<pre><code><b>schema</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_DeductSharesEnsures](DeductSharesEnsures) {
    pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool);
    shareholder: <b>address</b>;
    num_shares: u64;
    <b>let</b> remaining_shares = [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder) - num_shares;
    <b>ensures</b> remaining_shares &gt; 0 ==&gt; [table.md#0x1_table_spec_get](table::spec_get)(pool.shares, shareholder) == remaining_shares;
    <b>ensures</b> remaining_shares == 0 ==&gt; ![table.md#0x1_table_spec_contains](table::spec_contains)(pool.shares, shareholder);
}
</code></pre>



<a id="@Specification_1_amount_to_shares_with_total_coins"></a>

### Function `amount_to_shares_with_total_coins`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), coins_amount: u64, total_coins: u64): u128
</code></pre>




<pre><code><b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0
    && (coins_amount * pool.total_shares) / total_coins &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128);
<b>aborts_if</b> (pool.total_coins == 0 || pool.total_shares == 0)
    && coins_amount * pool.scaling_factor &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128);
<b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0 && total_coins == 0;
<b>ensures</b> result == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool, coins_amount, total_coins);
</code></pre>



<a id="@Specification_1_shares_to_amount_with_total_coins"></a>

### Function `shares_to_amount_with_total_coins`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), shares: u128, total_coins: u64): u64
</code></pre>




<pre><code><b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0
    && (shares * total_coins) / pool.total_shares &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U64](MAX_U64);
<b>ensures</b> result == [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares, total_coins);
</code></pre>




<a id="0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins"></a>


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool: [pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](Pool), shares: u128, total_coins: u64): u64 {
   <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
       0
   }
   <b>else</b> {
       (shares * total_coins) / pool.total_shares
   }
}
</code></pre>



<a id="@Specification_1_multiply_then_divide"></a>

### Function `multiply_then_divide`


<pre><code><b>public</b> <b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64_unbound.md#0x1_pool_u64_unbound_Pool](pool_u64_unbound::Pool), x: u128, y: u128, z: u128): u128
</code></pre>




<pre><code><b>aborts_if</b> z == 0;
<b>aborts_if</b> (x * y) / z &gt; [pool_u64_unbound.md#0x1_pool_u64_unbound_MAX_U128](MAX_U128);
<b>ensures</b> result == (x * y) / z;
</code></pre>



<a id="@Specification_1_to_u128"></a>

### Function `to_u128`


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u128](to_u128)(num: u64): u128
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == num;
</code></pre>



<a id="@Specification_1_to_u256"></a>

### Function `to_u256`


<pre><code><b>fun</b> [pool_u64_unbound.md#0x1_pool_u64_unbound_to_u256](to_u256)(num: u128): u256
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == num;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
