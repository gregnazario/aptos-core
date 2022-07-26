// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_cli_base::file::{create_dir_if_not_exist, write_to_file};
use aptos_cli_base::parse::{from_yaml, to_yaml};
use aptos_cli_base::types::{CliError, CliTypedResult};
use aptos_cli_common::command::CliCommand;
use aptos_genesis::config::Layout;
use async_trait::async_trait;
use clap::Parser;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, io::Read, path::PathBuf, str::FromStr};

pub const LAYOUT_NAME: &str = "layout";

/// Setup a shared Github repository for Genesis
///
#[derive(Parser)]
pub struct SetupGit {
    #[clap(flatten)]
    pub(crate) git_options: GitOptions,
    /// Path to `Layout` which defines where all the files are
    #[clap(long, parse(from_os_str))]
    pub(crate) layout_file: PathBuf,
}

#[async_trait]
impl CliCommand<()> for SetupGit {
    fn command_name(&self) -> &'static str {
        "SetupGit"
    }

    async fn execute(self) -> CliTypedResult<()> {
        let layout = Layout::from_disk(&self.layout_file).map_err(CliError::unexpected)?;

        // Upload layout file to ensure we can read later
        let client = self.git_options.get_client()?;
        client.put(LAYOUT_NAME, &layout)?;

        // Make a place for the modules to be uploaded
        client.create_dir("framework")?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct GithubRepo {
    owner: String,
    repository: String,
}

impl FromStr for GithubRepo {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('/').collect();
        if parts.len() != 2 {
            Err(CliError::CommandArgumentError("Invalid repository must be of the form 'owner/repository` e.g. 'aptos-labs/aptos-core'".to_string()))
        } else {
            Ok(GithubRepo {
                owner: parts.get(0).unwrap().to_string(),
                repository: parts.get(1).unwrap().to_string(),
            })
        }
    }
}

#[derive(Clone, Default, Parser)]
pub struct GitOptions {
    /// Github repository e.g. 'aptos-labs/aptos-core'
    #[clap(long)]
    pub(crate) github_repository: Option<GithubRepo>,
    /// Github repository branch e.g. main
    #[clap(long, default_value = "main")]
    pub(crate) github_branch: String,
    /// Path to Github API token.  Token must have repo:* permissions
    #[clap(long, parse(from_os_str))]
    pub(crate) github_token_file: Option<PathBuf>,
    /// Path to local git repository
    #[clap(long, parse(from_os_str))]
    pub(crate) local_repository_dir: Option<PathBuf>,
}

impl GitOptions {
    pub fn get_client(self) -> CliTypedResult<Client> {
        if self.github_repository.is_none()
            && self.github_token_file.is_none()
            && self.local_repository_dir.is_some()
        {
            Ok(Client::local(self.local_repository_dir.unwrap()))
        } else if self.github_repository.is_some()
            && self.github_token_file.is_some()
            && self.local_repository_dir.is_none()
        {
            Err(CliError::CommandArgumentError("Unsupported".to_string()))
        } else {
            Err(CliError::CommandArgumentError("Must provide either only --local-repository-dir or both --github-repository and --github-token-path".to_string()))
        }
    }
}

/// A client for abstracting away local vs Github storage
///
/// Note: Writes do not commit locally
pub enum Client {
    Local(PathBuf),
}

impl Client {
    pub fn local(path: PathBuf) -> Client {
        Client::Local(path)
    }

    /// Retrieves an object as a YAML encoded file from the appropriate storage
    pub fn get<T: DeserializeOwned + Debug>(&self, name: &str) -> CliTypedResult<T> {
        match self {
            Client::Local(local_repository_path) => {
                let path = local_repository_path.join(format!("{}.yaml", name));
                let mut file = std::fs::File::open(path.as_path())
                    .map_err(|e| CliError::IO(path.display().to_string(), e))?;

                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| CliError::IO(path.display().to_string(), e))?;
                from_yaml(&contents)
            }
        }
    }

    /// Puts an object as a YAML encoded file to the appropriate storage
    pub fn put<T: Serialize + ?Sized>(&self, name: &str, input: &T) -> CliTypedResult<()> {
        match self {
            Client::Local(local_repository_path) => {
                self.create_dir(local_repository_path.to_str().unwrap())?;

                let path = local_repository_path.join(format!("{}.yaml", name));
                write_to_file(
                    path.as_path(),
                    &path.display().to_string(),
                    to_yaml(input)?.as_bytes(),
                )?;
            }
        }

        Ok(())
    }

    pub fn create_dir(&self, name: &str) -> CliTypedResult<()> {
        match self {
            Client::Local(local_repository_path) => {
                let path = local_repository_path.join(name);
                create_dir_if_not_exist(path.as_path())?;
            }
        }

        Ok(())
    }

    /// Retrieve bytecode Move modules from a module folder
    pub fn get_modules(&self, name: &str) -> CliTypedResult<Vec<Vec<u8>>> {
        let mut modules = Vec::new();

        match self {
            Client::Local(local_repository_path) => {
                let module_folder = local_repository_path.join(name);
                if !module_folder.is_dir() {
                    return Err(CliError::UnexpectedError(format!(
                        "{} is not a directory!",
                        module_folder.display()
                    )));
                }

                let files = std::fs::read_dir(module_folder.as_path())
                    .map_err(|e| CliError::IO(module_folder.display().to_string(), e))?;

                for maybe_file in files {
                    let file = maybe_file
                        .map_err(|e| CliError::UnexpectedError(e.to_string()))?
                        .path();
                    let extension = file.extension();

                    // Only collect move files
                    if file.is_file() && extension.is_some() && extension.unwrap() == "mv" {
                        modules.push(
                            std::fs::read(file.as_path())
                                .map_err(|e| CliError::IO(file.display().to_string(), e))?,
                        );
                    }
                }
            }
        }
        Ok(modules)
    }
}
