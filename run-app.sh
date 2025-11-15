#!/bin/sh

# Run app for a specific press object
# Usage: ./run-app.sh <object_name>
# Note: Requires run-server.sh to be running first

if [ -z "$1" ]; then
    echo "Usage: $0 <object_name>"
    echo "Example: $0 mosquito"
    exit 1
fi

OBJECT=$1
MAX_VOXELS_SIDE=${MAX_VOXELS_SIDE:-64}

WASM_FILE="target/wasm32-unknown-unknown/release/press_${OBJECT}.wasm"

if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found: $WASM_FILE"
    echo "Please build the object first: ./build.sh $OBJECT"
    exit 1
fi

# The server serves the file at the path specified by -s flag in run-server.sh
# which is: target/wasm32-unknown-unknown/release/press_${OBJECT}.wasm
WASM_URL="http://127.0.0.1:8080/$WASM_FILE"

echo "Running app for object: $OBJECT"
echo "Connecting to server at: $WASM_URL"
../sdf-viewer/target/release/sdf-viewer app \
  --loading-passes=1 --max-voxels-side=$MAX_VOXELS_SIDE \
  url "$WASM_URL"
