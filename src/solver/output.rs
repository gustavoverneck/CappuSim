#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use super::lbm::LBM;
use crate::solver::transforms::{n_from_xyz, xyz_from_n};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

impl LBM {
    pub fn set_output_csv(&mut self, state: bool) {
        self.output_csv = state;
    }

    pub fn set_output_vtk(&mut self, state: bool) {
        self.output_vtk = state;
    }

    pub fn calculate_vorticity(&self, x: usize, y: usize, z: usize) -> f32 {
        let (vort_x, vort_y, vort_z) = self.calculate_vorticity_vector(x, y, z);

        (vort_x * vort_x + vort_y * vort_y + vort_z * vort_z).sqrt()
    }

    pub fn calculate_vorticity_vector(&self, x: usize, y: usize, z: usize) -> (f32, f32, f32) {
        let dx = 1.0;
        let dy = 1.0;
        let dz = 1.0;

        let get = |x, y, z, d| {
            if x >= self.Nx || y >= self.Ny || z >= self.Nz {
                0.0
            } else {
                let i = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny);
                self.u[i * 3 + d]
            }
        };

        let du_dy = (get(x, y + 1, z, 0) - get(x, y.saturating_sub(1), z, 0)) / (2.0 * dy);
        let du_dz = (get(x, y, z + 1, 0) - get(x, y, z.saturating_sub(1), 0)) / (2.0 * dz);
        let dv_dx = (get(x + 1, y, z, 1) - get(x.saturating_sub(1), y, z, 1)) / (2.0 * dx);
        let dv_dz = (get(x, y, z + 1, 1) - get(x, y, z.saturating_sub(1), 1)) / (2.0 * dz);
        let dw_dx = (get(x + 1, y, z, 2) - get(x.saturating_sub(1), y, z, 2)) / (2.0 * dx);
        let dw_dy = (get(x, y + 1, z, 2) - get(x, y.saturating_sub(1), z, 2)) / (2.0 * dy);

        let vort_x = dw_dy - dv_dz;
        let vort_y = du_dz - dw_dx;
        let vort_z = dv_dx - du_dy;

        (vort_x, vort_y, vort_z)
    }

    pub fn calculate_q_criterion(&self, x: usize, y: usize, z: usize) -> f32 {
        let dx = 1.0_f32;
        let dy = 1.0_f32;
        let dz = 1.0_f32;

        let get = |x: usize, y: usize, z: usize, d: usize| -> f32 {
            let xi = x.clamp(0, self.Nx - 1);
            let yi = y.clamp(0, self.Ny - 1);
            let zi = z.clamp(0, self.Nz - 1);
            let i = n_from_xyz(&xi, &yi, &zi, &self.Nx, &self.Ny);
            self.u[i * 3 + d]
        };

        let du_dx = (get(x + 1, y, z, 0) - get(x.saturating_sub(1), y, z, 0)) / (2.0 * dx);
        let du_dy = (get(x, y + 1, z, 0) - get(x, y.saturating_sub(1), z, 0)) / (2.0 * dy);
        let du_dz = (get(x, y, z + 1, 0) - get(x, y, z.saturating_sub(1), 0)) / (2.0 * dz);

        let dv_dx = (get(x + 1, y, z, 1) - get(x.saturating_sub(1), y, z, 1)) / (2.0 * dx);
        let dv_dy = (get(x, y + 1, z, 1) - get(x, y.saturating_sub(1), z, 1)) / (2.0 * dy);
        let dv_dz = (get(x, y, z + 1, 1) - get(x, y, z.saturating_sub(1), 1)) / (2.0 * dz);

        let dw_dx = (get(x + 1, y, z, 2) - get(x.saturating_sub(1), y, z, 2)) / (2.0 * dx);
        let dw_dy = (get(x, y + 1, z, 2) - get(x, y.saturating_sub(1), z, 2)) / (2.0 * dy);
        let dw_dz = (get(x, y, z + 1, 2) - get(x, y, z.saturating_sub(1), 2)) / (2.0 * dz);

        // Strain tensor S (Symmetric)
        let s_xx: f32 = du_dx;
        let s_yy: f32 = dv_dy;
        let s_zz: f32 = dw_dz;
        let s_xy: f32 = 0.5 * (du_dy + dv_dx);
        let s_xz: f32 = 0.5 * (du_dz + dw_dx);
        let s_yz: f32 = 0.5 * (dv_dz + dw_dy);

        // Vorticity tensor W (Antisymmetric)
        let w_xy: f32 = 0.5 * (du_dy - dv_dx);
        let w_xz: f32 = 0.5 * (du_dz - dw_dx);
        let w_yz: f32 = 0.5 * (dv_dz - dw_dy);

        let s_norm = s_xx.powi(2)
            + s_yy.powi(2)
            + s_zz.powi(2)
            + 2.0 * (s_xy.powi(2) + s_xz.powi(2) + s_yz.powi(2));
        let w_norm = 2.0 * (w_xy.powi(2) + w_xz.powi(2) + w_yz.powi(2));

        0.5 * (w_norm - s_norm)
    }

    pub fn output_to_csv(&self, path: &str) -> Result<(), Box<dyn Error>> {
        if self.found_errors {
            return Err("Errors were found in the input parameters. Cannot write output.".into());
        }
        // Create the file and wrap it in a BufWriter for better performance
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write the header
        writeln!(
            writer,
            "x, y, z, rho,      ux,       uy,       uz,       v,       q"
        )?;

        // Iterate over the grid and write the data
        for n in 0..self.N {
            // Get the x, y, z coordinates from the linear index n
            let (x, y, z) = xyz_from_n(&n, &self.Nx, &self.Ny);
            // Get density and velocity
            let rho = &self.density[n];
            let ux = self.u[n * 3];
            let uy = self.u[n * 3 + 1];
            let uz = self.u[n * 3 + 2];

            // Calculate vorticity
            let vorticity = self.calculate_vorticity(x, y, z);
            let q_criteria = self.calculate_q_criterion(x, y, z);
            // Write the data to the file
            writeln!(
                writer,
                "{}, {}, {}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}", // Format floating-point numbers to 6 decimal places
                x, y, z, rho, ux, uy, uz, vorticity, q_criteria
            )?;
        }

        // Flush the buffer to ensure all data is written to the file
        writer.flush()?;

        //println!("Simulation results have been written to {}", path);
        Ok(())
    }

    pub fn set_output_interval(&mut self, interval: usize) {
        self.output_interval = interval;
    }

    pub fn export_to_vtk(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        let total_points = self.N;

        writeln!(writer, "# vtk DataFile Version 3.0")?;
        writeln!(writer, "CappuSim Simulation Output")?;
        writeln!(writer, "ASCII")?;
        writeln!(writer, "DATASET STRUCTURED_POINTS")?;
        writeln!(writer, "DIMENSIONS {} {} {}", self.Nx, self.Ny, self.Nz)?;
        writeln!(writer, "ORIGIN 0 0 0")?;
        writeln!(writer, "SPACING 1 1 1")?;
        writeln!(writer, "POINT_DATA {}", total_points)?;

        // Cache Q-criterion and vorticity
        let mut q_crit = vec![0.0; self.N];
        let mut vorticity = vec![(0.0, 0.0, 0.0); self.N];
        for z in 0..self.Nz {
            for y in 0..self.Ny {
                for x in 0..self.Nx {
                    let i = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny);
                    q_crit[i] = self.calculate_q_criterion(x, y, z);
                    vorticity[i] = self.calculate_vorticity_vector(x, y, z);
                }
            }
        }

        // Density
        writeln!(writer, "SCALARS density float")?;
        writeln!(writer, "LOOKUP_TABLE default")?;
        for val in &self.density {
            writeln!(writer, "{:.6}", val)?;
        }

        // Velocity
        writeln!(writer, "VECTORS velocity float")?;
        for i in 0..total_points {
            writeln!(
                writer,
                "{:.6} {:.6} {:.6}",
                self.u[i * 3],
                self.u[i * 3 + 1],
                self.u[i * 3 + 2]
            )?;
        }

        // Q-Criterion
        writeln!(writer, "SCALARS q_criterion float")?;
        writeln!(writer, "LOOKUP_TABLE default")?;
        for val in &q_crit {
            writeln!(writer, "{:.6}", val)?;
        }

        // Vorticity
        writeln!(writer, "VECTORS vorticity float")?;
        for (vx, vy, vz) in &vorticity {
            writeln!(writer, "{:.6} {:.6} {:.6}", vx, vy, vz)?;
        }

        Ok(())
    }
}
