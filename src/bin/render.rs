use serde::Deserialize;
use std::path;

use quasar::{args, parse};

/// Configuration object.
#[derive(Deserialize)]
pub struct Config {
    // Output image resolution.
    pub res: [usize; 2],
}

/// Main recipe function.
fn main() {
    let config = init();
    let _output = run(config);
}

/// Read the input arguments.
/// Return the configuration object.
fn init() -> Config {
    args!(_bin_path: path::PathBuf, params_path: path::PathBuf);
    parse::json::load(&params_path)
}

/// Run the simulation.
fn run(config: Config) -> () {
    println!("Configuration: {}", config.res[0]);
    println!("Configuration: {}", config.res[1]);
}
