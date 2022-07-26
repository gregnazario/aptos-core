// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! Aptos Rosetta CLI
//!
//! Why have an Aptos version of the Rosetta CLI?
//!
//! The Rosetta CLI doesn't build on my Mac easily and I just wanted something simple to test out
//! the POST requests
//!
//! Why have a separate CLI?
//!
//! We want users to use the Aptos CLI over the Rosetta CLI because of the added complexity of a
//! proxy server.  So, we split it out so general users aren't confused.
//!
//! TODO: Make Aptos CLI framework common among multiple CLIs

#![forbid(unsafe_code)]

mod account;
mod block;
mod common;
mod construction;
mod network;

use crate::common::RosettaCliArgs;
use aptos_cli_common::utils::print_cli_result;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args: RosettaCliArgs = RosettaCliArgs::parse();

    let result = args.execute().await;

    print_cli_result(result)
}
