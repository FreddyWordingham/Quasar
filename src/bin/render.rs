use ndarray::Array;
use palette::LinSrgba;
use serde::Deserialize;
// use std::collections::HashMap;
use std::path::PathBuf;

use quasar::{
    args,
    // utility::ProgressBar,
    dom::SurfaceLoader,
    parse::{json, png},
};

/// Configuration object.
#[derive(Deserialize)]
pub struct Config {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Dumps.
    pub dumps: usize,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Scanning boundaries.
    pub scan: [f64; 4],
    /// Surfaces.
    pub surfaces: Vec<SurfaceLoader>,
}

/// Runtime parameters.
pub struct Parameters {
    /// Path to the top level resource directory.
    pub input_dir: PathBuf,
    /// Path to the output directory.
    pub output_dir: PathBuf,
    /// Dumps.
    pub dumps: usize,
    /// Output image resolution.
    pub res: [usize; 2],
    /// Scanning boundaries.
    pub scan: [f64; 4],
    // /// Attributes.
    // pub attrs: HashMap<String, i32>,
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
    Parameters {
        input_dir: config.input_dir,
        output_dir: config.output_dir,
        dumps: config.dumps,
        res: config.res,
        scan: config.scan,
    }
}

/// Run the simulation.
#[inline]
fn run(config: Parameters) {
    let image = Array::from_elem(
        (config.res[0], config.res[1]),
        LinSrgba::new(0.0, 0.0, 0.0, 0.5),
    );

    png::save(image.view(), &config.output_dir.join("render.png"));
    println!("FINISHED");
}
