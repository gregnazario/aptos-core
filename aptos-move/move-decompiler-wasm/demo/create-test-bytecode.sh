#!/bin/bash
# Create a test Move module and compile it to bytecode

set -e

echo "🔨 Creating test Move module..."

# Create temporary directory
TEST_DIR="./test-module"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR/sources"

# Create Move.toml
cat > "$TEST_DIR/Move.toml" << 'EOF'
[package]
name = "TestModule"
version = "0.1.0"

[addresses]
test_addr = "0x42"

[dependencies]
MoveStdlib = { git = "https://github.com/aptos-labs/aptos-core.git", subdir = "aptos-move/framework/move-stdlib", rev = "main" }
EOF

# Create a simple Move module
cat > "$TEST_DIR/sources/hello.move" << 'EOF'
module test_addr::Hello {
    use std::vector;

    /// A simple struct to demonstrate bytecode features
    struct Message has drop {
        content: vector<u8>,
        timestamp: u64,
    }

    /// Returns the answer to life, universe, and everything
    public fun answer(): u64 {
        42
    }

    /// Creates a greeting message
    public fun create_message(content: vector<u8>): Message {
        Message {
            content,
            timestamp: 1234567890,
        }
    }

    /// Gets the message content
    public fun get_content(msg: &Message): vector<u8> {
        *&msg.content
    }

    /// Complex function with control flow
    public fun is_even(n: u64): bool {
        if (n % 2 == 0) {
            true
        } else {
            false
        }
    }

    /// Function with loop
    public fun sum_to_n(n: u64): u64 {
        let sum = 0;
        let i = 0;
        while (i <= n) {
            sum = sum + i;
            i = i + 1;
        };
        sum
    }
}
EOF

echo "📦 Compiling Move module..."

# Change to test directory and compile
cd "$TEST_DIR"

# Try to compile with aptos CLI
if command -v aptos &> /dev/null; then
    aptos move compile

    echo ""
    echo "✅ Compilation successful!"
    echo ""
    echo "📁 Compiled bytecode location:"
    BYTECODE_DIR="build/TestModule/bytecode_modules"
    ls -lh "$BYTECODE_DIR"

    echo ""
    echo "🎯 To test in the web app:"
    echo "   1. Start the web server (from move-decompiler-wasm directory):"
    echo "      python3 -m http.server 8000"
    echo ""
    echo "   2. Open http://localhost:8000/demo/"
    echo ""
    echo "   3. Drag and drop this file into the browser:"
    echo "      $(pwd)/$BYTECODE_DIR/Hello.mv"

    # Copy bytecode to demo directory for easy access
    mkdir -p "../test-files"
    cp "$BYTECODE_DIR"/*.mv "../test-files/"

    echo ""
    echo "📋 Bytecode also copied to: ../test-files/"

else
    echo "❌ Error: 'aptos' CLI not found"
    echo ""
    echo "Please install the Aptos CLI first:"
    echo "  cargo install --git https://github.com/aptos-labs/aptos-core.git aptos"
    echo ""
    echo "Or compile manually from aptos-core:"
    echo "  cd /opt/git/aptos-core/aptos-move/cli"
    echo "  cargo build --release -p aptos-move-cli"
    exit 1
fi
