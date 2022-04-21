use std::{fs::OpenOptions, io::Write, path::PathBuf};

use quasar::args;

/// Handles a single computation session.
/// Watch an input file for newly added lines,
/// calling a processing function with any new commands.
/// After each check, sleep for the given interval [sec].
fn main() {
    args!(_bin_path: PathBuf, session_id: String, sleep_time: u64);

    let session_dir = PathBuf::from(format!("./app/static/sessions/{}", session_id));
    let session_input_file = session_dir.join("session.input");
    let session_output_file = session_dir.join("session.output");

    let mut outfile = OpenOptions::new()
        .append(true)
        .open(session_output_file)
        .unwrap();

    writeln!(outfile, "Hello world!").unwrap();
}
