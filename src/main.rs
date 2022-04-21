use std::path::PathBuf;

use quasar::args;

/// Handles a single computation session.
/// Watch an input file for newly added lines,
/// calling a processing function with any new commands.
/// After each check, sleep for the given interval [sec].
fn main() {
    args!(_bin_path: PathBuf, session_id: String, sleep_time: f32);

    let interval = (sleep_time * 1000.0) as u64;

    let mut loop_counter = 0;
    loop {
        println!("[{}] {:4} > Hello, world!", session_id, loop_counter);

        std::thread::sleep(std::time::Duration::from_millis(interval));
        loop_counter += 1;
    }
}
