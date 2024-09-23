use std::{io::self, io::Write, thread, time::Duration};
use clap::Parser;
use notify_rust::Notification;

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

fn main() {
    let args = Args::parse();
    for session in 1..=args.sessions {
        println!("Session {}/{}: Time to work for {} minutes", session, args.sessions, args.work);
        run_timer(args.work, "Work");
    
        if session == args.sessions {
            println!("Time for a long break of {} minutes!", args.long_break);
            run_timer(args.long_break, "Long Break");

        } else {
            println!("Time for a short break of {} minutes!", args.break_time);
            run_timer(args.break_time, "Short Break");
        }
    }
    println!("Pomodoro complete! Good job!");
}

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
    .summary("POMODORO: TIMER FINISHED")
    .body(&format!("{} finished!", label))
    .icon("dialog-information")
    .timeout(0)
    .show();
}