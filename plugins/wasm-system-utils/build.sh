#!/bin/bash
set -e

echo "Building WASM plugin..."
cargo build --target wasm32-wasip1 --release

WASM_FILE="target/wasm32-wasip1/release/$(echo "midimon-wasm-system-utils" | tr '-' '_').wasm"
SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')

echo "✅ Plugin built successfully!"
echo "   Size: $SIZE"
echo "   Path: $WASM_FILE"
