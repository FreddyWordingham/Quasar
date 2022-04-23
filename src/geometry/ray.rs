//! Ray.

use crate::algebra::{Dir3, Pos3, Rot3, Vec3};

/// Point and direction.
pub struct Ray {
    /// Ray origin.
    pub pos: Pos3,
    /// Ray direction.
    pub dir: Dir3,
}

impl Ray {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, mut dir: Dir3) -> Self {
        dir.renormalize();
        Self { pos, dir }
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.pos += self.dir.as_ref() * dist;
    }

    /// Rotate the photon with a given pitch and subsequent roll manoeuver.
    #[inline]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let arbitrary_axis = if (1.0 - self.dir.z.abs()) >= 1.0e-1 {
            Vec3::z_axis()
        } else {
            Vec3::y_axis()
        };

        let pitch_axis = Dir3::new_normalize(self.dir.cross(&arbitrary_axis));
        let pitch_rot = Rot3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rot3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
        self.dir.renormalize();
    }
}