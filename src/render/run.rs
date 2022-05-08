//! Run control.

use rand::{seq::SliceRandom, thread_rng};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{
    render::{Camera, Input, Output, Parameters},
    rt::Ray,
    util::ProgressBar,
};

/// Run the simulation with the given parameterisation.
#[inline]
pub fn run<T: Fn(&Input<'_>, Ray, f32, [usize; 2], &mut Output) -> () + Send + Sync + Copy>(
    parameters: Parameters,
    sample: T,
) {
    // Setup.
    let settings = parameters.build_settings();
    let meshes = parameters.load_meshes();
    let gradients = parameters.load_gradients();
    let attributes = parameters.load_attributes(&gradients);
    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    let tree = parameters.build_tree(&surfaces);
    let shader = parameters.build_shader(&gradients);
    let camera = parameters.build_camera();

    // Create runtime object.
    let runtime = Input::new(settings, shader, tree);

    // Run
    let output_dir = parameters.output_dir.join("tiles");
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("Failed to initialise output directory.");
    }
    fs::create_dir(&output_dir).expect("Failed to create output directory.");
    render(&output_dir, &runtime, &camera, sample);

    println!("FINISHED");
}

/// Perform the rendering.
#[inline]
fn render<T: Fn(&Input<'_>, Ray, f32, [usize; 2], &mut Output) -> () + Send + Sync + Clone>(
    output_dir: &Path,
    input: &Input,
    camera: &Camera,
    sample: T,
) {
    let tiles = input.settings.tiles;
    let tile_res = [camera.res[0] / tiles[0], camera.res[1] / tiles[1]];

    let mut tile_order = Vec::with_capacity(tiles[0] * tiles[1]);
    for iy in 0..tiles[1] {
        for ix in 0..tiles[0] {
            tile_order.push((ix, iy));
        }
    }
    tile_order.shuffle(&mut thread_rng());

    let pb = ProgressBar::new("Rendering image", tiles[0] * tiles[1]);
    let pb = Arc::new(Mutex::new(pb));
    let print_width = ((tiles[0].max(tiles[1])) as f64).log10() as usize + 1;
    tile_order.par_iter().for_each(|(ix, iy)| {
        let offset = [tile_res[0] * ix, tile_res[1] * iy];
        let data = render_tile(input, camera, offset, tile_res, sample.clone());
        data.save(
            output_dir,
            &format!(
                "{:0>width$}_{:0>width$}",
                ix,
                tiles[1] - iy - 1,
                width = print_width
            ),
        );

        let mut pb = pb.lock().expect("Could not lock progress bar.");
        pb.tick();
    });
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Rendering complete");
}

/// Render a sub-tile.
#[inline]
#[must_use]
fn render_tile<T: Fn(&Input<'_>, Ray, f32, [usize; 2], &mut Output) -> ()>(
    input: &Input,
    camera: &Camera,
    offset: [usize; 2],
    sub_res: [usize; 2],
    sample: T,
) -> Output {
    let mut data = Output::new(sub_res);

    let weight = 1.0 / (camera.ss_power * camera.ss_power) as f32;

    for px in 0..sub_res[0] {
        for py in 0..sub_res[1] {
            let rx = offset[0] + px;
            let ry = offset[1] + py;

            for ssx in 0..camera.ss_power {
                for ssy in 0..camera.ss_power {
                    let ray = camera.emit([rx, ry], [ssx, ssy]);
                    sample(input, ray, weight, [px, py], &mut data)
                }
            }
        }
    }

    data
}
