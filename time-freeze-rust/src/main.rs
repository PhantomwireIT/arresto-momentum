// main.rs

mod utils;
mod splash;
mod dashboard;
mod calendar;
mod ci_sim;
mod progress;
mod spinner;
mod logger;
mod animation;
mod notifications;
mod config;
mod errors;

use splash::run_splash;
use dashboard::run_dashboard;
use ci_sim::run_ci;
use spinner::spinner;
use logger::{init_logger, info, error};
use progress::run_progress;
use calendar::run_calendar;
use animation::breathe_ascii;
use notifications::{notify_info, notify_warn, notify_error};

fn main() {
    // Initialize logger
    init_logger();

    info("Starting Time-Freeze Application...");

    // Splash Banner (Breathing Ice-Blue)
    let ascii_banner = [
        "████████╗██╗ ███╗   ███╗███████╗",
        "╚══██╔══╝██║ ████╗ ████║██╔════╝",
        "   ██║   ██║ ██╔████╔██║█████╗  ",
        "   ██║   ██║ ██║╚██╔╝██║██╔══╝  ",
        "   ██║   ██║ ██║ ╚═╝ ██║███████╗",
        "   ╚═╝   ╚═╝ ╚═╝     ╚═╝╚══════╝",
    ];
    breathe_ascii(&ascii_banner, 3);

    // Dashboard display
    run_dashboard();

    // Simulate CI/CD steps
    info("Running CI/CD Simulation...");
    spinner(2000, "Executing CI/CD pipeline");
    run_ci();

    // Progress bar
    run_progress();

    // Weekly Calendar
    run_calendar();

    // Notifications examples
    notify_info("All modules loaded successfully!");
    notify_warn("This is a test warning.");
    notify_error("This is a test error.");

    println!("\x1b[1;32mAll systems frozen — Time-Freeze complete! ❄️\x1b[0m");
}




// main.rs

mod utils;
mod splash;
mod dashboard;
mod calendar;
mod ci_sim;
mod progress;
mod spinner;
mod logger;
mod animation;
mod notifications;
mod config;
mod errors;

use splash::run_splash;
use dashboard::run_dashboard;
use ci_sim::run_ci;
use spinner::spinner;
use logger::{init_logger, info, error};
use progress::run_progress;
use calendar::run_calendar;
use animation::breathe_ascii;
use notifications::{notify_info, notify_warn, notify_error};

fn main() {
    // Initialize logger
    init_logger();
    info("Starting Time-Freeze Application...");

    // Splash Banner (Breathing Ice-Blue)
    let ascii_banner = [
        "████████╗██╗ ███╗   ███╗███████╗",
        "╚══██╔══╝██║ ████╗ ████║██╔════╝",
        "   ██║   ██║ ██╔████╔██║█████╗  ",
        "   ██║   ██║ ██║╚██╔╝██║██╔══╝  ",
        "   ██║   ██║ ██║ ╚═╝ ██║███████╗",
        "   ╚═╝   ╚═╝ ╚═╝     ╚═╝╚══════╝",
    ];
    breathe_ascii(&ascii_banner, 3);

    // Dashboard display
    run_dashboard();

    // Simulate CI/CD steps
    info("Running CI/CD Simulation...");
    spinner(2000, "Executing CI/CD pipeline");
    run_ci();

    // Progress bar
    run_progress();

    // Weekly Calendar
    run_calendar();

    // Notifications examples
    notify_info("All modules loaded successfully!");
    notify_warn("This is a test warning.");
    notify_error("This is a test error.");

    println!("\x1b[1;32mAll systems frozen — Time-Freeze complete! ❄️\x1b[0m");
}
