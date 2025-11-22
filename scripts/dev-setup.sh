#!/bin/bash
set -e

echo "🚀 MIDIMon Development Environment Setup"
echo "========================================"

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo "✅ Rust found: $(rustc --version)"
fi

# Install required components
echo ""
echo "📦 Installing Rust components..."
rustup component add rustfmt clippy

# Install useful cargo extensions
echo ""
echo "🔧 Installing cargo extensions..."
cargo install --quiet cargo-watch 2>/dev/null || echo "  cargo-watch already installed"
cargo install --quiet cargo-edit 2>/dev/null || echo "  cargo-edit already installed"

# Set up git hooks
echo ""
echo "🪝 Setting up git hooks..."
mkdir -p .git/hooks

cat > .git/hooks/pre-commit << 'HOOK'
#!/bin/bash
set -e
echo "Running pre-commit checks..."
cargo fmt -- --check || { echo "❌ Format check failed. Run: cargo fmt"; exit 1; }
cargo clippy -- -D warnings || { echo "❌ Clippy failed"; exit 1; }
echo "✅ Pre-commit checks passed!"
HOOK

chmod +x .git/hooks/pre-commit
echo "✅ Pre-commit hook installed"

# Check system dependencies
echo ""
echo "🔍 Checking system dependencies..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✅ macOS detected"
    if xcode-select -p &> /dev/null; then
        echo "✅ Xcode Command Line Tools installed"
    else
        echo "⚠️  Xcode Command Line Tools not found"
        echo "   Run: xcode-select --install"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "✅ Linux detected"
    if pkg-config --exists alsa; then
        echo "✅ ALSA development libraries found"
    else
        echo "⚠️  ALSA development libraries not found"
        echo "   Run: sudo apt-get install libasound2-dev"
    fi
fi

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Next steps:"
echo "  1. cargo build --release"
echo "  2. cargo test --all"
echo "  3. cargo run --release"
echo ""
echo "See DEVELOPMENT.md for detailed instructions."
