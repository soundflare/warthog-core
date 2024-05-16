/// Enum with commands from the watcher.
#[derive(Debug)]
pub enum WatcherCommand {
    /// Change detected in the file system.
    ChangeDetected { paths: Vec<String> },
}
