use palette::LinSrgba;
use quasar::{
    args,
    dom::Tree,
    parse::json,
    render::{Camera, Data, Parameters, Settings, Shader},
    util::ProgressBar,
};
use std::{fs, path::PathBuf};

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

    // Run
    for (name, cam) in cameras {
        let output_dir = parameters.output_dir.join(name.clone());
        fs::create_dir(&output_dir).expect("Failed to create output directory.");

        let data = run(&output_dir, &settings, &tree, &shader, &cam);
        data.save(&output_dir);
    }
}

/// Run a simulation.
#[inline]
#[must_use]
pub fn run<T>(
    _output_dir: &PathBuf,
    settings: &Settings,
    tree: &Tree<T>,
    _shader: &Shader,
    camera: &Camera,
) -> Data {
    let mut data = Data::new(camera.res);

    let k = 1.0 / (camera.ss_power * camera.ss_power) as f32;

    let mut pb = ProgressBar::new("Rendering", camera.res[0] * camera.res[1]);
    for px in 0..camera.res[0] {
        for py in 0..camera.res[1] {
            for ssx in 0..camera.ss_power {
                for ssy in 0..camera.ss_power {
                    let ray = camera.emit([px, py], [ssx, ssy]);
                    if let Some(hit) = tree.scan(ray, settings.bump_dist, 200.0) {
                        // let d = hit.dist.min(20.0) / 20.0;
                        // image[(px, py)] = shader.data_grad.get(d as f32);

                        let n = hit.side.norm();
                        let r = n.x.abs() as f32;
                        let g = n.y.abs() as f32;
                        let b = n.z.abs() as f32;

                        data.colour[(px, py)] += LinSrgba::new(r, g, b, 1.0) * k;
                    }
                }
            }

            pb.tick();
        }
    }
    pb.finish_with_message("Rendered.");

    data
}
