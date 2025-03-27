// examples/poiseuille.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;

fn main() {
    // Initialize LBM simulation with a 100x100 grid (2D)
    let mut lbm = LBM::new(100, 100, 1, "D2Q9".to_string(), 0.1);

    // Set initial conditions: constant velocity at the inlet (left boundary)
    lbm.set_conditions(|lbm, x, y, z, n| {
        if x == 0 {
            lbm.u[n].x = 0.1; // Set a constant velocity at the inlet
            lbm.u[n].y = 0.0;
        } else {
            lbm.u[n].x = 0.0; // Set velocity to zero elsewhere
            lbm.u[n].y = 0.0;
        }
    });

    // Run the simulation for 1000 time steps
    lbm.run(1000);

    // Output the results to a file
    lbm.output_to("output_channel_flow.csv").expect("Failed to write output file.");
}