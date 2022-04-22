use serde::Deserialize;
use std::path;

use quasar::{args, parse};

/// Configuration object.
#[derive(Deserialize)]
pub struct Config {
    // Resolution.
    pub res: [usize; 2],
}

fn main() {
    let config = init();

    println!("Configuration: {}", config.res[0]);
    println!("Configuration: {}", config.res[1]);

    for n in 0..100 {
        println!("Line! {}", n);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Read the input arguments.
/// Return the configuration object.
fn init() -> Config {
    args!(_bin_path: path::PathBuf, params_path: path::PathBuf);
    parse::json::load(&params_path)
}
