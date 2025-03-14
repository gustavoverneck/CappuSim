// src/main.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;

fn main() {
    // Initialize LBM simulation
    let mut lbm = LBM::new(512, 512, 1, "D2Q9".to_string(), 0.7);
    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        let pi = std::f32::consts::PI;
        lbm.u[n].x = -f32::cos(2.0 * pi * x as f32 / 100.0) * f32::sin(2.0 * pi * y as f32 / 100.0);
        lbm.u[n].y = f32::sin(2.0 * pi * x as f32 / 100.0) * f32::cos(2.0 * pi * y as f32 / 100.0);
        lbm.u[n].z = 0.0f32;
        lbm.density[n] = 1.0f32;
    });

    lbm.run(1000);
    lbm.output_to("output.csv").expect("Failed to write output file.");
}