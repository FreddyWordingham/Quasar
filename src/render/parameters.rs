//! Render program runtime parameters.

use std::path::PathBuf;

/// Runtime data.
pub struct Parameters {
    /// Path to the output directory.
    pub output_dir: PathBuf,
}

impl Parameters {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            output_dir: output_dir,
        }
    }
}
