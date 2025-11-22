# Contributing to Conductor

Thank you for your interest in contributing to Conductor! We welcome contributions of all kinds - from bug reports and documentation improvements to new features and hardware support.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Communication](#communication)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@amiable.dev.

## Ways to Contribute

### 🐛 Bug Reports

Found a bug? Please open a [GitHub issue](https://github.com/amiable-dev/conductor/issues/new) with:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version, MIDI device)
- Relevant log output (run with `DEBUG=1` for verbose logs)

### 💡 Feature Requests

Have an idea? We'd love to hear it! Open a [GitHub Discussion](https://github.com/amiable-dev/conductor/discussions/new?category=ideas) with:
- Clear description of the proposed feature
- Use cases and benefits
- Potential implementation approaches (optional)
- Alternatives considered (optional)

### 📖 Documentation

Documentation improvements are always welcome:
- Fix typos or clarify existing docs
- Add examples for common use cases
- Improve API documentation
- Write tutorials or guides
- Translate documentation

### 🔧 Code Contributions

Ready to write code? Check out:
- [Good First Issues](https://github.com/amiable-dev/conductor/labels/good-first-issue)
- [Help Wanted](https://github.com/amiable-dev/conductor/labels/help-wanted)
- Incomplete feature implementations

### 🎹 Device Support

Help us support more MIDI controllers:
- Test Conductor with your device
- Create device profiles (.ncmm3 or config templates)
- Document device-specific quirks or features
- Implement LED feedback for new devices

### 🔌 WASM Plugins

Extend Conductor's functionality with sandboxed WASM plugins:
- Create plugins for media control (Spotify, iTunes, etc.)
- Build system utility plugins (screenshots, clipboard, etc.)
- Develop DAW integration plugins (Logic Pro, Ableton, etc.)
- Add home automation plugins (smart lights, etc.)
- See `plugins/README.md` for details
- Use `./scripts/new-plugin.sh` to get started

**Plugin requirements:**
- Must be useful and solve a real problem
- Request only necessary capabilities
- Include comprehensive tests
- Provide clear documentation
- Use MIT or compatible license

See the [Plugin Development Guide](docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md) for complete instructions.

## Development Setup

### Prerequisites

- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **macOS** 10.15+ (Linux/Windows support planned)
- **MIDI Controller** (optional for testing, required for LED feedback)
- **Git** for version control

### Setup Steps

1. **Fork and Clone**
   ```bash
   git fork amiable-dev/conductor
   git clone https://github.com/YOUR_USERNAME/conductor.git
   cd conductor
   ```

2. **Build the Project**
   ```bash
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

4. **Run Conductor**
   ```bash
   # List available MIDI ports
   cargo run --release

   # Connect to a specific port
   cargo run --release 2

   # Enable debug logging
   DEBUG=1 cargo run --release 2
   ```

5. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Maximum line length: 100 characters
- Use meaningful variable names
- Add comments for complex logic

### Project Structure

```
src/
├── main.rs              # Entry point, MIDI connection, event loop
├── config.rs            # Configuration structures (TOML parsing)
├── event_processor.rs   # MIDI event → Processed event
├── mappings.rs          # Mapping engine
├── actions.rs           # Action execution
├── feedback.rs          # LED feedback trait
├── mikro_leds.rs        # Maschine Mikro MK3 LED control
├── midi_feedback.rs     # Generic MIDI LED fallback
└── device_profile.rs    # NI profile parser
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add support for MIDI Learn mode
fix: resolve double-tap detection timing issue
docs: update configuration examples
test: add integration tests for velocity detection
refactor: extract LED control to trait
chore: update dependencies
```

### Documentation

- Add doc comments (`///`) for public items
- Include examples in doc comments where helpful
- Update `README.md` if adding user-facing features
- Update configuration documentation for new trigger/action types

## Pull Request Process

### Before Submitting

1. **Ensure Tests Pass**
   ```bash
   cargo test
   ```

2. **Check Formatting**
   ```bash
   cargo fmt --check
   ```

3. **Run Clippy**
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Update Documentation**
   - Update `README.md` for user-facing changes
   - Add doc comments for new public APIs
   - Update configuration examples if needed

5. **Test Manually**
   - Test with a real MIDI device if possible
   - Verify LED feedback works correctly
   - Check configuration loading

### Submitting

1. **Push to Your Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open Pull Request**
   - Clear title following conventional commit format
   - Description explaining what and why
   - Link related issues
   - Add screenshots/videos if applicable
   - Check the box for "Allow edits from maintainers"

3. **Respond to Review**
   - Address reviewer comments
   - Push updates to the same branch
   - Request re-review when ready

### PR Requirements

- ✅ All tests pass (CI will verify)
- ✅ Code follows style guidelines
- ✅ Commits follow conventional commit format
- ✅ Documentation updated
- ✅ No unrelated changes
- ✅ Squash commits if requested

## Testing Guidelines

### Test Coverage

- Aim for 85%+ test coverage
- Write unit tests for business logic
- Write integration tests for event processing
- Write E2E tests for critical workflows

### Test Organization

```rust
// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_detection() {
        // Test implementation
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_velocity_detection

# With output
cargo test -- --nocapture

# Coverage report
cargo tarpaulin --workspace
```

### Test Checklist

- [ ] Unit tests for new functions
- [ ] Integration tests for event flows
- [ ] Edge cases covered
- [ ] Error conditions tested
- [ ] Documentation examples tested

## Communication

### GitHub Discussions

Use [Discussions](https://github.com/amiable-dev/conductor/discussions) for:
- Questions about usage
- Feature ideas and proposals
- General community chat
- Show & tell (your configurations, videos, etc.)

### GitHub Issues

Use [Issues](https://github.com/amiable-dev/conductor/issues) for:
- Bug reports
- Approved feature requests
- Documentation improvements

### Discord (Coming Soon)

Real-time chat for:
- Development questions
- Collaboration
- Community support

### Email

- **Security**: security@amiable.dev
- **Code of Conduct**: conduct@amiable.dev
- **General**: hello@amiable.dev

## First-Time Contributors

New to open source? No problem! Here's how to get started:

1. **Look for Good First Issues**: We label beginner-friendly issues with [good-first-issue](https://github.com/amiable-dev/conductor/labels/good-first-issue)
2. **Read the Docs**: Familiarize yourself with the project structure
3. **Set Up Development**: Follow the development setup guide
4. **Ask Questions**: Don't hesitate to ask in Discussions
5. **Start Small**: Fix a typo, improve docs, or tackle a small bug
6. **Learn and Grow**: Every contribution helps you learn

## License

By contributing to Conductor, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

Thank you for contributing to Conductor! 🎹
