//! Camera builder.

use nalgebra::Vector3;
use serde::Deserialize;

/// Camera configuration.
#[derive(Deserialize)]
pub struct CameraBuilder {
    /// Position.
    pos: Vector3<f64>,
    /// Target.
    tar: Vector3<f64>,
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Optional super-sampling power.
    ss_power: Option<usize>,
}
