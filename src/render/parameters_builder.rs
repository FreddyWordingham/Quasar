//! Input configuration.

use std::path::PathBuf;

use crate::render::Parameters;

/// Input configuration.
pub struct ParametersBuilder {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
}

impl ParametersBuilder {
    pub fn build(self) -> Parameters {
        Parameters::new(self.input_dir, self.output_dir)
    }
}
