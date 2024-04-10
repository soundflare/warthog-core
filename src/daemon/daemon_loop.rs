use std::fs::File;
use std::path::Path;

use daemonize_me::{Daemon, DaemonError};
use notify::{recommended_watcher, RecursiveMode, Watcher};

pub fn set_up_daemon() -> Result<(), DaemonError> {
    let stdout = File::create("./logs/warthog.out").unwrap();
    let stderr = File::create("./logs/warthog.err").unwrap();
    return Daemon::new()
        .pid_file("warthog.pid", Some(false))
        .umask(0o000)
        .work_dir(".")
        .stdout(stdout)
        .stderr(stderr)
        .setup_post_fork_parent_hook(post_fork_parent)
        .start();
}

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
        println!("Hi ho, hi ho, it's off to work we go...")
    }
}