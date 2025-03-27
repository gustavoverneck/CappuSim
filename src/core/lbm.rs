#![allow(non_snake_case)]   // Allow non-snake_case naming convention

/// The `LBM` struct represents a Lattice Boltzmann Method (LBM) simulation.
/// It encapsulates the grid dimensions, physical parameters, and OpenCL resources
/// required for the simulation.
///
/// # Fields
/// - `Nx`, `Ny`, `Nz`: Dimensions of the simulation grid in the x, y, and z directions.
/// - `N`: Total number of grid points (Nx * Ny * Nz).
/// - `model`: The LBM model used (e.g., "D2Q9", "D3Q19").
/// - `D`: Number of spatial dimensions (2 or 3).
/// - `Q`: Number of discrete velocity directions in the LBM model.
/// - `viscosity`: Kinematic viscosity of the fluid.
/// - `omega`: Relaxation parameter derived from viscosity.
/// - `time_steps`: Number of time steps to simulate.
/// - `f`, `f_new`: Distribution functions for the LBM simulation.
/// - `density`: Fluid density at each grid point.
/// - `u`: Flattened velocity vector (3 components per grid point).
/// - `velocity`: Velocity vector as a `Velocity` struct for each grid point.
/// - `flags`: Flags indicating the type of each grid point (fluid, solid, eq).
/// - `f_buffer`, `f_new_buffer`, `density_buffer`, `u_buffer`, `flags_buffer`: OpenCL buffers for simulation data.
/// - `platform`, `device`, `context`, `queue`, `program`: OpenCL resources for GPU computation.
/// - `streaming_kernel`, `collision_kernel`, `swap_kernel`, `equilibrium_kernel`: OpenCL kernels for LBM operations.
/// - `found_errors`: Indicates if errors were found in the input parameters.
/// - `output_interval`: Interval for exporting simulation data to files.
///
/// # Methods
/// - `new`: Creates a new `LBM` instance with the specified grid dimensions, model, and viscosity.
/// - `initialize_ocl`: Initializes OpenCL resources and builds kernels for the simulation.
/// - `calculate_vram_usage`: Calculates the total VRAM usage of the simulation.
/// - `check_errors_in_input`: Validates the input parameters for the simulation.
/// - `set_conditions`: Sets initial conditions for the simulation using a user-defined function.
/// - `read_from_gpu`: Reads simulation data from GPU buffers to CPU memory.
/// - `run`: Runs the LBM simulation for a specified number of time steps.
/// - `calculate_vorticity`: Calculates the vorticity magnitude at a specific grid point.
/// - `calculate_q_criteria`: Calculates the Q-criteria at a specific grid point.
/// - `output_to`: Exports simulation data to a CSV file.
/// - `set_output_interval`: Sets the interval for exporting simulation data.
/// - `get_density`: Retrieves the density data from the GPU.
/// - `get_velocity`: Retrieves the velocity data from the GPU.
/// - `velocity_to_u`: Converts the `velocity` vector to a flattened `u` vector.
/// - `u_to_velocity`: Converts a flattened `u` vector to the `velocity` vector.
///
/// # Utility Functions
/// - `n_from_xyz`: Converts 3D grid coordinates (x, y, z) to a linear index.
/// - `xyz_from_n`: Converts a linear index to 3D grid coordinates (x, y, z).
// src/core/lbm.rs

use ocl::{Platform, Device, Context, Queue, Program, Buffer, Kernel, ProQue};
use ocl::flags::MEM_READ_WRITE;
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;
use crate::utils::velocity::Velocity;
use crate::utils::terminal_utils;
use crate::core::kernels::LBM_KERNEL;

// LBM FLAGS
pub const FLAG_FLUID: i32 = 0;
pub const FLAG_SOLID: i32 = 1;
pub const FLAG_EQ: i32 = 2;

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
    u: Vec<f32>,
    pub velocity: Vec<Velocity>,
    pub flags: Vec<i32>,
    f_buffer: Option<Buffer<f32>>,
    f_new_buffer: Option<Buffer<f32>>,
    density_buffer: Option<Buffer<f32>>,
    u_buffer: Option<Buffer<f32>>,
    flags_buffer: Option<Buffer<i32>>,
    platform: Option<Platform>,
    device: Option<Device>,
    context: Option<Context>,
    queue: Option<Queue>,
    program: Option<Program>,
    streaming_kernel: Option<Kernel>,
    collision_kernel: Option<Kernel>,
    swap_kernel: Option<Kernel>,
    equilibrium_kernel: Option<Kernel>,
    found_errors: bool,
    output_interval: usize,
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
        let D = match model.clone().as_str() {
            "D2Q9" => 2,
            "D3Q7" => 3,
            "D3Q15" => 3,
            "D3Q19" => 3,
            "D3Q27" => 3,
            _ => panic!("Unsupported model: {}", model),
        };

        LBM {
            Nx,
            Ny,
            Nz,
            N: size,
            model: model.clone(),
            D,
            Q,
            viscosity,
            omega: 1.0 / (3.0 * viscosity + 0.5),
            time_steps: 0,
            f: vec![0.0; size * Q],
            f_new: vec![0.0; size * Q],
            density: vec![1.0; size],   // Initialize density to 1.0
            u: vec![0.0; size * 3], // Initialize velocity to zero (size * 3 for 3 components per grid point)
            velocity: vec![Velocity::zero(); size], // Initialize input velocity to zero
            flags: vec![0; size],       // Initialize flags to 0 (fluid)
            f_buffer: None,
            f_new_buffer: None,
            density_buffer: None,
            u_buffer: None,
            flags_buffer: None,
            platform: None,
            device: None,
            context: None,
            queue: None,
            program: None,
            streaming_kernel: None,
            collision_kernel: None,
            swap_kernel: None,
            equilibrium_kernel: None,
            found_errors: false,
            output_interval: 0,
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
        let kernel_source = format!(r#"
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
        "#, self.Nx, self.Ny, self.Nz, self.N, self.Q, self.model.as_str(), LBM_KERNEL);
        //println!("{}", kernel_source); //Debug: print final kernel

        // Define OpenCL program
        self.program = Some(Program::builder()
            .src(kernel_source)
            .devices(self.device.as_ref().unwrap())
            .build(self.context.as_ref().unwrap())
            .expect("Failed to build program."));

        // Create f buffer
        self.f_buffer = Some(Buffer::<f32>::builder()
            .queue(self.queue.as_ref().unwrap().clone())
            .flags(MEM_READ_WRITE)
            .len(self.N * self.Q) // Ensures correct buffer size for 'f'
            .copy_host_slice(&self.f)
            .build()
            .expect("Failed to build 'f' buffer."));

        // Create f_new buffer
        self.f_new_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(MEM_READ_WRITE)
        .len(self.N * self.Q) // Ensures correct buffer size for 'f_new'
        .copy_host_slice(&self.f_new)
        .build()
        .expect("Failed to build 'f_new' buffer."));

        // Create density buffer
        self.density_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(MEM_READ_WRITE)
        .len(self.N) // Correct size for 'density'
        .copy_host_slice(&self.density)
        .build()
        .expect("Failed to build 'density' buffer."));
        
        // Create velocity buffer
        self.u_buffer = Some(Buffer::<f32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(MEM_READ_WRITE)
        .len(self.N * 3)
        .copy_host_slice(&self.u)
        .build()
        .expect("Failed to build 'velocity' buffer."));

        // Create kernels and set its arguments
        self.flags_buffer = Some(Buffer::<i32>::builder()
        .queue(self.queue.as_ref().unwrap().clone())
        .flags(MEM_READ_WRITE)
        .len(self.N) // Corrected size for 'flags'
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
            .arg(self.flags_buffer.as_ref().unwrap())
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
            .arg(self.flags_buffer.as_ref().unwrap())
            .arg(self.u_buffer.as_ref().unwrap())
            .arg(&self.omega)
            .build()
            .expect("Failed to build OpenCL 'collision_kernel'."));
        
        // Create swap kernel and set its arguments
        self.swap_kernel = Some(Kernel::builder()
            .program(self.program.as_ref().unwrap())
            .name("swap")
            .queue(self.queue.as_ref().unwrap().clone())
            .global_work_size(self.N * self.Q)
            .arg(self.f_buffer.as_ref().unwrap())
            .arg(self.f_new_buffer.as_ref().unwrap())
            .build()
            .expect("Failed to build OpenCL 'swap_kernel'."));

        // Create equilibrium kernel for initial conditions
        self.equilibrium_kernel = Some(Kernel::builder()
            .program(self.program.as_ref().unwrap())
            .name("equilibrium")
            .queue(self.queue.as_ref().unwrap().clone())
            .global_work_size(self.N)
            .arg(self.f_buffer.as_ref().unwrap())
            .arg(self.density_buffer.as_ref().unwrap())
            .arg(self.u_buffer.as_ref().unwrap())
            .build()
            .expect("Failed to build OpenCL 'equilibrium_kernel'."));

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
        if let Some(buffer) = &self.u_buffer {
            total_vram += buffer.len() * size_of::<f32>();
        }

        // Add size of flags buffer
        if let Some(buffer) = &self.flags_buffer {
            total_vram += buffer.len() * size_of::<i32>();
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
        self.u = self.velocity_to_u(); // Transform 3D array to Flattened array
        self.velocity = vec![Velocity::zero(); 0];
    }

    // Read data from GPU to CPU
    fn read_from_gpu(&mut self) -> Result<(), Box<dyn Error>> {        
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

        // Initialize OpenCL
        self.initialize_ocl();
        terminal_utils::print_name();

        // Initialize f in equilibrium from rho and u
        unsafe {
            self.equilibrium_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'collision_kernel'.");
            self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
        }

        // Create a progress bar
        let pb = ProgressBar::new(self.time_steps as u64);

        // Customize the progress bar style (optional)
        pb.set_style(
            ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:55.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
        );

        // Recreate output folder -> FUTURE FEATURE: add folder and file customization
        let output_path = Path::new("output");
        if output_path.exists() {
            std::fs::remove_dir_all(output_path).expect("Failed to remove existing output directory.");
        }
        std::fs::create_dir(output_path).expect("Failed to create output directory.");

        // Start timing
        let start_time = Instant::now();

        // Main Loop
        for t in 0..self.time_steps {
            // Execute kernels
            // Collision process
            unsafe {
                self.collision_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'collision_kernel'.");
                self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
            }
            // Streaming process
            unsafe {
                self.streaming_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'streaming_kernel'.");
                self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
            }
            // Swap f and f_new after streaming
            unsafe {
                self.swap_kernel.as_ref().unwrap().enq().expect("Failed to enqueue 'swap_kernel'.");
                self.queue.as_ref().unwrap().finish().expect("Queue finish failed.");
            }
            
            // Output data
            if self.output_interval != 0 {
                if t % self.output_interval == 0 {
                    // Read data from GPU to CPU
                    if let Err(err) = self.read_from_gpu() {
                        terminal_utils::print_error(&format!("Error reading data from GPU: {}", err));
                        return;
                    }
                    // Export data to output file
                    let magnitude = self.time_steps.to_string().len();
                    let filename = format!("output/data_{:0width$}.csv", t, width = magnitude);
                    if let Err(err) = self.output_to(&format!("{}", filename)) {
                        terminal_utils::print_error(&format!("Error exporting data: {}", err));
                        return;
                    }
                }
            }
            
            pb.inc(1); // Increment the progress bar by 1
        }
        pb.finish_with_message("");

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

    pub fn calculate_vorticity(&self, x: usize, y: usize, z: usize) -> f32 {
        let dx = 1.0;
        let dy = 1.0;
        let dz = 1.0;
    
        let get = |x, y, z, d| -> f32 {
            if x >= self.Nx || y >= self.Ny || z >= self.Nz {
                0.0
            } else {
                let i = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny, &self.Nz);
                self.u[i * 3 + d]
            }
        };
    
        // Curl components
        let du_dy = (get(x, y + 1, z, 0) - get(x, y.saturating_sub(1), z, 0)) / (2.0 * dy);
        let du_dz = (get(x, y, z + 1, 0) - get(x, y, z.saturating_sub(1), 0)) / (2.0 * dz);
    
        let dv_dx = (get(x + 1, y, z, 1) - get(x.saturating_sub(1), y, z, 1)) / (2.0 * dx);
        let dv_dz = (get(x, y, z + 1, 1) - get(x, y, z.saturating_sub(1), 1)) / (2.0 * dz);
    
        let dw_dx = (get(x + 1, y, z, 2) - get(x.saturating_sub(1), y, z, 2)) / (2.0 * dx);
        let dw_dy = (get(x, y + 1, z, 2) - get(x, y.saturating_sub(1), z, 2)) / (2.0 * dy);
    
        let vort_x = dw_dy - dv_dz;
        let vort_y = du_dz - dw_dx;
        let vort_z = dv_dx - du_dy;
    
        (vort_x * vort_x + vort_y * vort_y + vort_z * vort_z).sqrt()
    }

    pub fn calculate_q_criteria(&self, x: usize, y: usize, z: usize) -> f32 {
        let dx = 1.0;
        let dy = 1.0;
        let dz = 1.0;
    
        let get = |x, y, z, d| -> f32 {
            if x >= self.Nx || y >= self.Ny || z >= self.Nz {
                0.0
            } else {
                let i = n_from_xyz(&x, &y, &z, &self.Nx, &self.Ny, &self.Nz);
                self.u[i * 3 + d]
            }
        };
    
        // Partial derivatives of velocity components
        let du_dx = (get(x + 1, y, z, 0) - get(x.saturating_sub(1), y, z, 0)) / (2.0 * dx);
        let du_dy = (get(x, y + 1, z, 0) - get(x, y.saturating_sub(1), z, 0)) / (2.0 * dy);
        let du_dz = (get(x, y, z + 1, 0) - get(x, y, z.saturating_sub(1), 0)) / (2.0 * dz);
    
        let dv_dx = (get(x + 1, y, z, 1) - get(x.saturating_sub(1), y, z, 1)) / (2.0 * dx);
        let dv_dy = (get(x, y + 1, z, 1) - get(x, y.saturating_sub(1), z, 1)) / (2.0 * dy);
        let dv_dz = (get(x, y, z + 1, 1) - get(x, y, z.saturating_sub(1), 1)) / (2.0 * dz);
    
        let dw_dx = (get(x + 1, y, z, 2) - get(x.saturating_sub(1), y, z, 2)) / (2.0 * dx);
        let dw_dy = (get(x, y + 1, z, 2) - get(x, y.saturating_sub(1), z, 2)) / (2.0 * dy);
        let dw_dz = (get(x, y, z + 1, 2) - get(x, y, z.saturating_sub(1), 2)) / (2.0 * dz);
    
        // Symmetric (strain rate) tensor S_ij
        let s_xx = du_dx;
        let s_yy = dv_dy;
        let s_zz = dw_dz;
        let s_xy = 0.5 * (du_dy + dv_dx);
        let s_xz = 0.5 * (du_dz + dw_dx);
        let s_yz = 0.5 * (dv_dz + dw_dy);
    
        // Anti-symmetric (vorticity) tensor W_ij
        let w_xy = 0.5 * (du_dy - dv_dx);
        let w_xz = 0.5 * (du_dz - dw_dx);
        let w_yz = 0.5 * (dv_dz - dw_dy);
    
        // Frobenius norms
        let s_norm = s_xx.powi(2) + s_yy.powi(2) + s_zz.powi(2) + 2.0 * (s_xy.powi(2) + s_xz.powi(2) + s_yz.powi(2));
        let w_norm = 2.0 * (w_xy.powi(2) + w_xz.powi(2) + w_yz.powi(2));
    
        0.5 * (w_norm - s_norm)
    }

    pub fn output_to(&self, path: &str) -> Result<(), Box<dyn Error>> {
        if self.found_errors {
            return Err("Errors were found in the input parameters. Cannot write output.".into());
        }
        // Create the file and wrap it in a BufWriter for better performance
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        // Write the header
        writeln!(writer, "x, y, z, rho,      ux,       uy,       uz,       v,       q")?;
        
        // Iterate over the grid and write the data
        for n in 0..self.N {
            // Get the x, y, z coordinates from the linear index n
            let (x, y, z) = xyz_from_n(&n, &self.Nx, &self.Ny, &self.Nz);
            // Get density and velocity
            let rho = &self.density[n];
            let ux = self.u[n * 3];
            let uy = self.u[n * 3 + 1];
            let uz = self.u[n * 3 + 2];
            
            // Calculate vorticity
            let vorticity = self.calculate_vorticity(x, y, z);
            let q_criteria = self.calculate_q_criteria(x, y, z);
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

    pub fn get_density(&self) -> Vec<f32> {
        if let Some(buffer) = &self.density_buffer {
            let mut density_data = vec![0.0; self.N];
            buffer.read(&mut density_data).enq().expect("Failed to read 'density' buffer.");
            return density_data;
        }
        vec![] // Return an empty vector if the buffer is not available
    }

    pub fn get_velocity(&self) -> Vec<Velocity> {
        if let Some(buffer) = &self.u_buffer {
            let mut velocity_data = vec![0.0; self.N * 3];
            buffer.read(&mut velocity_data).enq().expect("Failed to read 'velocity' buffer.");
            return velocity_data
                .chunks(3)
                .map(|chunk| Velocity {
                    x: chunk[0],
                    y: chunk[1],
                    z: chunk[2],
                })
                .collect();
        }
        vec![] // Return an empty vector if the buffer is not available
    }

    fn velocity_to_u(&self) -> Vec<f32> {
        self.velocity.iter()
            .flat_map(|v| vec![v.x, v.y, v.z])
            .collect()
    }

    fn u_to_velocity(&mut self, flat_velocity_data: Vec<f32>) {
        self.velocity = flat_velocity_data
            .chunks(3)
            .map(|chunk| Velocity {
                x: chunk[0],
                y: chunk[1],
                z: chunk[2],
            })
            .collect();
    }    
}

pub fn n_from_xyz(x: &usize, y: &usize, z: &usize, Nx: &usize, Ny: &usize, Nz: &usize) -> usize {
    z * (Nx * Ny) + y * Nx + x
}
pub fn xyz_from_n(n: &usize, Nx: &usize, Ny: &usize, Nz: &usize) -> (usize, usize, usize) {
    let x = *n % Nx;
    let y = (*n / Nx) % Ny;
    let z = *n / (Ny * Nx);
    (x, y, z)
}