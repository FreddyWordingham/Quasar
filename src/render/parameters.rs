//! Input configuration.

use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    dom::TreeSettings,
    render::{CameraBuilder, Settings, ShaderBuilder, SurfaceBuilder},
};

/// Input configuration.
#[derive(Deserialize)]
pub struct Parameters {
    /// Path to the top level resource directory.
    _input_dir: PathBuf,
    /// Path to the output directory.
    _output_dir: PathBuf,
    /// Oct-tree settings.
    _tree: TreeSettings,
    /// Runtime settings.
    _settings: Settings,
    /// Shader settings.
    _shader: ShaderBuilder,
    /// Main camera.
    _cameras: Vec<CameraBuilder>,
    /// Surfaces.
    _surfaces: Vec<SurfaceBuilder>,
}
