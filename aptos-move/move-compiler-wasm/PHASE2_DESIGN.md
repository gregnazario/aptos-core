# Phase 2: API Design for In-Memory Compilation

**Date**: 2026-03-22
**Status**: Design Complete, Ready for Implementation

---

## Design Overview

Add source-based compilation APIs to Move compiler without modifying any existing functionality.

### Key Principle
**100% Backward Compatible** - All existing file-based APIs remain unchanged.

---

## Core Type: SourceMap

### Definition
```rust
// File: move-compiler-v2/src/sources.rs (NEW FILE)

use move_command_line_common::files::FileHash;
use move_symbol_pool::Symbol;
use std::collections::BTreeMap;

/// In-memory source file collection for compilation without filesystem
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    /// Map from virtual file path to source code content
    files: BTreeMap<Symbol, String>,
}

impl SourceMap {
    /// Create a new empty source map
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    /// Add a source file with virtual path
    ///
    /// # Arguments
    /// * `path` - Virtual file path (e.g., "MyModule.move", used in error messages)
    /// * `content` - Move source code as string
    ///
    /// # Example
    /// ```
    /// let mut sources = SourceMap::new();
    /// sources.add_file("test.move", r#"
    ///     module 0x1::Test {
    ///         public fun hello(): u64 { 42 }
    ///     }
    /// "#);
    /// ```
    pub fn add_file(&mut self, path: impl Into<Symbol>, content: impl Into<String>) {
        self.files.insert(path.into(), content.into());
    }

    /// Get source content for a virtual path
    pub fn get_file(&self, path: &Symbol) -> Option<&str> {
        self.files.get(path).map(|s| s.as_str())
    }

    /// Check if a file exists
    pub fn contains(&self, path: &Symbol) -> bool {
        self.files.contains_key(path)
    }

    /// Get all file paths
    pub fn paths(&self) -> impl Iterator<Item = &Symbol> {
        self.files.keys()
    }

    /// Get number of source files
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Convert to FilesSourceText format used internally by compiler
    ///
    /// This creates a HashMap keyed by content hash (SHA256) with values of
    /// (filename, content). The compiler uses this for error reporting.
    pub(crate) fn to_files_source_text(&self) -> FilesSourceText {
        use move_command_line_common::files::FileHash;

        self.files
            .iter()
            .map(|(name, content)| {
                let hash = FileHash::new(content);
                (hash, (*name, content.clone()))
            })
            .collect()
    }
}

impl FromIterator<(Symbol, String)> for SourceMap {
    fn from_iter<T: IntoIterator<Item = (Symbol, String)>>(iter: T) -> Self {
        Self {
            files: iter.into_iter().collect(),
        }
    }
}
```

### Design Rationale

**Why BTreeMap instead of HashMap?**
- Deterministic ordering (important for reproducible compilation)
- Better for debugging (files listed alphabetically)
- Slightly slower, but negligible for typical number of source files

**Why Symbol instead of String for paths?**
- Matches existing compiler conventions
- Symbols are interned, saving memory
- Fast comparison/hashing

**Why not expose FilesSourceText directly?**
- FilesSourceText is keyed by content hash - awkward for API
- SourceMap provides cleaner interface (path → content)
- Conversion to FilesSourceText is internal implementation detail

---

## New Parser Function

### Signature
```rust
// File: legacy-move-compiler/src/parser/mod.rs (ADD TO EXISTING FILE)

/// Parse Move source files from in-memory strings
///
/// This is the source-based equivalent of `parse_program()`.
/// Instead of reading from filesystem, it uses pre-loaded source content.
///
/// # Arguments
/// * `env` - Compilation environment for error reporting
/// * `targets` - Target source files to compile
/// * `deps` - Dependency source files (not compiled as targets)
///
/// # Returns
/// Tuple of:
/// - FilesSourceText: Map of file hashes to (name, content)
/// - Result of parsed program or diagnostics
pub fn parse_program_from_sources(
    env: &mut CompilationEnv,
    maps: NamedAddressMaps,
    targets: FilesSourceText,    // Already converted from SourceMap
    deps: FilesSourceText,        // Already converted from SourceMap
) -> anyhow::Result<(
    FilesSourceText,
    Result<(parser::ast::Program, CommentMap), Diagnostics>,
)> {
    // Combine targets and deps into single source map
    let mut all_files = targets.clone();
    all_files.extend(deps.clone());

    // Check for duplicates between targets and deps
    ensure_targets_deps_dont_intersect_sources(&targets, &deps)?;

    // Parse each file using existing parse_file_string
    let mut parsed_files: BTreeMap<FileHash, (Symbol, parser::ast::FileDefinition)> =
        BTreeMap::new();
    let mut all_comments = MatchedFileCommentMap::new();
    let mut all_diagnostics = Diagnostics::new();

    for (file_hash, (file_name, source_content)) in all_files.iter() {
        // Use existing string parser
        match parse_file_string(env, *file_hash, source_content) {
            Ok((defs, comments)) => {
                parsed_files.insert(
                    *file_hash,
                    (*file_name, parser::ast::FileDefinition { defs }),
                );
                all_comments.insert(*file_hash, comments);
            }
            Err(diags) => {
                all_diagnostics.extend(diags);
            }
        }
    }

    // If parsing failed, return errors
    if all_diagnostics.has_errors() {
        return Ok((all_files, Err(all_diagnostics)));
    }

    // Build program AST (same logic as file-based parse_program)
    let file_definitions: Vec<_> = parsed_files
        .into_iter()
        .map(|(file_hash, (fname, def))| (file_hash, fname, def))
        .collect();

    let pprog = parser::ast::Program {
        named_address_maps: maps.clone(),
        source_definitions: file_definitions
            .into_iter()
            .filter(|(hash, _, _)| targets.contains_key(hash))
            .collect(),
        lib_definitions: file_definitions
            .into_iter()
            .filter(|(hash, _, _)| deps.contains_key(hash))
            .collect(),
    };

    let comment_map = all_comments
        .into_iter()
        .flat_map(|(_, comments)| comments)
        .collect();

    Ok((all_files, Ok((pprog, comment_map))))
}

/// Check that targets and deps don't intersect (source-based version)
fn ensure_targets_deps_dont_intersect_sources(
    targets: &FilesSourceText,
    deps: &FilesSourceText,
) -> anyhow::Result<()> {
    // Check by file hash (content-based)
    let target_hashes: BTreeSet<_> = targets.keys().collect();
    let dep_hashes: BTreeSet<_> = deps.keys().collect();

    let intersection: Vec<_> = target_hashes
        .intersection(&dep_hashes)
        .collect();

    if !intersection.is_empty() {
        let duplicate_files: Vec<_> = intersection
            .iter()
            .map(|hash| {
                let (name, _) = targets.get(hash).or_else(|| deps.get(hash)).unwrap();
                name.as_str()
            })
            .collect();

        bail!(
            "The following files were marked as both targets and dependencies:\n{}",
            duplicate_files.join("\n")
        );
    }

    Ok(())
}
```

---

## New Compiler Entry Point

### Signature
```rust
// File: move-compiler-v2/src/lib.rs (ADD TO EXISTING FILE)

use crate::sources::SourceMap;

/// Compile Move modules from in-memory sources (no filesystem access)
///
/// This is the source-based equivalent of `run_move_compiler()`.
/// Enables compilation in WASM, testing, IDE integration, etc.
///
/// # Arguments
/// * `error_writer` - Diagnostic output writer
/// * `sources` - Target source files to compile
/// * `deps` - Dependency source files (stdlib, other modules)
/// * `named_addresses` - Named address mappings (e.g., "std" => 0x1)
/// * `flags` - Compiler flags (testing, verification, etc.)
///
/// # Returns
/// Tuple of:
/// - GlobalEnv: Global compilation environment
/// - Vec<AnnotatedCompiledUnit>: Compiled bytecode units
///
/// # Example
/// ```
/// use move_compiler_v2::sources::SourceMap;
/// use move_compiler_v2::Flags;
///
/// let mut sources = SourceMap::new();
/// sources.add_file("MyModule.move", r#"
///     module my_addr::MyModule {
///         public fun hello(): u64 { 42 }
///     }
/// "#);
///
/// let mut deps = load_stdlib_sources();  // Hypothetical stdlib loader
///
/// let named_addresses = [
///     ("my_addr".to_string(), AccountAddress::from_hex_literal("0x42").unwrap()),
/// ].into();
///
/// let (env, units) = run_move_compiler_from_sources(
///     &mut stderr_emitter(),
///     sources,
///     deps,
///     named_addresses,
///     Flags::empty(),
/// )?;
/// ```
pub fn run_move_compiler_from_sources(
    error_writer: &mut impl Emitter,
    sources: SourceMap,
    deps: SourceMap,
    named_addresses: BTreeMap<String, AccountAddress>,
    flags: Flags,
) -> anyhow::Result<(GlobalEnv, Vec<AnnotatedCompiledUnit>)> {
    // Convert SourceMap to FilesSourceText (internal format)
    let targets_files = sources.to_files_source_text();
    let deps_files = deps.to_files_source_text();

    // Create named address maps
    let mut maps = NamedAddressMaps::new();
    let named_addr_map: NamedAddressMap = named_addresses
        .into_iter()
        .map(|(name, addr)| (Symbol::from(name), addr.into()))
        .collect();
    let map_idx = maps.insert(named_addr_map);

    // Parse from sources instead of files
    let (files_source_text, pprog_res) = legacy_move_compiler::parser::parse_program_from_sources(
        &mut CompilationEnv::new(flags.clone(), BTreeSet::new()),
        maps.clone(),
        targets_files,
        deps_files,
    )?;

    // Continue with rest of compilation pipeline (unchanged)
    let (pprog, comment_map) = pprog_res.map_err(|diags| {
        // Emit diagnostics
        for diag in diags.into_iter() {
            error_writer.emit(&files_source_text, &diag);
        }
        anyhow!("Compilation failed")
    })?;

    // Run expansion, typing, bytecode generation (existing pipeline)
    // ... (rest of run_move_compiler logic, unchanged)

    Ok((env, units))
}
```

---

## WASM Integration

### Updated Compiler
```rust
// File: aptos-move/move-compiler-wasm/src/compiler.rs

use move_compiler_v2::sources::SourceMap;

pub fn compile_module(
    source: String,
    address: String,
    module_name: String,
) -> CompilationResult {
    compile_module_impl(source, address, module_name)
        .unwrap_or_else(|e| CompilationResult::new_failure(vec![e.to_string()]))
}

fn compile_module_impl(
    source: String,
    address: String,
    module_name: String,
) -> Result<CompilationResult, CompilerError> {
    // Parse address
    let addr = AccountAddress::from_hex_literal(&address)
        .or_else(|_| AccountAddress::from_str(&address))
        .map_err(|e| CompilerError::InvalidAddress(format!("Invalid address '{}': {}", address, e)))?;

    // Extract named address from source
    let named_addr = extract_address_name(&source)
        .unwrap_or_else(|| Symbol::from("default_addr"));

    // Create in-memory source map
    let mut sources = SourceMap::new();
    sources.add_file(
        format!("{}.move", module_name),
        source,
    );

    // Named address mappings
    let named_addresses = [(named_addr.to_string(), addr)].into();

    // Compile from sources (NO FILESYSTEM!)
    let mut emitter = StringEmitter::new();
    let result = move_compiler_v2::run_move_compiler_from_sources(
        &mut emitter,
        sources,
        SourceMap::new(),  // No deps for now
        named_addresses,
        Flags::empty(),
    );

    match result {
        Ok((_, units)) => {
            // Extract bytecode
            let mut all_bytecode = vec![];
            for unit in units {
                match unit {
                    AnnotatedCompiledUnit::Module(module) => {
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
```

### Benefits
- ❌ DELETE: All filesystem code (`std::fs::*`)
- ❌ DELETE: `memfs` module (no longer needed)
- ❌ DELETE: Temp file creation/deletion
- ✅ ADD: Clean source-based API
- ✅ RESULT: True browser-native compilation

---

## Error Reporting

### Virtual Paths in Errors

**Problem**: Errors need to reference file locations, but files don't exist on disk.

**Solution**: Use meaningful virtual paths.

### Examples

**Good virtual path**:
```
error: undefined function
   ┌─ MyModule.move:5:9
   │
 5 │         this_does_not_exist();
   │         ^^^^^^^^^^^^^^^^^^^ function not found
```

**Bad virtual path**:
```
error: undefined function
   ┌─ 3a7f5c9e.move:5:9  ← Opaque hash
   │
```

### Implementation
```rust
// When adding file to SourceMap, use descriptive names:
sources.add_file("MyModule.move", source);  // ✅ Good
sources.add_file(&format!("{}.move", module_name), source);  // ✅ Good
sources.add_file(&format!("module_{}.move", hash), source);  // ❌ Bad
```

---

## Dependency Handling Strategy

### Phase 1: No Built-in Dependencies (MVP)
```rust
// Caller must provide all dependencies
let mut deps = SourceMap::new();
deps.add_file("vector.move", VECTOR_SOURCE);
deps.add_file("signer.move", SIGNER_SOURCE);

compile_from_sources(sources, deps, ...);
```

**Pro**: Simple, no bloat
**Con**: User must provide stdlib manually

### Phase 2: Optional Stdlib Bundle
```rust
// Add convenience function
pub fn load_move_stdlib() -> SourceMap {
    let mut stdlib = SourceMap::new();
    stdlib.add_file("vector.move", include_str!("../stdlib/vector.move"));
    stdlib.add_file("signer.move", include_str!("../stdlib/signer.move"));
    // ... ~20 files, ~50KB total
    stdlib
}

// Usage
let deps = load_move_stdlib();
compile_from_sources(sources, deps, ...);
```

**Pro**: Convenient, still optional
**Con**: Increases WASM binary size by ~50KB

### Phase 3: Pre-compiled Stdlib (Future)
```rust
// Include pre-compiled bytecode
const STDLIB_COMPILED: &[u8] = include_bytes!("../stdlib.mv");

// Load as dependency
pub fn load_compiled_stdlib() -> CompiledDependency {
    CompiledDependency::from_bytes(STDLIB_COMPILED)
}
```

**Pro**: Smaller size, faster
**Con**: Requires API changes for compiled deps

**Recommendation**: Start with Phase 1, add Phase 2 if users request it.

---

## Testing Strategy

### Unit Tests
```rust
// File: move-compiler-v2/tests/sources_tests.rs (NEW FILE)

#[test]
fn test_simple_module() {
    let mut sources = SourceMap::new();
    sources.add_file("test.move", r#"
        module 0x1::Test {
            public fun hello(): u64 { 42 }
        }
    "#);

    let result = run_move_compiler_from_sources(
        &mut NullEmitter,
        sources,
        SourceMap::new(),
        [("std".into(), AccountAddress::ONE)].into(),
        Flags::empty(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_syntax_error() {
    let mut sources = SourceMap::new();
    sources.add_file("broken.move", r#"
        module 0x1::Broken {
            this is not valid move code
        }
    "#);

    let result = run_move_compiler_from_sources(
        &mut StringEmitter::new(),
        sources,
        SourceMap::new(),
        [].into(),
        Flags::empty(),
    );

    assert!(result.is_err());
}

#[test]
fn test_duplicate_targets_deps() {
    let mut sources = SourceMap::new();
    sources.add_file("dup.move", "module 0x1::Dup {}");

    let mut deps = SourceMap::new();
    deps.add_file("dup.move", "module 0x1::Dup {}");  // Same content!

    let result = run_move_compiler_from_sources(
        &mut NullEmitter,
        sources,
        deps,
        [].into(),
        Flags::empty(),
    );

    // Should fail - same file in both targets and deps
    assert!(result.is_err());
}
```

### Integration Tests
- Compile multi-module packages
- Cross-module dependencies
- Named address resolution
- Error message formatting

### WASM Tests
```rust
// File: aptos-move/move-compiler-wasm/tests/wasm.rs

#[wasm_bindgen_test]
fn test_wasm_compile() {
    let source = r#"
        module 0x1::Test {
            public fun hello(): u64 { 42 }
        }
    "#;

    let result = compile_module(source.into(), "0x1".into(), "Test".into());
    assert!(result.success());
    assert!(result.bytecode().len() > 0);
}
```

---

## Implementation Plan

### Step 1: Create SourceMap Type (2 hours)
- [ ] Create `move-compiler-v2/src/sources.rs`
- [ ] Implement `SourceMap` struct
- [ ] Add unit tests
- [ ] Update `mod.rs` to export

### Step 2: Add Parser Function (4 hours)
- [ ] Add `parse_program_from_sources()` to `parser/mod.rs`
- [ ] Add `ensure_targets_deps_dont_intersect_sources()`
- [ ] Test parsing from strings
- [ ] Verify error messages use virtual paths correctly

### Step 3: Add Compiler Entry Point (1 day)
- [ ] Add `run_move_compiler_from_sources()` to `lib.rs`
- [ ] Wire up to existing compilation pipeline
- [ ] Test full compilation flow
- [ ] Add integration tests

### Step 4: Update WASM Bindings (4 hours)
- [ ] Modify `aptos-move/move-compiler-wasm/src/compiler.rs`
- [ ] Remove all filesystem code
- [ ] Update to use `SourceMap`
- [ ] Test in Node.js and browsers

### Step 5: Documentation (1 day)
- [ ] Add rustdoc comments
- [ ] Create usage examples
- [ ] Update README files
- [ ] Write migration guide

### Step 6: Testing (2 days)
- [ ] Unit tests for all new functions
- [ ] Integration tests
- [ ] WASM tests (Node, Firefox, Chrome)
- [ ] Regression tests (ensure file-based still works)

**Total Estimated Time**: 4-5 days of focused work

---

## Success Criteria

### Must Have ✅
- [ ] `SourceMap` type implemented and tested
- [ ] `run_move_compiler_from_sources()` works end-to-end
- [ ] WASM compiler works in browsers without filesystem
- [ ] All existing tests still pass (backward compatibility)
- [ ] Error messages reference virtual paths correctly

### Should Have 📋
- [ ] Comprehensive unit tests (>80% coverage of new code)
- [ ] Integration tests for complex scenarios
- [ ] Documentation with examples
- [ ] Clean git history for PR

### Nice to Have 🎁
- [ ] Stdlib convenience loader
- [ ] Performance benchmarks
- [ ] Migration guide for existing users

---

## Next Steps

**Ready to implement!** Proceed to Phase 3.

Choose one:
1. **Start implementation now** - I'll create the `SourceMap` type and begin coding
2. **Review design first** - Discuss any concerns or modifications
3. **Create RFC for Move team** - Get feedback before coding

Which would you prefer?
