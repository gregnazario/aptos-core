# Move Compiler WASM - Status Report

**Created:** 2026-03-20
**Status:** 🚧 **PROOF OF CONCEPT - BLOCKED**

---

## Summary

Initial implementation of Move compiler WASM bindings created, but blocked by compiler API complexity. The path forward is clear but requires additional work.

### What Exists

✅ **Crate Structure**
- Standalone `move-compiler-wasm` crate
- WASM build configuration
- Clean separation from native CLI
- TypeScript-friendly API design

✅ **API Design**
- `compile_module()` - Single module compilation
- `compile_script()` - Script compilation
- `MovePackage` - Multi-file package builder
- `CompilationResult` - JavaScript-friendly result type

✅ **Demo Infrastructure**
- Beautiful browser demo (`demo/index.html`)
- Interactive code editor
- Example Move programs
- Error display

✅ **Documentation**
- Comprehensive README
- API reference
- Usage examples

### What's Blocked

❌ **Compiler Integration**
**Blocker:** `move-compiler-v2` has a complex API that requires:
- `Options` configuration
- `GlobalEnv` setup
- Pipeline configuration
- Custom error emitter

The original `move-compiler` (simpler API) doesn't exist in this repo - only `move-compiler-v2`.

**Evidence:**
```
error: failed to load manifest for dependency `move-compiler`
Caused by:
  failed to read `.../third_party/move/move-compiler/Cargo.toml`
  No such file or directory
```

Only `move-compiler-v2` exists:
```bash
$ ls third_party/move/move-compiler*
third_party/move/move-compiler-v2
```

---

## Technical Analysis

### Compiler v2 API Complexity

The `move-compiler-v2` API requires:

```rust
use move_compiler_v2::{Options, run_move_compiler};
use move_model::model::GlobalEnv;
use move_compiler_v2::diagnostics::Emitter;

// 1. Configure options
let mut options = Options::default();
options.sources = vec![/* source files */];
options.named_address_mapping = /* addresses */;

// 2. Create error emitter
let mut emitter = /* custom emitter */;

// 3. Run compiler
let (env, units) = run_move_compiler(&mut emitter, options)?;

// 4. Extract bytecode from units
// ...
```

This is significantly more complex than the planned simple API:
```rust
compile_module(source_code, address, module_name)
```

### Dependencies Still Need Resolution

Even with v2 integration, we need to handle:
- ❌ `move-package` (filesystem dependencies: tempfile, walkdir)
- ❌ Potential `aptos-crypto` transitive dependencies
- ❌ Standard library resolution

---

## Path Forward

### Option A: Implement v2 Integration (Recommended)

**Timeline:** 2-3 weeks
**Complexity:** Medium-High

**Steps:**
1. **Week 1:** Study `move-compiler-v2` API
   - Read `third_party/move/move-compiler-v2/src/lib.rs`
   - Understand `Options` configuration
   - Implement custom `Emitter` for WASM

2. **Week 2:** Implement single-file compilation
   - Create in-memory source provider
   - Configure `Options` for WASM
   - Extract bytecode from `AnnotatedCompiledUnit`
   - Handle errors properly

3. **Week 3:** Testing and optimization
   - Verify WASM build works
   - Test with real Move code
   - Optimize for size
   - Document usage

**Benefits:**
- Uses production Move compiler
- Full feature set
- Future-proof

**Risks:**
- May encounter WASM incompatibilities deep in v2
- Could hit same blockers as evaluation (aptos-crypto, etc.)

### Option B: Use Legacy Compiler (If Available)

**Timeline:** 1 week
**Complexity:** Low

If the old `move-compiler` exists elsewhere, use it:
- Simpler API
- Fewer dependencies
- Faster integration

**Problem:** Not available in this repo

### Option C: Bytecode-Only Tools (MVP)

**Timeline:** 3 days
**Complexity:** Very Low

Focus on what works TODAY:
- ✅ Decompile bytecode (already working in `move-decompiler-wasm`)
- ✅ Disassemble bytecode
- ✅ Verify bytecode
- ✅ Extract metadata

**Compilation:** Use server-side API or native tooling

**Benefits:**
- Ships immediately
- Zero blockers
- Still valuable for web tools

---

## Recommendation

### Immediate (This Week)

**Ship What Works:**
1. Rename to `move-bytecode-tools-wasm`
2. Remove compilation APIs
3. Add decompiler + verifier
4. Publish as MVP

This gives users:
- Browser-based bytecode analysis
- Move learning tools
- Package verification tools

### Next Phase (1-2 Months)

**Add Compilation:**
1. Study move-compiler-v2 deeply
2. Create WASM-compatible wrapper
3. Test with simple modules
4. Gradually add features

---

## What We Learned

### Confirmed from Evaluation

✅ The WASM evaluation docs were RIGHT:
- Dependency architecture is the blocker
- Need separate crate (we have it)
- v2 API is complex
- 6-8 weeks realistic for full implementation

### New Insights

1. **No Legacy Compiler:** Only v2 exists, so we MUST use it
2. **API Mismatch:** V2 API is not WASM-friendly out of the box
3. **Emitter Challenge:** Need custom error emitter for browser
4. **Source Management:** In-memory source files need careful design

---

## Files Created

```
aptos-move/move-compiler-wasm/
├── Cargo.toml                 # WASM-compatible dependencies
├── .cargo/config.toml          # WASM build config
├── src/
│   ├── lib.rs                 # Main WASM bindings (API designed)
│   ├── compiler.rs            # Compilation logic (blocked)
│   ├── package.rs             # Package builder (skeleton)
│   └── error.rs               # Error types (complete)
├── demo/
│   └── index.html             # Beautiful browser demo (ready)
├── README.md                  # Comprehensive docs (complete)
└── STATUS.md                  # This file
```

---

## Next Steps

### Immediate Choice Required

**Option 1:** Pivot to bytecode-only tools (ships this week)
**Option 2:** Commit to v2 integration (ships in 2-3 weeks)
**Option 3:** Wait for upstream simplifications

### If Continuing with Compilation

1. **Study Phase:**
   ```bash
   cd third_party/move/move-compiler-v2
   cargo doc --open
   # Study Options, run_move_compiler, AnnotatedCompiledUnit
   ```

2. **Prototype:**
   ```rust
   // Create simplest possible v2 integration
   fn compile_simple(source: &str) -> Result<Vec<u8>> {
       let options = Options {
           sources: vec![(/* ... */)],
           // ... minimal config
       };
       let (env, units) = run_move_compiler(/* ... */)?;
       // Extract bytecode
   }
   ```

3. **Test:**
   ```bash
   cargo build --target wasm32-unknown-unknown
   # Fix each blocker as encountered
   ```

---

## Conclusion

**Status:** Feasible but requires dedicated effort

**Recommendation:** Start with bytecode tools (MVP), add compilation later

**Timeline:**
- MVP (bytecode tools): 3 days
- Full compiler: 2-3 weeks additional

**Value:** High - enables web-based Move development tools

---

**Related Documentation:**
- [WASM Evaluation](../cli/WASM_EVALUATION_README.md)
- [Phase 5 Recommendation](../cli/PHASE5_FINAL_RECOMMENDATION.md)
- [Move Decompiler WASM](../move-decompiler-wasm/README.md) (working reference)
