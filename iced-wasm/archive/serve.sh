#!/usr/bin/env bash
set -e

echo "🔨 Building WASM in development mode..."
cd iced-wasm

# Build WASM with wasm-pack
nix develop -c wasm-pack build --target web --out-dir pkg --dev

echo "✅ Build complete!"
echo "🚀 Starting local server on http://localhost:8080"
echo "📁 Open http://localhost:8080 in your browser"
echo "Press Ctrl+C to stop the server"

# Serve the files locally
nix-shell -p python3 --run "python3 -m http.server 8080 --bind 127.0.0.1"