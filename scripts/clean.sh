#!/bin/bash
echo "🧹 Cleaning build artifacts..."
cargo clean
rm -rf dist/
echo "✅ Clean complete!"
