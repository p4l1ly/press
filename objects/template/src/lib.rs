// Template for new press objects
// This is a simple sphere example - modify as needed

pub use cgmath::Vector3;
pub use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};

/// Entry point called when the WASM module is loaded
#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(MyObject::default()));
}

/// Configuration for your object
#[derive(Debug, Clone)]
pub struct Config {
    pub radius: f64,
    // Add more configuration parameters here
}

impl Default for Config {
    fn default() -> Self {
        Self {
            radius: 10.0,
        }
    }
}

/// Your SDF object
#[derive(Debug, Clone)]
pub struct MyObject {
    pub cfg: Config,
}

impl Default for MyObject {
    fn default() -> Self {
        Self {
            cfg: Config::default(),
        }
    }
}

impl SDFSurface for MyObject {
    /// Define the bounding box for your object
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        let r = self.cfg.radius as f32;
        [
            Vector3::new(-r - 5.0, -r - 5.0, -r - 5.0),
            Vector3::new(r + 5.0, r + 5.0, r + 5.0),
        ]
    }

    /// Sample the SDF at point p
    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x as f64;
        let y = p.y as f64;
        let z = p.z as f64;

        // Simple sphere SDF: distance = sqrt(x^2 + y^2 + z^2) - radius
        let distance = (x * x + y * y + z * z).sqrt() - self.cfg.radius;

        SDFSample::new(
            distance as f32,
            Vector3::new(1.0, 0.0, 0.0), // Color (RGB)
        )
    }
}

// You can add helper functions here

