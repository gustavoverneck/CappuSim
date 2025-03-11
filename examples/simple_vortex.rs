// examples/simple_vortex.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;


fn main() {
    // Initialize LBM simulation with a 100x100 grid (2D)
    let mut lbm = LBM::new(100, 100, 1, "D2Q9".to_string(), 0.1);

    // Set initial conditions: create a vortex in the center of the domain
    lbm.set_conditions(|lbm, x, y, z, n| {
        let center_x = 50.0;
        let center_y = 50.0;
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let radius = (dx * dx + dy * dy).sqrt();

        if radius < 30.0 {
            lbm.u[n].x = -dy / radius * 0.1; // Tangential velocity for vortex
            lbm.u[n].y = dx / radius * 0.1;
        }
    });

    // Run the simulation for 1000 time steps
    lbm.run(1000);

    // Output the results to a file
    lbm.output_to("output_vortex_flow.csv").expect("Failed to write output file.");
}