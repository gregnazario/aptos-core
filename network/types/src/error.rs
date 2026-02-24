// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkTypesError {
    #[error("Invariant violation: {0}")]
    InvariantViolation(String),
}
