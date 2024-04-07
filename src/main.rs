use std::{fs, thread};
use std::time::{Duration, SystemTime};

fn main() {
    let path = "/Users/p.rojs/Desktop/DSC06326.png";
    let mut last_modified = get_last_modified_time(path);

    loop {
        thread::sleep(Duration::from_secs(5)); // Adjust the delay as needed
        let current_modified = get_last_modified_time(path);

        if current_modified != last_modified {
            println!("File has been modified.");
            last_modified = current_modified;
        }
    }
}

fn get_last_modified_time(path: &str) -> SystemTime {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .unwrap_or(SystemTime::now())
}
