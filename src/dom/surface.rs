//! Surface.

use crate::geom::Mesh;

/// Three-dimension triangular mesh with attribute data.
pub struct Surface<'a, T> {
    /// Mesh.
    pub mesh: Mesh,
    /// Attribute.
    pub attr: &'a T,
}
