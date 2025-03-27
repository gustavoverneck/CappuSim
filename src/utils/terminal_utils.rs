/// Utility functions for terminal output with styled messages.
///
/// This module provides functions to print styled messages to the terminal,
/// including warnings, logs, success messages, errors, and other formatted outputs.

use colored::*;

// Print a WARNING message with a prefix in bold yellow.
#[allow(dead_code)]
pub fn print_warning(message: &str) {
    println!("{}: {}", "[WARNING]".yellow().bold(), message);
}

// Print a simple LOG message with a prefix in bold white.
#[allow(dead_code)]
pub fn print_log(message: &str) {
    println!("{}: {}", "[LOG]".white().bold(), message);
}

// Print a SUCCESS message with a prefix in bold green.
pub fn print_success(message: &str) {
    println!("{}: {}", "[SUCCESS]".green().bold(), message);
}

// Print an ERROR message with a prefix in bold red.
#[allow(dead_code)]
pub fn print_error(message: &str) {
    println!("{}: {}", "[ERROR]".red().bold(), message);
}

// Print LatteLab welcome message
pub fn print_welcome_message() {
    println!("{}", "-".repeat(72));
    println!("{}", r#"
  _           _   _       _           _     
 | |         | | | |     | |         | |    
 | |     __ _| |_| |_ ___| |     __ _| |__  
 | |    / _` | __| __/ _ \ |    / _` | '_ \ 
 | |___| (_| | |_| ||  __/ |___| (_| | |_) |
 |______\__,_|\__|\__\___|______\__,_|_.__/ 
                                            
                                               Gustavo A. Verneck, 2025.
"#.blue().bold());
}


pub fn print_metrics(time_steps: u64, elapsed_time: f64, mlups: f64) {
    let seconds = elapsed_time;
    let days = seconds as i64 / 86400;
    let hours = (seconds as i64 % 86400) / 3600;
    let minutes = (seconds as i64 % 3600) / 60;
    let seconds = seconds % 60.0;
    println!("\n");
    println!("{}", "-".repeat(72));
    print_success("Simulation finished successfully!"); 
    println!(
        "Elapsed time: {}d {}h {}m {:.3 }s",
        days,
        hours,
        minutes,
        seconds
    );
    println!("{} time steps", time_steps);
    println!("{}: {:.2} MLUps\n", "Performance".white().bold(), mlups);
}

pub fn print_name() {
    println!("\n{}", "LatteLab".bold().blue());
}