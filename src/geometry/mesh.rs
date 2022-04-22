//! Triangle-mesh.

use crate::geometry::{Cube, Triangle};

/// Mesh of triangles.
pub struct Mesh {
    /// Bounding box.
    pub boundary: Cube,
    /// List of component triangles.
    pub tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tris: Vec<Triangle>) -> Self {
        debug_assert!(!tris.is_empty());

        // Calculate bounding box.
        let mut mins = tris[0].centre();
        let mut maxs = mins;
        for tri in &tris {
            for v in tri.verts {
                for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                    if *min > *v {
                        *min = *v;
                    } else if *max < *v {
                        *max = *v;
                    }
                }
            }
        }
        let boundary = Cube::new(mins, maxs);

        Self { boundary, tris }
    }
}
