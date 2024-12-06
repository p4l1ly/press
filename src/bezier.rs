use crate::polygon::Point;

fn interpolate(p0: Point, p1: Point, t: f64) -> Point {
    Point {
        x: (1.0 - t) * p0.x + t * p1.x,
        y: (1.0 - t) * p0.y + t * p1.y,
    }
}

fn de_casteljau(control_points: &[Point], t: f64) -> Point {
    if control_points.len() == 1 {
        control_points[0]
    } else {
        // Reduce the control points array size by one
        // by interpolating between each consecutive pair
        let mut reduced_points = Vec::with_capacity(control_points.len() - 1);
        for i in 0..control_points.len() - 1 {
            reduced_points.push(interpolate(control_points[i], control_points[i + 1], t));
        }
        // Recursive call
        de_casteljau(&reduced_points, t)
    }
}

pub fn bezier_curve_points(control_points: &[Point], num_segments: usize) -> Vec<Point> {
    let mut points = Vec::new();
    for i in 0..=num_segments {
        let t = i as f64 / num_segments as f64;
        points.push(de_casteljau(control_points, t));
    }
    points
}
