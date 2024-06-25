
<a id="0x1_transaction_fee"></a>

# Module `0x1::transaction_fee`

This module provides an interface to burn or collect and redistribute transaction fees.


-  [Resource `AptosCoinCapabilities`](#0x1_transaction_fee_AptosCoinCapabilities)
-  [Resource `AptosFABurnCapabilities`](#0x1_transaction_fee_AptosFABurnCapabilities)
-  [Resource `AptosCoinMintCapability`](#0x1_transaction_fee_AptosCoinMintCapability)
-  [Resource `CollectedFeesPerBlock`](#0x1_transaction_fee_CollectedFeesPerBlock)
-  [Struct `FeeStatement`](#0x1_transaction_fee_FeeStatement)
-  [Constants](#@Constants_0)
-  [Function `initialize_fee_collection_and_distribution`](#0x1_transaction_fee_initialize_fee_collection_and_distribution)
-  [Function `is_fees_collection_enabled`](#0x1_transaction_fee_is_fees_collection_enabled)
-  [Function `upgrade_burn_percentage`](#0x1_transaction_fee_upgrade_burn_percentage)
-  [Function `register_proposer_for_fee_collection`](#0x1_transaction_fee_register_proposer_for_fee_collection)
-  [Function `burn_coin_fraction`](#0x1_transaction_fee_burn_coin_fraction)
-  [Function `process_collected_fees`](#0x1_transaction_fee_process_collected_fees)
-  [Function `burn_fee`](#0x1_transaction_fee_burn_fee)
-  [Function `mint_and_refund`](#0x1_transaction_fee_mint_and_refund)
-  [Function `collect_fee`](#0x1_transaction_fee_collect_fee)
-  [Function `store_aptos_coin_burn_cap`](#0x1_transaction_fee_store_aptos_coin_burn_cap)
-  [Function `convert_to_aptos_fa_burn_ref`](#0x1_transaction_fee_convert_to_aptos_fa_burn_ref)
-  [Function `store_aptos_coin_mint_cap`](#0x1_transaction_fee_store_aptos_coin_mint_cap)
-  [Function `initialize_storage_refund`](#0x1_transaction_fee_initialize_storage_refund)
-  [Function `emit_fee_statement`](#0x1_transaction_fee_emit_fee_statement)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Resource `CollectedFeesPerBlock`](#@Specification_1_CollectedFeesPerBlock)
    -  [Function `initialize_fee_collection_and_distribution`](#@Specification_1_initialize_fee_collection_and_distribution)
    -  [Function `upgrade_burn_percentage`](#@Specification_1_upgrade_burn_percentage)
    -  [Function `register_proposer_for_fee_collection`](#@Specification_1_register_proposer_for_fee_collection)
    -  [Function `burn_coin_fraction`](#@Specification_1_burn_coin_fraction)
    -  [Function `process_collected_fees`](#@Specification_1_process_collected_fees)
    -  [Function `burn_fee`](#@Specification_1_burn_fee)
    -  [Function `mint_and_refund`](#@Specification_1_mint_and_refund)
    -  [Function `collect_fee`](#@Specification_1_collect_fee)
    -  [Function `store_aptos_coin_burn_cap`](#@Specification_1_store_aptos_coin_burn_cap)
    -  [Function `store_aptos_coin_mint_cap`](#@Specification_1_store_aptos_coin_mint_cap)
    -  [Function `initialize_storage_refund`](#@Specification_1_initialize_storage_refund)
    -  [Function `emit_fee_statement`](#@Specification_1_emit_fee_statement)


```move
module 0x1::transaction_fee {
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::fungible_asset;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::stake;
    use 0x1::system_addresses;
}
```


<a id="0x1_transaction_fee_AptosCoinCapabilities"></a>

## Resource `AptosCoinCapabilities`

Stores burn capability to burn the gas fees.


```move
module 0x1::transaction_fee {
    struct AptosCoinCapabilities has key
}
```


##### Fields


<dl>
<dt>
`burn_cap: coin::BurnCapability<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_transaction_fee_AptosFABurnCapabilities"></a>

## Resource `AptosFABurnCapabilities`

Stores burn capability to burn the gas fees.


```move
module 0x1::transaction_fee {
    struct AptosFABurnCapabilities has key
}
```


##### Fields


<dl>
<dt>
`burn_ref: fungible_asset::BurnRef`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_transaction_fee_AptosCoinMintCapability"></a>

## Resource `AptosCoinMintCapability`

Stores mint capability to mint the refunds.


```move
module 0x1::transaction_fee {
    struct AptosCoinMintCapability has key
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


<a id="0x1_transaction_fee_CollectedFeesPerBlock"></a>

## Resource `CollectedFeesPerBlock`

Stores information about the block proposer and the amount of fees
collected when executing the block.


```move
module 0x1::transaction_fee {
    struct CollectedFeesPerBlock has key
}
```


##### Fields


<dl>
<dt>
`amount: coin::AggregatableCoin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`proposer: option::Option<address>`
</dt>
<dd>

</dd>
<dt>
`burn_percentage: u8`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_transaction_fee_FeeStatement"></a>

## Struct `FeeStatement`

Breakdown of fee charge and refund for a transaction.
The structure is:

&#45; Net charge or refund (not in the statement)
&#45; total charge: total_charge_gas_units, matches `gas_used` in the on&#45;chain `TransactionInfo`.
This is the sum of the sub&#45;items below. Notice that there&apos;s potential precision loss when
the conversion between internal and external gas units and between native token and gas
units, so it&apos;s possible that the numbers don&apos;t add up exactly. &#45;&#45; This number is the final
charge, while the break down is merely informational.
&#45; gas charge for execution (CPU time): `execution_gas_units`
&#45; gas charge for IO (storage random access): `io_gas_units`
&#45; storage fee charge (storage space): `storage_fee_octas`, to be included in
`total_charge_gas_unit`, this number is converted to gas units according to the user
specified `gas_unit_price` on the transaction.
&#45; storage deletion refund: `storage_fee_refund_octas`, this is not included in `gas_used` or
`total_charge_gas_units`, the net charge / refund is calculated by
`total_charge_gas_units` &#42; `gas_unit_price` &#45; `storage_fee_refund_octas`.

This is meant to emitted as a module event.


```move
module 0x1::transaction_fee {
    #[event]
    struct FeeStatement has drop, store
}
```


##### Fields


<dl>
<dt>
`total_charge_gas_units: u64`
</dt>
<dd>
 Total gas charge.
</dd>
<dt>
`execution_gas_units: u64`
</dt>
<dd>
 Execution gas charge.
</dd>
<dt>
`io_gas_units: u64`
</dt>
<dd>
 IO gas charge.
</dd>
<dt>
`storage_fee_octas: u64`
</dt>
<dd>
 Storage fee charge.
</dd>
<dt>
`storage_fee_refund_octas: u64`
</dt>
<dd>
 Storage fee refund.
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_transaction_fee_EALREADY_COLLECTING_FEES"></a>

Gas fees are already being collected and the struct holding
information about collected amounts is already published.


```move
module 0x1::transaction_fee {
    const EALREADY_COLLECTING_FEES: u64 = 1;
}
```


<a id="0x1_transaction_fee_EFA_GAS_CHARGING_NOT_ENABLED"></a>



```move
module 0x1::transaction_fee {
    const EFA_GAS_CHARGING_NOT_ENABLED: u64 = 5;
}
```


<a id="0x1_transaction_fee_EINVALID_BURN_PERCENTAGE"></a>

The burn percentage is out of range [0, 100].


```move
module 0x1::transaction_fee {
    const EINVALID_BURN_PERCENTAGE: u64 = 3;
}
```


<a id="0x1_transaction_fee_ENO_LONGER_SUPPORTED"></a>

No longer supported.


```move
module 0x1::transaction_fee {
    const ENO_LONGER_SUPPORTED: u64 = 4;
}
```


<a id="0x1_transaction_fee_initialize_fee_collection_and_distribution"></a>

## Function `initialize_fee_collection_and_distribution`

Initializes the resource storing information about gas fees collection and
distribution. Should be called by on&#45;chain governance.


```move
module 0x1::transaction_fee {
    public fun initialize_fee_collection_and_distribution(aptos_framework: &signer, burn_percentage: u8)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public fun initialize_fee_collection_and_distribution(aptos_framework: &signer, burn_percentage: u8) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            !exists<CollectedFeesPerBlock>(@aptos_framework),
            error::already_exists(EALREADY_COLLECTING_FEES)
        );
        assert!(burn_percentage <= 100, error::out_of_range(EINVALID_BURN_PERCENTAGE));

        // Make sure stakng module is aware of transaction fees collection.
        stake::initialize_validator_fees(aptos_framework);

        // Initially, no fees are collected and the block proposer is not set.
        let collected_fees = CollectedFeesPerBlock {
            amount: coin::initialize_aggregatable_coin(aptos_framework),
            proposer: option::none(),
            burn_percentage,
        };
        move_to(aptos_framework, collected_fees);
    }
}
```


<a id="0x1_transaction_fee_is_fees_collection_enabled"></a>

## Function `is_fees_collection_enabled`



```move
module 0x1::transaction_fee {
    fun is_fees_collection_enabled(): bool
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    fun is_fees_collection_enabled(): bool {
        exists<CollectedFeesPerBlock>(@aptos_framework)
    }
}
```


<a id="0x1_transaction_fee_upgrade_burn_percentage"></a>

## Function `upgrade_burn_percentage`

Sets the burn percentage for collected fees to a new value. Should be called by on&#45;chain governance.


```move
module 0x1::transaction_fee {
    public fun upgrade_burn_percentage(aptos_framework: &signer, new_burn_percentage: u8)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public fun upgrade_burn_percentage(
        aptos_framework: &signer,
        new_burn_percentage: u8
    ) acquires AptosCoinCapabilities, CollectedFeesPerBlock {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(new_burn_percentage <= 100, error::out_of_range(EINVALID_BURN_PERCENTAGE));

        // Prior to upgrading the burn percentage, make sure to process collected
        // fees. Otherwise we would use the new (incorrect) burn_percentage when
        // processing fees later!
        process_collected_fees();

        if (is_fees_collection_enabled()) {
            // Upgrade has no effect unless fees are being collected.
            let burn_percentage = &mut borrow_global_mut<CollectedFeesPerBlock>(@aptos_framework).burn_percentage;
            *burn_percentage = new_burn_percentage
        }
    }
}
```


<a id="0x1_transaction_fee_register_proposer_for_fee_collection"></a>

## Function `register_proposer_for_fee_collection`

Registers the proposer of the block for gas fees collection. This function
can only be called at the beginning of the block.


```move
module 0x1::transaction_fee {
    public(friend) fun register_proposer_for_fee_collection(proposer_addr: address)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun register_proposer_for_fee_collection(proposer_addr: address) acquires CollectedFeesPerBlock {
        if (is_fees_collection_enabled()) {
            let collected_fees = borrow_global_mut<CollectedFeesPerBlock>(@aptos_framework);
            let _ = option::swap_or_fill(&mut collected_fees.proposer, proposer_addr);
        }
    }
}
```


<a id="0x1_transaction_fee_burn_coin_fraction"></a>

## Function `burn_coin_fraction`

Burns a specified fraction of the coin.


```move
module 0x1::transaction_fee {
    fun burn_coin_fraction(coin: &mut coin::Coin<aptos_coin::AptosCoin>, burn_percentage: u8)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    fun burn_coin_fraction(coin: &mut Coin<AptosCoin>, burn_percentage: u8) acquires AptosCoinCapabilities {
        assert!(burn_percentage <= 100, error::out_of_range(EINVALID_BURN_PERCENTAGE));

        let collected_amount = coin::value(coin);
        spec {
            // We assume that `burn_percentage * collected_amount` does not overflow.
            assume burn_percentage * collected_amount <= MAX_U64;
        };
        let amount_to_burn = (burn_percentage as u64) * collected_amount / 100;
        if (amount_to_burn > 0) {
            let coin_to_burn = coin::extract(coin, amount_to_burn);
            coin::burn(
                coin_to_burn,
                &borrow_global<AptosCoinCapabilities>(@aptos_framework).burn_cap,
            );
        }
    }
}
```


<a id="0x1_transaction_fee_process_collected_fees"></a>

## Function `process_collected_fees`

Calculates the fee which should be distributed to the block proposer at the
end of an epoch, and records it in the system. This function can only be called
at the beginning of the block or during reconfiguration.


```move
module 0x1::transaction_fee {
    public(friend) fun process_collected_fees()
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun process_collected_fees() acquires AptosCoinCapabilities, CollectedFeesPerBlock {
        if (!is_fees_collection_enabled()) {
            return
        };
        let collected_fees = borrow_global_mut<CollectedFeesPerBlock>(@aptos_framework);

        // If there are no collected fees, only unset the proposer. See the rationale for
        // setting proposer to option::none() below.
        if (coin::is_aggregatable_coin_zero(&collected_fees.amount)) {
            if (option::is_some(&collected_fees.proposer)) {
                let _ = option::extract(&mut collected_fees.proposer);
            };
            return
        };

        // Otherwise get the collected fee, and check if it can distributed later.
        let coin = coin::drain_aggregatable_coin(&mut collected_fees.amount);
        if (option::is_some(&collected_fees.proposer)) {
            // Extract the address of proposer here and reset it to option::none(). This
            // is particularly useful to avoid any undesired side-effects where coins are
            // collected but never distributed or distributed to the wrong account.
            // With this design, processing collected fees enforces that all fees will be burnt
            // unless the proposer is specified in the block prologue. When we have a governance
            // proposal that triggers reconfiguration, we distribute pending fees and burn the
            // fee for the proposal. Otherwise, that fee would be leaked to the next block.
            let proposer = option::extract(&mut collected_fees.proposer);

            // Since the block can be produced by the VM itself, we have to make sure we catch
            // this case.
            if (proposer == @vm_reserved) {
                burn_coin_fraction(&mut coin, 100);
                coin::destroy_zero(coin);
                return
            };

            burn_coin_fraction(&mut coin, collected_fees.burn_percentage);
            stake::add_transaction_fee(proposer, coin);
            return
        };

        // If checks did not pass, simply burn all collected coins and return none.
        burn_coin_fraction(&mut coin, 100);
        coin::destroy_zero(coin)
    }
}
```


<a id="0x1_transaction_fee_burn_fee"></a>

## Function `burn_fee`

Burn transaction fees in epilogue.


```move
module 0x1::transaction_fee {
    public(friend) fun burn_fee(account: address, fee: u64)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun burn_fee(account: address, fee: u64) acquires AptosFABurnCapabilities, AptosCoinCapabilities {
        if (exists<AptosFABurnCapabilities>(@aptos_framework)) {
            let burn_ref = &borrow_global<AptosFABurnCapabilities>(@aptos_framework).burn_ref;
            aptos_account::burn_from_fungible_store(burn_ref, account, fee);
        } else {
            let burn_cap = &borrow_global<AptosCoinCapabilities>(@aptos_framework).burn_cap;
            if (features::operations_default_to_fa_apt_store_enabled()) {
                let (burn_ref, burn_receipt) = coin::get_paired_burn_ref(burn_cap);
                aptos_account::burn_from_fungible_store(&burn_ref, account, fee);
                coin::return_paired_burn_ref(burn_ref, burn_receipt);
            } else {
                coin::burn_from<AptosCoin>(
                    account,
                    fee,
                    burn_cap,
                );
            };
        };
    }
}
```


<a id="0x1_transaction_fee_mint_and_refund"></a>

## Function `mint_and_refund`

Mint refund in epilogue.


```move
module 0x1::transaction_fee {
    public(friend) fun mint_and_refund(account: address, refund: u64)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun mint_and_refund(account: address, refund: u64) acquires AptosCoinMintCapability {
        let mint_cap = &borrow_global<AptosCoinMintCapability>(@aptos_framework).mint_cap;
        let refund_coin = coin::mint(refund, mint_cap);
        coin::force_deposit(account, refund_coin);
    }
}
```


<a id="0x1_transaction_fee_collect_fee"></a>

## Function `collect_fee`

Collect transaction fees in epilogue.


```move
module 0x1::transaction_fee {
    public(friend) fun collect_fee(account: address, fee: u64)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun collect_fee(account: address, fee: u64) acquires CollectedFeesPerBlock {
        let collected_fees = borrow_global_mut<CollectedFeesPerBlock>(@aptos_framework);

        // Here, we are always optimistic and always collect fees. If the proposer is not set,
        // or we cannot redistribute fees later for some reason (e.g. account cannot receive AptoCoin)
        // we burn them all at once. This way we avoid having a check for every transaction epilogue.
        let collected_amount = &mut collected_fees.amount;
        coin::collect_into_aggregatable_coin<AptosCoin>(account, fee, collected_amount);
    }
}
```


<a id="0x1_transaction_fee_store_aptos_coin_burn_cap"></a>

## Function `store_aptos_coin_burn_cap`

Only called during genesis.


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_burn_cap(aptos_framework: &signer, burn_cap: coin::BurnCapability<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_burn_cap(aptos_framework: &signer, burn_cap: BurnCapability<AptosCoin>) {
        system_addresses::assert_aptos_framework(aptos_framework);

        if (features::operations_default_to_fa_apt_store_enabled()) {
            let burn_ref = coin::convert_and_take_paired_burn_ref(burn_cap);
            move_to(aptos_framework, AptosFABurnCapabilities { burn_ref });
        } else {
            move_to(aptos_framework, AptosCoinCapabilities { burn_cap })
        }
    }
}
```


<a id="0x1_transaction_fee_convert_to_aptos_fa_burn_ref"></a>

## Function `convert_to_aptos_fa_burn_ref`



```move
module 0x1::transaction_fee {
    public entry fun convert_to_aptos_fa_burn_ref(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public entry fun convert_to_aptos_fa_burn_ref(aptos_framework: &signer) acquires AptosCoinCapabilities {
        assert!(features::operations_default_to_fa_apt_store_enabled(), EFA_GAS_CHARGING_NOT_ENABLED);
        system_addresses::assert_aptos_framework(aptos_framework);
        let AptosCoinCapabilities {
            burn_cap,
        } = move_from<AptosCoinCapabilities>(signer::address_of(aptos_framework));
        let burn_ref = coin::convert_and_take_paired_burn_ref(burn_cap);
        move_to(aptos_framework, AptosFABurnCapabilities { burn_ref });
    }
}
```


<a id="0x1_transaction_fee_store_aptos_coin_mint_cap"></a>

## Function `store_aptos_coin_mint_cap`

Only called during genesis.


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_mint_cap(aptos_framework: &signer, mint_cap: coin::MintCapability<aptos_coin::AptosCoin>)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_mint_cap(aptos_framework: &signer, mint_cap: MintCapability<AptosCoin>) {
        system_addresses::assert_aptos_framework(aptos_framework);
        move_to(aptos_framework, AptosCoinMintCapability { mint_cap })
    }
}
```


<a id="0x1_transaction_fee_initialize_storage_refund"></a>

## Function `initialize_storage_refund`



```move
module 0x1::transaction_fee {
    #[deprecated]
    public fun initialize_storage_refund(_: &signer)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    public fun initialize_storage_refund(_: &signer) {
        abort error::not_implemented(ENO_LONGER_SUPPORTED)
    }
}
```


<a id="0x1_transaction_fee_emit_fee_statement"></a>

## Function `emit_fee_statement`



```move
module 0x1::transaction_fee {
    fun emit_fee_statement(fee_statement: transaction_fee::FeeStatement)
}
```


##### Implementation


```move
module 0x1::transaction_fee {
    fun emit_fee_statement(fee_statement: FeeStatement) {
        event::emit(fee_statement)
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
<td>Given the blockchain is in an operating state, it guarantees that the Aptos framework signer may burn Aptos coins.</td>
<td>Critical</td>
<td>The AptosCoinCapabilities structure is defined in this module and it stores burn capability to burn the gas fees.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;1](module).</td>
</tr>

<tr>
<td>2</td>
<td>The initialization function may only be called once.</td>
<td>Medium</td>
<td>The initialize_fee_collection_and_distribution function ensures CollectedFeesPerBlock does not already exist.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](initialize_fee_collection_and_distribution).</td>
</tr>

<tr>
<td>3</td>
<td>Only the admin address is authorized to call the initialization function.</td>
<td>Critical</td>
<td>The initialize_fee_collection_and_distribution function ensures only the Aptos framework address calls it.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](initialize_fee_collection_and_distribution).</td>
</tr>

<tr>
<td>4</td>
<td>The percentage of the burnt collected fee is always a value from 0 to 100.</td>
<td>Medium</td>
<td>During the initialization of CollectedFeesPerBlock in Initialize_fee_collection_and_distribution, and while upgrading burn percentage, it asserts that burn_percentage is within the specified limits.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](CollectedFeesPerBlock).</td>
</tr>

<tr>
<td>5</td>
<td>Prior to upgrading the burn percentage, it must process all the fees collected up to that point.</td>
<td>Critical</td>
<td>The upgrade_burn_percentage function ensures process_collected_fees function is called before updating the burn percentage.</td>
<td>Formally verified in [#high&#45;level&#45;req&#45;5](ProcessCollectedFeesRequiresAndEnsures).</td>
</tr>

<tr>
<td>6</td>
<td>The presence of the resource, indicating collected fees per block under the Aptos framework account, is a prerequisite for the successful execution of the following functionalities: Upgrading burn percentage. Registering a block proposer. Processing collected fees.</td>
<td>Low</td>
<td>The functions: upgrade_burn_percentage, register_proposer_for_fee_collection, and process_collected_fees all ensure that the CollectedFeesPerBlock resource exists under aptos_framework by calling the is_fees_collection_enabled method, which returns a boolean value confirming if the resource exists or not.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;6.1](register_proposer_for_fee_collection), [#high&#45;level&#45;req&#45;6.2](process_collected_fees), and [#high&#45;level&#45;req&#45;6.3](upgrade_burn_percentage).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::transaction_fee {
    pragma verify = false;
    pragma aborts_if_is_strict;
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    invariant [suspendable] chain_status::is_operating() ==> exists<AptosCoinCapabilities>(@aptos_framework) || exists<AptosFABurnCapabilities>(@aptos_framework);
}
```


<a id="@Specification_1_CollectedFeesPerBlock"></a>

### Resource `CollectedFeesPerBlock`


```move
module 0x1::transaction_fee {
    struct CollectedFeesPerBlock has key
}
```


<dl>
<dt>
`amount: coin::AggregatableCoin<aptos_coin::AptosCoin>`
</dt>
<dd>

</dd>
<dt>
`proposer: option::Option<address>`
</dt>
<dd>

</dd>
<dt>
`burn_percentage: u8`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::transaction_fee {
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    invariant burn_percentage <= 100;
}
```


<a id="@Specification_1_initialize_fee_collection_and_distribution"></a>

### Function `initialize_fee_collection_and_distribution`


```move
module 0x1::transaction_fee {
    public fun initialize_fee_collection_and_distribution(aptos_framework: &signer, burn_percentage: u8)
}
```



```move
module 0x1::transaction_fee {
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if exists<CollectedFeesPerBlock>(@aptos_framework);
    aborts_if burn_percentage > 100;
    let aptos_addr = signer::address_of(aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    aborts_if exists<ValidatorFees>(aptos_addr);
    include system_addresses::AbortsIfNotAptosFramework { account: aptos_framework };
    include aggregator_factory::CreateAggregatorInternalAbortsIf;
    aborts_if exists<CollectedFeesPerBlock>(aptos_addr);
    ensures exists<ValidatorFees>(aptos_addr);
    ensures exists<CollectedFeesPerBlock>(aptos_addr);
}
```


<a id="@Specification_1_upgrade_burn_percentage"></a>

### Function `upgrade_burn_percentage`


```move
module 0x1::transaction_fee {
    public fun upgrade_burn_percentage(aptos_framework: &signer, new_burn_percentage: u8)
}
```



```move
module 0x1::transaction_fee {
    aborts_if new_burn_percentage > 100;
    let aptos_addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
// This enforces ### high&#45;level&#45;req&#45;5
[#high&#45;level&#45;req](high&#45;level requirement 5) and ### high&#45;level&#45;req&#45;6.3
[#high&#45;level&#45;req](high&#45;level requirement 6):
    include ProcessCollectedFeesRequiresAndEnsures;
    ensures exists<CollectedFeesPerBlock>(@aptos_framework) ==>
        global<CollectedFeesPerBlock>(@aptos_framework).burn_percentage == new_burn_percentage;
}
```


<a id="@Specification_1_register_proposer_for_fee_collection"></a>

### Function `register_proposer_for_fee_collection`


```move
module 0x1::transaction_fee {
    public(friend) fun register_proposer_for_fee_collection(proposer_addr: address)
}
```



```move
module 0x1::transaction_fee {
    aborts_if false;
// This enforces ### high&#45;level&#45;req&#45;6.1
[#high&#45;level&#45;req](high&#45;level requirement 6):
    ensures is_fees_collection_enabled() ==>
        option::spec_borrow(global<CollectedFeesPerBlock>(@aptos_framework).proposer) == proposer_addr;
}
```


<a id="@Specification_1_burn_coin_fraction"></a>

### Function `burn_coin_fraction`


```move
module 0x1::transaction_fee {
    fun burn_coin_fraction(coin: &mut coin::Coin<aptos_coin::AptosCoin>, burn_percentage: u8)
}
```



```move
module 0x1::transaction_fee {
    requires burn_percentage <= 100;
    requires exists<AptosCoinCapabilities>(@aptos_framework);
    requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
    let amount_to_burn = (burn_percentage * coin::value(coin)) / 100;
    include amount_to_burn > 0 ==> coin::CoinSubAbortsIf<AptosCoin> { amount: amount_to_burn };
    ensures coin.value == old(coin).value - amount_to_burn;
}
```



<a id="0x1_transaction_fee_collectedFeesAggregator"></a>


```move
module 0x1::transaction_fee {
    fun collectedFeesAggregator(): AggregatableCoin<AptosCoin> {
       global<CollectedFeesPerBlock>(@aptos_framework).amount
    }
}
```



<a id="0x1_transaction_fee_RequiresCollectedFeesPerValueLeqBlockAptosSupply"></a>


```move
module 0x1::transaction_fee {
    schema RequiresCollectedFeesPerValueLeqBlockAptosSupply {
        let maybe_supply = coin::get_coin_supply_opt<AptosCoin>();
        requires
            (is_fees_collection_enabled() && option::is_some(maybe_supply)) ==>
                (aggregator::spec_aggregator_get_val(global<CollectedFeesPerBlock>(@aptos_framework).amount.value) <=
                    optional_aggregator::optional_aggregator_value(
                        option::spec_borrow(coin::get_coin_supply_opt<AptosCoin>())
                    ));
    }
}
```



<a id="0x1_transaction_fee_ProcessCollectedFeesRequiresAndEnsures"></a>


```move
module 0x1::transaction_fee {
    schema ProcessCollectedFeesRequiresAndEnsures {
        requires exists<AptosCoinCapabilities>(@aptos_framework);
        requires exists<stake::ValidatorFees>(@aptos_framework);
        requires exists<CoinInfo<AptosCoin>>(@aptos_framework);
        include RequiresCollectedFeesPerValueLeqBlockAptosSupply;
        aborts_if false;
        let collected_fees = global<CollectedFeesPerBlock>(@aptos_framework);
        let post post_collected_fees = global<CollectedFeesPerBlock>(@aptos_framework);
        let pre_amount = aggregator::spec_aggregator_get_val(collected_fees.amount.value);
        let post post_amount = aggregator::spec_aggregator_get_val(post_collected_fees.amount.value);
        let fees_table = global<stake::ValidatorFees>(@aptos_framework).fees_table;
        let post post_fees_table = global<stake::ValidatorFees>(@aptos_framework).fees_table;
        let proposer = option::spec_borrow(collected_fees.proposer);
        let fee_to_add = pre_amount - pre_amount * collected_fees.burn_percentage / 100;
        ensures is_fees_collection_enabled() ==> option::spec_is_none(post_collected_fees.proposer) && post_amount == 0;
        ensures is_fees_collection_enabled() && aggregator::spec_read(collected_fees.amount.value) > 0 &&
            option::spec_is_some(collected_fees.proposer) ==>
            if (proposer != @vm_reserved) {
                if (table::spec_contains(fees_table, proposer)) {
                    table::spec_get(post_fees_table, proposer).value == table::spec_get(
                        fees_table,
                        proposer
                    ).value + fee_to_add
                } else {
                    table::spec_get(post_fees_table, proposer).value == fee_to_add
                }
            } else {
                option::spec_is_none(post_collected_fees.proposer) && post_amount == 0
            };
    }
}
```


<a id="@Specification_1_process_collected_fees"></a>

### Function `process_collected_fees`


```move
module 0x1::transaction_fee {
    public(friend) fun process_collected_fees()
}
```



```move
module 0x1::transaction_fee {
// This enforces ### high&#45;level&#45;req&#45;6.2
[#high&#45;level&#45;req](high&#45;level requirement 6):
    include ProcessCollectedFeesRequiresAndEnsures;
}
```


<a id="@Specification_1_burn_fee"></a>

### Function `burn_fee`


```move
module 0x1::transaction_fee {
    public(friend) fun burn_fee(account: address, fee: u64)
}
```

`AptosCoinCapabilities` should be exists.


```move
module 0x1::transaction_fee {
    pragma verify = false;
    aborts_if !exists<AptosCoinCapabilities>(@aptos_framework);
    let account_addr = account;
    let amount = fee;
    let aptos_addr = type_info::type_of<AptosCoin>().account_address;
    let coin_store = global<CoinStore<AptosCoin>>(account_addr);
    let post post_coin_store = global<CoinStore<AptosCoin>>(account_addr);
    aborts_if amount != 0 && !(exists<CoinInfo<AptosCoin>>(aptos_addr)
        && exists<CoinStore<AptosCoin>>(account_addr));
    aborts_if coin_store.coin.value < amount;
    let maybe_supply = global<CoinInfo<AptosCoin>>(aptos_addr).supply;
    let supply_aggr = option::spec_borrow(maybe_supply);
    let value = optional_aggregator::optional_aggregator_value(supply_aggr);
    let post post_maybe_supply = global<CoinInfo<AptosCoin>>(aptos_addr).supply;
    let post post_supply = option::spec_borrow(post_maybe_supply);
    let post post_value = optional_aggregator::optional_aggregator_value(post_supply);
    aborts_if option::spec_is_some(maybe_supply) && value < amount;
    ensures post_coin_store.coin.value == coin_store.coin.value - amount;
    ensures if (option::spec_is_some(maybe_supply)) {
        post_value == value - amount
    } else {
        option::spec_is_none(post_maybe_supply)
    };
    ensures coin::supply<AptosCoin> == old(coin::supply<AptosCoin>) - amount;
}
```


<a id="@Specification_1_mint_and_refund"></a>

### Function `mint_and_refund`


```move
module 0x1::transaction_fee {
    public(friend) fun mint_and_refund(account: address, refund: u64)
}
```



```move
module 0x1::transaction_fee {
    pragma verify = false;
    let aptos_addr = type_info::type_of<AptosCoin>().account_address;
    aborts_if (refund != 0) && !exists<CoinInfo<AptosCoin>>(aptos_addr);
    include coin::CoinAddAbortsIf<AptosCoin> { amount: refund };
    aborts_if !exists<CoinStore<AptosCoin>>(account);
    aborts_if !exists<AptosCoinMintCapability>(@aptos_framework);
    let supply = coin::supply<AptosCoin>;
    let post post_supply = coin::supply<AptosCoin>;
    aborts_if [abstract] supply + refund > MAX_U128;
    ensures post_supply == supply + refund;
}
```


<a id="@Specification_1_collect_fee"></a>

### Function `collect_fee`


```move
module 0x1::transaction_fee {
    public(friend) fun collect_fee(account: address, fee: u64)
}
```



```move
module 0x1::transaction_fee {
    pragma verify = false;
    let collected_fees = global<CollectedFeesPerBlock>(@aptos_framework).amount;
    let aggr = collected_fees.value;
    let coin_store = global<coin::CoinStore<AptosCoin>>(account);
    aborts_if !exists<CollectedFeesPerBlock>(@aptos_framework);
    aborts_if fee > 0 && !exists<coin::CoinStore<AptosCoin>>(account);
    aborts_if fee > 0 && coin_store.coin.value < fee;
    aborts_if fee > 0 && aggregator::spec_aggregator_get_val(aggr)
        + fee > aggregator::spec_get_limit(aggr);
    aborts_if fee > 0 && aggregator::spec_aggregator_get_val(aggr)
        + fee > MAX_U128;
    let post post_coin_store = global<coin::CoinStore<AptosCoin>>(account);
    let post post_collected_fees = global<CollectedFeesPerBlock>(@aptos_framework).amount;
    ensures post_coin_store.coin.value == coin_store.coin.value - fee;
    ensures aggregator::spec_aggregator_get_val(post_collected_fees.value) == aggregator::spec_aggregator_get_val(
        aggr
    ) + fee;
}
```


<a id="@Specification_1_store_aptos_coin_burn_cap"></a>

### Function `store_aptos_coin_burn_cap`


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_burn_cap(aptos_framework: &signer, burn_cap: coin::BurnCapability<aptos_coin::AptosCoin>)
}
```

Ensure caller is admin.
Aborts if `AptosCoinCapabilities` already exists.


```move
module 0x1::transaction_fee {
    pragma verify = false;
    let addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(addr);
    aborts_if exists<AptosFABurnCapabilities>(addr);
    aborts_if exists<AptosCoinCapabilities>(addr);
    ensures exists<AptosFABurnCapabilities>(addr) || exists<AptosCoinCapabilities>(addr);
}
```


<a id="@Specification_1_store_aptos_coin_mint_cap"></a>

### Function `store_aptos_coin_mint_cap`


```move
module 0x1::transaction_fee {
    public(friend) fun store_aptos_coin_mint_cap(aptos_framework: &signer, mint_cap: coin::MintCapability<aptos_coin::AptosCoin>)
}
```

Ensure caller is admin.
Aborts if `AptosCoinMintCapability` already exists.


```move
module 0x1::transaction_fee {
    let addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(addr);
    aborts_if exists<AptosCoinMintCapability>(addr);
    ensures exists<AptosCoinMintCapability>(addr);
}
```


<a id="@Specification_1_initialize_storage_refund"></a>

### Function `initialize_storage_refund`


```move
module 0x1::transaction_fee {
    #[deprecated]
    public fun initialize_storage_refund(_: &signer)
}
```

Historical. Aborts.


```move
module 0x1::transaction_fee {
    aborts_if true;
}
```


<a id="@Specification_1_emit_fee_statement"></a>

### Function `emit_fee_statement`


```move
module 0x1::transaction_fee {
    fun emit_fee_statement(fee_statement: transaction_fee::FeeStatement)
}
```

Aborts if module event feature is not enabled.
