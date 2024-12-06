#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn point_to_segment_distance(p: &Point, a: &Point, b: &Point) -> f64 {
    let v = Point {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let w = Point {
        x: p.x - a.x,
        y: p.y - a.y,
    };
    let c1 = w.x * v.x + w.y * v.y;
    let c2 = v.x * v.x + v.y * v.y;
    let t = if c2 != 0.0 { c1 / c2 } else { 0.0 };
    let t = t.max(0.0).min(1.0);
    let projection = Point {
        x: a.x + t * v.x,
        y: a.y + t * v.y,
    };
    p.distance_to(&projection)
}

fn is_outside(point: &Point, vertices: &[Point]) -> bool {
    let mut count = 0;
    let mut j = vertices.len() - 1;
    for i in 0..vertices.len() {
        let pi = &vertices[i];
        let pj = &vertices[j];
        if (pi.y > point.y) != (pj.y > point.y) {
            let x_intercept = pi.x + (point.y - pi.y) * (pj.x - pi.x) / (pj.y - pi.y);
            if point.x < x_intercept {
                count += 1;
            }
        }
        j = i;
    }
    count % 2 == 0
}

pub fn sdf_polygon(p: &Point, vertices: &[Point]) -> f64 {
    let mut min_distance = f64::INFINITY;
    let n = vertices.len();
    for i in 0..n {
        let j = if i == n - 1 { 0 } else { i + 1 };
        let distance = point_to_segment_distance(p, &vertices[i], &vertices[j]);
        if distance < min_distance {
            min_distance = distance;
        }
    }
    if is_outside(p, vertices) {
        min_distance
    } else {
        -min_distance
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_point_to_segment_distance() {
        let p = Point { x: 0.0, y: 0.0 };
        let a = Point { x: 1.0, y: -1.0 };
        let b = Point { x: 1.0, y: 2.0 };
        assert_eq!(point_to_segment_distance(&p, &a, &b), 1.0);
    }

    #[test]
    fn test_is_outside() {
        let vertices = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 2.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 2.0, y: 1.0 },
        ];
        let point = Point { x: 0.0, y: 0.0 };
        assert!(is_outside(&point, &vertices));
        let point = Point { x: 1.5, y: 1.5 };
        assert!(!is_outside(&point, &vertices));
    }

    #[test]
    fn test_sdf_polygon() {
        let vertices = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 2.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 2.0, y: 1.0 },
        ];
        let point = Point { x: 0.0, y: 0.0 };
        assert!((1.41 - sdf_polygon(&point, &vertices)).abs() < 0.005);
        let point = Point { x: 1.5, y: 1.5 };
        assert!((-0.5 - sdf_polygon(&point, &vertices)).abs() < 0.05);
    }
}
