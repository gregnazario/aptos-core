# Aptos Core: Dependency Reduction & Build Performance

**Date:** 2026-03-20
**Status:** Draft
**Scope:** Workspace-wide dependency graph optimization and build time reduction

## Problem Statement

Aptos Core's workspace has grown to 262 workspace members with ~1,653 locked packages and 380+ external dependencies. Cold builds are slow, incremental builds recompile more than necessary, and the dependency graph contains significant structural inefficiencies. Key symptoms:

- **127 crates** have duplicate versions in the dependency tree
- **`aptos-types`** (depended on by ~87 crates) transitively compiles the legacy Move compiler (20K lines) for 3 type imports
- **`aptos-framework`** unconditionally compiles the full Move compiler + prover (~125K lines) even when consumers only need the release bundle
- **Three web frameworks** coexist (poem, warp, axum), with `api/` using both poem and warp simultaneously
- **Two versions of `ring`** compile ~26 MB of C/ASM native crypto code redundantly
- **`librocksdb-sys`** compiles 735 C++ files (38 MB source) on every cold build
- **No compilation-level cache** (sccache) is configured; CI uses only Swatinem/rust-cache for `~/.cargo` and `./target`
- **18 git-patched crates** from 5 Aptos forks bypass the crates.io registry cache

## Current State Analysis

### Workspace Metrics

| Metric | Value |
|--------|-------|
| Workspace members | 262 |
| Locked packages (Cargo.lock) | 1,653 |
| External workspace dependencies | 380+ |
| Git-patched crates | 18 (from 5 forks) |
| Crates with duplicate versions | 127 |
| Default members | 10 (9 binaries + 1 library) |

### Build Profile Configuration

| Profile | Key Settings |
|---------|-------------|
| `dev` | Default (no build-override) |
| `release` | `debug = true`, `overflow-checks = true`, **`build-override opt-level = 3`** |
| `performance` | `lto = "thin"`, `codegen-units = 1`, `opt-level = 3` |
| `cli` | `opt-level = "z"`, `strip = true`, `lto = "thin"` |

**Note on `build-override opt-level = 3`:** This exists intentionally. The `aptos-cached-packages` crate uses the Move compiler as a build-dependency to compile framework packages. Running that compiler at `-O0` is extremely slow. The override forces `-O3` for build scripts so the Move compilation step executes faster. This is a trade-off: slower build-dep compilation → faster build script execution. The correct long-term fix is eliminating the need for the compiler in build scripts (see Tier 2, Item 2.2).

### Linker Configuration

- **Linux x86_64:** Uses `lld` (LLVM linker) via `.cargo/config.toml` — already optimized
- **macOS (aarch64-apple-darwin):** Uses default system linker (`ld_prime` since Xcode 15) — **no override configured**
- **Windows:** Default MSVC linker

### CI Caching Strategy (Current)

| Layer | Mechanism | Scope |
|-------|-----------|-------|
| GitHub Actions | `Swatinem/rust-cache@v2.7.8` | `~/.cargo` + `./target` per runner |
| Docker builds | BuildKit `--mount=type=cache` | Local per-builder (no remote sharing) |
| Replay-verify | `actions/cache@v4` | Binary cache keyed by GIT_SHA |

No compilation-level caching (sccache). No remote Docker cache sharing (`cache-from`/`cache-to` not used). Each CI runner on a fresh machine compiles from scratch.

### Most Expensive Dependencies (Compile Time)

| Dependency | Cost Factor | Duplication |
|------------|-------------|-------------|
| `librocksdb-sys` v0.17.3 | 735 C++ files, 38 MB source, + bzip2-sys + lz4-sys + libz-sys + zstd-sys + tikv-jemalloc-sys + bindgen | 1 version |
| `ring` | ~13 MB C/ASM each version | **2 versions** (0.16.20 + 0.17.7) |
| `move-compiler-v2` | 49K lines Rust, 288 transitive deps | 1 version, but over-included |
| `legacy-move-compiler` | 20K lines Rust | 1 version, but over-included |
| `move-model` | 42K lines Rust | 1 version, but over-included |
| `poem-openapi-derive` | 41 transitive deps (heaviest proc-macro) | 1 version |
| `syn` | Heavy proc-macro parser | **2 versions** (1.x + 2.x) |
| `darling` | Proc-macro helper | **4 versions** (0.13, 0.20, 0.21, 0.23) |

### Move Compiler Dependency Chain Problem

```
aptos-types (87 dependents)
  └─ move-model (42K lines)
       └─ legacy-move-compiler (20K lines)
            └─ ... (parsing, expansion, AST)

aptos-types only imports (types/src/vm/module_metadata.rs:25-28):
  - move_model::metadata::CompilationMetadata
  - move_model::metadata::COMPILATION_METADATA_KEY
  - move_model::model::StructEnv
```

This means every crate that depends on `aptos-types` must compile 62K+ lines of Move compiler code it never calls.

**Complication:** `StructEnv<'env>` is not a standalone type — it holds a `ModuleEnv<'env>` which references `GlobalEnv`, essentially the entire model. The actual usage in `aptos-types` (`types/src/vm/module_metadata.rs:726-734`) calls `resource.module_env.get_name().addr()` and `group.module_env.get_name()`. Clean extraction requires replacing `StructEnv` parameters with a simpler interface (see Item 2.1).

### aptos-framework Over-Inclusion Problem

```
aptos-node
  └─ aptos-framework
       ├─ move-compiler-v2      (49K lines) — unconditional
       ├─ move-prover            — unconditional
       ├─ move-prover-boogie-backend — unconditional
       ├─ move-prover-bytecode-pipeline — unconditional
       ├─ move-prover-lab        — unconditional
       ├─ move-docgen            — unconditional
       └─ move-stackless-bytecode — unconditional
```

**File:** `aptos-move/framework/Cargo.toml` (dependencies section)

`aptos-node` imports only `aptos_framework::ReleaseBundle`. It does not need the compiler or prover at runtime.

**Existing partial solution:** `aptos-release-bundle` already exists at `aptos-move/framework/release-bundle/` and contains `ReleaseBundle` / `ReleasePackage`. `aptos-framework` re-exports from it via `pub use aptos_release_bundle::*`. However, `aptos-release-bundle` itself depends on `move-model` (for `CodeWriter`, `emit!`, `emitln!`, `Loc` used by `generate_script_proposal`), which drags in the compiler chain. See Item 2.2 for the fix.

### Web Framework Landscape

| Framework | Version | Direct Users | Unique Deps (beyond poem) |
|-----------|---------|-------------|--------------------------|
| **poem** + poem-openapi | 3.1.3 / 5.1.2 | 6 crates | Baseline (448 total) |
| **warp** | 0.3.5 (declared) / 0.3.6 (resolved) | 11 crates | 36 |
| **axum** | 0.6.20 + 0.7.5 | 2 direct + tonic transitive | 6 |

The `api/` crate uses **both** poem and warp simultaneously.

### Duplicate Ring Versions

- **ring 0.16.20:** Direct dependency of `aptos-crypto` (used for core crypto operations)
- **ring 0.17.7:** Transitive via `rustls` (4 versions: 0.20, 0.21, 0.22, 0.23) → `hyper-rustls` → `reqwest`

Unification requires addressing both `aptos-crypto`'s direct usage AND the multiple rustls versions.

### Patched Dependencies (18 crates from 5 forks)

| Fork Repo | Crates | Branch/Rev | Purpose |
|-----------|--------|-----------|---------|
| `aptos-labs/futures-rs` | futures + 8 sub-crates | `backport` branch | Backported fixes; resolves to v0.3.30 (upstream at v0.3.31) |
| `aptos-labs/algebra` | ark-ec, ark-ff, ark-ff-macros, ark-ff-asm, ark-poly, ark-serialize | `fix-fft-parallelism-cutoff` | FFT parallelism fix; resolves to v0.5.0 |
| `aptos-labs/dudect-bencher` | dudect-bencher | Pinned rev `9515677...` | Exposes `run_bench_with_bencher` as public |
| `aptos-labs/serde-reflection` | serde-reflection | Pinned rev `9d2374a...` | Unknown customization |
| `aptos-labs/merlin` | merlin | Default branch | Schnorr proof transcript customization |

All 18 git-patched crates bypass the crates.io registry cache, requiring GitHub fetches on cache miss and complicating dependency management.

---

## Design: Three-Tiered Optimization Plan

### Tier 1: Conservative (Low-risk, config-only)

Changes that require no structural refactoring, no API changes, and carry near-zero risk of breakage.

#### 1.1 Remove Unused Dependencies via cargo-machete and cargo-udeps

**What:** Run `cargo machete --fix` across the workspace and commit the removals. Currently machete runs in CI in check mode only. Validate with `cargo-udeps` (compilation-based detection) for build-dependencies and proc-macros that machete's source-scanning misses.

**Where:** All `Cargo.toml` files across the workspace.

**Risk:** Very low. False positives are possible for build-dependencies and proc-macros, so each removal should be validated with `cargo check -p <crate>`.

**Validation:** `cargo check --workspace` must pass after removals.

#### 1.2 Consolidate Duplicate Proc-Macro Helper Crates

**What:** Unify versions of proc-macro ecosystem crates that have multiple versions due to transitive dependency drift.

| Crate | Current Versions | Target |
|-------|-----------------|--------|
| `darling` | 0.13, 0.20, 0.21, 0.23 | 0.23 (latest) |
| `strum` / `strum_macros` | 0.24, 0.25, 0.26, 0.27 | 0.27 |
| `prost` / `prost-derive` | 0.11, 0.12, 0.13 | 0.13 |

**How:** Update direct workspace dependency versions and add `[patch.crates-io]` entries if transitive deps pin older versions. For each, check if the latest version has breaking API changes and update call sites. Note: prost versions are tied to tonic/gRPC versions, so prost consolidation may require tonic version unification as well.

**Risk:** Low. These are derive-macro crates with generally stable APIs across minor versions.

#### 1.3 Consolidate Utility Crate Versions

| Crate | Current Versions | Target |
|-------|-----------------|--------|
| `base64` | 0.12, 0.13, 0.21, 0.22 | 0.22 |
| `serde_yaml` | 0.8, 0.9 | 0.9 |
| `toml` | 0.5, 0.7, 0.8, 0.9 | 0.9 |
| `thiserror` | 1.x, 2.x | 2.x (if feasible) |
| `num-bigint` | 0.2, 0.3, 0.4 | 0.4 |

**How:** Update workspace dependency declarations, fix any API changes at call sites. For `base64`, the 0.13→0.22 migration changes from `base64::encode()`/`decode()` to the engine-based API. For `toml`, the 0.5→0.9 migration is significant (complete API redesign). The 0.5 and 0.8 versions are likely from transitive deps and may require patches or upstream updates.

**Risk:** Low-medium. `base64` and `toml` APIs changed significantly across major versions.

#### 1.4 Evaluate Faster Linker on macOS

**What:** Test whether an alternative linker (lld via `brew install llvm`, or the sold/mold for macOS) provides measurable link-time improvement over the default `ld_prime` (Apple's linker since Xcode 15).

**Where:** `.cargo/config.toml`, new `[target.aarch64-apple-darwin]` section.

**Note:** Apple's `ld_prime` (Xcode 15+) is already significantly faster than the old `ld64`. The improvement from switching to `lld` may be marginal on modern macOS. This should be measured before committing the config change. If the improvement is <10%, skip this item.

**Risk:** Low. Would require contributors to install LLVM toolchain. Could be made opt-in via env var override.

#### 1.5 Add Dev Profile Overrides for Slow Crates

**What:** Add per-crate compilation settings in the dev profile. Note: the dev profile with incremental compilation already defaults to `codegen-units = 256`, so only `opt-level` overrides are meaningful.

**Where:** Root `Cargo.toml`, add:
```toml
[profile.dev.package.librocksdb-sys]
opt-level = 1  # Keep some optimization — RocksDB is unusably slow at -O0
```

**Risk:** None for correctness. May make debug-mode RocksDB slightly slower at runtime, but this only affects local dev, not CI or production.

---

### Tier 2: Moderate (Structural refactoring)

Changes that restructure internal crate boundaries and dependency edges. Require updating internal consumers but don't change external APIs.

#### 2.1 Decouple `aptos-types` from Move Compiler

**Priority: Highest. Single most impactful change.**

**What:** Replace `aptos-types`'s dependency on `move-model` with a dependency on a lightweight interface that carries only the data it needs.

**Current dependency:**
```
aptos-types → move-model (42K lines) → legacy-move-compiler (20K lines)
```

**The challenge:** `StructEnv<'env>` is a lifetime-bound reference type that holds `ModuleEnv<'env>` → `GlobalEnv` (the entire model hierarchy). It cannot be trivially extracted. The actual usage in `aptos-types` (`types/src/vm/module_metadata.rs:726-734`) only needs:
- `resource.module_env.get_name().addr()` — module address
- `group.module_env.get_name()` — module name

**Approach:** Replace `StructEnv` parameters in `aptos-types` with a simple trait or struct:

```rust
// New: move-model-types crate (or in aptos-types itself)
pub trait StructInfo {
    fn module_address(&self) -> &AccountAddress;
    fn module_name(&self) -> &IdentStr;
}
```

Then implement this trait for `StructEnv` in `move-model`, and update `aptos-types` to use the trait. The new crate would also contain `CompilationMetadata` and `COMPILATION_METADATA_KEY` (which are simple data types with no compiler dependency).

**Where:**
- New crate: `third_party/move/move-model/move-model-types/` (lightweight, no compiler dep)
- Modified: `types/Cargo.toml` (swap `move-model` → `move-model-types`)
- Modified: `types/src/vm/module_metadata.rs` (use trait instead of `StructEnv` directly)
- Modified: `third_party/move/move-model/src/` (implement trait for `StructEnv`)
- Audit: all callers of `ResourceGroupScope::are_equal_envs()` to ensure they can provide the simpler type

**Impact:** Frees ~87 crates from compiling `move-model` (42K lines) and `legacy-move-compiler` (20K lines). This is ~62K lines of Rust removed from the critical compilation path for most of the workspace.

**Risk:** Medium. Requires careful API design to avoid circular dependencies. The trait approach is lower risk than direct type extraction.

**Validation:** `cargo check --workspace` and `cargo test -p aptos-types`.

#### 2.2 Decouple `aptos-release-bundle` from Move Compiler

**Priority: Very high.**

**What:** Remove `move-model` dependency from `aptos-release-bundle` so that consumers who only need the release bundle (like `aptos-node`) don't transitively compile the compiler.

**Current state:** `aptos-release-bundle` exists at `aptos-move/framework/release-bundle/` and already contains `ReleaseBundle`/`ReleasePackage`. However, it depends on `move-model` because the `generate_script_proposal` functionality uses `CodeWriter`, `emit!`, `emitln!`, and `Loc`.

**Approach:** Move `generate_script_proposal` (and its `move-model` dependency) out of `aptos-release-bundle` into `aptos-framework` (which already depends on `move-model`). The `aptos-release-bundle` crate becomes a pure data/loading crate with minimal deps (move-core-types, bcs, serde).

**Consumers to update:**
- `aptos-node` → switch from `aptos-framework` to `aptos-release-bundle`
- `aptos-vm` → switch to `aptos-release-bundle` (only needs `ReleaseBundle`)
- `aptos-genesis` → evaluate: likely needs `aptos-release-bundle`
- CLI and testing crates → keep `aptos-framework` (need compiler)

**Also consider:** `aptos-cached-packages` has `aptos-framework` as a build-dependency (behind the `testing` feature). Normal builds use the precompiled `.mrb` blob and don't trigger the compiler. Verify that this gating remains correct after the restructuring.

**Impact:** Frees `aptos-node` and its dependency tree from compiling ~125K lines of compiler + prover code. Once `aptos-node` no longer requires the Move compiler, the `build-override opt-level = 3` becomes unnecessary for node builds.

**Risk:** Medium. Requires identifying exactly which APIs from `aptos-framework` each consumer uses.

#### 2.3 Unify `ring` to One Version

**What:** Consolidate from two versions (0.16.20 + 0.17.7) to one.

**Current state:**
- `ring 0.16.20`: Direct dep of `aptos-crypto` (workspace-level)
- `ring 0.17.7`: Transitive via `rustls` (4 versions: 0.20.9, 0.21.12, 0.22.4, 0.23.7) → `hyper-rustls` → `reqwest`

**Strategy:** Upgrade `aptos-crypto` (and any other direct consumers) from `ring 0.16` to `ring 0.17`. The 0.16 → 0.17 migration involves:
- API changes to `ring::aead` (nonce handling)
- API changes to `ring::signature` (verification)
- `ring::rand::SystemRandom` API is stable

**Note:** Consolidating rustls versions (4 versions currently) should be considered alongside ring unification, as both are driven by the same TLS ecosystem version drift.

**Alternative:** Pin `rustls` to a version that uses `ring 0.16` (regressive, not recommended).

**Impact:** Eliminates ~13 MB of redundant C/ASM compilation. Estimated 60-120s savings on cold builds.

**Risk:** Medium-high. `aptos-crypto` is security-critical code (listed as safety-critical directory). Ring API changes require careful security review and full crypto test suite.

#### 2.4 Eliminate Warp Framework

**What:** Migrate all 11 warp-dependent crates to poem, then remove warp from the workspace.

**Migration targets (ordered by complexity):**

| Crate | Warp Usage | Migration Complexity |
|-------|-----------|---------------------|
| `indexer-grpc-server-framework` | Health check/metrics endpoints | Trivial |
| `indexer-grpc-utils` | Health check server | Trivial |
| `indexer-grpc-data-service-v2` | Health endpoints | Trivial |
| `indexer-grpc-manager` | Health endpoints | Trivial |
| `storage/backup/backup-service` | Backup streaming endpoint | Low |
| `storage/backup/backup-cli` | Uses backup-service | Low (transitive) |
| `aptos-telemetry-service` | Telemetry HTTP endpoints | Low-medium |
| `aptos-warp-webserver` | Thin warp wrapper library | Medium (delete entirely) |
| `aptos-rosetta` | Full Rosetta API (uses aptos-warp-webserver) | Medium |
| `api/` | Legacy warp routes alongside poem | Medium |
| `api/test-context` | Test harness with warp-reverse-proxy | Medium |

**Impact:** Removes warp (36 unique deps) + `warp-reverse-proxy` from the tree. Simplifies API crate from dual-framework to single-framework.

**Risk:** Medium. Each migration needs functional testing. The Rosetta API has specific HTTP behavior requirements from the Rosetta specification.

#### 2.5 Feature-Gate Move Prover in aptos-framework

**What:** Make the prover and its dependencies optional behind a `prover` feature flag.

```toml
# aptos-move/framework/Cargo.toml
[features]
default = []
prover = [
  "dep:move-prover",
  "dep:move-prover-boogie-backend",
  "dep:move-prover-bytecode-pipeline",
  "dep:move-prover-lab",
]

[dependencies]
move-prover = { workspace = true, optional = true }
move-prover-boogie-backend = { workspace = true, optional = true }
move-prover-bytecode-pipeline = { workspace = true, optional = true }
move-prover-lab = { workspace = true, optional = true }
```

**Note:** If Item 2.2 is completed, most consumers won't depend on `aptos-framework` at all, reducing this item's impact. It remains valuable for crates that still need the full framework (CLI, test infrastructure).

**Impact:** Even for crates that still need the full `aptos-framework` (CLI, tests), this allows building without the prover unless formal verification is needed.

**Risk:** Low-medium. Requires `#[cfg(feature = "prover")]` guards in framework code that calls prover APIs.

#### 2.6 Unify Crypto Digest Stack

**What:** Three generations of `digest`/`sha2`/`sha3` coexist (0.9, 0.10, 0.11). Converge to eliminate duplicates.

**Current state:**
- `sha2 0.9.9` — workspace default, used by core crypto
- `sha2 0.10.9` — used by `aptos-batch-encryption` and `hmac 0.12` compatibility
- `sha2 0.11.0-rc.5` — pulled in by newer transitive deps

**Strategy:** Target `digest 0.10` / `sha2 0.10` as the convergence point. This requires:
1. Upgrade workspace `sha2` from `0.9.3` to `0.10.x` — update all call sites in `aptos-crypto` and other crates
2. Pin or patch away `digest 0.11` / `sha2 0.11.0-rc.5` by updating transitive deps that pull in the RC

If `sha2 0.11` reaches stable release during this work, consider targeting 0.11 instead to avoid a second migration later.

**Impact:** Eliminates 2 of 3 digest generations. Removes duplicate compilation of SHA-2, SHA-3, HMAC, and related crates.

**Risk:** Medium-high. Touches security-critical hashing code across many crates. Requires thorough testing.

#### 2.7 Incremental async-trait Replacement

**What:** Replace `async-trait` proc-macro usage with native `async fn in traits` (stable since Rust 1.75) where possible.

**Scope:** `async-trait` is declared as a dependency in ~44 workspace crates. Not all usages can be replaced — native async traits have limitations (no `dyn` dispatch without `trait_variant`). Focus on internal traits that are used with static dispatch (generics, not trait objects).

**Impact:** Medium. Removes proc-macro overhead from hot compilation paths. Also improves runtime performance (no `Box::pin` overhead).

**Risk:** Low if done incrementally. Each trait can be migrated independently.

---

### Tier 3: Aggressive (Architectural changes)

Fundamental changes to workspace structure, build infrastructure, or dependency strategy. Each item includes detailed implementation planning.

#### 3.1 Split Ecosystem/Indexer into Separate Workspace

**What:** Move standalone ecosystem service crates into a separate Cargo workspace with its own `Cargo.lock`, while keeping node-embedded indexer crates in the main workspace.

**Rationale:** The 18 ecosystem crates pull in heavy dependencies that node developers never need: `redis`, `cloud-storage`, `google-cloud-storage`, `diesel` + diesel-async (PostgreSQL ORM), `image` (image processing). These are exclusively used by indexer services that run as separate processes.

**Critical blocker:** `aptos-node` directly depends on 2 ecosystem crates:
- `aptos-indexer-grpc-fullnode` — embedded indexer gRPC stream (boot-time service)
- `aptos-indexer-grpc-table-info` — internal indexer DB service

These are hard runtime dependencies imported in `aptos-node/src/services.rs` and `aptos-node/src/storage.rs`.

**Proposed structure (3 phases):**

**Phase 3.1a — Extract standalone services (14 crates):**
```
aptos-core/
  ├─ Cargo.toml             (main workspace, ~248 crates)
  ├─ Cargo.lock
  └─ ecosystem/
       ├─ Cargo.toml         (ecosystem workspace, 14 crates)
       ├─ Cargo.lock
       ├─ indexer-grpc/
       │    ├─ indexer-grpc-cache-worker/
       │    ├─ indexer-grpc-data-service/
       │    ├─ indexer-grpc-data-service-v2/
       │    ├─ indexer-grpc-file-checker/
       │    ├─ indexer-grpc-file-store/
       │    ├─ indexer-grpc-file-store-backfiller/
       │    ├─ indexer-grpc-gateway/
       │    ├─ indexer-grpc-in-memory-cache-benchmark/
       │    ├─ indexer-grpc-manager/
       │    ├─ indexer-grpc-v2-file-store-backfiller/
       │    ├─ indexer-test-transactions/
       │    └─ indexer-transaction-generator/
       └─ nft-metadata-crawler/
```

These 14 crates have no reverse dependency from `aptos-node` and can be cleanly separated. Their dependency on core crates (`aptos-protos`, `aptos-metrics-core`, `aptos-types`) would be satisfied via path dependencies with `{ path = "../../crates/..." }`.

**Phase 3.1b — Decouple embedded indexer crates:**

Create a thin interface crate (`aptos-indexer-grpc-interface`) in the main workspace that defines the traits/APIs that `aptos-node` needs. The actual implementations (`indexer-grpc-fullnode`, `indexer-grpc-table-info`) stay in the main workspace but become optional behind feature flags:

```toml
# aptos-node/Cargo.toml
[features]
default = ["indexer"]
indexer = [
  "dep:aptos-indexer-grpc-fullnode",
  "dep:aptos-indexer-grpc-table-info",
]
```

This allows building `aptos-node` without the embedded indexer for faster local dev iteration.

**Phase 3.1c — Handle shared crates:**

`aptos-indexer-grpc-utils`, `aptos-transaction-filter`, and `aptos-indexer-grpc-server-framework` are used by both embedded and standalone crates. Options:
1. Keep them in the main workspace (standalone services use path deps to reach them)
2. Publish them as versioned crates on crates.io
3. Use git dependencies from the ecosystem workspace

Option 1 is simplest and recommended initially.

**Dependencies removed from main workspace:**
- `redis` v0.22.3
- `cloud-storage` v0.11.1
- `google-cloud-storage` v0.13.1 (+ google-cloud-auth, google-cloud-metadata, google-cloud-token)
- `diesel` v2.3 + `diesel-async` + `diesel_migrations`
- `image` v0.24.7
- `redis-test` v0.1.1

**CI impact:** Currently, all 18 ecosystem crates are built and tested as part of the monolithic workspace (`cargo test`). Only `indexer-processor-testing.yaml` is a separate workflow. After splitting:
- Main workspace CI runs faster (fewer crates + fewer deps)
- Ecosystem workspace gets its own CI pipeline
- The `indexer-processor-testing.yaml` workflow continues unchanged

**Risk:** High. Key challenges:
- Version synchronization between workspaces for shared types
- CI script updates across multiple workflows
- Maintaining path dependency compatibility when shared crates change
- The `aptos-node` feature flag for indexer support needs careful default handling

**Impact:** Removes ~14 crates and their unique heavy deps from the main workspace build graph. Main workspace lockfile shrinks. Node developers get faster builds by default.

#### 3.2 Implement sccache with Shared Storage

**What:** Configure `sccache` as a compilation cache with shared remote storage, enabling cache hits across CI runs, different machines, and developers.

**Current state:** No compilation-level caching exists. The CI caching strategy relies on:
- `Swatinem/rust-cache` — caches `~/.cargo` and `./target`, but is runner-local and doesn't survive machine rotation
- Docker BuildKit cache mounts — local per-builder, no remote sharing

**Architecture:**

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  CI Runner 1 │     │  CI Runner 2 │     │  Developer   │
│  sccache     │     │  sccache     │     │  sccache     │
└──────┬───────┘     └──────┬───────┘     └──────┬───────┘
       │                     │                     │
       └────────┬────────────┘                     │
                │                                  │
         ┌──────▼──────┐                  ┌────────▼────────┐
         │  S3 Bucket   │                  │  Local disk     │
         │  (CI cache)  │                  │  (~/.cache/     │
         │              │                  │   sccache/)     │
         └──────────────┘                  └─────────────────┘
```

**Implementation steps:**

1. **CI configuration** — Set `RUSTC_WRAPPER=sccache` via environment variable (NOT in `.cargo/config.toml`, to avoid forcing it on developers):
   ```yaml
   # .github/actions/rust-setup/action.yaml
   - name: Install sccache
     uses: mozilla-actions/sccache-action@v0.0.6
   - name: Configure sccache
     run: |
       echo "SCCACHE_GHA_ENABLED=true" >> $GITHUB_ENV
       echo "RUSTC_WRAPPER=sccache" >> $GITHUB_ENV
   ```
   Note: The `sccache-action` supports GitHub Actions built-in cache as a backend (no S3 needed initially).

2. **S3 backend (later)** — For broader sharing, configure an S3 bucket:
   ```yaml
   env:
     SCCACHE_BUCKET: aptos-sccache
     SCCACHE_REGION: us-west-2
     SCCACHE_S3_USE_SSL: "true"
     AWS_ACCESS_KEY_ID: ${{ secrets.SCCACHE_AWS_ACCESS_KEY_ID }}
     AWS_SECRET_ACCESS_KEY: ${{ secrets.SCCACHE_AWS_SECRET_ACCESS_KEY }}
   ```

3. **Docker builds** — Add `RUSTC_WRAPPER=sccache` to Dockerfile builders. Mount sccache directory:
   ```dockerfile
   # docker/builder/builder.Dockerfile
   RUN cargo install sccache --locked
   ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
   ENV SCCACHE_BUCKET=aptos-sccache
   ```

4. **Developer opt-in** — Document local sccache setup:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   # Uses local disk cache by default (~/.cache/sccache/)
   ```

**Cache effectiveness analysis:**

| Crate | Build Time (cold) | Cache Hit Rate (estimated) | Notes |
|-------|-------------------|---------------------------|-------|
| `librocksdb-sys` | 2-5 min | Very high (changes rarely) | Biggest single win |
| `ring` | 30-60s per version | Very high | C/ASM, changes rarely |
| `move-compiler-v2` | 30-60s | Medium-high | Changes with Move language updates |
| `move-model` | 20-40s | Medium-high | Same cadence as compiler |
| `syn` 1.x + 2.x | 10-20s total | Very high | Never changes (pinned versions) |
| All other Rust crates | Variable | High after first build | Most crates change infrequently |

**Cache invalidation triggers:** sccache keys on: source hash, compiler version, compiler flags, env vars. Automatic invalidation on:
- Rust toolchain upgrade (new rustc version)
- Feature flag changes
- Profile changes (dev vs release)
- Source code changes in the crate

**Security considerations:**
- **Cache poisoning:** sccache uses content-addressed storage (keyed by input hash). A malicious actor would need to inject a binary with the same hash as a legitimate compilation — computationally infeasible.
- **Access control:** S3 bucket should have write access restricted to CI service accounts only. Read can be broader.
- **Secret management:** AWS credentials stored as GitHub secrets, not in repo.

**Risk:** Medium. Key challenges:
- Initial setup and testing across all CI job types
- Docker builder integration requires Dockerfile changes
- sccache has known issues with some proc-macro crates (may need exclusion list)
- Cache storage costs (S3) — mitigated by TTL policies

**Impact:** Very high for CI. Estimated 50-80% reduction in cold build times after the cache is warm. The single biggest win is `librocksdb-sys` which would go from minutes to a cache hit in seconds. Particularly impactful for PRs that touch non-Rust files (docs, Move code, configs) — these would get near-complete cache hits.

#### 3.3 Pre-built librocksdb-sys Bindings

**What:** Eliminate the 2-5 minute C++ compilation of librocksdb-sys by providing pre-built static libraries.

**Current state:** `librocksdb-sys v0.17.3+10.4.2` compiles RocksDB 10.4.2 from source on every cold build. The resolved feature set includes: `bindgen-runtime`, `bzip2`, `jemalloc`, `lz4`, `snappy`, `static`, `zlib`, `zstd`. This triggers compilation of:
- RocksDB C++ source (735 files, 38 MB)
- `bzip2-sys` (C)
- `libz-sys` (C)
- `lz4-sys` (C)
- `zstd-sys` (C)
- `tikv-jemalloc-sys` (C)
- `bindgen` (Rust, invokes libclang to generate FFI bindings)

**Important:** `librocksdb-sys` does NOT have a `system` feature for linking against a system-installed RocksDB. The `pkg-config` feature only applies to `io-uring` (Linux).

**Three approaches (ordered by recommendation):**

**Approach A: sccache (recommended, use Item 3.2)**

The simplest solution is sccache (Item 3.2), which caches C/C++ compilation results. Since librocksdb-sys changes very rarely (only on `rocksdb` version bumps), the cache hit rate would be >99% after the first build. This avoids any changes to the RocksDB build system.

**Approach B: System RocksDB via custom build script**

Modify `aptos-rocksdb-options` to support linking against a system-installed RocksDB:

```toml
# storage/rocksdb-options/Cargo.toml
[features]
default = ["bundled"]
bundled = ["rocksdb/default"]
system = []  # Link against system librocksdb
```

With a `build.rs` that uses `pkg-config` to find the system library when `system` feature is active:

```rust
#[cfg(feature = "system")]
fn main() {
    pkg_config::probe_library("rocksdb").unwrap();
}
```

**Platforms:**
- macOS: `brew install rocksdb` (includes jemalloc, compression libs)
- Ubuntu: `apt install librocksdb-dev` (may be outdated — version 10.4.2 is very new)

**Risk:** Version mismatch between system RocksDB and expected API. The `bindgen-runtime` feature generates bindings at build time based on the installed headers — system headers may differ from bundled ones.

**Approach C: Pre-built binary artifacts**

Build platform-specific static libraries in CI and publish them as binary artifacts:

```
aptos-rocksdb-prebuilt/
  ├─ x86_64-unknown-linux-gnu/
  │    ├─ librocksdb.a
  │    ├─ libbz2.a
  │    ├─ liblz4.a
  │    ├─ libz.a
  │    ├─ libzstd.a
  │    ├─ libjemalloc.a
  │    └─ bindings.rs  (pre-generated)
  └─ aarch64-apple-darwin/
       └─ ... (same structure)
```

A custom build script would check for pre-built artifacts before falling back to source compilation.

**Risk:** High maintenance burden. Must regenerate on every RocksDB version bump. ABI compatibility across compiler versions. Platform coverage (need artifacts for every target triple). The pre-generated `bindings.rs` must match the exact librocksdb-sys version and features.

**Recommendation:** Use sccache (Approach A) as the primary solution. Consider Approach B as an optional developer convenience for fast local builds. Approach C is not recommended due to maintenance burden.

**Impact:** With sccache, librocksdb-sys goes from 2-5 minutes to seconds on cache hit. With system RocksDB, eliminates native compilation entirely but requires RocksDB installation.

**Risk:** Approach A (sccache) is low risk. Approach B is medium risk (version compatibility). Approach C is high risk (maintenance burden).

#### 3.4 Evaluate and Remove Forked Dependencies

**What:** Audit all 18 git-patched crates across 5 Aptos forks. Determine whether each fork's changes have been merged upstream or can be upstreamed. Remove patches where possible.

**Current patches:**

**Fork 1: `aptos-labs/futures-rs` (9 crates, branch: `backport`)**
- Workspace declares futures `0.3.29`, fork resolves to `0.3.30`
- Upstream futures-rs is at `0.3.31`
- **Action:** Diff the `backport` branch against upstream `0.3.31`. If all backported changes are included in 0.3.31, upgrade the workspace to futures `0.3.31` and remove all 9 patches. If not, identify remaining delta and either upstream the changes or document why the fork is still needed.
- **Impact of removal:** 9 fewer git dependencies. Enables crates.io registry caching for the entire futures ecosystem. Simplifies Cargo.lock.

**Fork 2: `aptos-labs/algebra` (6 crates, branch: `fix-fft-parallelism-cutoff`)**
- Workspace declares ark-* `0.5.0`, fork resolves to `0.5.0` at commit `2cacd5e...`
- **Action:** Check if the FFT parallelism fix has been merged into upstream `arkworks-rs/algebra`. If merged, bump to the upstream version containing the fix and remove patches. If not, submit PR upstream.
- **Note:** `ark-ec 0.4.2` from crates.io also exists in the tree (used by some transitive deps), so two versions of ark-ec coexist.
- **Impact of removal:** 6 fewer git dependencies. Cleaner arkworks version story.

**Fork 3: `aptos-labs/dudect-bencher` (1 crate, pinned rev)**
- Custom change: exposes `run_bench_with_bencher` as public
- **Action:** Submit PR upstream to make this function public. If accepted, remove patch. If rejected, evaluate whether the functionality can be achieved differently.
- **Impact:** Minor (1 crate), but reduces fork maintenance.

**Fork 4: `aptos-labs/serde-reflection` (1 crate, pinned rev)**
- Purpose unknown from the code — no comment explaining the customization.
- **Action:** Diff the fork against upstream to understand the changes. Document the reason. Attempt to upstream.
- **Impact:** Minor (1 crate).

**Fork 5: `aptos-labs/merlin` (1 crate, default branch)**
- Schnorr proof transcript library customization.
- **Action:** Diff against upstream. This is cryptographic code — any fork changes need security review before modification.
- **Impact:** Minor (1 crate), but git dep without pinned rev is a reproducibility concern.

**Overall impact of removing all patches:** 18 fewer git dependencies → faster `cargo fetch` (no GitHub cloning for these crates), better cache effectiveness (crates.io registry is cached more reliably), simpler Cargo.lock, reduced fork maintenance burden.

**Risk:** Medium-high for futures and arkworks (large surface area, need to verify behavior equivalence). Low for the 3 single-crate forks.

#### 3.5 Cargo Timings-Driven Critical Path Optimization

**What:** Use `cargo build --timings` to generate empirical build-time data, then use that data to identify and optimize the critical compilation path.

**Current state:** No one has run `cargo build --timings` on this project. All build-time estimates in this document are theoretical.

**Implementation:**

**Step 1: Baseline measurements**

Run timings for the two most important build targets:
```bash
# Clean build to get accurate cold-build timings
cargo clean
cargo build --timings -p aptos-node --release 2>&1
# Save: target/cargo-timings/cargo-timing.html

cargo clean
cargo build --timings -p aptos --profile cli 2>&1
# Save: target/cargo-timings/cargo-timing.html
```

Also measure incremental build times for common developer workflows:
```bash
# Simulate editing a file in aptos-vm and rebuilding
touch aptos-move/aptos-vm/src/lib.rs
cargo build --timings -p aptos-node 2>&1
```

**Step 2: Identify critical path**

The timings HTML report shows a waterfall chart with:
- **X-axis:** Wall-clock time
- **Y-axis:** Crate compilation units
- **Colors:** Blue = building, yellow = waiting for deps, red = codegen

Key metrics to extract:
- **Total wall-clock time** for cold build
- **Critical path crates** — the longest sequential chain of dependencies
- **Serialization bottlenecks** — crates with high compile time AND many dependents blocked on them
- **Parallelism utilization** — how many cores are idle during the build

**Step 3: Targeted optimizations based on data**

Common patterns found by timings analysis:

| Pattern | Optimization |
|---------|-------------|
| Single crate dominates critical path | Split into smaller crates that can compile in parallel |
| Large crate with few deps blocks many | Move heavy code into a separate crate, keep lightweight API crate |
| Long codegen phase | Increase `codegen-units` for that crate in dev profile |
| Many crates waiting on one proc-macro | Pre-compile proc-macro, or reduce its dependency footprint |
| Low parallelism late in build | Rebalance dependency graph to increase concurrency |

**Step 4: Integrate into CI**

Add a periodic (weekly) CI job that runs `cargo build --timings` and archives the HTML report:
```yaml
# .github/workflows/build-timings.yaml
name: Build Timings
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
jobs:
  timings:
    runs-on: ubuntu-latest-64-core
    steps:
      - uses: actions/checkout@v4
      - name: Build with timings
        run: |
          cargo build --timings -p aptos-node --release
      - uses: actions/upload-artifact@v4
        with:
          name: cargo-timings
          path: target/cargo-timings/
```

This provides trend data to track build time regressions and validate optimization efforts.

**Risk:** None for the analysis itself. Changes derived from the analysis carry their own risk profiles.

**Impact:** This item doesn't directly improve build times — it provides the data needed to make all other items more effective and to discover optimizations not yet identified. **Recommended as the first Tier 3 action.**

#### 3.6 Evaluate Workspace Dependency Resolution Strategy

**What:** Analyze whether Cargo's `resolver = "2"` feature unification is causing unnecessary compilation, and whether workspace-level feature declarations can be tightened.

**Context:** The workspace uses `resolver = "2"`, which means features are NOT unified between normal and dev dependencies. However, workspace-level feature declarations in `[workspace.dependencies]` apply globally. If a workspace dependency declares `features = ["full"]` (like tokio), every consumer gets those features even if they only need a subset.

**Action:**
1. Audit workspace dependencies with broad feature flags (tokio "full", serde with all features, etc.)
2. Move feature declarations from `[workspace.dependencies]` to per-crate `[dependencies]` where features vary significantly between consumers
3. Use `cargo tree -e features` to identify feature flag propagation paths

**Impact:** Low-medium. Primarily reduces binary size and may slightly reduce compile times for incremental builds.

**Risk:** Low. Feature flag changes are backward-compatible.

---

## Implementation Sequencing

### Phase 1: Measurement & Quick Wins (Tier 1 + Tier 3.5)

```
3.5 Cargo timings baseline      ← Do FIRST, informs everything else
1.1 cargo-machete cleanup       ← Independent, immediate wins
1.2 Consolidate proc-macro versions  ← Independent
1.3 Consolidate utility versions     ← Independent
1.4 Evaluate macOS linker           ← Independent, may be skipped
1.5 Dev profile overrides           ← Independent
```

All Tier 1 items are independent and can be done as separate PRs in parallel. Item 3.5 should be the very first action to establish baselines.

### Phase 2: High-Impact Structural Changes (Tier 2)

```
2.1 Decouple aptos-types from Move compiler  ← Do first, highest ROI
2.2 Decouple aptos-release-bundle            ← Do second, depends on understanding from 2.1
2.5 Feature-gate Move prover                 ← Can be combined with 2.2
2.3 Unify ring versions                      ← Independent, security review needed
2.4 Eliminate warp                            ← Independent, can parallelize
2.6 Unify digest stack                        ← After ring unification
2.7 async-trait replacement                   ← Ongoing, incremental
```

Dependencies:
- 2.1 should be done before 2.2 (cleaner `aptos-types` makes framework restructuring easier)
- 2.5 can be combined with 2.2 (both modify `aptos-framework/Cargo.toml`)
- 2.6 should follow 2.3 (both touch the crypto dependency tree)
- 2.7 is ongoing and independent

### Phase 3: Infrastructure & Architecture (Tier 3)

```
3.2 sccache implementation            ← High CI impact, independent of code changes
3.4 Evaluate/remove forked deps       ← Independent, start with futures fork
3.1 Workspace splitting               ← After Tier 2 changes stabilize
3.3 Pre-built RocksDB                 ← Mostly solved by 3.2 (sccache)
3.6 Feature resolution audit          ← Low priority, do when convenient
```

Dependencies:
- 3.2 should be done early — it accelerates all subsequent work
- 3.1 should wait until Tier 2 changes stabilize (cleaner crate boundaries make splitting easier)
- 3.3 is largely addressed by 3.2; only pursue Approach B (system RocksDB) if sccache proves insufficient

## Expected Impact Summary

| Phase | Cold Build Improvement | Incremental Build Improvement | Dependency Count Reduction |
|-------|----------------------|------------------------------|---------------------------|
| Tier 1 | 5-15% | 2-5% | ~20-40 duplicate versions eliminated |
| Tier 2 | 30-50% | 40-60% | ~80-120 deps removed from critical path |
| Tier 3 | 50-70% (with sccache) | 60-80% | ~200+ deps from main workspace |

**Note:** These are rough estimates based on dependency analysis, not empirical measurements. Item 3.5 (cargo timings) will provide actual baselines. Measure before and after each change to validate impact.

## Success Criteria

1. **Measurable:** Cold build time for `aptos-node` reduced by at least 30% after Tier 2
2. **Measurable:** `cargo check -p aptos-types` no longer compiles any Move compiler code after 2.1
3. **Measurable:** Duplicate crate versions reduced from 127 to <80 after Tier 1
4. **Measurable:** Warp removed entirely from workspace after 2.4
5. **Measurable:** CI build times tracked via cargo timings trend data after 3.5
6. **Measurable:** sccache cache hit rate >80% for CI builds after 3.2
7. **Qualitative:** New contributors can build faster; CI costs decrease

## Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| Type extraction (2.1) — `StructEnv` is deeply embedded in model hierarchy | Use trait-based approach instead of type extraction; audit all callers |
| Ring upgrade (2.3) introduces crypto bugs | Security review of all `ring` API call site changes; run full crypto test suite |
| Warp removal (2.4) breaks Rosetta API compatibility | Run Rosetta specification conformance tests after migration |
| Workspace split (3.1) — `aptos-node` depends on 2 ecosystem crates | Phase 3.1b: feature-gate embedded indexer, decouple via interface crate |
| sccache (3.2) cache poisoning | Content-addressed caching; restrict S3 write access to CI service accounts |
| Fork removal (3.4) — behavior differences from upstream | Diff each fork against upstream; run full test suite with upstream deps before removing patches |
| `aptos-release-bundle` restructuring (2.2) — `generate_script_proposal` uses `move-model` | Move `generate_script_proposal` into `aptos-framework`, keep `aptos-release-bundle` as pure data crate |
| Multiple rustls versions complicate ring unification (2.3) | Address rustls consolidation alongside ring; may need hyper/reqwest version alignment |
