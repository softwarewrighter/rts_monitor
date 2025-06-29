#!/bin/bash

# Build script for RTS Monitor

set -e  # Exit on error

echo "🔨 Building RTS Monitor..."

# Change to project root directory
cd "$(dirname "$0")/.."

# Build with wasm-pack
wasm-pack build --target web --out-dir pkg

echo "✅ Build complete!"
echo "📦 Output files are in the pkg/ directory"