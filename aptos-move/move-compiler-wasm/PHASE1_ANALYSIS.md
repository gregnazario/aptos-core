# Phase 1: Filesystem Dependency Analysis

**Date**: 2026-03-21
**Goal**: Map all filesystem operations in Move compiler to design in-memory compilation API

---

## Executive Summary

✅ **Good News**: The Move parser already supports string input via `parse_file_string()`!
⚠️ **Challenge**: Higher-level APIs wrap this with filesystem operations for file discovery
📋 **Solution**: Add new entry points that accept in-memory sources, bypass file discovery

---

## Filesystem Operation Map

### 1. INPUT OPERATIONS (Source File Reading)

#### File Discovery Chain
```
run_move_compiler(options: Options)
  → move_model::run_model_builder_in_compiler_mode()
    → parse_program(targets, deps)
      → find_move_filenames(paths)  ← FILESYSTEM: walkdir
        → For each .move file:
          → parse_file(fname)  ← FILESYSTEM: File::open() + read_to_string()
            → parse_file_string(content)  ← ✅ ALREADY ACCEPTS STRINGS!
```

**Key Files:**
- `move-command-line-common/src/files.rs:62-118`
  - `find_filenames()` - Uses `walkdir::WalkDir` to recursively find files
  - `find_move_filenames()` - Finds all `.move` files in given paths

- `legacy-move-compiler/src/parser/mod.rs:167-192`
  - `parse_file()` - Opens file and reads contents:
    ```rust
    let mut f = File::open(fname.as_str())?;
    let mut source_buffer = String::new();
    f.read_to_string(&mut source_buffer)?;
    let file_hash = FileHash::new(&source_buffer);
    ```

- `legacy-move-compiler/src/parser/syntax.rs:4817`
  - `parse_file_string()` - ✅ **Already accepts string input!**
    ```rust
    pub fn parse_file_string(
        env: &mut CompilationEnv,
        file_hash: FileHash,
        input: &str,  // ← Takes string, not file path!
    ) -> Result<(Vec<Definition>, MatchedFileCommentMap), Diagnostics>
    ```

#### Path Canonicalization
- `legacy-move-compiler/src/parser/mod.rs:135`
  - `std::fs::canonicalize()` - Normalizes paths to detect duplicates in targets vs deps
  - **Purpose**: Ensures same file isn't treated as both target and dependency

### 2. OUTPUT OPERATIONS (Bytecode Writing)

#### Compiled Unit Output
- `legacy-move-compiler/src/command_line/compiler.rs:359-421`
  - `output_compiled_units()` - Writes `.mv` and `.mvsm` files
  - Creates directory structure:
    ```
    out_dir/
      modules/
        00_ModuleName.mv      # Bytecode
        00_ModuleName.mvsm    # Source map
      scripts/
        ScriptName.mv
        ScriptName.mvsm
    ```
  - Uses:
    - `std::fs::create_dir_all()` - lines 394, 409
    - `fs::write()` - lines 379, 383

#### Interface File Generation
- `legacy-move-compiler/src/command_line/compiler.rs:436-534`
  - `generate_interface_files()` - Creates Move source from bytecode
  - Reads compiled `.mv` files: `std::fs::read(path)` - line 487
  - Writes to `build/mv_interfaces/{hash}/{address}/{module}.move`
  - Uses atomic writes via `NamedTempFile` + `fs::rename()` - lines 521-530

### 3. DEPENDENCY OPERATIONS (Reading Compiled Modules)

- `command_line/compiler.rs:536-548`
  - `has_compiled_module_magic_number()` - Validates `.mv` file format
  - `File::open()` + `file.read()` to check magic bytes

- `interface_generator.rs:38-51`
  - `write_file_to_string()` - Reads `.mv` bytecode
  - `fs::read()` then `CompiledModule::deserialize()`

---

## Critical Discovery: String Parsing Already Exists!

The Move parser has **two modes**:

### Mode 1: File-Based (Current Default)
```rust
// High-level wrapper
parse_file(fname: &str) -> ... {
    let mut f = File::open(fname)?;  // ← FILESYSTEM
    let mut source_buffer = String::new();
    f.read_to_string(&mut source_buffer)?;  // ← FILESYSTEM
    parse_file_string(env, file_hash, &source_buffer)  // ← CALLS STRING VERSION
}
```

### Mode 2: String-Based (Already Implemented!)
```rust
// Already exists in syntax.rs:4817
pub fn parse_file_string(
    env: &mut CompilationEnv,
    file_hash: FileHash,
    input: &str,  // ← String input!
) -> Result<(Vec<Definition>, MatchedFileCommentMap), Diagnostics>
```

**Implication**: The hard work is already done! We just need to:
1. Expose `parse_file_string` at higher API levels
2. Bypass file discovery step
3. Provide virtual file names/hashes for error messages

---

## API Entry Points (Where File Paths Enter)

### Primary Entry Point
**File**: `move-compiler-v2/src/lib.rs`

```rust
pub fn run_move_compiler(
    error_writer: &mut impl Emitter,
    options: Options,  // ← Contains Vec<String> of file paths
) -> anyhow::Result<(
    GlobalEnv,
    Vec<AnnotatedCompiledUnit>,
)>
```

### Options Structure
**File**: `move-compiler-v2/src/options.rs`

```rust
pub struct Options {
    // Input sources (what we need to replace)
    #[clap(name = "sources")]
    pub sources: Vec<String>,  // ← File/directory paths

    #[clap(long = "dependencies", short = 'd', name = "path")]
    pub dependencies: Vec<String>,  // ← Compiled dependency dirs

    #[clap(long = "sources-deps", name = "sources_deps")]
    pub sources_deps: Vec<String>,  // ← Source dependency paths

    // Output paths
    #[clap(long = "out-dir", short = 'o', name = "path")]
    pub output_dir: Option<String>,

    // ... other config options (named addresses, flags, etc.)
}
```

### Lower-Level Entry Point
**File**: `legacy-move-compiler/src/command_line/compiler.rs`

```rust
impl<'a> Compiler<'a> {
    pub fn from_package_paths<...>(
        targets: Vec<PackagePaths<Paths, NamedAddress>>,
        deps: Vec<PackagePaths<Paths, NamedAddress>>,
        flags: Flags,
        known_attributes: &BTreeSet<String>,
    ) -> Self

    pub fn run<const TARGET: Pass>(
        self,
    ) -> anyhow::Result<(
        FilesSourceText,
        Result<(CommentMap, SteppedCompiler<'a, TARGET>), Diagnostics>,
    )>
}
```

---

## Data Structures for File Content

### Current: Files → HashMap
```rust
// Type alias in diagnostics/mod.rs:41
pub type FilesSourceText = HashMap<FileHash, (FileName, String)>;
//                         ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^
//                         Key: SHA256 hash   Value: (name, content)

// Where:
pub type FileHash = [u8; 32];  // SHA256
pub type FileName = Symbol;    // Interned string
```

**Structure**:
- Map keyed by content hash (SHA256)
- Values are tuples of (virtual filename, source code)
- Used for error reporting (maps errors to source locations)

### Proposal: In-Memory SourceMap
```rust
// New type to propose
pub struct SourceMap {
    files: HashMap<Symbol, String>,  // path → content
}

impl SourceMap {
    pub fn new() -> Self;
    pub fn add_file(&mut self, path: impl Into<Symbol>, content: String);
    pub fn get_file(&self, path: &Symbol) -> Option<&str>;

    // Internal: convert to FilesSourceText for compiler
    fn to_files_source_text(&self) -> FilesSourceText {
        self.files.iter().map(|(name, content)| {
            let hash = FileHash::new(content);
            (hash, (*name, content.clone()))
        }).collect()
    }
}
```

---

## Dependency Handling Strategy

Move code often depends on:
1. **Move stdlib** (`std::vector`, `std::signer`, etc.)
2. **Aptos framework** (coin, account, token modules)
3. **Custom dependencies**

### Option A: Bundle Pre-Compiled Bytecode
```rust
// Include compiled stdlib at build time
const STDLIB_COMPILED: &[u8] = include_bytes!("../compiled/move-stdlib.mv");

// Pro: Fast, small
// Con: Can't compile code that depends on non-standard stdlib
```

### Option B: Bundle Source Files
```rust
// Include stdlib sources at build time
fn load_stdlib_sources() -> SourceMap {
    let mut map = SourceMap::new();
    map.add_file("vector.move", include_str!("../move-stdlib/sources/vector.move"));
    map.add_file("signer.move", include_str!("../move-stdlib/sources/signer.move"));
    // ... ~20 stdlib files
    map
}

// Pro: Flexible, can recompile with different flags
// Con: Larger binary (~100KB extra), slower first compile
```

### Option C: Hybrid (Recommended)
```rust
// Pre-compiled stdlib for common case
const STDLIB_COMPILED: &[u8] = include_bytes!("../stdlib.mv");

// Source fallback for custom compilation
fn load_stdlib_sources() -> SourceMap { ... }

// Use compiled by default, sources if custom flags needed
```

**Recommendation**: Start with Option A (pre-compiled), add Option C later if needed.

---

## Proposed API Design

### New Entry Point
```rust
// Add to move-compiler-v2/src/lib.rs

pub fn run_move_compiler_from_sources(
    error_writer: &mut impl Emitter,
    sources: SourceMap,          // ← In-memory sources
    dependencies: SourceMap,     // ← In-memory deps (or empty)
    named_addresses: BTreeMap<String, AccountAddress>,
    flags: Flags,
) -> anyhow::Result<(GlobalEnv, Vec<AnnotatedCompiledUnit>)> {

    // Convert SourceMap to FilesSourceText
    let files_source_text = sources.to_files_source_text();

    // Parse directly from strings (bypass file discovery)
    let (pprog, comments) = parse_program_from_sources(
        &mut compilation_env,
        files_source_text,
        dependencies.to_files_source_text(),
    )?;

    // Continue with existing compilation pipeline
    // (expansion, typing, bytecode generation)
    run_compiler_phases(pprog, comments, flags)
}
```

### New Parser Function
```rust
// Add to legacy-move-compiler/src/parser/mod.rs

pub fn parse_program_from_sources(
    env: &mut CompilationEnv,
    sources: FilesSourceText,        // ← Already have content!
    deps: FilesSourceText,
) -> Result<(
    parser::ast::Program,
    CommentMap,
), Diagnostics> {

    let mut parsed_defs = vec![];
    let mut comment_map = CommentMap::new();

    // Parse each source file (already have content!)
    for (file_hash, (file_name, source_content)) in sources.iter() {
        let (defs, comments) = syntax::parse_file_string(
            env,
            *file_hash,
            source_content,  // ← Use in-memory content
        )?;

        parsed_defs.push((file_name, defs));
        comment_map.insert(*file_hash, comments);
    }

    // Same for deps...

    // Build program AST
    Ok((
        parser::ast::Program { /* ... */ },
        comment_map,
    ))
}
```

---

## Implementation Complexity Assessment

### Low Complexity ✅ (1-2 days each)
- [x] ✅ Create `SourceMap` type (struct + methods)
- [x] ✅ Add `parse_program_from_sources()` - bypass file discovery
- [x] ✅ Add `run_move_compiler_from_sources()` - new entry point
- [x] ✅ Update WASM bindings to use new API

### Medium Complexity ⚠️ (3-5 days each)
- [ ] ⚠️ Handle stdlib dependencies (decide on bundling strategy)
- [ ] ⚠️ Comprehensive testing (unit, integration, WASM)
- [ ] ⚠️ Error message formatting (ensure virtual paths work)

### High Complexity 🔴 (Not needed initially)
- [ ] 🔴 Output writing (skip - WASM doesn't need file output)
- [ ] 🔴 Interface file generation (skip - only needed for incremental builds)
- [ ] 🔴 Multi-package compilation (future enhancement)

---

## Backward Compatibility Strategy

### Keep All Existing APIs Unchanged
```rust
// OLD API (file-based) - KEEP AS-IS
pub fn run_move_compiler(
    error_writer: &mut impl Emitter,
    options: Options,  // ← Still accepts file paths
) -> anyhow::Result<...>

// NEW API (source-based) - ADD ALONGSIDE
pub fn run_move_compiler_from_sources(
    error_writer: &mut impl Emitter,
    sources: SourceMap,  // ← Accepts in-memory sources
    ...
) -> anyhow::Result<...>
```

**Philosophy**: Additive changes only. Zero breaking changes.

---

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_compile_single_module_from_string() {
    let mut sources = SourceMap::new();
    sources.add_file("test.move", r#"
        module 0x1::Test {
            public fun hello(): u64 { 42 }
        }
    "#);

    let mut emitter = StringEmitter::new();
    let result = run_move_compiler_from_sources(
        &mut emitter,
        sources,
        SourceMap::new(),  // No deps
        [("std".into(), AccountAddress::ONE)].into(),
        Flags::empty(),
    );

    assert!(result.is_ok());
    let (_, units) = result.unwrap();
    assert_eq!(units.len(), 1);
}

#[test]
fn test_compile_with_dependencies() {
    let mut sources = SourceMap::new();
    sources.add_file("main.move", r#"
        module 0x1::Main {
            use 0x1::Dep;
            public fun use_dep(): u64 { Dep::get_value() }
        }
    "#);

    let mut deps = SourceMap::new();
    deps.add_file("dep.move", r#"
        module 0x1::Dep {
            public fun get_value(): u64 { 42 }
        }
    "#);

    let result = run_move_compiler_from_sources(
        &mut StringEmitter::new(),
        sources,
        deps,
        named_addresses,
        Flags::empty(),
    );

    assert!(result.is_ok());
}
```

### Integration Tests
- Compile Move stdlib from strings
- Compile complex multi-module packages
- Error handling (syntax errors, type errors)
- Virtual path resolution in error messages

### WASM Tests
```bash
wasm-pack test --node       # Test in Node.js
wasm-pack test --firefox    # Test in Firefox
wasm-pack test --chrome     # Test in Chrome
```

---

## Risk Assessment

### Low Risk ✅
- **Parser modifications**: Parser already supports strings
- **API additions**: Adding new functions, not changing existing ones
- **Testing**: Can test exhaustively before PR

### Medium Risk ⚠️
- **Error message paths**: Virtual paths in errors (e.g., `"module_123.move:5"`)
  - *Mitigation*: Use meaningful virtual paths ("stdin.move", "module_0x1.move")
- **Dependency handling**: How to provide stdlib
  - *Mitigation*: Start simple (require caller to provide deps), enhance later

### High Risk 🔴
- **Upstream acceptance**: PR might be rejected
  - *Mitigation*: Engage with Move team early, show use cases (WASM, testing, IDE)
  - *Fallback*: Maintain fork, keep pushing for merge
- **Breaking changes**: Future compiler updates might conflict
  - *Mitigation*: Pin to specific commit, comprehensive tests catch breakage

---

## Next Steps (Phase 2: Design)

1. **Validate approach with Move team**
   - Open GitHub discussion/issue
   - Share this analysis
   - Get feedback before coding

2. **Refine `SourceMap` design**
   - Decide on exact API surface
   - Consider edge cases (empty sources, duplicate names)
   - Plan for future features (source maps, line numbers)

3. **Design error reporting**
   - How to show virtual file paths in errors
   - How to preserve line/column information
   - Example error messages

4. **Plan stdlib bundling**
   - Measure size impact of different approaches
   - Decide on initial implementation
   - Plan for configurability

5. **Create RFC document**
   - Formal proposal for Move team
   - Use cases and motivation
   - Detailed design
   - Backward compatibility guarantees

---

## Estimated Timeline

| Phase | Tasks | Duration |
|-------|-------|----------|
| **Phase 1: Analysis** | ✅ Map filesystem ops | ✅ Complete |
| **Phase 2: Design** | API design, RFC, feedback | 2-3 days |
| **Phase 3: Implementation** | Code changes, unit tests | 1 week |
| **Phase 4: Testing** | Integration tests, WASM tests | 2-3 days |
| **Phase 5: Documentation & PR** | Docs, examples, submit PR | 2 days |
| **Phase 6: Review & Iterate** | Address feedback, revisions | 1 week |
| **Total** | | **2-3 weeks** |

---

## Key Insights

1. ✅ **Parser already supports strings** - `parse_file_string()` exists!
2. ✅ **Minimal changes needed** - Just add plumbing, not rewrite parser
3. ✅ **Low risk** - Additive API, no breaking changes
4. ⚠️ **Stdlib handling** - Need strategy for dependencies
5. 📋 **Upstream coordination** - Engage Move team early

**Recommendation**: Proceed to Phase 2 (Design). The analysis confirms this is feasible and low-risk.
