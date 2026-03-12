#!/bin/bash
# Pack Treeline .mcpb extension for a specific platform
#
# Usage: ./pack.sh <binary-path> <platform> <version> [output-dir]
#
# platform: macos-arm64, macos-x64, windows-x64
# binary-path: path to the tl binary for this platform
#
# Requires: npm install -g @anthropic-ai/mcpb

set -euo pipefail

BINARY_PATH="$1"
PLATFORM="$2"
VERSION="$3"
OUTPUT_DIR="${4:-.}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORK_DIR=$(mktemp -d)

trap 'rm -rf "$WORK_DIR"' EXIT

# Copy manifest and update version
cp "$SCRIPT_DIR/manifest.json" "$WORK_DIR/manifest.json"

# Update version in manifest
if command -v python3 &>/dev/null; then
  python3 -c "
import json, sys
with open('$WORK_DIR/manifest.json', 'r') as f:
    m = json.load(f)
m['version'] = '$VERSION'
with open('$WORK_DIR/manifest.json', 'w') as f:
    json.dump(m, f, indent=2)
"
else
  sed -i.bak "s/\"version\": \"0.0.0\"/\"version\": \"$VERSION\"/" "$WORK_DIR/manifest.json"
  rm -f "$WORK_DIR/manifest.json.bak"
fi

# Copy icon if it exists
if [ -f "$SCRIPT_DIR/icon.png" ]; then
  cp "$SCRIPT_DIR/icon.png" "$WORK_DIR/icon.png"
fi

# Copy binary into server/
mkdir -p "$WORK_DIR/server"
cp "$BINARY_PATH" "$WORK_DIR/server/tl"
chmod +x "$WORK_DIR/server/tl"

# Determine output filename
OUTPUT_FILE="$OUTPUT_DIR/treeline-${PLATFORM}.mcpb"

# Pack
mcpb pack "$WORK_DIR" "$OUTPUT_FILE"

echo "Packed: $OUTPUT_FILE"
