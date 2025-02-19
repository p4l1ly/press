#!/bin/sh

../sdf-viewer/target/release/sdf-viewer app \
  --loading-passes=1 --max-voxels-side=128 \
  url http://127.0.0.1:8080/target/wasm32-unknown-unknown/release/press.wasm
