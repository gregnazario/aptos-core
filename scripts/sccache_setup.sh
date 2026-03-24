#!/usr/bin/env bash
# sccache_setup.sh — Install and configure sccache for local Aptos Core development
#
# Usage:
#   ./scripts/sccache_setup.sh                # Install + print CC/CXX config (local disk cache)
#   ./scripts/sccache_setup.sh --rust         # Also print RUSTC_WRAPPER config
#   ./scripts/sccache_setup.sh --shelby       # Use Shelby decentralized cache backend
#   ./scripts/sccache_setup.sh --shelby-start # Start the shelby-cache-proxy (requires clone)
#
# The --shelby flag configures sccache to route through a local shelby-cache-proxy
# that stores artifacts on the Shelby network (persistent, shared across machines).
# See: https://github.com/gregnazario/shelby-sccache
#
# See docs/superpowers/build-timings/SCCACHE.md for details.

set -euo pipefail

INCLUDE_RUST=false
USE_SHELBY=false
START_SHELBY=false
SHELBY_PORT="${SHELBY_CACHE_SERVER_PORT:-9000}"
SHELBY_DIR="${SHELBY_SCCACHE_DIR:-$HOME/git/shelby-sccache}"

for arg in "$@"; do
  case "$arg" in
    --rust) INCLUDE_RUST=true ;;
    --shelby) USE_SHELBY=true; INCLUDE_RUST=true ;;
    --shelby-start) START_SHELBY=true; USE_SHELBY=true; INCLUDE_RUST=true ;;
    -h|--help)
      echo "Usage: $0 [--rust] [--shelby] [--shelby-start]"
      echo ""
      echo "Install sccache and print shell configuration lines."
      echo ""
      echo "Options:"
      echo "  --rust          Also configure RUSTC_WRAPPER (disables incremental compilation)"
      echo "  --shelby        Configure sccache to use Shelby decentralized cache"
      echo "  --shelby-start  Clone (if needed), install, and start the shelby-cache-proxy"
      echo "  -h              Show this help"
      echo ""
      echo "Environment variables:"
      echo "  SHELBY_SCCACHE_DIR          Path to shelby-sccache clone (default: ~/git/shelby-sccache)"
      echo "  SHELBY_CACHE_SERVER_PORT    Proxy port (default: 9000)"
      echo "  APTOS_PRIVATE_KEY           Required for --shelby-start"
      echo "  SHELBY_API_KEY              Required for --shelby-start"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg"
      exit 1
      ;;
  esac
done

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

# --- Shelby proxy setup ---
if [ "$START_SHELBY" = true ]; then
  if ! command -v bun &>/dev/null; then
    echo "Bun not found. Installing..."
    curl -fsSL https://bun.sh/install | bash
    export PATH="$HOME/.bun/bin:$PATH"
  fi

  if [ ! -d "$SHELBY_DIR" ]; then
    echo "Cloning shelby-sccache to $SHELBY_DIR..."
    git clone https://github.com/gregnazario/shelby-sccache.git "$SHELBY_DIR"
  fi

  echo "Installing dependencies..."
  (cd "$SHELBY_DIR" && bun install)

  echo "Initializing shelby-cache-proxy..."
  (cd "$SHELBY_DIR" && bun run src/cli/index.ts init 2>/dev/null || true)

  if [ -z "${APTOS_PRIVATE_KEY:-}" ] || [ -z "${SHELBY_API_KEY:-}" ]; then
    echo ""
    echo "WARNING: APTOS_PRIVATE_KEY and SHELBY_API_KEY must be set to start the proxy."
    echo "Export them in your shell, then run:"
    echo "  cd $SHELBY_DIR && bun run src/cli/index.ts start"
  else
    echo "Starting shelby-cache-proxy on port $SHELBY_PORT..."
    (cd "$SHELBY_DIR" && \
      SHELBY_CACHE_SERVER_PORT="$SHELBY_PORT" \
      bun run src/cli/index.ts start &)
    sleep 2
    echo "Proxy started on http://localhost:$SHELBY_PORT"
  fi
fi

# --- Print config ---
echo ""
echo "Add the following to your shell profile (~/.zshrc or ~/.bashrc):"
echo ""

if [ "$USE_SHELBY" = true ]; then
  echo "  # sccache: Shelby decentralized build cache"
  echo "  export SCCACHE_BUCKET=shelby"
  echo "  export SCCACHE_ENDPOINT=http://localhost:$SHELBY_PORT"
  echo "  export SCCACHE_REGION=shelbyland"
  echo "  export SCCACHE_S3_USE_SSL=false"
  echo "  export SCCACHE_S3_NO_CREDENTIALS=false"
  echo "  export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE"
  echo "  export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
  echo ""
  echo "  # Wrap compilers with sccache"
  echo "  export CC=\"sccache cc\""
  echo "  export CXX=\"sccache c++\""
  echo "  export RUSTC_WRAPPER=sccache"
  echo ""
  echo "  # Start shelby-cache-proxy (run once per session):"
  echo "  # cd $SHELBY_DIR && bun run src/cli/index.ts start &"
else
  echo "  # sccache: cache C/C++ compilations for -sys crates (~95s savings)"
  echo "  export CC=\"sccache cc\""
  echo "  export CXX=\"sccache c++\""

  if [ "$INCLUDE_RUST" = true ]; then
    echo ""
    echo "  # sccache: also wrap rustc (disables incremental compilation)"
    echo "  export RUSTC_WRAPPER=sccache"
    echo ""
    echo "  # Note: RUSTC_WRAPPER is incompatible with incremental compilation."
    echo "  # Only use this if you frequently do clean builds."
  fi
fi

echo ""
echo "Then restart your shell or run: source ~/.zshrc"
