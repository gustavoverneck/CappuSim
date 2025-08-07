// src/examples/von_karman
#![allow(dead_code)]
#![allow(unused_imports)]
// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode; 

// 2D Von-Kármán Vortex Street Example
pub fn von_karman_vortex_2d_example() {
    let nx = 256;
    let ny = 128;
    let viscosity = 0.01;
    let u0 = 0.1;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity, PrecisionMode::FP32);

    // Cylinder parameters
    let radius = nx as f32 * 0.08;
    let cx = nx as i32 / 4; // 25% from left
    let cy = ny as i32 / 2;

    // Set boundary and initial conditions
    lbm.set_conditions(|lbm, x, y, _z, n| {
        let dx = x as i32 - cx;
        let dy = y as i32 - cy;
        let dist = ((dx * dx + dy * dy) as f32).sqrt();

        if dist <= radius {
            lbm.flags[n] = FLAG_SOLID; // Cylinder obstacle
        } else if x == 0 {
            // Inlet with prescribed velocity
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        } else if x == nx - 1 {
            // Outflow: still FLAG_EQ for now, but zero-velocity to reduce reflection
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        } else if y == 0 || y == ny - 1 {
            // Top and bottom walls
            lbm.flags[n] = FLAG_SOLID;
        } else {
            // Normal fluid region
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
    });

    // Configure output
    lbm.set_output_vtk(true);
    lbm.set_output_interval(50);

    // Run the simulation
    lbm.run(10000);
}