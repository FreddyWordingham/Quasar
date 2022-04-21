//! Session data.

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
}
