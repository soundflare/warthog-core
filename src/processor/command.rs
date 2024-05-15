use std::path::PathBuf;

/// Enum with commands for processor between the threads.
#[derive(Debug)]
pub enum Command {
    /// Add folder for watching
    AddFolder { path: String },
    /// Change detected in the file system.
    ChangeDetected { paths: Vec<PathBuf> },
}
