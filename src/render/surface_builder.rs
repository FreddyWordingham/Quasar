//! Surface loader.

use serde::Deserialize;
use std::path::PathBuf;

use crate::parse::json;

/// Surface parameterisation.
#[derive(Deserialize)]
pub struct SurfaceBuilder(
    /// Mesh name.
    pub String,
    /// Attribute name.
    pub String,
);
