// src/main.rs

#![allow(dead_code)]
#![allow(unused_imports)]

// Import
mod solver;
mod utils;
mod examples;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::benchmark;
use solver::precision::PrecisionMode;
use examples::{poiseuille, von_karman, taylor_green, liddriven_cavity, airfoil};

use crate::examples::poiseuille::poiseuille_2d_example;
use crate::examples::von_karman::von_karman_vortex_2d_example;
use crate::examples::taylor_green::taylor_green_2d_example;
use crate::examples::liddriven_cavity::liddriven_cavity_2d_example;
use crate::examples::airfoil::{airfoil_2d_example, airfoil_3d_example};
use crate::examples::couette::couette_2d_example;

// =============================================================================
// Comprehensive Benchmark Suite
fn main() {
    // To run an example, uncomment the corresponding function call below:
    // or set your own setup. Check /examples for inspiration.

    LBM::benchmark();

    // airfoil_2d_example();
    // airfoil_3d_example();
    // couette_2d_example();
    // liddriven_cavity_2d_example();
    // poiseuille_2d_example();
    // von_karman_vortex_2d_example

}
