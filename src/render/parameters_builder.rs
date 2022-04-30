//! Input configuration.

use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    dom::{SurfaceBuilder, TreeSettings},
    render::{Parameters, Settings, ShaderBuilder},
};

/// Input configuration.
#[derive(Deserialize)]
pub struct ParametersBuilder {
    /// Path to the top level resource directory.
    _input_dir: PathBuf,
    /// Path to the output directory.
    output_dir: PathBuf,
    /// Runtime settings.
    settings: Settings,
    /// Shader settings.
    shader: ShaderBuilder,
    /// Oct-tree settings.
    tree: TreeSettings,
    /// Surfaces.
    surfs: Vec<SurfaceBuilder>,
}

impl ParametersBuilder {
    pub fn build(self) -> Parameters {
        Parameters::new(self.output_dir, self.settings)
    }
}
