# MIDIMon Development Tasks
# Install just: cargo install just
# Run: just <command>

# Default recipe to display help information
default:
    @just --list

# Run all tests
test:
    cargo test --all-features

# Run tests with nextest (improved output)
test-nextest:
    ./scripts/test-nextest.sh

# Run tests in watch mode
test-watch:
    cargo watch -x "test --all-features"

# Generate code coverage report (terminal summary)
coverage:
    ./scripts/coverage.sh

# Generate HTML coverage report
coverage-html:
    ./scripts/coverage.sh --html

# Generate HTML coverage report and open in browser
coverage-open:
    ./scripts/coverage.sh --open

# Generate lcov.info for CI
coverage-lcov:
    ./scripts/coverage.sh --lcov

# Run linter (clippy)
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Build in debug mode
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# Run the main application (requires port argument)
run PORT:
    cargo run --release {{PORT}}

# Run with LED lighting scheme
run-led PORT SCHEME:
    cargo run --release {{PORT}} --led {{SCHEME}}

# Run MIDI diagnostic tool
diagnostic PORT:
    cargo run --bin midi_diagnostic {{PORT}}

# List available MIDI ports
ports:
    cargo run --bin test_midi

# Run all CI checks locally (lint, format, test, coverage)
ci: fmt-check lint test coverage
    @echo "All CI checks passed!"

# Install development dependencies
dev-setup:
    ./scripts/dev-setup.sh

# Run security audit
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Generate documentation
docs:
    cargo doc --all-features --no-deps

# Open documentation in browser
docs-open:
    cargo doc --all-features --no-deps --open
