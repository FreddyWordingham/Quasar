//! Triangle.

use crate::algebra::{Dir3, Pos3};

/// Triangle.
pub struct Triangle {
    /// Vertex positions.
    verts: [Pos3; 3],
    /// vertex normals.
    norms: [Dir3; 3],
}
