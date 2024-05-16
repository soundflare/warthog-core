use crate::ipc::ipc_command::IpcCommand;
use crate::watcher::watcher_command::WatcherCommand;
use crate::watcher::watcher_command::WatcherCommand::ChangeDetected;
use anyhow::{anyhow, Result};
use log::{error, info};
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc::Sender;

pub struct FolderWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<IpcCommand>,
}

impl FolderWatcher {
    pub fn new(tx_watcher: Sender<WatcherCommand>, rx_ipc: Receiver<IpcCommand>) -> Result<Self> {
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

            match tx_watcher.try_send(ChangeDetected { paths }) {
                Ok(_) => info!("Change detected"),
                Err(e) => error!("Failed to send message: {}", e),
            }
        })
        .map_err(|e| anyhow!("Failed to create a watcher: {}", e))?;

        Ok(Self {
            watcher,
            rx: rx_ipc,
        })
    }

    pub async fn watch_for_folders(&mut self) {
        while let Ok(command) = self.rx.recv().await {
            match command {
                IpcCommand::WatchFolder { path, .. } => {
                    match self
                        .watcher
                        .watch(Path::new(&path), RecursiveMode::Recursive)
                        .map_err(|e| anyhow!("Failed to add folder for watching: {}", e))
                    {
                        Ok(_) => info!("Watching folder: {}", path),
                        Err(e) => error!("Error trying to watch folder: {}", e),
                    }
                }
                _ => (),
            }
        }
    }
}
