use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_logger() {
    println!("\x1b[1;34mLogger initialized\x1b[0m");
}

pub fn info(msg: &str) {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("\x1b[1;36m[INFO {}] {}\x1b[0m", timestamp, msg);
}

pub fn error(msg: &str) {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    eprintln!("\x1b[1;31m[ERROR {}] {}\x1b[0m", timestamp, msg);
}
