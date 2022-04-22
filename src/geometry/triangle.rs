//! Triangle.

use nalgebra;

use crate::algebra::{Dir3, Pos3};

/// Triangle.
pub struct Triangle {
    /// Vertex positions.
    verts: [Pos3; 3],
    /// vertex normals.
    norms: [Dir3; 3],
}

impl Triangle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Pos3; 3], norms: [Dir3; 3]) -> Self {
        Self { verts, norms }
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
}
