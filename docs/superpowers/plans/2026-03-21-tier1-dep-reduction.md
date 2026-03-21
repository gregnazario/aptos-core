# Tier 1 Dependency Reduction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce duplicate dependency versions in the aptos-core workspace and establish build-time baselines.

**Architecture:** Bump workspace-level dependency declarations to newer versions, update all call sites for breaking API changes, and run cargo timings to measure current build performance. Each version bump is an independent PR.

**Tech Stack:** Rust, Cargo, base64 0.22 (engine-based API)

---

## Pre-flight: What We Learned During Investigation

Several items from the original spec turned out to be non-actionable:

| Original Item | Finding | Action |
|---|---|---|
| 1.1 cargo-machete | Already clean — zero unused deps found | **Skip** |
| darling consolidation | All 2 versions are transitive (derive_builder, poem-openapi, diesel) | **Skip** — can't fix without patching upstream |
| strum consolidation | Old versions from transitive deps (passkey-types 0.25, processor 0.24) | **Skip** — transitive, low impact |
| prost consolidation | 0.11 is transitive via google-cloud-pubsub in `processor` git dep | **Skip** — transitive, ecosystem crate |
| thiserror consolidation | No duplication exists — only v1 in tree | **Skip** |
| serde_yaml consolidation | Our direct dep is 0.8; the 0.9 is transitive from poem | **Skip** — bumping ours doesn't eliminate duplication |
| toml consolidation | Our direct dep is 0.7; the 0.9 is transitive from diesel | **Skip** — bumping ours doesn't eliminate duplication |
| 1.6 tokio feature audit | Minimal compile-time impact (features control runtime, not compilation) | **Skip** |

**What IS actionable:**

| Item | Change | Files Affected | Impact |
|---|---|---|---|
| **base64 0.13 → 0.22** | Breaking API change across 10+ crates | ~15 files | Eliminates oldest of 3 base64 versions |
| ~~num-bigint 0.3 → 0.4~~ | **Blocked:** num-bigint 0.4's `rand` feature requires `rand 0.8`, but workspace uses `rand 0.7.3`. Upgrading rand is a workspace-wide breaking change. | N/A | **Deferred to Tier 2** |
| **Cargo timings baseline** | Run `cargo build --timings` for aptos-node | No code changes | Establishes measurement baseline |
| **1.5 Dev profile override** | Add `opt-level` for librocksdb-sys | 1 line in root Cargo.toml | Faster dev builds |

---

## Task 0: Cargo Timings Baseline

**Files:**
- No code changes

- [ ] **Step 1: Run timings for aptos-node release build**

```bash
cargo build --timings -p aptos-node --release 2>&1
```

This will generate `target/cargo-timings/cargo-timing.html`. The build will take a while (possibly 30+ minutes for a cold release build).

- [ ] **Step 2: Save the timings report**

```bash
mkdir -p docs/superpowers/build-timings
cp target/cargo-timings/cargo-timing.html docs/superpowers/build-timings/2026-03-21-aptos-node-release-baseline.html
```

- [ ] **Step 3: Run timings for aptos CLI build**

```bash
cargo build --timings -p aptos --profile cli 2>&1
cp target/cargo-timings/cargo-timing.html docs/superpowers/build-timings/2026-03-21-aptos-cli-baseline.html
```

- [ ] **Step 4: Record key metrics from the HTML reports**

Open the HTML files and note:
- Total wall-clock build time
- Top 10 longest-compiling crates
- Critical path length
- Peak parallelism utilization

Create a summary in `docs/superpowers/build-timings/BASELINE.md`.

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/build-timings/
git commit -m "[build] Add cargo timings baseline measurements"
```

---

## Task 1: Upgrade base64 from 0.13 to 0.22

The base64 0.13 API uses free functions (`base64::encode()`, `base64::decode()`). The 0.22 API uses an engine-based approach. The migration pattern is:

```rust
// OLD (0.13):
use base64;
base64::encode(data)
base64::decode(data)
base64::encode_config(data, base64::URL_SAFE_NO_PAD)
base64::decode_config(data, base64::URL_SAFE)

// NEW (0.22):
use base64::{Engine as _, engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD}};
STANDARD.encode(data)
STANDARD.decode(data)
URL_SAFE_NO_PAD.encode(data)
URL_SAFE.decode(data)
```

**Files:**
- Modify: `Cargo.toml` (root, line ~534: workspace base64 version)
- Modify: `types/src/jwks/rsa/mod.rs`
- Modify: `types/src/keyless/mod.rs`
- Modify: `types/src/keyless/test_utils.rs`
- Modify: `crates/aptos-crypto/src/encoding_type.rs`
- Modify: `crates/aptos-github-client/src/lib.rs`
- Modify: `crates/transaction-emitter-lib/src/emitter/local_account_generator.rs`
- Modify: `sdk/src/types.rs`
- Modify: `crates/aptos-telemetry-service/src/tests/test_context.rs`
- Modify: `secure/storage/src/lib.rs`
- Modify: `secure/storage/vault/src/lib.rs`
- Modify: `crates/aptos/src/genesis/git.rs`
- Modify: `ecosystem/indexer-grpc/indexer-grpc-utils/src/cache_operator.rs`
- Modify: `ecosystem/indexer-grpc/indexer-grpc-utils/src/compression_util.rs`
- Modify: `testsuite/fuzzer/fuzz/fuzz_targets/move/utils/helpers.rs`

- Modify: `testsuite/fuzzer/Cargo.toml` (line 12: local `base64 = "0.21.7"` pin)

Note: Files in `protos/rust/src/pb/*.serde.rs` use `pbjson::private::base64::encode` — these are auto-generated and use pbjson's own base64, NOT the workspace base64 crate. **Do not modify these files.**

Note: `crates/aptos-cli-common/src/types.rs` uses `base64::DecodeError` which is stable across versions — no source changes needed there.

- [ ] **Step 1: Bump the workspace base64 version**

In `/Users/greg/git/aptos-core/Cargo.toml`, change:
```toml
# Old:
base64 = "0.13.0"
# New:
base64 = "0.22.1"
```

- [ ] **Step 2: Update `types/src/jwks/rsa/mod.rs`**

Replace:
```rust
use base64::URL_SAFE_NO_PAD;
```
With:
```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
```

Replace:
```rust
base64::decode_config(&self.n, URL_SAFE_NO_PAD)
```
With:
```rust
URL_SAFE_NO_PAD.decode(&self.n)
```

- [ ] **Step 3: Update `types/src/keyless/mod.rs`**

Replace:
```rust
use base64::URL_SAFE_NO_PAD;
```
With:
```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
```

Replace all `base64::encode_config(...)` calls:
```rust
// Old:
base64::encode_config(data.as_bytes(), URL_SAFE_NO_PAD)
base64::encode_config(data, URL_SAFE_NO_PAD)
// New:
URL_SAFE_NO_PAD.encode(data.as_bytes())
URL_SAFE_NO_PAD.encode(data)
```

Replace `base64::decode_config(...)`:
```rust
// Old:
base64::decode_config(b64, URL_SAFE_NO_PAD)?
// New:
URL_SAFE_NO_PAD.decode(b64)?
```

- [ ] **Step 4: Update `types/src/keyless/test_utils.rs`**

Replace:
```rust
use base64::{encode_config, URL_SAFE_NO_PAD};
```
With:
```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
```

And update all `encode_config(...)` calls to `URL_SAFE_NO_PAD.encode(...)`.

- [ ] **Step 5: Update `crates/aptos-crypto/src/encoding_type.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace:
```rust
base64::encode(key.to_bytes())
// →
STANDARD.encode(key.to_bytes())

base64::decode(string.trim())
// →
STANDARD.decode(string.trim())
```

- [ ] **Step 6: Update `crates/aptos-github-client/src/lib.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace all `base64::encode(...)` with `STANDARD.encode(...)`.
Replace all `base64::decode(...)` with `STANDARD.decode(...)`.

- [ ] **Step 7: Update `crates/transaction-emitter-lib/src/emitter/local_account_generator.rs`**

Add import:
```rust
use base64::{engine::general_purpose::{STANDARD, URL_SAFE}, Engine as _};
```

Replace:
```rust
base64::decode(parts[0])       → STANDARD.decode(parts[0])
base64::decode_config(parts[1], base64::URL_SAFE) → URL_SAFE.decode(parts[1])
```

- [ ] **Step 8: Update `sdk/src/types.rs`**

Add import:
```rust
use base64::{engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD}, Engine as _};
```

Replace:
```rust
base64::encode_config(jwt_sig, base64::URL_SAFE_NO_PAD)  → URL_SAFE_NO_PAD.encode(jwt_sig)
base64::decode_config(..., base64::URL_SAFE)?             → URL_SAFE.decode(...)?
base64::decode(parts.first()...)?                         → STANDARD.decode(parts.first()...)?
```

- [ ] **Step 9: Update `crates/aptos-telemetry-service/src/tests/test_context.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace `base64::encode("jwt_secret_key")` with `STANDARD.encode("jwt_secret_key")`.

- [ ] **Step 10: Update `secure/storage/src/lib.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace:
```rust
base64::encode(bytes) → STANDARD.encode(bytes)
base64::decode(s)     → STANDARD.decode(s)
```

- [ ] **Step 11: Update `secure/storage/vault/src/lib.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace all `base64::encode(...)` with `STANDARD.encode(...)`.
Replace all `base64::decode(...)` with `STANDARD.decode(...)`.

- [ ] **Step 12: Update `crates/aptos/src/genesis/git.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace:
```rust
base64::decode(...)  → STANDARD.decode(...)
base64::encode(...)  → STANDARD.encode(...)
```

- [ ] **Step 13: Update `ecosystem/indexer-grpc/indexer-grpc-utils/src/cache_operator.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace:
```rust
base64::encode(&buf) → STANDARD.encode(&buf)
```

- [ ] **Step 14: Update `ecosystem/indexer-grpc/indexer-grpc-utils/src/compression_util.rs`**

Add import:
```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};
```

Replace:
```rust
base64::encode(bytes)  → STANDARD.encode(bytes)
base64::decode(bytes)  → STANDARD.decode(bytes)
base64::decode(base64) → STANDARD.decode(base64)
```

- [ ] **Step 15: Update `testsuite/fuzzer/fuzz/fuzz_targets/move/utils/helpers.rs`**

Add import:
```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
```

Replace:
```rust
base64::encode_config(data.as_bytes(), base64::URL_SAFE_NO_PAD)
→ URL_SAFE_NO_PAD.encode(data.as_bytes())
```

- [ ] **Step 16: Fix testsuite/fuzzer local base64 pin**

In `/Users/greg/git/aptos-core/testsuite/fuzzer/Cargo.toml`, line 12, change:
```toml
# Old:
base64 = "0.21.7"
# New:
base64 = { workspace = true }
```

The fuzzer's fuzz targets already use the engine-based API (0.21 and 0.22 have nearly identical APIs), so no source changes should be needed. If the 0.21→0.22 API has minor differences, fix them in `testsuite/fuzzer/fuzz/fuzz_targets/move/utils/helpers.rs`.

- [ ] **Step 17: Verify compilation**

```bash
cargo check -p aptos-types && \
cargo check -p aptos-crypto && \
cargo check -p aptos-github-client && \
cargo check -p transaction-emitter-lib && \
cargo check -p aptos-sdk && \
cargo check -p aptos-telemetry-service && \
cargo check -p aptos-secure-storage && \
cargo check -p aptos-vault-client && \
cargo check -p aptos && \
cargo check -p aptos-indexer-grpc-utils
```

Expected: All pass.

- [ ] **Step 18: Run tests for affected crates**

```bash
cargo test -p aptos-types -- --test-threads=4 && \
cargo test -p aptos-crypto -- --test-threads=4 && \
cargo test -p aptos-github-client -- --test-threads=4 && \
cargo test -p aptos-secure-storage -- --test-threads=4 && \
cargo test -p aptos-vault-client -- --test-threads=4 && \
cargo test -p aptos-sdk -- --test-threads=4 && \
cargo test -p aptos-indexer-grpc-utils -- --test-threads=4
```

Expected: All pass.

- [ ] **Step 19: Commit**

```bash
git add Cargo.toml Cargo.lock \
  types/src/jwks/rsa/mod.rs \
  types/src/keyless/mod.rs \
  types/src/keyless/test_utils.rs \
  crates/aptos-crypto/src/encoding_type.rs \
  crates/aptos-github-client/src/lib.rs \
  crates/transaction-emitter-lib/src/emitter/local_account_generator.rs \
  sdk/src/types.rs \
  crates/aptos-telemetry-service/src/tests/test_context.rs \
  secure/storage/src/lib.rs \
  secure/storage/vault/src/lib.rs \
  crates/aptos/src/genesis/git.rs \
  ecosystem/indexer-grpc/indexer-grpc-utils/src/cache_operator.rs \
  ecosystem/indexer-grpc/indexer-grpc-utils/src/compression_util.rs \
  testsuite/fuzzer/fuzz/fuzz_targets/move/utils/helpers.rs \
  testsuite/fuzzer/Cargo.toml
git commit -m "$(cat <<'EOF'
[deps] Upgrade base64 from 0.13 to 0.22

Migrate from free-function API (base64::encode/decode) to
engine-based API (STANDARD.encode/decode, URL_SAFE_NO_PAD, etc.).
Eliminates the oldest of three base64 versions in the dep tree.
Also fixes testsuite/fuzzer local pin from 0.21 to workspace.
EOF
)"
```

---

## Task 2: Add Dev Profile Override for librocksdb-sys

**Files:**
- Modify: `Cargo.toml` (root, add profile section)

- [ ] **Step 1: Add the override**

In `/Users/greg/git/aptos-core/Cargo.toml`, add after the existing `[profile.bench]` section (around line 960):

```toml
# Speed up dev builds by reducing RocksDB optimization level.
# RocksDB is unusably slow at -O0 but doesn't need full -O2 for dev work.
[profile.dev.package.librocksdb-sys]
opt-level = 1
```

- [ ] **Step 2: Verify dev build still works**

```bash
cargo check -p aptos-node
```

Expected: Passes. The override only affects optimization level, not correctness.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "[build] Add dev profile override for librocksdb-sys

Set opt-level=1 for librocksdb-sys in dev profile to speed up
debug builds while keeping RocksDB functional (unusably slow at -O0)."
```

---

## Verification Checklist

After all tasks are complete:

- [ ] `cargo check --workspace` passes
- [ ] `cargo tree -d 2>&1 | grep -c "^[a-z]"` shows fewer duplicate crate names than before (baseline: 127)
- [ ] `cargo tree -i base64 2>&1` shows no `base64 v0.13` entries from workspace crates
- [ ] `cargo tree -i base64 2>&1` shows no `base64 v0.21` entries from workspace crates (fuzzer pin fixed)
- [ ] Full lint check: `./scripts/rust_lint.sh --check` passes

## Deferred Items (for Tier 2)

- **num-bigint 0.3 → 0.4**: Blocked by `rand 0.7 → 0.8` workspace-wide upgrade. num-bigint 0.4's `rand` feature requires `rand 0.8`, which is incompatible with the workspace's `rand = "0.7.3"`. This is a large-scope change touching many crates.
