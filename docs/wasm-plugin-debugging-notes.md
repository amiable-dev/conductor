# WASM Plugin Debugging Notes

## Issue: WASM Plugin init() Crash

**Date:** 2025-01-18
**Status:** Investigating

### Problem

The WASM template plugin compiles successfully but crashes when calling `init()`:

```
Failed to initialize plugin: PluginLoadFailed("init() failed: error while executing at wasm backtrace:\n    0: 0x139b6 - <unknown>!<wasm function 278>")
```

### Root Cause Analysis

The crash happens inside the WAS module's `init()` function, likely during one of these operations:

1. **String allocation** in `alloc()` function
2. **JSON serialization** with `serde_json::to_string()`
3. **Memory copying** in `allocate_string()`

### WASM Allocator Issues

WASM plugins have special requirements for memory allocation:

1. **No Default Allocator**: Standard Rust allocator doesn't work in WASM without special configuration
2. **wee_alloc**: Optional small allocator, but adds complexity
3. **Custom Allocator**: Need to implement memory management manually

### Potential Solutions

#### Option 1: Use Static JSON String

Instead of dynamic allocation, return a pre-computed static string:

```rust
const METADATA_JSON: &str = r#"{"name":"example_wasm_plugin","version":"0.1.0","description":"Example WASM plugin for MIDIMon","author":"Amiable","license":"MIT","type":"action","capabilities":[]}"#;

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA_JSON.as_ptr() as usize;
    let len = METADATA_JSON.len();
    ((ptr as u64) << 32) | (len as u64)
}
```

**Pros:**
- No allocation needed
- Simple and fast
- No crash risk

**Cons:**
- Static metadata (can't be dynamic)
- String must outlive the call

#### Option 2: Enable wee_alloc Properly

Add proper wee_alloc configuration:

```toml
[dependencies]
wee_alloc = "0.4.5"

[features]
default = ["wee_alloc_support"]
wee_alloc_support = []
```

```rust
#[cfg(feature = "wee_alloc_support")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

#### Option 3: Use WASI libc Allocator

Ensure WASI is properly configured for memory allocation:

```toml
[dependencies]
libc = "0.2"
```

### Current Status

- ✅ WASM module loads successfully
- ✅ init() function found and callable
- ❌ init() crashes during execution
- ⏳ Investigating allocator setup

### Next Steps

1. Try static JSON string approach (quickest fix)
2. Add debug logging to WASM module
3. Test with simpler metadata (fewer fields)
4. Consider wasm-bindgen for better tooling

### Workaround for Testing

For now, we can test the WASM runtime infrastructure without relying on the template plugin working correctly. The key achievement is that:

1. ✅ WASM runtime compiles
2. ✅ WASM module loads
3. ✅ Functions can be called (even if they crash)
4. ✅ wasmtime v26 API is working

The crash is in the plugin code, not the runtime infrastructure.
