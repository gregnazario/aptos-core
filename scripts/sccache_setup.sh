#!/usr/bin/env bash
# sccache_setup.sh — Install and configure sccache for local Aptos Core development
#
# Usage:
#   ./scripts/sccache_setup.sh          # Install + print CC/CXX config (S3 shared cache)
#   ./scripts/sccache_setup.sh --rust   # Also print RUSTC_WRAPPER config
#   ./scripts/sccache_setup.sh --local  # Use local disk cache only (no S3)
#
# By default, sccache is configured to use the shared S3 bucket so that
# C/C++ compilation artifacts (RocksDB, jemalloc, ring, etc.) are shared
# across all developers.  This saves ~95s per clean build.
#
# AWS credentials (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY) must be set
# in your environment for S3 access.  Without them, sccache falls back to
# local-only caching automatically (no error, just no sharing).
#
# SCCACHE_BASEDIRS is set to the checkout root so that cache keys are
# path-independent — multiple checkouts of aptos-core on the same machine
# (or across machines) share cache hits.
#
# See docs/superpowers/build-timings/SCCACHE.md for details.

set -euo pipefail

INCLUDE_RUST=false
USE_LOCAL=false

for arg in "$@"; do
  case "$arg" in
    --rust) INCLUDE_RUST=true ;;
    --local) USE_LOCAL=true ;;
    -h|--help)
      echo "Usage: $0 [--rust] [--local]"
      echo ""
      echo "Install sccache and print shell configuration lines."
      echo ""
      echo "Options:"
      echo "  --rust   Also configure RUSTC_WRAPPER (disables incremental compilation)"
      echo "  --local  Use local disk cache only (skip S3 shared cache)"
      echo "  -h       Show this help"
      echo ""
      echo "Environment variables (for S3 mode):"
      echo "  AWS_ACCESS_KEY_ID       AWS credentials for shared S3 cache"
      echo "  AWS_SECRET_ACCESS_KEY   AWS credentials for shared S3 cache"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg"
      exit 1
      ;;
  esac
done

# --- Detect project root ---
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# --- Install sccache ---
if command -v sccache &>/dev/null; then
  echo "sccache already installed: $(sccache --version)"
else
  echo "Installing sccache..."
  case "$(uname -s)" in
    Darwin)
      if command -v brew &>/dev/null; then
        brew install sccache
      else
        echo "Homebrew not found. Install via: cargo install sccache --locked"
        exit 1
      fi
      ;;
    Linux)
      cargo install sccache --locked
      ;;
    *)
      echo "Unsupported platform: $(uname -s)"
      exit 1
      ;;
  esac
  echo "Installed: $(sccache --version)"
fi

# --- Print config ---
echo ""
echo "Add the following to your shell profile (~/.zshrc or ~/.bashrc):"
echo ""

if [ "$USE_LOCAL" = false ]; then
  echo "  # sccache: S3 shared build cache"
  echo "  export SCCACHE_BUCKET=aptos-sccache-shared"
  echo "  export SCCACHE_REGION=us-west-2"
  echo "  export SCCACHE_S3_USE_SSL=true"
  echo ""
fi

echo "  # Normalize checkout path for cache key (enables sharing across checkouts)"
echo "  # If you have multiple checkouts, separate them with ':'"
echo "  #   e.g. /path/to/aptos-core:/path/to/aptos-core-2"
echo "  export SCCACHE_BASEDIRS=$PROJECT_ROOT"
echo ""
echo "  # sccache: cache C/C++ compilations for -sys crates (~95s savings)"
echo "  export CC=\"sccache cc\""
echo "  export CXX=\"sccache c++\""

if [ "$INCLUDE_RUST" = true ]; then
  echo ""
  echo "  # sccache: also wrap rustc (disables incremental compilation)"
  echo "  export RUSTC_WRAPPER=sccache"
  echo ""
  echo "  # Note: RUSTC_WRAPPER is incompatible with incremental compilation."
  echo "  # Only use this if you frequently do clean builds or work on remote machines."
fi

if [ "$USE_LOCAL" = false ]; then
  echo ""
  echo "  # AWS credentials for shared S3 cache (get from team lead)"
  echo "  export AWS_ACCESS_KEY_ID=<your-key>"
  echo "  export AWS_SECRET_ACCESS_KEY=<your-secret>"
fi

echo ""
echo "Then restart your shell or run: source ~/.zshrc"
