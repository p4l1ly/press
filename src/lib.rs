use std::f64::INFINITY;

use cgmath::Vector3;
use sdf_viewer::sdf::{ffi::set_root_sdf, SDFSample, SDFSurface};

#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(Holder { cfg: Settings::default() }));
}

#[derive(Debug, Clone, PartialEq)]
pub struct GivenSettings {
    pub first_needle_x: f64,
    pub needle_distance: f64,
    pub needle_distance_z: f64,
    pub needle_count: usize,
    pub needle_count_z: usize,
    pub needletop_slope: f64,
    pub needletop_width: f64,
    pub outer_needletop_width: f64,
    pub outer_holder_height: f64,
    pub hill_slope: f64,
    pub hill_slope_z: f64,
    pub hill_xshift: f64,
    pub hill_zshift_outer: f64,
    pub hill_height: f64,
    pub hill_middle_pos: f64,
    pub thickness: f64,
    pub outer_thickness: f64,
    pub steel_thickness: f64,
    pub penetration_angle: f64,
    pub holder_gap: f64,
    pub holder_gap_z: f64,
    pub inner_holder_height: f64,
    pub inner_holder_xmin: f64,

    pub needle_ys: Vec<f64>,
    pub needle_xs1: Vec<f64>,
    pub needle_xs2: Vec<f64>,
    pub needle_cut_y: f64,
    pub needle_cut_r: f64,

    pub show_outer_holder1: bool,
    pub show_outer_holder2: bool,
    pub show_inner_holder: bool,
    pub show_inner_hole: bool,
    pub inner_holder_flat_z: bool,
    pub show_connector: bool,
    pub show_inner_needle: bool,
    pub show_outer_needle: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedSettings {
    pub needle_distance_x_diag: f64,
    pub inner_needle_x: f64,
    pub inner_holder_xmax: f64,
    pub outer_holder_xmax: f64,
    pub outer_holder_xmin: f64,
    pub outer_holder_zmax: f64,
    pub outer_holder_zmin: f64,
    pub needle_width: f64,
    pub needle_halfwidth: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub given: GivenSettings,
    pub derived: DerivedSettings,
}

impl Default for GivenSettings {
    fn default() -> Self {
        Self {
            first_needle_x: 153.0,
            needle_distance: 56.0,
            needle_distance_z: 27.5,
            needle_count: 4,
            needle_count_z: 4,
            needletop_slope: 0.35,
            needletop_width: 12.0,
            outer_needletop_width: 16.0,
            outer_holder_height: 60.0,
            hill_slope: -0.35,
            hill_slope_z: -0.45,
            hill_xshift: 8.0,
            hill_zshift_outer: 30.0,
            hill_height: 12.0,
            hill_middle_pos: 0.6,
            thickness: 18.0,
            outer_thickness: 19.0,
            steel_thickness: 1.5,
            penetration_angle: 0.3,
            holder_gap: 2.0,
            holder_gap_z: 2.0,
            inner_holder_height: 130.0,
            inner_holder_xmin: -40.0,

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

            show_outer_holder1: false,
            show_outer_holder2: false,
            show_inner_holder: true,
            show_inner_hole: true,
            inner_holder_flat_z: false,
            show_connector: false,
            show_inner_needle: false,
            show_outer_needle: false,
        }
    }
}

impl DerivedSettings {
    fn new(given: &GivenSettings) -> Self {
        let needle_distance_x_diag = given.needle_distance / 2.0;
        let needle_width = *given.needle_xs2.last().unwrap();
        let needle_halfwidth = needle_width / 2.0;
        let outer_holder_xmin = - needle_distance_x_diag - needle_halfwidth;
        let outer_holder_zmin = - given.needle_distance_z - given.hill_zshift_outer;
        Self {
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
            outer_holder_zmin,
            outer_holder_zmax:
                - given.needle_distance_z
                    + given.needle_distance_z * 2.0 * (given.needle_count_z - 1) as f64
                    + given.hill_zshift_outer,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let given = GivenSettings::default();
        Self {
            derived: DerivedSettings::new(&given),
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

fn xcos_outer(cfg: &Settings, x: f64) -> f64 {
    let x = x + cfg.derived.needle_distance_x_diag;
    -(x * 2.0 * std::f64::consts::PI / cfg.given.needle_distance).cos()
}

fn needletop_outer(cfg: &Settings, xcos: f64) -> f64 {
    cfg.given.hill_height
        * (xcos - cfg.given.needletop_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
        + cfg.given.outer_needletop_width
}

create_computation! {
    Settings,

    xcos: f64 => |slf: &Computation|
        -(slf.x * 2.0 * std::f64::consts::PI / slf.cfg.given.needle_distance).cos(),
    zcos: f64 => |slf: &Computation|
        -(slf.z * 2.0 * std::f64::consts::PI / slf.cfg.given.needle_distance_z / 2.0).cos(),

    xcos_outer: f64 => |slf: &Computation| { xcos_outer(slf.cfg, slf.x) },

    zcos_outer: f64 => |slf: &Computation| {
        let z = slf.z + slf.cfg.given.needle_distance_z;
        -(z * 2.0 * std::f64::consts::PI / slf.cfg.given.needle_distance_z / 2.0).cos()
    },

    xcos_connect: f64 => |slf: &Computation| {
        let a = slf.cfg.given.penetration_angle;
        xcos_outer(slf.cfg, (slf.x + slf.cfg.given.first_needle_x) * (-a).cos() - slf.y * (-a).sin())
    },

    needletop_connect: f64 => |slf: &Computation| {
        let a = slf.cfg.given.penetration_angle;
        (needletop_outer(slf.cfg, slf.xcos_connect())
            - (slf.x + slf.cfg.given.first_needle_x) * (-a).sin() - slf.y * (-a).cos())
            .max(slf.y - (slf.cfg.given.inner_holder_height))
            .max(-slf.x + slf.cfg.given.inner_holder_xmin)
    },

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

    holderbottom_inner: f64 => |slf: &Computation| {
        let zcos = if slf.cfg.given.inner_holder_flat_z { -1.0 } else { slf.zcos() };
        let base = zcos - slf.cfg.given.hill_slope_z * (zcos * zcos - 1.0);
        let height = slf.cfg.given.hill_height - slf.needlebottom();
        (height * base + 2.0 * slf.cfg.given.hill_height - height) / 2.0
    },

    holderbottom_outer: f64 => |slf: &Computation| {
        let zcos = slf.zcos_outer();
        let base = zcos - slf.cfg.given.hill_slope_z * (zcos * zcos - 1.0);
        let height = slf.cfg.given.hill_height - slf.needlebottom_outer();
        (height * base + 2.0 * slf.cfg.given.hill_height - height) / 2.0
    },

    inner_needle_handle: f64 => |slf: &Computation| {
        (slf.needlebottom() - 0.1 - slf.y).max(slf.y - slf.needletop())
    },

    needlebottom_outer: f64 => |slf: &Computation| {
        let xcos = slf.xcos_outer();
        slf.cfg.given.hill_height
            * (xcos - slf.cfg.given.hill_slope * (xcos * xcos - 1.0) + 1.0) / 2.0
    },

    needletop_outer: f64 => |slf: &Computation| { needletop_outer(slf.cfg, slf.xcos_outer()) },

    outer_needle_handle: f64 => |slf: &Computation| {
        (slf.needlebottom_outer() - 0.1 - slf.y).max(slf.y - slf.needletop_outer())
            .max(
                (slf.y - slf.cfg.given.outer_needletop_width)
                    .min(
                        (-slf.x - slf.cfg.derived.needle_distance_x_diag)
                            .max(
                                slf.x - slf.cfg.derived.outer_holder_xmax
                                    + slf.cfg.derived.needle_halfwidth
                            )
                    )
            )
    }
}

fn inner_holder(comp: &Computation) -> f64 {
    (comp.holderbottom_inner() - comp.y)
    .max(-comp.cfg.given.thickness - comp.z)
    .max(comp.z - comp.cfg.given.thickness)
    .max(comp.y - (comp.cfg.given.inner_holder_height))
    .max(-comp.x + comp.cfg.given.inner_holder_xmin)
}

fn outer_holder(comp: &Computation) -> f64 {
    (comp.holderbottom_outer() - comp.y)
    .max(-comp.z + comp.cfg.derived.outer_holder_zmin)
    .max(comp.z - comp.cfg.derived.outer_holder_zmax)
    .max(
        if comp.cfg.given.show_outer_holder1 {
            let outer_r = comp.cfg.given.first_needle_x
                - comp.cfg.given.hill_xshift - comp.cfg.given.holder_gap;
            let outer_circle =
                ((comp.x + comp.cfg.given.first_needle_x).powi(2) + comp.y.powi(2)).sqrt()
                - outer_r;

            let mut outer_circle_z = INFINITY;
            for i in 0..comp.cfg.given.needle_count_z - 1 {
                let zp = comp.cfg.given.needle_distance_z * i as f64 * 2.0;
                outer_circle_z = outer_circle_z
                    .min(
                        (-comp.z - comp.cfg.given.thickness - comp.cfg.given.holder_gap_z + zp)
                        .max(comp.z - comp.cfg.given.thickness - comp.cfg.given.holder_gap_z - zp)
                        );
            };

            (comp.cfg.derived.outer_holder_xmin - comp.x)
            .max(comp.x - comp.cfg.derived.outer_holder_xmin - comp.cfg.given.outer_thickness)
            .max(comp.x - comp.cfg.derived.outer_holder_xmin - comp.cfg.given.outer_thickness)
            .max(outer_circle.min(-outer_circle_z))
        } else { INFINITY }
        .min(
            if comp.cfg.given.show_outer_holder2 {
                (comp.x - comp.cfg.derived.outer_holder_xmax)
                .max(comp.cfg.derived.outer_holder_xmax - comp.cfg.given.outer_thickness - comp.x)
            } else { INFINITY }
        )
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Holder {
    cfg: Settings,
}

impl SDFSurface for Holder {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(
                -30.0 + self.cfg.derived.outer_holder_xmin as f32 - 5.0,
                -120.0 - 5.0,
                self.cfg.derived.outer_holder_zmin as f32 - 5.0,
            ),
            Vector3::new(
                self.cfg.derived.outer_holder_xmax as f32 + 5.0,
                self.cfg.given.inner_holder_height as f32 + 5.0,
                self.cfg.derived.outer_holder_zmax as f32 + 5.0,
            ),
        ]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let x = p.x as f64;
        let y = p.y as f64;
        let z = p.z as f64;
        let comp = Computation::new(&self.cfg, x, y, z);

        let mut outer_needle_z = INFINITY;
        for i in 0..self.cfg.given.needle_count_z {
            let zp = self.cfg.given.needle_distance_z * i as f64 * 2.0;
            outer_needle_z = outer_needle_z
                .min(
                    (-z - self.cfg.given.steel_thickness - self.cfg.given.needle_distance_z + zp)
                    .max(z - self.cfg.given.steel_thickness + self.cfg.given.needle_distance_z - zp)
                    );
        }

        let inner_r1 = self.cfg.given.first_needle_x - self.cfg.given.hill_xshift;
        let inner_r =
            self.cfg.given.first_needle_x + self.cfg.derived.outer_holder_xmax
            - self.cfg.given.outer_thickness - self.cfg.given.holder_gap;
        let inner_circle =
            (((x + self.cfg.given.first_needle_x).powi(2) + y*y).sqrt() - inner_r)
            .max(inner_r1 - ((x + self.cfg.given.first_needle_x).powi(2) + y*y).sqrt());

        let mut result = INFINITY;

        if self.cfg.given.show_inner_holder {
            let mut inhold = inner_holder(&comp);
            if self.cfg.given.show_inner_hole {
                inhold = inhold.max(
                    -comp.inner_needle_handle()
                        .max(-self.cfg.given.steel_thickness - z)
                        .max(z - self.cfg.given.steel_thickness),
                );
            }
            result = result.min(inhold);
        }


        if self.cfg.given.show_connector {
            result = result.min(
                comp.needletop_connect()
                    .max(self.cfg.given.thickness - z)
                    .max(z - self.cfg.given.needle_distance_z*2.0 + self.cfg.given.thickness)
            );
        }

        result = result.max(inner_circle);
        result = result.min(
            outer_holder(&comp)
                .max(-comp.outer_needle_handle().max(outer_needle_z))
                .max(y - self.cfg.given.outer_holder_height)
        );

        if self.cfg.given.show_inner_needle {
            result = result.min(inner_needle(&comp));
        }
        if self.cfg.given.show_outer_needle {
            result = result.min(outer_needle(&comp));
        }

        SDFSample::new(
            result as f32,
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
    d.max(cfg.given.needle_cut_r - (x*x - (y + cfg.given.needle_cut_y).powi(2)).sqrt())
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
        )
        .max(-comp.cfg.given.steel_thickness - comp.cfg.given.needle_distance_z - comp.z)
        .max(comp.z - comp.cfg.given.steel_thickness + comp.cfg.given.needle_distance_z)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Needle {
    cfg: Settings,
}

impl SDFSurface for Needle {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [
            Vector3::new(
                -30.0 - self.cfg.derived.needle_distance_x_diag as f32,
                -120.0,
                (-self.cfg.given.steel_thickness - self.cfg.given.needle_distance_z) as f32,
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

#[derive(Debug, Clone, PartialEq)]
pub struct OuterNeedle {
    cfg: Settings,
}
