//! Surface loader.

use serde::Deserialize;

/// Surface parameterisation.
#[derive(Deserialize)]
pub struct SurfaceBuilder(
    /// Mesh name.
    pub String,
    /// Attribute name.
    pub String,
);
