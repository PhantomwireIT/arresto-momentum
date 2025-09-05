// spinner.rs
use std::{thread, time};

pub fn spinner(duration_ms: u64, message: &str) {
    let symbols = ['|', '/', '-', '\\'];
    let sleep_time = time::Duration::from_millis(100);
    let cycles = duration_ms / 100;

    for i in 0..cycles {
        print!("\r\x1b[1;34m{} {}\x1b[0m", symbols[i as usize % 4], message);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(sleep_time);
    }
    println!("\r\x1b[1;32m✔ {} complete!\x1b[0m", message);
}
