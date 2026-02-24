// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

/// A collection of constants and default values for configuring various network components.

// NB: Almost all of these values are educated guesses, and not determined using any empirical
// data. If you run into a limit and believe that it is unreasonably tight, please submit a PR
// with your use-case. If you do change a value, please add a comment linking to the PR which
// advocated the change.
/// The timeout for any inbound RPC call before it's cut off
pub(crate) const INBOUND_RPC_TIMEOUT_MS: u64 = 10_000;
/// Limit on concurrent Outbound RPC requests before backpressure is applied
pub(crate) const MAX_CONCURRENT_OUTBOUND_RPCS: u32 = 100;
/// Limit on concurrent Inbound RPC requests before backpressure is applied
pub(crate) const MAX_CONCURRENT_INBOUND_RPCS: u32 = 100;

pub(crate) use aptos_network_types::NETWORK_CHANNEL_SIZE;
#[cfg(any(test, feature = "fuzzing"))]
pub(crate) use aptos_network_types::MAX_FRAME_SIZE;
pub use aptos_network_types::MAX_MESSAGE_SIZE;
