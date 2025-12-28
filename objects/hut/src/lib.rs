// Template for new press objects
// This is a simple sphere example - modify as needed

pub use cgmath::Vector3;
pub use cgmath::num_traits::Pow;
pub use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};

/// Entry point called when the WASM module is loaded
#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(MyObject::default()));
}

const BOUNDARY: f32 = 0.5;

/// Configuration for your object
#[derive(Debug, Clone)]
pub struct Config {
    pub root_z: f32,
    pub root_r: f32,
    pub wall_r: f32,
    pub thickness: f32,
    pub door_angle: f32,
    pub door_root_z: f32,
    pub door_root_r: f32,
    pub door_wall_r: f32,
    pub door_slope: f32,
    pub door_widening: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_z: 1.0,
            root_r: 2.0,
            wall_r: 5.0,
            thickness: 0.3,
            door_angle: 0.5,
            door_root_z: 1.0,
            door_root_r: 4.0,
            door_wall_r: 6.0,
            door_slope: 0.4,
            door_widening: 0.0,
        }
    }
}

/// Your SDF object
#[derive(Debug, Clone)]
pub struct MyObject {
    pub root_z: f32,
    pub root_r: f32,
    pub wall_r: f32,
    pub thickness: f32,
    pub door_angle: f32,
    pub door_root_z: f32,
    pub door_root_r: f32,
    pub door_wall_r: f32,
    pub door_slope: f32,
    pub door_widening: f32,
    pub top: f32,
}

impl MyObject {
    fn new(cfg: Config) -> Self {
        Self {
            root_z: cfg.root_z,
            root_r: cfg.root_r,
            wall_r: cfg.wall_r,
            thickness: cfg.thickness,
            door_angle: cfg.door_angle,
            door_root_z: cfg.door_root_z,
            door_root_r: cfg.door_root_r,
            door_wall_r: cfg.door_wall_r,
            door_slope: cfg.door_slope,
            door_widening: cfg.door_widening,
            top: cfg.wall_r - cfg.root_z,
        }
    }
}

impl Default for MyObject {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl SDFSurface for MyObject {
    /// Define the bounding box for your object
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        let r = self.wall_r - self.root_r;
        [
            Vector3::new(-r - BOUNDARY, -r - BOUNDARY, -BOUNDARY),
            Vector3::new(r + BOUNDARY, r + BOUNDARY,  self.top + BOUNDARY + 3.0),
        ]
    }

    /// Sample the SDF at point p
    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x;
        let y = p.y;
        let z = p.z;

        let xy_angle = y.atan2(x);
        let xy_from_root = (x * x + y * y).sqrt() + self.root_r;
        let z_from_root = z + self.root_z;
        let distance_from_root = (xy_from_root * xy_from_root + z_from_root * z_from_root).sqrt();

        let sdf = distance_from_root - self.wall_r - self.thickness;
        let sdf = sdf.max(self.door_angle - xy_angle.abs());

        let door_z_from_root = z + self.door_root_z + self.door_slope * x;
        let door_y_from_root = -y.abs() - self.door_root_r + self.door_widening * x;
        let door_distance_from_root = (door_y_from_root * door_y_from_root + door_z_from_root * door_z_from_root).sqrt();
        let door_sdf = door_distance_from_root - self.door_wall_r - self.thickness;
        let door_sdf = door_sdf.max(-(door_distance_from_root - self.door_wall_r + self.thickness));
        let door_sdf = door_sdf.max(-x);
        let sdf = sdf.min(door_sdf);
        let sdf = sdf.max(-(distance_from_root - self.wall_r + self.thickness));

        SDFSample::new(
            sdf.max(-z),
            Vector3::new((distance_from_root / self.wall_r / 1.5).powf(3.0), 0.0, 0.0), // Color (RGB)
        )
    }
}

// You can add helper functions here

