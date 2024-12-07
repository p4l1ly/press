use std::{f32::consts::PI, str::FromStr};

use cgmath::Vector3;
use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};
use clap;

#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(InnerHolder { cfg: Settings::default() }));
}

#[derive(Debug, Clone, PartialEq)]
struct GivenSettings {
    first_needle_x: f64,
    needle_distance: f64,
    needle_count: usize,
    needletop_slope: f64,
    needletop_width: f64,
    outer_needletop_width: f64,
    hill_slope: f64,
    hill_slope_z: f64,
    hill_xshift: f64,
    hill_height: f64,
    hill_middle_pos: f64,
    thickness: f64,
    steel_thickness: f64,
    penetration_angle: f64,

    needle_ys: Vec<f64>,
    needle_xs1: Vec<f64>,
    needle_xs2: Vec<f64>,
    needle_cut_y: f64,
    needle_cut_r: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct DerivedSettings {
    needle_distance_z: f64,
    needle_distance_x_diag: f64,
    inner_needle_x: f64,
    inner_holder_xmax: f64,
    outer_holder_xmax: f64,
    outer_holder_xmin: f64,
    needle_width: f64,
    needle_halfwidth: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Settings {
    given: GivenSettings,
    derived: DerivedSettings,
}

impl FromStr for Settings {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut settings = GivenSettings::default();
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
                "hill_middle_pos" => settings.hill_middle_pos = value.parse().map_err(|e| format!("{}", e))?,
                "thickness" => settings.thickness = value.parse().map_err(|e| format!("{}", e))?,
                "steel_thickness" => settings.steel_thickness = value.parse().map_err(|e| format!("{}", e))?,
                "needle_ys" => settings.needle_ys = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                "needle_xs1" => settings.needle_xs1 = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                "needle_xs2" => settings.needle_xs2 = value.split(',').map(|s| s.parse().map_err(|e| format!("{}", e))).collect::<Result<_, _>>()?,
                _ => return Err(format!("Unknown key {}", key)),
            }
        }
        Ok(Settings {
            derived: DerivedSettings::new(&settings),
            given: settings,
        })
    }
}

impl Default for GivenSettings {
    fn default() -> Self {
        Self {
            first_needle_x: 153.0,
            needle_distance: 50.0,
            needle_count: 4,
            needletop_slope: 0.35,
            needletop_width: 12.0,
            outer_needletop_width: 16.0,
            hill_slope: -0.35,
            hill_slope_z: -0.40,
            hill_xshift: 10.0,
            hill_height: 12.0,
            hill_middle_pos: 0.6,
            thickness: 12.0,
            steel_thickness: 1.5,
            penetration_angle: 0.4,

            needle_ys: vec![
                183.0 / 215.0 * 10.0,
                183.0 / 215.0 * 16.5,
                183.0 / 215.0 * 22.0,
                183.0 / 215.0 * 31.0,
                183.0 / 215.0 * 36.5,
                183.0 / 215.0 * 200.0,
            ],
            needle_xs1: vec![
                183.0 / 215.0 * 2.0,
                183.0 / 215.0 * 6.0,
                183.0 / 215.0 * 9.0,
                183.0 / 215.0 * 11.0,
                183.0 / 215.0 * 13.0,
                183.0 / 215.0 * 14.5,
            ],
            needle_xs2: vec![
                183.0 / 215.0 * 4.5,
                183.0 / 215.0 * 7.5,
                183.0 / 215.0 * 9.5,
                183.0 / 215.0 * 11.5,
                183.0 / 215.0 * 13.0,
                183.0 / 215.0 * 14.5,
            ],
            needle_cut_y: 0.6,
            needle_cut_r: 0.8,
        }
    }
}

impl DerivedSettings {
    fn new(given: &GivenSettings) -> Self {
        let needle_distance_x_diag = given.needle_distance * (PI as f64 / 3.0).cos();
        let needle_width = *given.needle_xs2.last().unwrap();
        let needle_halfwidth = needle_width / 2.0;
        let outer_holder_xmin = - needle_distance_x_diag - needle_halfwidth;
        Self {
            needle_distance_z: given.needle_distance * (PI as f64 / 3.0).sin(),
            needle_distance_x_diag,
            inner_needle_x: given.first_needle_x + needle_distance_x_diag,
            inner_holder_xmax:
                given.needle_distance * (given.needle_count - 2) as f64 + given.hill_xshift,
            needle_width,
            needle_halfwidth,
            outer_holder_xmin,
            outer_holder_xmax:
                outer_holder_xmin
                    + given.needle_distance * (given.needle_count - 1) as f64
                    + needle_width,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let given = GivenSettings::default();
        Self {
            derived: DerivedSettings::new(&GivenSettings::default()),
            given,
        }
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
        -(slf.x * 2.0 * std::f64::consts::PI / slf.cfg.given.needle_distance).cos(),
    zcos: f64 => |slf: &Computation|
        -(slf.z * 2.0 * std::f64::consts::PI / slf.cfg.derived.needle_distance_z).cos(),

    needletop: f64 => |slf: &Computation| {
        let xcos = slf.xcos();
        slf.cfg.given.hill_height
            * (xcos - slf.cfg.given.needletop_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
            + slf.cfg.given.needletop_width
    },

    needlebottom: f64 => |slf: &Computation| {
        let xcos = slf.xcos();
        let base = xcos - slf.cfg.given.hill_slope * (xcos * xcos - 1.0);
        slf.cfg.given.hill_height * (base + 1.0) / 2.0
    },

    holderbottom: f64 => |slf: &Computation| {
        let zcos = slf.zcos();
        let base = zcos - slf.cfg.given.hill_slope_z * (zcos * zcos - 1.0);
        let height = slf.cfg.given.hill_height - slf.needlebottom();
        (height * base + 2.0 * slf.cfg.given.hill_height - height) / 2.0
    },

    inner_needle_handle: f64 => |slf: &Computation| {
        (slf.needlebottom() - 0.1 - slf.y).max(slf.y - slf.needletop())
    },

    xcos_outer: f64 => |slf: &Computation| {
        let x = slf.x + slf.cfg.derived.needle_distance_x_diag;
        -(x * 2.0 * std::f64::consts::PI / slf.cfg.given.needle_distance).cos()
    },

    needlebottom_outer: f64 => |slf: &Computation| {
        let xcos = slf.xcos_outer();
        slf.cfg.given.hill_height
            * (xcos - slf.cfg.given.hill_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
    },

    needletop_outer: f64 => |slf: &Computation| {
        let xcos = slf.xcos_outer();
        slf.cfg.given.hill_height
            * (xcos - slf.cfg.given.needletop_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
            + slf.cfg.given.outer_needletop_width
    },

    outer_needle_handle: f64 => |slf: &Computation| {
        (slf.needlebottom_outer() - 0.1 - slf.y).max(slf.y - slf.needletop_outer())
    }
}

fn inner_holder(comp: &Computation) -> f64 {
    (comp.holderbottom() - comp.y)
    .max(comp.y - comp.cfg.given.hill_height - comp.cfg.given.needletop_width)
    .max(-comp.cfg.given.thickness - comp.z)
    .max(comp.z - comp.cfg.given.thickness)
    .max(-comp.x - comp.cfg.given.hill_xshift)
    .max(comp.x - comp.cfg.derived.inner_holder_xmax)
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct InnerHolder {
    cfg: Settings,
}

impl SDFSurface for InnerHolder {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(
                -self.cfg.given.hill_xshift as f32,
                0.0,
                -self.cfg.given.thickness as f32,
            ),
            Vector3::new(
                self.cfg.derived.inner_holder_xmax as f32,
                (self.cfg.given.hill_height + self.cfg.given.needletop_width) as f32,
                self.cfg.given.thickness as f32,
            ),
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x as f64;
        let y = p.y as f64;
        let z = p.z as f64;
        let comp = Computation::new(&self.cfg, x, y, z);

        let inner_holder = 
            inner_holder(&comp)
                .max(
                    -comp.inner_needle_handle()
                        .max(-self.cfg.given.steel_thickness - z)
                        .max(z - self.cfg.given.steel_thickness),
                ) as f32;

        SDFSample::new(
            inner_holder,
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

fn needle_straight(cfg: &Settings, x: f64, y: f64) -> f64 {
    let mut d = trapezoid(x, y, cfg.given.needle_xs1[0], cfg.given.needle_xs2[0], cfg.given.needle_ys[0]);
    for i in 1..cfg.given.needle_ys.len() {
        let x1 = if i & 1 == 0 {
            x + (cfg.given.needle_xs1[i] - cfg.given.needle_xs2[i-1])/2.0
        } else { x };
        d = d.min(trapezoid(
            x1, y - cfg.given.needle_ys[i-1],
            cfg.given.needle_xs1[i], cfg.given.needle_xs2[i],
            cfg.given.needle_ys[i] - cfg.given.needle_ys[i-1],
        ));
    }
    d.max(cfg.given.needle_cut_r.powi(2) - x*x - (y + cfg.given.needle_cut_y).powi(2))
}

fn inner_needles_straight(cfg: &Settings, x: f64, y: f64) -> f64 {
    let mut d = needle_straight(cfg, x, y);
    for i in 1..cfg.given.needle_count - 1 {
        d = d.min(needle_straight(cfg, x - cfg.given.needle_distance * i as f64, y));
    }
    d
}

fn outer_needles_straight(cfg: &Settings, x: f64, y: f64) -> f64 {
    let x2 = x + cfg.derived.needle_distance_x_diag;
    let mut d = needle_straight(cfg, x2, y);
    for i in 1..cfg.given.needle_count {
        d = d.min(needle_straight(cfg, x2 - cfg.given.needle_distance * i as f64, y));
    }
    d
}

fn inner_needle(comp: &Computation) -> f64 {
    let (xc, yc) = circ_coordinates(
        comp.x + comp.cfg.derived.inner_needle_x, comp.y, comp.cfg.given.penetration_angle);
    inner_needles_straight(comp.cfg, xc - comp.cfg.derived.inner_needle_x, yc)
        .max(comp.y - 2.0)
        .min(
            comp.inner_needle_handle()
                .max(-comp.x - comp.cfg.given.hill_xshift)
                .max(comp.x - comp.cfg.derived.inner_holder_xmax)
        )
        .max(-comp.cfg.given.steel_thickness - comp.z)
        .max(comp.z - comp.cfg.given.steel_thickness)
}

fn outer_needle(comp: &Computation) -> f64 {
    let (xc, yc) = circ_coordinates(
        comp.x + comp.cfg.derived.inner_needle_x, comp.y, comp.cfg.given.penetration_angle);
    outer_needles_straight(comp.cfg, xc - comp.cfg.derived.inner_needle_x, yc)
        .max(comp.y - 2.0)
        .min(
            comp.outer_needle_handle()
                .max(-comp.x + comp.cfg.derived.outer_holder_xmin)
                .max(comp.x - comp.cfg.derived.outer_holder_xmax)
                .max(
                    (comp.y - comp.cfg.given.outer_needletop_width)
                        .min(
                            (-comp.x - comp.cfg.derived.needle_distance_x_diag)
                                .max(
                                    comp.x - comp.cfg.derived.outer_holder_xmax
                                        + comp.cfg.derived.needle_halfwidth
                                )
                        )
                )
        )
        // .max(comp.x - comp.cfg.derived.inner_holder_xmax)
        .max(-comp.cfg.given.steel_thickness - comp.cfg.derived.needle_distance_z - comp.z)
        .max(comp.z - comp.cfg.given.steel_thickness + comp.cfg.derived.needle_distance_z)
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct InnerNeedle {
    cfg: Settings,
}

impl SDFSurface for InnerNeedle {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(
                -30.0 - self.cfg.derived.needle_distance_x_diag as f32,
                -120.0,
                (-self.cfg.given.steel_thickness - self.cfg.derived.needle_distance_z) as f32,
            ),
            Vector3::new(
                self.cfg.derived.outer_holder_xmax as f32 + 10.0,
                (self.cfg.given.hill_height + self.cfg.given.outer_needletop_width) as f32,
                self.cfg.given.steel_thickness as f32,
            ),
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x as f64;
        let y = p.y as f64;
        let z = p.z as f64;
        let comp = Computation::new(&self.cfg, x, y, z);

        SDFSample::new(
            inner_needle(&comp)
                .min(outer_needle(&comp))
                as f32,
            Vector3::new(1.0, 0.0, 1.0),
        )
    }
}

#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct OuterNeedle {
    cfg: Settings,
}
