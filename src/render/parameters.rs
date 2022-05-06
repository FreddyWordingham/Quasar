//! Input configuration.

use palette::{Gradient, LinSrgba};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    dom::TreeSettings,
    parse::json,
    render::{
        Attribute, AttributeBuilder, CameraBuilder, GradientBuilder, Settings, ShaderBuilder,
        SurfaceBuilder,
    },
};

/// Input configuration.
#[derive(Deserialize)]
pub struct Parameters {
    /// Path to the top level resource directory.
    input_dir: PathBuf,
    /// Path to the output directory.
    _output_dir: PathBuf,
    /// Oct-tree settings.
    _tree: TreeSettings,
    /// Runtime settings.
    _settings: Settings,
    /// Shader settings.
    shader: ShaderBuilder,
    /// Main camera.
    _cameras: Vec<CameraBuilder>,
    /// Surfaces.
    surfaces: Vec<SurfaceBuilder>,
}

impl Parameters {
    /// Get the names of the `Gradient`s used.
    #[inline]
    #[must_use]
    pub fn used_gradient_names(&self) -> Vec<String> {
        let mut gradient_names = Vec::new();

        gradient_names.extend(self.shader.used_gradient_names());
        gradient_names.extend(&mut self.used_attribute_names().iter().flat_map(|n| {
            json::load::<AttributeBuilder>(
                &self
                    .input_dir
                    .join("attributes")
                    .join(n)
                    .with_extension("json"),
            )
            .used_gradient_names()
        }));

        gradient_names.sort();
        gradient_names.dedup();

        gradient_names
    }

    /// Get the names of the `Attribute`s used.
    #[inline]
    #[must_use]
    pub fn used_attribute_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        for surf in &self.surfaces {
            names.push(surf.1.clone());
        }

        names.sort();
        names.dedup();

        names
    }

    /// Load the dictionary of `Gradients`.
    #[inline]
    #[must_use]
    pub fn load_gradients(&self) -> HashMap<String, Gradient<LinSrgba>> {
        let mut grads = HashMap::new();

        for name in self.used_gradient_names() {
            let grad = json::load::<GradientBuilder>(
                &self
                    .input_dir
                    .join("gradients")
                    .join(name.clone())
                    .with_extension("json"),
            )
            .build();
            grads.insert(name, grad);
        }

        grads
    }

    /// Load the dictionary of `Attributes`.
    #[inline]
    #[must_use]
    pub fn load_attributes<'a>(
        &self,
        grads: &'a HashMap<String, Gradient<LinSrgba>>,
    ) -> HashMap<String, Attribute<'a>> {
        let mut attrs = HashMap::new();

        for name in self.used_attribute_names() {
            let grad = json::load::<AttributeBuilder>(
                &self
                    .input_dir
                    .join("attributes")
                    .join(name.clone())
                    .with_extension("json"),
            )
            .build(grads);
            attrs.insert(name, grad);
        }

        attrs
    }
}
