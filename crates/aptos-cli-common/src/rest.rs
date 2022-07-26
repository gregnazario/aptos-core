// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::config::{CliConfig, DEFAULT_REST_URL};
use crate::types::{CliError, CliTypedResult};
use clap::Parser;

/// Options specific to using the Rest endpoint
#[derive(Debug, Default, Parser)]
pub struct RestOptions {
    /// URL to a fullnode on the network
    ///
    /// Defaults to <https://fullnode.devnet.aptoslabs.com>
    #[clap(long, parse(try_from_str))]
    url: Option<reqwest::Url>,
}

impl RestOptions {
    pub fn new(url: Option<reqwest::Url>) -> Self {
        RestOptions { url }
    }

    /// Retrieve the URL from the profile or the command line
    pub fn url(&self, profile: &str) -> CliTypedResult<reqwest::Url> {
        if let Some(ref url) = self.url {
            Ok(url.clone())
        } else if let Some(Some(url)) = CliConfig::load_profile(profile)?.map(|p| p.rest_url) {
            reqwest::Url::parse(&url)
                .map_err(|err| CliError::UnableToParse("Rest URL", err.to_string()))
        } else {
            reqwest::Url::parse(DEFAULT_REST_URL).map_err(|err| {
                CliError::UnexpectedError(format!("Failed to parse default rest URL {}", err))
            })
        }
    }

    pub fn client(&self, profile: &str) -> CliTypedResult<aptos_rest_client::Client> {
        Ok(aptos_rest_client::Client::new(self.url(profile)?))
    }
}
