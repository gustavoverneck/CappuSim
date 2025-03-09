// src/main.rs

// Imports
#[allow(unused)]
mod core;
mod utils;

use utils::terminal_utils::print_welcome_message;
use core::lbm::LBM;

fn main() {
    print_welcome_message();

    let mut lbm = LBM::new(100, 100, 100, "D2Q9".to_string(), 0.1);
    lbm.run(100);
}
