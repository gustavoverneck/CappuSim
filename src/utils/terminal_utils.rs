use colored::*;

// Print an ERROR message with a prefix in bold red.
pub fn print_error(message: &str) {
    println!("{}: {}", "[ERROR]".red().bold(), message);
}

// Print a WARNING message with a prefix in bold yellow.
pub fn print_warning(message: &str) {
    println!("{}: {}", "[WARNING]".yellow().bold(), message);
}

// Print a simple LOG message with a prefix in bold white.
pub fn print_log(message: &str) {
    println!("{}: {}", "[LOG]".white().bold(), message);
}

// Print a SUCCESS message with a prefix in bold green.
pub fn print_success(message: &str) {
    println!("{}: {}", "[SUCCESS]".green().bold(), message);
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
    println!("{}", "Done!".green().bold());
    println!(
        "Elapsed time: {}d {}h {}m {:.2 }s",
        days,
        hours,
        minutes,
        seconds
    );
    println!("{} time steps", time_steps);
    println!("{}: {:.2} MLUps\n", "Performance".white().bold(), mlups);
}


pub fn print_opencl_success() {
    println!("{}", "OpenCL device and context initialized successfully!");
}

pub fn print_yellow_name() {
    println!("\n{}", "LatteLab".bold().blue());
}