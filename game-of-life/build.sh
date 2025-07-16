#!/bin/bash

# Conway's Game of Life - WebAssembly Build Script
echo "🦀 Building Conway's Game of Life with WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build the WebAssembly module
echo "📦 Building WebAssembly module..."
wasm-pack build --target web --out-dir pkg

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    
    # Move generated files to www directory
    echo "📁 Moving files to www directory..."
    mkdir -p www/pkg
    cp -r pkg/* www/pkg/
    
    echo "🌐 You can now serve the application from the www directory"
    echo "💡 Run: cd www && python3 -m http.server 8000"
    echo "🚀 Then open: http://localhost:8000"
else
    echo "❌ Build failed!"
    exit 1
fi 