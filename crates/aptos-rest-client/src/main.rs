// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

extern crate core;

use aptos_rest_client::types::EventHandleGUID;
use aptos_rest_client::USER_AGENT;
use aptos_types::account_address::AccountAddress;
use aptos_types::contract_event::ContractEvent;
use aptos_types::event::EventKey;
use aptos_types::transaction::authenticator::AuthenticationKey;
use bytes::Bytes;
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

#[tokio::main]
async fn main() {
    // Run the corresponding tools
    RestTool::parse().execute().await;
}

#[derive(Parser)]
pub enum RestTool {
    Account(AccountTool),
    AccountEvents(AccountEventsTool),
    Resources(ResourcesTool),
    Resource(ResourceTool),
    Modules(ModulesTool),
    Module(ModuleTool),
    Events(EventsTool),
}

impl RestTool {
    async fn execute(self) {
        match self {
            RestTool::Account(tool) => tool.execute().await,
            RestTool::Resources(tool) => tool.execute().await,
            RestTool::Resource(tool) => tool.execute().await,
            RestTool::Module(tool) => tool.execute().await,
            RestTool::Modules(tool) => tool.execute().await,
            RestTool::AccountEvents(tool) => tool.execute().await,
            RestTool::Events(tool) => tool.execute().await,
        }
    }
}

#[derive(Parser)]
pub struct AccountTool {
    #[clap(long)]
    account: AccountAddressWrapper,
}

impl AccountTool {
    async fn execute(self) {
        let client = rest_client();
        let url = url()
            .join(&format!(
                "/accounts/{}",
                self.account.account_address.to_hex_literal()
            ))
            .unwrap();

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("bcs: {:?}", bcs);
    }
}

#[derive(Parser)]
pub struct ResourcesTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(flatten)]
    page: Paging,
}

impl ResourcesTool {
    async fn execute(self) {
        let client = rest_client();
        let mut url = url()
            .join(&format!(
                "v1/accounts/{}/resources",
                self.account.account_address.to_hex_literal()
            ))
            .unwrap();
        self.page.apply_page(&mut url);

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("bcs: {:?}", bcs);
    }
}

#[derive(Parser)]
pub struct ResourceTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(long)]
    resource: String,
}

impl ResourceTool {
    async fn execute(self) {
        let client = rest_client();
        let url = url()
            .join(&format!(
                "v1/accounts/{}/resource/{}",
                self.account.account_address.to_hex_literal(),
                self.resource
            ))
            .unwrap();

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );

        StructType::from_move_type(&self.resource).as_str(&bcs);
    }
}

#[derive(Parser)]
pub struct ModulesTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(flatten)]
    page: Paging,
}

impl ModulesTool {
    async fn execute(self) {
        let client = rest_client();
        let mut url = url()
            .join(&format!(
                "v1/accounts/{}/modules",
                self.account.account_address.to_hex_literal()
            ))
            .unwrap();
        self.page.apply_page(&mut url);

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("bcs: {:?}", bcs);
    }
}

#[derive(Parser)]
pub struct ModuleTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(long)]
    module: String,
}

impl ModuleTool {
    async fn execute(self) {
        let client = rest_client();
        let url = url()
            .join(&format!(
                "v1/accounts/{}/module/{}",
                self.account.account_address.to_hex_literal(),
                self.module
            ))
            .unwrap();

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("BCS: {:?}", bcs);
        StructType::from_move_type(&self.module).as_str(&bcs);
    }
}

#[derive(Parser)]
pub struct AccountEventsTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(long)]
    struct_name: String,
    #[clap(long)]
    field_name: String,
    #[clap(flatten)]
    page: Paging,
    #[clap(long)]
    event_type: String,
}

impl AccountEventsTool {
    async fn execute(self) {
        let client = rest_client();
        let mut url = url()
            .join(&format!(
                "v1/accounts/{}/events/{}/{}",
                self.account.account_address, self.struct_name, self.field_name
            ))
            .unwrap();
        self.page.apply_page(&mut url);

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("BCS: {:?}", bcs);

        StructType::from_move_type(&self.event_type).as_str(&bcs);
    }
}

#[derive(Parser)]
pub struct EventsTool {
    #[clap(long)]
    account: AccountAddressWrapper,
    #[clap(long)]
    num: u64,
    #[clap(flatten)]
    page: Paging,
    #[clap(long)]
    event_type: String,
}

impl EventsTool {
    async fn execute(self) {
        let client = rest_client();
        let event_key = EventKey::new(self.num, self.account.account_address);
        let mut url = url().join(&format!("v1/events/{}", event_key)).unwrap();
        self.page.apply_page(&mut url);

        let json = get(&client, url.clone()).await.unwrap();
        let bcs = get_bcs(&client, url).await.unwrap();
        println!(
            "JSON: {}",
            serde_json::from_slice::<serde_json::Value>(&json).unwrap()
        );
        println!("BCS: {:?}", bcs);

        StructType::from_move_type(&self.event_type).as_str(&bcs);
    }
}

async fn get(client: &Client, url: Url) -> reqwest::Result<Bytes> {
    println!("GET JSON: {}", url);
    client.get(url).send().await.unwrap().bytes().await
}

async fn get_bcs(client: &Client, url: Url) -> reqwest::Result<Bytes> {
    println!("GET BCS: {}", url);
    client
        .get(url)
        .header("Accept", "application/x-bcs")
        .send()
        .await
        .unwrap()
        .bytes()
        .await
}

#[derive(Parser)]
pub struct Paging {
    #[clap(long)]
    start: Option<u64>,
    #[clap(long)]
    limit: Option<u16>,
}

impl Paging {
    pub fn apply_page(&self, url: &mut Url) {
        let mut query_pairs = url.query_pairs_mut();
        if let Some(start) = self.start {
            query_pairs.append_pair("start", &start.to_string());
        }
        if let Some(limit) = self.limit {
            query_pairs.append_pair("limit", &limit.to_string());
        }
        query_pairs.finish();
    }
}

#[derive(clap::ArgEnum, Clone)]
pub enum StructType {
    Account,
    Generator,
    NewBlockEvent,
}

impl StructType {
    pub fn from_move_type(move_type: &str) -> Self {
        match move_type {
            "0x1::account::Account" => Self::Account,
            "0x1::guid::Generator" => Self::Generator,
            "0x1::block::NewBlockEvent" => Self::NewBlockEvent,
            _ => panic!("Unsupported type {}", move_type),
        }
    }

    pub fn as_str(&self, bytes: &[u8]) {
        match self {
            StructType::Account => {
                #[derive(Debug, Deserialize)]
                struct Inner {
                    _authentication_key: Vec<u8>,
                    _sequence_number: u64,
                    _event_handle: EventHandle,
                }

                println!("Bytes: {:?}", bytes);
                println!("{:?}", bcs::from_bytes::<Inner>(bytes));
            }
            StructType::Generator => {
                #[derive(Debug, Deserialize)]
                struct Inner {
                    _counter: u64,
                }
                println!("Bytes: {:?}", bytes);
                println!("{:?}", bcs::from_bytes::<Inner>(bytes));
            }
            StructType::NewBlockEvent => {
                #[derive(Debug, Deserialize)]
                struct Inner {
                    epoch: u64,
                    round: u64,
                    height: u64,
                    previous_block_votes_bitvec: Vec<u8>,
                    proposer: AccountAddress,
                    failed_proposer_indices: Vec<u64>,
                    time_microseconds: u64,
                }

                println!("Bytes: {:?}", bytes);
                println!("{:?}", bcs::from_bytes::<Vec<EventWithVersion>>(bytes));
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct EventWithVersion {
    version: u64,
    event: ContractEvent,
}

#[derive(Debug, Deserialize)]
struct EventHandle {
    counter: u64,
    guid: GUID,
}

#[derive(Debug, Deserialize)]
struct GUID {
    id: ID,
}

#[derive(Debug, Deserialize)]
struct ID {
    creation_num: u64,
    addr: AccountAddress,
}

fn url() -> Url {
    Url::parse("http://localhost:8080/").unwrap()
}

fn rest_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .build()
        .unwrap()
}

/// A wrapper around `AccountAddress` to be more flexible from strings than AccountAddress
#[derive(Clone, Copy, Debug)]
pub struct AccountAddressWrapper {
    pub account_address: AccountAddress,
}

impl FromStr for AccountAddressWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountAddressWrapper {
            account_address: load_account_arg(s)?,
        })
    }
}

/// Loads an account arg and allows for naming based on profiles
pub fn load_account_arg(str: &str) -> anyhow::Result<AccountAddress> {
    if str.starts_with("0x") {
        Ok(AccountAddress::from_hex_literal(str)?)
    } else {
        Ok(AccountAddress::from_str(str)?)
    }
}
