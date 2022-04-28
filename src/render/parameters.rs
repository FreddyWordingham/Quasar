//! Render program runtime parameters.

use std::path::PathBuf;

/// Runtime data.
pub struct Parameters {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
}

impl Parameters {
    pub fn new(input_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            input_dir: input_dir,
            output_dir: output_dir,
        }
    }
}
