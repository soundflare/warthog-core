extern crate daemonize_me;
extern crate notify;

use std::process::exit;

use crate::daemon::daemon_loop::set_up_daemon;

mod daemon;

fn main() {
    let daemon = set_up_daemon();

    match daemon {
        Ok(_) => println!("Running in background..."),
        Err(e) => {
            eprintln!("Error, {}", e);
            exit(-1);
        },
    }
}