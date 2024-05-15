extern crate daemonize_me;
extern crate notify;

use std::process::exit;
use env_logger::Env;
use log::info;

use crate::daemon::daemon_loop::set_up_daemon;

mod daemon;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let daemon = set_up_daemon();

    match daemon.await {
        Ok(_) => {
            info!("Daemon started successfully");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(-1);
        }
    }
}
