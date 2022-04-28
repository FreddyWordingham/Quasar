//! Gradient builder.

use serde::Deserialize;

/// Colour gradient.
#[derive(Deserialize)]
pub struct GradientBuilder(
    /// List of colours.
    Vec<String>,
);
