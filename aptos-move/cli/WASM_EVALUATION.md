# WASM Feasibility Evaluation for aptos-move-cli

**Date:** 2026-03-20
**Status:** Phase 1 Complete - Dependency Compatibility Analysis

---

## Executive Summary

This document tracks the evaluation of whether aptos-move-cli can be made WASM-compatible for browser/Node.js usage. The evaluation is structured in 5 phases over 3 weeks, with the goal of producing a Go/No-Go recommendation without disrupting existing native builds.

**Phase 1 Status: ⚠️ BLOCKERS IDENTIFIED** - Critical filesystem dependencies require abstraction

---

## Phase 1: Dependency Compatibility Analysis

### Testing Methodology

Attempted isolated WASM builds of critical dependencies using:
```bash
cargo build --target wasm32-unknown-unknown --lib
```

### Results Summary

| Dependency | Status | Blocker Details | Mitigation |
|------------|--------|-----------------|------------|
| **getrandom** | ⚠️ FIXABLE | Requires "js" feature for WASM | Add `getrandom = { workspace = true, features = ["js"] }` |
| **errno** | ❌ BLOCKER | No wasm32 sys module (crate doesn't support WASM) | Identify which deps use errno and find alternatives |
| **move-compiler-v2** | ⚠️ BLOCKED BY getrandom | Build stopped by getrandom error | Fix getrandom, retest |
| **move-binary-format** | ⚠️ BLOCKED BY getrandom | Build stopped by getrandom error | Fix getrandom, retest (likely ✅) |
| **move-package** | ❌ MAJOR BLOCKER | Multiple filesystem deps: tempfile, walkdir, named-lock, whoami | **Virtual filesystem abstraction required** |
| **move-linter** | ⚠️ BLOCKED BY errno | Uses move-package → inherit its blockers | Depends on move-package fix |
| **aptos-crypto** | ❌ BLOCKER | Build stopped early, likely has ring/blst native code | Not needed for core compilation commands |
| **aptos-rest-client** | ❌ BLOCKER | Network operations (not tested, known incompatible) | Not needed for core compilation |
| **tokio** | ❌ BLOCKER | Multi-threaded runtime (not tested, known incompatible) | Use wasm-bindgen-futures for async |
| **tempfile** | ❌ BLOCKER | Native filesystem operations | Virtual filesystem abstraction |

### Critical Finding: move-package Dependencies

The `move-package` crate is CRITICAL for Move CLI functionality (handles Move.toml, source files, package structure) but has significant filesystem dependencies:

```toml
# From third_party/move/tools/move-package/Cargo.toml
tempfile = { workspace = true }      # Native temp files
walkdir = { workspace = true }       # Directory traversal
named-lock = { workspace = true }    # File-based locking
whoami = { workspace = true }        # System user info
```

**Impact:** This is the biggest architectural challenge. All Move commands that work with packages (compile, lint, test, etc.) depend on move-package.

**Mitigation Path:** Virtual filesystem (VFS) abstraction layer (see Phase 4 design).

### errno Chain Analysis

The errno crate appears in dependency chains, causing build failures. Traced to:
- `clap` → terminal interaction crates → errno
- `walkdir` → filesystem operations → errno
- Various I/O crates in the dependency tree

**Mitigation:** May need to disable certain features or use WASM-compatible alternatives for CLI parsing.

### Reference: Successful WASM Crate

The `aptos-dynamic-transaction-composer` (script-composer) successfully builds to WASM with:

```toml
[dependencies]
getrandom = { workspace = true, features = ["js"] }  # ✅ Critical
wasm-bindgen = { workspace = true }                   # ✅ Required
wasm-bindgen-futures = { workspace = true }           # ✅ For async
move-binary-format = { workspace = true }             # ✅ Compatible
move-core-types = { workspace = true }                # ✅ Compatible

[lib]
crate-type = ["cdylib", "rlib"]  # ✅ Enables WASM output
```

**Key Takeaway:** Core Move types and binary format ARE WASM-compatible when configured correctly.

---

## Phase 2: Command Feasibility Classification

### Tier 1: Core Compilation (HIGH WASM VIABILITY) ✅

These commands only require Move compiler primitives and can work with virtual filesystem:

| Command | Feasibility | Dependencies | Notes |
|---------|-------------|--------------|-------|
| **Compile** | ✅ HIGH | move-compiler-v2, move-package | Requires VFS for reading sources |
| **CompileScript** | ✅ HIGH | move-compiler-v2 | Simpler than full package |
| **Lint** | ✅ HIGH | move-linter, move-compiler-v2 | Requires VFS |
| **Disassemble** | ✅ VERY HIGH | move-binary-format | Works on bytecode blobs (no filesystem) |
| **Decompile** | ✅ VERY HIGH | move-decompiler | Works on bytecode blobs |
| **VerifyPackage** | ✅ HIGH | move-bytecode-verifier | Pure verification logic |

**Recommended MVP:** Start with Disassemble + Decompile (no VFS needed), then add Compile + Lint (with VFS).

### Tier 2: Analysis Tools (MODERATE COMPLEXITY) ⚠️

| Command | Feasibility | Dependencies | Notes |
|---------|-------------|--------------|-------|
| **Document** | ⚠️ MEDIUM | move-docgen | Needs output handling |
| **Show** | ⚠️ MEDIUM | move-package | Read-only package info |
| **List** | ⚠️ MEDIUM | move-package | Directory traversal via VFS |
| **Prove** | ❌ LOW | move-prover | Likely has native dependencies (SMT solvers) |

### Tier 3: Filesystem Operations ❌

These directly manipulate filesystem - NOT WASM-compatible:

- **Init** - Creates directory structure
- **Clean** - Deletes build artifacts
- **Fmt** - Code formatting (may spawn external process)

**Verdict:** EXCLUDE from WASM scope

### Tier 4: Testing ❌

Multi-threading and process spawning - NOT WASM-compatible:

- **Test** - Runs unit tests (threads, process spawning)
- **Coverage** - Test coverage analysis

**Verdict:** EXCLUDE from WASM scope

### Tier 5: Network/Deployment ❌

Require blockchain interaction - NOT WASM-compatible (could be re-implemented with JS fetch):

- Publish, Deploy, Run, RunScript, Simulate, Replay, View, Download
- All object/resource account deployment commands

**Verdict:** EXCLUDE from initial WASM scope (potential future addition with JS network layer)

### WASM Scope Recommendation

**Minimal MVP (Phase 3 Prototype):** 2 commands
- Disassemble (no VFS needed)
- Decompile (no VFS needed)

**Extended MVP:** 4 additional commands (+VFS)
- Compile
- CompileScript
- Lint
- VerifyPackage

**Total WASM-Compatible Commands:** 6 of 25+ (24% of functionality)

**Value Proposition:** Core Move development workflow (write → compile → lint → verify) works in browser.

---

## Phase 3: Proof-of-Concept Plan

### Objectives
1. Build minimal WASM library with Disassemble command
2. Validate Move compiler WASM compatibility
3. Measure binary size
4. Confirm native builds unaffected

### Implementation Branch
```bash
git checkout -b wasm-eval-aptos-move-cli
```

### Minimal Configuration

**File:** `aptos-move/cli/Cargo.toml`
```toml
[lib]
name = "aptos_move_cli"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]  # Enable WASM output

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = { workspace = true }
getrandom = { workspace = true, features = ["js"] }
wasm-bindgen-futures = { workspace = true }

[features]
default = []
binary = ["testing"]
testing = ["aptos-vm/testing"]
wasm-eval = []  # Evaluation feature flag
```

### Minimal WASM API

**File:** `aptos-move/cli/src/wasm_api.rs`
```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn disassemble_bytecode(bytecode: Vec<u8>) -> Result<String, JsValue> {
    // Use move-binary-format to disassemble
    // Returns assembly text or error
}
```

### Success Criteria
- ✅ WASM build completes: `cargo build -p aptos-move-cli --target wasm32-unknown-unknown --lib --features wasm-eval`
- ✅ Native build works: `cargo test -p aptos-move-cli`
- ✅ Binary size < 50MB (before wasm-opt optimization)
- ✅ Can disassemble simple Move module bytecode

### Next Steps After Phase 3
- If successful: Proceed to Phase 4 (VFS design) for Compile command
- If blocked: Document root cause and evaluate alternatives

---

## Phase 4: Virtual Filesystem Design (PENDING)

**Status:** Not started - depends on Phase 3 success

**Challenge:** Move package system requires:
- Reading Move.toml manifest files
- Reading .move source files from sources/ directory
- Resolving dependencies
- Build artifact management

**Design Options:**
1. **VFS Trait** - Abstraction layer over std::fs
2. **Pre-compilation** - Distribute pre-compiled bytecode
3. **JavaScript Integration** - JS provides files to WASM

Details to be fleshed out in Phase 4 implementation.

---

## Phase 5: Risk Assessment (PENDING)

**Status:** Not started - depends on Phases 3-4

Will evaluate:
- Technical feasibility (Go/No-Go decision)
- Binary size optimization path
- Performance characteristics
- Implementation complexity/effort
- Maintenance burden

---

## Critical Blockers Summary

### 🔴 HIGH PRIORITY BLOCKERS

1. **move-package filesystem dependencies**
   - Impact: Affects all package-based commands
   - Mitigation: Virtual filesystem abstraction (complex, 2-3 weeks effort)
   - Confidence: Medium - VFS is architecturally sound but requires refactoring

2. **errno crate incompatibility**
   - Impact: Blocks many dependency chains
   - Mitigation: Identify and replace/disable features using errno
   - Confidence: Medium - may require upstream fixes or alternatives

3. **getrandom configuration**
   - Impact: Blocks ALL builds initially
   - Mitigation: Add "js" feature to workspace dependencies
   - Confidence: High - well-known, straightforward fix

### 🟡 MEDIUM PRIORITY CONCERNS

4. **Binary size**
   - Risk: WASM bundle may exceed 100MB
   - Mitigation: wasm-opt, code splitting, feature flags to exclude heavy deps
   - Confidence: Medium - won't know until Phase 3 prototype

5. **clap CLI parsing in WASM**
   - Risk: CLI argument parsing doesn't make sense in WASM context
   - Mitigation: Expose programmatic API instead of CLI interface
   - Confidence: High - this is expected and solvable

### 🟢 LOW PRIORITY / ACCEPTABLE LIMITATIONS

6. **Excluded commands** (test, network, filesystem ops)
   - Impact: 19+ commands won't work in WASM
   - Mitigation: Document as "native-only" features
   - Confidence: High - acceptable scope limitation

---

## Verification Testing

After each phase, run:

```bash
# MUST PASS: Native build and tests
cargo build -p aptos-move-cli
cargo test -p aptos-move-cli

# Phase 3+: WASM build
cargo build -p aptos-move-cli \
  --target wasm32-unknown-unknown \
  --lib \
  --no-default-features \
  --features wasm-eval

# Check binary size
ls -lh target/wasm32-unknown-unknown/debug/aptos_move_cli.wasm
wasm-opt -Oz target/wasm32-unknown-unknown/release/aptos_move_cli.wasm -o optimized.wasm
ls -lh optimized.wasm
```

---

## Next Actions

### Immediate (Phase 3)
1. ✅ Create evaluation branch: `wasm-eval-aptos-move-cli`
2. ⏳ Fix getrandom workspace dependency
3. ⏳ Add minimal WASM configuration to Cargo.toml
4. ⏳ Implement minimal Disassemble API
5. ⏳ Test WASM build and measure size

### If Phase 3 Succeeds (Phase 4)
6. Design VFS abstraction
7. Implement VFS for move-package
8. Add Compile command to WASM API
9. Test full compilation workflow

### If Phase 3 Fails
- Document specific blockers
- Evaluate alternative approaches:
  - Server-side compilation API
  - Move language server protocol (LSP)
  - Pre-compiled bytecode distribution

---

## Timeline

- **Week 1:** Phases 1-2 (Analysis) ✅ Phase 1 Complete
- **Week 2:** Phase 3 (Prototype) → In Progress
- **Week 3:** Phases 4-5 (Design + Recommendation) → Pending

**Next Milestone:** Working WASM build of Disassemble command by end of Week 2.

---

## Appendix: Testing Log

### Test 1: move-compiler-v2
```bash
cd third_party/move/move-compiler-v2
cargo build --target wasm32-unknown-unknown --lib
# Result: ❌ getrandom error (fixable)
```

### Test 2: move-binary-format
```bash
cd third_party/move/move-binary-format
cargo build --target wasm32-unknown-unknown --lib
# Result: ❌ getrandom error (fixable)
```

### Test 3: move-package
```bash
cd third_party/move/tools/move-package
cargo build --target wasm32-unknown-unknown --lib
# Result: ❌ getrandom + errno errors (errno is blocker)
```

### Test 4: move-linter
```bash
cd third_party/move/tools/move-linter
cargo build --target wasm32-unknown-unknown --lib
# Result: ❌ getrandom + errno errors (inherits move-package issues)
```

### Test 5: aptos-crypto
```bash
cd crates/aptos-crypto
cargo build --target wasm32-unknown-unknown --lib
# Result: ❌ getrandom error (likely deeper issues with ring/blst)
```

**Consistent Pattern:** All tests blocked by getrandom first, preventing discovery of deeper issues. Need to fix getrandom to continue evaluation.

---

*This document will be updated as evaluation progresses through phases 2-5.*
