use rand::Rng;

pub fn run_dashboard() {
    let mut rng = rand::thread_rng();
    let habits = vec![
        ("Code Daily", rng.gen_range(0..=100)),
        ("Exercise", rng.gen_range(0..=100)),
        ("Read", rng.gen_range(0..=100)),
        ("Meditate", rng.gen_range(0..=100)),
        ("Sleep Well", rng.gen_range(0..=100)),
    ];

    println!("\x1b[1;34m==== Habit Tracker Dashboard ====\x1b[0m");
    for (habit, progress) in habits {
        let bars = "#".repeat(progress / 10);
        let empty = "-".repeat(10 - bars.len());
        println!("{:<12} [{}{}] {:3}%", habit, bars, empty, progress);
    }
    println!("\x1b[1;34m==== Time-Freeze Dashboard ====\x1b[0m");
    println!("System Status:   \x1b[1;32mOK\x1b[0m");
    println!("Active Modules:  Splash, CI/CD, Progress, Calendar");
    println!("Notifications:   \x1b[1;33mNo new alerts\x1b[0m");
    println!("Config Loaded:   \x1b[1;36mtime-freeze.yml\x1b[0m\n");
    println!("\n\x1b[1;36mInsights:\x1b[0m");
    println!("- Dominate your day! ❄️");
    println!("- Balance is key ⚡");
    println!("- Commit often, level up 🏆");
    println!("- Stay frosty! 🧊\n");
}
