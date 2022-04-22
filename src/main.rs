use linemux::MuxedLines;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use quasar::args;

/// Handles a single computation session.
/// Watch an input file for newly added lines,
/// calling a processing function with any new commands.
/// After each check, sleep for the given interval [sec].
#[tokio::main]
async fn main() -> std::io::Result<()> {
    args!(_bin_path: PathBuf, session_id: String);

    let session_dir = PathBuf::from(format!("./app/static/sessions/{}", session_id));
    let input = session_dir.join("session.input");
    let output = session_dir.join("session.output");

    let mut session = Session::new(session_id);

    let mut lines = MuxedLines::new()?;
    lines.add_file(input).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        let command = line.line();

        if command.chars().next().unwrap() == '#' {
            continue;
        }

        let response = session.process(command);

        let mut outfile = OpenOptions::new()
            .append(true)
            .open(output.clone())
            .unwrap();
        writeln!(outfile, "{}", response).unwrap();
    }

    Ok(())
}

/// Session data.
pub struct Session {
    /// Unique identifier string.
    id: String,
}

impl Session {
    /// Construct a new session instance.
    pub fn new(id: String) -> Session {
        Session { id }
    }

    /// Run a command.
    pub fn process(&mut self, command: &str) -> String {
        format!("ERROR! Did not understand command '{}'.", command)
    }

    /// Get the session id.
    pub fn id(&self) -> &str {
        &self.id
    }
}
