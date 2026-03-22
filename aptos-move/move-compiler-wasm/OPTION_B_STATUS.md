# Option B Status: Quick WASM Fix

**Goal**: Get WASM compiler working in browsers without full move-model rework
**Reality Check**: Not fully achievable without Option A

---

## The Problem

After implementing Steps 1-2 (SourceMap and parse_program_from_sources), we discovered:

**The legacy compiler's full pipeline requires filesystem:**
```rust
Compiler::from_files(paths, deps, addrs, flags, attrs)
    .build()  // This internally parses files from paths
    -> Result<(FilesSourceText, Vec<CompiledUnit>)>
```

Even though we can now parse from strings using `parse_program_from_sources()`, the **rest of the compilation pipeline** (expansion, typing, bytecode generation) is encapsulated in the `Compiler` object which must be constructed via `from_files()` or `from_package_paths()`.

---

## What We Tried

### Attempt 1: Use parse_program_from_sources then continue pipeline
**Problem**: The parsed AST (parser::ast::Program) can't be fed directly into the rest of the legacy compiler. The expansion/typing stages aren't exposed as standalone functions.

### Attempt 2: Call individual pipeline stages
**Problem**: The pipeline stages (expansion, typing, bytecode_gen) are tightly coupled to the Compiler struct and CompilationEnv. They're not designed to be called independently.

### Attempt 3: Create simplified single-module compiler
**Problem**: Would need to re-implement expansion, typing, and bytecode generation - essentially duplicating the compiler.

---

## Current State: Hybrid Approach

### What Works Now
```rust
// ✅ Can manage sources in memory
let mut sources = SourceMap::new();
sources.add_file("test.move", source);

// ✅ Can parse from strings
let (files, pprog) = parse_program_from_sources(...)?;

// ❌ But then we still need files for the rest:
std::fs::write("/tmp/file.move", source)?;  // Still needed!
Compiler::from_files(vec!["/tmp/file.move"], ...)?;
```

---

## The Honest Truth

**Option B (quick WASM fix) is not possible without Option A.**

Here's why:
1. **Parse layer**: ✅ Filesystem-free (we did this!)
2. **Expansion/typing/bytecode layers**: ❌ Still need Compiler object
3. **Compiler construction**: ❌ Requires file paths

**To truly remove filesystem dependency, we must:**
- Extend move-model to accept parsed AST (Option A)
- OR reimplement the entire compiler pipeline (not feasible)
- OR wait for upstream Move team to add this feature

---

## Decision: Skip Option B, Do Option A Properly

**Option B is a dead end.** The "quick fix" doesn't exist because the compiler architecture requires deeper changes.

**Let's do Option A right:**
1. Extend move-model with `run_model_builder_from_ast()`
2. Add `run_move_compiler_from_sources()` entry point
3. Get true filesystem-free compilation

**Time**: 1-2 days (same as originally estimated)
**Result**: Actually works, production-ready, upstreamable

---

## Revised Plan

### ~~Option B: Quick Fix~~
~~Get WASM working today~~
**Status**: ❌ Not feasible without full integration

### Option A: Complete Implementation
Extend move-model, complete Step 3
**Status**: 🔄 Starting now
**Timeline**: 1-2 days

---

## Lessons Learned

1. **Parser layer is easy**: ✅ Done (Steps 1-2)
2. **Model layer is the blocker**: ⚠️ Need to extend move-model
3. **No shortcuts**: Can't bypass the architecture

**Bottom line**: We need to do the work properly. Option A is the only path forward.

---

## Next: Start Option A

See `NEXT_STEPS.md` Option A implementation guide.

Starting with:
1. Study move-model builder internals
2. Add `run_model_builder_from_ast()` function
3. Wire up complete pipeline
