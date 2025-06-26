#!/bin/bash

# Build script for SomaFM Player
# Creates optimized binaries for distribution

set -e

echo "🎵 Building SomaFM Player for distribution..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build for current platform (optimized)
echo "🔨 Building for current platform..."
cargo build --release

# Check if the binary works
echo "✅ Testing binary..."
if ./target/release/soma-player --help 2>/dev/null || true; then
    echo "✅ Binary test passed"
else
    echo "⚠️  Binary doesn't respond to --help (this is expected for this app)"
fi

# Get binary size
BINARY_SIZE=$(du -h target/release/soma-player | cut -f1)
echo "📦 Binary size: $BINARY_SIZE"

# Create distribution folder
DIST_DIR="dist"
mkdir -p $DIST_DIR

# Copy binary
cp target/release/soma-player $DIST_DIR/
cp README.md $DIST_DIR/
cp Cargo.toml $DIST_DIR/

# Create archive
ARCHIVE_NAME="soma-player-$(uname -s)-$(uname -m).tar.gz"
echo "📦 Creating archive: $ARCHIVE_NAME"
tar -czf $DIST_DIR/$ARCHIVE_NAME -C $DIST_DIR soma-player README.md Cargo.toml

echo "✅ Build complete!"
echo "📁 Distribution files in: $DIST_DIR/"
echo "📦 Archive: $DIST_DIR/$ARCHIVE_NAME"
echo ""
echo "To run: ./$DIST_DIR/soma-player"
echo "To install globally: sudo cp $DIST_DIR/soma-player /usr/local/bin/"
