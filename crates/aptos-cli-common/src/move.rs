// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::account::AccountAddressWrapper;
use aptos_cli_base::file::dir_default_to_current;
use aptos_cli_base::types::CliTypedResult;
use aptos_types::account_address::AccountAddress;
use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Options for compiling a move package dir
#[derive(Debug, Parser)]
pub struct MovePackageDir {
    /// Path to a move package (the folder with a Move.toml file)
    #[clap(long, parse(from_os_str))]
    pub package_dir: Option<PathBuf>,
    /// Path to save the compiled move package
    ///
    /// Defaults to `<package_dir>/build`
    #[clap(long, parse(from_os_str))]
    pub output_dir: Option<PathBuf>,
    /// Named addresses for the move binary
    ///
    /// Example: alice=0x1234, bob=0x5678
    ///
    /// Note: This will fail if there are duplicates in the Move.toml file remove those first.
    #[clap(long, parse(try_from_str = aptos_cli_base::parse::parse_map), default_value = "")]
    named_addresses: BTreeMap<String, AccountAddressWrapper>,
}

impl MovePackageDir {
    pub fn get_package_dir(&self) -> CliTypedResult<PathBuf> {
        dir_default_to_current(self.package_dir.clone())
    }

    /// Retrieve the NamedAddresses, resolving all the account addresses accordingly
    pub fn named_addresses(&self) -> BTreeMap<String, AccountAddress> {
        self.named_addresses
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.account_address))
            .collect()
    }
}
