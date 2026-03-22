# Cargo Build Timings — After Tier 2 (Move Compiler Decoupling)

**Date:** 2026-03-22
**Branch:** `fix-build-times` (after Tier 2 changes)
**Machine:** Apple Silicon Mac
**Profile:** dev (unoptimized + debuginfo)
**Target:** `aptos-node`

## Summary

| Metric | Baseline | After Tier 2 | Change |
|--------|----------|--------------|--------|
| Wall clock (cold build) | 2m 00s | ~1m 56s | ~3% faster |
| Crates compiled | 1,186 | 829 | **-357 (30% fewer)** |

## What Changed

The entire Move compiler chain is eliminated from `aptos-node`'s dependency graph:

| Crate | Baseline Time | After Tier 2 |
|-------|-------------|--------------|
| move-model (42K lines) | 13.7s | **not compiled** |
| move-compiler-v2 (49K lines) | 14.8s | **not compiled** |
| legacy-move-compiler (20K lines) | 7.7s | **not compiled** |
| move-prover-bytecode-pipeline | 6.9s | **not compiled** |
| move-stackless-bytecode | 6.6s | **not compiled** |
| move-prover-boogie-backend | 5.4s | **not compiled** |
| move-docgen | ~2s | **not compiled** |
| move-prover | ~2s | **not compiled** |
| move-prover-lab | ~1s | **not compiled** |

Total CPU time saved: ~60s across these crates.

## Why Wall Clock Improvement Is Modest

The eliminated crates were **not on the critical path** — they built in parallel with
heavier bottlenecks (librocksdb-sys 40s, tikv-jemalloc-sys 28s, aptos-consensus 30s).
With 7-10x parallelism, removing 60s of parallel work saves only a few seconds of wall time.

## Where The Real Wins Are

1. **Incremental builds**: Changes to `move-model` or the Move compiler no longer
   trigger rebuilds of `aptos-node` or its ~87 downstream dependents via `aptos-types`.

2. **Dependency isolation**: `aptos-types` now depends on `move-model-types` (1 file,
   ~50 lines) instead of `move-model` (42K lines + `legacy-move-compiler` 20K lines).

3. **Reduced compilation surface**: 30% fewer crates means faster `cargo check`,
   faster dependency resolution, and less disk usage.

4. **Feature-gated prover**: `aptos-framework` consumers that don't need formal
   verification skip 4 prover crates (~19s) by not enabling the `prover` feature.

## Changes Made

1. **move-model-types crate** — extracted `CompilationMetadata` + `COMPILATION_METADATA_KEY`
2. **aptos-types** — depends on `move-model-types` instead of `move-model`
3. **aptos-release-bundle** — replaced `CodeWriter` with local `CodeEmitter`
4. **aptos-node & aptos-genesis** — depend on `aptos-release-bundle` instead of `aptos-framework`
5. **aptos-framework** — prover deps optional behind `prover` feature flag
6. **move-model** — re-exports from `move-model-types`, `CompilationMetadataExt` trait

## HTML Report

Full interactive report: `2026-03-21-tier2-post-decoupling.html`
