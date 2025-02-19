#!/bin/sh

../sdf-viewer/target/release/sdf-viewer server \
  -s target/wasm32-unknown-unknown/release/press.wasm -w src/ -b ./build.sh
