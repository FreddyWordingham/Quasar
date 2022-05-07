use quasar::{args, parse::json, render::Parameters};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, parameters_path: PathBuf);

    let parameters = json::load::<Parameters>(&parameters_path);

    let meshes = parameters.load_meshes();
    let gradients = parameters.load_gradients();
    let attributes = parameters.load_attributes(&gradients);
    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    let _shader = parameters.build_shader(&gradients);
    let _settings = parameters.build_settings();
    let _tree = parameters.build_tree(&surfaces);
}
