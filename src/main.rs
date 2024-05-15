extern crate notify;

use env_logger::Env;
use log::info;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::channel;

use crate::utils::config::Config;
use crate::watcher::folder_watcher::FolderWatcher;

mod communication;
mod tus;
mod utils;
mod vcs;
mod watcher;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Arc::new(Config::new().unwrap());
    let (tx, mut rx) = channel(100);
    let mut watcher = FolderWatcher::new(tx).expect("Failed to create a folder watcher");

    // TODO: For testing purposes - remove
    watcher
        .watch_folder("/Users/p.rojs/Desktop/sample")
        .expect("TODO: panic message");

    thread::spawn(move || loop {
        // if let Ok(event) = rx.recv() {
        //     info!("Received event: {:?}", event);
        // }
    });

    loop {
        info!("Running...");
    }
}
