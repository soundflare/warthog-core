extern crate notify;

use env_logger::Env;
use std::sync::Arc;
use sqlx::migrate::Migrator;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use crate::db::database::Database;
use crate::processor::processor::Processor;

use crate::utils::config::Config;
use crate::watcher::folder_watcher::FolderWatcher;

mod processor;
mod tus;
mod utils;
mod vcs;
mod watcher;
mod db;
mod ipc;

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

    let (tx, mut rx) = channel(100);
    let mut watcher = FolderWatcher::new(tx).expect("Failed to create a folder watcher");
    let mut processor = Processor::new(rx);

    // TODO: For testing purposes - remove
    watcher
        .watch_folder("/Users/p.rojs/Desktop/sample")
        .expect("TODO: panic message");

    processor.process_commands().await;
}
