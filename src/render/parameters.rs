//! Render program runtime parameters.

use std::path::PathBuf;

use crate::render::Settings;

/// Runtime data.
pub struct Parameters {
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Runtime settings.
    pub settings: Settings,
}

impl Parameters {
    pub fn new(output_dir: PathBuf, settings: Settings) -> Self {
        Self {
            output_dir,
            settings,
        }
    }
}
