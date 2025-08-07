#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms
use super::lbm::LBM;
use crate::solver::precision::PrecisionMode;

use std::error::Error;

pub const KERNEL_EQUILIBRIUM_SRC: &str = include_str!("../kernels/kernel_equilibrium.cl");
//pub const KERNEL_STREAMING_SRC: &str = include_str!("../kernels/kernel_streaming.cl");
//pub const KERNEL_COLLISION_SRC: &str = include_str!("../kernels/kernel_collision.cl");
pub const KERNEL_VELOCITY_SETS_SRC: &str = include_str!("../kernels/kernel_velocity_sets.cl");
pub const KERNEL_STREAM_COLLIDE_SRC: &str = include_str!("../kernels/kernel_stream_collide.cl");

impl LBM {
    pub fn generate_custom_kernel(&mut self) -> Result<String, Box<dyn Error>> {
        let precision_defines = match self.precision_mode {
            PrecisionMode::FP32 => {
                "#define USE_FP32\n#define FLOAT_TYPE float\n#define FLOAT4_TYPE float4\n"
            },
            PrecisionMode::FP16S => {
                "#define USE_FP16S\n#define FLOAT_TYPE float\n#define STORAGE_TYPE half\n#define FLOAT4_TYPE float4\n"
            },
            PrecisionMode::FP16C => {
                "#define USE_FP16C\n#define FLOAT_TYPE half\n#define FLOAT4_TYPE half4\n"
            },
        };

        let kernel_source = format!(
            r#"
        {}
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
        "#,
            precision_defines,
            self.Nx,
            self.Ny,
            self.Nz,
            self.N,
            self.Q,
            self.model.as_str(),
            KERNEL_VELOCITY_SETS_SRC,
            KERNEL_STREAM_COLLIDE_SRC,
            KERNEL_EQUILIBRIUM_SRC,
        );
        Ok(kernel_source)
    }
}
