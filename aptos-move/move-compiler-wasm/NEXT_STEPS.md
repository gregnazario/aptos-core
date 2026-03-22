# Next Steps: Filesystem-Free Move Compiler

**Current Status**: Steps 1-2 Complete (40% Done)
**Time Invested**: ~4 hours
**Time Remaining**: ~1-2 days for complete implementation

---

## ✅ What's Working Now

### Step 1: SourceMap Type ✅
**File**: `third_party/move/move-compiler-v2/src/sources.rs`

In-memory source management is fully functional:
```rust
let mut sources = SourceMap::new();
sources.add_file("MyModule.move", "module 0x1::Test { ... }");
```

### Step 2: Parser Function ✅
**File**: `third_party/move/move-compiler-v2/legacy-move-compiler/src/parser/mod.rs`

Can parse Move code from strings:
```rust
let targets = sources.to_files_source_text();
let (files, pprog_res) = parse_program_from_sources(env, maps, index, targets, deps)?;
```

**Commits**:
- `73eac319b3` - Added SourceMap and parser function
- `40d61ebffb` - Completed parse_program_from_sources

---

## 🔄 What's Next

### Step 3: Compiler Entry Point (Required)

**Challenge**: The compiler pipeline requires `GlobalEnv` from move-model, but `run_model_builder_in_compiler_mode()` expects file paths.

**Solution Options**:

#### Option A: Extend move-model API (Clean, 1 day)
Add new entry point to move-model:
```rust
// File: third_party/move/move-model/src/builder/model_builder.rs

pub fn run_model_builder_from_ast(
    targets: parser::ast::Program,
    comment_map: CommentMap,
    files: FilesSourceText,
    named_addresses: BTreeMap<String, AccountAddress>,
    ...options
) -> anyhow::Result<GlobalEnv> {
    // Skip file reading, use provided AST directly
    // Continue with expansion, typing, etc.
}
```

Then wire it up:
```rust
// File: move-compiler-v2/src/lib.rs

pub fn run_move_compiler_from_sources(
    emitter: &mut impl Emitter,
    sources: SourceMap,
    deps: SourceMap,
    named_addresses: BTreeMap<String, AccountAddress>,
    options: CompilerOptions,
) -> anyhow::Result<(GlobalEnv, Vec<AnnotatedCompiledUnit>)> {
    // 1. Parse from sources
    let (files, pprog_res) = parse_program_from_sources(...)?;
    let (pprog, comments) = pprog_res?;

    // 2. Build model from AST (new function)
    let env = run_model_builder_from_ast(pprog, comments, files, ...)?;

    // 3. Continue with existing pipeline
    env_check_and_transform_pipeline(&options).run(&mut env);
    let targets = run_stackless_bytecode_gen(&env);
    // ... rest of pipeline

    Ok((env, annotated_units))
}
```

**Pros**: Proper architecture, reusable
**Cons**: Requires understanding move-model internals
**Time**: 1 day

#### Option B: Simplified Single-Module Compiler (Quick, 3-4 hours)
Create lightweight path for WASM use case:

```rust
// File: move-compiler-wasm/src/compiler_direct.rs

pub fn compile_single_module_direct(
    source: String,
    address: String,
    module_name: String,
) -> CompilationResult {
    // 1. Parse
    let mut sources = SourceMap::new();
    sources.add_file(format!("{}.move", module_name), source);
    let (files, pprog_res) = parse_program_from_sources(...)?;

    // 2. Expand AST
    let expanded = expand_program(pprog, &mut env)?;

    // 3. Type check
    let typed = type_check(expanded, &mut env)?;

    // 4. Generate bytecode
    let bytecode = generate_bytecode(typed)?;

    Ok(CompilationResult::new_success(bytecode, vec![]))
}
```

**Pros**: Works quickly for WASM
**Cons**: Limited functionality, not general-purpose
**Time**: 3-4 hours

#### Option C: Monkey-Patch for Now (Hacky, 1-2 hours)
Temporarily write sources to a virtual filesystem that move-model can read:

```rust
// Create in-memory filesystem that implements required traits
struct MemoryFileSystem { ... }

// Inject it somehow (fragile)
```

**Pros**: Fastest
**Cons**: Fragile, not maintainable
**Time**: 1-2 hours

**Recommendation**: **Option A** for production quality, **Option B** if you need WASM working today.

---

## Implementation Guide: Option A (Recommended)

### 3.1: Study move-model Builder (30 mins)
```bash
cd third_party/move/move-model
rg "run_model_builder_in_compiler_mode" -A 50
```

Understand how it:
- Takes PackageInfo (with file paths)
- Parses files (calls `parse_program`)
- Expands AST
- Type checks
- Builds GlobalEnv

### 3.2: Add run_model_builder_from_ast (2-3 hours)
**File**: `move-model/src/builder/model_builder.rs`

```rust
pub fn run_model_builder_from_ast(
    targets: parser::ast::Program,
    comment_map: CommentMap,
    files: FilesSourceText,
    named_addresses: BTreeMap<Symbol, NumericalAddress>,
    skip_attribute_checks: bool,
    known_attributes: &BTreeSet<String>,
    language_version: LanguageVersion,
) -> anyhow::Result<GlobalEnv> {
    // Initialize environment
    let mut env = GlobalEnv::new();
    env.add_source_files(files);

    // Skip parsing - we already have AST
    // Continue with expansion
    let expanded = expansion::ast_check_and_expand(
        &mut env,
        targets,
        known_attributes,
    )?;

    // Continue with typing
    let typed = typing::type_check(
        &mut env,
        expanded,
    )?;

    // Populate env with typed definitions
    populate_env_from_typed_program(&mut env, typed)?;

    Ok(env)
}
```

### 3.3: Add Compiler Entry Point (2 hours)
**File**: `move-compiler-v2/src/lib.rs`

```rust
pub fn run_move_compiler_from_sources(
    emitter: &mut impl Emitter,
    sources: SourceMap,
    deps: SourceMap,
    named_addresses: BTreeMap<String, AccountAddress>,
    options: CompilerOptions,
) -> anyhow::Result<(GlobalEnv, Vec<AnnotatedCompiledUnit>)> {
    // Convert addresses
    let addr_map: BTreeMap<Symbol, NumericalAddress> = named_addresses
        .into_iter()
        .map(|(k, v)| (Symbol::from(k), v.into()))
        .collect();

    // Setup named address maps
    let mut maps = NamedAddressMaps::new();
    let map_idx = maps.insert(addr_map.clone());

    // Parse sources
    let targets = sources.to_files_source_text();
    let deps_files = deps.to_files_source_text();

    let (files, pprog_res) = parse_program_from_sources(
        &mut CompilationEnv::new(Flags::empty(), BTreeSet::new()),
        maps,
        map_idx,
        targets,
        deps_files,
    )?;

    let (pprog, comments) = pprog_res.map_err(|diags| {
        for diag in diags.into_iter() {
            emitter.emit(&files, &diag);
        }
        anyhow!("Parse errors")
    })?;

    // Build model from AST
    let mut env = run_model_builder_from_ast(
        pprog,
        comments,
        files,
        addr_map,
        options.skip_attribute_checks,
        &options.known_attributes,
        options.language_version.unwrap_or_default(),
    )?;

    // Continue with existing pipeline (same as run_move_compiler)
    env_check_and_transform_pipeline(&options).run(&mut env);
    check_errors(&env, emitter, "env checking errors")?;

    let mut targets = run_stackless_bytecode_gen(&env);
    check_errors(&env, emitter, "bytecode generation errors")?;

    run_stackless_bytecode_pipeline(&env, stackless_bytecode_check_pipeline(&options), &mut targets);
    check_errors(&env, emitter, "bytecode checks failed")?;

    env_optimization_pipeline(&options).run(&mut env);
    let mut targets = run_stackless_bytecode_gen(&env);

    run_stackless_bytecode_pipeline(&env, stackless_bytecode_optimization_pipeline(&options), &mut targets);
    check_errors(&env, emitter, "bytecode optimization errors")?;

    let modules_and_scripts = run_file_format_gen(&mut env, &targets);
    check_errors(&env, emitter, "assembling errors")?;

    let annotated_units = annotate_units(modules_and_scripts);
    run_bytecode_verifier(&annotated_units, &mut env);
    check_errors(&env, emitter, "verification errors")?;

    env.set_compiler_v2(true);
    Ok((env, annotated_units))
}
```

### 3.4: Update WASM Bindings (2 hours)
**File**: `aptos-move/move-compiler-wasm/src/compiler.rs`

```rust
use move_compiler_v2::sources::SourceMap;
use move_compiler_v2::run_move_compiler_from_sources;

pub fn compile_module(source: String, address: String, module_name: String) -> CompilationResult {
    // Create sources
    let mut sources = SourceMap::new();
    sources.add_file(format!("{}.move", module_name), source);

    // Parse address
    let addr = AccountAddress::from_hex_literal(&address)?;
    let named_addr = extract_address_name(&source).unwrap_or(Symbol::from("default"));

    let named_addresses = [(named_addr.to_string(), addr)].into();

    // Compile!
    let mut emitter = StringEmitter::new();
    let (env, units) = run_move_compiler_from_sources(
        &mut emitter,
        sources,
        SourceMap::new(), // No deps
        named_addresses,
        CompilerOptions::default(),
    )?;

    // Extract bytecode
    let bytecode = serialize_units(units)?;
    Ok(CompilationResult::new_success(bytecode, vec![]))
}
```

### 3.5: Test (1 day)
```bash
# Unit tests
cargo test -p move-model run_model_builder_from_ast
cargo test -p move-compiler-v2 run_move_compiler_from_sources

# Integration test
cargo test -p move-compiler-wasm

# WASM test
cd aptos-move/move-compiler-wasm
wasm-pack build --target web --release
wasm-pack test --firefox --headless
```

### 3.6: Document (2 hours)
- Rustdoc comments
- Usage examples
- Update README

---

## Implementation Guide: Option B (Quick Win)

If you need WASM working today, create a simplified path:

### File: `move-compiler-wasm/src/compiler_simple.rs`
```rust
// Simplified compiler for single modules (no dependencies)
// Uses our SourceMap and parse_program_from_sources
// Implements minimal pipeline for WASM use case

pub fn compile_single_module_simple(
    source: String,
    address: AccountAddress,
) -> Result<Vec<u8>, String> {
    // 1. Parse
    let mut sources = SourceMap::new();
    sources.add_file("main.move", source);

    let (files, pprog) = parse_program_from_sources(...)?;

    // 2. Minimal expansion (no full model)
    // 3. Minimal type checking
    // 4. Bytecode generation

    Ok(bytecode)
}
```

**Then update later** when Option A is complete.

---

## Timeline

### Option A (Recommended)
- **Day 1**: Implement run_model_builder_from_ast, add compiler entry point
- **Day 2**: Update WASM bindings, test, document
- **Total**: 2 days for complete, production-ready solution

### Option B (Quick Win)
- **Today**: 3-4 hours for working WASM compiler (limited functionality)
- **Later**: Upgrade to Option A when ready

### Combined Approach
- **Today**: Option B (get WASM working)
- **This Week**: Option A (proper implementation)
- **Total**: Best of both worlds

---

## Questions to Answer

1. **Do you need WASM working today?**
   - Yes → Start with Option B
   - No → Go straight to Option A

2. **Will you contribute upstream?**
   - Yes → Definitely Option A (proper architecture)
   - No → Option B might be sufficient

3. **How important is full feature support?**
   - Critical → Option A
   - Just need basic compilation → Option B

---

## What I Can Do Next

**If you say "continue"**, I'll:
1. Start implementing Option A (run_model_builder_from_ast)
2. Work through the compiler entry point
3. Get to a working end-to-end implementation

**If you say "quick win"**, I'll:
1. Create Option B (simplified single-module compiler)
2. Get WASM compiler working in browsers today
3. Document limitations

**If you say "checkpoint"**, I'll:
1. Clean up and document current state
2. Create detailed implementation guide
3. Let you decide timing

Which would you prefer?
