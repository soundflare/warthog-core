/// Enum with commands from the IPC.
#[derive(Debug)]
pub enum IpcCommand {
    /// Add folder for watching
    WatchFolder { name: String, path: String },
}
