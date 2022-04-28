//! Oct-tree construction settings.

use serde::Deserialize;

/// Tree construction settings.
#[derive(Deserialize)]
pub struct TreeSettings {
    /// Target maximum number of triangles per cell.
    pub tar_tris: usize,
    /// Maximum mesh depth.
    pub max_depth: u32,
    /// Collision detection padding.
    pub padding: f64,
}
