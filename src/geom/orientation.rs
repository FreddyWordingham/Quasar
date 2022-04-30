//! Orientation.

use nalgebra::{Point3, Unit, Vector3};

/// Orientation.
pub struct Orientation {
    /// Position.
    pos: Point3<f64>,
    /// Forward direction.
    forward: Unit<Vector3<f64>>,
    /// Right direction.
    right: Unit<Vector3<f64>>,
    /// Up direction.
    up: Unit<Vector3<f64>>,
}
