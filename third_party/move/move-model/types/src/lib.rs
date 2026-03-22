// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

//! Lightweight types extracted from `move-model` so that crates like `aptos-types`
//! can deserialize compilation metadata without depending on the full Move compiler chain.

use serde::{Deserialize, Serialize};

/// Key used to look up [`CompilationMetadata`] in module metadata.
pub static COMPILATION_METADATA_KEY: &[u8] = "compilation_metadata".as_bytes();

/// Metadata about a compilation result. To maintain serialization
/// stability, this uses a free-form string to represent compiler version
/// and language version, which is interpreted by the `CompilerVersion`
/// and `LanguageVersion` types. This allows to always successfully
/// deserialize the metadata (even older code with newer data), and leave it
/// up to the program how to deal with decoding errors.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompilationMetadata {
    /// A flag indicating whether, at time of creation, the compilation
    /// result was considered as unstable. Unstable code may have restrictions
    /// for deployment on production networks. This flag is true if either the
    /// compiler or language versions are unstable.
    pub unstable: bool,
    /// The version of the compiler, as a string.
    pub compiler_version: String,
    /// The version of the language, as a string.
    pub language_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation_metadata_bcs_round_trip() {
        let meta = CompilationMetadata {
            unstable: true,
            compiler_version: "2.0".to_string(),
            language_version: "2.3".to_string(),
        };
        let bytes = bcs::to_bytes(&meta).unwrap();
        let decoded: CompilationMetadata = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.unstable, meta.unstable);
        assert_eq!(decoded.compiler_version, meta.compiler_version);
        assert_eq!(decoded.language_version, meta.language_version);
    }
}
