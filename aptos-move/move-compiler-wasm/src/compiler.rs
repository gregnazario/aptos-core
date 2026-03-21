// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License

//! Move compiler WASM bindings

use wasm_bindgen::prelude::*;
use move_compiler_v2::{run_move_compiler, Options};
use move_compiler_v2::diagnostics::Emitter;
use move_core_types::account_address::AccountAddress;
use move_symbol_pool::Symbol;
use std::str::FromStr;
use codespan::{FileId, Files};
use codespan_reporting::diagnostic::Diagnostic;

use crate::{CompilationResult, CompilerError};

/// Simple string-based error emitter for WASM
struct StringEmitter {
    errors: Vec<String>,
}

impl StringEmitter {
    fn new() -> Self {
        Self { errors: vec![] }
    }
}

impl Emitter for StringEmitter {
    fn emit(&mut self, _source_files: &Files<String>, diag: &Diagnostic<FileId>) {
        // Simple error collection - just grab the message
        self.errors.push(format!("{:?}: {}", diag.severity, diag.message));
    }
}

/// Compile a single Move module from source code
///
/// # Arguments
/// * `source` - Move source code
/// * `address` - Named address for the module (e.g., "0x1")
/// * `module_name` - Name of the module
///
/// # Returns
/// CompilationResult with bytecode or errors
#[wasm_bindgen]
pub fn compile_module(source: String, address: String, module_name: String) -> CompilationResult {
    compile_module_impl(source, address, module_name)
        .unwrap_or_else(|e| CompilationResult::new_failure(vec![e.to_string()]))
}

fn compile_module_impl(
    source: String,
    address: String,
    _module_name: String,
) -> Result<CompilationResult, CompilerError> {
    // Parse the address
    let addr = AccountAddress::from_hex_literal(&address)
        .or_else(|_| AccountAddress::from_str(&address))
        .map_err(|e| CompilerError::InvalidAddress(format!("Invalid address '{}': {}", address, e)))?;

    // Extract named address from source
    let named_addr = extract_address_name(&source)
        .unwrap_or_else(|| Symbol::from("default_addr"));

    // Create temporary file in memory (write to temp location)
    let temp_path = format!("/tmp/claude/module_{}.move", addr.short_str_lossless());
    std::fs::write(&temp_path, &source)
        .map_err(|e| CompilerError::InternalError(format!("Failed to write temp file: {}", e)))?;

    // Configure compiler options
    let mut options = Options::default();
    options.sources = vec![temp_path.clone()];
    options.named_address_mapping = vec![format!("{}={}", named_addr, address)];
    options.skip_attribute_checks = true;

    // Create emitter
    let mut emitter = StringEmitter::new();

    // Compile
    let result = run_move_compiler(&mut emitter, options);

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_path);

    match result {
        Ok((_env, units)) => {
            // Extract bytecode from compiled units
            let mut all_bytecode = vec![];

            for unit in units {
                match unit {
                    legacy_move_compiler::compiled_unit::AnnotatedCompiledUnit::Module(module) => {
                        let mut bytes = vec![];
                        module.named_module.module.serialize(&mut bytes)
                            .map_err(|e| CompilerError::InternalError(format!("Serialization failed: {}", e)))?;
                        all_bytecode.extend(bytes);
                    }
                    _ => {}
                }
            }

            if all_bytecode.is_empty() {
                Err(CompilerError::NoBytecodeGenerated)
            } else {
                Ok(CompilationResult::new_success(all_bytecode, vec![]))
            }
        }
        Err(e) => {
            if !emitter.errors.is_empty() {
                Err(CompilerError::CompilationFailed(emitter.errors))
            } else {
                Err(CompilerError::InternalError(format!("Compilation failed: {}", e)))
            }
        }
    }
}

/// Extract the named address from Move source code
fn extract_address_name(source: &str) -> Option<Symbol> {
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("module ") {
            let parts: Vec<&str> = trimmed.split("::").collect();
            if parts.len() >= 2 {
                let addr_part = parts[0].trim_start_matches("module ").trim();
                if !addr_part.starts_with("0x") {
                    return Some(Symbol::from(addr_part));
                }
            }
        }
    }
    None
}

/// Compile a Move script from source code
#[wasm_bindgen]
pub fn compile_script(source: String, address: String) -> CompilationResult {
    compile_script_impl(source, address)
        .unwrap_or_else(|e| CompilationResult::new_failure(vec![e.to_string()]))
}

fn compile_script_impl(
    source: String,
    address: String,
) -> Result<CompilationResult, CompilerError> {
    let addr = AccountAddress::from_hex_literal(&address)
        .or_else(|_| AccountAddress::from_str(&address))
        .map_err(|e| CompilerError::InvalidAddress(format!("Invalid address '{}': {}", address, e)))?;

    let temp_path = format!("/tmp/claude/script_{}.move", addr.short_str_lossless());
    std::fs::write(&temp_path, &source)
        .map_err(|e| CompilerError::InternalError(format!("Failed to write temp file: {}", e)))?;

    let mut options = Options::default();
    options.sources = vec![temp_path.clone()];
    options.skip_attribute_checks = true;

    let mut emitter = StringEmitter::new();
    let result = run_move_compiler(&mut emitter, options);
    let _ = std::fs::remove_file(&temp_path);

    match result {
        Ok((_env, units)) => {
            let mut all_bytecode = vec![];

            for unit in units {
                match unit {
                    legacy_move_compiler::compiled_unit::AnnotatedCompiledUnit::Script(script) => {
                        let mut bytes = vec![];
                        script.named_script.script.serialize(&mut bytes)
                            .map_err(|e| CompilerError::InternalError(format!("Serialization failed: {}", e)))?;
                        all_bytecode.extend(bytes);
                    }
                    _ => {}
                }
            }

            if all_bytecode.is_empty() {
                Err(CompilerError::NoBytecodeGenerated)
            } else {
                Ok(CompilationResult::new_success(all_bytecode, vec![]))
            }
        }
        Err(e) => {
            if !emitter.errors.is_empty() {
                Err(CompilerError::CompilationFailed(emitter.errors))
            } else {
                Err(CompilerError::InternalError(format!("Compilation failed: {}", e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_address_name() {
        let source = r#"
        module my_addr::Test {
            public fun hello() {}
        }
        "#;
        assert_eq!(extract_address_name(source), Some(Symbol::from("my_addr")));
    }
}
