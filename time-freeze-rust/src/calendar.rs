// calendar.rs
use crate::utils::{green_bright, cyan_bright};
use chrono::prelude::*;

pub fn run_calendar() {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let today_index = Local::today().weekday().num_days_from_monday() as usize;

    // Colored day names
    let colored_days = days
        .iter()
        .enumerate()
        .map(|(i, day)| {
            if i < today_index {
                green_bright(*day)  // Past days
            } else if i == today_index {
                cyan_bright(*day)   // Today
            } else {
                day.to_string()     // Future
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    // Symbols for visual calendar
    let symbols = (0..7)
        .map(|i| if i < today_index { "✓" } else if i == today_index { "→" } else { "✗" })
        .collect::<Vec<&str>>()
        .join(" ");

    println!("\x1b[1;35m==== Weekly Calendar ====\x1b[0m");
    println!("{}", colored_days);
    println!("[{}]", symbols);
    println!(); // extra spacing
}
