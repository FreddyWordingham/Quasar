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
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Meshes to render.
    pub meshes: Vec<String>,
}

/// Main recipe function.
fn main() {
    let config = init();
    run(load(config));
}

/// Read the input arguments.
/// Return the configuration object.
fn init() -> Config {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    json::load(&params_path)
}

/// Load resources.
fn load(config: Config) -> Config {
    let meshes: Vec<_> = config
        .meshes
        .iter()
        .map(|name| {
            let path = config
                .input_dir
                .join("meshes")
                .join(name)
                .with_extension("obj");

            println!("Loading mesh: {}", path.display());

            ()
        })
        .collect();

    config
}

/// Run the simulation.
fn run(config: Config) {
    let mut image = Array::from_elem(
        (config.res[0], config.res[1]),
        LinSrgba::new(0.4, 0.6, 0.9, 0.5),
    );

    for xi in 0..100 {
        for yi in 0..100 {
            image[(xi, yi)] = LinSrgba::new(0.6, 0.2, 0.1, 0.5);
        }
    }

    png::save(image.view(), &config.output_dir.join("output.png"));
}
