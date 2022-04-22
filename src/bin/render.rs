use ndarray::Array;
use palette::LinSrgba;
use rayon::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

use quasar::{
    args,
    geometry::Mesh,
    parse::{json, png, wavefront},
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
    /// Mesh names to render.
    pub meshes: Vec<String>,
}

/// Runtime parameters.
pub struct Parameters {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Meshes to render.
    pub meshes: Vec<Mesh>,
}

/// Main recipe function.
fn main() {
    let config = init();
    run(load(config));
}

/// Read the input arguments.
/// Return the configuration object.
#[inline]
#[must_use]
fn init() -> Config {
    args!(_bin_path: PathBuf, params_path: PathBuf);
    json::load(&params_path)
}

/// Load resources.
#[inline]
#[must_use]
fn load(config: Config) -> Parameters {
    let meshes: Vec<_> = config
        .meshes
        .into_par_iter()
        .map(|name| {
            let path = config
                .input_dir
                .join("meshes")
                .join(name)
                .with_extension("obj");

            wavefront::load(&path)
        })
        .collect();

    Parameters {
        input_dir: config.input_dir,
        output_dir: config.output_dir,
        res: config.res,
        meshes: meshes,
    }
}

/// Run the simulation.
#[inline]
fn run(config: Parameters) {
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
