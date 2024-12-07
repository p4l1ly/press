use std::{f32::consts::PI, str::FromStr};

use bezier::bezier_curve_points;
use cgmath::Vector3;
use polygon::{sdf_polygon, Point};
use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};
use clap;

mod polygon;
mod bezier;

#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(InnerHolder { cfg: Settings::default() }));
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
struct Settings {
    first_needle_x: f64,
    needle_distance: f64,
    needle_count: usize,
    needletop_slope: f64,
    needletop_width: f64,
    hill_slope: f64,
    hill_xshift: f64,
    hill_height: f64,
    hill_height_edge: f64,
    hill_height_middle: f64,
    hill_middle_pos: f64,
    thickness: f64,
    steel_thickness: f64,

    needle_ys: Vec<f64>,
    needle_xs1: Vec<f64>,
    needle_xs2: Vec<f64>,

    needle_distance_z: f64,
    needle_distance_x_diag: f64,
    inner_needle_x: f64,
}

impl FromStr for Settings {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut settings = Settings::default();
        for line in s.lines() {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap();
            let value = parts.next().ok_or_else(|| format!("No value for key {}", key))?;
            match key {
                "first_needle_x" => settings.first_needle_x = value.parse().map_err(|e| format!("{}", e))?,
                "needle_distance" => settings.needle_distance = value.parse().map_err(|e| format!("{}", e))?,
                "needle_count" => settings.needle_count = value.parse().map_err(|e| format!("{}", e))?,
                "needletop_slope" => settings.needletop_slope = value.parse().map_err(|e| format!("{}", e))?,
                "needletop_width" => settings.needletop_width = value.parse().map_err(|e| format!("{}", e))?,
                "hill_slope" => settings.hill_slope = value.parse().map_err(|e| format!("{}", e))?,
                "hill_xshift" => settings.hill_xshift = value.parse().map_err(|e| format!("{}", e))?,
                "hill_height" => settings.hill_height = value.parse().map_err(|e| format!("{}", e))?,
                "hill_height_edge" => settings.hill_height_edge = value.parse().map_err(|e| format!("{}", e))?,
                "hill_height_middle" => settings.hill_height_middle = value.parse().map_err(|e| format!("{}", e))?,
                "hill_middle_pos" => settings.hill_middle_pos = value.parse().map_err(|e| format!("{}", e))?,
                "thickness" => settings.thickness = value.parse().map_err(|e| format!("{}", e))?,
                "steel_thickness" => settings.steel_thickness = value.parse().map_err(|e| format!("{}", e))?,
                "needle_ys" => settings.needle_ys = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                "needle_xs1" => settings.needle_xs1 = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                "needle_xs2" => settings.needle_xs2 = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                _ => return Err(format!("Unknown key {}", key)),
            }
            settings.postprocess();
        }
        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        let first_needle_x = 3.0; // 153.0;
        let needle_distance = 50.0;
        let needle_count = 4;
        let needletop_slope = 0.35;
        let needletop_width = 8.0;
        let hill_slope = -0.35;
        let hill_xshift = 10.0;
        let hill_height = 12.0;
        let hill_height_edge = 2.0;
        let hill_height_middle = 12.0;
        let hill_middle_pos = 0.6;
        let thickness = 12.0;
        let steel_thickness = 1.5;

        let needle_ys = vec![
            183.0 / 215.0 * 10.0,
            183.0 / 215.0 * 16.5,
            183.0 / 215.0 * 22.0,
            183.0 / 215.0 * 31.0,
            183.0 / 215.0 * 36.5,
            183.0 / 215.0 * 200.0,
        ];
        let needle_xs1 = vec![
            183.0 / 215.0 * 2.0,
            183.0 / 215.0 * 6.0,
            183.0 / 215.0 * 9.0,
            183.0 / 215.0 * 11.0,
            183.0 / 215.0 * 13.0,
            183.0 / 215.0 * 14.5,
        ];
        let needle_xs2 = vec![
            183.0 / 215.0 * 4.5,
            183.0 / 215.0 * 7.5,
            183.0 / 215.0 * 9.5,
            183.0 / 215.0 * 11.5,
            183.0 / 215.0 * 13.0,
            183.0 / 215.0 * 14.5,
        ];

        let mut result = Self {
            first_needle_x,
            needle_distance,
            needle_count,
            needletop_slope,
            needletop_width,
            hill_slope,
            hill_xshift,
            hill_height,
            hill_height_edge,
            hill_height_middle,
            hill_middle_pos,
            thickness,
            steel_thickness,

            needle_ys,
            needle_xs1,
            needle_xs2,

            needle_distance_z: 0.0,
            needle_distance_x_diag: 0.0,
            inner_needle_x: 0.0,
        };

        result.postprocess();

        result
    }
}

impl Settings {
    fn postprocess(&mut self) {
        self.needle_distance_z = self.needle_distance * (PI as f64 / 3.0).sin();
        self.needle_distance_x_diag = self.needle_distance * (PI as f64 / 3.0).cos();
        self.inner_needle_x = self.first_needle_x + self.needle_distance_x_diag;
    }
}

macro_rules! create_computation {
    ($cfg:ty, $( $field:ident: $type:ty => $calc:expr ),* ) => {
        struct Computation<'a> {
            cfg: &'a $cfg,
            x: f64,
            y: f64,
            z: f64,
            $( $field: once_cell::unsync::OnceCell<$type>, )*
        }

        impl<'a> Computation<'a> {
            fn new(cfg: &'a $cfg, x: f64, y: f64, z: f64) -> Self {
                Self {
                    cfg,
                    x,
                    y,
                    z,
                    $( $field: once_cell::unsync::OnceCell::new(), )*
                }
            }

            $(
                fn $field(&self) -> $type {
                    *self.$field.get_or_init(|| $calc(self))
                }
            )*
        }
    };
}

create_computation! {
    Settings,
    xcos: f64 => |slf: &Computation|
        -(slf.x * 2.0 * std::f64::consts::PI / slf.cfg.needle_distance).cos(),
    holder_wave: f64 => |slf: &Computation| {
        let xcos = slf.xcos();
        xcos - slf.cfg.hill_slope * (xcos * xcos - 1.0)
    },
    needletop: f64 => |slf: &Computation| {
        let xcos = slf.xcos();
        slf.cfg.hill_height
            * (xcos - slf.cfg.needletop_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
            + slf.cfg.needletop_width
    },
    peak_holder_wave: f64 => |slf: &Computation|
        slf.cfg.hill_height * (slf.holder_wave() + 1.0) / 2.0,
    needle_handle: f64 => |slf: &Computation| {
        (slf.peak_holder_wave() - 0.1 - slf.y).max(slf.y - slf.needletop())
    }
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct InnerHolder {
    cfg: Settings,
}

impl SDFSurface for InnerHolder {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(
                -self.cfg.hill_xshift as f32,
                0.0,
                (self.cfg.steel_thickness - self.cfg.thickness)
                    as f32,
            ),
            Vector3::new(
                (
                    self.cfg.needle_distance * (self.cfg.needle_count - 2) as f64
                    + self.cfg.hill_xshift
                ) as f32,
                (self.cfg.hill_height + self.cfg.needletop_width) as f32,
                (self.cfg.steel_thickness + self.cfg.thickness)
                    as f32,
            ),
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x as f64;
        let y = p.y as f64;
        let z = p.z as f64;
        let comp = Computation::new(&self.cfg, x, y, z);

        let inner_holder =
            if -self.cfg.steel_thickness < z && z < self.cfg.steel_thickness {
                (comp.peak_holder_wave() - y)
                    .max(y - self.cfg.hill_height - self.cfg.needletop_width)
            } else {
                let middle = (
                    self.cfg.hill_height_middle * comp.holder_wave()
                    + 2.0 * self.cfg.hill_height - self.cfg.hill_height_middle
                ) / 2.0;
                let edge = (
                    self.cfg.hill_height_edge * comp.holder_wave()
                    + 2.0 * self.cfg.hill_height - self.cfg.hill_height_edge
                ) / 2.0;
                let control_points = [
                    Point { x: -0.1, y: comp.peak_holder_wave() },
                    Point { x: self.cfg.thickness * self.cfg.hill_middle_pos, y: middle },
                    Point { x: self.cfg.thickness, y: edge },
                ];
                let mut poly = bezier_curve_points(&control_points, 8);
                poly.push(Point {
                    x: self.cfg.thickness,
                    y: self.cfg.hill_height + self.cfg.needletop_width
                });
                poly.push(Point { x: 0.0, y: self.cfg.hill_height + self.cfg.needletop_width });
                sdf_polygon(
                    &Point { x: z.abs() - self.cfg.steel_thickness, y },
                    &poly
                )
            };

        SDFSample::new(
            inner_holder
                .max(-self.cfg.steel_thickness - self.cfg.thickness - z)
                .max(z - self.cfg.steel_thickness - self.cfg.thickness)
                .max(
                    -comp.needle_handle()
                        .max(-self.cfg.steel_thickness - z)
                        .max(z - self.cfg.steel_thickness),
                ) as f32,
            Vector3::new(1.0, 1.0, 0.0),
        )
    }
}

fn trapezoid(x: f64, y: f64, w1: f64, w2: f64, h: f64) -> f64 {
    let k = (w1 + y * (w2 - w1) / h) / 2.0;
    (x - k).max(-k - x).max(-y).max(y - h)
}

fn circ_coordinates(x: f64, y: f64, a: f64) -> (f64, f64) {
    let x2 = x * a.cos() - y * a.sin();
    let y2 = x * a.sin() + y * a.cos();
    let x3 = (x2*x2 + y2*y2).sqrt();
    let y3 = x3 * y2.atan2(x2);
    (x3, y3)
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct InnerNeedle {
    cfg: Settings,
}

impl InnerNeedle {
    fn needle_straight(&self, x: f64, y: f64) -> f64 {
        let mut d = trapezoid(x, y, self.cfg.needle_xs1[0], self.cfg.needle_xs2[0], self.cfg.needle_ys[0]);
        for i in 1..self.cfg.needle_ys.len() {
            let x1 = if i & 1 == 0 { x + (self.cfg.needle_xs1[i] - self.cfg.needle_xs2[i-1])/2.0 } else { x };
            d = d.min(trapezoid(
                x1, y - self.cfg.needle_ys[i-1],
                self.cfg.needle_xs1[i], self.cfg.needle_xs2[i],
                self.cfg.needle_ys[i] - self.cfg.needle_ys[i-1],
            ));
        }
        d
    }
}

impl SDFSurface for InnerNeedle {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(-14.5, -10.0, -self.cfg.steel_thickness as f32),
            Vector3::new(14.5, 200.0, self.cfg.steel_thickness as f32),
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let (x, y) = circ_coordinates(p.x as f64 + 215.0 - 6.5, p.y as f64, 0.03);
        SDFSample::new(
            self.needle_straight(x + 6.5 - 215.0, y) as f32,
            Vector3::new(1.0, 0.0, 1.0),
        )
    }
}
