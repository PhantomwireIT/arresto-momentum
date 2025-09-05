// splash.rs
mod animation;
use animation::breathe_ascii;

pub fn run_splash() {
    let ascii_banner = [
        "████████╗██╗ ███╗   ███╗███████╗",
        "╚══██╔══╝██║ ████╗ ████║██╔════╝",
        "   ██║   ██║ ██╔████╔██║█████╗  ",
        "   ██║   ██║ ██║╚██╔╝██║██╔══╝  ",
        "   ██║   ██║ ██║ ╚═╝ ██║███████╗",
        "   ╚═╝   ╚═╝ ╚═╝     ╚═╝╚══════╝",
        "███████╗ ██████╗    ███████╗   ███████╗   █████████╗   ███████╗",
        "██╔════╝ ██╔══ ██  ╗██╔════╝   ██╔════╝        ╚██╗    ██╔════╝",
        "█████╗   ██████╔   ╝█████╗     █████╗         ██╔╝     █████╗  ",
        "██╔══╝   ██╔═ ██╔   ██ ══╝     ██╔══╝       ██╔╝       ██╔══╝  ",
        "██║      ██║   ██   ███████╗   ███████╗   ██████████╝  ███████╗",
        "╚═╝      ╚═╝    ╚═   ╚══════╝   ╚══════╝   ╚════════╝   ╚══════╝",
    ];

    // Run breathing animation for 3 cycles
    breathe_ascii(&ascii_banner, 3);

    // Optional extra effect: simple spinner
    run_spinner();
}

fn run_spinner() {
    let spinner = ["|", "/", "-", "\\"];
    for _ in 0..6 {
        for s in &spinner {
            print!("\r\x1b[1;96m{} Loading...\x1b[0m", s);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    println!("\r\x1b[1;32mReady!           \x1b[0m");
}
