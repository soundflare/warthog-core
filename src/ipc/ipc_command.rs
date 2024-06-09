use std::path::PathBuf;

/// Enum with commands from the IPC.
#[derive(Clone, Debug)]
pub enum IpcCommand {
    /// Add folder for watching
    WatchFolder { path: PathBuf },
    /// Remove folder for watching
    UnwatchFolder { path: PathBuf },
}
