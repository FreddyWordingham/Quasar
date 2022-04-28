//! Render program runtime parameters.

use palette::{Gradient, LinSrgba};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::render::Attribute;

/// Runtime data.
pub struct Parameters<'a> {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Dumps.
    pub dumps: usize,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Scanning boundaries.
    pub scan: [f64; 4],
    /// Gradients.
    pub grads: HashMap<String, Gradient<LinSrgba>>,
    /// Attributes.
    pub attrs: HashMap<String, Attribute<'a>>,
}
