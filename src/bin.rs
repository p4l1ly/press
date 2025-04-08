use press::{Settings,needle_straight};

pub fn main() {
    let cfg = Settings::default();
    println!(
        "Hello, world! {} {} {} {}",
        cfg.derived.outer_holder_xmax,
        cfg.derived.outer_holder_xmin,
        cfg.derived.outer_holder_xmax - cfg.derived.outer_holder_xmin - 19.0,
        2.0 * cfg.given.needle_distance_z - 2.0 * cfg.given.thickness
    );

    needle_straight(&cfg, 0.0, 0.0);
}

