use std::io::{stdout, Write};
use std::{thread, time};

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H"); // Clear screen and move cursor to home
    stdout().flush().unwrap(); // Flush to show immediately
}

pub fn pause_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

pub fn cyan_bright(text: &str) -> String {
    format!("\x1b[1;96m{}\x1b[0m", text)
}

pub fn cyan_dim(text: &str) -> String {
    format!("\x1b[0;96m{}\x1b[0m", text)
}

pub fn green_bright(text: &str) -> String {
    format!("\x1b[1;32m{}\x1b[0m", text)
}
