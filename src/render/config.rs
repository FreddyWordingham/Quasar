//! Render program input configuration.

use serde::Deserialize;
use std::path::PathBuf;

use crate::dom::SurfaceLoader;

/// Configuration object.
#[derive(Deserialize)]
pub struct Config {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Dumps.
    pub dumps: usize,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Scanning boundaries.
    pub scan: [f64; 4],
    /// Surfaces.
    pub surfaces: Vec<SurfaceLoader>,
}
