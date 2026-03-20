# Complete Command Feasibility Classification for WASM

**Total Commands:** 30
**WASM-Compatible (with VFS):** 6 commands (20%)
**WASM-Compatible (no VFS):** 2 commands (7%)
**Status:** Phase 2 Complete

---

## Classification Methodology

Commands analyzed based on:
1. **Dependency analysis** - Does it require WASM-incompatible crates?
2. **Filesystem requirements** - Can it work with virtual filesystem?
3. **Network requirements** - Does it need blockchain RPC calls?
4. **Runtime requirements** - Threading, process spawning, etc.
5. **Value for web-based tools** - Is it useful in browser context?

---

## ✅ TIER 1: High WASM Viability (Core Compilation)

### 1. Disassemble
**Command:** `aptos move disassemble`
**Source:** `src/bytecode.rs:36`
**WASM Feasibility:** ✅ **VERY HIGH**

**Dependencies:**
- move-binary-format ✅
- move-decompiler ✅

**Filesystem Ops:**
- Input: Read .mv bytecode files
- Output: Write .masm assembly files

**WASM Adaptation:**
```rust
// Native: Read from filesystem
let bytecode = std::fs::read(path)?;

// WASM: Accept bytecode directly
#[wasm_bindgen]
pub fn disassemble_bytecode(bytecode: Vec<u8>) -> Result<String, JsValue>
```

**Value:** High - useful for education, debugging bytecode in browser

**Priority:** 🔥 **MVP CANDIDATE** (no VFS needed if accepting bytecode directly)

---

### 2. Decompile
**Command:** `aptos move decompile`
**Source:** `src/bytecode.rs:50`
**WASM Feasibility:** ✅ **VERY HIGH**

**Dependencies:**
- move-binary-format ✅
- move-decompiler ✅

**Similar to Disassemble** - converts bytecode → Move source

**Priority:** 🔥 **MVP CANDIDATE** (no VFS needed)

---

### 3. Compile
**Command:** `aptos move compile` (alias: `build`)
**Source:** `src/commands.rs:100`
**WASM Feasibility:** ✅ **HIGH** (requires VFS)

**Dependencies:**
- move-compiler-v2 ⚠️ (fixable getrandom issue)
- move-package ⚠️ (needs VFS abstraction)

**Filesystem Ops:**
- Input: Read Move.toml, sources/*.move
- Output: Write build/ artifacts

**WASM Adaptation:** Requires Virtual Filesystem (Phase 4)

**Value:** **CRITICAL** - core workflow for web-based Move IDEs

**Priority:** 🔥 **Extended MVP** (post-VFS implementation)

---

### 4. CompileScript
**Command:** `aptos move compile-script` (alias: `build-script`)
**Source:** `src/commands.rs:102`
**WASM Feasibility:** ✅ **HIGH** (simpler than full package)

**Dependencies:**
- move-compiler-v2 ⚠️
- Simpler than Compile (single script vs package)

**WASM Adaptation:**
```rust
#[wasm_bindgen]
pub fn compile_script(source: String) -> Result<Vec<u8>, JsValue>
```

**Value:** High - scripting in browser

**Priority:** 🔥 **Extended MVP**

---

### 5. Lint
**Command:** `aptos move lint`
**Source:** `src/lint.rs`, `src/commands.rs:116`
**WASM Feasibility:** ✅ **HIGH** (requires VFS)

**Dependencies:**
- move-linter ⚠️ (has errno blocker)
- move-compiler-v2 ⚠️
- move-package ⚠️ (needs VFS)

**Value:** **HIGH** - linting in web-based IDEs

**Priority:** 🔥 **Extended MVP**

---

### 6. VerifyPackage
**Command:** `aptos move verify-package`
**Source:** `src/commands.rs:127`
**WASM Feasibility:** ✅ **HIGH** (requires VFS)

**Dependencies:**
- move-bytecode-verifier ✅
- Verification logic is pure computation

**Value:** Medium - bytecode verification

**Priority:** ⭐ **Extended MVP**

---

## ⚠️ TIER 2: Moderate Complexity (Analysis Tools)

### 7. Document
**Command:** `aptos move document` (alias: `doc`)
**Source:** `src/commands.rs:113`
**WASM Feasibility:** ⚠️ **MEDIUM**

**Dependencies:**
- move-docgen (unknown WASM compatibility)
- Generates HTML/markdown documentation

**Blockers:**
- Filesystem output handling
- May have template rendering dependencies

**Value:** Medium - documentation generation could work in browser

**Priority:** 🟡 **Future consideration**

---

### 8. Show
**Command:** `aptos move show` (subcommand, hidden)
**Source:** `src/show.rs`, `src/commands.rs:125`
**WASM Feasibility:** ⚠️ **MEDIUM**

**Description:** Display package metadata

**Dependencies:**
- move-package (read-only operations)

**Value:** Low - primarily informational

**Priority:** 🟡 **Low priority**

---

### 9. List
**Command:** `aptos move list`
**Source:** `src/commands.rs:117`
**WASM Feasibility:** ⚠️ **MEDIUM** (requires VFS)

**Description:** List packages

**Dependencies:**
- Directory traversal via VFS

**Value:** Low - directory listing

**Priority:** 🟡 **Low priority**

---

### 10. Prove
**Command:** `aptos move prove`
**Source:** `src/commands.rs:118`
**WASM Feasibility:** ❌ **LOW**

**Dependencies:**
- move-prover (likely has SMT solver dependencies: z3, boogie)
- Native process spawning for external tools

**Blockers:**
- External toolchain (z3 SMT solver) not available in WASM
- Heavy computational workload

**Value:** Low - prover is advanced feature, not critical for MVP

**Priority:** ❌ **Exclude from WASM scope**

---

### 11. BuildPublishPayload
**Command:** `aptos move build-publish-payload`
**Source:** `src/commands.rs:96`
**WASM Feasibility:** ⚠️ **MEDIUM**

**Description:** Build transaction payload for publishing

**Dependencies:**
- Compilation pipeline (same as Compile)
- BCS serialization ✅

**WASM Adaptation:** Could generate unsigned transaction payload

**Value:** Medium - useful for wallet integrations

**Priority:** 🟡 **Future consideration** (after Compile works)

---

## ❌ TIER 3: Filesystem Operations (Not WASM-Compatible)

### 12. Init
**Command:** `aptos move init`
**Source:** `src/commands.rs:115`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- std::fs::create_dir_all (native filesystem)

**Description:** Creates new Move package directory structure

**Alternative:** Web UI could generate package structure client-side

**Priority:** ❌ **Exclude** - not meaningful in browser without filesystem

---

### 13. Clean
**Command:** `aptos move clean`
**Source:** `src/commands.rs:97`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- std::fs::remove_dir_all (native filesystem)

**Description:** Deletes build/ directory

**Priority:** ❌ **Exclude** - no persistent filesystem in WASM

---

### 14. Fmt
**Command:** `aptos move fmt`
**Source:** `src/fmt.rs`, `src/commands.rs:130`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- May spawn external movefmt process
- Filesystem operations

**Alternative:** Could implement formatter in Rust that's WASM-compatible

**Priority:** ❌ **Exclude** (but formatter logic could be ported)

---

## ❌ TIER 4: Testing (Not WASM-Compatible)

### 15. Test
**Command:** `aptos move test`
**Source:** `src/commands.rs:126`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- move-unit-test (uses threads, process spawning)
- tokio multi-threaded runtime ❌

**Description:** Runs Move unit tests

**Blockers:**
- Multi-threading not supported in WASM
- Test runner spawns processes

**Value:** High (but fundamentally incompatible)

**Priority:** ❌ **Exclude** - architectural blocker

---

### 16. Coverage
**Command:** `aptos move coverage` (subcommand)
**Source:** `src/coverage.rs`, `src/commands.rs:104`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- move-coverage (filesystem-intensive)
- Trace file generation/analysis

**Description:** Test coverage analysis

**Priority:** ❌ **Exclude** - depends on Test working

---

## ❌ TIER 5: Network/Deployment (Not WASM-Compatible)

These commands require blockchain RPC interaction via aptos-rest-client, which uses reqwest with native TLS.

### 17. Publish
**Command:** `aptos move publish` (alias: `deploy`)
**Source:** `src/commands.rs:120`
**WASM Feasibility:** ❌ **NOT COMPATIBLE** (could be re-implemented)

**Dependencies:**
- aptos-rest-client ❌ (reqwest + native TLS)
- Network operations

**WASM Alternative:** Use JavaScript fetch() for network, WASM for payload building

**Value:** High - publishing is core workflow

**Priority:** 🟡 **Future consideration** with JS network layer

---

### 18. Run
**Command:** `aptos move run`
**Source:** `src/commands.rs:121`
**WASM Feasibility:** ❌ **NOT COMPATIBLE** (could be re-implemented)

**Description:** Execute entry function on-chain

**Dependencies:** Network operations

**Priority:** 🟡 **Future consideration** with JS network layer

---

### 19. RunScript
**Command:** `aptos move run-script`
**Source:** `src/commands.rs:122`
**WASM Feasibility:** ❌ **NOT COMPATIBLE** (could be re-implemented)

**Description:** Execute Move script on-chain

**Priority:** 🟡 **Future consideration**

---

### 20. View
**Command:** `aptos move view`
**Source:** `src/commands.rs:128`
**WASM Feasibility:** ❌ **NOT COMPATIBLE** (could be re-implemented)

**Description:** Call view function (read-only on-chain query)

**Priority:** 🟡 **Future consideration**

---

### 21. Simulate
**Command:** `aptos move simulate`
**Source:** `src/commands.rs:123`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Dependencies:**
- aptos-transaction-simulation
- Needs chain state snapshot

**Description:** Simulate transaction execution

**Priority:** ❌ **Exclude** - needs full chain state

---

### 22. Replay
**Command:** `aptos move replay`
**Source:** `src/commands.rs:129`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Replay historical transaction

**Dependencies:** Network + simulation

**Priority:** ❌ **Exclude**

---

### 23. Download
**Command:** `aptos move download`
**Source:** `src/commands.rs:114`
**WASM Feasibility:** ❌ **NOT COMPATIBLE** (could be re-implemented)

**Dependencies:**
- aptos-rest-client ❌
- Filesystem write operations

**Description:** Download on-chain package to filesystem

**Priority:** 🟡 **Future consideration** - could download to virtual FS

---

### 24. CreateObjectAndPublishPackage
**Command:** `aptos move create-object-and-publish-package`
**Source:** `src/commands.rs:105`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Deploy package to object address

**Dependencies:** Network operations + complex deployment flow

**Priority:** ❌ **Exclude** - advanced feature

---

### 25. UpgradeObjectPackage
**Command:** `aptos move upgrade-object-package`
**Source:** `src/commands.rs:106`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Upgrade existing object package

**Priority:** ❌ **Exclude**

---

### 26. DeployObject
**Command:** `aptos move deploy-object`
**Source:** `src/commands.rs:107`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Deploy code to object

**Priority:** ❌ **Exclude**

---

### 27. UpgradeObject
**Command:** `aptos move upgrade-object`
**Source:** `src/commands.rs:108`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Upgrade object code

**Priority:** ❌ **Exclude**

---

### 28. CreateResourceAccountAndPublishPackage
**Command:** `aptos move create-resource-account-and-publish-package`
**Source:** `src/commands.rs:109`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Create resource account and deploy

**Priority:** ❌ **Exclude**

---

### 29. ClearStagingArea
**Command:** `aptos move clear-staging-area`
**Source:** `src/commands.rs:98`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Clear chunked publish staging area

**Priority:** ❌ **Exclude**

---

### 30. Sim (subcommand)
**Command:** `aptos move sim`
**Source:** `src/sim.rs`, `src/commands.rs:132`
**WASM Feasibility:** ❌ **NOT COMPATIBLE**

**Description:** Local simulation subcommands

**Dependencies:**
- aptos-transaction-simulation-session
- Complex state management

**Priority:** ❌ **Exclude** - advanced simulation features

---

## Summary Statistics

| Category | Count | Percentage |
|----------|-------|------------|
| **Tier 1: WASM-Compatible Core** | 6 | 20% |
| **Tier 2: Moderate Complexity** | 5 | 17% |
| **Tier 3: Filesystem Ops** | 3 | 10% |
| **Tier 4: Testing** | 2 | 7% |
| **Tier 5: Network/Deployment** | 14 | 47% |
| **TOTAL** | 30 | 100% |

### WASM MVP Scope

**Minimal MVP (No VFS):** 2 commands
- ✅ Disassemble
- ✅ Decompile

**Extended MVP (With VFS):** +4 commands
- ✅ Compile
- ✅ CompileScript
- ✅ Lint
- ✅ VerifyPackage

**Total WASM Viable:** 6 of 30 commands (20%)

**Future Possibilities (With JS Network Layer):** +7 commands
- ⚠️ Publish, Run, RunScript, View, Download, Document, BuildPublishPayload

**Total Potential WASM:** 13 of 30 commands (43% if network abstraction added)

---

## Recommended Phasing

### Phase 3 Prototype: Tier 1a (No VFS)
Focus: Bytecode operations only
- Disassemble
- Decompile

**Timeline:** 1 week
**Risk:** Low
**Value:** Proof-of-concept demonstrating WASM feasibility

### Phase 4-5: Tier 1b (With VFS)
Focus: Core compilation workflow
- Compile
- Lint
- VerifyPackage
- CompileScript

**Timeline:** 3-4 weeks (includes VFS implementation)
**Risk:** Medium (VFS abstraction complexity)
**Value:** **HIGH** - enables web-based Move IDEs

### Future Phase: Network Layer
Focus: Blockchain interaction via JS
- Publish (build payload in WASM, submit via JS fetch)
- Run, View (call via JS, process results in WASM)
- Download (fetch via JS, process in WASM)

**Timeline:** 2-3 weeks
**Risk:** Medium (architecture bridging WASM/JS)
**Value:** Medium - completes publish workflow

---

## Key Insights

1. **Core Value is in Compilation:** The most valuable WASM commands are compilation-related (Compile, Lint). These enable web-based Move IDEs.

2. **VFS is Critical Enabler:** Virtual filesystem abstraction unlocks 4 additional high-value commands (67% of Tier 1).

3. **Testing is Fundamentally Blocked:** Multi-threading requirements make move-unit-test incompatible with WASM. No reasonable workaround.

4. **Network Commands Need Architectural Re-think:** 14 commands (47%) involve network operations. These could work with JS fetch() bridge, but require careful API design.

5. **Filesystem Commands are Not Applicable:** Init, Clean, Fmt don't make sense in browser context without persistent filesystem.

6. **20% Baseline, 43% Aspirational:** We can achieve 20% command coverage with pure WASM, potentially 43% with hybrid WASM+JS architecture.

---

## Phase 2 Complete

**Status:** ✅ All 30 commands classified
**Next Step:** Phase 3 - Build minimal WASM proof-of-concept with Disassemble command
**Recommendation:** Proceed with Phase 3 prototype to validate core Move compiler WASM compatibility

---

*Document created: 2026-03-20*
*Last updated: 2026-03-20*
