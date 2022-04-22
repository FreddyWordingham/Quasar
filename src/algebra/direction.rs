//! Unit vectors.

use nalgebra::{Unit, Vector2, Vector3, Vector4};

/// Normalised two-dimensional vector.
pub type Dir2 = Unit<Vector2<f64>>;

/// Normalised three-dimensional vector.
pub type Dir3 = Unit<Vector3<f64>>;

/// Normalised four-dimensional vector.
pub type Dir4 = Unit<Vector4<f64>>;
