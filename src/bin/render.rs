use nalgebra::Point3;
use ndarray::Array;
use palette::LinSrgba;
use rayon::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

use quasar::{
    args,
    geom::{Mesh, Ray},
    parse::{json, png, wavefront},
    // utility::ProgressBar,
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
    /// Dumps.
    pub dumps: usize,
    /// Mesh names to render.
    pub meshes: Vec<String>,
    /// Scanning boundaries.
    pub scan: [f64; 4],
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
    /// Dumps.
    pub dumps: usize,
    /// Scanning boundaries.
    pub scan: [f64; 4],
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
        dumps: config.dumps,
        meshes: meshes,
        scan: config.scan,
    }
}

/// Run the simulation.
#[inline]
fn run(config: Parameters) {
    let mut image = Array::from_elem(
        (config.res[0], config.res[1]),
        LinSrgba::new(0.0, 0.0, 0.0, 0.5),
    );

    let min_x = config.scan[0];
    let dx = (config.scan[1] - config.scan[0]) / (config.res[0] - 1) as f64;
    let min_y = config.scan[2];
    let dy = (config.scan[3] - config.scan[2]) / (config.res[1] - 1) as f64;

    let dump = (config.res[0] * config.res[1]) / config.dumps;
    let mut n = 0;

    // let mut pb = ProgressBar::new("Rendering", config.res[0] * config.res[1]);
    for xi in 0..config.res[0] {
        println!("{:.2}%", 100.0 * xi as f64 / config.res[0] as f64);

        let x = min_x + (xi as f64 * dx);

        for yi in 0..config.res[1] {
            let y = min_y + (yi as f64 * dy);

            let ray = Ray::new(Point3::new(x, -10.0, y), nalgebra::Vector3::y_axis());

            let mut min_dist = 20.0;
            let mut min_norm = None;
            for mesh in &config.meshes {
                if let Some((mesh_dist, mesh_side)) = mesh.dist_side(&ray) {
                    if mesh_dist < min_dist {
                        min_dist = mesh_dist;
                        min_norm = Some(mesh_side.norm().clone());
                    }
                }
            }
            if let Some(norm) = min_norm {
                image[(xi, yi)] = LinSrgba::new(
                    (norm.x as f32).abs(),
                    (norm.y as f32).abs(),
                    (norm.z as f32).abs(),
                    1.0,
                );
            }

            // pb.tick();
            if n > dump {
                png::save(image.view(), &config.output_dir.join("render.png"));
                n = 0;
            }
            n += 1;
        }
    }

    png::save(image.view(), &config.output_dir.join("render.png"));
    println!("FINISHED");
}
