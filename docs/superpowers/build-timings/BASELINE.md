# Cargo Build Timings Baseline

**Date:** 2026-03-21
**Branch:** `fix-build-times` (after Tier 1 changes)
**Machine:** Apple Silicon Mac
**Profile:** dev (unoptimized + debuginfo)
**Target:** `aptos-node`

## Summary

| Metric | Value |
|--------|-------|
| Wall clock (cold build) | **2m 00s** |
| Total compilation time (sum all crates) | 839s |
| Crates compiled | 1,186 |
| Parallelism factor | 7.0x |

## Top 25 Slowest Crates

| Rank | Crate | Duration (s) | Notes |
|------|-------|-------------|-------|
| 1 | librocksdb-sys 0.17.3 | 40.4 | C++ compilation (735 files) |
| 2 | aptos-consensus 0.1.0 | 30.5 | Large Rust crate |
| 3 | tikv-jemalloc-sys 0.6.1 | 28.5 | C compilation |
| 4 | aptos-api 0.2.0 | 24.5 | poem-openapi derive macros |
| 5 | aptos-executor-service 0.1.0 | 19.5 | |
| 6 | aptos-types 0.0.3 | 17.8 | Heavy deps (move-model) |
| 7 | libgit2-sys 0.14.2 | 17.4 | C compilation |
| 8 | libgit2-sys 0.14.2 | 17.2 | (duplicate build?) |
| 9 | move-compiler-v2 0.1.0 | 14.8 | 49K lines Rust |
| 10 | move-model 0.1.0 | 13.7 | 42K lines Rust |
| 11 | aptos-telemetry-service 0.1.0 | 12.4 | |
| 12 | zstd-sys 2.0.9 | 9.2 | C compilation |
| 13 | ring 0.16.20 | 8.0 | C/ASM crypto |
| 14 | legacy-move-compiler 0.0.1 | 7.7 | 20K lines Rust |
| 15 | aptos-config 0.1.0 | 7.5 | |
| 16 | aptos-crypto 0.0.3 | 7.2 | |
| 17 | move-prover-bytecode-pipeline 0.1.0 | 6.9 | Prover (could be feature-gated) |
| 18 | move-stackless-bytecode 0.1.0 | 6.6 | Prover (could be feature-gated) |
| 19 | aptos-data-client 0.1.0 | 6.4 | |
| 20 | aptos-indexer-grpc-fullnode 1.0.0 | 6.4 | Ecosystem (could be feature-gated) |
| 21 | aptos-network 0.1.0 | 6.1 | |
| 22 | aptos-api-types 0.0.1 | 5.9 | |
| 23 | aptos-indexer-grpc-utils 1.0.0 | 5.6 | Ecosystem |
| 24 | move-prover-boogie-backend 0.1.0 | 5.4 | Prover (could be feature-gated) |
| 25 | aptos-framework-natives 0.1.0 | 5.3 | |

## Key Observations

### Native C/C++ compilation dominates
The top 3 slowest crates are all C/C++ compilation:
- **librocksdb-sys** (40.4s) — single biggest bottleneck
- **tikv-jemalloc-sys** (28.5s) — jemalloc allocator
- **libgit2-sys** (17.4s) — git library (appears twice?)
- **zstd-sys** (9.2s) — compression
- **ring** (8.0s) — crypto

Total native compilation: ~120s out of 839s total (14% of CPU time)

### Move compiler chain on critical path
- move-compiler-v2 (14.8s) + move-model (13.7s) + legacy-move-compiler (7.7s) = **36.2s**
- These are compiled for aptos-node even though it only needs the release bundle

### Prover crates are unnecessary for node build
- move-prover-bytecode-pipeline (6.9s) + move-stackless-bytecode (6.6s) + move-prover-boogie-backend (5.4s) = **18.9s**
- Feature-gating these (Tier 2, Item 2.5) would save ~19s

### Tier 2 impact estimates (validated by data)
| Tier 2 Item | Crates Eliminated | Time Saved |
|-------------|-------------------|------------|
| 2.1 Decouple aptos-types from Move compiler | move-model (13.7s), legacy-move-compiler (7.7s) | ~21s |
| 2.2 Decouple aptos-release-bundle | move-compiler-v2 (14.8s) + prover chain | ~36s |
| 2.5 Feature-gate prover | 3 prover crates | ~19s |
| Combined 2.1 + 2.2 + 2.5 | All Move toolchain | ~55s (of 120s wall clock) |

## HTML Report

Full interactive report: `2026-03-21-aptos-node-dev-cold-baseline.html`
