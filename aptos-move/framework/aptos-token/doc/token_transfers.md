
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


<pre><code><b>use</b> [../../aptos-framework/doc/account.md#0x1_account](0x1::account);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error](0x1::error);
<b>use</b> [../../aptos-framework/doc/event.md#0x1_event](0x1::event);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/features.md#0x1_features](0x1::features);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](0x1::signer);
<b>use</b> [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string](0x1::string);
<b>use</b> [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table](0x1::table);
<b>use</b> [token.md#0x3_token](0x3::token);
</code></pre>



<a id="0x3_token_transfers_PendingClaims"></a>

## Resource `PendingClaims`



<pre><code><b>struct</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pending_claims: [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_Table](table::Table)&lt;[token_transfers.md#0x3_token_transfers_TokenOfferId](token_transfers::TokenOfferId), [token.md#0x3_token_Token](token::Token)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>offer_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_transfers.md#0x3_token_transfers_TokenOfferEvent](token_transfers::TokenOfferEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>cancel_offer_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_transfers.md#0x3_token_transfers_TokenCancelOfferEvent](token_transfers::TokenCancelOfferEvent)&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>claim_events: [../../aptos-framework/doc/event.md#0x1_event_EventHandle](event::EventHandle)&lt;[token_transfers.md#0x3_token_transfers_TokenClaimEvent](token_transfers::TokenClaimEvent)&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenOfferId"></a>

## Struct `TokenOfferId`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenOfferId](TokenOfferId) <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenOffer"></a>

## Struct `TokenOffer`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenOffer](TokenOffer) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenOfferEvent"></a>

## Struct `TokenOfferEvent`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenOfferEvent](TokenOfferEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenCancelOfferEvent"></a>

## Struct `TokenCancelOfferEvent`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenCancelOfferEvent](TokenCancelOfferEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenCancelOffer"></a>

## Struct `TokenCancelOffer`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenCancelOffer](TokenCancelOffer) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenClaimEvent"></a>

## Struct `TokenClaimEvent`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenClaimEvent](TokenClaimEvent) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x3_token_transfers_TokenClaim"></a>

## Struct `TokenClaim`



<pre><code>#[[../../aptos-framework/doc/event.md#0x1_event](event)]
<b>struct</b> [token_transfers.md#0x3_token_transfers_TokenClaim](TokenClaim) <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>to_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: [token.md#0x3_token_TokenId](token::TokenId)</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST"></a>

Token offer doesn't exist


<pre><code><b>const</b> [token_transfers.md#0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST](ETOKEN_OFFER_NOT_EXIST): u64 = 1;
</code></pre>



<a id="0x3_token_transfers_initialize_token_transfers"></a>

## Function `initialize_token_transfers`



<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_initialize_token_transfers](initialize_token_transfers)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_initialize_token_transfers](initialize_token_transfers)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer)) {
    <b>move_to</b>(
        [../../aptos-framework/doc/account.md#0x1_account](account),
        [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
            pending_claims: [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_new](table::new)&lt;[token_transfers.md#0x3_token_transfers_TokenOfferId](TokenOfferId), Token&gt;(),
            offer_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_transfers.md#0x3_token_transfers_TokenOfferEvent](TokenOfferEvent)&gt;([../../aptos-framework/doc/account.md#0x1_account](account)),
            cancel_offer_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_transfers.md#0x3_token_transfers_TokenCancelOfferEvent](TokenCancelOfferEvent)&gt;([../../aptos-framework/doc/account.md#0x1_account](account)),
            claim_events: [../../aptos-framework/doc/account.md#0x1_account_new_event_handle](account::new_event_handle)&lt;[token_transfers.md#0x3_token_transfers_TokenClaimEvent](TokenClaimEvent)&gt;([../../aptos-framework/doc/account.md#0x1_account](account)),
        }
    )
}
</code></pre>



</details>

<a id="0x3_token_transfers_create_token_offer_id"></a>

## Function `create_token_offer_id`



<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(to_addr: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId)): [token_transfers.md#0x3_token_transfers_TokenOfferId](token_transfers::TokenOfferId)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(to_addr: <b>address</b>, token_id: TokenId): [token_transfers.md#0x3_token_transfers_TokenOfferId](TokenOfferId) {
    [token_transfers.md#0x3_token_transfers_TokenOfferId](TokenOfferId) {
        to_addr,
        token_id
    }
}
</code></pre>



</details>

<a id="0x3_token_transfers_offer_script"></a>

## Function `offer_script`



<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_offer_script](offer_script)(sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_offer_script](offer_script)(
    sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    receiver: <b>address</b>,
    creator: <b>address</b>,
    collection: String,
    name: String,
    property_version: u64,
    amount: u64,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
    [token_transfers.md#0x3_token_transfers_offer](offer)(&sender, receiver, token_id, amount);
}
</code></pre>



</details>

<a id="0x3_token_transfers_offer"></a>

## Function `offer`



<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_offer](offer)(sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId), amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_offer](offer)(
    sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    receiver: <b>address</b>,
    token_id: TokenId,
    amount: u64,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>let</b> sender_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender);
    <b>if</b> (!<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr)) {
        [token_transfers.md#0x3_token_transfers_initialize_token_transfers](initialize_token_transfers)(sender)
    };

    <b>let</b> pending_claims =
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).pending_claims;
    <b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(receiver, token_id);
    <b>let</b> [token.md#0x3_token](token) = [token.md#0x3_token_withdraw_token](token::withdraw_token)(sender, token_id, amount);
    <b>if</b> (![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_contains](table::contains)(pending_claims, token_offer_id)) {
        [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_add](table::add)(pending_claims, token_offer_id, [token.md#0x3_token](token));
    } <b>else</b> {
        <b>let</b> dst_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_borrow_mut](table::borrow_mut)(pending_claims, token_offer_id);
        [token.md#0x3_token_merge](token::merge)(dst_token, [token.md#0x3_token](token));
    };

    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_transfers.md#0x3_token_transfers_TokenOffer](TokenOffer) {
                to_address: receiver,
                token_id,
                amount,
            }
        )
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_transfers.md#0x3_token_transfers_TokenOfferEvent](TokenOfferEvent)&gt;(
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).offer_events,
        [token_transfers.md#0x3_token_transfers_TokenOfferEvent](TokenOfferEvent) {
            to_address: receiver,
            token_id,
            amount,
        },
    );
}
</code></pre>



</details>

<a id="0x3_token_transfers_claim_script"></a>

## Function `claim_script`



<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_claim_script](claim_script)(receiver: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), sender: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_claim_script](claim_script)(
    receiver: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    sender: <b>address</b>,
    creator: <b>address</b>,
    collection: String,
    name: String,
    property_version: u64,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
    [token_transfers.md#0x3_token_transfers_claim](claim)(&receiver, sender, token_id);
}
</code></pre>



</details>

<a id="0x3_token_transfers_claim"></a>

## Function `claim`



<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_claim](claim)(receiver: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), sender: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_claim](claim)(
    receiver: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    sender: <b>address</b>,
    token_id: TokenId,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>assert</b>!(<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender), [token_transfers.md#0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST](ETOKEN_OFFER_NOT_EXIST));
    <b>let</b> pending_claims =
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender).pending_claims;
    <b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver), token_id);
    <b>assert</b>!([../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_contains](table::contains)(pending_claims, token_offer_id), [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found](error::not_found)([token_transfers.md#0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST](ETOKEN_OFFER_NOT_EXIST)));
    <b>let</b> tokens = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_remove](table::remove)(pending_claims, token_offer_id);
    <b>let</b> amount = [token.md#0x3_token_get_token_amount](token::get_token_amount)(&tokens);
    [token.md#0x3_token_deposit_token](token::deposit_token)(receiver, tokens);

    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_transfers.md#0x3_token_transfers_TokenClaim](TokenClaim) {
                to_address: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver),
                token_id,
                amount,
            }
        )
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_transfers.md#0x3_token_transfers_TokenClaimEvent](TokenClaimEvent)&gt;(
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender).claim_events,
        [token_transfers.md#0x3_token_transfers_TokenClaimEvent](TokenClaimEvent) {
            to_address: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver),
            token_id,
            amount,
        },
    );
}
</code></pre>



</details>

<a id="0x3_token_transfers_cancel_offer_script"></a>

## Function `cancel_offer_script`



<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer_script](cancel_offer_script)(sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer_script](cancel_offer_script)(
    sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    receiver: <b>address</b>,
    creator: <b>address</b>,
    collection: String,
    name: String,
    property_version: u64,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
    [token_transfers.md#0x3_token_transfers_cancel_offer](cancel_offer)(&sender, receiver, token_id);
}
</code></pre>



</details>

<a id="0x3_token_transfers_cancel_offer"></a>

## Function `cancel_offer`



<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer](cancel_offer)(sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId))
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer](cancel_offer)(
    sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer),
    receiver: <b>address</b>,
    token_id: TokenId,
) <b>acquires</b> [token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims) {
    <b>let</b> sender_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender);
    <b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(receiver, token_id);
    <b>assert</b>!(<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr), [token_transfers.md#0x3_token_transfers_ETOKEN_OFFER_NOT_EXIST](ETOKEN_OFFER_NOT_EXIST));
    <b>let</b> pending_claims =
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).pending_claims;
    <b>let</b> [token.md#0x3_token](token) = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_remove](table::remove)(pending_claims, token_offer_id);
    <b>let</b> amount = [token.md#0x3_token_get_token_amount](token::get_token_amount)(&[token.md#0x3_token](token));
    [token.md#0x3_token_deposit_token](token::deposit_token)(sender, [token.md#0x3_token](token));

    <b>if</b> (std::features::module_event_migration_enabled()) {
        [../../aptos-framework/doc/event.md#0x1_event_emit](event::emit)(
            [token_transfers.md#0x3_token_transfers_TokenCancelOffer](TokenCancelOffer) {
                to_address: receiver,
                token_id,
                amount,
            },
        )
    };
    [../../aptos-framework/doc/event.md#0x1_event_emit_event](event::emit_event)&lt;[token_transfers.md#0x3_token_transfers_TokenCancelOfferEvent](TokenCancelOfferEvent)&gt;(
        &<b>mut</b> <b>borrow_global_mut</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).cancel_offer_events,
        [token_transfers.md#0x3_token_transfers_TokenCancelOfferEvent](TokenCancelOfferEvent) {
            to_address: receiver,
            token_id,
            amount,
        },
    );
}
</code></pre>



</details>

<a id="@Specification_1"></a>

## Specification



<pre><code><b>pragma</b> verify = <b>true</b>;
<b>pragma</b> aborts_if_is_strict;
</code></pre>



<a id="@Specification_1_initialize_token_transfers"></a>

### Function `initialize_token_transfers`


<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_initialize_token_transfers](initialize_token_transfers)([../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer))
</code></pre>




<pre><code><b>include</b> [token_transfers.md#0x3_token_transfers_InitializeTokenTransfersAbortsIf](InitializeTokenTransfersAbortsIf);
</code></pre>


Abort according to the code


<a id="0x3_token_transfers_InitializeTokenTransfersAbortsIf"></a>


<pre><code><b>schema</b> [token_transfers.md#0x3_token_transfers_InitializeTokenTransfersAbortsIf](InitializeTokenTransfersAbortsIf) {
    [../../aptos-framework/doc/account.md#0x1_account](account): &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer);
    <b>let</b> addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)([../../aptos-framework/doc/account.md#0x1_account](account));
    <b>aborts_if</b> <b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(addr);
    <b>let</b> [../../aptos-framework/doc/account.md#0x1_account](account) = <b>global</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> !<b>exists</b>&lt;Account&gt;(addr);
    <b>aborts_if</b> [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 3 &gt;= [../../aptos-framework/doc/account.md#0x1_account_MAX_GUID_CREATION_NUM](account::MAX_GUID_CREATION_NUM);
    <b>aborts_if</b> [../../aptos-framework/doc/account.md#0x1_account](account).guid_creation_num + 3 &gt; MAX_U64;
}
</code></pre>



<a id="@Specification_1_create_token_offer_id"></a>

### Function `create_token_offer_id`


<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(to_addr: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId)): [token_transfers.md#0x3_token_transfers_TokenOfferId](token_transfers::TokenOfferId)
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



<a id="@Specification_1_offer_script"></a>

### Function `offer_script`


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_offer_script](offer_script)(sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64, amount: u64)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
<b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
</code></pre>



<a id="@Specification_1_offer"></a>

### Function `offer`


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_offer](offer)(sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId), amount: u64)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
<b>let</b> sender_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender);
<b>include</b> !<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr) ==&gt; [token_transfers.md#0x3_token_transfers_InitializeTokenTransfersAbortsIf](InitializeTokenTransfersAbortsIf){[../../aptos-framework/doc/account.md#0x1_account](account) : sender};
<b>let</b> pending_claims = <b>global</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).pending_claims;
<b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(receiver, token_id);
<b>let</b> tokens = <b>global</b>&lt;TokenStore&gt;(sender_addr).tokens;
<b>aborts_if</b> amount &lt;= 0;
<b>aborts_if</b> [token.md#0x3_token_spec_balance_of](token::spec_balance_of)(sender_addr, token_id) &lt; amount;
<b>aborts_if</b> !<b>exists</b>&lt;TokenStore&gt;(sender_addr);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(tokens, token_id);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>let</b> a = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>let</b> dst_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(pending_claims, token_offer_id);
<b>aborts_if</b> dst_token.amount + [token_transfers.md#0x3_token_transfers_spce_get](spce_get)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender), token_id, amount) &gt; MAX_U64;
</code></pre>


Get the amount from sender token


<a id="0x3_token_transfers_spce_get"></a>


<pre><code><b>fun</b> [token_transfers.md#0x3_token_transfers_spce_get](spce_get)(
   account_addr: <b>address</b>,
   id: TokenId,
   amount: u64
): u64 {
   <b>use</b> aptos_token::token::{TokenStore};
   <b>use</b> aptos_std::table::{<b>Self</b>};
   <b>let</b> tokens = <b>global</b>&lt;TokenStore&gt;(account_addr).tokens;
   <b>let</b> balance = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(tokens, id).amount;
   <b>if</b> (balance &gt; amount) {
       amount
   } <b>else</b> {
       [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(tokens, id).amount
   }
}
</code></pre>



<a id="@Specification_1_claim_script"></a>

### Function `claim_script`


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_claim_script](claim_script)(receiver: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), sender: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
<b>aborts_if</b> !<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender);
<b>let</b> pending_claims = <b>global</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender).pending_claims;
<b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver), token_id);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>let</b> tokens = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(pending_claims, token_offer_id);
<b>include</b> [token.md#0x3_token_InitializeTokenStore](token::InitializeTokenStore){[../../aptos-framework/doc/account.md#0x1_account](account): receiver };
<b>let</b> account_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver);
<b>let</b> [token.md#0x3_token](token) = tokens;
<b>let</b> token_store = <b>global</b>&lt;TokenStore&gt;(account_addr);
<b>let</b> recipient_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(token_store.tokens, [token.md#0x3_token](token).id);
<b>let</b> b = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(token_store.tokens, [token.md#0x3_token](token).id);
<b>aborts_if</b> [token.md#0x3_token](token).amount &lt;= 0;
</code></pre>



<a id="@Specification_1_claim"></a>

### Function `claim`


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_claim](claim)(receiver: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), sender: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId))
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>aborts_if</b> !<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender);
<b>let</b> pending_claims = <b>global</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender).pending_claims;
<b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)([../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver), token_id);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>let</b> tokens = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(pending_claims, token_offer_id);
<b>include</b> [token.md#0x3_token_InitializeTokenStore](token::InitializeTokenStore){[../../aptos-framework/doc/account.md#0x1_account](account): receiver };
<b>let</b> account_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(receiver);
<b>let</b> [token.md#0x3_token](token) = tokens;
<b>let</b> token_store = <b>global</b>&lt;TokenStore&gt;(account_addr);
<b>let</b> recipient_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(token_store.tokens, [token.md#0x3_token](token).id);
<b>let</b> b = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(token_store.tokens, [token.md#0x3_token](token).id);
<b>aborts_if</b> [token.md#0x3_token](token).amount &lt;= 0;
</code></pre>



<a id="@Specification_1_cancel_offer_script"></a>

### Function `cancel_offer_script`


<pre><code><b>public</b> entry <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer_script](cancel_offer_script)(sender: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, creator: <b>address</b>, collection: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), name: [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/string.md#0x1_string_String](string::String), property_version: u64)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> token_id = [token.md#0x3_token_create_token_id_raw](token::create_token_id_raw)(creator, collection, name, property_version);
<b>let</b> sender_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender);
<b>aborts_if</b> !<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr);
<b>let</b> pending_claims = <b>global</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).pending_claims;
<b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(receiver, token_id);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>include</b> [token.md#0x3_token_InitializeTokenStore](token::InitializeTokenStore){[../../aptos-framework/doc/account.md#0x1_account](account): sender };
<b>let</b> dst_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(pending_claims, token_offer_id);
<b>let</b> account_addr = sender_addr;
<b>let</b> [token.md#0x3_token](token) = dst_token;
<b>let</b> token_store = <b>global</b>&lt;TokenStore&gt;(account_addr);
<b>let</b> recipient_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(token_store.tokens, [token.md#0x3_token](token).id);
<b>let</b> b = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(token_store.tokens, [token.md#0x3_token](token).id);
<b>aborts_if</b> [token.md#0x3_token](token).amount &lt;= 0;
</code></pre>



<a id="@Specification_1_cancel_offer"></a>

### Function `cancel_offer`


<pre><code><b>public</b> <b>fun</b> [token_transfers.md#0x3_token_transfers_cancel_offer](cancel_offer)(sender: &[../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer](signer), receiver: <b>address</b>, token_id: [token.md#0x3_token_TokenId](token::TokenId))
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>let</b> sender_addr = [../../aptos-framework/../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of](signer::address_of)(sender);
<b>aborts_if</b> !<b>exists</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr);
<b>let</b> pending_claims = <b>global</b>&lt;[token_transfers.md#0x3_token_transfers_PendingClaims](PendingClaims)&gt;(sender_addr).pending_claims;
<b>let</b> token_offer_id = [token_transfers.md#0x3_token_transfers_create_token_offer_id](create_token_offer_id)(receiver, token_id);
<b>aborts_if</b> ![../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(pending_claims, token_offer_id);
<b>include</b> [token.md#0x3_token_InitializeTokenStore](token::InitializeTokenStore){[../../aptos-framework/doc/account.md#0x1_account](account): sender };
<b>let</b> dst_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(pending_claims, token_offer_id);
<b>let</b> account_addr = sender_addr;
<b>let</b> [token.md#0x3_token](token) = dst_token;
<b>let</b> token_store = <b>global</b>&lt;TokenStore&gt;(account_addr);
<b>let</b> recipient_token = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_get](table::spec_get)(token_store.tokens, [token.md#0x3_token](token).id);
<b>let</b> b = [../../aptos-framework/../aptos-stdlib/doc/table.md#0x1_table_spec_contains](table::spec_contains)(token_store.tokens, [token.md#0x3_token](token).id);
<b>aborts_if</b> [token.md#0x3_token](token).amount &lt;= 0;
</code></pre>


[move-book]: https://aptos.dev/move/book/SUMMARY
