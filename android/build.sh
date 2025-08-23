#!/bin/bash

# Android build script for WebSocketReflectorX
# This script should be run from the android/ directory

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DESKTOP_DIR="$PROJECT_ROOT/crates/desktop"

echo "🤖 Building WebSocketReflectorX Android App"
echo "Android config: $SCRIPT_DIR"
echo "Desktop crate: $DESKTOP_DIR"
echo ""

# Check if we have the required tools
if ! command -v cargo-apk >/dev/null 2>&1; then
    echo "❌ cargo-apk not found. Install it with: cargo install cargo-apk"
    exit 1
fi

# Verify Android files exist
if [ ! -f "$SCRIPT_DIR/AndroidManifest.xml" ]; then
    echo "❌ AndroidManifest.xml not found in android/ directory"
    exit 1
fi

if [ ! -d "$SCRIPT_DIR/res" ]; then
    echo "❌ res/ directory not found in android/ directory"
    exit 1
fi

echo "✅ Android configuration files found"
echo ""

# Build for ARM64 (recommended for modern devices)
echo "📱 Building for ARM64 (aarch64)..."
cd "$DESKTOP_DIR"
cargo apk build --release --target aarch64-linux-android

# Build for ARM32 (compatibility with older devices)
echo "📱 Building for ARM32 (armv7)..."
cargo apk build --release --target armv7-linux-androideabi

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 Generated APK files:"
find "$DESKTOP_DIR/target" -name "*.apk" -type f | while read apk; do
    echo "   $apk"
    echo "   Size: $(du -h "$apk" | cut -f1)"
done

echo ""
echo "🚀 To install on device:"
echo "   # For ARM64 devices (most modern phones)"
echo "   adb install \"\$(find $DESKTOP_DIR/target/aarch64-linux-android -name '*.apk' | head -1)\""
echo ""
echo "   # For ARM32 devices (older phones)"
echo "   adb install \"\$(find $DESKTOP_DIR/target/armv7-linux-androideabi -name '*.apk' | head -1)\""
