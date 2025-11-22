# WASM Plugin System Roadmap

**Current Version:** v2.5
**Status:** Production-ready with documented limitations
**Last Updated:** 2025-01-18

---

## Current Capabilities (v2.5) ✅

### What Works Today

1. **Plugin Loading & Execution**
   - ✅ Load WASM modules (wasm32-wasip1)
   - ✅ Sandboxed execution with fuel metering
   - ✅ Memory isolation (128MB limit)
   - ✅ Async/await support
   - ✅ Timeout enforcement (5s default)

2. **Data Processing**
   - ✅ JSON serialization/deserialization
   - ✅ Static metadata
   - ✅ Action routing
   - ✅ TriggerContext handling

3. **WASI Preview1 Support**
   - ✅ Standard I/O (stdin, stdout, stderr)
   - ✅ Environment variables (read-only)
   - ✅ Clock/time access
   - ✅ Random number generation

4. **Developer Experience**
   - ✅ Scaffolding tool
   - ✅ Three templates (minimal, json, template)
   - ✅ Comprehensive documentation
   - ✅ Integration tests

---

## Current Limitations (v2.5) ⚠️

### WASI Preview1 Constraints

1. **Process Execution** ❌
   - **Status:** Not supported in WASI Preview1
   - **Impact:** Can't execute shell commands or AppleScript
   - **Workaround:** Use network-based APIs instead
   - **Timeline:** v2.7 with WASI Preview2

2. **Filesystem Access** ⚠️
   - **Status:** Partially implemented
   - **Current:** Can use stdin/stdout
   - **Missing:** Directory preopening (API changed in wasmtime v26)
   - **Timeline:** v2.7

3. **Network Sockets** ⚠️
   - **Status:** WASI Preview1 has limited socket support
   - **Current:** Basic TCP/UDP available
   - **Missing:** High-level HTTP client
   - **Timeline:** v2.6 (add HTTP wrapper)

4. **Resource Limiting** ⚠️
   - **Status:** Fuel metering works, memory limiting disabled
   - **Impact:** Relies on OS-level limits
   - **Timeline:** v2.7 (re-implement with v26 API)

---

## v2.6 Roadmap (Next Release)

### Goals

Make plugins more practical with network-based integrations and better examples.

### Features

#### 1. Network-Based Plugin Examples

**OBS Control Plugin** (High Priority)
- Use OBS WebSocket API instead of process execution
- Actions: scene switching, recording, streaming
- Demonstrates network capability
- Real-world practical use case

**Home Automation Plugin** (Medium Priority)
- Control Philips Hue lights
- Use HTTP REST API
- Demonstrates JSON + Network
- Popular use case

**Web Hooks Plugin** (Low Priority)
- Send HTTP POST requests
- Trigger IFTTT, Zapier, etc.
- Simple but powerful

#### 2. HTTP Client Wrapper

**Goal:** Make HTTP requests easier from WASM plugins

```rust
// Instead of raw WASI sockets:
let socket = TcpStream::connect("api.example.com:80")?;
socket.write_all(b"GET / HTTP/1.1\r\n...")?;

// Provide a simple wrapper:
let response = http_get("https://api.example.com/endpoint")?;
```

**Implementation:**
- Add `http` module to plugin runtime
- Wrap WASI socket calls
- Handle TLS via host
- Provide simple Request/Response API

#### 3. Plugin Marketplace UI (Initial)

**Features:**
- Browse available plugins
- View plugin details (description, capabilities, size)
- Install/uninstall plugins
- Enable/disable plugins
- Manage capabilities

**Tech Stack:**
- Tauri GUI integration
- Local plugin registry
- JSON metadata format

#### 4. Additional Documentation

- HTTP client usage guide
- Network plugin examples
- WebSocket integration patterns
- API design best practices

### Timeline

- **Month 1:** OBS plugin + HTTP wrapper
- **Month 2:** Marketplace UI design + implementation
- **Month 3:** Additional examples + documentation

---

## v2.7 Roadmap (Medium Term)

### Goals

Implement advanced WASI features and improve security/resource management.

### Features

#### 1. Directory Preopening

**Goal:** Enable safe filesystem access

```rust
// Runtime configuration:
let config = WasmConfig {
    preopened_dirs: vec![
        ("/config", "./plugins/my-plugin/config"),
        ("/data", "./plugins/my-plugin/data"),
    ],
    // ...
};

// In plugin:
let config_file = std::fs::read_to_string("/config/settings.toml")?;
```

**Implementation:**
- Update to wasmtime v26 directory API
- Support capability-based dir access
- Sandboxed path restrictions

#### 2. Resource Limiting

**Goal:** Re-implement memory and table limiting

```rust
impl ResourceLimiter for PluginResourceLimiter {
    fn memory_growing(&mut self, current: usize, desired: usize, maximum: Option<usize>)
        -> anyhow::Result<bool>
    {
        // Limit memory growth
    }

    fn table_growing(&mut self, current: usize, desired: usize, maximum: Option<usize>)
        -> anyhow::Result<bool>
    {
        // Limit table growth
    }
}
```

**Benefits:**
- Prevent memory exhaustion
- DoS protection
- Better resource accounting

#### 3. Plugin Signing & Verification

**Goal:** Ensure plugin authenticity and integrity

**Features:**
- Digital signatures (ed25519)
- SHA256 checksums
- Trust model (verified publishers)
- Signature verification UI

**Flow:**
```
1. Developer signs plugin with private key
2. Signature + public key included in plugin metadata
3. Runtime verifies signature before loading
4. User sees "Verified by Amiable" or "Unverified"
```

#### 4. WASI Preview2 Migration (Research)

**Goal:** Evaluate Preview2 benefits

**Preview2 Advantages:**
- Better process execution support
- Improved resource model
- Component model support
- Better networking APIs

**Challenges:**
- Ecosystem maturity
- Tooling support
- Migration complexity
- Breaking changes

**Decision:** Evaluate in Q2 2025, migrate if benefits are clear

### Timeline

- **Q1 2025:** Directory preopening + resource limiting
- **Q2 2025:** Plugin signing + Preview2 research
- **Q3 2025:** Preview2 migration (if approved)

---

## v3.0+ Roadmap (Long Term)

### Major Features

#### 1. Component Model Support

**Goal:** Use WASM Component Model for better composability

**Benefits:**
- Language-agnostic interfaces
- Better tooling
- Type-safe APIs
- Module composition

**Example:**
```wit
// Plugin interface in WIT (WebAssembly Interface Types)
interface midimon-plugin {
  record trigger-context {
    velocity: option<u8>,
    mode: option<u32>,
    timestamp: u64,
  }

  variant action-result {
    success,
    error(string),
  }

  execute: func(action: string, context: trigger-context) -> action-result
}
```

#### 2. Multi-Language Support

**Goal:** Support plugins in multiple languages

**Supported Languages:**
- ✅ Rust (current)
- 🔄 C/C++ (via WASI SDK)
- 🔄 AssemblyScript (TypeScript-like)
- 🔄 Go (via TinyGo)
- 🔄 Python (via PyO3/WASM)

**Example (AssemblyScript):**
```typescript
// plugin.ts
export function init(): u64 {
  const metadata = '{"name":"my-plugin",...}';
  return packPointer(metadata);
}

export function execute(ptr: u32, len: u32): i32 {
  const request = readString(ptr, len);
  const data = JSON.parse(request);

  // Handle action
  return 0; // Success
}
```

#### 3. Advanced Plugin Features

**Plugin-to-Plugin Communication**
- Plugins can call other plugins
- Shared state management
- Event bus for inter-plugin messaging

**Live Reload**
- Hot-reload plugins without restart
- Preserve state across reloads
- Development mode support

**Plugin Marketplace Backend**
- Central registry
- Automatic updates
- User ratings & reviews
- Download statistics
- Search & discovery

#### 4. Performance Optimizations

**AOT Compilation**
- Pre-compile WASM to native code
- Faster startup time
- Reduced runtime overhead

**Persistent Compilation Cache**
- Cache compiled modules
- Share cache across instances
- Faster subsequent loads

**Streaming Instantiation**
- Load large plugins progressively
- Reduce memory footprint
- Better startup experience

---

## Migration Guides

### v2.5 → v2.6

**Breaking Changes:** None
**New Features:** HTTP client wrapper, marketplace UI
**Migration:** No code changes required

### v2.6 → v2.7

**Breaking Changes:** Possible changes to capability API
**New Features:** Directory preopening, resource limiting
**Migration:**
1. Update capability requests if using filesystem
2. Test with new resource limits
3. Update to new directory API if using files

### v2.7 → v3.0

**Breaking Changes:** WASI Preview2 migration (major)
**New Features:** Component model, multi-language
**Migration:**
1. Re-compile plugins for Preview2
2. Update to Component Model interfaces
3. Test thoroughly with new runtime

---

## Alternative Approaches

### For Process Execution (Current Limitation)

Since WASI Preview1 doesn't support process execution well, here are alternatives:

#### Option 1: Host-Provided API (Recommended for v2.6)

**Approach:** Runtime provides process execution via custom host functions

```rust
// In runtime (host):
linker.func_wrap_async("env", "exec_command",
    |caller: Caller<'_, PluginHostState>, cmd_ptr: u32, cmd_len: u32| {
        Box::new(async move {
            // Execute command on host
            // Return result to plugin
        })
    }
)?;

// In plugin:
extern "C" {
    fn exec_command(cmd_ptr: *const u8, cmd_len: u32) -> i32;
}
```

**Pros:**
- Works with Preview1
- Full control over execution
- Can sandbox commands

**Cons:**
- Custom API (not standard WASI)
- Plugin depends on MIDIMon runtime

#### Option 2: Network-Based Services (Recommended for now)

**Approach:** Use HTTP/WebSocket APIs instead of local processes

**Examples:**
- **Spotify:** Use Spotify Web API instead of AppleScript
- **OBS:** Use OBS WebSocket instead of process execution
- **System utils:** Use web services or REST APIs

**Pros:**
- Cross-platform
- Secure
- Scalable
- Standard WASI

**Cons:**
- Requires network access
- May need API keys
- Latency

#### Option 3: Wait for WASI Preview2 (Future)

**Approach:** Use WASI Preview2 process execution when available

**Timeline:** Q3 2025 or later
**Status:** Preview2 is still evolving

---

## Decision Log

### Why WASI Preview1 for v2.5?

**Decision:** Use WASI Preview1 instead of Preview2
**Date:** 2025-01-17
**Reasoning:**
- Preview1 is stable and well-supported
- Preview2 is still evolving
- Wasmtime v26 has excellent Preview1 support
- Can migrate to Preview2 later without breaking core architecture

**Outcome:** Correct decision - system is stable and production-ready

### Why No Process Execution in v2.5?

**Decision:** Document process execution as v2.7+ feature
**Date:** 2025-01-18
**Reasoning:**
- Preview1 has poor process execution support
- Custom host functions would be non-standard
- Network-based alternatives exist for most use cases
- Preview2 will provide better process support

**Outcome:** Honest about limitations, clear migration path

### Why HTTP Wrapper in v2.6?

**Decision:** Add HTTP client wrapper as priority feature
**Date:** 2025-01-18
**Reasoning:**
- Most practical plugins need network access
- Raw WASI sockets are difficult to use
- HTTP is common protocol
- Enables many real-world use cases

**Outcome:** TBD (planned for v2.6)

---

## Community Input

We welcome feedback on this roadmap! Please share your thoughts:

- **GitHub Discussions:** Feature requests and ideas
- **GitHub Issues:** Bug reports and specific proposals
- **Discord:** Real-time discussion (coming soon)

### Most Wanted Features (Survey)

Help us prioritize by voting on what matters most:

1. ⬜ Process execution support
2. ⬜ Better filesystem access
3. ⬜ HTTP client wrapper
4. ⬜ Plugin marketplace
5. ⬜ Multi-language support
6. ⬜ Performance improvements
7. ⬜ Plugin signing/verification
8. ⬜ Component model
9. ⬜ Better documentation
10. ⬜ More examples

Vote by commenting on: [Issue #XXX]

---

## Conclusion

The WASM plugin system is production-ready today with clear limitations and a strong roadmap for future enhancements.

**What works now (v2.5):**
- Secure sandboxed execution
- JSON data processing
- Basic WASI functionality
- Excellent developer experience

**What's coming soon (v2.6):**
- Network-based plugins
- HTTP client wrapper
- Plugin marketplace
- More examples

**What's planned (v2.7+):**
- Full filesystem access
- Resource limiting
- Plugin signing
- WASI Preview2 (maybe)

We're committed to continuous improvement while maintaining stability and backward compatibility.

---

**Questions?** Open a discussion on GitHub!
**Want to contribute?** See `CONTRIBUTING.md`!
**Need help?** Check the documentation or ask in discussions!
