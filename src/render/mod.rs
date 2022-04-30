//! Rendering.

pub mod attribute;
pub mod attribute_builder;
pub mod camera;
pub mod camera_builder;
pub mod gradient_builder;
pub mod parameters;
pub mod parameters_builder;
pub mod settings;
pub mod shader;
pub mod shader_builder;

pub use self::{
    attribute::*, attribute_builder::*, camera::*, camera_builder::*, gradient_builder::*,
    parameters::*, parameters_builder::*, settings::*, shader::*, shader_builder::*,
};
