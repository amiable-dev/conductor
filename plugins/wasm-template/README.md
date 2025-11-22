# MIDIMon WASM Plugin Template

This template provides everything you need to build sandboxed WebAssembly plugins for MIDIMon.

## Features

✅ **Process Isolation**: Plugins run in separate memory space
✅ **Resource Limits**: Configurable memory and execution time limits
✅ **Capability System**: Fine-grained permissions (filesystem, network)
✅ **Platform Independent**: Same .wasm runs on macOS/Linux/Windows
✅ **Crash Isolation**: Plugin crashes don't affect MIDIMon daemon

## Quick Start

### 1. Install Dependencies

```bash
# Add WASM target to Rust toolchain
make install-deps

# Or manually:
rustup target add wasm32-wasip1
brew install binaryen  # For wasm-opt (optional)
```

### 2. Build Your Plugin

```bash
# Debug build
make build

# Release build (optimized)
make build-release

# Optimized with wasm-opt
make optimize
```

### 3. Test Your Plugin

```bash
# Run tests
make test

# Inspect WASM exports
make inspect
```

### 4. Install Plugin

```bash
# Copy to MIDIMon plugins directory
cp *.wasm ~/.config/midimon/plugins/
```

## Plugin Interface

Your plugin must export two functions:

### `init() -> u64`

Returns plugin metadata as JSON. The return value is packed as:
- High 32 bits: pointer to JSON string
- Low 32 bits: length of JSON string

```rust
#[no_mangle]
pub extern "C" fn init() -> u64 {
    let metadata = PluginMetadata {
        name: "My Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "Example plugin".to_string(),
        author: "Your Name".to_string(),
        actions: vec!["my_action".to_string()],
        capabilities: vec![],
    };

    let json = serde_json::to_string(&metadata).unwrap();
    let (ptr, len) = allocate_string(&json);
    ((ptr as u64) << 32) | (len as u64)
}
```

### `execute(ptr: u32, len: u32) -> i32`

Executes a plugin action. Arguments:
- `ptr`: Pointer to JSON request in WASM memory
- `len`: Length of JSON request

Returns:
- `0`: Success
- Non-zero: Error code

```rust
#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    let request_json = read_string(ptr as usize, len as usize)?;
    let request: ActionRequest = serde_json::from_str(&request_json)?;

    match request.action.as_str() {
        "my_action" => {
            println!("Executing my action!");
            0 // Success
        }
        _ => 3 // Unknown action
    }
}
```

## Request Format

The `execute()` function receives JSON requests:

```json
{
  "action": "my_action",
  "context": {
    "velocity": 100,
    "note": 60,
    "cc": null,
    "value": null,
    "timestamp": 1234567890,
    "metadata": {
      "custom_key": "custom_value"
    }
  }
}
```

## Metadata Format

The `init()` function returns JSON metadata:

```json
{
  "name": "My Plugin",
  "version": "1.0.0",
  "description": "Example WASM plugin",
  "author": "Your Name",
  "actions": ["action1", "action2"],
  "capabilities": ["filesystem", "network"]
}
```

## Available Capabilities

Request capabilities in your metadata:

- `filesystem`: Read/write to sandboxed directory (`~/.local/share/midimon/plugin-data/`)
- `network`: Make HTTP/HTTPS requests (implicit in WASI)

**Note**: Capabilities are granted at load time and enforced by WASI.

## Configuration Example

Use your plugin in `config.toml`:

```toml
[[modes.mappings]]
trigger = { Note = { number = 60, velocity = "Any" } }
action = { Plugin = {
    name = "my-plugin",
    action = "my_action",
    params = { "key" = "value" }
}}
```

## Memory Management

The template includes memory allocation helpers:

```rust
// Allocate memory (called by host)
#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

// Deallocate memory (called by host)
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size as usize);
    }
}
```

## Optimization Tips

### 1. Binary Size

```bash
# Use wee_alloc for smaller binary
cargo build --release --target wasm32-wasip1 --features wee_alloc_support

# Run wasm-opt for aggressive optimization
make optimize
```

### 2. Performance

- Avoid allocations in hot paths
- Use `#[inline]` for small functions
- Profile with `wasm-opt --metrics`

### 3. Debugging

```bash
# Build with debug info
cargo build --target wasm32-wasip1

# Inspect exports
wasm-objdump -x your-plugin.wasm

# Disassemble
wasm-objdump -d your-plugin.wasm
```

## Security Considerations

### Sandboxing

Your plugin runs in a WASM sandbox with:
- **Memory isolation**: Cannot access daemon memory
- **Instruction limits**: Max 100M instructions per call
- **Time limits**: Max 5 seconds execution (configurable)
- **Resource limits**: Max 128 MB memory (configurable)

### Capabilities

Only request capabilities you need:
- `filesystem`: Grants access to sandboxed directory only
- `network`: Allows outbound connections (no incoming)

**Never** try to:
- Access files outside sandboxed directory
- Execute shell commands (not possible in WASM)
- Bypass capability checks (enforced by WASI runtime)

## Testing

The template includes example tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let packed = init();
        let ptr = (packed >> 32) as u32;
        let len = (packed & 0xFFFFFFFF) as u32;

        let json = read_string(ptr as usize, len as usize).unwrap();
        let metadata: PluginMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(metadata.name, "My Plugin");
    }
}
```

Run tests:

```bash
cargo test
```

## Common Issues

### `cargo build` fails with "target not found"

```bash
rustup target add wasm32-wasip1
```

### Binary size is too large (>1MB)

```bash
# Use wee_alloc
cargo build --release --target wasm32-wasip1 --features wee_alloc_support

# Run wasm-opt
make optimize
```

### Plugin not loading

Check:
1. Exports are present: `make inspect`
2. File is in plugins directory: `~/.config/midimon/plugins/`
3. Metadata is valid JSON
4. Action names match configuration

### Runtime errors

Enable debug logging:

```bash
DEBUG=1 midimon
```

Check daemon logs:

```bash
tail -f ~/.local/share/midimon/daemon.log
```

## Examples

See the template code for examples of:
- ✅ Returning metadata from `init()`
- ✅ Parsing requests in `execute()`
- ✅ Memory allocation/deallocation
- ✅ Error handling
- ✅ Tests

## Resources

- [WASM by Example](https://wasmbyexample.dev/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [wasmtime Documentation](https://docs.wasmtime.dev/)
- [WASI Specification](https://github.com/WebAssembly/WASI)

## License

MIT License - see LICENSE file for details
