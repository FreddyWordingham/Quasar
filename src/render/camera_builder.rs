//! Camera builder.

use nalgebra::Point3;
use serde::Deserialize;

use crate::{geom::Orientation, render::Camera};

/// Camera configuration.
#[derive(Deserialize)]
pub struct CameraBuilder {
    /// Position.
    pos: Point3<f64>,
    /// Target.
    tar: Point3<f64>,
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Optional super-sampling power.
    ss_power: Option<usize>,
}

impl CameraBuilder {
    /// Construct the `Camera`.
    #[inline]
    #[must_use]
    pub fn build(self) -> Camera {
        Camera::new(
            Orientation::new_tar(self.pos, &self.tar),
            self.fov.to_radians(),
            self.res,
            self.ss_power.map_or(1, |ss| ss),
        )
    }
}
