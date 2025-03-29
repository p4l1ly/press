#! /bin/sh

if [ -z "$1" ]; then
  echo "Usage: $0 <name>"
  exit 1
fi

rm -rf out/$1.ply
../sdf-viewer/target/release/sdf-viewer mesh \
  -v 200 -i ./target/wasm32-unknown-unknown/release/press.wasm -o out/$1.ply marching-cubes
