use palette::LinSrgba;
use quasar::{
    args,
    parse::json,
    render::{run, Input, Output, Parameters},
    rt::Ray,
};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, parameters_path: PathBuf);
    let parameters = json::load::<Parameters>(&parameters_path);
    run(parameters, sample);
}

/// Sample the first hit normal direction.
#[allow(dead_code)]
#[inline]
fn sample(input: &Input, ray: Ray, weight: f32, pixel: [usize; 2], data: &mut Output) {
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
