//! Input configuration.

use crate::parse::json;
use palette::{Gradient, LinSrgba};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    dom::TreeSettings,
    render::{CameraBuilder, GradientBuilder, Settings, ShaderBuilder, SurfaceBuilder},
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

impl Parameters {
    /// Load an instance from a file.
    #[inline]
    #[must_use]
    pub fn load(path: &PathBuf) -> Self {
        json::load(path)
    }

    /// Load the gradients.
    #[inline]
    #[must_use]
    pub fn load_gradients(&self) -> HashMap<String, Gradient<LinSrgba>> {
        let mut gradient_names = Vec::new();
        gradient_names.append(&mut self.shader.used_gradient_names());
        // gradient_names.append(
        //     &mut self
        //         .surfaces
        //         .iter()
        //         .map(|s| s.used_gradient_names())
        //         .collect()
        //         .flatten(),
        // );

        let mut gradients = HashMap::with_capacity(gradient_names.len());
        for name in gradient_names {
            let gradient = GradientBuilder::load(
                &self
                    .input_dir
                    .join("gradients")
                    .join(&name)
                    .with_extension("json"),
            )
            .build();

            println!("Loaded: {}", &name);
            gradients.insert(name, gradient);
        }

        gradients
    }
}
