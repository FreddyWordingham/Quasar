//! Triangle.

use nalgebra;

use crate::{
    algebra::{Dir3, Pos3, Vec3},
    geometry::Cube,
};

/// Triangle.
pub struct Triangle {
    /// Vertex positions.
    pub verts: [Pos3; 3],
    /// vertex normals.
    pub norms: [Dir3; 3],
    /// Plane normal.
    plane_norm: Dir3,
}

impl Triangle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Pos3; 3], norms: [Dir3; 3]) -> Self {
        let plane_norm = Dir3::new_normalize((verts[0] - verts[2]).cross(&(verts[1] - verts[0])));

        Self {
            verts,
            norms,
            plane_norm,
        }
    }

    /// Calculate the central position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Pos3 {
        Pos3::from(
            ((self.verts[0].to_homogeneous()
                + self.verts[1].to_homogeneous()
                + self.verts[2].to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }

    /// Calculate the side lengths.
    #[inline]
    #[must_use]
    pub fn side_lengths(&self) -> [f64; 3] {
        let ab = nalgebra::distance(&self.verts[0], &self.verts[1]);
        let bc = nalgebra::distance(&self.verts[1], &self.verts[2]);
        let ca = nalgebra::distance(&self.verts[2], &self.verts[0]);

        [ab, bc, ca]
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        self.side_lengths().iter().sum()
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        let s = (ab + bc + ca) * 0.5;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Check for an intersection with a given bounding box.
    #[inline]
    #[must_use]
    pub fn collides(&self, cube: &Cube) -> bool {
        let c = cube.centre();
        let e = cube.half_widths();

        let v0 = self.verts[0] - c;
        let v1 = self.verts[1] - c;
        let v2 = self.verts[2] - c;

        let f0 = v1 - v0;
        let f1 = v2 - v1;
        let f2 = v0 - v2;

        let u0 = Vec3::x_axis();
        let u1 = Vec3::y_axis();
        let u2 = Vec3::z_axis();

        let axis_test = |axis: &Vec3| {
            let p0 = v0.dot(axis);
            let p1 = v1.dot(axis);
            let p2 = v2.dot(axis);

            let r = e.z.mul_add(
                u2.dot(axis).abs(),
                e.x.mul_add(u0.dot(axis).abs(), e.y * u1.dot(axis).abs()),
            );

            if (-(p0.max(p1).max(p2))).max(p0.min(p1).min(p2)) > r {
                return false;
            }

            true
        };

        if !axis_test(&u0) {
            return false;
        }
        if !axis_test(&u1) {
            return false;
        }
        if !axis_test(&u2) {
            return false;
        }

        let axis_u0_f0 = u0.cross(&f0);
        let axis_u0_f1 = u0.cross(&f1);
        let axis_u0_f2 = u0.cross(&f2);

        let axis_u1_f0 = u1.cross(&f0);
        let axis_u1_f1 = u1.cross(&f1);
        let axis_u1_f2 = u1.cross(&f2);

        let axis_u2_f0 = u2.cross(&f0);
        let axis_u2_f1 = u2.cross(&f1);
        let axis_u2_f2 = u2.cross(&f2);

        if !axis_test(&axis_u0_f0) {
            return false;
        }
        if !axis_test(&axis_u0_f1) {
            return false;
        }
        if !axis_test(&axis_u0_f2) {
            return false;
        }

        if !axis_test(&axis_u1_f0) {
            return false;
        }
        if !axis_test(&axis_u1_f1) {
            return false;
        }
        if !axis_test(&axis_u1_f2) {
            return false;
        }

        if !axis_test(&axis_u2_f0) {
            return false;
        }
        if !axis_test(&axis_u2_f1) {
            return false;
        }
        if !axis_test(&axis_u2_f2) {
            return false;
        }

        if !axis_test(&self.plane_norm) {
            return false;
        }

        true
    }
}
