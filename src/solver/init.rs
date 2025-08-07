#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use super::lbm::LBM;

use crate::solver::transforms::xyz_from_n;
use crate::utils::velocity::Velocity;
use crate::solver::precision::PrecisionMode;


impl LBM {
    pub fn new(
        Nx: usize,
        Ny: usize,
        Nz: usize,
        model: String,
        viscosity: f32,
        precision: PrecisionMode,
    ) -> Self {
        let size = Nx * Ny * Nz;
        let Q = match model.clone().as_str() {
            "D2Q9" => 9,
            "D3Q7" => 7,
            "D3Q15" => 15,
            "D3Q19" => 19,
            "D3Q27" => 27,
            _ => panic!("Unsupported model: {}", model),
        };

        println!(
            "Initializing LBM with precision mode: {} - {}",
            format!("{:?}", precision).to_uppercase(),
            precision.description()
        );

        let (f_storage, f_compute_buffer) = match precision {
            PrecisionMode::FP16S => {
                (Some(vec![0u16; size * Q]), Some(vec![0.0f32; size * Q]))
            }
            _ => (None, None),
        };

        LBM {
            Nx,
            Ny,
            Nz,
            N: size,
            model: model.clone(),
            Q,
            viscosity,
            omega: 1.0 / (3.0 * viscosity + 0.5),
            time_steps: 0,
            precision_mode: precision,
            f: vec![0.0; size * Q],
            f_new: vec![0.0; size * Q],
            f_storage,
            f_compute_buffer,
            density: vec![1.0; size], // Initialize density to 1.0
            u: vec![0.0; size * 3], // Initialize velocity to zero (size * 3 for 3 components per grid point)
            velocity: vec![Velocity::zero(); size], // Initialize input velocity to zero
            flags: vec![0u8; size],   // Initialize flags to 0 (fluid)
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
            // streaming_kernel: None,
            // collision_kernel: None,
            stream_collide_kernel: None,
            equilibrium_kernel: None,
            found_errors: false,
            output_interval: 0,
            output_csv: false,
            output_vtk: false,
        }
    }

    pub fn initialize(&mut self) {
        self.platform = Some(
            self.get_ocl_platform()
                .expect("Failed to get OpenCL platform"),
        );
        self.device = Some(self.get_ocl_device().expect("Failed to get OpenCL device"));
        self.context = Some(
            self.get_ocl_context()
                .expect("Failed to get OpenCL context"),
        );
        self.queue = Some(self.get_ocl_queue().expect("Failed to get OpenCL queue"));
        self.program = Some(
            self.get_ocl_program()
                .expect("Failed to generate OpenCL program."),
        );
        self.f_buffer = Some(
            self.reserve_f_buffer()
                .expect("Failed to reserve f_buffer."),
        );
        self.f_new_buffer = Some(
            self.reserve_f_new_buffer()
                .expect("Failed to reserve f_new_buffer."),
        );
        self.density_buffer = Some(
            self.reserve_density_buffer()
                .expect("Failed to reserve density_buffer."),
        );
        self.u_buffer = Some(
            self.reserve_u_buffer()
                .expect("Failed to reserve u_buffer."),
        );
        self.flags_buffer = Some(
            self.reserve_flags_buffer()
                .expect("Failed to reserve flags_buffer."),
        );

        self.create_equilibrium_kernel()
            .expect("Failed to create 'equilibrium kernel'.");

        self.create_stream_collide_kernel()
            .expect("Failed to create 'stream_collide' kernel.");

        self.calculate_vram_usage();
    }

    pub fn set_conditions<F>(&mut self, f: F)
    where
        F: Fn(&mut LBM, usize, usize, usize, usize), // x, y, z, n
    {
        for n in 0..self.N {
            // Get the x, y, z coordinates from the linear index n
            let (x, y, z) = xyz_from_n(&n, &self.Nx, &self.Ny);
            // Call the user-defined lambda function
            f(self, x, y, z, n);
        }
        self.u = self.velocity_to_u(); // Transform 3D array to Flattened array
        self.velocity = vec![];
    }
}
