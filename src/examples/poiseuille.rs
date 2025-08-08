// src/examples/poiseuille

// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode; 

pub fn poiseuille_2d_example() {
    let nx = 512;
    let ny = 128;
    let nz = 1;
    let viscosity = 0.1;
    let u0 = 0.1;
    let steps = 100000;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, "D2Q9".to_string(), viscosity, PrecisionMode::FP32);

    // Set boundary and initial conditions
    lbm.set_conditions(|lbm, x, y, _z, n| {
        if y == 0 || y == ny - 1 {
            // Top and bottom walls
            lbm.flags[n] = FLAG_SOLID;
        } else if x == 0 && y < ny - 1 {
            // Inlet: left boundary
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        } else if x == nx - 1 && y < ny - 1 {
            // Outlet: right boundary
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        } else {
            // Interior: fluid
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
    });

    // Configure output
//    lbm.set_output_vtk(true);
//    lbm.set_output_interval(1000);

    // Run the simulation
    lbm.run(steps);
    lbm.export_to_vtk("output/poiseuille.vtk").unwrap();
}


// 3D Poiseuille Example
pub fn poiseuille_3d_example() {
    let nx = 64;
    let ny = 32;
    let nz = 64;
    let viscosity = 0.01;
    let fz = 1e-6; // Small body force in z-direction

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, "D3Q19".to_string(), viscosity, PrecisionMode::FP32);

    // Set boundary and initial conditions
    lbm.set_conditions(|lbm, _x, y, _z, n| {
        if y == 0 || y == ny - 1 {
            lbm.flags[n] = FLAG_SOLID; // No-slip top and bottom walls
        } else {
            lbm.flags[n] = FLAG_FLUID;

            // Approximate initial velocity profile (optional)
            let y_f = y as f32;
            let h = ny as f32;
            let uz = (fz / (2.0 * viscosity)) * y_f * (h - y_f);

            lbm.density[n] = 1.0;
            lbm.velocity[n].x = 0.0;
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = uz;
        }
    });

    // Configure output
    lbm.set_output_interval(20);
    lbm.set_output_vtk(true);

    // Run the simulation
    lbm.run(100);
}