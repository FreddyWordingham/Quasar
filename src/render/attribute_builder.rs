//! Observable attributes.

use serde::Deserialize;

/// Attribute builder.
#[derive(Deserialize)]
pub enum AttributeBuilder {
    /// Opaque coloured surface.
    Opaque(String),
    /// Partially reflective mirror, absorption fraction.
    Mirror(String, f64),
    /// Partially transparent, absorption fraction.
    Transparent(String, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(String, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(String, f64),
    /// Switchable condition, conditional value.
    Switchable([String; 2], f64),
}

impl AttributeBuilder {
    /// Get the names of the gradients used.
    #[inline]
    #[must_use]
    pub fn colours(&self) -> Vec<String> {
        match *self {
            Self::Opaque(ref grad)
            | Self::Mirror(ref grad, ..)
            | Self::Transparent(ref grad, ..)
            | Self::Refractive(ref grad, ..)
            | Self::Luminous(ref grad, ..) => vec![grad.clone()],
            Self::Switchable(ref grads, ..) => grads.clone().to_vec(),
        }
    }
}
