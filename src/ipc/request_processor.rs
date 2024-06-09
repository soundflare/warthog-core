use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::{error, info};
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;

use crate::db::database::Database;
use crate::ipc::ipc_command::IpcCommand;
use crate::vcs::version_control::create_repository;

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
                IpcCommand::WatchFolder { path } => {
                    self.handle_watch_folder(&path).await.unwrap_or_else(|e| {
                        error!("Failed to handle watch folder command: {:?}", e)
                    });
                }
                IpcCommand::UnwatchFolder { path } => {
                    self.handle_unwatch_folder(&path).await.unwrap_or_else(|e| {
                        error!("Failed to handle unwatch folder command: {:?}", e)
                    });
                }
            }
        }
    }

    async fn handle_watch_folder(&self, path: &PathBuf) -> Result<()> {
        info!("Received watch folder event");
        let name_str = path
            .file_stem()
            .ok_or(anyhow!("Failed to get file stem"))?
            .to_str()
            .ok_or(anyhow!("Failed to get str from file stem"))?;

        let path_str = path
            .as_path()
            .to_str()
            .ok_or(anyhow!("Failed to get path string"))?;

        self.database
            .lock()
            .await
            .insert_project(name_str, path_str)
            .await?;
        info!("Successfully added project to database");

        create_repository(path).await
    }

    async fn handle_unwatch_folder(&self, path: &PathBuf) -> Result<()> {
        info!("Received unwatch folder event");
        let path_str = path
            .as_path()
            .to_str()
            .ok_or(anyhow!("Failed to get path string"))?;

        match self.database.lock().await.remove_project(path_str).await {
            Ok(_) => {
                info!("Successfully removed project from database");
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
