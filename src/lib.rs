use std::f32::consts::PI;

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

const NEEDLETOP_SLOPE: f64 = 0.35;
const NEEDLETOP_WIDTH: f64 = 8.0;
const HILL_SLOPE: f64 = -0.35;
const HILL_XSHIFT: f64 = -10.0;
const HILL_PERIOD: f64 = 35.0;
const HILL_HEIGHT: f64 = 12.0;
const HILL_HEIGHT_EDGE: f64 = 2.0;
const HILL_HEIGHT_MIDDLE: f64 = 12.0;
const HILL_MIDDLE_POS: f64 = 0.6;
const THICKNESS: f64 = 12.0;
const STEEL_THICKNESS: f64 = 1.5;

impl SDFSurface for InnerHolder {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(0.0, 0.0, -(STEEL_THICKNESS + THICKNESS) as f32),
            Vector3::new(85.0, (HILL_HEIGHT + NEEDLETOP_WIDTH) as f32, (STEEL_THICKNESS + THICKNESS) as f32)
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let xgon = (p.x as f64 - HILL_XSHIFT) * 2.0 * PI as f64 / HILL_PERIOD;
        let xcos = xgon.cos();
        let wave_y = xcos - HILL_SLOPE * (xcos * xcos - 1.0);

        let peak = HILL_HEIGHT * (wave_y + 1.0) / 2.0;
        let inner_holder =
            if (-STEEL_THICKNESS as f32) < p.z && p.z < (STEEL_THICKNESS as f32) {
                (peak - p.y as f64).max(p.y as f64 - HILL_HEIGHT - NEEDLETOP_WIDTH)
            } else {
                let middle = (HILL_HEIGHT_MIDDLE * wave_y + 2.0 * HILL_HEIGHT - HILL_HEIGHT_MIDDLE) / 2.0;
                let edge = (HILL_HEIGHT_EDGE * wave_y + 2.0 * HILL_HEIGHT - HILL_HEIGHT_EDGE) / 2.0;
                let control_points = [
                    Point { x: -0.1, y: peak },
                    Point { x: THICKNESS * HILL_MIDDLE_POS, y: middle },
                    Point { x: THICKNESS, y: edge },
                ];
                let mut poly = bezier_curve_points(&control_points, 8);
                poly.push(Point { x: THICKNESS, y: HILL_HEIGHT + NEEDLETOP_WIDTH });
                poly.push(Point { x: 0.0, y: HILL_HEIGHT + NEEDLETOP_WIDTH });
                sdf_polygon(&Point { x: p.z.abs() as f64 - STEEL_THICKNESS, y: p.y as f64 }, &poly)
            };

        let needletop = HILL_HEIGHT * (xcos - NEEDLETOP_SLOPE * (xcos * xcos - 1.0) + 1.0) / 2.0 + NEEDLETOP_WIDTH;
        let inner_needle = (peak - 0.1 - p.y as f64).max(p.y as f64 - needletop);


        SDFSample::new(
            (inner_holder as f32)
                .max(-(STEEL_THICKNESS + THICKNESS) as f32 - p.z)
                .max(p.z - (STEEL_THICKNESS + THICKNESS) as f32)
                .max(
                    -(inner_needle as f32)
                        .max(-STEEL_THICKNESS as f32 - p.z)
                        .max(p.z - STEEL_THICKNESS as f32),
                ),
            Vector3::new(1.0, 1.0, 0.0),
        )
    }
}
