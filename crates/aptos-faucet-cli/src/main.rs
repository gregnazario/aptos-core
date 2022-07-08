// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos::{
    common::{
        types::{account_address_from_public_key, EncodingType, RngArgs},
        utils::write_to_user_only_file,
    },
    genesis::git::to_yaml,
};
use aptos_config::keys::ConfigKey;
use aptos_crypto::{ed25519::Ed25519PrivateKey, PrivateKey};
use aptos_faucet::{mint, mint::MintParams, Service};
use aptos_sdk::types::{
    account_address::AccountAddress, account_config::aptos_root_address, chain_id::ChainId,
    LocalAccount,
};
use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

#[tokio::main]
async fn main() {
    aptos_logger::Logger::new().init();
    let args: FaucetArgs = FaucetArgs::parse();
    args.execute().await
}

#[derive(Debug, Parser)]
#[clap(name = "aptos-faucet-cli", author, version, propagate_version = true)]
pub enum FaucetArgs {
    Fund(FundArgs),
    CreateAndFund(CreateAndFundArgs),
}

impl FaucetArgs {
    async fn execute(self) {
        match self {
            FaucetArgs::Fund(inner) => inner.execute().await,
            FaucetArgs::CreateAndFund(inner) => inner.execute().await,
        };
    }
}

/// Fund accounts that already exist
#[derive(Debug, Parser)]
pub struct FundArgs {
    #[clap(flatten)]
    pub faucet_args: CommonFaucetArgs,
    /// Addresses of accounts to mint coins to, split by commas
    #[clap(long, group = "account-group")]
    pub accounts: Option<String>,
    /// File of addresses of account to mint coins to.  Formatted in YAML
    #[clap(long, group = "account-group", parse(from_os_str))]
    pub account_file: Option<PathBuf>,
}

impl FundArgs {
    async fn execute(self) {
        let mint_account_address = self
            .faucet_args
            .mint_account_address
            .unwrap_or_else(aptos_root_address);
        let mint_key = if let Some(ref key) = self.faucet_args.mint_key {
            key.private_key()
        } else {
            EncodingType::BCS
                .load_key::<Ed25519PrivateKey>(
                    "mint key",
                    self.faucet_args.mint_key_file_path.as_path(),
                )
                .unwrap()
        };
        let faucet_account = LocalAccount::new(mint_account_address, mint_key, 0);
        let service = Service::new(
            self.faucet_args.server_url,
            self.faucet_args.chain_id,
            faucet_account,
            None,
        );

        let accounts: HashSet<AccountAddress> = if let Some(accounts) = self.accounts {
            accounts
                .trim()
                .split(',')
                .map(process_account_address)
                .collect()
        } else if let Some(path) = self.account_file {
            let strings: Vec<String> =
                serde_yaml::from_str(&std::fs::read_to_string(path.as_path()).unwrap()).unwrap();
            strings
                .into_iter()
                .map(|str| process_account_address(&str))
                .collect()
        } else {
            panic!("Either --accounts or --account-file must be specified");
        };

        // Iterate through accounts to mint the tokens
        for account in accounts {
            let response = mint::process(
                &service,
                MintParams {
                    amount: self.faucet_args.amount,
                    auth_key: None,
                    address: Some(account.to_hex_literal()),
                    pub_key: None,
                    return_txns: None,
                },
            )
            .await;
            match response {
                Ok(response) => println!(
                    "SUCCESS: Account: {} Response: {:?}",
                    account.to_hex_literal(),
                    response
                ),
                Err(response) => println!(
                    "FAILURE: Account: {} Response: {:?}",
                    account.to_hex_literal(),
                    response
                ),
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct CommonFaucetArgs {
    /// Aptos fullnode/validator server URL
    #[clap(long, default_value = "https://fullnode.devnet.aptoslabs.com/")]
    pub server_url: String,
    /// Path to the private key for creating test account and minting coins.
    /// To keep Testnet simple, we used one private key for aptos root account
    /// To manually generate a keypair, use generate-key:
    /// `cargo run -p generate-keypair -- -o <output_file_path>`
    #[clap(long, default_value = "/opt/aptos/etc/mint.key", parse(from_os_str))]
    pub mint_key_file_path: PathBuf,
    /// Ed25519PrivateKey for minting coins
    #[clap(long, parse(try_from_str = ConfigKey::from_encoded_string))]
    pub mint_key: Option<ConfigKey<Ed25519PrivateKey>>,
    /// Address of the account to send transactions from.
    /// On Testnet, for example, this is a550c18.
    /// If not present, the mint key's address is used
    #[clap(long, parse(try_from_str = AccountAddress::from_hex_literal))]
    pub mint_account_address: Option<AccountAddress>,
    /// Chain ID of the network this client is connecting to.
    /// For mainnet: "MAINNET" or 1, testnet: "TESTNET" or 2, devnet: "DEVNET" or 3,
    /// local swarm: "TESTING" or 4
    /// Note: Chain ID of 0 is not allowed; Use number if chain id is not predefined.
    #[clap(long, default_value = "3")]
    pub chain_id: ChainId,
    /// Amount of coins to mint
    #[clap(long)]
    pub amount: u64,
}

/// Generate private keys for accounts, and fund the accounts
#[derive(Debug, Parser)]
pub struct CreateAndFundArgs {
    #[clap(flatten)]
    pub faucet_args: CommonFaucetArgs,
    #[clap(long)]
    pub num_accounts: u64,
    #[clap(flatten)]
    pub rng_args: RngArgs,
    #[clap(long)]
    seed: Option<String>,
    #[clap(long, parse(from_os_str))]
    output_file: PathBuf,
}

impl CreateAndFundArgs {
    async fn execute(self) {
        let mut keygen = if let Some(ref seed) = self.seed {
            aptos_keygen::KeyGen::from_seed(parse_seed(seed))
        } else {
            aptos_keygen::KeyGen::from_os_rng()
        };

        let mut addresses = Vec::new();
        let mut accounts_to_keys = BTreeMap::new();
        for _ in 0..self.num_accounts {
            let private_key = keygen.generate_ed25519_private_key();
            let address = account_address_from_public_key(&private_key.public_key());
            addresses.push(address);
            accounts_to_keys.insert(address, private_key);
        }

        write_to_user_only_file(
            self.output_file.as_path(),
            "Keys file",
            to_yaml(&accounts_to_keys).unwrap().as_bytes(),
        )
        .unwrap();

        FundArgs {
            faucet_args: self.faucet_args,
            accounts: Some(
                addresses
                    .iter()
                    .map(|address| address.to_string())
                    .join(","),
            ),
            account_file: None,
        }
        .execute()
        .await;
    }
}

/// Allow 0x to be in front of addresses
fn process_account_address(str: &str) -> AccountAddress {
    let str = str.trim();
    if let Ok(address) = AccountAddress::from_hex_literal(str) {
        address
    } else if let Ok(address) = AccountAddress::from_str(str) {
        address
    } else {
        panic!("Account address is in an invalid format {}", str)
    }
}

/// Parses a random seed
pub fn parse_seed(str: &str) -> [u8; 32] {
    let mut array = [0; 32];
    hex::decode_to_slice(str, &mut array).unwrap();
    array
}
