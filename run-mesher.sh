#! /bin/sh

# Run mesher for a specific press object
# Usage: ./run-mesher.sh <output_name> <object_name>

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <output_name> <object_name>"
  echo "Example: $0 my_mesh mosquito"
  exit 1
fi

MAX_VOXELS_SIDE=${MAX_VOXELS_SIDE:-64}
OUTPUT_NAME=$1
OBJECT=$2

echo "Generating mesh for object: $OBJECT -> out/$OUTPUT_NAME.ply"
rm -rf out/$OUTPUT_NAME.ply
../sdf-viewer/target/release/sdf-viewer mesh \
  -v $MAX_VOXELS_SIDE -i ./target/wasm32-unknown-unknown/release/press_${OBJECT}.wasm -o out/$OUTPUT_NAME.ply marching-cubes
