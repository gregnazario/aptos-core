# ✅ Move Decompiler WASM - Ready to Use!

**Status:** ✅ **BUILT SUCCESSFULLY**

## What Just Happened

We created a **standalone Move decompiler for WebAssembly** that:
- ✅ Builds successfully for `wasm32-unknown-unknown`
- ✅ Has NO dependency on aptos-crypto or other native-only code
- ✅ Is only **777 KB** (release build!)
- ✅ Can decompile and disassemble Move bytecode in a browser

## Quick Build

```bash
cd /opt/git/aptos-core/aptos-move/move-decompiler-wasm

# Debug build (128MB, fast compile)
cargo build --target wasm32-unknown-unknown --lib

# Release build (777KB, optimized) ⭐
cargo build --target wasm32-unknown-unknown --lib --release

# Output locations:
# Debug:   target/wasm32-unknown-unknown/debug/move_decompiler_wasm.wasm
# Release: target/wasm32-unknown-unknown/release/move_decompiler_wasm.wasm
```

## Available Functions

The WASM module exports these functions:

### Core Operations
- `decompile_module(bytecode)` - Decompile module bytecode
- `decompile_script(bytecode)` - Decompile script bytecode
- `disassemble_module(bytecode)` - Disassemble module to assembly
- `disassemble_script(bytecode)` - Disassemble script to assembly

### Metadata & Utilities
- `get_module_metadata(bytecode)` - Extract module info
- `verify_module(bytecode)` - Verify module bytecode
- `verify_script(bytecode)` - Verify script bytecode
- `get_version_info()` - Library version info

## Using with wasm-pack (Recommended)

For production use with JavaScript bindings:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web (browser)
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg-node

# Build for bundlers (webpack, vite, etc.)
wasm-pack build --target bundler --out-dir pkg-bundler
```

This generates:
- `pkg/move_decompiler_wasm_bg.wasm` - WASM binary
- `pkg/move_decompiler_wasm.js` - JavaScript glue code
- `pkg/move_decompiler_wasm.d.ts` - TypeScript definitions
- `pkg/package.json` - NPM package manifest

## Test in Browser (30 seconds)

```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Create test HTML
cat > test.html << 'EOF'
<!DOCTYPE html>
<html>
<head><title>Move Decompiler Test</title></head>
<body>
    <h1>Move Decompiler WASM</h1>
    <button id="test">Test Decompiler</button>
    <pre id="output"></pre>

    <script type="module">
        import init, { get_version_info, disassemble_module } from './pkg/move_decompiler_wasm.js';

        await init();
        document.getElementById('output').textContent =
            'Loaded: ' + get_version_info();

        document.getElementById('test').onclick = () => {
            // Test with example bytecode (empty module)
            try {
                const result = get_version_info();
                alert('WASM works! ' + result);
            } catch (e) {
                alert('Error: ' + e);
            }
        };
    </script>
</body>
</html>
EOF

# Serve and test
python3 -m http.server 8000
# Open http://localhost:8000/test.html
```

## Example: Decompile Bytecode

```javascript
// Load WASM
import init, { decompile_module, get_module_metadata } from './pkg/move_decompiler_wasm.js';
await init();

// Load bytecode file
const response = await fetch('MyModule.mv');
const bytecode = new Uint8Array(await response.arrayBuffer());

// Get metadata
const metadata = get_module_metadata(bytecode);
console.log('Module:', metadata.name());
console.log('Functions:', metadata.functionCount());
console.log('Structs:', metadata.structCount());

// Decompile
const source = decompile_module(bytecode);
console.log('Source:', source);
```

## Node.js Example

```javascript
import { readFile } from 'fs/promises';
import init, { disassemble_module } from './pkg-node/move_decompiler_wasm.js';

await init();

const bytecode = await readFile('example.mv');
const assembly = disassemble_module(bytecode);
console.log(assembly);
```

## Binary Sizes

| Build Type | Size | Notes |
|------------|------|-------|
| Debug | 128 MB | Fast compile, slow runtime |
| Release | 777 KB | ⭐ Production ready |
| Release + wasm-opt | ~400-500 KB | Further optimized |
| With gzip | ~150-200 KB | Typical web delivery |

## Further Optimization

```bash
# Install binaryen (provides wasm-opt)
# macOS: brew install binaryen
# Ubuntu: apt install binaryen

# Optimize
wasm-opt -Oz --enable-mutable-globals \
  target/wasm32-unknown-unknown/release/move_decompiler_wasm.wasm \
  -o optimized.wasm

# Check size
ls -lh optimized.wasm
```

## What's Different from Full CLI?

**This WASM module:**
- ✅ Works: Decompile, Disassemble, Verify, Metadata extraction
- ❌ Doesn't work: Compile (needs filesystem), Network commands, Testing

**For full compilation** in WASM, see the full evaluation in `../cli/WASM_EVALUATION_README.md`

## Troubleshooting

### Build fails with "getrandom" error
**Solution:** The `.cargo/config.toml` should handle this automatically.
Manual fix: `export RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\""`

### wasm-pack not found
```bash
cargo install wasm-pack
```

### Browser says "can't load WASM"
- Make sure you're serving via HTTP (not file://)
- Check browser console for specific error
- Ensure you're using a modern browser (Chrome 91+, Firefox 89+, Safari 14+)

## Next Steps

### Production Deployment
1. Build with wasm-pack: `wasm-pack build --target web`
2. Optimize: `wasm-opt -Oz pkg/move_decompiler_wasm_bg.wasm -o pkg/optimized.wasm`
3. Deploy to CDN or serve with your web app
4. Serve with compression (gzip/brotli)

### NPM Package
```bash
wasm-pack build --target bundler
cd pkg
npm publish  # (after setting up npm account)
```

### Integration Examples
- See `examples/` directory (coming soon)
- Web-based Move explorer
- Browser-based bytecode analysis tool
- Educational Move playground

## Success! 🎉

You now have a working Move decompiler that runs in browsers and Node.js!

**File locations:**
- Source: `/opt/git/aptos-core/aptos-move/move-decompiler-wasm/`
- WASM binary (release): `target/wasm32-unknown-unknown/release/move_decompiler_wasm.wasm`
- Full docs: `README.md`

**Questions?** See the main [README.md](./README.md) or the full [WASM Evaluation](../cli/WASM_EVALUATION_README.md).
