// src/examples/couette

// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;

pub fn couette_2d_example() {
    let nx = 256;
    let ny = 64;
    let nz = 1;
    let viscosity = 0.05;
    let u0 = 0.1;
    let steps = 20000;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, "D2Q9".to_string(), viscosity);

    // Set boundary and initial conditions for Couette flow
    lbm.set_conditions(|lbm, _x, y, _z, n| {
        if y == 0 {
            // Bottom wall: stationary
            lbm.flags[n] = FLAG_SOLID;
            lbm.velocity[n].x = 0.0;
            lbm.velocity[n].y = 0.0;
        } else if y == ny - 1 {
            // Top wall: moving with velocity u0
            lbm.flags[n] = FLAG_SOLID;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
        } else {
            // Interior: fluid
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = 0.0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
    });

    // Run the simulation
    lbm.run(steps);
    lbm.export_to_vtk("output/couette.vtk").unwrap();
}
