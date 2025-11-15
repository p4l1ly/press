#!/bin/sh

# Build script for press objects
# Usage: ./build.sh <object_name>

if [ -z "$1" ]; then
    echo "Usage: $0 <object_name>"
    echo "Example: $0 mosquito"
    exit 1
fi

OBJECT=$1

echo "Building object: $OBJECT"
cargo build -p press-$OBJECT --lib --target wasm32-unknown-unknown --release
