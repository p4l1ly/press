# Press - SDF Object Library

This project contains multiple 3D objects defined using Signed Distance Functions (SDFs) for visualization and meshing.

## Project Structure

```
press/
├── Cargo.toml              # Workspace definition
├── build.sh                # Build script for objects
├── run-app.sh              # Run viewer application
├── run-mesher.sh           # Generate mesh files
├── run-server.sh           # Development server with hot reload
├── common/                 # Shared library for all objects
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          # Common utilities (SDF helpers, macros)
└── objects/                # Individual object definitions
    └── mosquito/            # Mosquito press object
        ├── Cargo.toml
        └── src/
            ├── lib.rs      # Main SDF implementation
            └── bin.rs      # Debug binary
```

## Building Objects

Build a specific object:

```bash
./build.sh <object_name>
```

Examples:
```bash
./build.sh mosquito       # Builds mosquito
```

The compiled WASM files are located at:
```
target/wasm32-unknown-unknown/release/press_<object_name>.wasm
```

## Running Objects

### Interactive Viewer

```bash
./run-app.sh <object_name>
```

### Development Server (with hot reload)

```bash
./run-server.sh <object_name>
```

### Generate Mesh

```bash
./run-mesher.sh <output_name> <object_name>
```

Example:
```bash
./run-mesher.sh my_mosquito mosquito
# Creates out/my_mosquito.ply
```

## Adding a New Object

1. Create a new directory under `objects/`:
   ```bash
   mkdir -p objects/my_object/src
   ```

2. Create `objects/my_object/Cargo.toml`:
   ```toml
   [package]
   name = "press-my_object"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   press-common = { path = "../../common" }
   cgmath = { version = "0.18" }
   wasm-bindgen = "0.2.95"
   once_cell = "1.8.0"

   [lib]
   crate-type = ["cdylib", "rlib"]

   [[bin]]
   name = "press-my_object"
   path = "src/bin.rs"
   ```

3. Create `objects/my_object/src/lib.rs`:
   ```rust
   use press_common::{set_root_sdf, SDFSample, SDFSurface, Vector3};

   #[no_mangle]
   pub extern "C" fn init() {
       set_root_sdf(Box::new(MyObject::default()));
   }

   #[derive(Default)]
   pub struct MyObject;

   impl SDFSurface for MyObject {
       fn bounding_box(&self) -> [Vector3<f32>; 2] {
           [Vector3::new(-10.0, -10.0, -10.0), Vector3::new(10.0, 10.0, 10.0)]
       }

       fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
           // Simple sphere SDF
           let distance = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt() - 5.0;
           SDFSample::new(distance, Vector3::new(1.0, 0.0, 0.0))
       }
   }
   ```

4. Add the new object to the workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "common",
       "objects/mosquito",
       "objects/my_object",  # Add this line
   ]
   ```

5. Build and run:
   ```bash
   ./build.sh my_object
   ./run-app.sh my_object
   ```

## Common Library

The `common` library provides shared utilities:

- **`SDFSurface`** - Trait for SDF objects
- **`SDFSample`** - SDF sample result
- **`set_root_sdf`** - Set the root SDF object
- **`circ_coordinates`** - Circular coordinate transformation
- **`trapezoid`** - Trapezoid SDF helper
- **`create_computation!`** - Macro for lazy-evaluated computation structs

See `common/src/lib.rs` for detailed documentation.

## Environment Variables

- **`MAX_VOXELS_SIDE`** - Maximum voxel grid resolution (default: 128)
  ```bash
  MAX_VOXELS_SIDE=256 ./run-app.sh mosquito
  ```

## Requirements

- Rust toolchain with `wasm32-unknown-unknown` target
- [sdf-viewer](https://github.com/eliasdaler/sdf-viewer) built in parallel directory (`../sdf-viewer`)

## Install WASM target

```bash
rustup target add wasm32-unknown-unknown
```

