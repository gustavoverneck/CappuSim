// src/main.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;
use core::lbm::{FLAG_FLUID, FLAG_SOLID, FLAG_EQ};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Lid-driven Cavity example
fn main() {
    let nx = 128;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);

    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        lbm.velocity[n].x = 0.0;
        lbm.velocity[n].y = 0.0;
        lbm.velocity[n].z = 0.0;
        lbm.density[n] = 1.0f32;

        // Set FLAG_SOLID for the walls
        if y == 0 || y == nx - 1 || x == 0 || x == nx - 1 {
            lbm.flags[n] = FLAG_SOLID;
        }

        // Set FLAG_EQ for the top lid with a constant velocity
        if y == nx - 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = 0.1; // Lid moving to the right
            lbm.velocity[n].y = 0.0;
        }
    });

    lbm.set_output_interval(10);
    lbm.run(100);

    lbm.output_to("output.csv").expect("Failed to write output file.");
}