use crate::processor::command::Command;
use crate::processor::command::Command::ChangeDetected;
use anyhow::Result;
use log::{info, warn};
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc::Sender;

pub struct FolderWatcher {
    watcher: RecommendedWatcher,
}

impl FolderWatcher {
    pub fn new(tx: Sender<Command>) -> Result<Self> {
        let watcher = recommended_watcher(move |res: std::result::Result<Event, notify::Error>| {
            match tx.try_send(ChangeDetected {
                paths: res
                    .unwrap()
                    .paths
                    .iter()
                    .map(|path| path.to_str().expect("Path is not valid UTF-8").to_string())
                    .collect(),
            }) {
                Ok(_) => info!("Change detected"),
                Err(e) => warn!("Failed to send message: {:?}", e),
            }
        })
        .map_err(|e| anyhow::anyhow!(e));

        match watcher {
            Ok(watcher) => Ok(Self { watcher }),
            Err(e) => Err(anyhow::anyhow!("Failed to create watcher: {}", e)),
        }
    }

    pub fn watch_folder(&mut self, path: &str) -> Result<()> {
        self.watcher
            .watch(Path::new(path), RecursiveMode::Recursive)
            .map_err(|e| anyhow::anyhow!(e))
    }
}
