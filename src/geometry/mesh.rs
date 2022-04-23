//! Triangle-mesh.

use itertools::izip;
use ndarray::parallel::prelude::IntoParallelRefIterator;
use ndarray::parallel::prelude::ParallelIterator;

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
            for vert in tri.verts {
                // for (vert, (min, max)) in vert.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                for (vert, (min, max)) in
                    izip!(vert.iter(), izip!(mins.iter_mut(), maxs.iter_mut()))
                {
                    if *min > *vert {
                        *min = *vert;
                    } else if *max < *vert {
                        *max = *vert;
                    }
                }
            }
        }
        let boundary = Cube::new(mins, maxs);

        Self { boundary, tris }
    }

    /// Check for an intersection with a given bounding box.
    #[inline]
    #[must_use]
    pub fn collides(&self, cube: &Cube) -> bool {
        if !self.boundary.collides(cube) {
            return false;
        }

        !self.tris.par_iter().all(|tri| !tri.collides(cube))
    }
}
