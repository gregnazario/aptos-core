// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::types::CliResult;
use aptos_logger::Level;
use std::process::exit;

pub fn start_logger() {
    let mut logger = aptos_logger::Logger::new();
    logger
        .channel_size(1000)
        .is_async(false)
        .level(Level::Warn)
        .read_env();
    logger.build();
}

pub fn print_cli_result(result: CliResult) {
    match result {
        Ok(inner) => println!("{}", inner),
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        }
    }
}
