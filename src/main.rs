use linemux::MuxedLines;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use quasar::args;

/// Handles a single computation session.
/// Watch an input file for newly added lines,
/// calling a processing function with any new commands.
/// After each check, sleep for the given interval [sec].
#[tokio::main]
async fn main() {
    args!(_bin_path: PathBuf, session_id: String);

    let session_dir = PathBuf::from(format!("./app/static/sessions/{}", session_id));
    let session_input_file = session_dir.join("session.input");
    let session_output_file = session_dir.join("session.output");

    // Watching loop.
    start(session_input_file, session_output_file).await;
}

async fn start(input: PathBuf, output: PathBuf) -> std::io::Result<()> {
    // Output
    let mut outfile = OpenOptions::new().append(true).open(output).unwrap();
    writeln!(outfile, "Hello world!").unwrap();

    // Input
    let mut lines = MuxedLines::new()?;
    lines.add_file(input).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        let command = line.line();
        println!("Processing: {}", command);
    }

    return Ok(());
}
