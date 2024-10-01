use clap::Parser;
use eframe::egui;
use notify_rust::Notification;
use std::{io, io::Write, thread, time::Duration, time::Instant};

#[derive(Parser)]
struct Args {
    /// Work duration in minutes (default: 25)
    #[arg(short, long, default_value_t = 25)]
    work: u64,

    /// Short break duration in minutes (default: 5)
    #[arg(short, long, default_value_t = 5)]
    break_time: u64,

    /// Long break duration in minutes (default: 15)
    #[arg(short, long, default_value_t = 15)]
    long_break: u64,

    /// Number of work sessions before long break (default: 4)
    #[arg(short = 'n', long, default_value_t = 4)]
    sessions: u32,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();
}

// Application states
struct PomodoroApp {
    timing: Args,
    session: str,
    seconds: u32,
    running: bool,
    last_tick: Instant,
    state_index: u32,
}

impl PomodoroApp {
    fn new(self: _, args: Args) -> Self {
        Self {
            timing: args,
            session: "Work",
            seconds: args.work,
            running: false,
            last_tick: Instant::now(),
            state_index: 0,
        }
    }

    fn start_timer(&mut self) {
        self.running = true;
    }
    fn stop_timer(&mut self) {
        self.running = false;
    }
    fn next_timer(&mut self) {
        // TODO maybe disable pause on next timer?
        self.stop_timer();

        self.state_index += 1;
        if (self.state_index >= self.timing.sessions * 2) {
            // All sessions finished
            self.session = "Finished, Good Job!";
            self.seconds = 0;
        } else if (self.state_index % 2 == 1) {
            // break
            if (self.state_index + 1 == self.timing.sessions * 2) {
                // Long break
                self.session = "Long break";
                self.seconds = self.timing.long_break * 60;
            } else {
                // Short break
                self.session = "Short Break";
                self.seconds = self.timing.break_time * 60;
            }
        } else {
            // work time
            self.session = "Work";
            self.seconds = self.timing.work * 60;
        }
    }
    fn notify(self) {
        let _ = Notification::new()
            .summary("Pomodoro: Timer Finished")
            .body(&format!("{} finished!", self.session))
            .icon("dialog-information")
            .timeout(0)
            .show();
    }
}

/*
fn run_timer(minutes: u64, label: &str) {
    let total_seconds = minutes * 60;
    for remaining_seconds in (0..=total_seconds - 1).rev() {
        let minutes_left = remaining_seconds / 60;
        let seconds_left = remaining_seconds % 60;
        print!("\r{}: {:02}:{:02} remaining", label, minutes_left, seconds_left);
        io::stdout().flush().unwrap(); // Actually print
        thread::sleep(Duration::from_secs(1));
    }
    println!();
    let _ = Notification::new()
    .summary("Pomodoro: Timer Finished")
    .body(&format!("{} finished!", label))
    .icon("dialog-information")
    .timeout(0)
    .show();

}
*/
