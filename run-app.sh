#!/bin/sh

MAX_VOXELS_SIDE=${MAX_VOXELS_SIDE:-128}

../sdf-viewer/target/release/sdf-viewer app \
  --loading-passes=1 --max-voxels-side=$MAX_VOXELS_SIDE \
  url http://127.0.0.1:8080/target/wasm32-unknown-unknown/release/press.wasm
