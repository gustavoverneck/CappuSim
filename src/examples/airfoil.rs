// src/examples/airfoil

use crate::solver;
use solver::flags::{FLAG_EQ, FLAG_FLUID, FLAG_SOLID};
use solver::lbm::LBM;
use solver::precision::PrecisionMode;

// 2D NACA Airfoil Flow Example
pub fn airfoil_2d_example() {
    let nx = 1024;
    let ny = 512;
    let nz = 1;
    let viscosity = 0.01;
    let u0 = 0.1; // Inlet velocity
    let angle_of_attack = 10.0; // Degrees
    let chord_length = nx as f32 * 0.3; // 30% of domain width

    let mut lbm = LBM::new(nx, ny, nz, "D2Q9".to_string(), viscosity, PrecisionMode::FP32);

    let thickness = 0.12; // 12% thickness
    let cx = nx as f32 * 0.3; // Airfoil center x
    let cy = ny as f32 * 0.5; // Airfoil center y

    let angle_rad = angle_of_attack * std::f32::consts::PI / 180.0;

    lbm.set_conditions(|lbm, x, y, _z, n| {
        let xf = x as f32;
        let yf = y as f32;

        // Transform to airfoil reference frame
        let dx = xf - cx;
        let dy = yf - cy;

        // Rotate coordinates
        let x_rot = dx * angle_rad.cos() + dy * angle_rad.sin();
        let y_rot = -dx * angle_rad.sin() + dy * angle_rad.cos();

        let x_c = x_rot / chord_length;

        if x_c >= 0.0 && x_c <= 1.0 {
            // NACA 0012 formula
            let yt = 5.0 * thickness * (0.2969 * x_c.sqrt() - 0.1260 * x_c
                - 0.3516 * x_c.powi(2) + 0.2843 * x_c.powi(3)
                - 0.1015 * x_c.powi(4));

            if (y_rot).abs() <= yt * chord_length {
                lbm.flags[n] = FLAG_SOLID;
                return;
            }
        }

        // Inlet
        if x == 0 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
        // Outlet
        else if x == nx - 1 {
            lbm.flags[n] = FLAG_EQ;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
        // Top and bottom walls
        else if y == 0 || y == ny - 1 {
            lbm.flags[n] = FLAG_SOLID;
        }
        // Fluid region
        else {
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = u0;
            lbm.velocity[n].y = 0.0;
            lbm.density[n] = 1.0;
        }
    });

    // lbm.set_output_vtk(true);
    // lbm.set_output_interval(200);

    lbm.run(10000);
    lbm.export_to_vtk(&format!("results/airfoil_aoa_{}.vtk", angle_of_attack))
        .expect("Failed to write VTK output");

    let re = u0 * chord_length / viscosity;
    println!("Reynolds number: {}", re);
}

// 3D NACA Airfoil Flow Example with constant force for pressure gradient
pub fn airfoil_3d_example() {
    let nx = 512;
    let ny = 256;
    let nz = 128;
    let viscosity = 0.01;
    let target_velocity = 0.1;
    let angle_of_attack = 10.0;
    let chord_length = nx as f32 * 0.3;
    let span_length = nz as f32 * 0.8;

    // Force magnitude for pressure gradient
    let force_magnitude = 5e-6 * viscosity * target_velocity;

    let mut lbm = LBM::new(nx, ny, nz,
        "D3Q19".to_string(),
        viscosity,
        PrecisionMode::FP32
    );

    // Apply constant force in X direction
    lbm.set_constant_force(vec![force_magnitude, 0.0, 0.0]);

    let thickness = 0.12;
    let cx = nx as f32 * 0.3;
    let cy = ny as f32 * 0.5;
    let cz = nz as f32 * 0.5;

    let angle_rad = angle_of_attack * std::f32::consts::PI / 180.0;

    let re = target_velocity * chord_length / viscosity;
    println!("Reynolds number: {}", re);

    lbm.set_conditions(|lbm, x, y, z, n| {
        let xf = x as f32;
        let yf = y as f32;
        let zf = z as f32;

        // Transform to airfoil reference frame
        let dx = xf - cx;
        let dy = yf - cy;
        let dz = zf - cz;

        // Rotate around airfoil center (z fixed)
        let x_rot = dx * angle_rad.cos() + dy * angle_rad.sin();
        let y_rot = -dx * angle_rad.sin() + dy * angle_rad.cos();
        let z_rot = dz;

        let x_c = x_rot / chord_length;

        // Airfoil body
        if x_c >= 0.0 && x_c <= 1.0 && z_rot.abs() <= span_length / 2.0 {
            let yt = 5.0 * thickness * (0.2969 * x_c.sqrt() - 0.1260 * x_c
                - 0.3516 * x_c.powi(2) + 0.2843 * x_c.powi(3)
                - 0.1015 * x_c.powi(4));

            if y_rot.abs() <= yt * chord_length {
                lbm.flags[n] = FLAG_SOLID;
                return;
            }
        }

        // Inlet
        if x == 0 {
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = target_velocity;
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = 0.0;
            lbm.density[n] = 1.0;
        }
        // Outlet
        else if x == nx - 1 {
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = target_velocity;
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = 0.0;
            lbm.density[n] = 1.0;
        }
        // Top and bottom walls
        else if y == 0 || y == ny - 1 {
            lbm.flags[n] = FLAG_SOLID;
        }
        // Front and back periodic boundaries
        else if z == 0 || z == nz - 1 {
            lbm.flags[n] = FLAG_FLUID;
        }
        // Fluid region
        else {
            lbm.flags[n] = FLAG_FLUID;
            lbm.velocity[n].x = target_velocity;
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = 0.0;
            lbm.density[n] = 1.0;
        }
    });

    // lbm.set_output_vtk(true);
    // lbm.set_output_interval(200);

    lbm.run(10000);
    lbm.export_to_vtk(&format!("output/airfoil3d_aoa_{}_force.vtk", angle_of_attack))
        .expect("Failed to write VTK output");

    let re = target_velocity * chord_length / viscosity;
    println!("Reynolds number: {}", re);
}
