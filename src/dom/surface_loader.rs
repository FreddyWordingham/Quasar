//! Surface loader.

use serde::Deserialize;

/// Surface parameterisation.
#[derive(Deserialize)]
pub struct SurfaceLoader(
    /// Mesh name.
    pub String,
    /// Attribute name.
    pub String,
);
