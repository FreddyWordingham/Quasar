//! Input configuration.

use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    dom::{SurfaceBuilder, TreeSettings},
    render::{CameraBuilder, Parameters, Settings, ShaderBuilder},
};

/// Input configuration.
#[derive(Deserialize)]
pub struct ParametersBuilder {
    /// Path to the top level resource directory.
    _input_dir: PathBuf,
    /// Path to the output directory.
    output_dir: PathBuf,
    /// Oct-tree settings.
    tree: TreeSettings,
    /// Runtime settings.
    settings: Settings,
    /// Shader settings.
    shader: ShaderBuilder,
    /// Main camera.
    cameras: Vec<CameraBuilder>,
    /// Surfaces.
    surfs: Vec<SurfaceBuilder>,
}

impl ParametersBuilder {
    /// Construct the `Parameters`.
    #[inline]
    #[must_use]
    pub fn build(self) -> Parameters {
        Parameters::new(self.output_dir, self.settings)
    }
}
