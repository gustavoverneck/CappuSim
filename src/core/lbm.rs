// src/core/lbm.rs

//#![allow(unused)]           // Allow unused imports and warnings
#![allow(non_snake_case)]   // Allow non-snake_case naming convention
use ocl::{Platform, Device, Context, Queue, Program, Buffer, Kernel};
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::mem::swap;
use crate::utils::terminal_utils;
use crate::utils::velocity::{Velocity};
use crate::core::kernels::LBM_KERNEL;

pub struct LBM {
    pub Nx: usize,
    pub Ny: usize,
    pub Nz: usize,
    pub N: usize,
    pub model: String,
    D: usize,
    Q: usize,
    pub viscosity: f32,
    pub omega: f32,
    pub time_steps: usize,
    f: Vec<f32>,
    f_new: Vec<f32>,
    pub density: Vec<f32>,
    pub u: Vec<Velocity>,
    pub flags: Vec<u8>,
    f_buffer: Option<Buffer<f32>>,
    f_new_buffer: Option<Buffer<f32>>,
    density_buffer: Option<Buffer<f32>>,
    velocity_buffer: Option<Buffer<f32>>,
    flags_buffer: Option<Buffer<u8>>,
    platform: Option<Platform>,
    device: Option<Device>,
    context: Option<Context>,
    queue: Option<Queue>,
    program: Option<Program>,
    streaming_kernel: Option<Kernel>,
    collision_kernel: Option<Kernel>,
    found_errors: bool,
}

impl LBM {
    pub fn new(Nx: usize, Ny: usize, Nz: usize, model: String, viscosity: f32) -> Self {
        let size = Nx * Ny * Nz;
        let Q = match model.clone().as_str() {
            "D2Q9" => 9,
            "D3Q7" => 7,
            "D3Q15" => 15,
            "D3Q19" => 19,
            "D3Q27" => 27,
            _ => panic!("Unsupported model: {}", model),
        };

        LBM {
            Nx,
            Ny,
            Nz,
            N: size,
            model: model.clone(),
            D: match model.clone().as_str() {
                "D2Q9" => 2,
                "D3Q7" | "D3Q15" | "D3Q19" | "D3Q27" => 3,
                _ => panic!("Unsupported model: {}", model),
            },
            Q,
            viscosity,
            omega: 1.0 / (3.0 * viscosity + 0.5),
            time_steps: 0,
            f: vec![0.0; size * Q],
            f_new: vec![0.0; size * Q],
            density: vec![1.0; size],   // Initialize density to 1.0
            u: vec![Velocity::zero(); size], // Initialize velocity to zero
            flags: vec![0; size],       // Initialize flags to 0 (fluid)
            f_buffer: None,
            f_new_buffer: None,
            density_buffer: None,
            velocity_buffer: None,
            flags_buffer: None,
            platform: None,
            device: None,
            context: None,
            queue: None,
            program: None,
            streaming_kernel: None,
            collision_kernel: None,
            found_errors: false,
        }
    }

    fn initialize_ocl(&mut self) -> Result<(), Box<dyn Error>> {
        // Select default platform and device
        self.platform = Some(Platform::list()
            .into_iter()
            .next()
            .ok_or("Platform not found")?);
        println!("Platform: {}", self.platform.as_ref().unwrap().name()?);
        
        self.device = Some(Device::list_all(self.platform.as_ref().unwrap())?
            .into_iter()
            .next()
            .ok_or("Device not found")?);
        println!("Device: {}", self.device.as_ref().unwrap().name()?);

        // Create a context for the selected device
        self.context = Some(Context::builder()
            .platform(self.platform.unwrap())
            .devices(self.device.unwrap().clone())
            .build()
            .expect("Failed to build context."));
        
        // Create a command queue for the device
        self.queue = Some(Queue::new(self.context.as_ref().unwrap(), self.device.unwrap().clone(), None)
            .expect("Failed to create command queue."));

        // Write defines on kernel
        let kernel_source = format!(
                                r#"
                #define NX {}
                #define NY {}
                #define NZ {}
                #define Q {}
                #define {}
                {}"#, self.Nx, self.Ny, self.Nz, self.Q, self.model.as_str(), LBM_KERNEL);
        //println!("{}", kernel_source); //Debug: print final kernel

        // Define OpenCL program
        self.program = Some(Program::builder()
            .src(kernel_source)
            .devices(self.device.as_ref().unwrap())
            .build(self.context.as_ref().unwrap())
            .expect("Failed to build program."));

        // Create buffers
        self.f_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(self.f.len()) // Ensures correct buffer size for 'f'
        .copy_host_slice(&self.f)
        .build()
        .expect("Failed to build 'f' buffer."));

        self.f_new_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(self.u.len() * self.Q) // Ensures correct buffer size for 'f_new'
        .copy_host_slice(&self.f_new)
        .build()
        .expect("Failed to build 'f_new' buffer."));

        self.density_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(self.density.len()) // Correct size for 'density'
        .copy_host_slice(&self.density)
        .build()
        .expect("Failed to build 'density' buffer."));

        // Optimized velocity buffer creation
        let velocity_data: Vec<f32> = self.u.iter()
        .flat_map(|v| [v.x, v.y, v.z])
        .collect();

        self.velocity_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(velocity_data.len()) // Corrected size — directly matches velocity data length
        .copy_host_slice(&velocity_data)
        .build()
        .expect("Failed to build 'velocity' buffer."));

        // Corrected flags buffer size
        self.flags_buffer = Some(Buffer::<u8>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(self.flags.len()) // Corrected size for 'flags'
        .copy_host_slice(&self.flags)
        .build()
        .expect("Failed to build 'flags' buffer."));


        // Create kernels and set its arguments
        self.streaming_kernel = Some(Kernel::builder()
            .program(self.program.as_ref().unwrap())
            .name("streaming_kernel")
            .queue(self.queue.as_ref().unwrap().clone())
            .global_work_size(self.N)
            .arg(self.f_buffer.as_ref().unwrap())
            .arg(self.f_new_buffer.as_ref().unwrap())
            .build()
            .expect("Failed to build OpenCL 'streaming_kernel'."));
        
        // Create kernels and set its arguments
        self.collision_kernel = Some(Kernel::builder()
            .program(self.program.as_ref().unwrap())
            .name("collision_kernel")
            .queue(self.queue.as_ref().unwrap().clone())
            .global_work_size(self.N)
            .arg(self.f_buffer.as_ref().unwrap())
            .arg(self.density_buffer.as_ref().unwrap())
            .arg(self.velocity_buffer.as_ref().unwrap())
            .arg(&self.omega)
            .build()
            .expect("Failed to build OpenCL 'collision_kernel'."));

        println!("VRAM usage: {:.2} MB", self.calculate_vram_usage() as f64 / (1024.0 * 1024.0));        
        terminal_utils::print_success("OpenCL device and context initialized successfully!");
        Ok(())
    }

    pub fn calculate_vram_usage(&self) -> usize {
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
        if let Some(buffer) = &self.velocity_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of flags buffer
        if let Some(buffer) = &self.flags_buffer {
            total_vram += buffer.len() * size_of::<u8>();
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
        let expected_size = self.Nx * self.Ny * self.Nz;
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
        F: Fn(&mut LBM, usize, usize, usize, usize),    // x, y, z, n
    {
        for n in 0..self.N {
            // Get the x, y, z coordinates from the linear index n
            let (x, y, z) = xyz_from_n(&n, &self.Nx, &self.Ny, &self.Nz);
            // Call the user-defined lambda function
            f(self, x, y, z, n);
        }
    }

    // Read data from GPU to CPU
    fn read_from_gpu(&mut self) -> Result<(), Box<dyn Error>> {
        // Velocity
        let mut flat_velocity_data = vec![0.0; self.u.len() * 3]; // Create a flat buffer
        self.velocity_buffer
            .as_ref()
            .unwrap()
            .read(&mut flat_velocity_data)
            .enq()
            .expect("Failed to read 'velocity' buffer.");
    
        // Map flat vector to vec<Velocity>
        self.u_to_velocity(flat_velocity_data);

        // Density
        self.density_buffer
        .as_ref()
        .unwrap()
        .read(&mut self.density)
        .enq()
        .expect("Failed to read 'density' buffer.");    
        Ok(())
    }

    fn velocity_to_u(&self) -> Vec<f32> {
        self.u.iter()
            .flat_map(|v| vec![v.x, v.y, v.z])
            .collect()
    }

    fn u_to_velocity(&mut self, flat_velocity_data: Vec<f32>) {
        self.u = flat_velocity_data
            .chunks(3)
            .map(|chunk| Velocity {
                x: chunk[0],
                y: chunk[1],
                z: chunk[2],
            })
            .collect();
    }

    fn equilibrium(&self, rho: &f32, u: &Velocity, i: usize) -> f32 {
        let c: &[[i32; 3]] = match self.model.as_str() {
            "D2Q9" => &[
                [0, 0, 0], [1, 0, 0], [0, 1, 0], [-1, 0, 0], [0, -1, 0],
                [1, 1, 0], [-1, 1, 0], [-1, -1, 0], [1, -1, 0]
            ],
            "D3Q7" => &[
                [0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1],
                [-1, 0, 0], [0, -1, 0], [0, 0, -1]
            ],
            "D3Q15" => &[
                [0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1],
                [-1, 0, 0], [0, -1, 0], [0, 0, -1], [1, 1, 0],
                [-1, 1, 0], [-1, -1, 0], [1, -1, 0], [1, 0, 1],
                [-1, 0, 1], [-1, 0, -1], [1, 0, -1]
            ],
            "D3Q19" => &[
                [0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1],
                [-1, 0, 0], [0, -1, 0], [0, 0, -1], [1, 1, 0],
                [-1, 1, 0], [-1, -1, 0], [1, -1, 0], [1, 0, 1],
                [-1, 0, 1], [-1, 0, -1], [1, 0, -1], [0, 1, 1],
                [0, -1, 1], [0, -1, -1], [0, 1, -1]
            ],
            "D3Q27" => &[
                [0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1],
                [-1, 0, 0], [0, -1, 0], [0, 0, -1], [1, 1, 0],
                [-1, 1, 0], [-1, -1, 0], [1, -1, 0], [1, 0, 1],
                [-1, 0, 1], [-1, 0, -1], [1, 0, -1], [0, 1, 1],
                [0, -1, 1], [0, -1, -1], [0, 1, -1], [1, 1, 1],
                [-1, 1, 1], [-1, -1, 1], [1, -1, 1], [1, 1, -1],
                [-1, 1, -1], [-1, -1, -1], [1, -1, -1]
            ],
            _ => panic!("Unsupported model: {}", self.model),
        };

        let w: &[f32] = match self.model.as_str() {
            "D2Q9" => &[4.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0],
            "D3Q7" => &[1.0 / 4.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0],
            "D3Q15" => &[2.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0],
            "D3Q19" => &[1.0 / 3.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0],
            "D3Q27" => &[8.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0],
            _ => panic!("Unsupported model: {}", self.model),
        };

        let cu = c[i].iter().zip(&[u.x, u.y, u.z]).map(|(ci, ui)| *ci as f32 * ui).sum::<f32>();
        let u2 = u.x * u.x + u.y * u.y + u.z * u.z;
        w[i] * *rho * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2)
    }

    pub fn run(&mut self, time_steps: usize) {
        // Print LatteLab welcome message
        terminal_utils::print_welcome_message();
        self.time_steps = time_steps;
        println!("{}", "-".repeat(72));

        // Check for errors in input parameters
        if let Err(err) = self.check_errors_in_input() {
            terminal_utils::print_error(&format!("Error: {}", err));
            return;
        }

        // Initialize f in equilibrium from rho and u
        for n in 0..self.N {
            let (x, y, z) = xyz_from_n(&n, &self.Nx, &self.Ny, &self.Nz);
            let rho = self.density[n];
            let u = self.u[n];
            for i in 0..self.Q {
                self.f[n * self.Q + i] = self.equilibrium(&rho, &u, i);
            }
        }

        // Initialize OpenCL
        self.initialize_ocl();
        terminal_utils::print_name();
        
        // Create a progress bar
        let pb = ProgressBar::new(self.time_steps as u64);

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
            // Execute kernels
            unsafe {
                self.streaming_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'streaming_kernel'.");
                self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
                self.collision_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'collision_kernel'.");
                self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
            }
        
            // Swap f and f_new buffers
            swap(&mut self.f_buffer, &mut self.f_new_buffer);
        
            pb.inc(1); // Increment the progress bar by 1
        }
        // End queue
        self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");

        // Copy buffers from GPU to CPU
        self.read_from_gpu().expect("Failed to read data from GPU.");


        // Finish the progress bar
        pb.finish_with_message("Done!");

        // Calculate total execution time
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();

        // Calculate MLUps
        let mlups = (self.N as f64 * self.time_steps as f64) / elapsed_seconds / 1_000_000.0;   // Performance in Millions Lattice Updates per Second (MLUps)

        terminal_utils::print_metrics(
            self.time_steps as u64,
            elapsed_seconds,
            mlups,
        );
    }

    fn calculate_vorticity(&self, x: usize, y: usize, z: usize) -> f32 {
        let dx = 1.0; /// self.Nx as f32;
        let dy = 1.0; /// self.Ny as f32;
        let dz = 1.0; /// self.Nz as f32;

        let get_velocity = |x, y, z| {
            if x >= self.Nx || y >= self.Ny || z >= self.Nz {
                Velocity::zero()
            } else {
                let index = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny, &self.Nz);
                self.u[index]
            }
        };

        let du_dy = (get_velocity(x, y + 1, z).x - get_velocity(x, y.saturating_sub(1), z).x) / (2.0 * dy);
        let du_dz = (get_velocity(x, y, z + 1).x - get_velocity(x, y, z.saturating_sub(1)).x) / (2.0 * dz);
        let dv_dx = (get_velocity(x + 1, y, z).y - get_velocity(x.saturating_sub(1), y, z).y) / (2.0 * dx);
        let dv_dz = (get_velocity(x, y, z + 1).y - get_velocity(x, y, z.saturating_sub(1)).y) / (2.0 * dz);
        let dw_dx = (get_velocity(x + 1, y, z).z - get_velocity(x.saturating_sub(1), y, z).z) / (2.0 * dx);
        let dw_dy = (get_velocity(x, y + 1, z).z - get_velocity(x, y.saturating_sub(1), z).z) / (2.0 * dy);

        let vorticity_x = dw_dy - dv_dz;
        let vorticity_y = du_dz - dw_dx;
        let vorticity_z = dv_dx - du_dy;

        (vorticity_x * vorticity_x + vorticity_y * vorticity_y + vorticity_z * vorticity_z).sqrt()
    }

    pub fn output_to(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        if self.found_errors {
            return Err("Errors were found in the input parameters. Cannot write output.".into());
        }
        // Create the file and wrap it in a BufWriter for better performance
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        // Write the header
        writeln!(writer, "x, y, z, rho,      ux,       uy,       uz,       v")?;
        
        // Iterate over the grid and write the data
        for x in 0..self.Nx {
            for y in 0..self.Ny {
            for z in 0..self.Nz {
                // Calculate the linear index
                let index = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny, &self.Nz);
                // Get density and velocity
                let rho = &self.density[index];
                let u = &self.u[index];
                
                // Calculate vorticity
                let vorticity = self.calculate_vorticity(x, y, z);
        
                // Write the data to the file
                writeln!(
                writer,
                "{}, {}, {}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}", // Format floating-point numbers to 6 decimal places
                x, y, z, rho, u.x, u.y, u.z, vorticity
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


pub fn n_from_xyz(x: &usize, y: &usize, z: &usize, Nx: &usize, Ny: &usize, Nz: &usize) -> usize {
    x + Nx * (y + Ny * z)
}
pub fn xyz_from_n(n: &usize, Nx: &usize, Ny: &usize, Nz: &usize) -> (usize, usize, usize) {
    let x = *n % Nx;
    let y = (*n / Nx) % Ny;
    let z = *n / (Ny * Nx);
    (x, y, z)
}
