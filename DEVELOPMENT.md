# Development Guide

This guide provides comprehensive instructions for setting up a development environment and contributing to MIDIMon.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/amiable-dev/midimon.git
cd midimon

# Run automated setup
./scripts/dev-setup.sh

# Build and run
cargo build --release
cargo run --release
```

## Prerequisites

### Required

- **Rust**: 1.75.0 or later (install via [rustup](https://rustup.rs/))
- **Git**: For version control
- **MIDI Device**: For testing (or use virtual MIDI ports)

### Platform-Specific

**macOS**:
- Xcode Command Line Tools: `xcode-select --install`
- Input Monitoring permission for HID access

**Linux** (future):
- ALSA development libraries: `sudo apt-get install libasound2-dev`
- udev rules for device access

**Windows** (future):
- Visual Studio Build Tools

## Repository Setup

### 1. Fork and Clone

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/midimon.git
cd midimon

# Add upstream remote
git remote add upstream https://github.com/amiable-dev/midimon.git
```

### 2. Install Rust Toolchain

```bash
# Install rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required components
rustup component add rustfmt clippy

# Verify installation
cargo --version
rustc --version
```

### 3. Run Setup Script

```bash
# Automated development environment setup
chmod +x scripts/dev-setup.sh
./scripts/dev-setup.sh
```

This script will:
- Verify Rust installation
- Install cargo extensions
- Set up git hooks
- Check system dependencies

## Building and Running

### Debug Build

```bash
# Fast compilation, includes debug symbols
cargo build

# Run with debug build
cargo run
```

### Release Build

```bash
# Optimized build (3-5MB binary)
cargo build --release

# Run with release build (recommended for testing)
cargo run --release

# Connect to MIDI port 2
cargo run --release 2
```

### With Options

```bash
# Enable debug logging
DEBUG=1 cargo run --release 2

# With LED scheme
cargo run --release 2 --led reactive

# With device profile
cargo run --release 2 --profile mikro.ncmm3
```

## Testing

### Run All Tests

```bash
# Run full test suite
cargo test --all

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific module
cargo test mappings::
```

### Code Quality Checks

```bash
# Run clippy (linter)
cargo clippy -- -D warnings

# Run formatter check
cargo fmt --check

# Apply formatting
cargo fmt

# Run all quality checks
./scripts/test.sh
```

## Diagnostic Tools

```bash
# MIDI event visualizer
cargo run --bin midi_diagnostic 2

# LED testing
cargo run --bin led_diagnostic
cargo run --bin led_tester

# Pad mapping utility
cargo run --bin pad_mapper
```

## Project Structure

```
midimon/
├── src/
│   ├── main.rs              # Entry point, MIDI connection, event loop
│   ├── config.rs            # Configuration structures (TOML parsing)
│   ├── event_processor.rs   # MidiEvent → ProcessedEvent (timing detection)
│   ├── mappings.rs          # ProcessedEvent → Action (mapping engine)
│   ├── actions.rs           # Action execution (keyboard, shell, etc.)
│   ├── feedback.rs          # LED feedback trait and factory
│   ├── mikro_leds.rs        # HID-based RGB LED control
│   ├── midi_feedback.rs     # MIDI-based LED control fallback
│   └── device_profile.rs    # NI Controller Editor profile parser
├── docs/                    # Additional documentation
├── scripts/                 # Build and development scripts
├── .github/                 # CI/CD workflows and templates
└── config.toml              # Example configuration
```

### Key Modules

#### Event Processing Pipeline

1. **main.rs**: MIDI input → `MidiEvent`
2. **event_processor.rs**: `MidiEvent` → `ProcessedEvent` (detect velocity, timing, chords)
3. **mappings.rs**: `ProcessedEvent` → `Action` (match against config)
4. **actions.rs**: `Action` → execution (simulate input, run commands)

#### Adding New Features

**New Trigger Type**:
1. Add variant to `Trigger` enum in `config.rs`
2. Add variant to `ProcessedEvent` enum in `event_processor.rs`
3. Add detection logic in `EventProcessor::process()`
4. Add matching case in `MappingEngine::trigger_matches_processed()`

**New Action Type**:
1. Add variant to `ActionConfig` enum in `config.rs`
2. Add variant to `Action` enum in `actions.rs`
3. Add execution logic in `ActionExecutor::execute()`
4. Update `compile_action()` in `mappings.rs`

**New LED Scheme**:
1. Add variant to `LightingScheme` enum in `feedback.rs`
2. Implement in `MikroMK3LEDs::run_scheme()` (mikro_leds.rs)
3. Add fallback in `MidiFeedback::run_scheme()` (midi_feedback.rs)

## Code Style

### Formatting

- Use `rustfmt` (automatically applied by pre-commit hook)
- 4-space indentation
- 100-character line limit

### Linting

- All `clippy` warnings must be resolved
- Run `cargo clippy -- -D warnings` before committing

### Documentation

```rust
/// Brief description of function
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Example
///
/// ```
/// let result = function(arg);
/// ```
pub fn function(param: Type) -> Result<ReturnType, Error> {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for operations that can fail
- Provide context with error messages
- Avoid `unwrap()` in production code (use `expect()` with clear message)

## Debugging

### Enable Debug Logging

```bash
# Set DEBUG environment variable
DEBUG=1 cargo run --release 2
```

### VS Code Debugging

Use the provided launch configurations in `.vscode/launch.json`:

- **Debug Main**: Run main application with debugger
- **Debug Diagnostic**: Run MIDI diagnostic tool
- **Run Tests**: Debug specific tests

### Common Issues

**MIDI device not found**:
```bash
# Check USB connection
system_profiler SPUSBDataType | grep -i mikro

# List available ports
cargo run --release
```

**Permission denied (HID)**:
- Grant Input Monitoring permission on macOS
- System Settings → Privacy & Security → Input Monitoring

**Build errors**:
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

## Git Workflow

### Branch Naming

- Feature: `feature/short-description` or `username/feature-name`
- Bug fix: `fix/issue-number-description`
- Docs: `docs/what-changed`

### Commit Messages

Follow conventional commits format:

```
type(scope): brief description

Longer description if needed

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Pull Request Process

1. Create feature branch
2. Make changes with clear commits
3. Run `./scripts/test.sh` to ensure quality
4. Push to your fork
5. Open PR with comprehensive description
6. Address review feedback
7. Merge once approved

## Pre-Commit Hooks

Automated checks run before each commit:

```bash
# Install hooks
./scripts/dev-setup.sh

# Manually run checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all
```

To bypass hooks (emergency only):
```bash
git commit --no-verify
```

## IDE Setup

### VS Code (Recommended)

Install extensions (`.vscode/extensions.json`):
- rust-analyzer
- CodeLLDB (debugging)
- Even Better TOML
- Error Lens

Settings are pre-configured in `.vscode/settings.json`.

### IntelliJ IDEA / CLion

- Install Rust plugin
- Import project as Cargo project
- Run configurations provided

## Performance Profiling

### Flamegraph

```bash
cargo install flamegraph
sudo cargo flamegraph
```

### Benchmarking

```bash
# Run benchmarks (if implemented)
cargo bench
```

## Release Build Optimization

The release profile in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

Results in:
- Binary size: 3-5MB
- Response latency: <1ms
- Memory usage: 5-10MB

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for full guidelines.

### Quick Checklist

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for features)
- [ ] Commit messages are clear
- [ ] No new clippy warnings

## Getting Help

- **Documentation**: [https://amiable-dev.github.io/midimon/](https://amiable-dev.github.io/midimon/)
- **Discussions**: [GitHub Discussions](https://github.com/amiable-dev/midimon/discussions)
- **Issues**: [GitHub Issues](https://github.com/amiable-dev/midimon/issues)

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [midir Documentation](https://docs.rs/midir/)
- [MIDI Specification](https://www.midi.org/specifications)

---

**Happy coding!** Thank you for contributing to MIDIMon.
