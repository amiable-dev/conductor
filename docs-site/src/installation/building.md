# Building Conductor from Source

## Overview

This guide covers building Conductor from source code on any supported platform. Whether you're developing new features, customizing behavior, or just want to run the latest code, this guide walks through the complete build process.

## Prerequisites

### Rust Toolchain

Conductor requires Rust 1.70.0 or later.

#### Installing Rust

The recommended way to install Rust is via **rustup**, the official Rust toolchain installer:

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell)
# Download and run: https://win.rustup.rs/
```

Follow the installation prompts. The installer will:
1. Install `rustc` (Rust compiler)
2. Install `cargo` (Rust package manager and build tool)
3. Configure your shell environment

**After installation**, reload your shell:

```bash
# macOS/Linux
source $HOME/.cargo/env

# Windows: Close and reopen terminal
```

**Verify installation**:

```bash
rustc --version  # Should show: rustc 1.75.0 (or later)
cargo --version  # Should show: cargo 1.75.0 (or later)
```

#### Updating Rust

Keep your toolchain up to date:

```bash
rustup update
```

## Workspace Architecture (v0.2.0+)

Conductor uses a **Cargo workspace** with three packages since v0.2.0:

### Package Structure

1. **conductor-core** - Pure Rust engine library
   - Zero UI dependencies (no colored output, pure logic)
   - Public API for embedding in other applications
   - 30+ public types exported
   - Event processing, mapping engine, action execution

2. **conductor-daemon** - CLI daemon + diagnostic tools
   - Main `conductor` binary (daemon service)
   - `conductorctl` - CLI control tool (v1.0.0+)
   - 6 diagnostic binaries:
     - `midi_diagnostic` - MIDI event viewer
     - `led_diagnostic` - LED testing tool
     - `led_tester` - Interactive LED control
     - `pad_mapper` - Note number mapper
     - `test_midi` - Port connectivity test
     - `midi_simulator` - MIDI event simulator (testing)

3. **conductor** (root) - Backward compatibility layer
   - Re-exports conductor-core types
   - For existing v0.1.0 tests only
   - New code should use conductor-core directly

### When to Use Each Package

- **Use conductor-core**: Embed Conductor in your application
- **Use conductor-daemon**: Run as standalone CLI/daemon
- **Use conductor (root)**: Only for backward compatibility

### Public API Example

```rust
use conductor_core::{Config, MappingEngine, EventProcessor, ActionExecutor};

let config = Config::load("config.toml")?;
let mut engine = MappingEngine::new();
// Process events, execute actions...
```

### Platform-Specific Dependencies

#### macOS

**Required**:
- **Xcode Command Line Tools**: Provides compilers and linkers
  ```bash
  xcode-select --install
  ```

**Optional (for Maschine Mikro MK3)**:
- **Native Instruments Drivers**: Required for HID/LED support
  - Download from [Native Instruments](https://www.native-instruments.com/en/support/downloads/)
  - Install via Native Access

**Verify**:
```bash
# Check compiler
clang --version

# Check USB device (if connected)
system_profiler SPUSBDataType | grep -i mikro
```

#### Linux (Ubuntu/Debian)

**Required**:
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libusb-1.0-0-dev
```

**For Fedora/RHEL**:
```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    alsa-lib-devel \
    systemd-devel \
    libusbx-devel
```

**For Arch Linux**:
```bash
sudo pacman -S base-devel alsa-lib systemd-libs libusb
```

**udev rules** (for HID access without sudo):

Create `/etc/udev/rules.d/50-conductor.rules`:
```bash
sudo tee /etc/udev/rules.d/50-conductor.rules << 'EOF'
# Native Instruments Maschine Mikro MK3
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", MODE="0666", GROUP="plugdev"

# Generic MIDI devices
SUBSYSTEM=="usb", ATTRS{bInterfaceClass}=="01", ATTRS{bInterfaceSubClass}=="03", MODE="0666", GROUP="plugdev"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

Add your user to the `plugdev` group:
```bash
sudo usermod -a -G plugdev $USER
# Log out and back in for changes to take effect
```

#### Windows

**Required**:
- **Microsoft C++ Build Tools**: For compiling native dependencies
  - Download: [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
  - Install "Desktop development with C++" workload

  OR

  - Full Visual Studio (Community edition is free)

**Optional**:
- **Native Instruments Drivers**: For Maschine Mikro MK3 support
  - Download from [Native Instruments](https://www.native-instruments.com/en/support/downloads/)
  - Install via Native Access

**Verify**:
```powershell
# Check MSVC compiler
where cl

# Check Rust
rustc --version
cargo --version
```

## Cloning the Repository

### Via Git

```bash
# HTTPS (recommended for most users)
git clone https://github.com/yourusername/conductor.git
cd conductor

# SSH (if you have SSH keys configured)
git clone git@github.com:yourusername/conductor.git
cd conductor
```

### Verify Repository Contents

```bash
ls -la

# Expected files/directories:
# - Cargo.toml     (Rust project manifest)
# - Cargo.lock     (Dependency lock file)
# - src/           (Source code)
# - config.toml    (Example configuration)
# - README.md      (Project readme)
# - docs/          (Documentation)
```

## Build Commands

### Workspace Builds (v0.2.0+)

Build the entire workspace (all 3 packages in parallel):

```bash
# Development build
cargo build --workspace

# Release build (optimized)
cargo build --release --workspace
```

**Build times** (workspace):
- Clean build: ~12s (was 15-20s in v0.1.0)
- Incremental: <2s
- Parallel compilation across all packages

### Package-Specific Builds

Build individual packages for faster iteration:

```bash
# Core engine only
cargo build --package conductor-core
cargo build -p conductor-core  # Short form

# Daemon + tools
cargo build -p conductor-daemon

# Compatibility layer
cargo build -p conductor

# Specific binary
cargo build --release --bin conductor
cargo build --release --bin conductorctl
cargo build --release --bin midi_diagnostic
```

### Running Binaries

```bash
# Main daemon
cargo run --release --bin conductor 2

# Daemon control
cargo run --release --bin conductorctl status

# Diagnostic tool
cargo run --release --bin midi_diagnostic 2
```

### Debug Build

Fastest compilation, includes debug symbols, no optimization:

```bash
cargo build
```

**Output**: `target/debug/conductor` (~10-20MB binary)

**When to use**:
- Development and testing
- Debugging with `lldb`/`gdb`
- Frequent recompilation

**Performance**: ~20-30% slower than release builds

### Release Build

Optimized compilation, stripped debug symbols, smaller binary:

```bash
cargo build --release
```

**Output**: `target/release/conductor` (~3-5MB binary)

**When to use**:
- Production use
- Performance-critical applications
- Distribution

**Build time**: 12s clean, <2s incremental (workspace)

**Performance**: Full optimization, <1ms MIDI latency

### Clean Build

Remove all build artifacts and start fresh:

```bash
# Clean build artifacts
cargo clean

# Then rebuild
cargo build --release
```

**When to use**:
- After updating dependencies
- Troubleshooting build issues
- Freeing disk space (build artifacts can be 1-2GB)

### Check Only (No Binary)

Verify code compiles without producing a binary:

```bash
cargo check
```

**Fastest** way to verify code correctness during development. Use this for quick iteration.

### Run Directly

Build and run in one command:

```bash
# Debug mode
cargo run

# Release mode (recommended)
cargo run --release

# With arguments
cargo run --release -- 2 --led reactive
```

Note the `--` separator between cargo arguments and program arguments.

## Build Optimization

### Release Profile Configuration

Conductor's `Cargo.toml` includes optimized release settings:

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-Time Optimization (smaller binary, longer build)
codegen-units = 1    # Single codegen unit (better optimization)
strip = true         # Strip debug symbols (smaller binary)
panic = 'abort'      # Abort on panic (smaller binary, no unwinding)
```

**Results**:
- Binary size: ~3-5MB (vs ~15-20MB without optimization)
- Startup time: <100ms
- MIDI latency: <1ms
- Memory usage: 5-10MB

### Custom Optimization Levels

Override optimization for specific dependencies:

```toml
# In Cargo.toml
[profile.dev.package.midir]
opt-level = 3  # Optimize MIDI library even in debug builds
```

### Faster Debug Builds

Trade optimization for compile speed during development:

```toml
# In Cargo.toml
[profile.dev]
opt-level = 1        # Basic optimization
debug = true         # Keep debug symbols
incremental = true   # Enable incremental compilation
```

### Parallel Compilation

Cargo uses all CPU cores by default. To limit:

```bash
# Use 4 cores
cargo build -j 4

# Use 1 core (debugging build issues)
cargo build -j 1
```

## Cross-Compilation

### macOS: Universal Binary (Intel + Apple Silicon)

Build a single binary that runs on both architectures:

```bash
# Install targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for both
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/conductor \
    target/aarch64-apple-darwin/release/conductor \
    -output target/release/conductor-universal

# Verify
file target/release/conductor-universal
# Output: Mach-O universal binary with 2 architectures: [x86_64:Mach-O 64-bit executable x86_64] [arm64]
```

### Linux: Cross-Compile for Different Architectures

```bash
# Install target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compiler (Ubuntu/Debian)
sudo apt install gcc-aarch64-linux-gnu

# Build
cargo build --release --target aarch64-unknown-linux-gnu
```

### Windows: Cross-Compile from Linux

Using the `cross` tool:

```bash
# Install cross
cargo install cross

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

## Dependency Management

### Updating Dependencies

```bash
# Update all dependencies to latest compatible versions
cargo update

# Check for outdated dependencies
cargo outdated

# Update specific dependency
cargo update -p midir
```

### Dependency Audit

Check for security vulnerabilities:

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit
```

### Vendoring Dependencies (Offline Builds)

For building without internet access:

```bash
# Download all dependencies
cargo vendor

# Configure to use vendored deps
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

# Now cargo build works offline
cargo build --release
```

## Binary Size Optimization

### Size Comparison

```bash
# Default release build
cargo build --release
ls -lh target/release/conductor
# ~3-5MB

# Further optimization
cargo build --release --features "optimize-size"
ls -lh target/release/conductor
# ~2-3MB
```

### Extreme Size Reduction

Add to `Cargo.toml`:

```toml
[profile.release-min]
inherits = "release"
opt-level = "z"      # Optimize for size
lto = true
codegen-units = 1
strip = true
panic = 'abort'
```

Build with:
```bash
cargo build --profile release-min
```

Result: ~1-2MB binary (may be slightly slower)

### Analyze Binary Size

```bash
# Install cargo-bloat
cargo install cargo-bloat

# Analyze
cargo bloat --release --crates
cargo bloat --release -n 20  # Show top 20 functions by size
```

## Development Builds

### Watch Mode (Auto-Rebuild on Changes)

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild on save
cargo watch -x check
cargo watch -x 'run -- 2'
cargo watch -x 'test'
```

### Build with Specific Features

```bash
# Build with all features
cargo build --all-features

# Build with specific feature
cargo build --features "midi-learn"

# Build without default features
cargo build --no-default-features
```

### Build Examples and Binaries

Conductor includes diagnostic tools:

```bash
# Build all binaries
cargo build --release --bins

# Build specific binary
cargo build --release --bin midi_diagnostic

# List available binaries
ls target/release/midi_*
# midi_diagnostic
# led_diagnostic
# led_tester
# pad_mapper
# test_midi
```

## Testing Builds

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_event_processor

# Run tests with release optimizations
cargo test --release
```

### Run Benchmarks

```bash
# Install cargo-criterion
cargo install cargo-criterion

# Run benchmarks
cargo bench
```

### Integration Tests

```bash
# Run only integration tests
cargo test --test '*'

# Run with real MIDI device (requires hardware)
cargo test --test integration_midi -- --ignored
```

## Troubleshooting Build Issues

### Common Errors

#### Error: "linker 'cc' not found"

**Cause**: Missing C compiler

**Solution**:
- **macOS**: `xcode-select --install`
- **Linux**: `sudo apt install build-essential`
- **Windows**: Install Visual Studio Build Tools

---

#### Error: "could not compile 'hidapi'"

**Cause**: Missing USB/HID development libraries

**Solution**:
- **macOS**: Install Xcode Command Line Tools
- **Linux**: `sudo apt install libudev-dev libusb-1.0-0-dev`
- **Windows**: Install Windows SDK

---

#### Error: "failed to fetch dependencies"

**Cause**: Network issues or outdated index

**Solution**:
```bash
# Update cargo index
cargo update

# Or manually remove and re-fetch
rm -rf ~/.cargo/registry/index
cargo build
```

---

#### Error: "out of disk space"

**Cause**: Build artifacts consuming too much space

**Solution**:
```bash
# Clean all build artifacts
cargo clean

# Clean entire cargo cache (frees 1-5GB)
rm -rf ~/.cargo/registry/cache
rm -rf ~/.cargo/registry/src
```

---

### Incremental Build Issues

If incremental builds produce errors:

```bash
# Disable incremental compilation
CARGO_INCREMENTAL=0 cargo build --release

# Or add to ~/.cargo/config.toml:
[build]
incremental = false
```

### Dependency Conflicts

If cargo reports conflicting dependencies:

```bash
# Show dependency tree
cargo tree

# Show why a dependency is included
cargo tree -i <dependency-name>

# Update all dependencies
cargo update
```

## Build Performance Tips

### Parallel Compilation

Use all CPU cores:
```bash
# Set in ~/.cargo/config.toml
[build]
jobs = 8  # Or number of cores
```

### Use Faster Linker

#### macOS

```bash
# Install zld (faster linker)
brew install michaeleisel/zld/zld

# Configure in .cargo/config.toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/bin/zld"]
```

#### Linux

```bash
# Install mold (very fast linker)
sudo apt install mold  # Ubuntu 22.04+
# or download from: https://github.com/rui314/mold

# Configure in .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**Result**: 2-3x faster linking (especially for incremental builds)

### Use sccache (Shared Compilation Cache)

```bash
# Install sccache
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache

# Check stats
sccache --show-stats
```

Speeds up rebuilds across multiple projects.

## Distribution

### Creating Release Artifacts

```bash
# Build optimized binary
cargo build --release

# Copy to distribution directory
mkdir -p dist
cp target/release/conductor dist/
cp config.toml dist/config.example.toml
cp README.md dist/

# Create tarball
cd dist
tar czf conductor-v0.1.0-macos-aarch64.tar.gz *
```

### Code Signing (macOS)

For distribution outside the App Store:

```bash
# Sign the binary
codesign --force --deep --sign "Developer ID Application: Your Name" \
    target/release/conductor

# Verify signature
codesign -dv --verbose=4 target/release/conductor

# Notarize (requires paid Apple Developer account)
xcrun notarytool submit conductor.zip \
    --keychain-profile "AC_PASSWORD" \
    --wait
```

## Next Steps

After building successfully:

1. **Run the binary**: `./target/release/conductor`
2. **Configure mappings**: Edit `config.toml`
3. **Read documentation**:
   - [macOS Installation](macos.md)
   - [First Mapping](../getting-started/first-mapping.md)
   - [Configuration Overview](../configuration/overview.md)

## See Also

- [Cargo Book](https://doc.rust-lang.org/cargo/) - Complete cargo documentation
- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html) - All supported targets
- [CLI Commands](../reference/cli-commands.md) - Running the binary with options

---

**Last Updated**: November 11, 2025
**Rust Version**: 1.70.0+ required, 1.75.0+ recommended
