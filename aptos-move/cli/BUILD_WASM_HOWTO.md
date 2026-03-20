# How to Build aptos-move-cli for WASM

**Status:** Currently blocked by dependencies. Here's how to attempt it and what's needed to fix it.

## Current Build Attempt

```bash
cd /opt/git/aptos-core/aptos-move/cli

# Set getrandom backend for WASM
export RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""

# Attempt WASM build
cargo build \
  --target wasm32-unknown-unknown \
  --lib \
  --no-default-features \
  --features wasm-eval
```

**Expected Result:** ❌ Build fails at `wait-timeout` crate

## Why It Fails

**Dependency Chain:**
```
aptos-move-cli
  ↓ aptos-crypto
    ↓ proptest (property testing)
      ↓ rusty-fork (process forking)
        ↓ wait-timeout ❌ (no WASM support)
```

The `wait-timeout` crate requires process management (Unix/Windows only), which doesn't exist in WASM.

## Quick Test: Core Move Dependencies

To verify core Move dependencies ARE WASM-compatible:

```bash
# Test move-binary-format (WILL WORK)
cd /opt/git/aptos-core/third_party/move/move-binary-format
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" \
  cargo build --target wasm32-unknown-unknown --lib

# Success! Check output
ls -lh target/wasm32-unknown-unknown/debug/libmove_binary_format.rlib
```

## How to Fix: Make Dependencies Optional

### Step 1: Make Aptos Dependencies Optional

Edit `Cargo.toml`:

```toml
[dependencies]
# ... existing deps ...

# Make these optional for WASM
aptos-crypto = { workspace = true, optional = true }
aptos-api-types = { workspace = true, optional = true }
aptos-rest-client = { workspace = true, optional = true }
aptos-sdk = { workspace = true, optional = true }
aptos-vm = { workspace = true, optional = true }
aptos-transaction-simulation = { workspace = true, optional = true }
tokio = { workspace = true, optional = true }
tempfile = { workspace = true, optional = true }

[features]
default = ["network", "crypto", "simulation"]
binary = ["testing"]
testing = ["aptos-vm/testing"]

# WASM features (minimal deps)
wasm-minimal = []
wasm-compiler = ["wasm-minimal", "move-compiler-v2", "move-package"]
wasm-eval = ["wasm-minimal"]

# Native-only features
network = ["aptos-rest-client", "aptos-sdk", "tokio"]
crypto = ["aptos-crypto"]
simulation = ["aptos-vm", "aptos-transaction-simulation"]
```

### Step 2: Add Conditional Compilation

Edit `src/lib.rs`:

```rust
// Crypto features
#[cfg(feature = "crypto")]
pub use aptos_crypto;

#[cfg(feature = "network")]
pub use aptos_rest_client;

// WASM-only exports
#[cfg(target_arch = "wasm32")]
pub mod wasm_api;
```

### Step 3: Update Commands

Edit files that use optional dependencies:

```rust
// In commands.rs, bytecode.rs, etc.
#[cfg(feature = "crypto")]
use aptos_crypto::HashValue;

#[cfg(not(feature = "crypto"))]
compile_error!("This command requires the 'crypto' feature");
```

### Step 4: Build WASM

```bash
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" \
  cargo build \
  --target wasm32-unknown-unknown \
  --lib \
  --no-default-features \
  --features wasm-minimal
```

**Estimated effort:** 2-3 hours to make all dependencies optional

## Alternative: Use wasm-pack

For a production WASM build with JavaScript bindings:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build with wasm-pack (after fixing dependencies)
wasm-pack build \
  --target web \
  --out-dir pkg \
  --no-default-features \
  --features wasm-minimal

# Output will be in pkg/
ls pkg/
# → aptos_move_cli_bg.wasm  (binary)
# → aptos_move_cli.js        (JavaScript glue)
# → aptos_move_cli.d.ts      (TypeScript types)
```

## Recommended: Create Separate Crate

Instead of modifying aptos-move-cli, create a new lightweight crate:

```bash
# Create new WASM-focused crate
cd /opt/git/aptos-core/aptos-move
cargo new --lib wasm-cli

# In wasm-cli/Cargo.toml
[dependencies]
move-binary-format = { workspace = true }
move-core-types = { workspace = true }
move-compiler-v2 = { workspace = true }
move-decompiler = { workspace = true }
wasm-bindgen = { workspace = true }
getrandom = { version = "0.2", features = ["js"] }

[lib]
crate-type = ["cdylib", "rlib"]
```

This approach:
- ✅ No risk to existing CLI
- ✅ Clean dependency tree
- ✅ Will build successfully for WASM
- ✅ Can iterate independently

## Test the WASM Output

After successful build:

```bash
# Check binary size
ls -lh target/wasm32-unknown-unknown/release/aptos_move_cli.wasm

# Optimize with wasm-opt (install from binaryen)
wasm-opt -Oz \
  target/wasm32-unknown-unknown/release/aptos_move_cli.wasm \
  -o optimized.wasm

# Compare sizes
ls -lh optimized.wasm

# Test in Node.js
node -e "
const fs = require('fs');
const wasm = fs.readFileSync('pkg/aptos_move_cli_bg.wasm');
WebAssembly.compile(wasm).then(module => {
  console.log('WASM module compiled successfully!');
  console.log('Exports:', WebAssembly.Module.exports(module));
});
"
```

## Troubleshooting

### Error: "getrandom not configured"
**Solution:** Add `RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""`

### Error: "wait-timeout failed to compile"
**Solution:** Make `aptos-crypto` optional (it pulls in `proptest` → `wait-timeout`)

### Error: "tokio doesn't support wasm32-unknown-unknown"
**Solution:** Make `tokio` optional or use `wasm-bindgen-futures` instead

### Binary size too large (>100MB)
**Solution:**
1. Use `--release` mode
2. Run `wasm-opt -Oz`
3. Enable LTO in Cargo.toml: `lto = true`
4. Use code splitting

## Current Status

**Native build:** ✅ Works perfectly
**WASM build:** ❌ Blocked by `wait-timeout` via `aptos-crypto`

**To unblock:**
- Option A: Make dependencies optional (2-3 hours)
- Option B: Create separate wasm-cli crate (1 day) ← **Recommended**

See **PHASE5_FINAL_RECOMMENDATION.md** for full implementation plan.
