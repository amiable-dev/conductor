#!/bin/bash
set -e

echo "🔨 Building MIDIMon Release"
echo "==========================="

# Detect architecture
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

echo "Platform: $OS-$ARCH"

# Build release binary
echo ""
echo "📦 Building release binary..."
cargo build --release

# Strip symbols
echo ""
echo "🔪 Stripping debug symbols..."
strip target/release/midimon 2>/dev/null || echo "  (strip not available)"

# Get binary size
SIZE=$(du -h target/release/midimon | cut -f1)
echo "✅ Binary size: $SIZE"

# Create dist directory
mkdir -p dist

# Copy binary
cp target/release/midimon dist/
cp config.toml dist/config.toml.example

echo ""
echo "✅ Release build complete!"
echo "   Binary: dist/midimon"
echo "   Example config: dist/config.toml.example"
