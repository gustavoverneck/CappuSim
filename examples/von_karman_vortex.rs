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
    let nx = 512;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);
    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        let pi = std::f32::consts::PI;
        let cx = nx as f32 / 4.0; // Circle center x
        let cy = nx as f32 / 2.0; // Circle center y
        let radius = nx as f32 / 10.0; // Circle radius

        // Set velocity field for von Karman vortex
        lbm.u[n].x = -f32::cos(2.0 * pi * x as f32 / nx as f32) * f32::sin(2.0 * pi * y as f32 / nx as f32);
        lbm.u[n].y = f32::sin(2.0 * pi * x as f32 / nx as f32) * f32::cos(2.0 * pi * y as f32 / nx as f32);
        lbm.u[n].z = 0.0f32;

        // Set density field
        lbm.density[n] = 1.0f32 - 0.25 * (f32::cos(4.0 * pi * x as f32 / nx as f32) + f32::cos(4.0 * pi * y as f32 / nx as f32));

        // Set FLAG_SOLID for a circular obstacle
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;
        if dx * dx + dy * dy <= radius * radius {
            lbm.flags[n] = FLAG_SOLID;
        }
    });

    lbm.run(1);

    lbm.output_to("output.csv").expect("Failed to write output file.");
}