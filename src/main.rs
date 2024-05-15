extern crate daemonize_me;
extern crate notify;

use std::process::exit;
use std::sync::Arc;
use env_logger::Env;
use log::{error, info};

use crate::daemon::daemon::set_up_daemon;
use crate::utils::config::Config;

mod daemon;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let daemon = set_up_daemon();

    match daemon.await {
        Ok(_) => {
            info!("Daemon finished successfully");
        }
        Err(e) => {
            error!("Error: {}", e);
            exit(-1);
        }
    }
}
