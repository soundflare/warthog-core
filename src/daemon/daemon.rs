use std::path::Path;
use std::sync::{Arc, mpsc};
use std::thread;

use daemonize_me::{Daemon, DaemonError};
use log::info;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use crate::utils::config::Config;

pub async fn set_up_daemon() -> Result<(), DaemonError> {
    return Daemon::new()
        .pid_file("warthog.pid", Some(false))
        .umask(0o000)
        .work_dir(".")
        .setup_post_fork_parent_hook(post_fork_parent)
        .start();
}

fn post_fork_parent(_: i32, _: i32) -> ! {
    let config = Arc::new(Config::new().unwrap());
    let (sender, receiver) = mpsc::channel();

    let mut watcher = recommended_watcher(move |res| match res {
        Ok(event) => {
            sender.send(event).unwrap();
        }
        Err(e) => info!("watch error: {:?}", e),
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

    // The main thread waits for the daemonization to handle process management.
    loop {}
}
