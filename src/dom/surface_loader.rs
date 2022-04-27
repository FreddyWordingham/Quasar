//! Surface loader.

use serde::Deserialize;

/// Surface parameterisation.
#[derive(Deserialize)]
pub struct SurfaceLoader(
    /// Mesh name.
    String,
    /// Attribute name.
    String,
);
