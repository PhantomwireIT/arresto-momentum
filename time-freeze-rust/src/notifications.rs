pub fn notify_info(msg: &str) {
    println!("\x1b[1;36m[NOTIFY]\x1b[0m {}", msg);
}

pub fn notify_warn(msg: &str) {
    println!("\x1b[1;33m[WARNING]\x1b[0m {}", msg);
}

pub fn notify_error(msg: &str) {
    eprintln!("\x1b[1;31m[ERROR]\x1b[0m {}", msg);
}
