// Template for new press objects
// This is a simple sphere example - modify as needed

pub use cgmath::Vector3;
pub use cgmath::num_traits::Pow;
pub use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};
pub use press_common::{rotate_x, rotate_z, translate};
pub use std::f32::consts::PI;

/// Entry point called when the WASM module is loaded
#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(MyObject::default()));
}

const BOUNDARY: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct BrickRow {
    pub angle: f32,
    pub step: f32,
    pub count: usize,
    pub odd: bool,
}

#[derive(Debug, Clone)]
pub struct DoorBrick {
    pub angle: f32,
    pub shift: f32,
    pub brick_width: f32,
}

#[derive(Debug, Clone)]
pub enum Material {
    Straw,
    Wood,
    Clay,
    Theory,
}

/// Configuration for your object
#[derive(Debug, Clone)]
pub struct Config {
    pub root_z: f32,
    pub root_r: f32,
    pub wall_r: f32,
    pub thickness: f32,
    pub brick_width: f32,
    pub brick_height: f32,
    pub door_angle: f32,
    pub door_root_z: f32,
    pub door_root_r: f32,
    pub door_wall_r: f32,
    pub door_slope: f32,
    pub door_length: f32,
    pub half_brick_angle: f32,
    pub brick_rows: Vec<BrickRow>,
    pub door_bricks: Vec<DoorBrick>,
    pub material: Material,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            material: Material::Straw,
            root_z: 1.0,
            root_r: 2.0,
            wall_r: 5.0,
            thickness: 0.3,
            brick_width: 0.4,
            brick_height: 0.2,
            door_angle: 0.5,
            door_root_z: 1.0,
            door_root_r: 4.0,
            door_wall_r: 6.25,
            door_slope: 0.4,
            door_length: 3.6,
            half_brick_angle: 0.1,
            brick_rows: vec![
                BrickRow {
                    angle: 0.25,
                    step: 0.317,
                    count: 8,
                    odd: true,
                },
                BrickRow {
                    angle: 0.345,
                    step: 0.365,
                    count: 7,
                    odd: true,
                },
                BrickRow {
                    angle: 0.44,
                    step: 0.365,
                    count: 7,
                    odd: true,
                },
                BrickRow {
                    angle: 0.535,
                    step: 0.421,
                    count: 6,
                    odd: true,
                },
                BrickRow {
                    angle: 0.63,
                    step: 0.51,
                    count: 5,
                    odd: true,
                },
                BrickRow {
                    angle: 0.725,
                    step: 0.57,
                    count: 5,
                    odd: false,
                },
                BrickRow {
                    angle: 0.82,
                    step: 0.73,
                    count: 4,
                    odd: false,
                },
                BrickRow {
                    angle: 0.915,
                    step: 1.02,
                    count: 3,
                    odd: false,
                },
                BrickRow {
                    angle: 1.01,
                    step: 1.75,
                    count: 2,
                    odd: false,
                },
            ],
            door_bricks: vec![
                DoorBrick {
                    angle: 0.425,
                    shift: 0.0,
                    brick_width: 0.4,
                },
                DoorBrick {
                    angle: 0.5,
                    shift: 0.0,
                    brick_width: 0.4,
                },
                DoorBrick {
                    angle: 0.575,
                    shift: 0.0,
                    brick_width: 0.4,
                },
                DoorBrick {
                    angle: 0.65,
                    shift: 0.0,
                    brick_width: 0.4,
                },
                DoorBrick {
                    angle: 0.725,
                    shift: 0.0,
                    brick_width: 0.4,
                },
                DoorBrick {
                    angle: 0.8,
                    shift: 0.0,
                    brick_width: 0.4,
                },

                DoorBrick {
                    angle: 0.59,
                    shift: 0.86 - (0.4 - 0.09),
                    brick_width: 0.09,
                },
                DoorBrick {
                    angle: 0.66,
                    shift: 0.86 - (0.4 - 0.2),
                    brick_width: 0.2,
                },
                DoorBrick {
                    angle: 0.73,
                    shift: 0.86 - (0.4 - 0.33),
                    brick_width: 0.33,
                },
                DoorBrick {
                    angle: 0.8,
                    shift: 0.86,
                    brick_width: 0.4,
                },
            ],
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
    pub brick_width: f32,
    pub brick_height: f32,
    pub door_angle: f32,
    pub door_root_z: f32,
    pub door_root_r: f32,
    pub door_wall_r: f32,
    pub door_slope: f32,
    pub door_length: f32,
    pub half_brick_angle: f32,
    pub top: f32,
    pub brick_rows: Vec<BrickRow>,
    pub door_bricks: Vec<DoorBrick>,
    pub material: Material,
}

impl MyObject {
    fn new(cfg: Config) -> Self {
        Self {
            root_z: cfg.root_z,
            root_r: cfg.root_r,
            wall_r: cfg.wall_r,
            thickness: cfg.thickness,
            brick_width: cfg.brick_width,
            brick_height: cfg.brick_height,
            door_angle: cfg.door_angle,
            door_root_z: cfg.door_root_z,
            door_root_r: cfg.door_root_r,
            door_wall_r: cfg.door_wall_r,
            door_slope: cfg.door_slope,
            door_length: cfg.door_length,
            half_brick_angle: cfg.half_brick_angle,
            top: cfg.wall_r - cfg.root_z,
            brick_rows: cfg.brick_rows,
            door_bricks: cfg.door_bricks,
            material: cfg.material,
        }
    }

    /// Transform a point for brick SDF sampling.
    /// Applies: translate by T1, rotate by R, translate by T2
    /// For SDFs, applies inverse transformations in reverse order: T2^-1, R^-1, T1^-1
    fn transform_brick_point(&self, p: Vector3<f32>, z_angle: f32, x_angle: f32) -> Vector3<f32> {
        let p = rotate_z(p, z_angle);
        // Step 3 inverse: translate by -T2
        let p = translate(p, Vector3::new(0.0, self.root_r, -self.root_z));
        // Step 2 inverse: rotate by -R
        let p = rotate_x(p, -x_angle);
        // Step 1 inverse: translate by -T1
        translate(p, Vector3::new(0.0, -self.wall_r, 0.0))
    }

    /// Calculate brick SDF at transformed point.
    fn brick_sdf(&self, p: Vector3<f32>) -> f32 {
        self.brick_sdf_smaller(p, self.brick_width)
    }

    fn brick_sdf_smaller(&self, p: Vector3<f32>, brick_width: f32) -> f32 {
        (p.x.abs() - brick_width).max(p.z.abs() - self.brick_height).max(p.y.abs() - self.thickness)
    }

    fn transform_door_brick_point(&self, p: Vector3<f32>, x_angle: f32, x_shift: f32) -> Vector3<f32> {
        // Step 3 inverse: translate by -T2
        let p = translate(p, Vector3::new(
            self.door_length - self.brick_width - x_shift,
             self.door_root_r,
             -self.door_root_z - self.door_slope * (self.door_length - self.brick_width - x_shift)
        ));
        // Step 2 inverse: rotate by -R
        let p = rotate_x(p, -x_angle);
        // Step 1 inverse: translate by -T1
        translate(p, Vector3::new(0.0, -self.door_wall_r, 0.0))
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
            Vector3::new(self.door_length + BOUNDARY, r + BOUNDARY,  self.top + BOUNDARY),
        ]
    }

    /// Sample the SDF at point p
    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x;
        let y = p.y;
        let z = p.z;

        let xy_from_root = (x * x + y * y).sqrt() + self.root_r;
        let z_from_root = z + self.root_z;
        let distance_from_root = (xy_from_root * xy_from_root + z_from_root * z_from_root).sqrt();

        let sdf = match self.material {
             Material::Straw => {
                let mut brick = std::f32::INFINITY;
                for row in self.brick_rows.iter() {
                    for i in 0..row.count + row.odd as usize {
                        let z_angle = PI / 2.0 - self.door_angle - self.half_brick_angle - row.step * i as f32;

                        // Regular brick
                        let p = self.transform_brick_point(Vector3::new(x, y, z), z_angle, row.angle);
                        brick = brick.min(self.brick_sdf(p));

                        if i < row.count {
                            // Brick mirrored by x-z plane
                            let p = self.transform_brick_point(Vector3::new(x, -y, z), z_angle, row.angle);
                            brick = brick.min(self.brick_sdf(p));
                        }
                    }
                }

                for door_brick in self.door_bricks.iter() {
                    let p = self.transform_door_brick_point(Vector3::new(x, y, z), door_brick.angle, door_brick.shift);
                    brick = brick.min(self.brick_sdf_smaller(p, door_brick.brick_width));
                    let p = self.transform_door_brick_point(Vector3::new(x, -y, z), door_brick.angle, door_brick.shift);
                    brick = brick.min(self.brick_sdf_smaller(p, door_brick.brick_width));
                }

                brick
            },

            Material::Wood => {
                unimplemented!()
            },

            Material::Clay => {
                unimplemented!()
            },

            Material::Theory => {
                let xy_angle = y.atan2(x);
        
                let sdf = distance_from_root - self.wall_r - self.thickness;
                let sdf = sdf.max(self.door_angle - xy_angle.abs());
        
                let door_z_from_root = z + self.door_root_z + self.door_slope * x;
                let door_y_from_root = -y.abs() - self.door_root_r;
                let door_distance_from_root = (door_y_from_root * door_y_from_root + door_z_from_root * door_z_from_root).sqrt();
                let door_sdf = door_distance_from_root - self.door_wall_r - self.thickness;
                let door_sdf = door_sdf.max(-(door_distance_from_root - self.door_wall_r + self.thickness));
                let door_sdf = door_sdf.max(-x).max(x - self.door_length);
                let sdf = sdf.min(door_sdf);
                sdf.max(-(distance_from_root - self.wall_r + self.thickness)).max(-z)
            },
        };

        SDFSample::new(
            sdf,
            Vector3::new((distance_from_root / self.wall_r / 1.5).powf(3.0), 0.0, 0.0), // Color (RGB)
        )
    }
}