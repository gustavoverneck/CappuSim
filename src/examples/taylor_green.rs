// src/examples/taylor_green
#![allow(dead_code)]
#![allow(unused_imports)]
// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode; 

// 2D Taylor-Green Vortex Example
pub fn taylor_green_2d_example() {
    let nx = 128;
    let ny = 128;
    let nz = 1;
    let viscosity = 0.01;
    let u0 = 0.1;
    let model = "D2Q9".to_string();

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, model, viscosity, PrecisionMode::FP32);

    // Set initial conditions for Taylor-Green vortex
    lbm.set_conditions(|lbm, x, y, _z, n| {
        let pi = std::f32::consts::PI;
        let lx = nx as f32;
        let ly = ny as f32;

        let fx = x as f32 / lx;
        let fy = y as f32 / ly;

        lbm.flags[n] = FLAG_FLUID;
        lbm.density[n] = 1.0;

        lbm.velocity[n].x = -u0 * (2.0 * pi * fx).cos() * (2.0 * pi * fy).sin();
        lbm.velocity[n].y = u0 * (2.0 * pi * fx).sin() * (2.0 * pi * fy).cos();
        lbm.velocity[n].z = 0.0;
    });

    // Configure output
    lbm.set_output_vtk(true);
    lbm.set_output_interval(200);

    // Run the simulation
    lbm.run(10000);
    // lbm.export_to_vtk("vtk_output/taylor_green_final.vtk").unwrap();
}

// 3D Taylor-Green Vortex Example
pub fn taylor_green_3d_example() {
    let nx = 128;
    let ny = 128;
    let nz = 128;
    let viscosity = 0.01;
    let a = nx as f32;
    let b = ny as f32;
    let c = nz as f32;
    let a_amp = 0.25;
    let pi = std::f32::consts::PI;

    let model = "D3Q19".to_string();

    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, ny, nz, model, viscosity, PrecisionMode::FP16C);

    // Set initial conditions for Taylor-Green vortex in 3D (FluidX3D style)
    lbm.set_conditions(|lbm, x, y, z, n| {
        let fx = x as f32 + 0.5 - 0.5 * nx as f32;
        let fy = y as f32 + 0.5 - 0.5 * ny as f32;
        let fz = z as f32 + 0.5 - 0.5 * nz as f32;

        lbm.flags[n] = FLAG_FLUID;

        lbm.velocity[n].x =  a_amp * (2.0 * pi * fx / a).cos() * (2.0 * pi * fy / b).sin() * (2.0 * pi * fz / c).sin();
        lbm.velocity[n].y = -a_amp * (2.0 * pi * fx / a).sin() * (2.0 * pi * fy / b).cos() * (2.0 * pi * fz / c).sin();
        lbm.velocity[n].z =  a_amp * (2.0 * pi * fx / a).sin() * (2.0 * pi * fy / b).sin() * (2.0 * pi * fz / c).cos();

        lbm.density[n] = 1.0 - a_amp.powi(2) * 3.0 / 4.0 * (
            (4.0 * pi * fx / a).cos() + (4.0 * pi * fy / b).cos()
        );
    });

    // Configure output
    lbm.set_output_vtk(true);
    lbm.set_output_interval(50);

    // Run the simulation
    lbm.run(10000);
    // lbm.export_to_vtk("vtk_output/taylor_green_3d_final.vtk").unwrap();
}