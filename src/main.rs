// src/main.rs

// Imports
#[allow(unused)]
mod core;
mod utils;

use core::lbm::LBM;

fn main() {
    // Initialize LBM simulation
    let mut lbm = LBM::new(100, 100, 100, "D2Q9".to_string(), 0.1);
    lbm.run(100);
}
