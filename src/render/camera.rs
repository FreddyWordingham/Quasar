//! Observation camera.

use crate::geom::Orientation;

/// Tracer emission.
pub struct Camera {
    /// Orientation.
    pub orient: Orientation,
    /// Resolution.
    pub res: [usize; 2],
    /// Super sampling power.
    pub ss_power: usize,
    /// Rotation delta.
    half_delta_theta: f64,
}
