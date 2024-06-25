
<a id="0x1_pool_u64"></a>

# Module `0x1::pool_u64`


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


-  [Struct `Pool`](#0x1_pool_u64_Pool)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_pool_u64_new)
-  [Function `create`](#0x1_pool_u64_create)
-  [Function `create_with_scaling_factor`](#0x1_pool_u64_create_with_scaling_factor)
-  [Function `destroy_empty`](#0x1_pool_u64_destroy_empty)
-  [Function `total_coins`](#0x1_pool_u64_total_coins)
-  [Function `total_shares`](#0x1_pool_u64_total_shares)
-  [Function `contains`](#0x1_pool_u64_contains)
-  [Function `shares`](#0x1_pool_u64_shares)
-  [Function `balance`](#0x1_pool_u64_balance)
-  [Function `shareholders`](#0x1_pool_u64_shareholders)
-  [Function `shareholders_count`](#0x1_pool_u64_shareholders_count)
-  [Function `update_total_coins`](#0x1_pool_u64_update_total_coins)
-  [Function `buy_in`](#0x1_pool_u64_buy_in)
-  [Function `add_shares`](#0x1_pool_u64_add_shares)
-  [Function `redeem_shares`](#0x1_pool_u64_redeem_shares)
-  [Function `transfer_shares`](#0x1_pool_u64_transfer_shares)
-  [Function `deduct_shares`](#0x1_pool_u64_deduct_shares)
-  [Function `amount_to_shares`](#0x1_pool_u64_amount_to_shares)
-  [Function `amount_to_shares_with_total_coins`](#0x1_pool_u64_amount_to_shares_with_total_coins)
-  [Function `shares_to_amount`](#0x1_pool_u64_shares_to_amount)
-  [Function `shares_to_amount_with_total_coins`](#0x1_pool_u64_shares_to_amount_with_total_coins)
-  [Function `multiply_then_divide`](#0x1_pool_u64_multiply_then_divide)
-  [Function `to_u128`](#0x1_pool_u64_to_u128)
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


<pre><code><b>use</b> [../../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [simple_map.md#0x1_simple_map](0x1::simple_map);
<b>use</b> [../../move-stdlib/doc/vector.md#0x1_vector](0x1::vector);
</code></pre>



<a id="0x1_pool_u64_Pool"></a>

## Struct `Pool`



<pre><code><b>struct</b> [pool_u64.md#0x1_pool_u64_Pool](Pool) <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>shareholders_limit: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_coins: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_shares: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>shares: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>shareholders: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;<b>address</b>&gt;</code>
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


<a id="0x1_pool_u64_MAX_U64"></a>



<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64): u64 = 18446744073709551615;
</code></pre>



<a id="0x1_pool_u64_EINSUFFICIENT_SHARES"></a>

Cannot redeem more shares than the shareholder has in the pool.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES): u64 = 4;
</code></pre>



<a id="0x1_pool_u64_EPOOL_IS_NOT_EMPTY"></a>

Cannot destroy non-empty pool.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_EPOOL_IS_NOT_EMPTY](EPOOL_IS_NOT_EMPTY): u64 = 3;
</code></pre>



<a id="0x1_pool_u64_EPOOL_TOTAL_COINS_OVERFLOW"></a>

Pool's total coins cannot exceed u64.max.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_EPOOL_TOTAL_COINS_OVERFLOW](EPOOL_TOTAL_COINS_OVERFLOW): u64 = 6;
</code></pre>



<a id="0x1_pool_u64_EPOOL_TOTAL_SHARES_OVERFLOW"></a>

Pool's total shares cannot exceed u64.max.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_EPOOL_TOTAL_SHARES_OVERFLOW](EPOOL_TOTAL_SHARES_OVERFLOW): u64 = 7;
</code></pre>



<a id="0x1_pool_u64_ESHAREHOLDER_NOT_FOUND"></a>

Shareholder not present in pool.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND): u64 = 1;
</code></pre>



<a id="0x1_pool_u64_ESHAREHOLDER_SHARES_OVERFLOW"></a>

Shareholder cannot have more than u64.max shares.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_ESHAREHOLDER_SHARES_OVERFLOW](ESHAREHOLDER_SHARES_OVERFLOW): u64 = 5;
</code></pre>



<a id="0x1_pool_u64_ETOO_MANY_SHAREHOLDERS"></a>

There are too many shareholders in the pool.


<pre><code><b>const</b> [pool_u64.md#0x1_pool_u64_ETOO_MANY_SHAREHOLDERS](ETOO_MANY_SHAREHOLDERS): u64 = 2;
</code></pre>



<a id="0x1_pool_u64_new"></a>

## Function `new`

Create a new pool.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_new](new)(shareholders_limit: u64): [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_new](new)(shareholders_limit: u64): [pool_u64.md#0x1_pool_u64_Pool](Pool) {
    // Default <b>to</b> a scaling factor of 1 (effectively no scaling).
    [pool_u64.md#0x1_pool_u64_create_with_scaling_factor](create_with_scaling_factor)(shareholders_limit, 1)
}
</code></pre>



</details>

<a id="0x1_pool_u64_create"></a>

## Function `create`

Deprecated. Use <code>new</code> instead.
Create a new pool.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_create](create)(shareholders_limit: u64): [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_create](create)(shareholders_limit: u64): [pool_u64.md#0x1_pool_u64_Pool](Pool) {
    [pool_u64.md#0x1_pool_u64_new](new)(shareholders_limit)
}
</code></pre>



</details>

<a id="0x1_pool_u64_create_with_scaling_factor"></a>

## Function `create_with_scaling_factor`

Create a new pool with custom <code>scaling_factor</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_create_with_scaling_factor](create_with_scaling_factor)(shareholders_limit: u64, scaling_factor: u64): [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_create_with_scaling_factor](create_with_scaling_factor)(shareholders_limit: u64, scaling_factor: u64): [pool_u64.md#0x1_pool_u64_Pool](Pool) {
    [pool_u64.md#0x1_pool_u64_Pool](Pool) {
        shareholders_limit,
        total_coins: 0,
        total_shares: 0,
        shares: [simple_map.md#0x1_simple_map_create](simple_map::create)&lt;<b>address</b>, u64&gt;(),
        shareholders: [../../move-stdlib/doc/vector.md#0x1_vector_empty](vector::empty)&lt;<b>address</b>&gt;(),
        scaling_factor,
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_destroy_empty"></a>

## Function `destroy_empty`

Destroy an empty pool. This will fail if the pool has any balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_destroy_empty](destroy_empty)(pool: [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_destroy_empty](destroy_empty)(pool: [pool_u64.md#0x1_pool_u64_Pool](Pool)) {
    <b>assert</b>!(pool.total_coins == 0, [../../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([pool_u64.md#0x1_pool_u64_EPOOL_IS_NOT_EMPTY](EPOOL_IS_NOT_EMPTY)));
    <b>let</b> [pool_u64.md#0x1_pool_u64_Pool](Pool) {
        shareholders_limit: _,
        total_coins: _,
        total_shares: _,
        shares: _,
        shareholders: _,
        scaling_factor: _,
    } = pool;
}
</code></pre>



</details>

<a id="0x1_pool_u64_total_coins"></a>

## Function `total_coins`

Return <code>pool</code>'s total balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_total_coins](total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_total_coins](total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool)): u64 {
    pool.total_coins
}
</code></pre>



</details>

<a id="0x1_pool_u64_total_shares"></a>

## Function `total_shares`

Return the total number of shares across all shareholders in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_total_shares](total_shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_total_shares](total_shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool)): u64 {
    pool.total_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_contains"></a>

## Function `contains`

Return true if <code>shareholder</code> is in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_contains](contains)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_contains](contains)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>): bool {
    [simple_map.md#0x1_simple_map_contains_key](simple_map::contains_key)(&pool.shares, &shareholder)
}
</code></pre>



</details>

<a id="0x1_pool_u64_shares"></a>

## Function `shares`

Return the number of shares of <code>stakeholder</code> in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares](shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares](shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>): u64 {
    <b>if</b> ([pool_u64.md#0x1_pool_u64_contains](contains)(pool, shareholder)) {
        *[simple_map.md#0x1_simple_map_borrow](simple_map::borrow)(&pool.shares, &shareholder)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_balance"></a>

## Function `balance`

Return the balance in coins of <code>shareholder</code> in <code>pool.</code>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_balance](balance)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_balance](balance)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>): u64 {
    <b>let</b> num_shares = [pool_u64.md#0x1_pool_u64_shares](shares)(pool, shareholder);
    [pool_u64.md#0x1_pool_u64_shares_to_amount](shares_to_amount)(pool, num_shares)
}
</code></pre>



</details>

<a id="0x1_pool_u64_shareholders"></a>

## Function `shareholders`

Return the list of shareholders in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shareholders](shareholders)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shareholders](shareholders)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool)): [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;<b>address</b>&gt; {
    pool.shareholders
}
</code></pre>



</details>

<a id="0x1_pool_u64_shareholders_count"></a>

## Function `shareholders_count`

Return the number of shareholders in <code>pool</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shareholders_count](shareholders_count)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool)): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shareholders_count](shareholders_count)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool)): u64 {
    [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&pool.shareholders)
}
</code></pre>



</details>

<a id="0x1_pool_u64_update_total_coins"></a>

## Function `update_total_coins`

Update <code>pool</code>'s total balance of coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_update_total_coins](update_total_coins)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), new_total_coins: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_update_total_coins](update_total_coins)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool), new_total_coins: u64) {
    pool.total_coins = new_total_coins;
}
</code></pre>



</details>

<a id="0x1_pool_u64_buy_in"></a>

## Function `buy_in`

Allow an existing or new shareholder to add their coins to the pool in exchange for new shares.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, coins_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>, coins_amount: u64): u64 {
    <b>if</b> (coins_amount == 0) <b>return</b> 0;

    <b>let</b> new_shares = [pool_u64.md#0x1_pool_u64_amount_to_shares](amount_to_shares)(pool, coins_amount);
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64) - pool.total_coins &gt;= coins_amount, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_EPOOL_TOTAL_COINS_OVERFLOW](EPOOL_TOTAL_COINS_OVERFLOW)));
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64) - pool.total_shares &gt;= new_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_EPOOL_TOTAL_COINS_OVERFLOW](EPOOL_TOTAL_COINS_OVERFLOW)));

    pool.total_coins = pool.total_coins + coins_amount;
    pool.total_shares = pool.total_shares + new_shares;
    [pool_u64.md#0x1_pool_u64_add_shares](add_shares)(pool, shareholder, new_shares);
    new_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_add_shares"></a>

## Function `add_shares`

Add the number of shares directly for <code>shareholder</code> in <code>pool</code>.
This would dilute other shareholders if the pool's balance of coins didn't change.


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, new_shares: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>, new_shares: u64): u64 {
    <b>if</b> ([pool_u64.md#0x1_pool_u64_contains](contains)(pool, shareholder)) {
        <b>let</b> existing_shares = [simple_map.md#0x1_simple_map_borrow_mut](simple_map::borrow_mut)(&<b>mut</b> pool.shares, &shareholder);
        <b>let</b> current_shares = *existing_shares;
        <b>assert</b>!([pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64) - current_shares &gt;= new_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_ESHAREHOLDER_SHARES_OVERFLOW](ESHAREHOLDER_SHARES_OVERFLOW)));

        *existing_shares = current_shares + new_shares;
        *existing_shares
    } <b>else</b> <b>if</b> (new_shares &gt; 0) {
        <b>assert</b>!(
            [../../move-stdlib/doc/vector.md#0x1_vector_length](vector::length)(&pool.shareholders) &lt; pool.shareholders_limit,
            [../../move-stdlib/doc/error.md#0x1_error_invalid_state](error::invalid_state)([pool_u64.md#0x1_pool_u64_ETOO_MANY_SHAREHOLDERS](ETOO_MANY_SHAREHOLDERS)),
        );

        [../../move-stdlib/doc/vector.md#0x1_vector_push_back](vector::push_back)(&<b>mut</b> pool.shareholders, shareholder);
        [simple_map.md#0x1_simple_map_add](simple_map::add)(&<b>mut</b> pool.shares, shareholder, new_shares);
        new_shares
    } <b>else</b> {
        new_shares
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_redeem_shares"></a>

## Function `redeem_shares`

Allow <code>shareholder</code> to redeem their shares in <code>pool</code> for coins.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, shares_to_redeem: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>, shares_to_redeem: u64): u64 {
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_contains](contains)(pool, shareholder), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_shares](shares)(pool, shareholder) &gt;= shares_to_redeem, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));

    <b>if</b> (shares_to_redeem == 0) <b>return</b> 0;

    <b>let</b> redeemed_coins = [pool_u64.md#0x1_pool_u64_shares_to_amount](shares_to_amount)(pool, shares_to_redeem);
    pool.total_coins = pool.total_coins - redeemed_coins;
    pool.total_shares = pool.total_shares - shares_to_redeem;
    [pool_u64.md#0x1_pool_u64_deduct_shares](deduct_shares)(pool, shareholder, shares_to_redeem);

    redeemed_coins
}
</code></pre>



</details>

<a id="0x1_pool_u64_transfer_shares"></a>

## Function `transfer_shares`

Transfer shares from <code>shareholder_1</code> to <code>shareholder_2</code>.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_transfer_shares](transfer_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder_1: <b>address</b>, shareholder_2: <b>address</b>, shares_to_transfer: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_transfer_shares](transfer_shares)(
    pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool),
    shareholder_1: <b>address</b>,
    shareholder_2: <b>address</b>,
    shares_to_transfer: u64,
) {
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_contains](contains)(pool, shareholder_1), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_shares](shares)(pool, shareholder_1) &gt;= shares_to_transfer, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));
    <b>if</b> (shares_to_transfer == 0) <b>return</b>;

    [pool_u64.md#0x1_pool_u64_deduct_shares](deduct_shares)(pool, shareholder_1, shares_to_transfer);
    [pool_u64.md#0x1_pool_u64_add_shares](add_shares)(pool, shareholder_2, shares_to_transfer);
}
</code></pre>



</details>

<a id="0x1_pool_u64_deduct_shares"></a>

## Function `deduct_shares`

Directly deduct <code>shareholder</code>'s number of shares in <code>pool</code> and return the number of remaining shares.


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, num_shares: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>, num_shares: u64): u64 {
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_contains](contains)(pool, shareholder), [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_ESHAREHOLDER_NOT_FOUND](ESHAREHOLDER_NOT_FOUND)));
    <b>assert</b>!([pool_u64.md#0x1_pool_u64_shares](shares)(pool, shareholder) &gt;= num_shares, [../../move-stdlib/doc/error.md#0x1_error_invalid_argument](error::invalid_argument)([pool_u64.md#0x1_pool_u64_EINSUFFICIENT_SHARES](EINSUFFICIENT_SHARES)));

    <b>let</b> existing_shares = [simple_map.md#0x1_simple_map_borrow_mut](simple_map::borrow_mut)(&<b>mut</b> pool.shares, &shareholder);
    *existing_shares = *existing_shares - num_shares;

    // Remove the shareholder completely <b>if</b> they have no shares left.
    <b>let</b> remaining_shares = *existing_shares;
    <b>if</b> (remaining_shares == 0) {
        <b>let</b> (_, shareholder_index) = [../../move-stdlib/doc/vector.md#0x1_vector_index_of](vector::index_of)(&pool.shareholders, &shareholder);
        [../../move-stdlib/doc/vector.md#0x1_vector_remove](vector::remove)(&<b>mut</b> pool.shareholders, shareholder_index);
        [simple_map.md#0x1_simple_map_remove](simple_map::remove)(&<b>mut</b> pool.shares, &shareholder);
    };

    remaining_shares
}
</code></pre>



</details>

<a id="0x1_pool_u64_amount_to_shares"></a>

## Function `amount_to_shares`

Return the number of new shares <code>coins_amount</code> can buy in <code>pool</code>.
<code>amount</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_amount_to_shares](amount_to_shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), coins_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_amount_to_shares](amount_to_shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), coins_amount: u64): u64 {
    [pool_u64.md#0x1_pool_u64_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool, coins_amount, pool.total_coins)
}
</code></pre>



</details>

<a id="0x1_pool_u64_amount_to_shares_with_total_coins"></a>

## Function `amount_to_shares_with_total_coins`

Return the number of new shares <code>coins_amount</code> can buy in <code>pool</code> with a custom total coins number.
<code>amount</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), coins_amount: u64, total_coins: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), coins_amount: u64, total_coins: u64): u64 {
    // No shares yet so amount is worth the same number of shares.
    <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
        // Multiply by scaling factor <b>to</b> minimize rounding errors during <b>internal</b> calculations for buy ins/redeems.
        // This can overflow but scaling factor is expected <b>to</b> be chosen carefully so this would not overflow.
        coins_amount * pool.scaling_factor
    } <b>else</b> {
        // Shares price = total_coins / total existing shares.
        // New number of shares = new_amount / shares_price = new_amount * existing_shares / total_amount.
        // We rearrange the calc and do multiplication first <b>to</b> avoid rounding errors.
        [pool_u64.md#0x1_pool_u64_multiply_then_divide](multiply_then_divide)(pool, coins_amount, pool.total_shares, total_coins)
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_shares_to_amount"></a>

## Function `shares_to_amount`

Return the number of coins <code>shares</code> are worth in <code>pool</code>.
<code>shares</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares_to_amount](shares_to_amount)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shares: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares_to_amount](shares_to_amount)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), shares: u64): u64 {
    [pool_u64.md#0x1_pool_u64_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool, shares, pool.total_coins)
}
</code></pre>



</details>

<a id="0x1_pool_u64_shares_to_amount_with_total_coins"></a>

## Function `shares_to_amount_with_total_coins`

Return the number of coins <code>shares</code> are worth in <code>pool</code> with a custom total coins number.
<code>shares</code> needs to big enough to avoid rounding number.


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shares: u64, total_coins: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), shares: u64, total_coins: u64): u64 {
    // No shares or coins yet so shares are worthless.
    <b>if</b> (pool.total_coins == 0 || pool.total_shares == 0) {
        0
    } <b>else</b> {
        // Shares price = total_coins / total existing shares.
        // Shares worth = shares * shares price = shares * total_coins / total existing shares.
        // We rearrange the calc and do multiplication first <b>to</b> avoid rounding errors.
        [pool_u64.md#0x1_pool_u64_multiply_then_divide](multiply_then_divide)(pool, shares, total_coins, pool.total_shares)
    }
}
</code></pre>



</details>

<a id="0x1_pool_u64_multiply_then_divide"></a>

## Function `multiply_then_divide`



<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), x: u64, y: u64, z: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64.md#0x1_pool_u64_Pool](Pool), x: u64, y: u64, z: u64): u64 {
    <b>let</b> result = ([pool_u64.md#0x1_pool_u64_to_u128](to_u128)(x) * [pool_u64.md#0x1_pool_u64_to_u128](to_u128)(y)) / [pool_u64.md#0x1_pool_u64_to_u128](to_u128)(z);
    (result <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_pool_u64_to_u128"></a>

## Function `to_u128`



<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_to_u128](to_u128)(num: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_to_u128](to_u128)(num: u64): u128 {
    (num <b>as</b> u128)
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_1_Pool"></a>

### Struct `Pool`


<pre><code><b>struct</b> [pool_u64.md#0x1_pool_u64_Pool](Pool) <b>has</b> store
</code></pre>



<dl>
<dt>
<code>shareholders_limit: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_coins: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_shares: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>shares: [simple_map.md#0x1_simple_map_SimpleMap](simple_map::SimpleMap)&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>shareholders: [../../move-stdlib/doc/vector.md#0x1_vector](vector)&lt;<b>address</b>&gt;</code>
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
    ([simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(shares, addr) == [../../move-stdlib/doc/vector.md#0x1_vector_spec_contains](vector::spec_contains)(shareholders, addr));
<b>invariant</b> <b>forall</b> i in 0..len(shareholders), j in 0..len(shareholders):
    shareholders[i] == shareholders[j] ==&gt; i == j;
</code></pre>




<a id="0x1_pool_u64_spec_contains"></a>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_spec_contains](spec_contains)(pool: [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>): bool {
   [simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder)
}
</code></pre>



<a id="@Specification_1_contains"></a>

### Function `contains`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_contains](contains)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): bool
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [pool_u64.md#0x1_pool_u64_spec_contains](spec_contains)(pool, shareholder);
</code></pre>




<a id="0x1_pool_u64_spec_shares"></a>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool: [pool_u64.md#0x1_pool_u64_Pool](Pool), shareholder: <b>address</b>): u64 {
   <b>if</b> ([simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder)) {
       [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder)
   }
   <b>else</b> {
       0
   }
}
</code></pre>



<a id="@Specification_1_shares"></a>

### Function `shares`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares](shares)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): u64
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>ensures</b> result == [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool, shareholder);
</code></pre>



<a id="@Specification_1_balance"></a>

### Function `balance`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_balance](balance)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>): u64
</code></pre>




<pre><code><b>let</b> shares = [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool, shareholder);
<b>let</b> total_coins = pool.total_coins;
<b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0 && (shares * total_coins) / pool.total_shares &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>ensures</b> result == [pool_u64.md#0x1_pool_u64_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares, total_coins);
</code></pre>



<a id="@Specification_1_buy_in"></a>

### Function `buy_in`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_buy_in](buy_in)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, coins_amount: u64): u64
</code></pre>




<pre><code><b>let</b> new_shares = [pool_u64.md#0x1_pool_u64_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool, coins_amount, pool.total_coins);
<b>aborts_if</b> pool.total_coins + coins_amount &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>aborts_if</b> pool.total_shares + new_shares &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>include</b> coins_amount &gt; 0 ==&gt; [pool_u64.md#0x1_pool_u64_AddSharesAbortsIf](AddSharesAbortsIf) { new_shares: new_shares };
<b>include</b> coins_amount &gt; 0 ==&gt; [pool_u64.md#0x1_pool_u64_AddSharesEnsures](AddSharesEnsures) { new_shares: new_shares };
<b>ensures</b> pool.total_coins == <b>old</b>(pool.total_coins) + coins_amount;
<b>ensures</b> pool.total_shares == <b>old</b>(pool.total_shares) + new_shares;
<b>ensures</b> result == new_shares;
</code></pre>



<a id="@Specification_1_add_shares"></a>

### Function `add_shares`


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_add_shares](add_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, new_shares: u64): u64
</code></pre>




<pre><code><b>include</b> [pool_u64.md#0x1_pool_u64_AddSharesAbortsIf](AddSharesAbortsIf);
<b>include</b> [pool_u64.md#0x1_pool_u64_AddSharesEnsures](AddSharesEnsures);
<b>let</b> key_exists = [simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder);
<b>ensures</b> result == <b>if</b> (key_exists) { [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder) }
<b>else</b> { new_shares };
</code></pre>




<a id="0x1_pool_u64_AddSharesAbortsIf"></a>


<pre><code><b>schema</b> [pool_u64.md#0x1_pool_u64_AddSharesAbortsIf](AddSharesAbortsIf) {
    pool: [pool_u64.md#0x1_pool_u64_Pool](Pool);
    shareholder: <b>address</b>;
    new_shares: u64;
    <b>let</b> key_exists = [simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder);
    <b>let</b> current_shares = [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder);
    <b>aborts_if</b> key_exists && current_shares + new_shares &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
    <b>aborts_if</b> !key_exists && new_shares &gt; 0 && len(pool.shareholders) &gt;= pool.shareholders_limit;
}
</code></pre>




<a id="0x1_pool_u64_AddSharesEnsures"></a>


<pre><code><b>schema</b> [pool_u64.md#0x1_pool_u64_AddSharesEnsures](AddSharesEnsures) {
    pool: [pool_u64.md#0x1_pool_u64_Pool](Pool);
    shareholder: <b>address</b>;
    new_shares: u64;
    <b>let</b> key_exists = [simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder);
    <b>let</b> current_shares = [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder);
    <b>ensures</b> key_exists ==&gt;
        pool.shares == [simple_map.md#0x1_simple_map_spec_set](simple_map::spec_set)(<b>old</b>(pool.shares), shareholder, current_shares + new_shares);
    <b>ensures</b> (!key_exists && new_shares &gt; 0) ==&gt;
        pool.shares == [simple_map.md#0x1_simple_map_spec_set](simple_map::spec_set)(<b>old</b>(pool.shares), shareholder, new_shares);
    <b>ensures</b> (!key_exists && new_shares &gt; 0) ==&gt;
        [../../move-stdlib/doc/vector.md#0x1_vector_eq_push_back](vector::eq_push_back)(pool.shareholders, <b>old</b>(pool.shareholders), shareholder);
}
</code></pre>




<a id="0x1_pool_u64_spec_amount_to_shares_with_total_coins"></a>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool: [pool_u64.md#0x1_pool_u64_Pool](Pool), coins_amount: u64, total_coins: u64): u64 {
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


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_redeem_shares](redeem_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, shares_to_redeem: u64): u64
</code></pre>




<pre><code><b>let</b> redeemed_coins = [pool_u64.md#0x1_pool_u64_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares_to_redeem, pool.total_coins);
<b>aborts_if</b> ![pool_u64.md#0x1_pool_u64_spec_contains](spec_contains)(pool, shareholder);
<b>aborts_if</b> [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool, shareholder) &lt; shares_to_redeem;
<b>aborts_if</b> pool.[pool_u64.md#0x1_pool_u64_total_coins](total_coins) &lt; redeemed_coins;
<b>aborts_if</b> pool.[pool_u64.md#0x1_pool_u64_total_shares](total_shares) &lt; shares_to_redeem;
<b>ensures</b> pool.total_coins == <b>old</b>(pool.total_coins) - redeemed_coins;
<b>ensures</b> pool.total_shares == <b>old</b>(pool.total_shares) - shares_to_redeem;
<b>include</b> shares_to_redeem &gt; 0 ==&gt; [pool_u64.md#0x1_pool_u64_DeductSharesEnsures](DeductSharesEnsures) { num_shares: shares_to_redeem };
<b>ensures</b> result == redeemed_coins;
</code></pre>



<a id="@Specification_1_transfer_shares"></a>

### Function `transfer_shares`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_transfer_shares](transfer_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder_1: <b>address</b>, shareholder_2: <b>address</b>, shares_to_transfer: u64)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>aborts_if</b> ![pool_u64.md#0x1_pool_u64_spec_contains](spec_contains)(pool, shareholder_1);
<b>aborts_if</b> [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool, shareholder_1) &lt; shares_to_transfer;
</code></pre>



<a id="@Specification_1_deduct_shares"></a>

### Function `deduct_shares`


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_deduct_shares](deduct_shares)(pool: &<b>mut</b> [pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shareholder: <b>address</b>, num_shares: u64): u64
</code></pre>




<pre><code><b>aborts_if</b> ![pool_u64.md#0x1_pool_u64_spec_contains](spec_contains)(pool, shareholder);
<b>aborts_if</b> [pool_u64.md#0x1_pool_u64_spec_shares](spec_shares)(pool, shareholder) &lt; num_shares;
<b>include</b> [pool_u64.md#0x1_pool_u64_DeductSharesEnsures](DeductSharesEnsures);
<b>let</b> remaining_shares = [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder) - num_shares;
<b>ensures</b> remaining_shares &gt; 0 ==&gt; result == [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder);
<b>ensures</b> remaining_shares == 0 ==&gt; result == 0;
</code></pre>




<a id="0x1_pool_u64_DeductSharesEnsures"></a>


<pre><code><b>schema</b> [pool_u64.md#0x1_pool_u64_DeductSharesEnsures](DeductSharesEnsures) {
    pool: [pool_u64.md#0x1_pool_u64_Pool](Pool);
    shareholder: <b>address</b>;
    num_shares: u64;
    <b>let</b> remaining_shares = [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder) - num_shares;
    <b>ensures</b> remaining_shares &gt; 0 ==&gt; [simple_map.md#0x1_simple_map_spec_get](simple_map::spec_get)(pool.shares, shareholder) == remaining_shares;
    <b>ensures</b> remaining_shares == 0 ==&gt; ![simple_map.md#0x1_simple_map_spec_contains_key](simple_map::spec_contains_key)(pool.shares, shareholder);
    <b>ensures</b> remaining_shares == 0 ==&gt; ![../../move-stdlib/doc/vector.md#0x1_vector_spec_contains](vector::spec_contains)(pool.shareholders, shareholder);
}
</code></pre>



<a id="@Specification_1_amount_to_shares_with_total_coins"></a>

### Function `amount_to_shares_with_total_coins`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_amount_to_shares_with_total_coins](amount_to_shares_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), coins_amount: u64, total_coins: u64): u64
</code></pre>




<pre><code><b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0
    && (coins_amount * pool.total_shares) / total_coins &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>aborts_if</b> (pool.total_coins == 0 || pool.total_shares == 0)
    && coins_amount * pool.scaling_factor &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0 && total_coins == 0;
<b>ensures</b> result == [pool_u64.md#0x1_pool_u64_spec_amount_to_shares_with_total_coins](spec_amount_to_shares_with_total_coins)(pool, coins_amount, total_coins);
</code></pre>



<a id="@Specification_1_shares_to_amount_with_total_coins"></a>

### Function `shares_to_amount_with_total_coins`


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_shares_to_amount_with_total_coins](shares_to_amount_with_total_coins)(pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), shares: u64, total_coins: u64): u64
</code></pre>




<pre><code><b>aborts_if</b> pool.total_coins &gt; 0 && pool.total_shares &gt; 0
    && (shares * total_coins) / pool.total_shares &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>ensures</b> result == [pool_u64.md#0x1_pool_u64_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool, shares, total_coins);
</code></pre>




<a id="0x1_pool_u64_spec_shares_to_amount_with_total_coins"></a>


<pre><code><b>fun</b> [pool_u64.md#0x1_pool_u64_spec_shares_to_amount_with_total_coins](spec_shares_to_amount_with_total_coins)(pool: [pool_u64.md#0x1_pool_u64_Pool](Pool), shares: u64, total_coins: u64): u64 {
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


<pre><code><b>public</b> <b>fun</b> [pool_u64.md#0x1_pool_u64_multiply_then_divide](multiply_then_divide)(_pool: &[pool_u64.md#0x1_pool_u64_Pool](pool_u64::Pool), x: u64, y: u64, z: u64): u64
</code></pre>




<pre><code><b>aborts_if</b> z == 0;
<b>aborts_if</b> (x * y) / z &gt; [pool_u64.md#0x1_pool_u64_MAX_U64](MAX_U64);
<b>ensures</b> result == (x * y) / z;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
