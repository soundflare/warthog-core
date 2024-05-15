/// Enum with commands from the watcher to the processor.
#[derive(Debug)]
pub enum WatcherCommand {
    /// Change detected in the file system.
    ChangeDetected { paths: Vec<String> },
}

/// Enum with commands from the IPC to the processor.
#[derive(Debug)]
pub enum IpcCommand {
    /// Add folder for watching
    WatchFolder { name: String, path: String },
}
