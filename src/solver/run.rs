#![allow(non_snake_case)] // Allow non-snake_case naming convention
#![allow(clippy::upper_case_acronyms)] // Allow uppercase acronyms

use super::lbm::LBM;
use crate::utils::terminal_utils;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Instant;

impl LBM {
    pub fn run(&mut self, time_steps: usize) {
        // Print welcome message
        terminal_utils::print_welcome_message();
        self.time_steps = time_steps;
        println!("{}", "-".repeat(72));

        // Check for errors in input parameters
        if let Err(err) = self.check_errors_in_input() {
            terminal_utils::print_error(&format!("Error: {}", err));
            return;
        }

        // Initialize OpenCL
        self.initialize();

        terminal_utils::print_name();

        // Initialize f in equilibrium from rho and u
        unsafe {
            self.equilibrium_kernel
                .as_ref()
                .unwrap()
                .enq()
                .expect("Failed to enqueue 'collision_kernel'.");
            self.queue
                .as_ref()
                .unwrap()
                .finish()
                .expect("Queue finish failed.");
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
            std::fs::remove_dir_all(output_path)
                .expect("Failed to remove existing output directory.");
        }
        std::fs::create_dir(output_path).expect("Failed to create output directory.");

        // Start timing
        let start_time = Instant::now();

        // Main Loop
        for t in 0..self.time_steps {
            // Execute kernels
            // Collision process
            unsafe {
                let kernel = self.collision_kernel.as_ref().unwrap();
                kernel.set_arg(6, &(t as i32))
                    .expect("Failed to set kernel argument.");
                kernel.enq()
                    .expect("Failed to enqueue 'collision_kernel'.");
                self.queue
                    .as_ref()
                    .unwrap()
                    .finish()
                    .expect("Queue finish failed.");
            }
            // Streaming process
            unsafe {
                let kernel = self.streaming_kernel.as_ref().unwrap();
                kernel.set_arg(3, &(t as i32))
                    .expect("Failed to set kernel argument.");
                kernel.enq()
                    .expect("Failed to enqueue 'streaming_kernel'.");
                self.queue
                    .as_ref()
                    .unwrap()
                    .finish()
                    .expect("Queue finish failed.");
            }

            // Output data
            if (self.output_interval != 0) && (t % self.output_interval == 0) {
                // Read data from GPU to CPU
                if let Err(err) = self.read_from_gpu() {
                    terminal_utils::print_error(&format!("Error reading data from GPU: {}", err));
                    return;
                }
                let magnitude = self.time_steps.to_string().len();
                if self.output_csv {
                    // Export data to output csv file
                    let filename = format!("output/data_{:0width$}.csv", t, width = magnitude);
                    if let Err(err) = self.output_to_csv(&filename.to_string()) {
                        terminal_utils::print_error(&format!("Error exporting data: {}", err));
                        return;
                    }
                }
                if self.output_vtk {
                    // Export data to VTK file
                    let filename = format!("output/data_{:0width$}.vtk", t, width = magnitude);
                    if let Err(err) = self.export_to_vtk(&filename) {
                        terminal_utils::print_error(&format!("Error exporting VTK data: {}", err));
                        return;
                    }
                }
            }

            pb.inc(1); // Increment the progress bar by 1
        }
        // Read data from GPU to CPU
        if let Err(err) = self.read_from_gpu() {
            terminal_utils::print_error(&format!("Error reading data from GPU: {}", err));
            return;
        }
        pb.finish_with_message("");

        // Calculate total execution time
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();

        // Calculate MLUps
        let mlups = (self.N as f64 * self.time_steps as f64) / elapsed_seconds / 1_000_000.0; // Performance in Millions Lattice Updates per Second (MLUps)

        terminal_utils::print_metrics(self.time_steps as u64, elapsed_seconds, mlups);
    }
}
