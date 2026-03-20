# Move Decompiler WASM - Demo Web App

Beautiful web interface for testing the Move decompiler in your browser!

## Quick Start

### 1. Build the WASM module

```bash
cd /opt/git/aptos-core/aptos-move/move-decompiler-wasm

# Install wasm-pack if needed
cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir pkg
```

### 2. Serve the demo

```bash
# From the move-decompiler-wasm directory
python3 -m http.server 8000

# Or use any web server
# npx serve .
# php -S localhost:8000
```

### 3. Open in browser

```
http://localhost:8000/demo/
```

## Features

✨ **Drag & Drop** - Drop .mv files directly into the browser
📊 **Metadata View** - See module name, version, function count, dependencies
🔍 **Decompiled Source** - View decompiled Move code
⚙️ **Disassembly** - Low-level bytecode assembly view
📋 **Copy/Download** - Export results easily

## Getting Test Files

### Option 1: Compile a Move module

```bash
# Create a simple Move module
mkdir test_module && cd test_module

cat > Move.toml << 'EOF'
[package]
name = "TestModule"
version = "0.1.0"

[addresses]
test_addr = "0x42"
EOF

mkdir sources
cat > sources/test.move << 'EOF'
module test_addr::MyModule {
    public fun hello(): u64 {
        42
    }

    struct MyStruct has drop {
        value: u64
    }
}
EOF

# Compile
aptos move compile

# The .mv files will be in build/TestModule/bytecode_modules/
# Copy them to test with the demo
```

### Option 2: Use existing Aptos framework modules

```bash
# Build Aptos framework
cd /opt/git/aptos-core/aptos-move/framework
cargo build -p aptos-cached-packages

# Find compiled modules in:
# aptos-move/framework/aptos-framework/build/AptosFramework/bytecode_modules/
```

### Option 3: Download from chain

```bash
# Download a package from mainnet
aptos move download \
  --account 0x1 \
  --package AptosFramework \
  --bytecode

# Bytecode will be in AptosFramework/bytecode_modules/*.mv
```

## Screenshot

The demo includes:
- 🎨 Beautiful gradient UI
- 🌙 Dark code editor theme
- 📱 Responsive design
- ⚡ Real-time analysis
- 🎯 Tab-based navigation

## Troubleshooting

### "Failed to load WASM module"

Make sure you built the WASM first:
```bash
wasm-pack build --target web --out-dir pkg
```

### "Cannot find pkg/move_decompiler_wasm.js"

The HTML expects the pkg folder to be at `../pkg/` relative to the demo folder.
Make sure you're serving from the `move-decompiler-wasm` directory, not `demo`.

### CORS errors

Use a proper web server (python http.server, etc) instead of opening file:// URLs directly.

### Module doesn't load

Check browser console (F12) for specific error messages.

## Browser Support

Requires modern browser with WebAssembly support:
- Chrome 91+
- Firefox 89+
- Safari 14+
- Edge 91+

## Tips

- Use small .mv files first (~1-10 KB) to test
- Larger modules (>100 KB) may take a few seconds to analyze
- Check browser DevTools console for detailed logs
- Try different Aptos framework modules to see various features

## What's Next?

This demo shows decompilation only. For full Move compilation in the browser, see:
- `../cli/WASM_EVALUATION_README.md` - Full feasibility study
- `../cli/PHASE5_FINAL_RECOMMENDATION.md` - Implementation roadmap

Enjoy exploring Move bytecode! 🚀
