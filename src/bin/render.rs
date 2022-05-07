use ndarray::Array2;
use palette::LinSrgba;
use quasar::{
    args,
    parse::{json, png},
    render::Parameters,
    util::ProgressBar,
};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, parameters_path: PathBuf);

    let parameters = json::load::<Parameters>(&parameters_path);

    let meshes = parameters.load_meshes();
    let gradients = parameters.load_gradients();
    let attributes = parameters.load_attributes(&gradients);
    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    let _shader = parameters.build_shader(&gradients);
    let settings = parameters.build_settings();
    let cameras = parameters.build_cameras();
    let tree = parameters.build_tree(&surfaces);

    for (name, cam) in cameras {
        let mut pb = ProgressBar::new("Rendering", cam.res[0] * cam.res[1]);
        let mut image = Array2::from_elem(cam.res, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
        for px in 0..cam.res[0] {
            for py in 0..cam.res[1] {
                for ssx in 0..cam.ss_power {
                    for ssy in 0..cam.ss_power {
                        let ray = cam.emit([px, py], [ssx, ssy]);
                        if let Some(hit) = tree.scan(ray, settings.bump_dist, 200.0) {
                            // let d = hit.dist.min(20.0) / 20.0;
                            // image[(px, py)] = shader.data_grad.get(d as f32);

                            let n = hit.side.norm();
                            let r = n.x.abs() as f32;
                            let g = n.y.abs() as f32;
                            let b = n.z.abs() as f32;

                            image[(px, py)] += LinSrgba::new(r, g, b, 1.0);
                        }
                    }
                }

                pb.tick();
            }
        }
        pb.finish_with_message("Rendered.");

        png::save(
            image.view(),
            &parameters.output_dir.join(name).with_extension("png"),
        );
    }
}
