use crate::watcher::watcher_command::WatcherCommand;
use crate::watcher::watcher_command::WatcherCommand::ChangeDetected;
use anyhow::{anyhow, Context, Result};
use log::{error, info, warn};
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc::Sender;

pub struct FolderWatcher {
    watcher: RecommendedWatcher,
}

impl FolderWatcher {
    pub fn new(tx: Sender<WatcherCommand>) -> Result<Self> {
        let watcher = recommended_watcher(move |res: std::result::Result<Event, notify::Error>| {
            let event = match res {
                Ok(event) => event,
                Err(_) => return,
            };

            let paths: Vec<String> = event
                .paths
                .iter()
                .filter_map(|path| path.to_str().map(|s| s.to_string()))
                .collect();

            if paths.is_empty() {
                return;
            }

            match tx.try_send(ChangeDetected { paths }) {
                Ok(_) => info!("Change detected"),
                Err(e) => error!("Failed to send message: {}", e),
            }
        })
        .map_err(|e| anyhow!("Failed to create a watcher: {}", e))?;

        Ok(Self { watcher })
    }

    pub fn watch_folder(&mut self, path: &str) -> Result<()> {
        self.watcher
            .watch(Path::new(path), RecursiveMode::Recursive)
            .map_err(|e| anyhow!("Failed to add folder for watching: {}", e))
    }
}
