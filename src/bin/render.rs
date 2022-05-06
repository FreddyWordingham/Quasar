use quasar::{args, render::Parameters};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, parameters_path: PathBuf);

    let parameters = Parameters::load(&parameters_path);

    let _gradients = parameters.load_gradients();
}
