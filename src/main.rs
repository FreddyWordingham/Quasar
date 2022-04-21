use notify::{watcher, RecursiveMode, Watcher};
use std::{env::current_dir, path::PathBuf, sync::mpsc::channel, time::Duration};

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

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(sleep_time)).unwrap();

    let session_dir = format!(
        "{}{}",
        "/app/static/sessions/server_command.log", session_id
    );
    println!("Session dir: {}", session_dir);
    watcher
        .watch(session_dir, RecursiveMode::Recursive)
        .unwrap();
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
