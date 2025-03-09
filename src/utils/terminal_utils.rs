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