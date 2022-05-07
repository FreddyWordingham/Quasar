//! Program runtime.

use crate::{
    dom::Tree,
    render::{Settings, Shader},
};

/// Program runtime data.
pub struct Input<'a, T> {
    pub settings: Settings,
    pub shader: Shader<'a>,
    pub tree: Tree<'a, T>,
}

impl<'a, T> Input<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(settings: Settings, shader: Shader<'a>, tree: Tree<'a, T>) -> Self {
        Self {
            settings,
            shader,
            tree,
        }
    }
}
