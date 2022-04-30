//! Input configuration.

use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    dom::{SurfaceBuilder, TreeSettings},
    render::Parameters,
};

/// Input configuration.
#[derive(Deserialize)]
pub struct ParametersBuilder {
    /// Path to the top level resource directory.
    _input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Oct-tree settings.
    pub tree: TreeSettings,
    /// Surfaces.
    pub surfs: Vec<SurfaceBuilder>,
}

impl ParametersBuilder {
    pub fn build(self) -> Parameters {
        Parameters::new(self.output_dir)
    }
}
