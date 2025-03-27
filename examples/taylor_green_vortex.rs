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

// Taylor-Green Vortex example
fn main() {
    use std::f32::consts::PI;
    let nx = 512;
    let ny = 512;
    let viscosity = 0.01;
    let u0 = 0.1;
    let t_max = 1000;

    // Cria uma simulação LBM D2Q9
    let mut lbm = LBM::new(nx, ny, 1, "D2Q9".to_string(), viscosity);

    // Define as condições iniciais para o vórtice de Taylor-Green
    lbm.set_conditions(|lbm, x, y, _z, n| {
        let x_f = x as f32 / nx as f32;
        let y_f = y as f32 / ny as f32;

        let u = -u0 * (2.0 * PI * x_f).cos() * (2.0 * PI * y_f).sin();
        let v =  u0 * (2.0 * PI * x_f).sin() * (2.0 * PI * y_f).cos();

        lbm.velocity[n].x = u;
        lbm.velocity[n].y = v;
        lbm.velocity[n].z = 0.0;
        lbm.density[n] = 1.0 - 0.25 * ((4.0 * PI * x_f).cos() + (4.0 * PI * y_f).cos());
    });

    // Define a frequência de saída
    lbm.set_output_interval(100);

    // Executa a simulação
    lbm.run(t_max);
}