# Quick Start Guide

## Common Commands

### Build a specific object
```bash
./build.sh mosquito
```

### Run the viewer
```bash
./run-app.sh mosquito
```

### Start development server (with hot reload)
```bash
./run-server.sh mosquito
```

### Generate a mesh file
```bash
./run-mesher.sh output_name mosquito
# Creates: out/output_name.ply
```

### Run the debug binary
```bash
cargo run -p press-mosquito --bin press-mosquito --release
```

## Project Structure at a Glance

```
press/
├── common/                     # Shared utilities
│   └── src/lib.rs             # SDF helpers, macros
├── objects/
│   ├── mosquito/              # The mosquito press object
│   │   └── src/
│   │       ├── lib.rs         # Main implementation
│   │       └── bin.rs         # Debug binary
│   └── template/              # Template for new objects
├── build.sh                   # Build: ./build.sh <object>
├── run-app.sh                 # View: ./run-app.sh <object>
├── run-mesher.sh              # Mesh: ./run-mesher.sh <name> <object>
└── run-server.sh              # Dev: ./run-server.sh <object>
```

## Adding a New Object (5 minutes)

1. **Copy the template**
   ```bash
   cp -r objects/template objects/sphere
   ```

2. **Edit `objects/sphere/Cargo.toml`**
   - Replace `<OBJECT_NAME>` with `sphere`

3. **Edit `objects/sphere/src/lib.rs`**
   - Implement your SDF in the `sample()` method
   - Update `bounding_box()` if needed

4. **Add to workspace**
   Edit root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "common",
       "objects/mosquito",
       "objects/sphere",  # Add this
   ]
   ```

5. **Build and test**
   ```bash
   ./build.sh sphere
   ./run-app.sh sphere
   ```

## Example: Create a Cube Object

```bash
# 1. Copy template
cp -r objects/template objects/cube

# 2. Edit objects/cube/Cargo.toml
sed -i 's/<OBJECT_NAME>/cube/g' objects/cube/Cargo.toml

# 3. Edit objects/cube/src/lib.rs
# (implement cube SDF - see below)

# 4. Add to workspace (edit Cargo.toml manually)

# 5. Build
./build.sh cube
```

Example cube SDF for `objects/cube/src/lib.rs`:
```rust
fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
    let size = 10.0;
    let dx = p.x.abs() - size;
    let dy = p.y.abs() - size;
    let dz = p.z.abs() - size;
    
    let distance = dx.max(dy).max(dz);
    
    SDFSample::new(distance, Vector3::new(0.0, 1.0, 0.0))
}
```

## Useful Resources

- **Main docs**: See `README.md`
- **Migration info**: See `MIGRATION.md`
- **Template guide**: See `objects/template/README.md`
- **SDF functions**: https://iquilezles.org/articles/distfunctions/

## Troubleshooting

### Build fails with "package not found"
Make sure you added your object to the workspace in root `Cargo.toml`

### WASM file not found
The file is named `press_<object>.wasm`, not `press.wasm`

### Changes not reloading
Make sure you're using `run-server.sh` with the correct object name

## Environment Variables

```bash
# Higher resolution
MAX_VOXELS_SIDE=256 ./run-app.sh mosquito
```

