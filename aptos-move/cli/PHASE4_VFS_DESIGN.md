# Phase 4: Virtual Filesystem Design

**Date:** 2026-03-20
**Status:** Design Complete
**Dependencies:** Phase 3 Results

---

## Challenge Statement

Move package operations require filesystem access:
- Reading `Move.toml` manifests
- Loading `.move` source files from `sources/` directory
- Resolving dependencies
- Writing build artifacts to `build/` directory

**Problem:** WASM has no native filesystem. Web browsers and Node.js use different file access patterns.

**Goal:** Design abstraction layer that works in both native and WASM environments.

---

## Design Options Analysis

### Option A: Virtual Filesystem Trait ⭐ **RECOMMENDED**

**Concept:** Define trait abstracting filesystem operations, with native and WASM implementations.

#### Architecture

```rust
/// Abstraction over filesystem operations for Move packages
pub trait VirtualFilesystem: Send + Sync {
    /// Read entire file contents
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;

    /// Check if file exists
    fn exists(&self, path: &Path) -> bool;

    /// List files in directory
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;

    /// Read directory recursively with filter
    fn read_dir_recursive(&self, path: &Path, extension: &str) -> Result<Vec<(PathBuf, Vec<u8>)>>;

    /// Get file metadata (optional, for timestamp checks)
    fn metadata(&self, path: &Path) -> Result<FileMetadata>;
}

pub struct FileMetadata {
    pub size: u64,
    pub modified: Option<SystemTime>,
}

// Native implementation: thin wrapper over std::fs
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeFilesystem;

impl VirtualFilesystem for NativeFilesystem {
    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path).map_err(|e| anyhow!("Failed to read {}: {}", path.display(), e))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let entries = std::fs::read_dir(path)?;
        Ok(entries.filter_map(|e| e.ok().map(|e| e.path())).collect())
    }

    fn read_dir_recursive(&self, path: &Path, extension: &str) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let mut results = Vec::new();
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            if entry.path().extension() == Some(extension.as_ref()) {
                let contents = std::fs::read(entry.path())?;
                results.push((entry.path().to_owned(), contents));
            }
        }
        Ok(results)
    }

    fn metadata(&self, path: &Path) -> Result<FileMetadata> {
        let meta = std::fs::metadata(path)?;
        Ok(FileMetadata {
            size: meta.len(),
            modified: meta.modified().ok(),
        })
    }
}

// WASM implementation: in-memory storage
#[cfg(target_arch = "wasm32")]
pub struct WasmFilesystem {
    files: HashMap<PathBuf, Vec<u8>>,
}

impl WasmFilesystem {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add_file(&mut self, path: PathBuf, contents: Vec<u8>) {
        self.files.insert(path, contents);
    }

    pub fn from_js_map(files: js_sys::Map) -> Result<Self> {
        // Convert JavaScript Map to Rust HashMap
        let mut fs = Self::new();
        // Implementation: iterate JS Map, convert to PathBuf/Vec<u8>
        Ok(fs)
    }
}

impl VirtualFilesystem for WasmFilesystem {
    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        self.files.get(path)
            .cloned()
            .ok_or_else(|| anyhow!("File not found: {}", path.display()))
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let path_str = path.to_str().ok_or_else(|| anyhow!("Invalid path"))?;
        let results: Vec<PathBuf> = self.files.keys()
            .filter(|p| {
                p.parent() == Some(path) ||
                p.to_str().map(|s| s.starts_with(path_str)).unwrap_or(false)
            })
            .cloned()
            .collect();
        Ok(results)
    }

    fn read_dir_recursive(&self, path: &Path, extension: &str) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let path_str = path.to_str().ok_or_else(|| anyhow!("Invalid path"))?;
        let results: Vec<(PathBuf, Vec<u8>)> = self.files.iter()
            .filter(|(p, _)| {
                p.to_str().map(|s| s.starts_with(path_str) && s.ends_with(extension)).unwrap_or(false)
            })
            .map(|(p, contents)| (p.clone(), contents.clone()))
            .collect();
        Ok(results)
    }

    fn metadata(&self, path: &Path) -> Result<FileMetadata> {
        let contents = self.read_file(path)?;
        Ok(FileMetadata {
            size: contents.len() as u64,
            modified: None, // No timestamps in WASM VFS
        })
    }
}
```

#### Integration with move-package

**Challenge:** `move-package` crate uses `std::fs` directly throughout its codebase.

**Solutions:**

**1. Fork and Modify move-package (Invasive)**
- Fork `move-package` into `move-package-wasm`
- Replace all `std::fs` calls with VFS trait calls
- Maintain fork indefinitely

**2. Wrapper Layer (Recommended)**
- Keep `move-package` unchanged
- Create wrapper that:
  1. Accepts VFS as input
  2. Materializes files to temp directory (native) or in-memory structure (WASM)
  3. Calls `move-package` functions
  4. Cleans up temp files

```rust
pub struct PackageCompiler<FS: VirtualFilesystem> {
    fs: Arc<FS>,
}

impl<FS: VirtualFilesystem> PackageCompiler<FS> {
    pub fn compile(&self, package_root: &Path) -> Result<CompiledPackage> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: pass through to move-package directly
            let config = BuildConfig::default();
            config.compile_package(package_root, &mut std::io::stderr())
        }

        #[cfg(target_arch = "wasm32")]
        {
            // WASM: materialize to in-memory package structure
            let manifest = self.fs.read_file(&package_root.join("Move.toml"))?;
            let manifest: MovePackageManifest = toml::from_slice(&manifest)?;

            let sources = self.fs.read_dir_recursive(&package_root.join("sources"), "move")?;

            // Use move-compiler-v2 directly instead of move-package
            let mut compiler = MoveCompiler::new(sources, manifest.dependencies);
            compiler.compile()
        }
    }
}
```

**3. Compile-Only API (Simplest)**
- Bypass `move-package` entirely for WASM
- Use `move-compiler-v2` directly
- Manually handle dependency resolution

```rust
pub fn compile_move_sources(
    sources: Vec<(String, String)>,  // (filename, source_code)
    dependencies: Vec<CompiledPackage>,
) -> Result<CompiledPackage> {
    // Direct compilation without filesystem
    let compiler = MoveCompilerV2::new(sources);
    compiler.compile()
}
```

#### JavaScript Integration

```typescript
// JavaScript/TypeScript API
import init, { compile_package } from './aptos_move_wasm_cli.js';

await init();

// Provide files as Map
const files = new Map([
  ['Move.toml', `
[package]
name = "MyPackage"
version = "0.1.0"

[addresses]
my_addr = "0x42"
  `],
  ['sources/MyModule.move', `
module my_addr::MyModule {
    public fun hello(): u64 {
        42
    }
}
  `],
]);

const result = compile_package(files);
console.log(result); // { modules: [...], errors: [] }
```

#### Pros
- ✅ Clean abstraction
- ✅ Testable (mock filesystem)
- ✅ Flexible (works with any storage backend)
- ✅ Doesn't require forking Move crates

#### Cons
- ⚠️ Requires wrapper layer if using move-package
- ⚠️ More complex than direct compilation

#### Implementation Effort
- **VFS trait + native impl:** 1-2 days
- **WASM impl:** 2-3 days
- **Wrapper/integration:** 3-5 days
- **Testing:** 2-3 days
- **Total:** 2 weeks

---

### Option B: Package Pre-compilation

**Concept:** Compile packages natively, distribute compiled bytecode. WASM only handles already-compiled packages.

#### Workflow

```
Developer Machine (Native):
  Move Source → [Native Compiler] → Compiled Bytecode

Browser (WASM):
  Compiled Bytecode → [WASM CLI] → Disassemble/Decompile/Verify
```

#### Architecture

```rust
// WASM API only accepts pre-compiled bytecode
#[wasm_bindgen]
pub fn verify_compiled_package(bytecode_modules: Vec<Vec<u8>>) -> Result<(), JsValue> {
    // Verification logic only
}

#[wasm_bindgen]
pub fn disassemble_module(bytecode: Vec<u8>) -> Result<String, JsValue> {
    // Bytecode → Assembly
}
```

#### Pros
- ✅ Simple implementation
- ✅ No VFS needed
- ✅ Smaller WASM binary (no compiler)

#### Cons
- ❌ Can't compile in browser (major limitation)
- ❌ Requires native toolchain for compilation
- ❌ Poor DX for web-based IDEs

#### Use Cases
- ✅ Bytecode analysis tools
- ✅ Verification-only applications
- ❌ Web-based IDEs (NO)

#### Implementation Effort
- **Bytecode API:** 3-5 days
- **Testing:** 2 days
- **Total:** 1 week

**Verdict:** ⚠️ Only suitable if compilation is not needed in WASM.

---

### Option C: JavaScript-Provided Files

**Concept:** JavaScript provides files via `wasm-bindgen`, WASM processes them in-memory.

#### Architecture

```rust
#[wasm_bindgen]
pub struct MovePackage {
    manifest: String,
    sources: Vec<MoveSourceFile>,
}

#[wasm_bindgen]
pub struct MoveSourceFile {
    path: String,
    content: String,
}

#[wasm_bindgen]
impl MovePackage {
    #[wasm_bindgen(constructor)]
    pub fn new(manifest: String) -> Self {
        Self {
            manifest,
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, path: String, content: String) {
        self.sources.push(MoveSourceFile { path, content });
    }

    pub fn compile(&self) -> Result<CompilationResult, JsValue> {
        // Parse manifest
        let manifest: MovePackageManifest = toml::from_str(&self.manifest)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Compile sources
        let mut compiler = MoveCompilerV2::new();
        for source in &self.sources {
            compiler.add_source(&source.path, &source.content);
        }

        compiler.compile()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
```

#### JavaScript Usage

```typescript
const pkg = new MovePackage(`
[package]
name = "Test"
version = "0.1.0"
`);

pkg.add_source("sources/test.move", `
module 0x1::Test {
    public fun main() {}
}
`);

const result = pkg.compile();
```

#### Pros
- ✅ No filesystem abstraction needed
- ✅ Direct JavaScript integration
- ✅ Simple API

#### Cons
- ⚠️ Tight coupling with JavaScript
- ⚠️ Manual file management in JS
- ⚠️ No dependency resolution (user must provide all files)

#### Implementation Effort
- **API design:** 2-3 days
- **Compilation integration:** 3-5 days
- **Testing:** 2-3 days
- **Total:** 1.5 weeks

**Verdict:** ✅ Good for MVP, can be enhanced with VFS later.

---

## Recommendation: Hybrid Approach ⭐

**Phase 1 MVP:** Option C (JavaScript-Provided Files)
- Fast to implement
- Sufficient for single-file compilation
- Good developer experience for simple cases

**Phase 2 Enhancement:** Option A (VFS Trait)
- Add VFS for multi-file packages
- Support dependency resolution
- Enable advanced features

### Staged Implementation

#### Stage 1: Single-File Compilation (Week 1)
```rust
#[wasm_bindgen]
pub fn compile_move_module(
    source: String,
    address: String,
) -> Result<Vec<u8>, JsValue> {
    // Simplest API: one module, no dependencies
}
```

#### Stage 2: Multi-File via JavaScript (Week 2)
```rust
#[wasm_bindgen]
pub struct MovePackage { /* as shown in Option C */ }
```

#### Stage 3: VFS Abstraction (Week 3-4)
```rust
pub trait VirtualFilesystem { /* as shown in Option A */ }

#[wasm_bindgen]
pub fn compile_package_from_vfs(
    files: js_sys::Map  // JavaScript Map → VFS
) -> Result<CompilationResult, JsValue>
```

---

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_native() {
        let fs = NativeFilesystem;
        let content = fs.read_file(Path::new("test.txt")).unwrap();
        assert!(!content.is_empty());
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_vfs_wasm() {
        let mut fs = WasmFilesystem::new();
        fs.add_file(PathBuf::from("test.move"), b"module test {}".to_vec());
        assert!(fs.exists(Path::new("test.move")));
    }
}
```

### Integration Tests
```rust
#[test]
fn test_compile_simple_package() {
    let pkg = MovePackage::new(r#"
        [package]
        name = "Test"
        version = "0.1.0"
    "#.to_string());

    pkg.add_source("sources/test.move".to_string(), r#"
        module 0x1::Test {
            public fun answer(): u64 { 42 }
        }
    "#.to_string());

    let result = pkg.compile();
    assert!(result.is_ok());
}
```

### JavaScript Integration Tests
```javascript
import { describe, it, expect } from 'vitest';
import init, { MovePackage } from './aptos_move_wasm_cli.js';

describe('WASM Move Compiler', () => {
  beforeAll(async () => {
    await init();
  });

  it('compiles simple module', () => {
    const pkg = new MovePackage('[package]\nname = "Test"\n');
    pkg.add_source('sources/test.move', 'module 0x1::Test { }');

    const result = pkg.compile();
    expect(result.modules.length).toBeGreaterThan(0);
  });
});
```

---

## Performance Considerations

### Memory Usage
- **VFS storage:** In-memory HashMap scales with file count/size
- **Mitigation:** Lazy loading, streaming for large files
- **Target:** <100MB memory for typical package

### Compilation Speed
- **WASM overhead:** ~2-5x slower than native (acceptable)
- **Optimization:** Use release build with wasm-opt
- **Target:** <5s for small package, <30s for large package

### Binary Size
- **VFS overhead:** Minimal (~5-10KB)
- **Compiler overhead:** Major (10-30MB for move-compiler-v2)
- **Target:** <20MB after wasm-opt optimization

---

## Migration Path

### For Existing move-package Users

**Before (Native):**
```rust
let config = BuildConfig::default();
let pkg = config.compile_package(path, &mut stderr())?;
```

**After (WASM-compatible):**
```rust
#[cfg(not(target_arch = "wasm32"))]
let pkg = {
    let config = BuildConfig::default();
    config.compile_package(path, &mut stderr())?
};

#[cfg(target_arch = "wasm32")]
let pkg = {
    let compiler = PackageCompiler::new(WasmFilesystem::new());
    compiler.compile(path)?
};
```

### For New Code

```rust
// Write once, works everywhere
let fs: Box<dyn VirtualFilesystem> = create_fs();
let compiler = PackageCompiler::new(fs);
let pkg = compiler.compile(path)?;
```

---

## Phase 4 Conclusion

**Recommended Approach:** Hybrid staged implementation
1. **MVP:** JavaScript-provided files (Option C) - 1.5 weeks
2. **Enhanced:** VFS abstraction (Option A) - 2 weeks
3. **Total:** 3.5 weeks for full VFS support

**Key Decision:** Start with Option C for fast MVP, add Option A for production quality.

**Next Phase:** Phase 5 - Final Risk Assessment and GO/NO-GO recommendation.

---

*Phase 4 completed: 2026-03-20*
*Recommendation: Proceed to Phase 5*
