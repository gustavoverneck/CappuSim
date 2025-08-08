// src/examples/liddriven_cavity

// Import
use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode; 

// 2D Lid-driven Cavity Example
pub fn liddriven_cavity_2d_example() {
    let lid_velocity = 0.05;
    let nx = 100;
    let viscosity = 0.0125f32;
    let re = lid_velocity * nx as f32 / viscosity;
    println!("Reynolds number: {}", re);
    
    // Initialize LBM simulation
    let mut lbm = LBM::new(nx, nx, 1, "D2Q9".to_string(), viscosity, PrecisionMode::FP32);

    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, _z, n| {
        lbm.velocity[n].x = 0.0;
        lbm.velocity[n].y = 0.0;
        lbm.velocity[n].z = 0.0;
        lbm.density[n] = 1.0;
        
        lbm.flags[n] = FLAG_FLUID;
        
        if y == 0 || x == 0 || x == nx - 1 {
            lbm.flags[n] = FLAG_SOLID;
        }
        
        else if y == nx - 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = lid_velocity;
            lbm.velocity[n].y = 0.0;
        }
    });

    // Configure output
    // lbm.set_output_interval(10);

    // Run the simulation
    lbm.run(1000000);
    
    lbm.export_to_vtk(&format!("output/liddriven_cavity_re{}.vtk", re as i32)).unwrap();
}


pub fn liddriven_cavity_3d_example() {
    let nx = 200;

    // Initialize LBM simulation for 3D
    let mut lbm = LBM::new(nx, nx, nx, "D3Q19".to_string(), 0.1, PrecisionMode::FP32);

    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        lbm.velocity[n].x = 0.0;
        lbm.velocity[n].y = 0.0;
        lbm.velocity[n].z = 0.0;
        lbm.density[n] = 1.0f32;

        // Set FLAG_SOLID for the walls
        if x == 0 || x == nx - 1 || y == 0 || y == nx - 1 || z == 0 || z == nx - 1 {
            lbm.flags[n] = FLAG_SOLID;
        }

        // Set FLAG_EQ for the top lid (z = nx - 1) with a constant velocity
        if z == nx - 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = -0.1; // Lid moving in x-direction
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = 0.0;
        }
    });

    // Configure output
    lbm.set_output_interval(10);

    // Run the simulation
    lbm.run(10000);

    lbm.export_to_vtk("output/liddriven_cavity_3d.vtk").expect("Failed to write output file.");
}