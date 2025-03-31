#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms
use super::lbm::LBM;

use crate::utils::terminal_utils;
use ocl::{flags::MEM_READ_WRITE, Buffer, Context, Device, Kernel, Platform, Program, Queue};
use std::error::Error;

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

    pub fn reserve_flags_buffer(&mut self) -> Result<Buffer<i32>, Box<dyn Error>> {
        let flags_buffer = Buffer::<i32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N)
            .copy_host_slice(&self.flags)
            .build()
            .expect("Failed to build 'flags' buffer.");
        Ok(flags_buffer)
    }

    pub fn create_streaming_kernel(&mut self) -> Result<(), Box<dyn Error>> {
        self.streaming_kernel = Some(
            Kernel::builder()
                .program(self.program.as_ref().unwrap())
                .name("streaming_kernel")
                .queue(self.queue.as_ref().unwrap().clone())
                .global_work_size(self.N)
                .arg(self.f_buffer.as_ref().unwrap())
                .arg(self.f_new_buffer.as_ref().unwrap())
                .arg(self.flags_buffer.as_ref().unwrap())
                .build()
                .expect("Failed to build OpenCL 'streaming_kernel'."),
        );
        Ok(())
    }

    pub fn create_collision_kernel(&mut self) -> Result<(), Box<dyn Error>> {
        self.collision_kernel = Some(
            Kernel::builder()
                .program(self.program.as_ref().unwrap())
                .name("collision_kernel")
                .queue(self.queue.as_ref().unwrap().clone())
                .global_work_size(self.N)
                .arg(self.f_buffer.as_ref().unwrap())
                .arg(self.density_buffer.as_ref().unwrap())
                .arg(self.flags_buffer.as_ref().unwrap())
                .arg(self.u_buffer.as_ref().unwrap())
                .arg(self.omega)
                .build()
                .expect("Failed to build OpenCL 'collision_kernel'."),
        );
        Ok(())
    }

    pub fn create_swap_kernel(&mut self) -> Result<(), Box<dyn Error>> {
        self.swap_kernel = Some(
            Kernel::builder()
                .program(self.program.as_ref().unwrap())
                .name("swap")
                .queue(self.queue.as_ref().unwrap().clone())
                .global_work_size(self.N * self.Q)
                .arg(self.f_buffer.as_ref().unwrap())
                .arg(self.f_new_buffer.as_ref().unwrap())
                .build()
                .expect("Failed to build OpenCL 'swap_kernel'."),
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
        // Note: Kernel size is not included in VRAM usage calculation as it cannot be easily determined
        let mut total_vram = 0;

        // Add size of f buffer
        if let Some(buffer) = &self.f_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of f_new buffer
        if let Some(buffer) = &self.f_new_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of density buffer
        if let Some(buffer) = &self.density_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of velocity buffer
        if let Some(buffer) = &self.u_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of flags buffer
        if let Some(buffer) = &self.flags_buffer {
            total_vram += buffer.len() * size_of::<i32>();
        }

        println!(
            "VRAM usage: {:.2} MB",
            total_vram as f64 / (1024.0 * 1024.0)
        );
        terminal_utils::print_success("OpenCL device and context initialized successfully!");
    }
}
