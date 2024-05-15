extern crate notify;

use env_logger::Env;
use log::{error, info};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::utils::config::Config;

mod services;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Arc::new(Config::new().unwrap());
    info!("Starting Warthog");

    let (sender, receiver) = mpsc::channel();

    let mut watcher = recommended_watcher(move |res| match res {
        Ok(event) => {
            sender.send(event).unwrap();
        }
        Err(e) => info!("Watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(
            Path::new("/Users/p.rojs/Desktop/DSC0632.png"),
            RecursiveMode::Recursive,
        )
        .expect("File not found");

    thread::spawn(move || loop {
        if let Ok(event) = receiver.recv() {
            info!("Received event: {:?}", event);
        }
    });

    loop {
        info!("Running...");
    }
}
