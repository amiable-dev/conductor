# Plugin Security

**Since:** v2.7
**Status:** Production-ready

Conductor provides enterprise-grade security for WASM plugins through cryptographic signatures, resource limiting, and filesystem sandboxing.

## Security Architecture

```
┌─────────────────────────────────────────────────┐
│  Security Layers                                │
│                                                 │
│  ┌───────────────────────────────────────────┐ │
│  │  Layer 1: Cryptographic Verification     │ │
│  │  - Ed25519 digital signatures             │ │
│  │  - SHA-256 integrity checking             │ │
│  │  - Trust management                       │ │
│  └───────────────────────────────────────────┘ │
│                                                 │
│  ┌───────────────────────────────────────────┐ │
│  │  Layer 2: Resource Limiting               │ │
│  │  - CPU fuel metering (100M instructions)  │ │
│  │  - Memory limits (128 MB)                 │ │
│  │  - Table growth limits                    │ │
│  └───────────────────────────────────────────┘ │
│                                                 │
│  ┌───────────────────────────────────────────┐ │
│  │  Layer 3: Filesystem Sandboxing           │ │
│  │  - Directory preopening (WASI)            │ │
│  │  - Path validation                        │ │
│  │  - No escape from sandbox                 │ │
│  └───────────────────────────────────────────┘ │
│                                                 │
│  ┌───────────────────────────────────────────┐ │
│  │  Layer 4: Capability System               │ │
│  │  - Explicit permission model              │ │
│  │  - Risk-based approval                    │ │
│  │  - Revocable grants                       │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

## Plugin Signing

### Overview

Plugin signing uses **Ed25519 digital signatures** to ensure:
- **Authenticity**: Plugin comes from claimed developer
- **Integrity**: Plugin hasn't been tampered with
- **Non-repudiation**: Developer cannot deny signing

### CLI Tool: conductor-sign

#### Installation

```bash
# Build with signing support
cargo build --package conductor-daemon \
  --bin conductor-sign \
  --features plugin-signing \
  --release
```

Binary location: `target/release/conductor-sign`

### Signing Workflow

#### 1. Generate Keypair

```bash
conductor-sign generate-key ~/.conductor/my-plugin-key
```

**Output:**
- `~/.conductor/my-plugin-key.private` (32 bytes, keep secure!)
- `~/.conductor/my-plugin-key.public` (hex-encoded)
- Public key displayed in terminal

**Example output:**
```
Generating Ed25519 keypair...
✓ Keypair generated successfully!

Private key: /Users/you/.conductor/my-plugin-key.private
Public key:  /Users/you/.conductor/my-plugin-key.public

Public key (hex): 3dab8dbfaeb804085e879791d395d6afabe268535a2bc98ea70afa1edd291cca

⚠️  Keep your private key secure and never share it!
```

#### 2. Sign Plugin

```bash
conductor-sign sign my_plugin.wasm ~/.conductor/my-plugin-key \
  --name "Your Name" \
  --email "you@example.com"
```

**Creates:** `my_plugin.wasm.sig` (JSON signature metadata)

**Example signature file:**
```json
{
  "version": 1,
  "algorithm": "Ed25519",
  "plugin_hash": "a1b2c3d4...",
  "plugin_size": 123456,
  "public_key": "3dab8dbf...",
  "signature": "9f8e7d6c...",
  "signed_at": "2025-11-19T08:59:12Z",
  "developer": {
    "name": "Your Name",
    "email": "you@example.com"
  }
}
```

#### 3. Verify Signature

```bash
conductor-sign verify my_plugin.wasm
```

**Example output:**
```
Verifying plugin: my_plugin.wasm
Signature file: "my_plugin.wasm.sig"
Trusted keys: 1

Signature Details:
  Version:     1
  Algorithm:   Ed25519
  Signed at:   2025-11-19T08:59:12Z
  Developer:   Your Name <you@example.com>
  Public key:  3dab8dbf...

✓ Signature verified successfully!
✓ Plugin signed by trusted key
```

#### 4. Manage Trusted Keys

**Add trusted key:**
```bash
conductor-sign trust add 3dab8dbfaeb804085e879791d395d6afabe268535a2bc98ea70afa1edd291cca "Official Plugin"
```

**List trusted keys:**
```bash
conductor-sign trust list
```

**Remove trusted key:**
```bash
conductor-sign trust remove 3dab8dbf...
```

**Trusted keys file:** `~/.config/conductor/trusted_keys.toml`

## Trust Models

Conductor supports three trust levels:

### Level 1: Unsigned (Development)

**Use case:** Development and testing

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.require_signature = false;  // Default
```

**Behavior:**
- No signature required
- Plugin loads without verification
- Suitable for development only

### Level 2: Self-Signed

**Use case:** Personal plugins, one-off scripts

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.require_signature = true;
config.allow_self_signed = true;
```

**Behavior:**
- Signature must be valid (cryptographic check)
- Any key accepted (no trust check)
- Ensures binary integrity
- Good for personal use

### Level 3: Trusted Keys (Production)

**Use case:** Production deployments, marketplace plugins

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.require_signature = true;
config.allow_self_signed = false;  // Default
```

**Behavior:**
- Signature must be valid
- Public key must be in trusted list
- Full security model
- Recommended for production

## Security Best Practices

### For Plugin Developers

1. **Protect Private Keys**
   ```bash
   # Set secure permissions
   chmod 600 ~/.conductor/my-plugin-key.private

   # Never commit to version control
   echo "*.private" >> .gitignore
   ```

2. **Sign Every Release**
   ```bash
   # Include signing in your release script
   cargo build --target wasm32-wasip1 --release
   conductor-sign sign \
     target/wasm32-wasip1/release/my_plugin.wasm \
     ~/.conductor/my-key \
     --name "Your Name" --email "you@example.com"
   ```

3. **Publish Public Key**
   - Include in README
   - Post on official website
   - Add to plugin registry

4. **Use Separate Keys per Project**
   ```bash
   # One key per plugin project
   conductor-sign generate-key ~/.conductor/plugin-a-key
   conductor-sign generate-key ~/.conductor/plugin-b-key
   ```

### For End Users

1. **Verify Before Trust**
   ```bash
   # Always verify signature first
   conductor-sign verify downloaded_plugin.wasm

   # Check developer information
   # Only trust if matches expected developer
   ```

2. **Add Keys Carefully**
   ```bash
   # Only add keys from verified sources
   # Check plugin author's website for official key
   conductor-sign trust add <key> "Official Project Name"
   ```

3. **Audit Trusted Keys**
   ```bash
   # Regularly review trusted keys
   conductor-sign trust list

   # Remove unused keys
   conductor-sign trust remove <key>
   ```

4. **Use Strict Mode in Production**
   ```toml
   # ~/.config/conductor/config.toml
   [wasm]
   require_signature = true
   allow_self_signed = false  # Strict mode
   ```

## Resource Limiting

### CPU Limits (Fuel Metering)

**Default:** 100,000,000 instructions (~100ms)

**What it prevents:**
- Infinite loops
- Excessive CPU usage
- Denial of service

**How it works:**
```
Every WASM instruction consumes 1 unit of fuel
Plugin execution stops when fuel exhausted
```

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.max_fuel = 200_000_000;  // 200M instructions
```

**Example:**
```rust
// This would exceed fuel limit:
#[no_mangle]
pub extern "C" fn execute(...) -> i32 {
    loop {
        // Infinite loop - blocked by fuel limit
        std::thread::sleep(Duration::from_millis(1));
    }
}
// Conductor automatically terminates after ~100ms
```

### Memory Limits

**Default:** 128 MB per plugin

**What it prevents:**
- Memory exhaustion
- Out-of-memory crashes
- Resource hogging

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.max_memory_bytes = 256 * 1024 * 1024;  // 256 MB
```

**Example:**
```rust
#[no_mangle]
pub extern "C" fn execute(...) -> i32 {
    // OK - 10 MB allocation
    let buffer = vec![0u8; 10 * 1024 * 1024];

    // BLOCKED - 200 MB exceeds limit
    // let huge = vec![0u8; 200 * 1024 * 1024];
    // ^ This allocation would fail

    0
}
```

### Table Growth Limits

**Default:** 10,000 elements

**What it prevents:**
- Unbounded table allocation
- Memory exhaustion via tables
- DoS attacks

**Configuration:**
```rust
let mut config = WasmConfig::default();
config.max_table_elements = 20_000;
```

## Filesystem Sandboxing

### Directory Preopening

Plugins with `Filesystem` capability can only access a specific directory:

**macOS:**
```
~/Library/Application Support/conductor/plugin-data/
```

**Linux:**
```
~/.local/share/conductor/plugin-data/
```

**Windows:**
```
%APPDATA%\conductor\plugin-data\
```

### What's Blocked

```rust
// ✅ ALLOWED - within sandbox
std::fs::write("/my-data.json", data)?;
std::fs::write("/subdir/file.txt", data)?;

// ❌ BLOCKED - path traversal
std::fs::write("/../../../etc/passwd", data)?;

// ❌ BLOCKED - absolute path outside sandbox
std::fs::write("/etc/passwd", data)?;

// ❌ BLOCKED - home directory escape
std::fs::write("~/other-file.txt", data)?;
```

### How It Works

1. **WASI Preopening**: Conductor pre-opens the plugin data directory
2. **Path Mapping**: All plugin paths mapped to sandbox root
3. **Validation**: WASI runtime blocks access outside preopened directories
4. **No Escape**: Path traversal attempts automatically blocked

## Capability System

### Permission Model

Plugins must explicitly request capabilities:

```rust
// In plugin code
#[no_mangle]
pub extern "C" fn capabilities() -> *const u8 {
    let caps = vec!["Network", "Filesystem"];
    let json = serde_json::to_string(&caps).unwrap();
    // ... return JSON
}
```

### Risk Levels

| Capability | Risk | Auto-Grant | Requires Approval |
|-----------|------|------------|-------------------|
| Network | 🟢 Low | Yes | No |
| Audio | 🟢 Low | Yes | No |
| Midi | 🟢 Low | Yes | No |
| Filesystem | 🟡 Medium | No | Yes |
| Subprocess | 🔴 High | No | Yes + Warning |
| SystemControl | 🔴 High | No | Yes + Warning |

### Granting Capabilities

**Via GUI:** Plugin Manager → Select Plugin → Grant Capability

**Via Config:**
```toml
[plugins.my_plugin]
granted_capabilities = ["Network", "Filesystem"]
```

## Threat Model

### Threats Mitigated

✅ **Binary Tampering**
- **Mitigation:** SHA-256 hash verification
- **Detection:** Signature validation fails if binary modified

✅ **Malicious Plugin Injection**
- **Mitigation:** Ed25519 signature verification
- **Detection:** Invalid signature rejected

✅ **Man-in-the-Middle**
- **Mitigation:** Cryptographic signatures
- **Detection:** Signature doesn't match binary

✅ **Supply Chain Attacks**
- **Mitigation:** Trusted key model
- **Detection:** Untrusted keys rejected

✅ **Resource Exhaustion (DoS)**
- **Mitigation:** Fuel metering, memory limits
- **Detection:** Automatic termination

✅ **Filesystem Access**
- **Mitigation:** Directory sandboxing
- **Detection:** WASI blocks access

✅ **Privilege Escalation**
- **Mitigation:** Capability system
- **Detection:** Capability checks

### Threats NOT Mitigated

⚠️ **Side-Channel Attacks**
- Timing attacks possible
- Mitigation: Careful crypto implementation

⚠️ **Social Engineering**
- User could trust malicious key
- Mitigation: User education

⚠️ **Key Compromise**
- Stolen private key can sign malicious plugins
- Mitigation: Hardware security keys, key rotation

## Security Checklist

### For Plugin Developers

- [ ] Generate unique keypair for project
- [ ] Protect private key (chmod 600, never commit)
- [ ] Sign every release
- [ ] Publish public key on official channels
- [ ] Request minimum necessary capabilities
- [ ] Include security disclosure policy
- [ ] Test with strict mode enabled
- [ ] Document all security considerations

### For End Users

- [ ] Only download plugins from trusted sources
- [ ] Always verify signatures before use
- [ ] Check developer information matches expected
- [ ] Only add keys from verified sources
- [ ] Enable strict mode in production
- [ ] Regularly audit trusted keys
- [ ] Review capability requests
- [ ] Keep Conductor updated

## Troubleshooting

### Signature Verification Failed

**Check signature exists:**
```bash
ls -la my_plugin.wasm.sig
```

**Verify signature manually:**
```bash
conductor-sign verify my_plugin.wasm
```

**Common causes:**
- Binary modified after signing
- Signature file missing
- Wrong public key used
- Corrupted signature file

### Key Not Trusted

**List trusted keys:**
```bash
conductor-sign trust list
```

**Add key:**
```bash
conductor-sign trust add <public-key> "Plugin Name"
```

**Verify key matches:**
```bash
# Compare key in signature vs. expected
cat my_plugin.wasm.sig | jq '.public_key'
```

### Out of Fuel

**Symptoms:**
- Plugin terminates mid-execution
- "fuel exhausted" error

**Solutions:**
```rust
// Increase fuel limit
let mut config = WasmConfig::default();
config.max_fuel = 200_000_000;

// Or optimize plugin code
// - Reduce loop iterations
// - Move heavy work to init()
// - Use lazy initialization
```

## Advanced Topics

### Hardware Security Keys

For maximum security, use hardware keys (YubiKey, etc.):

```bash
# Generate key on hardware device
# (Implementation depends on HSM/hardware)

# Sign using hardware key
conductor-sign sign my_plugin.wasm \
  --hardware-key /dev/yubikey \
  --name "Your Name" --email "you@example.com"
```

### Key Rotation

```bash
# Generate new key
conductor-sign generate-key ~/.conductor/my-plugin-key-v2

# Sign with new key
conductor-sign sign my_plugin.wasm ~/.conductor/my-plugin-key-v2 \
  --name "Your Name" --email "you@example.com"

# Announce rotation to users
# Include both old and new public keys in transition period
```

### Multi-Signature

For critical plugins, require multiple signatures:

```bash
# Sign with first key
conductor-sign sign my_plugin.wasm ~/.conductor/key1 \
  --name "Developer 1" --email "dev1@example.com"

# Co-sign with second key
conductor-sign cosign my_plugin.wasm ~/.conductor/key2 \
  --name "Developer 2" --email "dev2@example.com"

# Verify requires both signatures
```

## Further Reading

- [Ed25519 Specification (RFC 8032)](https://tools.ietf.org/html/rfc8032)
- [WASI Security Model](https://github.com/WebAssembly/WASI/blob/main/docs/WASI-security-model.md)
- [WebAssembly Security](https://webassembly.org/docs/security/)
- [Cryptographic Signing Best Practices](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
