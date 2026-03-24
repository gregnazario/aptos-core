# Speeding Up Builds with sccache

## Problem

Clean builds of `aptos-node` spend ~95s compiling C/C++/ASM from bundled source code in `-sys` crates. These crates rarely change — their source is pinned to specific versions — but `cargo clean` or a fresh checkout recompiles everything from scratch.

## Solution

Use `sccache` to cache only the C/C++ compilation by setting `CC` and `CXX` environment variables. This leaves Rust compilation untouched (no interference with incremental builds) while caching the expensive native code.

## Setup

### Install

```bash
# macOS
brew install sccache

# Linux
cargo install sccache --locked
```

### Configure (add to shell profile)

```bash
# ~/.zshrc or ~/.bashrc
export CC="sccache cc"
export CXX="sccache c++"
```

That's it. No `RUSTC_WRAPPER` needed — we only wrap the C/C++ compiler.

### Verify

```bash
# After a clean build:
sccache --show-stats
# Should show cache hits for C/C++ compilations
```

## What Gets Cached

All of these crates bundle C/C++/ASM source and compile it from scratch on every clean build:

| Package | Build time (cold) | What it compiles |
|---------|-------------------|------------------|
| librocksdb-sys | 48.4s | 329 C++ files (RocksDB 10.4.2) |
| tikv-jemalloc-sys | 18.1s | C (jemalloc 5.3.0) |
| zstd-sys | 9.0s | C (Zstandard 1.5.5) |
| ring 0.16.20 | 7.7s | C/ASM (crypto) |
| lz4-sys | 4.0s | C (LZ4 1.10.0) |
| ring 0.17.7 | 3.7s | C/ASM (crypto) |
| libz-sys | 1.6s | C (zlib) |
| blst | 1.5s | C (BLS12-381 crypto) |
| bzip2-sys | 1.1s | C (bzip2) |
| **Total** | **~95s** | **442 compilation units** |

On a warm cache, all 442 compilations are served from disk in ~1s total.

## What Does NOT Change

- **Rust compilation** — completely unaffected, incremental builds work as before
- **Runtime behavior** — identical binaries, same code, same linking
- **Build correctness** — sccache keys on source hash + compiler flags + env vars; if anything changes, it recompiles

## Results (Apple Silicon Mac, 16 cores)

| Build | Wall clock | Notes |
|-------|-----------|-------|
| No sccache (baseline) | 1m 55s | All C/C++ compiled from source |
| sccache cold (first run) | 2m 03s | +8s overhead populating cache |
| **sccache warm** | **1m 27s** | **28s faster (24% improvement)** |

## Cache Location and Size

Default: `~/Library/Caches/Mozilla.sccache/` (macOS) or `~/.cache/sccache/` (Linux)

The aptos-node cache is ~138MB. Default max cache size is 10GB.

```bash
# Check stats
sccache --show-stats

# Clear cache if needed
sccache --zero-stats
rm -rf ~/Library/Caches/Mozilla.sccache/
```

## Static Linking Details

All the cached crates use **static linking** — they compile C/C++ source into `.a` archives that get linked into the final binary. None of them link against system-installed libraries at runtime.

| Crate | Linking | Features |
|-------|---------|----------|
| librocksdb-sys | static (bundled RocksDB) | `static`, `bzip2`, `jemalloc`, `lz4`, `snappy`, `zlib`, `zstd` |
| tikv-jemalloc-sys | static (bundled jemalloc) | `default`, `profiling`, `stats` |
| zstd-sys | static (bundled zstd) | `legacy`, `std`, `zdict_builder` |
| lz4-sys | static (bundled lz4) | default |
| libz-sys | static (bundled zlib) | `static` |
| bzip2-sys | static (bundled bzip2) | `static` |
| ring | static (bundled C/ASM) | default |
| blst | static (bundled C) | default |

This means:
- No system library version mismatches at runtime
- Binaries are self-contained (good for Docker, distribution)
- But every clean build recompiles everything from source (why sccache helps)

The non-compiling `-sys` crates in the tree (`core-foundation-sys`, `security-framework-sys`, `system-configuration-sys`, `clang-sys`) only generate FFI bindings and link against macOS system frameworks — they don't compile C code and aren't affected by sccache.

## Quick Setup Script

```bash
./scripts/sccache_setup.sh          # Install + print CC/CXX config
./scripts/sccache_setup.sh --rust   # Also print RUSTC_WRAPPER config
```

## CI Usage

CI uses two composite actions that install sccache and configure both C/C++ and Rust caching:

- **`lint-test.yaml` jobs** use `.github/actions/sccache-setup` (standalone action)
- **Other workflows** use sccache steps built into `.github/actions/rust-setup`

Both actions configure:
- `RUSTC_WRAPPER=sccache` — caches Rust compilation units via GitHub Actions cache
- `CC_x86_64_unknown_linux_gnu=sccache clang` — caches C compilations
- `CXX_x86_64_unknown_linux_gnu=sccache clang++` — caches C++ compilations

Coverage builds pass `sccache-cache-rust: "false"` because `cargo-llvm-cov` is incompatible with `RUSTC_WRAPPER`.

```yaml
# Standalone usage (lint-test.yaml pattern):
- uses: ./.github/actions/sccache-setup

# Within rust-setup (other workflows):
- uses: ./.github/actions/rust-setup
  with:
    sccache-cache-rust: "false"  # only for coverage builds
```

The `mozilla-actions/sccache-action@v0.0.9` action uses GitHub Actions cache as the storage backend automatically (`SCCACHE_GHA_ENABLED=true`).
