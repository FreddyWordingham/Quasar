//! Orientation.

use nalgebra::{Point3, Unit, Vector3};

use crate::geom::Ray;

/// Orientation.
pub struct Orientation {
    /// Position.
    pub pos: Point3<f64>,
    /// Forward direction.
    pub forward: Unit<Vector3<f64>>,
    /// Right direction.
    pub right: Unit<Vector3<f64>>,
    /// Up direction.
    pub up: Unit<Vector3<f64>>,
}

impl Orientation {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        let (pos, forward) = ray.destruct();
        let right = if forward.z.abs() <= 0.9 {
            Unit::new_normalize(forward.cross(&Vector3::z_axis())) // Universal up is z-axis.
        } else {
            Unit::new_normalize(forward.cross(&Vector3::x_axis())) // If facing along z-axis, compute relative up using x-axis.
        };
        let up = Unit::new_normalize(right.cross(&forward));

        Self {
            pos,
            forward,
            right,
            up,
        }
    }

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new_tar(pos: Point3<f64>, tar: &Point3<f64>) -> Self {
        Self::new(Ray::new(pos, Unit::new_normalize(tar - pos)))
    }
}
