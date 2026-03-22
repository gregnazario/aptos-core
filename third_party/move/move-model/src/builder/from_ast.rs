// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Build GlobalEnv from already-parsed AST (no filesystem access required)
//!
//! This module provides an alternative to run_model_builder_in_compiler_mode()
//! that accepts an already-parsed AST instead of file paths. This enables
//! compilation in environments without filesystem access (WASM, testing, etc.).

use crate::{add_move_lang_diagnostics, model::GlobalEnv, options::ModelBuilderOptions};
use itertools::Itertools;
use legacy_move_compiler::{
    command_line::compiler::{PASS_EXPANSION, PASS_PARSER},
    diagnostics::FilesSourceText,
    parser::ast::Program as ParsedProgram,
    shared::{Compiler, Flags, NamedAddressMaps},
};
use move_symbol_pool::Symbol as MoveSymbol;
use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

/// Build Move model from already-parsed AST
///
/// This is the filesystem-free equivalent of `run_model_builder_in_compiler_mode()`.
/// Instead of accepting file paths and parsing them, it accepts an already-parsed
/// program AST and file contents.
///
/// # Arguments
/// * `parsed_program` - Already-parsed Move program (from `parse_program_from_sources`)
/// * `files` - Map of file hashes to (name, content) for source files
/// * `named_address_maps` - Named address mappings
/// * `target_file_names` - Set of file names that are compilation targets (vs deps)
/// * `options` - Model builder options
/// * `flags` - Compilation flags
/// * `known_attributes` - Set of recognized attributes
///
/// # Returns
/// GlobalEnv with the compiled model, or errors if compilation fails
///
/// # Example
/// ```ignore
/// use move_compiler_v2::sources::SourceMap;
/// use legacy_move_compiler::parser::parse_program_from_sources;
///
/// // Parse from sources
/// let mut sources = SourceMap::new();
/// sources.add_file("test.move", "module 0x1::Test {}");
///
/// let targets = sources.to_files_source_text();
/// let (files, pprog_res) = parse_program_from_sources(...)?;
/// let (pprog, comments) = pprog_res?;
///
/// // Build model from AST
/// let env = run_model_builder_from_ast(
///     pprog,
///     files,
///     named_address_maps,
///     target_files,
///     options,
///     flags,
///     &known_attrs,
/// )?;
/// ```
pub fn run_model_builder_from_ast(
    parsed_program: ParsedProgram,
    files: FilesSourceText,
    named_address_maps: NamedAddressMaps,
    target_file_names: BTreeSet<String>,
    options: ModelBuilderOptions,
    flags: Flags,
    known_attributes: &BTreeSet<String>,
) -> anyhow::Result<GlobalEnv> {
    let mut env = GlobalEnv::new();
    env.set_language_version(options.language_version);
    env.set_extension(options);

    // Identify which files are dependencies vs targets
    let dep_files: BTreeSet<_> = parsed_program
        .lib_definitions
        .iter()
        .map(|p| p.def.file_hash())
        .collect();

    // Add source files to the environment
    for member in parsed_program
        .source_definitions
        .iter()
        .chain(parsed_program.lib_definitions.iter())
    {
        let fhash = member.def.file_hash();
        let (fname, fsrc) = files.get(&fhash).unwrap();
        let is_target = !dep_files.contains(&fhash);

        let aliases = parsed_program
            .named_address_maps
            .get(member.named_address_map)
            .iter()
            .map(|(symbol, addr)| (env.symbol_pool().make(symbol.as_str()), *addr))
            .collect();

        env.add_source(
            fhash,
            Rc::new(aliases),
            fname.as_str(),
            fsrc,
            is_target,
            target_file_names.contains(fname.as_str()),
        );
    }

    // Add any files that don't contain definitions
    for fhash in files.keys().sorted() {
        if env.get_file_id(*fhash).is_none() {
            let (fname, fsrc) = files.get(fhash).unwrap();
            let is_target = !dep_files.contains(fhash);
            env.add_source(
                *fhash,
                Rc::new(BTreeMap::new()),
                fname.as_str(),
                fsrc,
                is_target,
                target_file_names.contains(fname.as_str()),
            );
        }
    }

    // Create a compiler from the parsed AST (bypasses file reading/parsing)
    // This is the key: we use at_parser() instead of from_files()
    let compiler = Compiler::new_for_testing(
        vec![],  // No sources (already parsed)
        vec![],  // No deps (already parsed)
        named_address_maps.clone(),
        flags.clone(),
        known_attributes,
    )
    .at_parser(parsed_program);

    // Step 2: Run expansion
    let (_, expansion_ast) = match compiler.run::<PASS_EXPANSION>() {
        Err(diags) => {
            add_move_lang_diagnostics(&mut env, diags);
            return Ok(env);
        },
        Ok(res) => res,
    };

    // Continue with the rest of the pipeline (same as run_model_builder_with_options_and_compilation_flags)
    // TODO: Continue with typing, etc.
    // For now, this is a placeholder - need to implement the rest

    Ok(env)
}
