

use std::{thread, time};

pub fn breathe_ascii(ascii: &[&str], cycles: u32) {
    for _ in 0..cycles {
        // bright cyan
        for line in ascii {
            println!("\x1b[1;96m{}\x1b[0m", line);
        }
        thread::sleep(time::Duration::from_millis(400));
        clear_screen();

        // dim cyan
        for line in ascii {
            println!("\x1b[0;96m{}\x1b[0m", line);
        }
        thread::sleep(time::Duration::from_millis(400));
        clear_screen();
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H"); // ANSI escape for clear + move cursor home
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
