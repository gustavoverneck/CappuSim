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

fn main() {
    let nx = 128;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);
    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        let cx = nx as f32 / 6.0; // Circle center x
        let cy = nx as f32 / 2.0; // Circle center y
        let radius = nx as f32 / 10.0; // Circle radius

        lbm.velocity[n].x = 0.0;
        lbm.velocity[n].y = 0.0;
        lbm.velocity[n].z = 0.0;

        // Set density field
        lbm.density[n] = 1.0f32;

        // Set FLAG_SOLID for a circular obstacle
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;
        if dx * dx + dy * dy <= radius * radius {
            lbm.flags[n] = FLAG_SOLID;
            lbm.density[n] = 0.0f32;
        }

        // Set FLAG_EQ for x == 1
        if x == 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = 0.1; // Uniform horizontal velocity
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }

        // Set FLAG_EQ for x == nx - 1
        if x == nx - 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = 0.0; // Uniform horizontal velocity
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 0.0;
        }
    });
    lbm.set_output_interval(10);
    lbm.run(1000);
}