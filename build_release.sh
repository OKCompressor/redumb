#!/usr/bin/env bash
# build_release.sh — build a stripped & compressed release binary

set -euo pipefail

echo "🛠  Building release..."
cargo build --release

echo "🧹 Stripping symbols..."
strip target/release/redumb

echo "📦 Compressing with UPX..."
upx target/release/redumb

echo "✅ Done! Release binary is at target/release/redumb"

# Copy the packed binary to project root for easy access
cp target/release/redumb .
chmod +x build_release.sh

