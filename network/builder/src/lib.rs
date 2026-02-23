// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

pub use aptos_network::protocols::rpc::error::RpcError;
pub mod builder;

#[cfg(test)]
mod dummy;
#[cfg(test)]
mod test;
