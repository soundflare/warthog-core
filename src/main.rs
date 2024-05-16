extern crate notify;

use crate::db::database::Database;
use crate::ipc::ipc_command::IpcCommand;
use env_logger::Env;
use sqlx::migrate::Migrator;
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
// use crate::ipc::ipc_service::IpcService;

use crate::utils::config::Config;
use crate::watcher::folder_watcher::FolderWatcher;
use crate::watcher::watcher_command::WatcherCommand;
use crate::watcher::watcher_processor::WatcherProcessor;

mod db;
mod ipc;
mod tus;
mod utils;
mod vcs;
mod watcher;
mod protos {
    pub mod pipe {
        include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
    }
}

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Arc::new(Config::new().unwrap());
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
    let (ipc_tx, ipc_rx) = channel::<IpcCommand>(100);

    let mut watcher = FolderWatcher::new(watcher_tx).expect("Failed to create a folder watcher");
    let mut watcher_processor = WatcherProcessor::new(watcher_rx, database.clone());
    // let mut ipc_service = IpcService::new(ipc_tx);

    // TODO: For testing purposes - remove
    watcher
        .watch_folder("/Users/p.rojs/Desktop/sample")
        .expect("TODO: panic message");

    watcher_processor.process_commands().await;
}
