#!/bin/sh

# Run development server for a specific press object
# Usage: ./run-server.sh <object_name>

if [ -z "$1" ]; then
    echo "Usage: $0 <object_name>"
    echo "Example: $0 mosquito"
    exit 1
fi

OBJECT=$1

# Get absolute path to build script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_SCRIPT="$SCRIPT_DIR/build.sh"

echo "Starting development server for object: $OBJECT"
cd "$SCRIPT_DIR"
../sdf-viewer/target/release/sdf-viewer server \
  -s target/wasm32-unknown-unknown/release/press_${OBJECT}.wasm \
  -w objects/$OBJECT/src/ \
  -b "$BUILD_SCRIPT" -b "$OBJECT"
