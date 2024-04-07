extern crate daemonize_me;
extern crate notify;

use std::fs::File;
use std::path::Path;
use std::process::exit;

use daemonize_me::Daemon;
use notify::{recommended_watcher, RecursiveMode, Watcher};

fn post_fork_parent(_: i32, _: i32) -> ! {
    let mut watcher = recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(
            Path::new("/Users/p.rojs/Desktop/DSC0632.png"),
            RecursiveMode::Recursive,
        )
        .expect("File not found");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    let stdout = File::create("./logs/warthog.out").unwrap();
    let stderr = File::create("./logs/warthog.err").unwrap();
    let daemon = Daemon::new()
        .pid_file("warthog.pid", Some(false))
        .umask(0o000)
        .work_dir(".")
        .stdout(stdout)
        .stderr(stderr)
        .setup_post_fork_parent_hook(post_fork_parent)
        .start();

    match daemon {
        Ok(_) => println!("Running in background..."),
        Err(e) => {
            eprintln!("Error, {}", e);
            exit(-1);
        },
    }
}