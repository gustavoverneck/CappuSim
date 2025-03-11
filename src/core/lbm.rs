// src/core/lbm.rs

#![allow(unused)]           // Allow unused imports and warnings
#![allow(non_snake_case)]   // Allow non-snake_case naming convention
use ocl::{Platform, Device, Context, Queue, ProQue, Buffer, Kernel};
use ocl::enums::DeviceInfo;
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;

use crate::utils::terminal_utils;
use crate::utils::velocity::{Velocity};
use crate::core::kernels::LBM_KERNEL;

pub struct LBM {
    pub Nx: u64,
    pub Ny: u64,
    pub Nz: u64,
    pub N: u64,
    pub model: String,
    pub viscosity: f32,
    pub time_steps: u64,
    pub density: Vec<f32>,
    pub u: Vec<Velocity>,
    pub flags: Vec<u8>,
    context: Option<Context>,
    queue: Option<Queue>,
    pro_que: Option<ProQue>,
    density_buffer: Option<Buffer<f32>>,
    velocity_buffer: Option<Buffer<f32>>,
    flags_buffer: Option<Buffer<u8>>,
    streaming_kernel: Option<Kernel>,
    collision_kernel: Option<Kernel>,
    found_errors: bool,
}

impl LBM {
    pub fn new(Nx: u64, Ny: u64, Nz: u64, model: String, viscosity: f32) -> Self {
        let size = Nx * Ny * Nz;
        LBM {
            Nx,
            Ny,
            Nz,
            N: size,
            model,
            viscosity,
            time_steps: 0,
            density: vec![1.0; size as usize],   // Initialize density to 1.0
            u: vec![Velocity::zero(); size as usize], // Initialize velocity to zero
            flags: vec![0; size as usize],       // Initialize flags to 0 (fluid)
            context: None,
            queue: None,
            pro_que: None,
            density_buffer: None,
            velocity_buffer: None,
            flags_buffer: None,
            streaming_kernel: None,
            collision_kernel: None,
            found_errors: false,
        }
    }

    fn initialize_ocl(&mut self) -> Result<(), Box<dyn Error>> {
        // Step 1: Get the first available platform
        let platform = Platform::list().into_iter().next().ok_or("Platform not found")?;
        println!("Platform: {}", platform.name()?);
    
        // Step 2: Get the first available device
        let device = Device::list_all(&platform)?
            .into_iter()
            .next()
            .ok_or("Device not found")?;
        println!("Device: {}", device.name()?);
    
        // Step 3: Create a context and command queue
        let context = Context::builder()
            .platform(platform)
            .devices(device.clone())
            .build()?;
    
        let queue = Queue::new(&context, device, None)?;
    
        // Step 4: Compile the OpenCL kernel
        let kernel_source = match self.model.as_str() {
            "D2Q9" => format!("#define D2Q9\n{}", LBM_KERNEL),
            "D3Q7" => format!("#define D3Q7\n{}", LBM_KERNEL),
            "D3Q15" => format!("#define D3Q15\n{}", LBM_KERNEL),
            "D3Q19" => format!("#define D3Q19\n{}", LBM_KERNEL),
            "D3Q27" => format!("#define D3Q27\n{}", LBM_KERNEL),
            _ => return Err("Unsupported model".into()),
        };
    
        // Create the ProQue object with the specified dimensions
        let pro_que = ProQue::builder()
            .context(context.clone())
            .src(kernel_source)
            .dims((self.Nx as usize, self.Ny as usize, self.Nz as usize)) // Set the dimensions here
            .build()
            .map_err(|e| {
                eprintln!("OpenCL kernel compilation failed: {}", e);
                e
            })?;
    
        // Step 5: Create OpenCL buffers with the correct size
        let density_buffer = pro_que.buffer_builder::<f32>()
        .len(self.density.len()) // Ensure this matches the grid size
        .copy_host_slice(&self.density)
        .build()?;

        let velocity_buffer = pro_que.buffer_builder::<f32>()
        .len(self.u.len() * 3) // Ensure this matches the grid size (3 components per velocity)
        .copy_host_slice(&self.u.iter().flat_map(|v| vec![v.x, v.y, v.z]).collect::<Vec<f32>>())
        .build()?;

        let flags_buffer = pro_que.buffer_builder::<u8>()
        .len(self.flags.len()) // Ensure this matches the grid size
        .copy_host_slice(&self.flags)
        .build()?;
    
        // Step 6: Store resources in the LBM struct
        self.context = Some(context);
        self.queue = Some(queue);
        self.pro_que = Some(pro_que);
        self.density_buffer = Some(density_buffer);
        self.velocity_buffer = Some(velocity_buffer);
        self.flags_buffer = Some(flags_buffer);

        println!("VRAM usage: {:.2} MB", self.calculate_vram_usage() as f64 / (1024.0 * 1024.0));

        // Step 7: Create and store the kernels
        self.streaming_kernel = Some(self.pro_que.as_ref().unwrap().kernel_builder("streaming_kernel")
            .arg(self.density_buffer.as_ref().unwrap())
            .arg(self.velocity_buffer.as_ref().unwrap())
            .arg(self.Nx as i32)
            .arg(self.Ny as i32)
            .arg(self.Nz as i32)
            .build()?);
            
        self.collision_kernel = Some(self.pro_que.as_ref().unwrap().kernel_builder("collision_kernel")
            .arg(self.density_buffer.as_ref().unwrap())
            .arg(self.Nx as i32)
            .arg(self.Ny as i32)
            .arg(self.Nz as i32)
            .arg(1.0 / self.viscosity) // omega = 1 / tau
            .build()?);

        terminal_utils::print_success("OpenCL device and context initialized successfully!");
        Ok(())
    }

    pub fn calculate_vram_usage(&self) -> usize {
        // Note: Kernel size is not included in VRAM usage calculation as it cannot be easily determined
        let mut total_vram = 0;

        // Add size of density buffer
        if let Some(buffer) = &self.density_buffer {
            total_vram += buffer.len() * std::mem::size_of::<f32>();
        }

        // Add size of velocity buffer
        if let Some(buffer) = &self.velocity_buffer {
            total_vram += buffer.len() * std::mem::size_of::<f32>();
        }

        // Add size of flags buffer
        if let Some(buffer) = &self.flags_buffer {
            total_vram += buffer.len() * std::mem::size_of::<u8>();
        }
        total_vram
    }

    
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

        // Check if D2Q9 model is used with Nz != 0
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
        let expected_size = (self.Nx * self.Ny * self.Nz) as usize;
        if self.density.len() != expected_size {
            self.found_errors = true;
            return Err("Density vector has incorrect length.".into());
        }
        if self.u.len() != expected_size {
            self.found_errors = true;
            return Err("Velocity vector has incorrect length.".into());
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

    pub fn set_conditions<F>(&mut self, f: F)
    where
        F: Fn(&mut LBM, usize, usize, usize, usize),
    {
        for n in 0..self.N as usize {
            // Get the x, y, z coordinates from the linear index n
            let (x, y, z) = xyz_from_n(n, self.Nx, self.Ny);
            // Call the user-defined lambda function
            f(self, x as usize, y as usize, z as usize, n);
        }
    }

    pub fn collide(&mut self) {
        if let Some(kernel) = &self.collision_kernel {
            unsafe {
                kernel.enq().expect("Failed to enqueue collision kernel");
            }
        }
    }

    pub fn stream(&mut self) {
        if let Some(kernel) = &self.streaming_kernel {
            unsafe {
                kernel.enq().expect("Failed to enqueue streaming kernel");
            }
        }
    }

    // Read data from GPU to CPU
    fn read_from_gpu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Read density buffer
        if let Some(buffer) = &self.density_buffer {
            buffer.read(&mut self.density).enq()?;
            //println!("Density data read from GPU.");
        } else {
            terminal_utils::print_error("Density buffer not found.");
        }
    
        // Read velocity buffer
        if let Some(buffer) = &self.velocity_buffer {
            let mut velocity_data_flat = vec![0.0; self.N as usize * 3];
            buffer.read(&mut velocity_data_flat).enq()?;
            self.u = velocity_data_flat
                .chunks(3)
                .map(|chunk| Velocity::new(chunk[0], chunk[1], chunk[2]))
                .collect();
            //println!("Velocity data read from GPU.");
        } else {
            terminal_utils::print_error("Velocity buffer not found.");
        }
    
        Ok(())
    }

    pub fn run(&mut self, time_steps: u64) {
        // Print LatteLab welcome message
        terminal_utils::print_welcome_message();
        self.time_steps = time_steps as u64;
        println!("{}", "-".repeat(72));

        // Check for errors in input parameters
        if let Err(err) = self.check_errors_in_input() {
            terminal_utils::print_error(&format!("Error: {}", err));
            return;
        }

        // Initialize OpenCL
        self.initialize_ocl();
        terminal_utils::print_name();
        
        // Create a progress bar
        let pb = ProgressBar::new(self.time_steps);

        // Customize the progress bar style (optional)
        pb.set_style(
            ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:55.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
        );

        // Start timing
        let start_time = Instant::now();

        // Main Loop
        for _ in 0..self.time_steps {
            // Update the simulation state
            self.stream();
            self.collide();
            pb.inc(1); // Increment the progress bar by 1
        }
        // Copy buffers from GPU to CPU
        self.read_from_gpu().expect("Failed to read data from GPU.");

        // Finish the progress bar
        pb.finish_with_message("Done!");

        // Calculate total execution time
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();

        // Calculate MLUps
        let mlups = (self.N as f64 * self.time_steps as f64) / elapsed_seconds / 1_000_000.0;   // Performance in Million Lattice Updates per Second (MLUps)

        terminal_utils::print_metrics(
            self.time_steps,
            elapsed_seconds,
            mlups,  
        );
    }

    pub fn output_to(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.found_errors {
            return Err("Errors were found in the input parameters. Cannot write output.".into());
        }
        // Create the file and wrap it in a BufWriter for better performance
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
    
        // Write the header
        writeln!(writer, "x, y, z, rho, v")?;
    
        // Iterate over the grid and write the data
        for z in 0..self.Nz {
            for y in 0..self.Ny {
                for x in 0..self.Nx {
                    // Calculate the linear index
                    let index = n_from_xyz(x, y, z, self.Nx, self.Ny);
    
                    // Get density and velocity
                    let rho = self.density[index];
                    let u = &self.u[index];
    
                    // Calculate the absolute velocity
                    let abs_u = (u.x.powi(2) + u.y.powi(2) + u.z.powi(2)).sqrt();
    
                    // Write the data to the file
                    writeln!(
                        writer,
                        "{}, {}, {}, {:.6}, {:.8}", // Format floating-point numbers to 6 decimal places
                        x, y, z, rho, abs_u
                    )?;
                }
            }
        }
    
        // Flush the buffer to ensure all data is written to the file
        writer.flush()?;
    
        println!("Simulation results have been written to {}", path);
        Ok(())
    }
}


pub fn n_from_xyz(x: u64, y: u64, z: u64, Nx: u64, Ny: u64) -> usize {
    (z * Ny * Nx + y * Nx + x) as usize
}

pub fn xyz_from_n(index: usize, Nx: u64, Ny: u64) -> (u64, u64, u64) {
    let z = (index as u64) / (Ny * Nx);
    let y = ((index as u64) % (Ny * Nx)) / Nx;
    let x = (index as u64) % Nx;
    (x, y, z)
}