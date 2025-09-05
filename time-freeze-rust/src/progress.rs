use crate::utils::{green_bright, cyan_bright};

pub fn run_progress() {
    let total_steps = 100;
    for i in 0..=total_steps {
        let bar = format!(
            "[{}{}] {}%",
            "=".repeat(i / 2), // Progress bar filled
            " ".repeat((total_steps - i) / 2), // Remaining space
            i
        );

        if i < total_steps {
            println!("{}", cyan_bright(&bar)); // Simulating progress in cyan
        } else {
            println!("{}", green_bright(&bar)); // Final step in green
        }

        std::thread::sleep(std::time::Duration::from_millis(50)); // simulate progress
    }
}
