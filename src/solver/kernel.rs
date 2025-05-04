#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms
use super::lbm::LBM;

use std::error::Error;

pub const KERNEL_EQUILIBRIUM_SRC: &str = include_str!("../kernels/kernel_equilibrium.cl");
pub const KERNEL_STREAMING_SRC: &str = include_str!("../kernels/kernel_streaming.cl");
pub const KERNEL_COLLISION_SRC: &str = include_str!("../kernels/kernel_collision.cl");
pub const KERNEL_VELOCITY_SETS_SRC: &str = include_str!("../kernels/kernel_velocity_sets.cl");

impl LBM {
    pub fn generate_custom_kernel(&mut self) -> Result<String, Box<dyn Error>> {
        let kernel_source = format!(
            r#"
        #define NX {}
        #define NY {}
        #define NZ {}
        #define N {}
        #define Q {}
        #define {}
        #define FLAG_FLUID 0
        #define FLAG_SOLID 1
        #define FLAG_EQ 2
        {}
        {}
        {}
        {}
        "#,
            self.Nx,
            self.Ny,
            self.Nz,
            self.N,
            self.Q,
            self.model.as_str(),
            KERNEL_VELOCITY_SETS_SRC,
            KERNEL_STREAMING_SRC,
            KERNEL_COLLISION_SRC,
            KERNEL_EQUILIBRIUM_SRC,
        );
        Ok(kernel_source)
    }
}
