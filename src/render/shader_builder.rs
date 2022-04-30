//! Shader settings.

use nalgebra::Vector3;
use serde::Deserialize;

/// Shader configuration.
#[derive(Deserialize)]
pub struct ShaderBuilder {
    /// Sun position (m).
    sun_pos: Vector3<f64>,
    /// Ambient, diffuse, and occlusion lighting fractions.
    light: [f64; 3],
    /// Ambient, diffuse, and occlusion shadowing fractions.
    shadow: [f64; 2],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius (deg).
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky colour gradient.
    sky_grad: String,
    /// Data colouring gradient.
    data_grad: String,
}

impl ShaderBuilder {
    /// Get the names of the gradients used.
    #[inline]
    #[must_use]
    pub fn gradients(&self) -> Vec<String> {
        vec![self.sky_grad.clone(), self.data_grad.clone()]
    }
}
