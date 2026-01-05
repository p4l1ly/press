// Common utilities and helpers for all press objects

pub use cgmath::{Vector3, Matrix3, Rad, InnerSpace, Matrix};

/// Macro to create computation structs with lazy evaluated fields
#[macro_export]
macro_rules! create_computation {
    ($cfg:ty, $( $field:ident: $type:ty => $calc:expr ),* $(,)?) => {
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

/// 3D transformation utilities
///
/// These functions transform points for sampling.
/// To translate/rotate a shape, apply the inverse transformation to the sample point.

/// Translate a point.
///
/// To translate a shape by `offset`, sample at `translate(p, offset)`.
///
/// # Example
/// ```rust
/// // Sample a sphere translated by (1.0, 2.0, 3.0)
/// let translated_p = translate(p, Vector3::new(1.0, 2.0, 3.0));
/// let distance = sphere(translated_p);
/// ```
pub fn translate(p: Vector3<f32>, offset: Vector3<f32>) -> Vector3<f32> {
    p - offset
}

/// Rotate a point around an axis.
///
/// To rotate a shape around `axis` by `angle` radians, sample at `rotate_axis(p, axis, angle)`.
///
/// # Arguments
/// * `p` - Point to transform
/// * `axis` - Rotation axis (will be normalized)
/// * `angle` - Rotation angle in radians
///
/// # Example
/// ```rust
/// // Rotate 90 degrees around Z-axis
/// let rotated_p = rotate_axis(p, Vector3::new(0.0, 0.0, 1.0), std::f32::consts::PI / 2.0);
/// let distance = shape(rotated_p);
/// ```
pub fn rotate_axis(p: Vector3<f32>, axis: Vector3<f32>, angle: f32) -> Vector3<f32> {
    let axis = axis.normalize();
    let cos_a = (-angle).cos(); // Inverse rotation: negate angle
    let sin_a = (-angle).sin();
    let one_minus_cos = 1.0 - cos_a;

    // Rodrigues' rotation formula (inverse rotation)
    let cross = axis.cross(p);
    let dot = axis.dot(p);

    p * cos_a + cross * sin_a + axis * (dot * one_minus_cos)
}

/// Rotate a point using Euler angles (ZYX order).
///
/// To rotate a shape by Euler angles, sample at `rotate_euler(p, pitch, yaw, roll)`.
///
/// # Arguments
/// * `p` - Point to transform
/// * `pitch` - Rotation around X-axis in radians
/// * `yaw` - Rotation around Y-axis in radians
/// * `roll` - Rotation around Z-axis in radians
///
/// # Example
/// ```rust
/// // Rotate 45 degrees around Y-axis
/// let rotated_p = rotate_euler(p, 0.0, std::f32::consts::PI / 4.0, 0.0);
/// let distance = shape(rotated_p);
/// ```
pub fn rotate_euler(p: Vector3<f32>, pitch: f32, yaw: f32, roll: f32) -> Vector3<f32> {
    // Build rotation matrix (inverse rotation: negate angles)
    let (sp, cp) = (-pitch).sin_cos();
    let (sy, cy) = (-yaw).sin_cos();
    let (sr, cr) = (-roll).sin_cos();

    // Rotation matrix (ZYX order, inverse)
    let m = Matrix3::new(
        cy * cr, -cy * sr, sy,
        sp * sy * cr + cp * sr, -sp * sy * sr + cp * cr, -sp * cy,
        -cp * sy * cr + sp * sr, cp * sy * sr + sp * cr, cp * cy,
    );

    m * p
}

/// Rotate a point using a rotation matrix.
///
/// To rotate a shape by rotation matrix `m`, sample at `rotate_matrix(p, m)`.
/// This applies the transpose (inverse) of the rotation matrix.
///
/// # Example
/// ```rust
/// let rotation = Matrix3::from_angle_z(Rad(std::f32::consts::PI / 4.0));
/// let rotated_p = rotate_matrix(p, rotation);
/// let distance = shape(rotated_p);
/// ```
pub fn rotate_matrix(p: Vector3<f32>, rotation: Matrix3<f32>) -> Vector3<f32> {
    rotation.transpose() * p
}

/// Rotate a point around the X-axis.
pub fn rotate_x(p: Vector3<f32>, angle: f32) -> Vector3<f32> {
    rotate_axis(p, Vector3::new(1.0, 0.0, 0.0), angle)
}

/// Rotate a point around the Y-axis.
pub fn rotate_y(p: Vector3<f32>, angle: f32) -> Vector3<f32> {
    rotate_axis(p, Vector3::new(0.0, 1.0, 0.0), angle)
}

/// Rotate a point around the Z-axis.
pub fn rotate_z(p: Vector3<f32>, angle: f32) -> Vector3<f32> {
    rotate_axis(p, Vector3::new(0.0, 0.0, 1.0), angle)
}

/// Calculate the signed distance to a cylinder between two points.
///
/// Returns the SDF value for a cylinder with given `radius` extending from point `a` to point `b`.
///
/// # Arguments
/// * `p` - Sample point
/// * `a` - First endpoint of the cylinder
/// * `b` - Second endpoint of the cylinder
/// * `radius` - Radius of the cylinder
///
/// # Example
/// ```rust
/// let a = Vector3::new(0.0, 0.0, 0.0);
/// let b = Vector3::new(0.0, 0.0, 5.0);
/// let distance = cylinder_between(p, a, b, 0.5);
/// ```
pub fn cylinder_between(p: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>, radius: f32) -> f32 {
    let ab = b - a;
    let ap = p - a;

    // Project ap onto ab
    let ab_len_sq = ab.dot(ab);
    if ab_len_sq < 1e-6 {
        // Degenerate case: a and b are the same point, treat as sphere
        return (p - a).magnitude() - radius;
    }

    let t = (ap.dot(ab) / ab_len_sq).max(0.0).min(1.0);

    // Closest point on the line segment
    let q = a + ab * t;

    // Distance from p to the line segment, minus radius
    (p - q).magnitude() - radius
}
