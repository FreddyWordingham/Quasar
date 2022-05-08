use palette::LinSrgba;
use quasar::{
    args,
    parse::json,
    render::{Camera, Input, Output, Parameters},
    rt::Ray,
    // util::ProgressBar,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Main recipe function.
fn main() {
    // Setup.
    args!(_bin_path: PathBuf, parameters_path: PathBuf);
    let parameters = json::load::<Parameters>(&parameters_path);
    let settings = parameters.build_settings();
    let meshes = parameters.load_meshes();
    let gradients = parameters.load_gradients();
    let attributes = parameters.load_attributes(&gradients);
    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    let tree = parameters.build_tree(&surfaces);
    let shader = parameters.build_shader(&gradients);
    let cameras = parameters.build_cameras();

    // Create runtime object.
    let runtime = Input::new(settings, shader, tree);

    // Run
    for (name, cam) in cameras {
        let output_dir = parameters.output_dir.join(name.clone());
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("Failed to initialise output directory.");
        }
        fs::create_dir(&output_dir).expect("Failed to create output directory.");
        render(&output_dir, &runtime, &cam);
    }
}

/// Perform the rendering.
#[inline]
fn render<T>(output_dir: &Path, input: &Input<T>, camera: &Camera) {
    let tiles = input.settings.tiles.unwrap_or([1, 1]);
    let tile_res = [camera.res[0] / tiles[0], camera.res[1] / tiles[1]];

    // let mut pb = ProgressBar::new("Rendering", tiles[0] * tiles[1]);
    for iy in 0..tiles[1] {
        for ix in 0..tiles[0] {
            let offset = [tile_res[0] * ix, tile_res[1] * iy];
            let data = render_tile(input, camera, offset, tile_res);
            data.save(
                output_dir,
                &format!("_{:0>3}_{:0>3}", ix, tiles[1] - iy - 1),
            );
            // pb.tick();
            println!(
                "{:.2}",
                (iy + (ix * tiles[1])) as f64 / (tiles[0] * tiles[1]) as f64 * 100.0,
            );
        }
    }
    // pb.finish_with_message("Rendering complete.");
    println!("FINISHED");
}

/// Render a sub-tile.
#[inline]
#[must_use]
fn render_tile<T>(
    input: &Input<T>,
    camera: &Camera,
    offset: [usize; 2],
    sub_res: [usize; 2],
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

/// Sample the scene.
#[inline]
fn sample<T>(input: &Input<T>, ray: Ray, weight: f32, pixel: [usize; 2], data: &mut Output) {
    let settings = &input.settings;
    let tree = &input.tree;
    let _shader = &input.shader;

    if let Some(hit) = tree.scan(ray, settings.bump_dist, 200.0) {
        // let d = hit.dist.min(20.0) / 20.0;
        // image[(px, py)] = shader.data_grad.get(d as f32);

        let n = hit.side.norm();
        let r = n.x.abs() as f32;
        let g = n.y.abs() as f32;
        let b = n.z.abs() as f32;

        data.colour[pixel] += LinSrgba::new(r, g, b, 1.0) * weight;
    }
}
