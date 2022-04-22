use ndarray::Array;
use palette::LinSrgba;
use serde::Deserialize;
use std::path::PathBuf;

use quasar::{
    args,
    parse::{json, png},
};

/// Configuration object.
#[derive(Deserialize)]
pub struct Config {
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Output image resolution.
    pub res: [usize; 2],
}

/// Main recipe function.
fn main() {
    let config = init();
    run(config);
}

/// Read the input arguments.
/// Return the configuration object.
fn init() -> Config {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    json::load(&params_path)
}

/// Run the simulation.
fn run(config: Config) {
    println!("Configuration: {}", config.res[0]);
    println!("Configuration: {}", config.res[1]);

    let mut image = Array::from_elem(
        (config.res[0], config.res[1]),
        LinSrgba::new(0.4, 0.6, 0.9, 0.5),
    );

    for xi in 0..100 {
        for yi in 0..100 {
            image[(xi, yi)] = LinSrgba::new(0.6, 0.2, 0.1, 0.5);
        }
    }

    println!("CWD: {}", std::env::current_dir().unwrap().display());
    png::save(image.view(), &config.output_dir.join("output.png"));
}
