use clap::Parser;
use eframe::egui;
use notify_rust::Notification;
use std::{time::Duration, time::Instant};


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
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_cc| Ok(Box::new(PomodoroApp::new()))),  // Updated line
    )
}

// Application states
struct PomodoroApp {
    timing: Args,
    session: String,
    seconds: u64,
    running: bool,
    last_tick: Instant,
    state_index: u32,
}

impl PomodoroApp {
    fn new() -> Self {
        Self {
            timing: Args::parse(),
            session: "Work".to_string(),
            seconds: Args::parse().work * 60,
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
        if self.state_index >= self.timing.sessions * 2 {
            // All sessions finished
            self.session = "Finished, Good Job!".to_string();
            self.seconds = 0;
        } else if self.state_index % 2 == 1 {
            // break
            if self.state_index + 1 == self.timing.sessions * 2 {
                // Long break
                self.session = "Long break".to_string();
                self.seconds = self.timing.long_break * 60;
            } else {
                // Short break
                self.session = "Short Break".to_string();
                self.seconds = self.timing.break_time * 60;
            }
        } else {
            // work time
            self.session = "Work".to_string();
            self.seconds = self.timing.work * 60;
        }
    }
    fn notify(&self) {
        let _ = Notification::new()
            .summary("Pomodoro: Timer Finished")
            .body(&format!("{} finished!", self.session))
            .icon("dialog-information")
            .timeout(0)
            .show();
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(format!("Pomodoro {} Timer", self.session));
            ui.label(format!("Time remaining: {:02}:{:02}", self.seconds / 60, self.seconds % 60));


        ui.horizontal(|ui| {
        if self.running { // Replace button with start or stop label
            if ui.button("Stop").clicked() {
                self.stop_timer();
            }
        } else {
            if ui.button("Start").clicked() {
                self.start_timer();
            }
        }
        if ui.button("Skip Session").clicked() {
            self.next_timer();
        }
    });

    if self.seconds <= 0 {
        self.next_timer();
        self.notify();
    }
        
    if self.running && self.last_tick.elapsed() >= Duration::from_secs(1) {
            self.seconds -= 1;
            self.last_tick = Instant::now();
        }



    });
    ctx.request_repaint();

    }
}

