//! Surface.

use crate::geom::Mesh;

/// Surface with properties.
pub struct Surface<'a, T> {
    /// Mesh.
    pub mesh: Mesh,
    /// Attribute.
    pub attr: &'a T,
}
