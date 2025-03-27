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


// =============================================================================

// Taylor-Green Vortex example
// fn main() {
//     use std::f32::consts::PI;
//     let nx = 512;
//     let ny = 512;
//     let viscosity = 0.01;
//     let u0 = 0.1;
//     let t_max = 1000;

//     // Cria uma simulação LBM D2Q9
//     let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity);

//     // Define as condições iniciais para o vórtice de Taylor-Green
//     lbm.set_conditions(|lbm, x, y, _z, n| {
//         let x_f = x as f32 / nx as f32;
//         let y_f = y as f32 / ny as f32;

//         let u = -u0 * (2.0 * PI * x_f).cos() * (2.0 * PI * y_f).sin();
//         let v =  u0 * (2.0 * PI * x_f).sin() * (2.0 * PI * y_f).cos();

//         lbm.velocity[n].x = u;
//         lbm.velocity[n].y = v;
//         lbm.velocity[n].z = 0.0;
//         lbm.density[n] = 1.0 - 0.25 * ((4.0 * PI * x_f).cos() + (4.0 * PI * y_f).cos());
//     });

//     // Define a frequência de saída
//     lbm.set_output_interval(100);

//     // Executa a simulação
//     lbm.run(t_max);
// }

// =============================================================================

// Von-Kármán Vortex example
// fn main() {
//     let nx = 128;

//     // Initialize LBM simulation
//     let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);
//     // Set initial conditions
//     lbm.set_conditions(|lbm, x, y, z, n| {
//         let cx = nx as f32 / 6.0; // Circle center x
//         let cy = nx as f32 / 2.0; // Circle center y
//         let radius = nx as f32 / 10.0; // Circle radius

//         lbm.velocity[n].x = 0.0;
//         lbm.velocity[n].y = 0.0;
//         lbm.velocity[n].z = 0.0;

//         // Set density field
//         lbm.density[n] = 1.0f32;

//         // Set FLAG_SOLID for a circular obstacle
//         let dx = x as f32 - cx;
//         let dy = y as f32 - cy;
//         if dx * dx + dy * dy <= radius * radius {
//             lbm.flags[n] = FLAG_SOLID;
//             lbm.density[n] = 0.0f32;
//         }

//         // Set FLAG_EQ for x == 1
//         if x == 1 {
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = 0.1; // Uniform horizontal velocity
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 1.0;
//         }

//         // Set FLAG_EQ for x == nx - 1
//         if x == nx - 1 {
//             lbm.flags[n] = FLAG_EQ;
//             lbm.velocity[n].x = 0.0; // Uniform horizontal velocity
//             lbm.velocity[n].y = 0.0;
//             lbm.density[n] = 0.0;
//         }
//     });
//     lbm.set_output_interval(10);
//     lbm.run(1000);

//     //lbm.output_to("output.csv").expect("Failed to write output file.");
// }

// =============================================================================

// Lid-driven Cavity example
fn main() {
    let nx = 128;

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), 0.1);

    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
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
            lbm.velocity[n].x = 0.1; // Lid moving to the right
            lbm.velocity[n].y = 0.0;
        }
    });

    lbm.set_output_interval(10);
    lbm.run(2000);

    lbm.output_to("output.csv").expect("Failed to write output file.");
}

// =============================================================================