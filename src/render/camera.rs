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

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(orient: Orientation, fov: f64, res: [usize; 2], ss_power: usize) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[0] > 0);
        debug_assert!(res[1] > 0);
        debug_assert!(ss_power > 0);

        let half_delta_theta = fov / ((2 * (ss_power * (res[0] - 1))) as f64);

        Self {
            orient,
            half_delta_theta,
            res,
            ss_power,
        }
    }
}
