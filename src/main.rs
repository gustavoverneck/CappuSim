// src/main.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;
use core::lbm::{FLAG_FLUID, FLAG_SOLID, FLAG_EQ};
use std::fs::{File, create_dir_all};
use std::io::{self, BufRead};
use std::path::Path;

// =============================================================================

// // Taylor-Green Vortex example
fn main() {
    let nx = 128;
    let ny = 128;
    let nz = 1;

    let viscosity = 0.01;
    let u0 = 0.1;
    let model = "D2Q9".to_string();

    let mut lbm = LBM::new(nx, ny, nz, model, viscosity);

    // Initial condition: Taylor-Green vortex
    lbm.set_conditions(|lbm, x, y, _z, n| {
        let pi = std::f32::consts::PI;
        let lx = nx as f32;
        let ly = ny as f32;

        let fx = x as f32 / lx;
        let fy = y as f32 / ly;

        lbm.flags[n] = FLAG_FLUID;
        lbm.density[n] = 1.0;

        lbm.velocity[n].x = -u0 * (2.0 * pi * fx).cos() * (2.0 * pi * fy).sin();
        lbm.velocity[n].y =  u0 * (2.0 * pi * fx).sin() * (2.0 * pi * fy).cos();
        lbm.velocity[n].z = 0.0;
    });

    lbm.set_output_vtk(true);
    lbm.set_output_interval(100);
    lbm.run(5000);
    lbm.export_to_vtk("vtk_output/taylor_green_final.vtk").unwrap();
}


// Poiseuille Flow example
// fn main() {
//     let nx = 128;
//     let ny = 64;
//     let viscosity = 0.01;
//     let pressure_gradient = 1e-5;
//     let t_max = 500;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity);

//     // Set initial conditions: constant velocity at the inlet (left boundary)
//     lbm.set_conditions(|lbm, x, y, z, n| {
//         if x == 0 {
//             lbm.velocity[n].x = 0.1; // Set a constant velocity at the inlet
//             lbm.velocity[n].y = 0.0;
//         } else {
//             lbm.velocity[n].x = 0.0; // Set velocity to zero elsewhere
//             lbm.velocity[n].y = 0.0;
//         }
//     });

//     lbm.set_output_interval(10);
//     lbm.run(t_max);
// }

// =============================================================================

// Von-Kármán Vortex Street Example
// fn main() {
//     let nx = 256;
//     let ny = 128;

//     let viscosity = 0.01;
//     let omega = 1.0 / (3.0 * viscosity + 0.5);
//     let u0 = 0.1;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity);

//     // Cylinder parameters
//     let radius = nx as f32 * 0.08;
//     let cx = nx as i32 / 4; // 25% from left
//     let cy = ny as i32 / 2;

//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         let dx = x as i32 - cx;
//         let dy = y as i32 - cy;
//         let dist = ((dx * dx + dy * dy) as f32).sqrt();

//         if dist <= radius {
//             lbm.flags[n] = FLAG_SOLID; // Cylinder obstacle
//         } else if x == 0 {
//             // Inlet with prescribed velocity
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         } else if x == nx - 1 {
//             // Outflow: still FLAG_EQ for now, but zero-velocity to reduce reflection
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         } else if y == 0 || y == ny - 1 {
//             // Top and bottom walls
//             lbm.flags[n] = FLAG_SOLID;
//         } else {
//             // Normal fluid region
//             lbm.flags[n] = FLAG_FLUID;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         }
//     });

//     lbm.set_output_interval(50);
//     lbm.run(10000);

//     // Create folder for VTK output
//     let _ = create_dir_all("vtk_output");
//     lbm.export_to_vtk("vtk_output/vkv_final_state.vtk").unwrap();
// }

// =============================================================================

// Lid-driven Cavity example
// fn main() {
//     let nx = 128;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);

//     // Set initial conditions
//     lbm.set_conditions(|lbm, x, y, z, n| {
//         lbm.velocity[n].x = 0.0;
//         lbm.velocity[n].y = 0.0;
//         lbm.velocity[n].z = 0.0;
//         lbm.density[n] = 1.0f32;

//         // Set FLAG_SOLID for the walls
//         if y == 0 || y == nx - 1 || x == 0 || x == nx - 1 {
//             lbm.flags[n] = FLAG_SOLID;
//         }

//         // Set FLAG_EQ for the top lid with a constant velocity
//         if y == nx - 1 {
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = -0.1; // Lid moving to the right
//             lbm.velocity[n].y = 0.0;
//         }
//     });

//     lbm.set_output_interval(10);
//     lbm.run(2000);

//     lbm.output_to("output.csv").expect("Failed to write output file.");
// }

// =============================================================================