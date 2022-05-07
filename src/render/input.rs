//! Program runtime.

use crate::{
    dom::Tree,
    render::{Settings, Shader},
};

/// Program runtime data.
pub struct Input<'a, T> {
    /// Simulation settings.
    pub settings: Settings,
    /// Aesthetic settings.
    pub shader: Shader<'a>,
    /// Scene hierarchy.
    pub tree: Tree<'a, T>,
}

impl<'a, T> Input<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(settings: Settings, shader: Shader<'a>, tree: Tree<'a, T>) -> Self {
        Self {
            settings,
            shader,
            tree,
        }
    }
}
