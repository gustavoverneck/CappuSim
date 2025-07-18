// src/lbm/check.rs

#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms
use super::lbm::LBM;

use std::error::Error;

impl LBM {
    pub fn check_errors_in_input(&mut self) -> Result<(), Box<dyn Error>> {
        // Check if the dimensions are positive
        if self.Nx == 0 || self.Ny == 0 || self.Nz == 0 {
            self.found_errors = true;
            return Err("Dimensions Nx, Ny, and Nz must be greater than 0.".into());
        }

        // Check if the model is supported
        let supported_models = ["D2Q9", "D3Q7", "D3Q15", "D3Q19", "D3Q27"];
        if !supported_models.contains(&self.model.as_str()) {
            self.found_errors = true;
            return Err(format!("Unsupported model: {}.", self.model).into());
        }

        // Check if D2Q9 model is used with Nz != 1
        if self.model == "D2Q9" && self.Nz != 1 {
            self.found_errors = true;
            return Err("D2Q9 model should have Nz equal to 1.".into());
        }

        // Check if viscosity is positive
        if self.viscosity <= 0.0 {
            self.found_errors = true;
            return Err("Viscosity must be greater than 0.".into());
        }

        // Check if density and velocity vectors have the correct length
        let expected_size = self.Nx * self.Ny * self.Nz;
        if self.density.len() != expected_size {
            self.found_errors = true;
            return Err("Density vector has incorrect length.".into());
        }
        if self.u.len() != expected_size * 3 {
            self.found_errors = true;
            return Err("Velocity vector has incorrect length. Expected size * 3.".into());
        }
        if self.flags.len() != expected_size {
            self.found_errors = true;
            return Err("Flags vector has incorrect length.".into());
        }

        // Check if OpenCL queue is available
        if let Some(queue) = &self.queue {
            if let Err(err) = queue.finish() {
                self.found_errors = true;
                return Err(format!("OpenCL queue error: {}", err).into());
            }
        }

        Ok(())
    }
}
