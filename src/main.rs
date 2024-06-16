extern crate notify;

use std::sync::Arc;

use env_logger::Env;
use sqlx::migrate::Migrator;
use tokio::sync::mpsc::channel;
use tokio::sync::{broadcast, Mutex};

use crate::db::database::Database;
use crate::ipc::ipc_command::IpcCommand;
use crate::ipc::ipc_request_processor::IpcRequestProcessor;
use crate::ipc::ipc_service::IpcService;
use crate::utils::config::Config;
use crate::watcher::event_processor::EventProcessor;
use crate::watcher::folder_watcher::FolderWatcher;
use crate::watcher::watcher_command::WatcherCommand;

mod db;
mod ipc;
mod utils;
mod vcs;
mod watcher;

mod generated {
    #[rustfmt::skip]
    pub mod local;
    #[rustfmt::skip]
    pub mod pipe;
}

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config =
        Arc::new(Config::new().expect("Error collecting configuration from environment variables"));
    let database: Arc<Mutex<Database>> = Arc::new(Mutex::new(Database::new(config.clone()).await));
    MIGRATOR
        .run(&database.lock().await.pool)
        .await
        .expect("Error running migrations");
    database
        .lock()
        .await
        .populate_cache()
        .await
        .expect("Error populating cache");

    let (watcher_tx, watcher_rx) = channel::<WatcherCommand>(100);
    let (ipc_tx, ipc_rx) = broadcast::channel::<IpcCommand>(16);

    let mut folder_watcher =
        FolderWatcher::new(watcher_tx, ipc_rx).expect("Failed to create a folder watcher");
    tokio::spawn(async move {
        folder_watcher.watch_for_folders().await;
    });

    let mut request_processor = IpcRequestProcessor::new(ipc_tx.subscribe(), database.clone());
    tokio::spawn(async move {
        request_processor.process_commands().await;
    });

    let ipc_service = IpcService::new(ipc_tx);
    ipc_service
        .run(config.pipe_path.as_str())
        .await
        .expect("Failed to run IPC service");

    let mut event_processor = EventProcessor::new(watcher_rx, database.clone());
    event_processor.process_commands().await;
}
