# Template for New Press Objects

This directory contains a template for creating new SDF objects.

## Quick Start

1. Copy this entire directory to a new name:
   ```bash
   cp -r objects/template objects/my_object
   ```

2. Edit `objects/my_object/Cargo.toml`:
   - Replace `<OBJECT_NAME>` with your object name (e.g., `sphere`, `cube`)

3. Edit `objects/my_object/src/lib.rs`:
   - Implement your SDF logic in the `sample()` method
   - Update the `bounding_box()` to fit your object
   - Add any configuration parameters to `Config`

4. Edit `objects/my_object/src/bin.rs`:
   - Replace `press_<OBJECT_NAME>` with your crate name
   - Add any debug code you need

5. Add your object to the workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "common",
       "objects/mosquito",
       "objects/my_object",  # Add this
   ]
   ```

6. Build and run:
   ```bash
   ./build.sh my_object
   ./run-app.sh my_object
   ```

## SDF Resources

- [Inigo Quilez's SDF functions](https://iquilezles.org/articles/distfunctions/)
- [SDF Tutorial](https://www.shadertoy.com/view/Xds3zN)

## Example: Simple Box

```rust
fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
    let x = p.x.abs() as f64;
    let y = p.y.abs() as f64;
    let z = p.z.abs() as f64;
    
    let size = 10.0;
    let distance = (x - size).max(y - size).max(z - size);
    
    SDFSample::new(distance as f32, Vector3::new(0.0, 1.0, 0.0))
}
```

