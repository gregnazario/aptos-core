
<a id="0x1_transaction_context"></a>

# Module `0x1::transaction_context`



-  [Struct `AUID`](#0x1_transaction_context_AUID)
-  [Struct `EntryFunctionPayload`](#0x1_transaction_context_EntryFunctionPayload)
-  [Struct `MultisigPayload`](#0x1_transaction_context_MultisigPayload)
-  [Constants](#@Constants_0)
-  [Function `get_txn_hash`](#0x1_transaction_context_get_txn_hash)
-  [Function `get_transaction_hash`](#0x1_transaction_context_get_transaction_hash)
-  [Function `generate_unique_address`](#0x1_transaction_context_generate_unique_address)
-  [Function `generate_auid_address`](#0x1_transaction_context_generate_auid_address)
-  [Function `get_script_hash`](#0x1_transaction_context_get_script_hash)
-  [Function `generate_auid`](#0x1_transaction_context_generate_auid)
-  [Function `auid_address`](#0x1_transaction_context_auid_address)
-  [Function `sender`](#0x1_transaction_context_sender)
-  [Function `sender_internal`](#0x1_transaction_context_sender_internal)
-  [Function `secondary_signers`](#0x1_transaction_context_secondary_signers)
-  [Function `secondary_signers_internal`](#0x1_transaction_context_secondary_signers_internal)
-  [Function `gas_payer`](#0x1_transaction_context_gas_payer)
-  [Function `gas_payer_internal`](#0x1_transaction_context_gas_payer_internal)
-  [Function `max_gas_amount`](#0x1_transaction_context_max_gas_amount)
-  [Function `max_gas_amount_internal`](#0x1_transaction_context_max_gas_amount_internal)
-  [Function `gas_unit_price`](#0x1_transaction_context_gas_unit_price)
-  [Function `gas_unit_price_internal`](#0x1_transaction_context_gas_unit_price_internal)
-  [Function `chain_id`](#0x1_transaction_context_chain_id)
-  [Function `chain_id_internal`](#0x1_transaction_context_chain_id_internal)
-  [Function `entry_function_payload`](#0x1_transaction_context_entry_function_payload)
-  [Function `entry_function_payload_internal`](#0x1_transaction_context_entry_function_payload_internal)
-  [Function `account_address`](#0x1_transaction_context_account_address)
-  [Function `module_name`](#0x1_transaction_context_module_name)
-  [Function `function_name`](#0x1_transaction_context_function_name)
-  [Function `type_arg_names`](#0x1_transaction_context_type_arg_names)
-  [Function `args`](#0x1_transaction_context_args)
-  [Function `multisig_payload`](#0x1_transaction_context_multisig_payload)
-  [Function `multisig_payload_internal`](#0x1_transaction_context_multisig_payload_internal)
-  [Function `multisig_address`](#0x1_transaction_context_multisig_address)
-  [Function `inner_entry_function_payload`](#0x1_transaction_context_inner_entry_function_payload)
-  [Specification](#@Specification_1)
    -  [Function `get_txn_hash`](#@Specification_1_get_txn_hash)
    -  [Function `get_transaction_hash`](#@Specification_1_get_transaction_hash)
    -  [Function `generate_unique_address`](#@Specification_1_generate_unique_address)
    -  [Function `generate_auid_address`](#@Specification_1_generate_auid_address)
    -  [Function `get_script_hash`](#@Specification_1_get_script_hash)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `auid_address`](#@Specification_1_auid_address)
    -  [Function `sender_internal`](#@Specification_1_sender_internal)
    -  [Function `secondary_signers_internal`](#@Specification_1_secondary_signers_internal)
    -  [Function `gas_payer_internal`](#@Specification_1_gas_payer_internal)
    -  [Function `max_gas_amount_internal`](#@Specification_1_max_gas_amount_internal)
    -  [Function `gas_unit_price_internal`](#@Specification_1_gas_unit_price_internal)
    -  [Function `chain_id_internal`](#@Specification_1_chain_id_internal)
    -  [Function `entry_function_payload_internal`](#@Specification_1_entry_function_payload_internal)
    -  [Function `multisig_payload_internal`](#@Specification_1_multisig_payload_internal)


```move
module 0x1::transaction_context {
    use 0x1::error;
    use 0x1::features;
    use 0x1::option;
    use 0x1::string;
}
```


<a id="0x1_transaction_context_AUID"></a>

## Struct `AUID`

A wrapper denoting aptos unique identifer (AUID)
for storing an address


```move
module 0x1::transaction_context {
    struct AUID has drop, store
}
```


##### Fields


<dl>
<dt>
`unique_address: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_transaction_context_EntryFunctionPayload"></a>

## Struct `EntryFunctionPayload`

Represents the entry function payload.


```move
module 0x1::transaction_context {
    struct EntryFunctionPayload has copy, drop
}
```


##### Fields


<dl>
<dt>
`account_address: address`
</dt>
<dd>

</dd>
<dt>
`module_name: string::String`
</dt>
<dd>

</dd>
<dt>
`function_name: string::String`
</dt>
<dd>

</dd>
<dt>
`ty_args_names: vector<string::String>`
</dt>
<dd>

</dd>
<dt>
`args: vector<vector<u8>>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_transaction_context_MultisigPayload"></a>

## Struct `MultisigPayload`

Represents the multisig payload.


```move
module 0x1::transaction_context {
    struct MultisigPayload has copy, drop
}
```


##### Fields


<dl>
<dt>
`multisig_address: address`
</dt>
<dd>

</dd>
<dt>
`entry_function_payload: option::Option<transaction_context::EntryFunctionPayload>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_transaction_context_ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED"></a>

The transaction context extension feature is not enabled.


```move
module 0x1::transaction_context {
    const ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED: u64 = 2;
}
```


<a id="0x1_transaction_context_ETRANSACTION_CONTEXT_NOT_AVAILABLE"></a>

Transaction context is only available in the user transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    const ETRANSACTION_CONTEXT_NOT_AVAILABLE: u64 = 1;
}
```


<a id="0x1_transaction_context_get_txn_hash"></a>

## Function `get_txn_hash`

Returns the transaction hash of the current transaction.


```move
module 0x1::transaction_context {
    fun get_txn_hash(): vector<u8>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun get_txn_hash(): vector<u8>;
}
```


<a id="0x1_transaction_context_get_transaction_hash"></a>

## Function `get_transaction_hash`

Returns the transaction hash of the current transaction.
Internally calls the private function `get_txn_hash`.
This function is created for to feature gate the `get_txn_hash` function.


```move
module 0x1::transaction_context {
    public fun get_transaction_hash(): vector<u8>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun get_transaction_hash(): vector<u8> {
        get_txn_hash()
    }
}
```


<a id="0x1_transaction_context_generate_unique_address"></a>

## Function `generate_unique_address`

Returns a universally unique identifier (of type address) generated
by hashing the transaction hash of this transaction and a sequence number
specific to this transaction. This function can be called any
number of times inside a single transaction. Each such call increments
the sequence number and generates a new unique address.
Uses Scheme in types/src/transaction/authenticator.rs for domain separation
from other ways of generating unique addresses.


```move
module 0x1::transaction_context {
    fun generate_unique_address(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun generate_unique_address(): address;
}
```


<a id="0x1_transaction_context_generate_auid_address"></a>

## Function `generate_auid_address`

Returns a aptos unique identifier. Internally calls
the private function `generate_unique_address`. This function is
created for to feature gate the `generate_unique_address` function.


```move
module 0x1::transaction_context {
    public fun generate_auid_address(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun generate_auid_address(): address {
        generate_unique_address()
    }
}
```


<a id="0x1_transaction_context_get_script_hash"></a>

## Function `get_script_hash`

Returns the script hash of the current entry function.


```move
module 0x1::transaction_context {
    public fun get_script_hash(): vector<u8>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public native fun get_script_hash(): vector<u8>;
}
```


<a id="0x1_transaction_context_generate_auid"></a>

## Function `generate_auid`

This method runs `generate_unique_address` native function and returns
the generated unique address wrapped in the AUID class.


```move
module 0x1::transaction_context {
    public fun generate_auid(): transaction_context::AUID
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun generate_auid(): AUID {
        return AUID {
            unique_address: generate_unique_address()
        }
    }
}
```


<a id="0x1_transaction_context_auid_address"></a>

## Function `auid_address`

Returns the unique address wrapped in the given AUID struct.


```move
module 0x1::transaction_context {
    public fun auid_address(auid: &transaction_context::AUID): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun auid_address(auid: &AUID): address {
        auid.unique_address
    }
}
```


<a id="0x1_transaction_context_sender"></a>

## Function `sender`

Returns the sender&apos;s address for the current transaction.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun sender(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun sender(): address {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        sender_internal()
    }
}
```


<a id="0x1_transaction_context_sender_internal"></a>

## Function `sender_internal`



```move
module 0x1::transaction_context {
    fun sender_internal(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun sender_internal(): address;
}
```


<a id="0x1_transaction_context_secondary_signers"></a>

## Function `secondary_signers`

Returns the list of the secondary signers for the current transaction.
If the current transaction has no secondary signers, this function returns an empty vector.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun secondary_signers(): vector<address>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun secondary_signers(): vector<address> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        secondary_signers_internal()
    }
}
```


<a id="0x1_transaction_context_secondary_signers_internal"></a>

## Function `secondary_signers_internal`



```move
module 0x1::transaction_context {
    fun secondary_signers_internal(): vector<address>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun secondary_signers_internal(): vector<address>;
}
```


<a id="0x1_transaction_context_gas_payer"></a>

## Function `gas_payer`

Returns the gas payer address for the current transaction.
It is either the sender&apos;s address if no separate gas fee payer is specified for the current transaction,
or the address of the separate gas fee payer if one is specified.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun gas_payer(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun gas_payer(): address {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        gas_payer_internal()
    }
}
```


<a id="0x1_transaction_context_gas_payer_internal"></a>

## Function `gas_payer_internal`



```move
module 0x1::transaction_context {
    fun gas_payer_internal(): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun gas_payer_internal(): address;
}
```


<a id="0x1_transaction_context_max_gas_amount"></a>

## Function `max_gas_amount`

Returns the max gas amount in units which is specified for the current transaction.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun max_gas_amount(): u64
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun max_gas_amount(): u64 {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        max_gas_amount_internal()
    }
}
```


<a id="0x1_transaction_context_max_gas_amount_internal"></a>

## Function `max_gas_amount_internal`



```move
module 0x1::transaction_context {
    fun max_gas_amount_internal(): u64
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun max_gas_amount_internal(): u64;
}
```


<a id="0x1_transaction_context_gas_unit_price"></a>

## Function `gas_unit_price`

Returns the gas unit price in Octas which is specified for the current transaction.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun gas_unit_price(): u64
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun gas_unit_price(): u64 {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        gas_unit_price_internal()
    }
}
```


<a id="0x1_transaction_context_gas_unit_price_internal"></a>

## Function `gas_unit_price_internal`



```move
module 0x1::transaction_context {
    fun gas_unit_price_internal(): u64
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun gas_unit_price_internal(): u64;
}
```


<a id="0x1_transaction_context_chain_id"></a>

## Function `chain_id`

Returns the chain ID specified for the current transaction.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun chain_id(): u8
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun chain_id(): u8 {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        chain_id_internal()
    }
}
```


<a id="0x1_transaction_context_chain_id_internal"></a>

## Function `chain_id_internal`



```move
module 0x1::transaction_context {
    fun chain_id_internal(): u8
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun chain_id_internal(): u8;
}
```


<a id="0x1_transaction_context_entry_function_payload"></a>

## Function `entry_function_payload`

Returns the entry function payload if the current transaction has such a payload. Otherwise, return `None`.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun entry_function_payload(): option::Option<transaction_context::EntryFunctionPayload>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun entry_function_payload(): Option<EntryFunctionPayload> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        entry_function_payload_internal()
    }
}
```


<a id="0x1_transaction_context_entry_function_payload_internal"></a>

## Function `entry_function_payload_internal`



```move
module 0x1::transaction_context {
    fun entry_function_payload_internal(): option::Option<transaction_context::EntryFunctionPayload>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun entry_function_payload_internal(): Option<EntryFunctionPayload>;
}
```


<a id="0x1_transaction_context_account_address"></a>

## Function `account_address`

Returns the account address of the entry function payload.


```move
module 0x1::transaction_context {
    public fun account_address(payload: &transaction_context::EntryFunctionPayload): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun account_address(payload: &EntryFunctionPayload): address {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.account_address
    }
}
```


<a id="0x1_transaction_context_module_name"></a>

## Function `module_name`

Returns the module name of the entry function payload.


```move
module 0x1::transaction_context {
    public fun module_name(payload: &transaction_context::EntryFunctionPayload): string::String
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun module_name(payload: &EntryFunctionPayload): String {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.module_name
    }
}
```


<a id="0x1_transaction_context_function_name"></a>

## Function `function_name`

Returns the function name of the entry function payload.


```move
module 0x1::transaction_context {
    public fun function_name(payload: &transaction_context::EntryFunctionPayload): string::String
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun function_name(payload: &EntryFunctionPayload): String {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.function_name
    }
}
```


<a id="0x1_transaction_context_type_arg_names"></a>

## Function `type_arg_names`

Returns the type arguments names of the entry function payload.


```move
module 0x1::transaction_context {
    public fun type_arg_names(payload: &transaction_context::EntryFunctionPayload): vector<string::String>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun type_arg_names(payload: &EntryFunctionPayload): vector<String> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.ty_args_names
    }
}
```


<a id="0x1_transaction_context_args"></a>

## Function `args`

Returns the arguments of the entry function payload.


```move
module 0x1::transaction_context {
    public fun args(payload: &transaction_context::EntryFunctionPayload): vector<vector<u8>>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun args(payload: &EntryFunctionPayload): vector<vector<u8>> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.args
    }
}
```


<a id="0x1_transaction_context_multisig_payload"></a>

## Function `multisig_payload`

Returns the multisig payload if the current transaction has such a payload. Otherwise, return `None`.
This function aborts if called outside of the transaction prologue, execution, or epilogue phases.


```move
module 0x1::transaction_context {
    public fun multisig_payload(): option::Option<transaction_context::MultisigPayload>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun multisig_payload(): Option<MultisigPayload> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        multisig_payload_internal()
    }
}
```


<a id="0x1_transaction_context_multisig_payload_internal"></a>

## Function `multisig_payload_internal`



```move
module 0x1::transaction_context {
    fun multisig_payload_internal(): option::Option<transaction_context::MultisigPayload>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    native fun multisig_payload_internal(): Option<MultisigPayload>;
}
```


<a id="0x1_transaction_context_multisig_address"></a>

## Function `multisig_address`

Returns the multisig account address of the multisig payload.


```move
module 0x1::transaction_context {
    public fun multisig_address(payload: &transaction_context::MultisigPayload): address
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun multisig_address(payload: &MultisigPayload): address {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.multisig_address
    }
}
```


<a id="0x1_transaction_context_inner_entry_function_payload"></a>

## Function `inner_entry_function_payload`

Returns the inner entry function payload of the multisig payload.


```move
module 0x1::transaction_context {
    public fun inner_entry_function_payload(payload: &transaction_context::MultisigPayload): option::Option<transaction_context::EntryFunctionPayload>
}
```


##### Implementation


```move
module 0x1::transaction_context {
    public fun inner_entry_function_payload(payload: &MultisigPayload): Option<EntryFunctionPayload> {
        assert!(features::transaction_context_extension_enabled(), error::invalid_state(ETRANSACTION_CONTEXT_EXTENSION_NOT_ENABLED));
        payload.entry_function_payload
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_get_txn_hash"></a>

### Function `get_txn_hash`


```move
module 0x1::transaction_context {
    fun get_txn_hash(): vector<u8>
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
    aborts_if [abstract] false;
    ensures result == spec_get_txn_hash();
}
```



<a id="0x1_transaction_context_spec_get_txn_hash"></a>


```move
module 0x1::transaction_context {
    fun spec_get_txn_hash(): vector<u8>;
}
```


<a id="@Specification_1_get_transaction_hash"></a>

### Function `get_transaction_hash`


```move
module 0x1::transaction_context {
    public fun get_transaction_hash(): vector<u8>
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
    aborts_if [abstract] false;
    ensures result == spec_get_txn_hash();
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures [abstract] len(result) == 32;
}
```


<a id="@Specification_1_generate_unique_address"></a>

### Function `generate_unique_address`


```move
module 0x1::transaction_context {
    fun generate_unique_address(): address
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
    ensures [abstract] result == spec_generate_unique_address();
}
```



<a id="0x1_transaction_context_spec_generate_unique_address"></a>


```move
module 0x1::transaction_context {
    fun spec_generate_unique_address(): address;
}
```


<a id="@Specification_1_generate_auid_address"></a>

### Function `generate_auid_address`


```move
module 0x1::transaction_context {
    public fun generate_auid_address(): address
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures [abstract] result == spec_generate_unique_address();
}
```


<a id="@Specification_1_get_script_hash"></a>

### Function `get_script_hash`


```move
module 0x1::transaction_context {
    public fun get_script_hash(): vector<u8>
}
```




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>Fetching the transaction hash should return a vector with 32 bytes.</td>
<td>Medium</td>
<td>The get_transaction_hash function calls the native function get_txn_hash, which fetches the NativeTransactionContext struct and returns the txn_hash field.</td>
<td>Audited that the native function returns the txn hash, whose size is 32 bytes. This has been modeled as the abstract postcondition that the returned vector is of length 32. Formally verified via [#high&#45;level&#45;req&#45;1](get_txn_hash).</td>
</tr>

<tr>
<td>2</td>
<td>Fetching the unique address should never abort.</td>
<td>Low</td>
<td>The function auid_address returns the unique address from a supplied AUID resource.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](auid_address).</td>
</tr>

<tr>
<td>3</td>
<td>Generating the unique address should return a vector with 32 bytes.</td>
<td>Medium</td>
<td>The generate_auid_address function checks calls the native function generate_unique_address which fetches the NativeTransactionContext struct, increments the auid_counter by one, and then creates a new authentication key from a preimage, which is then returned.</td>
<td>Audited that the native function returns an address, and the length of an address is 32 bytes. This has been modeled as the abstract postcondition that the returned vector is of length 32. Formally verified via [#high&#45;level&#45;req&#45;3](generate_auid_address).</td>
</tr>

<tr>
<td>4</td>
<td>Fetching the script hash of the current entry function should never fail and should return a vector with 32 bytes if the transaction payload is a script, otherwise an empty vector.</td>
<td>Low</td>
<td>The native function get_script_hash returns the NativeTransactionContext.script_hash field.</td>
<td>Audited that the native function holds the required property. This has been modeled as the abstract spec. Formally verified via [#high&#45;level&#45;req&#45;4](get_script_hash).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::transaction_context {
    pragma opaque;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    aborts_if [abstract] false;
    ensures [abstract] result == spec_get_script_hash();
    ensures [abstract] len(result) == 32;
}
```



<a id="0x1_transaction_context_spec_get_script_hash"></a>


```move
module 0x1::transaction_context {
    fun spec_get_script_hash(): vector<u8>;
}
```


<a id="@Specification_1_auid_address"></a>

### Function `auid_address`


```move
module 0x1::transaction_context {
    public fun auid_address(auid: &transaction_context::AUID): address
}
```



```move
module 0x1::transaction_context {
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    aborts_if false;
}
```


<a id="@Specification_1_sender_internal"></a>

### Function `sender_internal`


```move
module 0x1::transaction_context {
    fun sender_internal(): address
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_secondary_signers_internal"></a>

### Function `secondary_signers_internal`


```move
module 0x1::transaction_context {
    fun secondary_signers_internal(): vector<address>
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_gas_payer_internal"></a>

### Function `gas_payer_internal`


```move
module 0x1::transaction_context {
    fun gas_payer_internal(): address
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_max_gas_amount_internal"></a>

### Function `max_gas_amount_internal`


```move
module 0x1::transaction_context {
    fun max_gas_amount_internal(): u64
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_gas_unit_price_internal"></a>

### Function `gas_unit_price_internal`


```move
module 0x1::transaction_context {
    fun gas_unit_price_internal(): u64
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_chain_id_internal"></a>

### Function `chain_id_internal`


```move
module 0x1::transaction_context {
    fun chain_id_internal(): u8
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_entry_function_payload_internal"></a>

### Function `entry_function_payload_internal`


```move
module 0x1::transaction_context {
    fun entry_function_payload_internal(): option::Option<transaction_context::EntryFunctionPayload>
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```


<a id="@Specification_1_multisig_payload_internal"></a>

### Function `multisig_payload_internal`


```move
module 0x1::transaction_context {
    fun multisig_payload_internal(): option::Option<transaction_context::MultisigPayload>
}
```



```move
module 0x1::transaction_context {
    pragma opaque;
}
```
