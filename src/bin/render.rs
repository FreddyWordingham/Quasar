use quasar::{args, parse::json, render::Config};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    let _config: Config = json::load(&params_path);
}
