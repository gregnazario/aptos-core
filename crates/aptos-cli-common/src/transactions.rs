// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::config::ProfileOptions;
use crate::keys::{EncodingOptions, PrivateKeyInputOptions};
use crate::rest::RestOptions;
use crate::types::{CliError, CliTypedResult};
use crate::utils::chain_id;
use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_crypto::PrivateKey;
use aptos_rest_client::aptos_api_types::{
    DeleteModule, DeleteResource, DeleteTableItem, WriteModule, WriteResource, WriteSetChange,
    WriteTableItem,
};
use aptos_rest_client::Transaction;
use aptos_sdk::move_types::ident_str;
use aptos_sdk::move_types::language_storage::{ModuleId, TypeTag};
use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_sdk::types::LocalAccount;
use aptos_types::account_address::AccountAddress;
use aptos_types::transaction::authenticator::AuthenticationKey;
use aptos_types::transaction::{ScriptFunction, TransactionPayload};
use clap::Parser;
use serde::Serialize;

pub const DEFAULT_MAX_GAS: u64 = 1000;
pub const DEFAULT_GAS_UNIT_PRICE: u64 = 1;

/// Gas price options for manipulating how to prioritize transactions
#[derive(Debug, Eq, Parser, PartialEq)]
pub struct GasOptions {
    /// Amount to increase gas bid by for a transaction
    ///
    /// Defaults to 1 coin per gas unit
    #[clap(long, default_value_t = DEFAULT_GAS_UNIT_PRICE)]
    pub gas_unit_price: u64,
    /// Maximum gas to be used to send a transaction
    ///
    /// Defaults to 1000 gas units
    #[clap(long, default_value_t = DEFAULT_MAX_GAS)]
    pub max_gas: u64,
}

impl Default for GasOptions {
    fn default() -> Self {
        GasOptions {
            gas_unit_price: DEFAULT_GAS_UNIT_PRICE,
            max_gas: DEFAULT_MAX_GAS,
        }
    }
}

/// Common options for interacting with an account for a validator
#[derive(Debug, Default, Parser)]
pub struct TransactionOptions {
    #[clap(flatten)]
    pub private_key_options: PrivateKeyInputOptions,
    #[clap(flatten)]
    pub encoding_options: EncodingOptions,
    #[clap(flatten)]
    pub profile_options: ProfileOptions,
    #[clap(flatten)]
    pub rest_options: RestOptions,
    #[clap(flatten)]
    pub gas_options: GasOptions,
}

impl TransactionOptions {
    /// Retrieves the private key
    fn private_key(&self) -> CliTypedResult<Ed25519PrivateKey> {
        self.private_key_options.extract_private_key(
            self.encoding_options.encoding,
            &self.profile_options.profile,
        )
    }

    /// Builds a rest client
    fn rest_client(&self) -> CliTypedResult<aptos_rest_client::Client> {
        self.rest_options.client(&self.profile_options.profile)
    }

    /// Submits a script function based on module name and function inputs
    pub async fn submit_script_function(
        &self,
        address: AccountAddress,
        module: &'static str,
        function: &'static str,
        type_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> CliTypedResult<Transaction> {
        let txn = TransactionPayload::ScriptFunction(ScriptFunction::new(
            ModuleId::new(address, ident_str!(module).to_owned()),
            ident_str!(function).to_owned(),
            type_args,
            args,
        ));
        self.submit_transaction(txn).await
    }

    /// Submit a transaction
    pub async fn submit_transaction(
        &self,
        payload: TransactionPayload,
    ) -> CliTypedResult<Transaction> {
        let sender_key = self.private_key()?;
        let client = self.rest_client()?;

        // Get sender address
        let sender_address = AuthenticationKey::ed25519(&sender_key.public_key()).derived_address();
        let sender_address = AccountAddress::new(*sender_address);

        // Get sequence number for account
        let sequence_number = get_sequence_number(&client, sender_address).await?;

        // Sign and submit transaction
        let transaction_factory = TransactionFactory::new(chain_id(&client).await?)
            .with_gas_unit_price(self.gas_options.gas_unit_price)
            .with_max_gas_amount(self.gas_options.max_gas);
        let sender_account = &mut LocalAccount::new(sender_address, sender_key, sequence_number);
        let transaction =
            sender_account.sign_with_transaction_builder(transaction_factory.payload(payload));
        let response = client
            .submit_and_wait(&transaction)
            .await
            .map_err(|err| CliError::ApiError(err.to_string()))?;

        Ok(response.into_inner())
    }
}

/// Retrieves sequence number from the rest client
async fn get_sequence_number(
    client: &aptos_rest_client::Client,
    address: AccountAddress,
) -> CliTypedResult<u64> {
    let account_response = client
        .get_account(address)
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?;
    let account = account_response.inner();
    Ok(account.sequence_number)
}

/// A shortened transaction output
#[derive(Clone, Debug, Default, Serialize)]
pub struct TransactionSummary {
    changes: Vec<ChangeSummary>,
    gas_used: Option<u64>,
    success: bool,
    version: Option<u64>,
    vm_status: String,
}

impl From<Transaction> for TransactionSummary {
    fn from(transaction: Transaction) -> Self {
        let mut summary = TransactionSummary {
            success: transaction.success(),
            version: transaction.version(),
            vm_status: transaction.vm_status(),
            ..Default::default()
        };

        if let Ok(info) = transaction.transaction_info() {
            summary.gas_used = Some(info.gas_used.0);
            summary.changes = info
                .changes
                .iter()
                .map(|change| match change {
                    WriteSetChange::DeleteModule(DeleteModule { module, .. }) => ChangeSummary {
                        event: change.type_str(),
                        module: Some(module.to_string()),
                        ..Default::default()
                    },
                    WriteSetChange::DeleteResource(DeleteResource {
                        address, resource, ..
                    }) => ChangeSummary {
                        event: change.type_str(),
                        address: Some(*address.inner()),
                        resource: Some(resource.to_string()),
                        ..Default::default()
                    },
                    WriteSetChange::DeleteTableItem(DeleteTableItem { handle, key, .. }) => {
                        ChangeSummary {
                            event: change.type_str(),
                            handle: Some(handle.to_string()),
                            key: Some(key.to_string()),
                            ..Default::default()
                        }
                    }
                    WriteSetChange::WriteModule(WriteModule { address, .. }) => ChangeSummary {
                        event: change.type_str(),
                        address: Some(*address.inner()),
                        ..Default::default()
                    },
                    WriteSetChange::WriteResource(WriteResource { address, data, .. }) => {
                        ChangeSummary {
                            event: change.type_str(),
                            address: Some(*address.inner()),
                            resource: Some(data.typ.to_string()),
                            data: Some(serde_json::to_value(&data.data).unwrap_or_default()),
                            ..Default::default()
                        }
                    }
                    WriteSetChange::WriteTableItem(WriteTableItem {
                        handle, key, value, ..
                    }) => ChangeSummary {
                        event: change.type_str(),
                        handle: Some(handle.to_string()),
                        key: Some(key.to_string()),
                        value: Some(value.to_string()),
                        ..Default::default()
                    },
                })
                .collect();
        }

        summary
    }
}

/// A summary of a [`WriteSetChange`] for easy printing
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<AccountAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    event: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}
