use notify::{watcher, RecursiveMode, Watcher};
use std::{
    env::current_dir, fs::OpenOptions, io::Write, path::PathBuf, sync::mpsc::channel,
    time::Duration,
};

use quasar::args;

/// Handles a single computation session.
/// Watch an input file for newly added lines,
/// calling a processing function with any new commands.
/// After each check, sleep for the given interval [sec].
fn main() {
    args!(_bin_path: PathBuf, session_id: String, sleep_time: u64);
    println!(
        "[{}] Running at: {}",
        session_id,
        current_dir().unwrap().display()
    );

    let session_dir = PathBuf::from(format!("./app/static/sessions/{}", session_id));
    let session_input = session_dir.join("session.input");
    let session_output = session_dir.join("session.output");

    println!("Session dir: {}", session_dir.display());
    println!("Session output: {}", session_output.display());

    let mut outfile = OpenOptions::new()
        .append(true)
        .open(session_output)
        .unwrap();
    writeln!(outfile, "Hello world!");
}
