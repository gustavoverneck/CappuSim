// src/main.rs

#![allow(dead_code)]
#![allow(unused_imports)]

// Import
mod lbm;
mod utils;
use lbm::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use lbm::lbm::LBM;

// =============================================================================
// Benchmark Example
fn main() {
    let nx = 128;
    let ny = 128;
    let nz = 128;
    let viscosity = 0.1;
    let time_steps = 50;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, "D3Q19".to_string(), viscosity);

    // Run the simulation
    lbm.run(time_steps);
}

// =============================================================================

// 2D Taylor-Green Vortex Example
// fn main() {
//     let nx = 128;
//     let ny = 128;
//     let nz = 1;
//     let viscosity = 0.01;
//     let u0 = 0.1;
//     let model = "D2Q9".to_string();

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, nz, model, viscosity);

//     // Set initial conditions for Taylor-Green vortex
//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         let pi = std::f32::consts::PI;
//         let lx = nx as f32;
//         let ly = ny as f32;

//         let fx = x as f32 / lx;
//         let fy = y as f32 / ly;

//         lbm.flags[n] = FLAG_FLUID;
//         lbm.density[n] = 1.0;

//         lbm.velocity[n].x = -u0 * (2.0 * pi * fx).cos() * (2.0 * pi * fy).sin();
//         lbm.velocity[n].y = u0 * (2.0 * pi * fx).sin() * (2.0 * pi * fy).cos();
//         lbm.velocity[n].z = 0.0;
//     });

//     // Configure output
//     lbm.set_output_vtk(true);
//     lbm.set_output_interval(200);

//     // Run the simulation
//     lbm.run(10000);
//     // lbm.export_to_vtk("vtk_output/taylor_green_final.vtk").unwrap();
// }

// =============================================================================

// 2D Poiseuille Flow Example
// fn main() {
//     let nx = 256;
//     let ny = 64;
//     let nz = 1;
//     let viscosity = 0.05;
//     let u0 = 0.1;
//     let steps = 20000;
//     let output_interval = 19999;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, nz, "D2Q9".to_string(), viscosity);

//     // Set boundary and initial conditions
//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         if y == 0 || y == ny - 1 {
//             // Top and bottom walls
//             lbm.flags[n] = FLAG_SOLID;
//         } else if x == 0 && y < ny - 1 {
//             // Inlet: left boundary
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         } else if x == nx - 1 && y < ny - 1 {
//             // Outlet: right boundary
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         } else {
//             // Interior: fluid
//             lbm.flags[n] = FLAG_FLUID;
//             lbm.velocity[n].x = u0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         }
//     });

//     // Configure output
//     lbm.set_output_vtk(true);
//     lbm.set_output_interval(output_interval);

//     // Run the simulation
//     lbm.run(steps);
//     lbm.export_to_vtk("output/poiseuille.vtk").unwrap();
// }

// =============================================================================

// 2D Von-Kármán Vortex Street Example
// fn main() {
//     let nx = 256;
//     let ny = 128;
//     let viscosity = 0.01;
//     let u0 = 0.1;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity);

//     // Cylinder parameters
//     let radius = nx as f32 * 0.08;
//     let cx = nx as i32 / 4; // 25% from left
//     let cy = ny as i32 / 2;

//     // Set boundary and initial conditions
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

//     // Configure output
//     lbm.set_output_vtk(true);
//     lbm.set_output_interval(50);

//     // Run the simulation
//     lbm.run(10000);
// }

// =============================================================================

// 2D Lid-driven Cavity Example
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

//     // Configure output
//     lbm.set_output_interval(10);

//     // Run the simulation
//     lbm.run(2000);

//     lbm.output_to("output.csv").expect("Failed to write output file.");
// }

// =============================================================================

// 2D Splash Example
// fn main() {
//     // --- Simulation parameters ---
//     let nx = 256;
//     let ny = 128;
//     let nz = 1;
//     let viscosity = 0.01;
//     let u0 = 0.2;
//     let output_interval = 200;
//     let steps = 5000;

//     let model = "D2Q9".to_string();
//     let mut lbm = LBM::new(nx, ny, nz, model, viscosity);

//     // --- Jet parameters ---
//     let jet_width = nx / 16;
//     let jet_center = nx / 2;
//     let jet_start = jet_center - jet_width;
//     let jet_end = jet_center + jet_width;

//     // --- Initial conditions: LatteSplash™ ---
//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         if y == 0 {
//             lbm.flags[n] = FLAG_SOLID; // ground
//         } else if y == ny - 1 && x >= jet_start && x <= jet_end {
//             lbm.flags[n] = FLAG_EQ; // inlet (jet)
//             lbm.velocity[n].x = 0.0;
//             lbm.velocity[n].y = -u0;
//             lbm.density[n] = 1.0;
//         } else if x == 0 || x == nx - 1 {
//             lbm.flags[n] = FLAG_SOLID; // side walls
//         } else {
//             lbm.flags[n] = FLAG_FLUID;
//             lbm.velocity[n].x = 0.0;
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         }
//     });

//     // --- Output parameters ---
//     lbm.set_output_vtk(true);
//     lbm.set_output_interval(output_interval);

//     // --- Run simulation ---
//     lbm.run(steps);
// }

// =============================================================================

// 3D Poiseuille Example
// fn main() {
//     let nx = 64;
//     let ny = 32;
//     let nz = 64;
//     let viscosity = 0.01;
//     let fz = 1e-6; // Small body force in z-direction

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, nz, "D3Q19".to_string(), viscosity);

//     // Set boundary and initial conditions
//     lbm.set_conditions(|lbm, x, y, z, n| {
//         if y == 0 || y == ny - 1 {
//             lbm.flags[n] = FLAG_SOLID; // No-slip top and bottom walls
//         } else {
//             lbm.flags[n] = FLAG_FLUID;

//             // Approximate initial velocity profile (optional)
//             let y_f = y as f32;
//             let h = ny as f32;
//             let uz = (fz / (2.0 * viscosity)) * y_f * (h - y_f);

//             lbm.density[n] = 1.0;
//             lbm.velocity[n].x = 0.0;
//             lbm.velocity[n].y = 0.0;
//             lbm.velocity[n].z = uz;
//         }
//     });

//     // Configure output
//     lbm.set_output_interval(20);
//     lbm.set_output_vtk(true);

//     // Run the simulation
//     lbm.run(100);
// }

// =============================================================================

// 2D Kelvin-Helmholtz Instability Example
// fn main() {
//     let nx = 512;
//     let ny = 256;
//     let u0 = 0.04; // shear velocity
//     let amplitude = 0.01; // sinusoidal perturbation
//     let cy = ny / 2;
//     let thickness = 20.0; // central transition band

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), 0.005);

//     // Set boundary and initial conditions
//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         let y_f = y as f32;
//         let dy = (y_f - cy as f32) / thickness;
//         let shear = 0.5 * (1.0 + (-dy).tanh()); // smooth profile (hyperbolic tangent)

//         let perturb = amplitude * (2.0 * std::f32::consts::PI * x as f32 / nx as f32).sin();

//         lbm.flags[n] = if y == 0 || y == ny - 1 {
//             FLAG_SOLID
//         } else {
//             FLAG_FLUID
//         };

//         lbm.velocity[n].x = u0 * (2.0 * shear - 1.0); // -u0 at the bottom, +u0 at the top
//         lbm.velocity[n].y = perturb;
//         lbm.density[n] = 1.0;
//     });

//     // Configure output
//     lbm.set_output_vtk(true);
//     lbm.set_output_interval(200);

//     // Run the simulation
//     lbm.run(20000);
// }
