# Move Compiler Deployment Plans

## Plan A: Cloudflare Pages Functions (Immediate - 2 hours)

**Goal**: Deploy a serverless Move compiler with near-zero cost

### Cost Analysis
- **Free tier**: 100,000 requests/day (~3M/month)
- **Paid tier**: $5/month for 10M requests
- **Example**: 10M compilations/month = $5/month (0.0005¢ per compilation)

### Architecture
```
Browser → Cloudflare Pages Function → Node.js + WASM Compiler → Bytecode
```

### Steps

#### 1. Prepare WASM Package (15 mins)
```bash
cd aptos-move/move-compiler-wasm
wasm-pack build --target nodejs --release
```

**Deliverable**: `pkg/` directory with Node.js-compatible WASM

#### 2. Create Cloudflare Pages Project (10 mins)

**File**: `functions/api/compile.js`
```javascript
import init, { compile_module, compile_script } from '../../pkg/move_compiler_wasm.js';

let initialized = false;

export async function onRequestPost(context) {
  // Initialize WASM (only once)
  if (!initialized) {
    await init();
    initialized = true;
  }

  try {
    const { action, source, address, module_name } = await context.request.json();

    let result;
    if (action === 'compile_module') {
      result = compile_module(source, address, module_name);
    } else if (action === 'compile_script') {
      result = compile_script(source, address);
    } else {
      return new Response(JSON.stringify({ error: 'Invalid action' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    return new Response(JSON.stringify({
      success: result.success,
      bytecode: result.success ? Array.from(result.bytecode) : [],
      errors: result.success ? [] : JSON.parse(result.errors),
    }), {
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      }
    });

  } catch (error) {
    return new Response(JSON.stringify({
      success: false,
      errors: [error.message],
    }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' }
    });
  }
}

// Handle CORS preflight
export async function onRequestOptions() {
  return new Response(null, {
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type',
    }
  });
}
```

**File**: `package.json`
```json
{
  "name": "move-compiler-serverless",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {}
}
```

**File**: `wrangler.toml`
```toml
name = "move-compiler"
compatibility_date = "2024-01-01"
pages_build_output_dir = "public"

[build]
command = ""

[[rules]]
type = "CompiledWasm"
globs = ["**/*.wasm"]
fallthrough = true
```

#### 3. Deploy to Cloudflare (5 mins)

```bash
# Install Wrangler CLI
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Deploy
wrangler pages deploy public --project-name move-compiler
```

**Alternative**: Use Cloudflare Dashboard
1. Go to pages.cloudflare.com
2. Connect GitHub repo
3. Auto-deploy on push

#### 4. Update Browser Client (15 mins)

**File**: `demo/index-serverless.html`
```javascript
async function compileCode() {
  const source = document.getElementById('sourceCode').value;
  const address = document.getElementById('address').value;
  const moduleName = document.getElementById('moduleName').value;

  const response = await fetch('https://move-compiler.your-site.pages.dev/api/compile', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      action: 'compile_module',
      source,
      address,
      module_name: moduleName,
    })
  });

  const result = await response.json();
  // Display result...
}
```

#### 5. Test & Monitor (15 mins)

**Test requests**:
```bash
curl -X POST https://move-compiler.your-site.pages.dev/api/compile \
  -H "Content-Type: application/json" \
  -d '{
    "action": "compile_module",
    "source": "module 0x1::Test { public fun hello(): u64 { 42 } }",
    "address": "0x1",
    "module_name": "Test"
  }'
```

**Monitor**: Cloudflare Dashboard → Analytics
- Request count
- Error rate
- Latency

### Limitations
- ✅ Works: Desktop browsers, mobile browsers, anywhere with internet
- ❌ Doesn't work: Offline use
- ⚠️ Cold start: First request ~500ms, subsequent ~50ms

### Optimization Options
- **Reduce cold starts**: Keep-alive ping every 5 mins
- **Cache**: Cache compiled stdlib modules
- **Rate limiting**: Prevent abuse (built into Cloudflare)

---

## Plan B: Filesystem-Free Compiler (Long-term - 2-3 weeks)

**Goal**: Modify Move compiler to accept source strings directly, no filesystem

### Why This Matters
- ✅ True browser-native compilation
- ✅ Offline support
- ✅ Zero server costs forever
- ✅ Lower latency (no network round-trip)

### Architecture Changes

```
Current:  Source String → Write to File → Compiler reads file → Bytecode
New:      Source String → Compiler (in-memory) → Bytecode
```

### Implementation Plan

#### Phase 1: Analysis (3 days)

**1.1 Map Filesystem Dependencies**
```bash
cd third_party/move/move-compiler-v2
rg "std::fs::" --type rust
rg "Path::new|PathBuf|read_to_string" --type rust
```

**Deliverable**: Document listing all filesystem touchpoints
- Where files are read
- Where paths are constructed
- Where file metadata is checked

**1.2 Identify API Boundaries**
Find the earliest point where we can inject in-memory sources.

Key files to examine:
- `move-compiler-v2/src/lib.rs` - Main entry point
- `legacy-move-compiler/src/command_line/compiler.rs` - Compiler setup
- `legacy-move-compiler/src/parser/syntax.rs` - Parsing (already has `parse_file_string!`)

**Finding**: The parser already supports string input via `parse_file_string()`, but the higher-level APIs wrap this with filesystem operations.

#### Phase 2: Design New API (2 days)

**2.1 Define In-Memory Source Type**
```rust
// New file: move-compiler-v2/src/sources.rs

use move_symbol_pool::Symbol;
use std::collections::HashMap;

/// In-memory source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: Symbol,        // Virtual path for error messages
    pub content: String,     // Source code
}

/// Collection of in-memory source files
#[derive(Clone, Debug)]
pub struct SourceMap {
    files: HashMap<Symbol, String>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add_file(&mut self, path: impl Into<Symbol>, content: String) {
        self.files.insert(path.into(), content);
    }

    pub fn get_file(&self, path: &Symbol) -> Option<&str> {
        self.files.get(path).map(|s| s.as_str())
    }
}
```

**2.2 Design New Compiler API**
```rust
// Add to move-compiler-v2/src/lib.rs

/// Compile from in-memory sources (no filesystem access)
pub fn compile_from_sources(
    sources: SourceMap,
    dependencies: Vec<SourceMap>,
    named_addresses: BTreeMap<String, AccountAddress>,
    options: CompilerOptions,
) -> Result<Vec<CompiledUnit>, Diagnostics> {
    // Implementation in Phase 3
}
```

**2.3 Backward Compatibility**
Keep existing file-based APIs, add new source-based APIs alongside:
```rust
// Old API (keep for backward compatibility)
pub fn compile_from_files(paths: Vec<String>, ...) -> Result<...>

// New API (add for WASM/browser use)
pub fn compile_from_sources(sources: SourceMap, ...) -> Result<...>
```

#### Phase 3: Implementation (1.5 weeks)

**3.1 Modify Parser Integration (3 days)**

File: `legacy-move-compiler/src/command_line/compiler.rs`

**Current code** (lines 203-204):
```rust
let (source_text, pprog_and_comments_res) =
    parse_program(&mut compilation_env, maps, targets, deps)?;
```

**New approach**:
```rust
// Add new function
fn parse_program_from_sources(
    env: &mut CompilationEnv,
    maps: NamedAddressMaps,
    sources: &SourceMap,
    deps_sources: Vec<&SourceMap>,
) -> anyhow::Result<(FilesSourceText, Result<...>)> {
    let mut files_source_text = HashMap::new();
    let mut parsed_defs = vec![];

    for (path, content) in sources.files.iter() {
        let file_hash = FileHash::new(content);
        files_source_text.insert(file_hash, (*path, content.clone()));

        // Use existing parse_file_string
        let (defs, comments) = parser::syntax::parse_file_string(
            env,
            file_hash,
            content,
        )?;

        parsed_defs.push((path, defs, comments));
    }

    // Continue with expansion, typing, etc.
    // ... rest of compilation pipeline
}
```

**3.2 Add Source-Based Entry Points (2 days)**

File: `move-compiler-v2/src/lib.rs`

```rust
/// Compile from in-memory sources
pub fn compile_from_sources(
    sources: SourceMap,
    dependencies: Vec<SourceMap>,
    named_addresses: BTreeMap<String, AccountAddress>,
    flags: Flags,
) -> Result<Vec<AnnotatedCompiledUnit>, Diagnostics> {

    let mut env = CompilationEnv::new(flags, BTreeSet::new());

    // Convert SourceMap to internal representation
    let (source_text, pprog_res) = parse_program_from_sources(
        &mut env,
        /* ... */
    )?;

    // Continue with existing compilation pipeline
    pprog_res.and_then(|(pprog, comments)| {
        // expansion
        // typing
        // bytecode generation
        Ok(compiled_units)
    })
}
```

**3.3 Update WASM Bindings (1 day)**

File: `aptos-move/move-compiler-wasm/src/compiler.rs`

**Replace**:
```rust
// OLD: Write temp file
std::fs::write(&temp_path, &source)?;
let mut options = Options::default();
options.sources = vec![temp_path.clone()];
```

**With**:
```rust
// NEW: Use in-memory sources
use move_compiler_v2::sources::SourceMap;

let mut sources = SourceMap::new();
sources.add_file(
    format!("{}.move", module_name),
    source,
);

let result = move_compiler_v2::compile_from_sources(
    sources,
    vec![], // No dependencies
    named_addresses,
    Flags::empty(),
)?;
```

**3.4 Handle Dependencies (2 days)**

For stdlib and other dependencies, we need to:

1. **Option A: Bundle pre-compiled stdlib**
```rust
// Include compiled stdlib bytecode
const STDLIB_BYTECODE: &[u8] = include_bytes!("../stdlib.compiled");
```

2. **Option B: Bundle stdlib sources**
```rust
// Include stdlib sources at compile time
fn load_stdlib_sources() -> SourceMap {
    let mut map = SourceMap::new();
    map.add_file("vector.move", include_str!("../move-stdlib/vector.move"));
    map.add_file("signer.move", include_str!("../move-stdlib/signer.move"));
    // ... all stdlib files
    map
}
```

**Recommendation**: Option B for flexibility, cache compiled output.

#### Phase 4: Testing (3 days)

**4.1 Unit Tests**
```rust
#[test]
fn test_compile_from_sources() {
    let mut sources = SourceMap::new();
    sources.add_file("test.move", r#"
        module 0x1::Test {
            public fun hello(): u64 { 42 }
        }
    "#);

    let result = compile_from_sources(
        sources,
        vec![],
        [("std".to_string(), AccountAddress::ONE)].into(),
        Flags::empty(),
    );

    assert!(result.is_ok());
    let units = result.unwrap();
    assert_eq!(units.len(), 1);
}
```

**4.2 Integration Tests**
- Compile single module ✓
- Compile with dependencies ✓
- Compile with stdlib ✓
- Error handling (syntax errors, type errors) ✓
- Multi-file packages ✓

**4.3 WASM Tests**
```bash
cd aptos-move/move-compiler-wasm
wasm-pack test --node
wasm-pack test --firefox --headless
wasm-pack test --chrome --headless
```

**4.4 Regression Tests**
Ensure existing file-based API still works:
```bash
cd third_party/move
cargo test --all
```

#### Phase 5: Documentation & PR (2 days)

**5.1 Update Documentation**

File: `move-compiler-v2/README.md`
```markdown
## Usage

### From Files (traditional)
```rust
let options = Options {
    sources: vec!["MyModule.move".to_string()],
    ...
};
run_move_compiler(options)?;
```

### From Strings (new - WASM compatible)
```rust
use move_compiler_v2::sources::SourceMap;

let mut sources = SourceMap::new();
sources.add_file("MyModule.move", source_code);
compile_from_sources(sources, ...)?;
```
```

**5.2 Create RFC for Move Community**

File: `move-compiler-v2/rfcs/in-memory-compilation.md`
```markdown
# RFC: In-Memory Source Compilation

## Summary
Add APIs to compile Move code from in-memory strings instead of requiring
filesystem access. Enables WASM/browser deployment and testing.

## Motivation
- WASM environments don't have filesystem access
- Testing frameworks want to compile code without touching disk
- IDE integrations want to compile unsaved buffers

## Design
[Detailed design from Phase 2]

## Backward Compatibility
All existing APIs remain unchanged. New APIs are additive only.

## Alternatives Considered
1. Virtual filesystem shim - rejected (too invasive)
2. WASI with polyfill - rejected (complex, large overhead)
```

**5.3 Submit Pull Request**

To: https://github.com/move-language/move

PR Title: `[move-compiler-v2] Add in-memory source compilation APIs`

PR Description:
```markdown
## Overview
Adds `compile_from_sources()` API that accepts source code as strings
instead of file paths, enabling:
- ✅ WASM/browser deployment
- ✅ Testing without filesystem
- ✅ IDE language server integration

## Changes
- New `SourceMap` type for in-memory sources
- New `compile_from_sources()` entry point
- Backward compatible - all existing APIs unchanged
- Comprehensive tests included

## Example
```rust
let mut sources = SourceMap::new();
sources.add_file("test.move", "module 0x1::Test { ... }");
let bytecode = compile_from_sources(sources, ...)?;
```

## Testing
- Unit tests: ✓
- Integration tests: ✓
- WASM tests: ✓
- Regression tests: ✓
```

#### Phase 6: WASM Integration (1 day)

Once PR is merged, update our WASM package:

**6.1 Update Dependencies**
```toml
[dependencies]
move-compiler-v2 = { git = "https://github.com/move-language/move", branch = "main" }
```

**6.2 Simplify Compiler**

File: `aptos-move/move-compiler-wasm/src/compiler.rs`
```rust
// DELETE all filesystem code
// DELETE memfs module
// DELETE temp file handling

pub fn compile_module(source: String, address: String, module_name: String)
    -> CompilationResult
{
    let mut sources = SourceMap::new();
    sources.add_file(format!("{}.move", module_name), source);

    match compile_from_sources(sources, stdlib_sources(), named_addresses, Flags::empty()) {
        Ok(units) => {
            let bytecode = serialize_units(units)?;
            CompilationResult::new_success(bytecode, vec![])
        }
        Err(diagnostics) => {
            CompilationResult::new_failure(format_diagnostics(diagnostics))
        }
    }
}
```

**6.3 Rebuild & Test**
```bash
wasm-pack build --target web --release
# Should now be truly browser-native!
```

**6.4 Update Demo**
```html
<!-- No more serverless function needed! -->
<script type="module">
  import init, { compile_module } from './pkg/move_compiler_wasm.js';
  await init();

  // Works directly in browser, no server needed!
  const result = compile_module(source, "0x1", "Test");
</script>
```

---

## Timeline Comparison

| Approach | Time | Cost (monthly) | Offline Support |
|----------|------|----------------|-----------------|
| **Plan A: Serverless** | 2 hours | $0-5 | ❌ No |
| **Plan B: Filesystem-Free** | 2-3 weeks | $0 | ✅ Yes |
| **Both (Recommended)** | 2 hours + 3 weeks | $0 | ✅ Yes |

## Recommendation

**Start with Plan A immediately** (2 hours):
- Get working solution today
- Minimal cost (~$5/month even with high usage)
- Validate user demand

**Then pursue Plan B** (3 weeks):
- Proper long-term solution
- Zero ongoing costs
- Better user experience (offline, faster)
- Contributes to Move ecosystem

This way you're not blocked waiting for upstream changes, and you end up with the best of both worlds.

---

## Resources Needed

### Plan A (Serverless)
- Cloudflare account (free)
- Domain name (optional, Cloudflare provides `*.pages.dev`)
- ~15 mins setup time

### Plan B (Filesystem-Free)
- Rust development environment
- Access to Move repository (fork + PR)
- Testing across platforms (Node.js, browsers)
- Move team review/approval time (1-2 weeks)

---

## Risk Assessment

### Plan A Risks
- 🟡 **Vendor lock-in**: Tied to Cloudflare
  - *Mitigation*: Easy to port to other serverless platforms
- 🟡 **Network dependency**: Requires internet
  - *Mitigation*: Add offline fallback or Plan B
- 🟢 **Cost overrun**: Unlikely with free tier
  - *Mitigation*: Set billing alerts

### Plan B Risks
- 🟡 **Upstream rejection**: PR might not be accepted
  - *Mitigation*: Maintain fork, keep pushing for acceptance
- 🟡 **Breaking changes**: Move compiler updates might break integration
  - *Mitigation*: Pin to specific commit, update when stable
- 🟢 **Technical feasibility**: Already have most pieces
  - *Mitigation*: Parser supports strings, just need plumbing

---

## Next Steps

**Immediate** (Choose one):
1. "Set up Plan A" → I'll create the Cloudflare deployment in next 30 mins
2. "Start Plan B" → I'll begin filesystem dependency analysis
3. "Do both" → Plan A now, Plan B in parallel

Which would you like me to proceed with?
