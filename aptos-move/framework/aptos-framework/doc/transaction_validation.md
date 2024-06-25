
<a id="0x1_transaction_validation"></a>

# Module `0x1::transaction_validation`



-  [Resource `TransactionValidation`](#0x1_transaction_validation_TransactionValidation)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_transaction_validation_initialize)
-  [Function `collect_deposit`](#0x1_transaction_validation_collect_deposit)
-  [Function `return_deposit`](#0x1_transaction_validation_return_deposit)
-  [Function `prologue_common`](#0x1_transaction_validation_prologue_common)
-  [Function `script_prologue`](#0x1_transaction_validation_script_prologue)
-  [Function `script_prologue_collect_deposit`](#0x1_transaction_validation_script_prologue_collect_deposit)
-  [Function `multi_agent_script_prologue`](#0x1_transaction_validation_multi_agent_script_prologue)
-  [Function `multi_agent_common_prologue`](#0x1_transaction_validation_multi_agent_common_prologue)
-  [Function `fee_payer_script_prologue`](#0x1_transaction_validation_fee_payer_script_prologue)
-  [Function `fee_payer_script_prologue_collect_deposit`](#0x1_transaction_validation_fee_payer_script_prologue_collect_deposit)
-  [Function `epilogue`](#0x1_transaction_validation_epilogue)
-  [Function `epilogue_return_deposit`](#0x1_transaction_validation_epilogue_return_deposit)
-  [Function `epilogue_gas_payer`](#0x1_transaction_validation_epilogue_gas_payer)
-  [Function `epilogue_gas_payer_return_deposit`](#0x1_transaction_validation_epilogue_gas_payer_return_deposit)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `collect_deposit`](#@Specification_1_collect_deposit)
    -  [Function `return_deposit`](#@Specification_1_return_deposit)
    -  [Function `prologue_common`](#@Specification_1_prologue_common)
    -  [Function `script_prologue`](#@Specification_1_script_prologue)
    -  [Function `script_prologue_collect_deposit`](#@Specification_1_script_prologue_collect_deposit)
    -  [Function `multi_agent_script_prologue`](#@Specification_1_multi_agent_script_prologue)
    -  [Function `multi_agent_common_prologue`](#@Specification_1_multi_agent_common_prologue)
    -  [Function `fee_payer_script_prologue`](#@Specification_1_fee_payer_script_prologue)
    -  [Function `fee_payer_script_prologue_collect_deposit`](#@Specification_1_fee_payer_script_prologue_collect_deposit)
    -  [Function `epilogue`](#@Specification_1_epilogue)
    -  [Function `epilogue_return_deposit`](#@Specification_1_epilogue_return_deposit)
    -  [Function `epilogue_gas_payer`](#@Specification_1_epilogue_gas_payer)
    -  [Function `epilogue_gas_payer_return_deposit`](#@Specification_1_epilogue_gas_payer_return_deposit)


```move
module 0x1::transaction_validation {
    use 0x1::account;
    use 0x1::aptos_account;
    use 0x1::aptos_coin;
    use 0x1::bcs;
    use 0x1::chain_id;
    use 0x1::coin;
    use 0x1::error;
    use 0x1::features;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::system_addresses;
    use 0x1::timestamp;
    use 0x1::transaction_fee;
}
```


<a id="0x1_transaction_validation_TransactionValidation"></a>

## Resource `TransactionValidation`

This holds information that will be picked up by the VM to call the
correct chain&#45;specific prologue and epilogue functions


```move
module 0x1::transaction_validation {
    struct TransactionValidation has key
}
```


##### Fields


<dl>
<dt>
`module_addr: address`
</dt>
<dd>

</dd>
<dt>
`module_name: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`script_prologue_name: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`module_prologue_name: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`multi_agent_prologue_name: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`user_epilogue_name: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_transaction_validation_MAX_U64"></a>

MSB is used to indicate a gas payer tx


```move
module 0x1::transaction_validation {
    const MAX_U64: u128 = 18446744073709551615;
}
```


<a id="0x1_transaction_validation_EOUT_OF_GAS"></a>

Transaction exceeded its allocated max gas


```move
module 0x1::transaction_validation {
    const EOUT_OF_GAS: u64 = 6;
}
```


<a id="0x1_transaction_validation_PROLOGUE_EACCOUNT_DOES_NOT_EXIST"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_EACCOUNT_DOES_NOT_EXIST: u64 = 1004;
}
```


<a id="0x1_transaction_validation_PROLOGUE_EBAD_CHAIN_ID"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_EBAD_CHAIN_ID: u64 = 1007;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ECANT_PAY_GAS_DEPOSIT"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ECANT_PAY_GAS_DEPOSIT: u64 = 1005;
}
```


<a id="0x1_transaction_validation_PROLOGUE_EFEE_PAYER_NOT_ENABLED"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_EFEE_PAYER_NOT_ENABLED: u64 = 1010;
}
```


<a id="0x1_transaction_validation_PROLOGUE_EINSUFFICIENT_BALANCE_FOR_REQUIRED_DEPOSIT"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_EINSUFFICIENT_BALANCE_FOR_REQUIRED_DEPOSIT: u64 = 1011;
}
```


<a id="0x1_transaction_validation_PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY"></a>

Prologue errors. These are separated out from the other errors in this
module since they are mapped separately to major VM statuses, and are
important to the semantics of the system.


```move
module 0x1::transaction_validation {
    const PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY: u64 = 1001;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ESECONDARY_KEYS_ADDRESSES_COUNT_MISMATCH"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ESECONDARY_KEYS_ADDRESSES_COUNT_MISMATCH: u64 = 1009;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ESEQUENCE_NUMBER_TOO_BIG"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ESEQUENCE_NUMBER_TOO_BIG: u64 = 1008;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ESEQUENCE_NUMBER_TOO_NEW"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ESEQUENCE_NUMBER_TOO_NEW: u64 = 1003;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ESEQUENCE_NUMBER_TOO_OLD"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ESEQUENCE_NUMBER_TOO_OLD: u64 = 1002;
}
```


<a id="0x1_transaction_validation_PROLOGUE_ETRANSACTION_EXPIRED"></a>



```move
module 0x1::transaction_validation {
    const PROLOGUE_ETRANSACTION_EXPIRED: u64 = 1006;
}
```


<a id="0x1_transaction_validation_initialize"></a>

## Function `initialize`

Only called during genesis to initialize system resources for this module.


```move
module 0x1::transaction_validation {
    public(friend) fun initialize(aptos_framework: &signer, script_prologue_name: vector<u8>, module_prologue_name: vector<u8>, multi_agent_prologue_name: vector<u8>, user_epilogue_name: vector<u8>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    public(friend) fun initialize(
        aptos_framework: &signer,
        script_prologue_name: vector<u8>,
        // module_prologue_name is deprecated and not used.
        module_prologue_name: vector<u8>,
        multi_agent_prologue_name: vector<u8>,
        user_epilogue_name: vector<u8>,
    ) {
        system_addresses::assert_aptos_framework(aptos_framework);

        move_to(aptos_framework, TransactionValidation {
            module_addr: @aptos_framework,
            module_name: b"transaction_validation",
            script_prologue_name,
            // module_prologue_name is deprecated and not used.
            module_prologue_name,
            multi_agent_prologue_name,
            user_epilogue_name,
        });
    }
}
```


<a id="0x1_transaction_validation_collect_deposit"></a>

## Function `collect_deposit`

Called in prologue to optionally hold some amount for special txns (e.g. randomness txns).
`return_deposit()` should be invoked in the corresponding epilogue with the same arguments.


```move
module 0x1::transaction_validation {
    fun collect_deposit(gas_payer: address, amount: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun collect_deposit(gas_payer: address, amount: Option<u64>) {
        if (option::is_some(&amount)) {
            let amount = option::extract(&mut amount);
            let balance = coin::balance<AptosCoin>(gas_payer);
            assert!(balance >= amount, error::invalid_state(PROLOGUE_EINSUFFICIENT_BALANCE_FOR_REQUIRED_DEPOSIT));
            transaction_fee::burn_fee(gas_payer, amount);
        }
    }
}
```


<a id="0x1_transaction_validation_return_deposit"></a>

## Function `return_deposit`

Called in epilogue to optionally released the amount held in prologue for special txns (e.g. randomness txns).


```move
module 0x1::transaction_validation {
    fun return_deposit(gas_payer: address, amount: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun return_deposit(gas_payer: address, amount: Option<u64>) {
        if (option::is_some(&amount)) {
            let amount = option::extract(&mut amount);
            transaction_fee::mint_and_refund(gas_payer, amount);
        }
    }
}
```


<a id="0x1_transaction_validation_prologue_common"></a>

## Function `prologue_common`



```move
module 0x1::transaction_validation {
    fun prologue_common(sender: signer, gas_payer: address, txn_sequence_number: u64, txn_authentication_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun prologue_common(
        sender: signer,
        gas_payer: address,
        txn_sequence_number: u64,
        txn_authentication_key: vector<u8>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
    ) {
        assert!(
            timestamp::now_seconds() < txn_expiration_time,
            error::invalid_argument(PROLOGUE_ETRANSACTION_EXPIRED),
        );
        assert!(chain_id::get() == chain_id, error::invalid_argument(PROLOGUE_EBAD_CHAIN_ID));

        let transaction_sender = signer::address_of(&sender);

        if (
            transaction_sender == gas_payer
            || account::exists_at(transaction_sender)
            || !features::sponsored_automatic_account_creation_enabled()
            || txn_sequence_number > 0
        ) {
            assert!(account::exists_at(transaction_sender), error::invalid_argument(PROLOGUE_EACCOUNT_DOES_NOT_EXIST));
            assert!(
                txn_authentication_key == account::get_authentication_key(transaction_sender),
                error::invalid_argument(PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY),
            );

            let account_sequence_number = account::get_sequence_number(transaction_sender);
            assert!(
                txn_sequence_number < (1u64 << 63),
                error::out_of_range(PROLOGUE_ESEQUENCE_NUMBER_TOO_BIG)
            );

            assert!(
                txn_sequence_number >= account_sequence_number,
                error::invalid_argument(PROLOGUE_ESEQUENCE_NUMBER_TOO_OLD)
            );

            assert!(
                txn_sequence_number == account_sequence_number,
                error::invalid_argument(PROLOGUE_ESEQUENCE_NUMBER_TOO_NEW)
            );
        } else {
            // In this case, the transaction is sponsored and the account does not exist, so ensure
            // the default values match.
            assert!(
                txn_sequence_number == 0,
                error::invalid_argument(PROLOGUE_ESEQUENCE_NUMBER_TOO_NEW)
            );

            assert!(
                txn_authentication_key == bcs::to_bytes(&transaction_sender),
                error::invalid_argument(PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY),
            );
        };

        let max_transaction_fee = txn_gas_price * txn_max_gas_units;

        if (features::operations_default_to_fa_apt_store_enabled()) {
            assert!(
                aptos_account::is_fungible_balance_at_least(gas_payer, max_transaction_fee),
                error::invalid_argument(PROLOGUE_ECANT_PAY_GAS_DEPOSIT)
            );
        } else {
            assert!(
                coin::is_balance_at_least<AptosCoin>(gas_payer, max_transaction_fee),
                error::invalid_argument(PROLOGUE_ECANT_PAY_GAS_DEPOSIT)
            );
        }
    }
}
```


<a id="0x1_transaction_validation_script_prologue"></a>

## Function `script_prologue`



```move
module 0x1::transaction_validation {
    fun script_prologue(sender: signer, txn_sequence_number: u64, txn_public_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, _script_hash: vector<u8>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun script_prologue(
        sender: signer,
        txn_sequence_number: u64,
        txn_public_key: vector<u8>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
        _script_hash: vector<u8>,
    ) {
        let gas_payer = signer::address_of(&sender);
        prologue_common(sender, gas_payer, txn_sequence_number, txn_public_key, txn_gas_price, txn_max_gas_units, txn_expiration_time, chain_id)
    }
}
```


<a id="0x1_transaction_validation_script_prologue_collect_deposit"></a>

## Function `script_prologue_collect_deposit`

`script_prologue()` then collect an optional deposit depending on the txn.

Deposit collection goes last so `script_prologue()` doesn&apos;t have to be aware of the deposit logic.


```move
module 0x1::transaction_validation {
    fun script_prologue_collect_deposit(sender: signer, txn_sequence_number: u64, txn_public_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, script_hash: vector<u8>, required_deposit: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun script_prologue_collect_deposit(
        sender: signer,
        txn_sequence_number: u64,
        txn_public_key: vector<u8>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
        script_hash: vector<u8>,
        required_deposit: Option<u64>,
    ) {
        let gas_payer = signer::address_of(&sender);
        script_prologue(sender, txn_sequence_number, txn_public_key, txn_gas_price, txn_max_gas_units, txn_expiration_time, chain_id, script_hash);
        collect_deposit(gas_payer, required_deposit);
    }
}
```


<a id="0x1_transaction_validation_multi_agent_script_prologue"></a>

## Function `multi_agent_script_prologue`



```move
module 0x1::transaction_validation {
    fun multi_agent_script_prologue(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun multi_agent_script_prologue(
        sender: signer,
        txn_sequence_number: u64,
        txn_sender_public_key: vector<u8>,
        secondary_signer_addresses: vector<address>,
        secondary_signer_public_key_hashes: vector<vector<u8>>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
    ) {
        let sender_addr = signer::address_of(&sender);
        prologue_common(
            sender,
            sender_addr,
            txn_sequence_number,
            txn_sender_public_key,
            txn_gas_price,
            txn_max_gas_units,
            txn_expiration_time,
            chain_id,
        );
        multi_agent_common_prologue(secondary_signer_addresses, secondary_signer_public_key_hashes);
    }
}
```


<a id="0x1_transaction_validation_multi_agent_common_prologue"></a>

## Function `multi_agent_common_prologue`



```move
module 0x1::transaction_validation {
    fun multi_agent_common_prologue(secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun multi_agent_common_prologue(
        secondary_signer_addresses: vector<address>,
        secondary_signer_public_key_hashes: vector<vector<u8>>,
    ) {
        let num_secondary_signers = vector::length(&secondary_signer_addresses);
        assert!(
            vector::length(&secondary_signer_public_key_hashes) == num_secondary_signers,
            error::invalid_argument(PROLOGUE_ESECONDARY_KEYS_ADDRESSES_COUNT_MISMATCH),
        );

        let i = 0;
        while ({
            spec {
                invariant i <= num_secondary_signers;
                invariant forall j in 0..i:
                    account::exists_at(secondary_signer_addresses[j])
                    && secondary_signer_public_key_hashes[j]
                       == account::get_authentication_key(secondary_signer_addresses[j]);
            };
            (i < num_secondary_signers)
        }) {
            let secondary_address = *vector::borrow(&secondary_signer_addresses, i);
            assert!(account::exists_at(secondary_address), error::invalid_argument(PROLOGUE_EACCOUNT_DOES_NOT_EXIST));

            let signer_public_key_hash = *vector::borrow(&secondary_signer_public_key_hashes, i);
            assert!(
                signer_public_key_hash == account::get_authentication_key(secondary_address),
                error::invalid_argument(PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY),
            );
            i = i + 1;
        }
    }
}
```


<a id="0x1_transaction_validation_fee_payer_script_prologue"></a>

## Function `fee_payer_script_prologue`



```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, fee_payer_address: address, fee_payer_public_key_hash: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue(
        sender: signer,
        txn_sequence_number: u64,
        txn_sender_public_key: vector<u8>,
        secondary_signer_addresses: vector<address>,
        secondary_signer_public_key_hashes: vector<vector<u8>>,
        fee_payer_address: address,
        fee_payer_public_key_hash: vector<u8>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
    ) {
        assert!(features::fee_payer_enabled(), error::invalid_state(PROLOGUE_EFEE_PAYER_NOT_ENABLED));
        prologue_common(
            sender,
            fee_payer_address,
            txn_sequence_number,
            txn_sender_public_key,
            txn_gas_price,
            txn_max_gas_units,
            txn_expiration_time,
            chain_id,
        );
        multi_agent_common_prologue(secondary_signer_addresses, secondary_signer_public_key_hashes);
        assert!(
            fee_payer_public_key_hash == account::get_authentication_key(fee_payer_address),
            error::invalid_argument(PROLOGUE_EINVALID_ACCOUNT_AUTH_KEY),
        );
    }
}
```


<a id="0x1_transaction_validation_fee_payer_script_prologue_collect_deposit"></a>

## Function `fee_payer_script_prologue_collect_deposit`

`fee_payer_script_prologue()` then collect an optional deposit depending on the txn.

Deposit collection goes last so `fee_payer_script_prologue()` doesn&apos;t have to be aware of the deposit logic.


```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue_collect_deposit(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, fee_payer_address: address, fee_payer_public_key_hash: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, required_deposit: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue_collect_deposit(
        sender: signer,
        txn_sequence_number: u64,
        txn_sender_public_key: vector<u8>,
        secondary_signer_addresses: vector<address>,
        secondary_signer_public_key_hashes: vector<vector<u8>>,
        fee_payer_address: address,
        fee_payer_public_key_hash: vector<u8>,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        txn_expiration_time: u64,
        chain_id: u8,
        required_deposit: Option<u64>,
    ) {
        fee_payer_script_prologue(
            sender,
            txn_sequence_number,
            txn_sender_public_key,
            secondary_signer_addresses,
            secondary_signer_public_key_hashes,
            fee_payer_address,
            fee_payer_public_key_hash,
            txn_gas_price,
            txn_max_gas_units,
            txn_expiration_time,
            chain_id,
        );
        collect_deposit(fee_payer_address, required_deposit);
    }
}
```


<a id="0x1_transaction_validation_epilogue"></a>

## Function `epilogue`

Epilogue function is run after a transaction is successfully executed.
Called by the Adapter


```move
module 0x1::transaction_validation {
    fun epilogue(account: signer, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun epilogue(
        account: signer,
        storage_fee_refunded: u64,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        gas_units_remaining: u64
    ) {
        let addr = signer::address_of(&account);
        epilogue_gas_payer(account, addr, storage_fee_refunded, txn_gas_price, txn_max_gas_units, gas_units_remaining);
    }
}
```


<a id="0x1_transaction_validation_epilogue_return_deposit"></a>

## Function `epilogue_return_deposit`

Return the deposit held in prologue, then `epilogue()`.

Deposit return goes first so `epilogue()` doesn&apos;t have to be aware of this change.


```move
module 0x1::transaction_validation {
    fun epilogue_return_deposit(account: signer, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64, required_deposit: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun epilogue_return_deposit(
        account: signer,
        storage_fee_refunded: u64,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        gas_units_remaining: u64,
        required_deposit: Option<u64>,
    ) {
        let gas_payer = signer::address_of(&account);
        return_deposit(gas_payer, required_deposit);
        epilogue(
            account,
            storage_fee_refunded,
            txn_gas_price,
            txn_max_gas_units,
            gas_units_remaining,
        );
    }
}
```


<a id="0x1_transaction_validation_epilogue_gas_payer"></a>

## Function `epilogue_gas_payer`

Epilogue function with explicit gas payer specified, is run after a transaction is successfully executed.
Called by the Adapter


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer(account: signer, gas_payer: address, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer(
        account: signer,
        gas_payer: address,
        storage_fee_refunded: u64,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        gas_units_remaining: u64
    ) {
        assert!(txn_max_gas_units >= gas_units_remaining, error::invalid_argument(EOUT_OF_GAS));
        let gas_used = txn_max_gas_units - gas_units_remaining;

        assert!(
            (txn_gas_price as u128) * (gas_used as u128) <= MAX_U64,
            error::out_of_range(EOUT_OF_GAS)
        );
        let transaction_fee_amount = txn_gas_price * gas_used;

        // it's important to maintain the error code consistent with vm
        // to do failed transaction cleanup.
        if (features::operations_default_to_fa_apt_store_enabled()) {
            assert!(
                aptos_account::is_fungible_balance_at_least(gas_payer, transaction_fee_amount),
                error::out_of_range(PROLOGUE_ECANT_PAY_GAS_DEPOSIT),
            );
        } else {
            assert!(
                coin::is_balance_at_least<AptosCoin>(gas_payer, transaction_fee_amount),
                error::out_of_range(PROLOGUE_ECANT_PAY_GAS_DEPOSIT),
            );
        };

        let amount_to_burn = if (features::collect_and_distribute_gas_fees()) {
            // TODO(gas): We might want to distinguish the refundable part of the charge and burn it or track
            // it separately, so that we don't increase the total supply by refunding.

            // If transaction fees are redistributed to validators, collect them here for
            // later redistribution.
            transaction_fee::collect_fee(gas_payer, transaction_fee_amount);
            0
        } else {
            // Otherwise, just burn the fee.
            // TODO: this branch should be removed completely when transaction fee collection
            // is tested and is fully proven to work well.
            transaction_fee_amount
        };

        if (amount_to_burn > storage_fee_refunded) {
            let burn_amount = amount_to_burn - storage_fee_refunded;
            transaction_fee::burn_fee(gas_payer, burn_amount);
        } else if (amount_to_burn < storage_fee_refunded) {
            let mint_amount = storage_fee_refunded - amount_to_burn;
            transaction_fee::mint_and_refund(gas_payer, mint_amount)
        };

        // Increment sequence number
        let addr = signer::address_of(&account);
        account::increment_sequence_number(addr);
    }
}
```


<a id="0x1_transaction_validation_epilogue_gas_payer_return_deposit"></a>

## Function `epilogue_gas_payer_return_deposit`

Return the deposit held in prologue to the gas payer, then `epilogue_gas_payer()`.

Deposit return should go first so `epilogue_gas_payer()` doesn&apos;t have to be aware of this change.


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer_return_deposit(account: signer, gas_payer: address, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64, required_deposit: option::Option<u64>)
}
```


##### Implementation


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer_return_deposit(
        account: signer,
        gas_payer: address,
        storage_fee_refunded: u64,
        txn_gas_price: u64,
        txn_max_gas_units: u64,
        gas_units_remaining: u64,
        required_deposit: Option<u64>,
    ) {
        return_deposit(gas_payer, required_deposit);
        epilogue_gas_payer(
            account,
            gas_payer,
            storage_fee_refunded,
            txn_gas_price,
            txn_max_gas_units,
            gas_units_remaining,
        );
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
<td>The sender of a transaction should have sufficient coin balance to pay the transaction fee.</td>
<td>High</td>
<td>The prologue_common function asserts that the transaction sender has enough coin balance to be paid as the max_transaction_fee.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](PrologueCommonAbortsIf). Moreover, the native transaction validation patterns have been manually audited.</td>
</tr>

<tr>
<td>2</td>
<td>All secondary signer addresses are verified to be authentic through a validation process.</td>
<td>Critical</td>
<td>The function multi_agent_script_prologue ensures that each secondary signer address undergoes authentication validation, including verification of account existence and authentication key matching, confirming their authenticity.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](multi_agent_script_prologue). Moreover, the native transaction validation patterns have been manually audited.</td>
</tr>

<tr>
<td>3</td>
<td>After successful execution, base the transaction fee on the configuration set by the features library.</td>
<td>High</td>
<td>The epilogue function collects the transaction fee for either redistribution or burning based on the feature::collect_and_distribute_gas_fees result.</td>
<td>Formally Verified via [#high&#45;level&#45;req&#45;3](epilogue). Moreover, the native transaction validation patterns have been manually audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::transaction_validation {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::transaction_validation {
    public(friend) fun initialize(aptos_framework: &signer, script_prologue_name: vector<u8>, module_prologue_name: vector<u8>, multi_agent_prologue_name: vector<u8>, user_epilogue_name: vector<u8>)
}
```

Ensure caller is `aptos_framework`.
Aborts if TransactionValidation already exists.


```move
module 0x1::transaction_validation {
    let addr = signer::address_of(aptos_framework);
    aborts_if !system_addresses::is_aptos_framework_address(addr);
    aborts_if exists<TransactionValidation>(addr);
    ensures exists<TransactionValidation>(addr);
}
```

Create a schema to reuse some code.
Give some constraints that may abort according to the conditions.


<a id="0x1_transaction_validation_PrologueCommonAbortsIf"></a>


```move
module 0x1::transaction_validation {
    schema PrologueCommonAbortsIf {
        sender: signer;
        gas_payer: address;
        txn_sequence_number: u64;
        txn_authentication_key: vector<u8>;
        txn_gas_price: u64;
        txn_max_gas_units: u64;
        txn_expiration_time: u64;
        chain_id: u8;
        aborts_if !exists<CurrentTimeMicroseconds>(@aptos_framework);
        aborts_if !(timestamp::now_seconds() < txn_expiration_time);
        aborts_if !exists<ChainId>(@aptos_framework);
        aborts_if !(chain_id::get() == chain_id);
        let transaction_sender = signer::address_of(sender);
        aborts_if (
            !features::spec_is_enabled(features::SPONSORED_AUTOMATIC_ACCOUNT_CREATION)
            || account::exists_at(transaction_sender)
            || transaction_sender == gas_payer
            || txn_sequence_number > 0
        ) && (
            !(txn_sequence_number >= global<Account>(transaction_sender).sequence_number)
            || !(txn_authentication_key == global<Account>(transaction_sender).authentication_key)
            || !account::exists_at(transaction_sender)
            || !(txn_sequence_number == global<Account>(transaction_sender).sequence_number)
        );
        aborts_if features::spec_is_enabled(features::SPONSORED_AUTOMATIC_ACCOUNT_CREATION)
            && transaction_sender != gas_payer
            && txn_sequence_number == 0
            && !account::exists_at(transaction_sender)
            && txn_authentication_key != bcs::to_bytes(transaction_sender);
        aborts_if !(txn_sequence_number < (1u64 << 63));
        let max_transaction_fee = txn_gas_price * txn_max_gas_units;
        aborts_if max_transaction_fee > MAX_U64;
        aborts_if !exists<CoinStore<AptosCoin>>(gas_payer);
    // This enforces ### high&#45;level&#45;req&#45;1
    [#high&#45;level&#45;req](high&#45;level requirement 1):
        aborts_if !(global<CoinStore<AptosCoin>>(gas_payer).coin.value >= max_transaction_fee);
    }
}
```


<a id="@Specification_1_collect_deposit"></a>

### Function `collect_deposit`


```move
module 0x1::transaction_validation {
    fun collect_deposit(gas_payer: address, amount: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```


<a id="@Specification_1_return_deposit"></a>

### Function `return_deposit`


```move
module 0x1::transaction_validation {
    fun return_deposit(gas_payer: address, amount: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```


<a id="@Specification_1_prologue_common"></a>

### Function `prologue_common`


```move
module 0x1::transaction_validation {
    fun prologue_common(sender: signer, gas_payer: address, txn_sequence_number: u64, txn_authentication_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
    include PrologueCommonAbortsIf;
}
```


<a id="@Specification_1_script_prologue"></a>

### Function `script_prologue`


```move
module 0x1::transaction_validation {
    fun script_prologue(sender: signer, txn_sequence_number: u64, txn_public_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, _script_hash: vector<u8>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
    include PrologueCommonAbortsIf {
        gas_payer: signer::address_of(sender),
        txn_authentication_key: txn_public_key
    };
}
```



<a id="0x1_transaction_validation_MultiAgentPrologueCommonAbortsIf"></a>


```move
module 0x1::transaction_validation {
    schema MultiAgentPrologueCommonAbortsIf {
        secondary_signer_addresses: vector<address>;
        secondary_signer_public_key_hashes: vector<vector<u8>>;
        let num_secondary_signers = len(secondary_signer_addresses);
        aborts_if len(secondary_signer_public_key_hashes) != num_secondary_signers;
    // This enforces ### high&#45;level&#45;req&#45;2
    [#high&#45;level&#45;req](high&#45;level requirement 2):
        aborts_if exists i in 0..num_secondary_signers:
            !account::exists_at(secondary_signer_addresses[i])
                || secondary_signer_public_key_hashes[i] !=
                account::get_authentication_key(secondary_signer_addresses[i]);
        ensures forall i in 0..num_secondary_signers:
            account::exists_at(secondary_signer_addresses[i])
                && secondary_signer_public_key_hashes[i] ==
                    account::get_authentication_key(secondary_signer_addresses[i]);
    }
}
```


<a id="@Specification_1_script_prologue_collect_deposit"></a>

### Function `script_prologue_collect_deposit`


```move
module 0x1::transaction_validation {
    fun script_prologue_collect_deposit(sender: signer, txn_sequence_number: u64, txn_public_key: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, script_hash: vector<u8>, required_deposit: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```


<a id="@Specification_1_multi_agent_script_prologue"></a>

### Function `multi_agent_script_prologue`


```move
module 0x1::transaction_validation {
    fun multi_agent_script_prologue(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```

Aborts if length of public key hashed vector
not equal the number of singers.


```move
module 0x1::transaction_validation {
    pragma verify_duration_estimate = 120;
    let gas_payer = signer::address_of(sender);
    pragma verify = false;
    include PrologueCommonAbortsIf {
        gas_payer,
        txn_sequence_number,
        txn_authentication_key: txn_sender_public_key,
    };
    include MultiAgentPrologueCommonAbortsIf {
        secondary_signer_addresses,
        secondary_signer_public_key_hashes,
    };
}
```


<a id="@Specification_1_multi_agent_common_prologue"></a>

### Function `multi_agent_common_prologue`


```move
module 0x1::transaction_validation {
    fun multi_agent_common_prologue(secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>)
}
```



```move
module 0x1::transaction_validation {
    include MultiAgentPrologueCommonAbortsIf {
        secondary_signer_addresses,
        secondary_signer_public_key_hashes,
    };
}
```


<a id="@Specification_1_fee_payer_script_prologue"></a>

### Function `fee_payer_script_prologue`


```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, fee_payer_address: address, fee_payer_public_key_hash: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8)
}
```



```move
module 0x1::transaction_validation {
    pragma verify_duration_estimate = 120;
    aborts_if !features::spec_is_enabled(features::FEE_PAYER_ENABLED);
    let gas_payer = fee_payer_address;
    include PrologueCommonAbortsIf {
        gas_payer,
        txn_sequence_number,
        txn_authentication_key: txn_sender_public_key,
    };
    include MultiAgentPrologueCommonAbortsIf {
        secondary_signer_addresses,
        secondary_signer_public_key_hashes,
    };
    aborts_if !account::exists_at(gas_payer);
    aborts_if !(fee_payer_public_key_hash == account::get_authentication_key(gas_payer));
    aborts_if !features::spec_fee_payer_enabled();
}
```


<a id="@Specification_1_fee_payer_script_prologue_collect_deposit"></a>

### Function `fee_payer_script_prologue_collect_deposit`


```move
module 0x1::transaction_validation {
    fun fee_payer_script_prologue_collect_deposit(sender: signer, txn_sequence_number: u64, txn_sender_public_key: vector<u8>, secondary_signer_addresses: vector<address>, secondary_signer_public_key_hashes: vector<vector<u8>>, fee_payer_address: address, fee_payer_public_key_hash: vector<u8>, txn_gas_price: u64, txn_max_gas_units: u64, txn_expiration_time: u64, chain_id: u8, required_deposit: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```


<a id="@Specification_1_epilogue"></a>

### Function `epilogue`


```move
module 0x1::transaction_validation {
    fun epilogue(account: signer, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64)
}
```

Abort according to the conditions.
`AptosCoinCapabilities` and `CoinInfo` should exists.
Skip transaction_fee::burn_fee verification.


```move
module 0x1::transaction_validation {
    pragma verify = false;
    include EpilogueGasPayerAbortsIf { gas_payer: signer::address_of(account) };
}
```


<a id="@Specification_1_epilogue_return_deposit"></a>

### Function `epilogue_return_deposit`


```move
module 0x1::transaction_validation {
    fun epilogue_return_deposit(account: signer, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64, required_deposit: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```


<a id="@Specification_1_epilogue_gas_payer"></a>

### Function `epilogue_gas_payer`


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer(account: signer, gas_payer: address, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64)
}
```

Abort according to the conditions.
`AptosCoinCapabilities` and `CoinInfo` should exist.
Skip transaction_fee::burn_fee verification.


```move
module 0x1::transaction_validation {
    pragma verify = false;
    include EpilogueGasPayerAbortsIf;
}
```



<a id="0x1_transaction_validation_EpilogueGasPayerAbortsIf"></a>


```move
module 0x1::transaction_validation {
    schema EpilogueGasPayerAbortsIf {
        account: signer;
        gas_payer: address;
        storage_fee_refunded: u64;
        txn_gas_price: u64;
        txn_max_gas_units: u64;
        gas_units_remaining: u64;
        aborts_if !(txn_max_gas_units >= gas_units_remaining);
        let gas_used = txn_max_gas_units - gas_units_remaining;
        aborts_if !(txn_gas_price * gas_used <= MAX_U64);
        let transaction_fee_amount = txn_gas_price * gas_used;
        let addr = signer::address_of(account);
        let pre_account = global<account::Account>(addr);
        let post account = global<account::Account>(addr);
        aborts_if !exists<CoinStore<AptosCoin>>(gas_payer);
        aborts_if !exists<Account>(addr);
        aborts_if !(global<Account>(addr).sequence_number < MAX_U64);
        ensures account.sequence_number == pre_account.sequence_number + 1;
        let collect_fee_enabled = features::spec_is_enabled(features::COLLECT_AND_DISTRIBUTE_GAS_FEES);
        let collected_fees = global<CollectedFeesPerBlock>(@aptos_framework).amount;
        let aggr = collected_fees.value;
        let aggr_val = aggregator::spec_aggregator_get_val(aggr);
        let aggr_lim = aggregator::spec_get_limit(aggr);
    // This enforces ### high&#45;level&#45;req&#45;3
    [#high&#45;level&#45;req](high&#45;level requirement 3):
        aborts_if collect_fee_enabled && !exists<CollectedFeesPerBlock>(@aptos_framework);
        aborts_if collect_fee_enabled && transaction_fee_amount > 0 && aggr_val + transaction_fee_amount > aggr_lim;
        let amount_to_burn= if (collect_fee_enabled) {
            0
        } else {
            transaction_fee_amount - storage_fee_refunded
        };
        let apt_addr = type_info::type_of<AptosCoin>().account_address;
        let maybe_apt_supply = global<CoinInfo<AptosCoin>>(apt_addr).supply;
        let total_supply_enabled = option::spec_is_some(maybe_apt_supply);
        let apt_supply = option::spec_borrow(maybe_apt_supply);
        let apt_supply_value = optional_aggregator::optional_aggregator_value(apt_supply);
        let post post_maybe_apt_supply = global<CoinInfo<AptosCoin>>(apt_addr).supply;
        let post post_apt_supply = option::spec_borrow(post_maybe_apt_supply);
        let post post_apt_supply_value = optional_aggregator::optional_aggregator_value(post_apt_supply);
        aborts_if amount_to_burn > 0 && !exists<AptosCoinCapabilities>(@aptos_framework);
        aborts_if amount_to_burn > 0 && !exists<CoinInfo<AptosCoin>>(apt_addr);
        aborts_if amount_to_burn > 0 && total_supply_enabled && apt_supply_value < amount_to_burn;
        ensures total_supply_enabled ==> apt_supply_value - amount_to_burn == post_apt_supply_value;
        let amount_to_mint = if (collect_fee_enabled) {
            storage_fee_refunded
        } else {
            storage_fee_refunded - transaction_fee_amount
        };
        let total_supply = coin::supply<AptosCoin>;
        let post post_total_supply = coin::supply<AptosCoin>;
        aborts_if amount_to_mint > 0 && !exists<CoinStore<AptosCoin>>(addr);
        aborts_if amount_to_mint > 0 && !exists<AptosCoinMintCapability>(@aptos_framework);
        aborts_if amount_to_mint > 0 && total_supply + amount_to_mint > MAX_U128;
        ensures amount_to_mint > 0 ==> post_total_supply == total_supply + amount_to_mint;
        let aptos_addr = type_info::type_of<AptosCoin>().account_address;
        aborts_if (amount_to_mint != 0) && !exists<coin::CoinInfo<AptosCoin>>(aptos_addr);
        include coin::CoinAddAbortsIf<AptosCoin> { amount: amount_to_mint };
    }
}
```


<a id="@Specification_1_epilogue_gas_payer_return_deposit"></a>

### Function `epilogue_gas_payer_return_deposit`


```move
module 0x1::transaction_validation {
    fun epilogue_gas_payer_return_deposit(account: signer, gas_payer: address, storage_fee_refunded: u64, txn_gas_price: u64, txn_max_gas_units: u64, gas_units_remaining: u64, required_deposit: option::Option<u64>)
}
```



```move
module 0x1::transaction_validation {
    pragma verify = false;
}
```
