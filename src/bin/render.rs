use quasar::{args, parse::json, render::Parameters};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    let _params = Parameters::load(&params_path);
}
