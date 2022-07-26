// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::types::CliTypedResult;
use aptos_keygen::KeyGen;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct RngArgs {
    /// The seed used for key generation, should be a 64 character hex string and mainly used for testing
    ///
    /// This field is hidden from the CLI input for now
    #[clap(skip)]
    random_seed: Option<String>,
}

impl RngArgs {
    pub fn from_seed(seed: [u8; 32]) -> RngArgs {
        RngArgs {
            random_seed: Some(hex::encode(seed)),
        }
    }

    /// Returns a key generator with the seed if given
    pub fn key_generator(&self) -> CliTypedResult<KeyGen> {
        if let Some(ref seed) = self.random_seed {
            // Strip 0x
            let seed = seed.strip_prefix("0x").unwrap_or(seed);
            let mut seed_slice = [0u8; 32];

            hex::decode_to_slice(seed, &mut seed_slice)?;
            Ok(KeyGen::from_seed(seed_slice))
        } else {
            Ok(KeyGen::from_os_rng())
        }
    }
}
