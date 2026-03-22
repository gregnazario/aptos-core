# Implementation Status: In-Memory Source Compilation

**Date**: 2026-03-22
**Goal**: Enable Move compiler to work without filesystem (for WASM/browsers)

---

## ✅ Completed

### Step 1: SourceMap Type (100% Complete)
**File**: `third_party/move/move-compiler-v2/src/sources.rs`

**Status**: ✅ Fully implemented, tested, and compiling

**Features**:
- `SourceMap` type for managing in-memory sources
- Clean API: `add_file(path, content)`, `get_file()`, `contains()`, etc.
- Comprehensive unit tests (11 tests, all passing)
- BTreeMap for deterministic ordering
- Converts to internal `FilesSourceText` format

**Example**:
```rust
use move_compiler_v2::sources::SourceMap;

let mut sources = SourceMap::new();
sources.add_file("MyModule.move", r#"
    module 0x1::MyModule {
        public fun hello(): u64 { 42 }
    }
"#);
```

### Step 2: Parser Function (100% Complete)
**File**: `third_party/move/move-compiler-v2/legacy-move-compiler/src/parser/mod.rs`

**Status**: ✅ Fully implemented and compiling (no warnings)

**Features**:
- `parse_program_from_sources()` function added
- Accepts in-memory sources instead of file paths
- Uses existing `parse_file_string()` for actual parsing
- Handles targets and dependencies separately
- Checks for duplicate files (by content hash)

**Signature**:
```rust
pub fn parse_program_from_sources(
    compilation_env: &mut CompilationEnv,
    named_address_maps: NamedAddressMaps,
    named_address_map_index: NamedAddressMapIndex,
    targets: FilesSourceText,
    deps: FilesSourceText,
) -> anyhow::Result<(
    FilesSourceText,
    Result<(parser::ast::Program, CommentMap), Diagnostics>,
)>
```

**Example Usage**:
```rust
use move_compiler_v2::sources::SourceMap;

let mut sources = SourceMap::new();
sources.add_file("test.move", "module 0x1::Test {}");

let targets = sources.to_files_source_text();
let deps = SourceMap::new().to_files_source_text();

let (files, result) = parse_program_from_sources(
    &mut env,
    named_address_maps,
    map_index,
    targets,
    deps,
)?;
```

---

## 🔄 In Progress

### Step 3: Compiler Entry Point (10% Complete)
**File**: `third_party/move/move-compiler-v2/src/lib.rs`

**Status**: 🔄 Design phase - need to integrate with model builder

**Challenge**:
The existing `run_move_compiler()` function calls `run_checker()` which uses `move_model::run_model_builder_in_compiler_mode()`. This function expects file paths in `PackageInfo.sources`.

**Options**:

#### Option A: Extend move-model API (Preferred)
Add new function to move-model that accepts parsed AST:
```rust
pub fn run_model_builder_from_ast(
    parsed_program: parser::ast::Program,
    comment_map: CommentMap,
    files_source_text: FilesSourceText,
    ...
) -> anyhow::Result<GlobalEnv>
```

**Pros**: Clean, proper layering
**Cons**: Requires changes to move-model

#### Option B: Bypass model builder for simple cases
For WASM use case (single module, no dependencies), create simplified path:
```rust
pub fn compile_single_module_from_source(
    source: String,
    address: AccountAddress,
    module_name: String,
) -> anyhow::Result<Vec<u8>>
```

**Pros**: Works immediately for WASM
**Cons**: Limited functionality, not general-purpose

#### Option C: Create in-memory FileProvider
Implement trait/interface that model builder uses for file access:
```rust
trait FileProvider {
    fn read_file(&self, path: &str) -> Result<String>;
    fn list_files(&self, pattern: &str) -> Vec<String>;
}
```

**Pros**: Minimal changes to existing code
**Cons**: More invasive, need to thread through all layers

**Recommendation**: Start with **Option B** for immediate WASM needs, then pursue **Option A** for general solution.

---

## 📋 Remaining Work

### Step 4: WASM Integration (Not Started)
**File**: `aptos-move/move-compiler-wasm/src/compiler.rs`

**Tasks**:
- Remove all filesystem code (`std::fs::*`)
- Update to use `SourceMap`
- Use new compiler entry point
- Test in Node.js and browsers

**Estimated**: 4 hours

### Step 5: Documentation (Not Started)
**Tasks**:
- Add rustdoc examples
- Update README files
- Create migration guide
- Document limitations

**Estimated**: 4 hours

### Step 6: Testing (Not Started)
**Tasks**:
- Unit tests for new functions
- Integration tests
- WASM tests (Node, Firefox, Chrome)
- Regression tests

**Estimated**: 1-2 days

---

## Quick Win: Simplified WASM Implementation

For immediate WASM needs, we can create a simplified implementation:

```rust
// File: aptos-move/move-compiler-wasm/src/compiler_simple.rs

use move_compiler_v2::sources::SourceMap;
use legacy_move_compiler::parser::parse_program_from_sources;

pub fn compile_module_simple(
    source: String,
    address: String,
    module_name: String,
) -> CompilationResult {
    // 1. Create SourceMap
    let mut sources = SourceMap::new();
    sources.add_file(format!("{}.move", module_name), source);

    // 2. Parse
    let targets = sources.to_files_source_text();
    let (files, pprog_res) = parse_program_from_sources(
        &mut env,
        maps,
        map_index,
        targets,
        empty_deps,
    )?;

    let (pprog, comments) = pprog_res?;

    // 3. Continue with expansion, typing, bytecode gen
    // (This is where we'd need to bypass or extend the model builder)

    // 4. Serialize bytecode
    Ok(CompilationResult::new_success(bytecode, vec![]))
}
```

**This approach**:
- ✅ Uses our new `SourceMap` (Step 1)
- ✅ Uses our new parser function (Step 2)
- ⚠️ Still needs integration with rest of pipeline (Step 3)

---

## Technical Achievements

### What Works Now
1. ✅ **In-memory source management** - SourceMap fully functional
2. ✅ **String-based parsing** - Can parse Move code from strings
3. ✅ **No filesystem dependencies** - Parser layer is filesystem-free
4. ✅ **Backward compatible** - All existing APIs unchanged

### What's Missing
1. ❌ **Model builder integration** - Need to extend or bypass
2. ❌ **Complete compilation pipeline** - Parse → Bytecode
3. ❌ **WASM bindings update** - Still using old filesystem-based code
4. ❌ **Comprehensive tests** - Only unit tests for SourceMap

---

## Commits

**Commit 1**: `73eac319b3`
- Added `SourceMap` type
- Added `parse_program_from_sources()` function
- Planning documents (PHASE1, PHASE2, DEPLOYMENT_PLAN)

---

## Next Steps

### Immediate (Today)
1. **Option B Quick Win**: Create simplified compiler for WASM
   - Bypass model builder for single-module case
   - Get WASM compiler working in browsers
   - **Time**: 2-3 hours

2. **Commit progress**: Commit Step 3 partial implementation

### Short-term (This Week)
3. **Extend move-model**: Add `run_model_builder_from_ast()`
4. **Complete Step 3**: Full compiler entry point
5. **Update WASM bindings**: Remove filesystem code
6. **Test in browsers**: Verify true browser-native compilation

### Long-term (2-3 weeks)
7. **Comprehensive testing**: Unit, integration, WASM tests
8. **Documentation**: Rustdoc, examples, guides
9. **Upstream PR**: Submit to move-language/move
10. **Community feedback**: Iterate based on reviews

---

## Key Insights

1. **Parser already supports strings** ✅ - Biggest technical risk eliminated
2. **Minimal changes needed** ✅ - Just plumbing, not rewriting
3. **Backward compatible** ✅ - No breaking changes
4. **Model builder is bottleneck** ⚠️ - Need to extend or bypass

**Bottom line**: The hard part (string-based parsing) is done. The remaining work is integration.

---

## Decision Point

**Do you want to**:

**A. Quick Win** - Get WASM compiler working today with simplified implementation
  - ✅ Works in browsers immediately
  - ✅ Removes filesystem dependency
  - ⚠️ Limited to single modules initially
  - **Time**: 2-3 hours

**B. Complete Implementation** - Finish full general-purpose API
  - ✅ Supports all use cases
  - ✅ Proper architecture
  - ⚠️ Requires more time
  - **Time**: 2-3 days

**C. Both** - Quick win first, then complete implementation
  - ✅ Unblocked immediately
  - ✅ Proper solution long-term
  - **Time**: 2-3 hours + 2-3 days

**Recommendation**: **Option C** - Get WASM working today, then polish for upstream contribution.
