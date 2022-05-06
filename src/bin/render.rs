use quasar::{args, parse::json, render::Parameters};
use std::path::PathBuf;

/// Main recipe function.
fn main() {
    args!(_bin_path: PathBuf, parameters_path: PathBuf);

    let parameters = json::load::<Parameters>(&parameters_path);

    let meshes = parameters.load_meshes();
    println!("- Meshes");
    print_set(&meshes);

    let gradients = parameters.load_gradients();
    println!("- Gradients");
    print_set(&gradients);

    let attributes = parameters.load_attributes(&gradients);
    println!("- Attributes");
    print_set(&attributes);

    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    println!("- Surfaces");
    println!("{}", surfaces.len());
}

fn print_set<T>(set: &std::collections::HashMap<String, T>) {
    set.keys().map(|k| println!("[{}]", k)).collect::<()>()
}
