//  core/src/lbm/lbm.rs
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use crate::utils::velocity::Velocity;
use ocl::{Buffer, Context, Device, Kernel, Platform, Program, Queue};

pub struct LBM {
    pub Nx: usize,
    pub Ny: usize,
    pub Nz: usize,
    pub N: usize,
    pub model: String,
    pub Q: usize,
    pub viscosity: f32,
    pub omega: f32,
    pub time_steps: usize,
    pub f: Vec<f32>,
    pub f_new: Vec<f32>,
    pub density: Vec<f32>,
    pub u: Vec<f32>,
    pub velocity: Vec<Velocity>,
    pub flags: Vec<i32>,
    pub f_buffer: Option<Buffer<f32>>,
    pub f_new_buffer: Option<Buffer<f32>>,
    pub density_buffer: Option<Buffer<f32>>,
    pub u_buffer: Option<Buffer<f32>>,
    pub flags_buffer: Option<Buffer<i32>>,
    pub platform: Option<Platform>,
    pub device: Option<Device>,
    pub context: Option<Context>,
    pub queue: Option<Queue>,
    pub program: Option<Program>,
    // pub streaming_kernel: Option<Kernel>,
    // pub collision_kernel: Option<Kernel>,
    pub equilibrium_kernel: Option<Kernel>,
    pub stream_collide_kernel: Option<Kernel>,
    pub found_errors: bool,
    pub output_interval: usize,
    pub output_csv: bool,
    pub output_vtk: bool,
}
