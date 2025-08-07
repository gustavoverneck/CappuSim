#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms
use super::lbm::LBM;

use crate::solver::precision::PrecisionMode;
use crate::utils::terminal_utils;
use ocl::{flags::MEM_READ_WRITE, Buffer, Context, Device, Kernel, Platform, Program, Queue};
use std::error::Error;
use std::mem::size_of;

impl LBM {
    pub fn get_ocl_platform(&mut self) -> Result<Platform, Box<dyn Error>> {
        let platform = Platform::list()
            .into_iter()
            .next()
            .ok_or("Platform not found")?;
        println!("Platform: {}", &platform.name()?);
        Ok(platform)
    }

    pub fn get_ocl_device(&mut self) -> Result<Device, Box<dyn Error>> {
        let device = Device::list_all(self.platform.as_ref().unwrap())?
            .into_iter()
            .next()
            .ok_or("Device not found")?;
        println!("Device: {}", device.name()?);
        Ok(device)
    }

    pub fn get_ocl_context(&mut self) -> Result<Context, Box<dyn Error>> {
        // Create a context for the selected device
        let context = Context::builder()
            .platform(self.platform.unwrap())
            .devices(self.device.unwrap())
            .build()
            .expect("Failed to build context.");
        Ok(context)
    }

    pub fn get_ocl_queue(&mut self) -> Result<Queue, Box<dyn Error>> {
        // Create a command queue for the device
        let queue = Queue::new(self.context.as_ref().unwrap(), self.device.unwrap(), None)
            .expect("Failed to create command queue.");
        Ok(queue)
    }

    pub fn get_ocl_program(&mut self) -> Result<Program, Box<dyn Error>> {
        // Define OpenCL program
        let program = Program::builder()
            .src(self.generate_custom_kernel().unwrap())
            .devices(self.device.as_ref().unwrap())
            .build(self.context.as_ref().unwrap())
            .expect("Failed to build program.");
        Ok(program)
    }

    pub fn reserve_f_buffer(&mut self) -> Result<Buffer<f32>, Box<dyn Error>> {
        let f_buffer = Buffer::<f32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N * self.Q)
            .copy_host_slice(&self.f)
            .build()
            .expect("Failed to build 'f' buffer.");
        Ok(f_buffer)
    }

    pub fn reserve_f_new_buffer(&mut self) -> Result<Buffer<f32>, Box<dyn Error>> {
        let f_new_buffer = Buffer::<f32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N * self.Q)
            .copy_host_slice(&self.f_new)
            .build()
            .expect("Failed to build 'f_new' buffer.");
        Ok(f_new_buffer)
    }

    pub fn reserve_density_buffer(&mut self) -> Result<Buffer<f32>, Box<dyn Error>> {
        let density_buffer = Buffer::<f32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N)
            .copy_host_slice(&self.density)
            .build()
            .expect("Failed to build 'density' buffer.");
        Ok(density_buffer)
    }

    pub fn reserve_u_buffer(&mut self) -> Result<Buffer<f32>, Box<dyn Error>> {
        let u_buffer = Buffer::<f32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N * 3)
            .copy_host_slice(&self.u)
            .build()
            .expect("Failed to build 'velocity' buffer.");
        Ok(u_buffer)
    }

    pub fn reserve_flags_buffer(&mut self) -> Result<Buffer<u8>, Box<dyn Error>> {
        let flags_buffer = Buffer::<u8>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N)
            .copy_host_slice(&self.flags)
            .build()
            .expect("Failed to build 'flags' buffer.");
        Ok(flags_buffer)
    }

    pub fn get_optimal_work_group_size(&self) -> Result<usize, Box<dyn Error>> {
        Ok(64)  // Always return 64
    }

    // pub fn create_streaming_kernel(&mut self) -> Result<(), Box<dyn Error>> {
    //     let work_group_size = self.get_optimal_work_group_size()?;
        
    //     self.streaming_kernel = Some(
    //         Kernel::builder()
    //             .program(self.program.as_ref().unwrap())
    //             .name("streaming_kernel")
    //             .queue(self.queue.as_ref().unwrap().clone())
    //             .global_work_size(self.N)
    //             .local_work_size(work_group_size)
    //             .arg(self.f_buffer.as_ref().unwrap())
    //             .arg(self.f_new_buffer.as_ref().unwrap())
    //             .arg(self.flags_buffer.as_ref().unwrap())
    //             .arg(0i32)
    //             .build()
    //             .expect("Failed to build OpenCL 'streaming_kernel'."),
    //     );
    //     Ok(())
    // }

    // pub fn create_collision_kernel(&mut self) -> Result<(), Box<dyn Error>> {
    //     let work_group_size = self.get_optimal_work_group_size()?;
        
    //     self.collision_kernel = Some(
    //         Kernel::builder()
    //             .program(self.program.as_ref().unwrap())
    //             .name("collision_kernel")
    //             .queue(self.queue.as_ref().unwrap().clone())
    //             .global_work_size(self.N)
    //             .local_work_size(work_group_size)
    //             .arg(self.f_buffer.as_ref().unwrap())
    //             .arg(self.f_new_buffer.as_ref().unwrap())
    //             .arg(self.density_buffer.as_ref().unwrap())
    //             .arg(self.flags_buffer.as_ref().unwrap())
    //             .arg(self.u_buffer.as_ref().unwrap())
    //             .arg(self.omega)
    //             .arg(0i32)
    //             .build()
    //             .expect("Failed to build OpenCL 'collision_kernel'."),
    //     );
    //     Ok(())
    // }

    pub fn create_stream_collide_kernel(&mut self) -> Result<(), Box<dyn Error>> {
        // let work_group_size = self.get_optimal_work_group_size()?;

        self.stream_collide_kernel = Some(
            Kernel::builder()
                .program(self.program.as_ref().unwrap())
                .name("stream_collide_kernel")
                .queue(self.queue.as_ref().unwrap().clone())
                .global_work_size(self.N)
                // .local_work_size(work_group_size)
                .arg(self.f_buffer.as_ref().unwrap())
                .arg(self.f_new_buffer.as_ref().unwrap())
                .arg(self.density_buffer.as_ref().unwrap())
                .arg(self.u_buffer.as_ref().unwrap())
                .arg(self.flags_buffer.as_ref().unwrap())
                .arg(self.omega)
                .arg(0i32) // timestep or other args as needed
                .build()
                .expect("Failed to build OpenCL 'stream_collide_kernel'."),
        );
        Ok(())
    }

    pub fn create_equilibrium_kernel(&mut self) -> Result<(), Box<dyn Error>> {
        self.equilibrium_kernel = Some(
            Kernel::builder()
                .program(self.program.as_ref().unwrap())
                .name("equilibrium")
                .queue(self.queue.as_ref().unwrap().clone())
                .global_work_size(self.N)
                .arg(self.f_buffer.as_ref().unwrap())
                .arg(self.density_buffer.as_ref().unwrap())
                .arg(self.u_buffer.as_ref().unwrap())
                .build()
                .expect("Failed to build OpenCL 'equilibrium_kernel'."),
        );
        Ok(())
    }

    // Read data from GPU to CPU
    pub fn read_from_gpu(&mut self) -> Result<(), Box<dyn Error>> {
        // Velocity
        self.u_buffer
            .as_ref()
            .ok_or("Velocity buffer is None")?
            .read(&mut self.u)
            .enq()
            .map_err(|e| format!("Failed to read 'velocity' buffer: {}", e))?;

        // Density
        self.density_buffer
            .as_ref()
            .ok_or("Density buffer is None")?
            .read(&mut self.density)
            .enq()
            .map_err(|e| format!("Failed to read 'density' buffer: {}", e))?;

        Ok(())
    }

    pub fn calculate_vram_usage(&self) {
        // Manual calculation based on precision mode
        // f, f_new: N*Q, density: N, u: N*3, flags: N
        let n = self.N;
        let q = self.Q;
        let f_bytes;
        let f_new_bytes;
        let density_bytes = n * std::mem::size_of::<f32>();
        let u_bytes = n * 3 * std::mem::size_of::<f32>();
        let flags_bytes = n * std::mem::size_of::<u8>();

        // Assume self.precision_mode: String or enum ("FP32", "FP16S", "FP16C")
        let precision = &self.precision_mode;
        match precision {
            PrecisionMode::FP32 => {
                f_bytes = n * q * std::mem::size_of::<f32>();
                f_new_bytes = n * q * std::mem::size_of::<f32>();
            },
            PrecisionMode::FP16S | PrecisionMode::FP16C => {
                f_bytes = n * q * 2; // half = 2 bytes
                f_new_bytes = n * q * 2;
            }
        }

        let total_vram = f_bytes + f_new_bytes + density_bytes + u_bytes + flags_bytes;

        println!(
            "VRAM usage: {:.2} MB",
            total_vram as f64 / (1024.0 * 1024.0)
        );
        terminal_utils::print_success("OpenCL device and context initialized successfully!");
    }
}
