// src/examples/liddriven_cavity

// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode; 

// 2D Lid-driven Cavity Example
pub fn liddriven_cavity_2d_example() {
    let nx = 128;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1, PrecisionMode::FP32);

    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, _z, n| {
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
            lbm.velocity[n].x = -0.1; // Lid moving to the right
            lbm.velocity[n].y = 0.0;
        }
    });

    // Configure output
    lbm.set_output_interval(10);

    // Run the simulation
    lbm.run(10000);

    lbm.export_to_vtk("output/liddriven_cavity.vtk").expect("Failed to write output file.");
}