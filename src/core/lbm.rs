// src/core/lbm.rs

#[allow(unused)]
use ocl::{Buffer, Kernel, Context, Queue};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};


use crate::core::kernels;
use crate::utils::ocl_utils;
use crate::utils::terminal_utils;
use crate::utils::lbm_velocity_sets;

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Velocity { x, y, z }
    }
    
    pub fn zero() -> Self {
        Velocity { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub struct LBM {
    pub Nx: usize,
    pub Ny: usize,
    pub Nz: usize,
    pub N: usize,
    pub model: String,
    pub viscosity: f32,
    pub time_steps: usize,
    pub density: Vec<f32>,
    pub velocity: Vec<Velocity>,
    pub flags: Vec<u32>,
    context: Option<Context>,
    queue: Option<Queue>,
    buffer: Option<Buffer<f32>>,
}

impl LBM {
    pub fn new(Nx: usize, Ny: usize, Nz: usize, model: String, viscosity: f32) -> Self {
        let size = Nx * Ny * Nz;
        LBM {
            Nx,
            Ny,
            Nz,
            N: size,
            model,
            viscosity,
            time_steps: 0,
            density: vec![1.0; size],   // Initialize density to 1.0
            velocity: vec![Velocity::zero(); size], // Initialize velocity to zero
            flags: vec![0; size],       // Initialize flags to 0 (fluid)
            context: None,
            queue: None,
            buffer: None,
        }
    }

    // Initialize OpenCL and store resources in the Lbm object
    fn initialize_ocl(&mut self) {
        // Initialize OpenCL
        match ocl_utils::setup_opencl() {
            Ok((_platform, _device, context, queue)) => {
                // Store the context and queue in the Lbm object
                self.context = Some(context);
                self.queue = Some(queue.clone());

                // Create a buffer and store it in the Lbm object
                let buffer = ocl::Buffer::<f32>::builder()
                    .queue(queue)
                    .len(1024)
                    .fill_val(0.0f32)
                    .build()
                    .expect("Failed to create buffer");

                self.buffer = Some(buffer);

                println!("OpenCL initialized and buffer created successfully!");
            }
            Err(e) => {
                eprintln!("Failed to initialize OpenCL: {}", e);
            }
        }
    }

    pub fn update(&mut self) {
        self.collide();
        self.stream();
        self.apply_boundary_conditions();
    }

    pub fn collide(&mut self) {
        // Apply collision logic
    }

    pub fn stream(&mut self) {
        // Streaming logic
    }

    pub fn apply_boundary_conditions(&mut self) {
        // Apply boundary conditions logic
    }

    pub fn run(&mut self, time_steps: usize) {
        self.time_steps = time_steps;
        
        // Initialize OpenCL
        self.initialize_ocl();
        
        // Create a progress bar
        let pb = ProgressBar::new(self.time_steps as u64);

        // Customize the progress bar style (optional)
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("=> "),
        );

         // Start timing
        let start_time = Instant::now();

        // Simulate work by iterating and updating the progress bar -> REMOVE
        for _ in 0..100 {
            self.update();
            // Call OpenCL kernels for collision and streaming
            // Example: self.queue.enqueue_kernel(&self.kernel_collision, &self.buffer);
            thread::sleep(Duration::from_millis(50)); // Simulate work
            pb.inc(1); // Increment the progress bar by 1
        }

        // Finish the progress bar
        pb.finish_with_message("Done!");

        // Calculate total execution time
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();

        // Calculate MLUps
        let mlups = (self.N as f64 * self.time_steps as f64) / elapsed_seconds / 1_000_000.0;

        // Print metrics
        println!("Performance: (MLUps): {:.2}", mlups);
        println!("Total execution time: {:.2} seconds", elapsed_seconds);
        
    }
}