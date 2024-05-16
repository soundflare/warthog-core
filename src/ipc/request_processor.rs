use crate::db::database::Database;
use crate::ipc::ipc_command::IpcCommand;
use anyhow::Result;
use log::{error, info};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::{create_dir_all, File};
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;

pub struct RequestProcessor {
    rx: Receiver<IpcCommand>,
    database: Arc<Mutex<Database>>,
}

impl RequestProcessor {
    pub fn new(rx: Receiver<IpcCommand>, database: Arc<Mutex<Database>>) -> Self {
        Self { rx, database }
    }

    pub async fn process_commands(&mut self) {
        while let Ok(command) = self.rx.recv().await {
            match command {
                IpcCommand::WatchFolder { name, path } => {
                    self.handle_watch_folder(name, path).await
                }
                IpcCommand::UnwatchFolder { path } => self.handle_unwatch_folder(path).await,
            }
        }
    }

    async fn handle_watch_folder(&self, name: String, path: String) {
        info!("Received watch folder event");

        match self
            .database
            .lock()
            .await
            .insert_project(name.as_str(), path.as_str())
            .await
        {
            Ok(_) => info!("Successfully inserted project"),
            Err(err) => error!("Failed to insert project: {:?}", err),
        }

        match self.create_metadata_folder(path).await {
            Ok(_) => info!("Successfully created metadata folder"),
            Err(err) => error!("Failed to create metadata folder: {:?}", err),
        }
    }

    async fn handle_unwatch_folder(&self, paths: String) {
        info!("Received unwatch folder event");

        match self
            .database
            .lock()
            .await
            .remove_project(paths.as_str())
            .await
        {
            Ok(_) => info!("Successfully removed project"),
            Err(err) => error!("Failed to remove project: {:?}", err),
        }
    }

    async fn create_metadata_folder(&self, path: String) -> Result<()> {
        info!("Creating metadata folder for path: {}", path);
        let warthog_dir_path = Path::new(&path).join(".warthog");
        create_dir_all(&warthog_dir_path).await?;

        let warthfile_path = warthog_dir_path.join("Warthfile");
        File::create(warthfile_path).await?;
        Ok(())
    }
}
