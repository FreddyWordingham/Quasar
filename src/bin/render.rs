use quasar::{args, parse::json, render::ParametersBuilder};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    let _params_builder: ParametersBuilder = json::load(&params_path);
}
