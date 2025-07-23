use super::lbm::LBM;
use crate::utils::terminal_utils;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use ocl;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub model: String,
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub grid_size: usize,
    pub time_steps: usize,
    pub elapsed_time: f64,
    pub mlups: f64,
    pub memory_usage_mb: f64,
    pub device_name: String,
    pub platform_name: String,
    pub compute_units: u32,
    pub max_work_group_size: usize,
    pub global_memory_gb: f64,
    pub local_memory_kb: f64,
    pub cell_memory_bytes: f64, // Add this line
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_name: String,
    pub platform_name: String,
    pub compute_units: u32,
    pub max_work_group_size: usize,
    pub global_memory_gb: f64,
    pub local_memory_kb: f64,
}

impl LBM {
    /// Runs comprehensive benchmarks for different models and grid sizes
    pub fn benchmark() {
        println!("{}", "=".repeat(80));
        terminal_utils::print_success("Starting CappuSim Benchmark Suite");
        println!("{}", "=".repeat(80));
        
        let mut results = Vec::new();
        
        // Define benchmark configurations
        let configs = Self::get_benchmark_configs();
        
        let total_tests = configs.len();
        println!("Running {} benchmark configurations...\n", total_tests);
        
        for (i, config) in configs.iter().enumerate() {
            println!("Progress: [{}/{}] Testing {} {}×{}×{}", 
                i + 1, total_tests, config.model, config.nx, config.ny, config.nz);
            
            match Self::run_single_benchmark(config) {
                Ok(result) => {
                    Self::print_benchmark_result(&result);
                    results.push(result);
                }
                Err(e) => {
                    terminal_utils::print_error(&format!("Failed to run benchmark for {} {}×{}×{}: {}", 
                        config.model, config.nx, config.ny, config.nz, e));
                }
            }
            println!("{}", "-".repeat(80));
        }
        
        // Save results to CSV
        match Self::save_results_to_csv(&results) {
            Ok(filename) => {
                terminal_utils::print_success(&format!("Benchmark results saved to: {}", filename));
            }
            Err(e) => {
                terminal_utils::print_error(&format!("Failed to save CSV: {}", e));
            }
        }
        
        // Print summary
        Self::print_benchmark_summary(&results);
    }
    
    /// Defines all benchmark configurations to test
    fn get_benchmark_configs() -> Vec<BenchmarkConfig> {
        let mut configs = Vec::new();
        
        // 2D Models (D2Q9)
        let grid_sizes_2d = vec![
            (64, 64, 1),
            (128, 128, 1),
            (256, 256, 1),
            (512, 512, 1),
            (1024, 1024, 1),
            (2048, 2048, 1),
        ];
        
        for (nx, ny, nz) in grid_sizes_2d {
            configs.push(BenchmarkConfig {
                model: "D2Q9".to_string(),
                nx, ny, nz,
                time_steps: 500,
                viscosity: 0.1,
            });
        }
        
        // 3D Models
        let models_3d = vec!["D3Q7", "D3Q15", "D3Q19", "D3Q27"];
        let grid_sizes_3d = vec![
            (32, 32, 32),
            (64, 64, 64),
            (100, 100, 100),
            (128, 128, 128),
            (256, 256, 256),
        ];
        
        for model in models_3d {
            for (nx, ny, nz) in &grid_sizes_3d {
                configs.push(BenchmarkConfig {
                    model: model.to_string(),
                    nx: *nx, ny: *ny, nz: *nz,
                    time_steps: 250, // Fewer timesteps for 3D due to computational cost
                    viscosity: 0.1,
                });
            }
        }
        
        configs
    }
    
    /// Runs a single benchmark configuration
    fn run_single_benchmark(config: &BenchmarkConfig) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        // Create LBM instance
        let mut lbm = LBM::new(config.nx, config.ny, config.nz, config.model.clone(), config.viscosity);
        
        // Set simple initial conditions (fluid everywhere)
        lbm.set_conditions(|lbm, _x, _y, _z, n| {
            lbm.flags[n] = 0; // FLAG_FLUID
            lbm.density[n] = 1.0;
            lbm.velocity[n].x = 0.0;
            lbm.velocity[n].y = 0.0;
            lbm.velocity[n].z = 0.0;
        });
        
        // Initialize OpenCL
        lbm.initialize();
        
        // Get device information
        let device_info = Self::get_device_info(&lbm)?;
        
        // Calculate memory usage
        let memory_usage_mb = Self::calculate_memory_usage(&lbm);
        
        // Run benchmark
        let start_time = Instant::now();
        
        // Initialize equilibrium
        unsafe {
            lbm.equilibrium_kernel
                .as_ref()
                .unwrap()
                .enq()
                .expect("Failed to enqueue equilibrium kernel");
            lbm.queue
                .as_ref()
                .unwrap()
                .finish()
                .expect("Queue finish failed");
        }
        
        // Main simulation loop (simplified version of run method)
        for t in 0..config.time_steps {
            // Collision
            unsafe {
                let kernel = lbm.collision_kernel.as_ref().unwrap();
                kernel.set_arg(6, &(t as i32))
                    .expect("Failed to set kernel argument");
                kernel.enq()
                    .expect("Failed to enqueue collision kernel");
                lbm.queue
                    .as_ref()
                    .unwrap()
                    .finish()
                    .expect("Queue finish failed");
            }
            
            // Streaming
            unsafe {
                let kernel = lbm.streaming_kernel.as_ref().unwrap();
                kernel.set_arg(3, &(t as i32))
                    .expect("Failed to set kernel argument");
                kernel.enq()
                    .expect("Failed to enqueue streaming kernel");
                lbm.queue
                    .as_ref()
                    .unwrap()
                    .finish()
                    .expect("Queue finish failed");
            }
        }
        
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();
        
        // Calculate MLUps
        let mlups = (lbm.N as f64 * config.time_steps as f64) / elapsed_seconds / 1_000_000.0;
        
        // Calculate cell memory usage
        let cell_memory_bytes = (
            lbm.Q * 2 * 4 + // f and f_new: Q floats each, 4 bytes per float
            1 * 4 +         // density: 1 float
            3 * 4 +         // velocity: 3 floats
            1 * 4           // flags: 1 i32
        ) as f64;
        
        Ok(BenchmarkResult {
            model: config.model.clone(),
            nx: config.nx,
            ny: config.ny,
            nz: config.nz,
            grid_size: lbm.N,
            time_steps: config.time_steps,
            elapsed_time: elapsed_seconds,
            mlups,
            memory_usage_mb,
            device_name: device_info.device_name,
            platform_name: device_info.platform_name,
            compute_units: device_info.compute_units,
            max_work_group_size: device_info.max_work_group_size,
            global_memory_gb: device_info.global_memory_gb,
            local_memory_kb: device_info.local_memory_kb,
            cell_memory_bytes,
        })
    }
    
    /// Gets device information from the LBM instance
    fn get_device_info(lbm: &LBM) -> Result<DeviceInfo, Box<dyn std::error::Error>> {
        let device = lbm.device.as_ref().unwrap();
        let platform = lbm.platform.as_ref().unwrap();
        
        let device_name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());
        let platform_name = platform.name().unwrap_or_else(|_| "Unknown Platform".to_string());
        
        let compute_units = match device.info(ocl::enums::DeviceInfo::MaxComputeUnits)? {
            ocl::enums::DeviceInfoResult::MaxComputeUnits(units) => units,
            _ => 0,
        };
        
        let max_work_group_size = match device.info(ocl::enums::DeviceInfo::MaxWorkGroupSize)? {
            ocl::enums::DeviceInfoResult::MaxWorkGroupSize(size) => size,
            _ => 0,
        };
        
        let global_memory_bytes = match device.info(ocl::enums::DeviceInfo::GlobalMemSize)? {
            ocl::enums::DeviceInfoResult::GlobalMemSize(size) => size as usize,
            _ => 0,
        };
        let global_memory_gb = global_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        
        let local_memory_bytes = match device.info(ocl::enums::DeviceInfo::LocalMemSize)? {
            ocl::enums::DeviceInfoResult::LocalMemSize(size) => size as usize,
            _ => 0,
        };
        let local_memory_kb = local_memory_bytes as f64 / 1024.0;
        
        Ok(DeviceInfo {
            device_name,
            platform_name,
            compute_units,
            max_work_group_size,
            global_memory_gb,
            local_memory_kb,
        })
    }
    
    /// Calculates approximate memory usage in MB
    fn calculate_memory_usage(lbm: &LBM) -> f64 {
        let bytes_per_f32 = 4;
        let bytes_per_i32 = 4;
        
        let cell_memory_bytes = (
            lbm.Q * 2 * bytes_per_f32 + // f and f_new: Q floats each, 4 bytes per float
            1 * bytes_per_f32 +         // density: 1 float
            3 * bytes_per_f32 +         // velocity: 3 floats
            1 * bytes_per_i32           // flags: 1 i32
        ) as f64;
        
        let total_bytes = lbm.N as f64 * cell_memory_bytes;
        total_bytes / (1024.0 * 1024.0) // Convert to MB
    }
    
    /// Calculates memory usage per cell in bytes
    fn calculate_cell_memory_usage(lbm: &LBM) -> f64 {
        let bytes_per_f32 = 4;
        let bytes_per_i32 = 4;
        
        // Memory usage for one cell (density, velocity, flags, and Q distributions)
        let cell_memory_bytes = (
            1 * bytes_per_f32 +     // density
            3 * bytes_per_f32 +     // velocity
            1 * bytes_per_i32 +     // flags
            lbm.Q * 2 * bytes_per_f32 // f and f_new
        ) as f64;
        
        cell_memory_bytes
    }
    
    /// Prints result for a single benchmark
    fn print_benchmark_result(result: &BenchmarkResult) {
        println!("  Model: {}", result.model);
        println!("  Grid: {}×{}×{} ({} cells)", result.nx, result.ny, result.nz, result.grid_size);
        println!("  Time steps: {}", result.time_steps);
        println!("  Elapsed time: {:.3}s", result.elapsed_time);
        println!("  Performance: {:.2} MLUps", result.mlups);
        println!("  Memory usage: {:.1} MB", result.memory_usage_mb);
        println!("  Device: {} ({} CUs)", result.device_name, result.compute_units);
    }
    
    /// Saves benchmark results to CSV file
    fn save_results_to_csv(results: &[BenchmarkResult]) -> Result<String, Box<dyn std::error::Error>> {
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::fs;
        
        // Create benchmarks directory if it doesn't exist
        let benchmarks_dir = "benchmarks";
        fs::create_dir_all(benchmarks_dir)?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let filename = format!("{}/benchmark_results_{}.csv", benchmarks_dir, timestamp);
        
        let mut file = File::create(&filename)?;
        
        // Write CSV header
        writeln!(file, "Model,Nx,Ny,Nz,GridSize,TimeSteps,ElapsedTime,MLUps,MemoryUsageMB,CellMemoryBytes,DeviceName,PlatformName,ComputeUnits,MaxWorkGroupSize,GlobalMemoryGB,LocalMemoryKB")?;
        
        // Write data rows
        for result in results {
            writeln!(file, "{},{},{},{},{},{},{:.6},{:.6},{:.2},{:.2},{},{},{},{},{:.2},{:.1}",
                result.model,
                result.nx,
                result.ny, 
                result.nz,
                result.grid_size,
                result.time_steps,
                result.elapsed_time,
                result.mlups,
                result.memory_usage_mb,
                result.cell_memory_bytes, // Add here
                result.device_name,
                result.platform_name,
                result.compute_units,
                result.max_work_group_size,
                result.global_memory_gb,
                result.local_memory_kb
            )?;
        }
        
        Ok(filename)
    }
    
    /// Prints benchmark summary
    fn print_benchmark_summary(results: &[BenchmarkResult]) {
        if results.is_empty() {
            return;
        }
        
        println!("\n{}", "=".repeat(80));
        terminal_utils::print_success("Benchmark Summary");
        println!("{}", "=".repeat(80));
        
        // Group results by model
        let mut model_results: std::collections::HashMap<String, Vec<&BenchmarkResult>> = std::collections::HashMap::new();
        
        for result in results {
            model_results.entry(result.model.clone()).or_insert_with(Vec::new).push(result);
        }
        
        println!("Performance by model:");
        for (model, model_res) in &model_results {
            let max_mlups = model_res.iter().map(|r| r.mlups).fold(0.0f64, f64::max);
            let avg_mlups = model_res.iter().map(|r| r.mlups).sum::<f64>() / model_res.len() as f64;
            
            let best = model_res.iter().max_by(|a, b| a.mlups.partial_cmp(&b.mlups).unwrap()).unwrap();
            
            println!("  {}: Max {:.2} MLUps ({}×{}×{}), Avg {:.2} MLUps ({} configs)", 
                model, max_mlups, best.nx, best.ny, best.nz, avg_mlups, model_res.len());
        }
        
        // Overall best
        let best_overall = results.iter().max_by(|a, b| a.mlups.partial_cmp(&b.mlups).unwrap());
        if let Some(best) = best_overall {
            println!("\nOverall best performance:");
            println!("  {}: {:.2} MLUps ({}×{}×{})", 
                best.model, best.mlups, best.nx, best.ny, best.nz);
        }
        
        // Overall statistics
        let total_mlups: f64 = results.iter().map(|r| r.mlups).sum();
        let avg_mlups = total_mlups / results.len() as f64;
        
        println!("\nOverall statistics:");
        println!("  Total configurations tested: {}", results.len());
        println!("  Average performance: {:.2} MLUps", avg_mlups);
        println!("{}", "=".repeat(80));
    }
}

#[derive(Debug, Clone)]
struct BenchmarkConfig {
    model: String,
    nx: usize,
    ny: usize, 
    nz: usize,
    time_steps: usize,
    viscosity: f32,
}