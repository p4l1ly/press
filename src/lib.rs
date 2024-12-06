use bezier::bezier_curve_points;
use cgmath::{Vector3, Zero};
use polygon::{sdf_polygon, Point};
use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};
use clap;

mod polygon;
mod bezier;

#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(InnerHolder));
}

#[derive(clap::Parser, Debug, Clone, PartialEq, Eq)]
pub struct InnerHolder;

impl SDFSurface for InnerHolder {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [Vector3::new(-1.0, 0.0, 0.0), Vector3::new(1.0, 3.0, 1.0)]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let control_points = [
            Point { x: -1.0, y: 0.0 },
            Point { x: 0.0, y: 3.0 },
            Point { x: 1.0, y: 0.0 },
        ];
        let poly = bezier_curve_points(&control_points, 10);
        let distance = sdf_polygon(&Point { x: p.x as f64, y: p.y as f64 }, &poly);
        SDFSample::new(distance as f32, Vector3::zero())
    }
}
