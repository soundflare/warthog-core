use crate::db::database::Database;
use crate::ipc::ipc_command::IpcCommand;
use crate::vcs::version_control::create_repository;
use log::{error, info};
use std::sync::Arc;
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

        match create_repository(path).await {
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
}
