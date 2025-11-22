#!/bin/bash
set -e

echo "🧪 Running MIDIMon Test Suite"
echo "============================="

# Format check
echo ""
echo "📝 Checking code formatting..."
cargo fmt -- --check

# Clippy
echo ""
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Tests
echo ""
echo "🧪 Running tests..."
cargo test --all

echo ""
echo "✅ All checks passed!"
