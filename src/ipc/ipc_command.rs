/// Enum with commands from the IPC.
#[derive(Clone, Debug)]
pub enum IpcCommand {
    /// Add folder for watching
    WatchFolder { name: String, path: String },
    /// Remove folder for watching
    UnwatchFolder { path: String },
}
