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
    input_dir: PathBuf,
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
    surfaces: Vec<SurfaceBuilder>,
}
