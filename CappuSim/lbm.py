# src.lbm.py

# External Imports
import pyopencl as cl
import pyopencl.tools
import numpy as np
from time import perf_counter
from os import path
from tqdm import tqdm

# Internal Imports
from .utils import velocities_sets, unflatten, Config, py_to_cl

# Kernel Path
lib_dir = path.dirname(__file__)
stream_collide_kernel = lib_dir + r"\stream_collide_kernel.cl"

class LBM:
    def __init__(self, config):
        # Check if the config is a Config object
        if isinstance(config, Config):
            config = config.get()
        # Set configuration
        self.config = config
        self.initial_time = perf_counter()
        self.velocity_set = config['velocities_set']
        self.simtype = config['simtype']
        self.use_temperature = config['use_temperature']
        self.use_graphics = config['use_graphics']
        self.grid_size = config['grid_size']
        self.viscosity = config['viscosity']
        self.total_timesteps = config['total_timesteps']
        self.dtype = config['dtype']
        self.select_dtype()     # FP32, FP16, FP64

        self.D = int(self.velocity_set[1])    # Number of dimensions
        self.Q = int(self.velocity_set[3])    # Number of velocities
        self.Nx = int(self.grid_size[0])    # Number of cells in x direction
        self.Ny = int(self.grid_size[1])    # Number of cells in y direction
        self.Nz = int(self.grid_size[2])    # Number of cells in z direction
        self.N = self.Nx * self.Ny * self.Nz
        
        self.cs = 1.0/np.sqrt(3)    # Sound speed
        self.cs2 = 1.0/3.0  # Sound speed squared
        self.tau = 3.0 * self.viscosity + 0.5   # Relaxation time
        self.omega = 1.0 / self.tau   # Relaxation Frequency
        
        # Set velocity set variables c, w and opposite
        if self.velocity_set in velocities_sets:  # config.velocity_sets
            self.c, self.w = np.array(velocities_sets[self.velocity_set]["c"], np.int32), np.array(velocities_sets[self.velocity_set]["w"], self.dtype)
        else:
            raise ValueError(f"Unknown velocity set: {self.velocities_set}")

        # Initialize arrays (flattened)
        self.f = np.ones((self.N, self.Q), dtype=self.dtype)
        self.rho = np.ones(self.N, dtype=self.dtype)
        self.u = np.zeros((self.N, self.D), dtype=self.dtype)
        self.flags = np.zeros((self.N, self.Q), dtype=np.int32)  # All cells are fluid cells
    
        self.initializeOpenCL() # Initialize OpenCL device
        self.initializeOpenCLKernels()  # Initialize OpenCL kernels
        print("LBM initialized successfully.")

    def run(self, timesteps):
        # Initialize OpenCL Buffers
        self.initializeBuffers()
        
        # Run the simulation for a given number of timesteps
        # Start the timer
        self.start_time = perf_counter()
        
        # Calculate VRAM usage
        self.get_vram_usage([self.f_buf, self.rho_buf, self.u_buf, self.c_buf, self.w_buf])
        
        # Run the simulation with a progress bar
        progress_bar = tqdm(range(timesteps), desc="CappuSim", unit="step")
        for _ in progress_bar:
            # Get the current steps/s value from tqdm
            steps_per_second = progress_bar.format_dict["rate"]
            # Calculate MLUps
            if steps_per_second is not None and steps_per_second > 0:
                MLUps = (self.N * steps_per_second) / 1e6  # Convert to millions
            else:
                MLUps = 0.0
            progress_bar.set_postfix(MLUps=f"{MLUps:.2f}")
            
            self.program.lbm_collide_and_stream(
                self.queue, (self.Nx, self.Ny, self.Nz), None,
                self.f_buf, self.rho_buf, self.u_buf, self.c_buf, self.w_buf,
                self.dtype(self.omega), np.int32(self.Q), np.int32(self.D),
                np.int32(self.Nx), np.int32(self.Ny), np.int32(self.Nz)
            )
        print("Simulation completed successfully.")
        #self.queue.finish()
        # End the timer
        print(f"Total execution time: {perf_counter() - self.start_time:.2f} seconds.")
        return
    
    def setInitialConditions(self, func):
        self.initializeBuffers()    # Initialize OpenCL Buffers
        initial_conditions_kernel = py_to_cl(func)  # Translate Python function to OpenCL kernel
        print(initial_conditions_kernel)    # Debugging
        self.program_IC = cl.Program(self.ctx, initial_conditions_kernel).build()    # Compile the kernel
        self.program_IC.initial_conditions(    
            self.queue, (self.Nx, self.Ny, self.Nz), None,
            self.rho_buf, self.u_buf, self.flags_buf,
            np.int32(self.Nx), np.int32(self.Ny), np.int32(self.Nz)
        ) # Run the kernel
        # Transfer the results back to the host
        cl.enqueue_copy(self.queue, self.u, self.u_buf)
        cl.enqueue_copy(self.queue, self.rho, self.rho_buf)
        cl.enqueue_copy(self.queue, self.flags, self.flags_buf)

    def get_results(self):
        cl.enqueue_copy(self.queue, self.rho, self.rho_buf)
        cl.enqueue_copy(self.queue, self.u, self.u_buf)
        return self.rho, self.u
    
    def initializeOpenCL(self):
        # Initialize OpenCL
        # Automatically select the best GPU
        platforms = cl.get_platforms()
        devices = []
        for platform in platforms:
            devices.extend(platform.get_devices(device_type=cl.device_type.GPU))
        
        if not devices:
            raise RuntimeError("No GPU devices found.")
        
        # Select the GPU with the highest compute units
        self.best_device = max(devices, key=lambda d: d.get_info(cl.device_info.MAX_COMPUTE_UNITS))
        
        self.ctx = cl.Context(devices=[self.best_device])
        self.queue = cl.CommandQueue(self.ctx)
        print(f"Using device: {self.best_device.name}")

    def initializeOpenCLKernels(self):
        # Load and compile the OpenCL kernel
        with open(stream_collide_kernel, "r") as f:
            kernel_code = f.read()
        self.program = cl.Program(self.ctx, kernel_code).build()
        
    def initializeBuffers(self):
        # Transfer arrays to device
        self.mf = cl.mem_flags
        self.f_buf = cl.Buffer(self.ctx, self.mf.READ_WRITE | self.mf.COPY_HOST_PTR, hostbuf=self.f)
        self.rho_buf = cl.Buffer(self.ctx, self.mf.READ_WRITE | self.mf.COPY_HOST_PTR, hostbuf=self.rho)
        self.u_buf = cl.Buffer(self.ctx, self.mf.READ_WRITE | self.mf.COPY_HOST_PTR, hostbuf=self.u)
        self.c_buf = cl.Buffer(self.ctx, self.mf.READ_ONLY | self.mf.COPY_HOST_PTR, hostbuf=self.c)
        self.w_buf = cl.Buffer(self.ctx, self.mf.READ_ONLY | self.mf.COPY_HOST_PTR, hostbuf=self.w)
        self.flags_buf = cl.Buffer(self.ctx, self.mf.READ_ONLY | self.mf.COPY_HOST_PTR, hostbuf=self.flags)
    
    def select_dtype(self):
        # Select the appropriate data type based on the simulation type
        if self.dtype == 'FP32':
            self.dtype = np.float32
        elif self.dtype == 'FP16':
            self.dtype = np.float16
        elif self.dtype == 'FP64':
            self.dtype = np.float64
        else:
            raise ValueError("Unknown simulation type")
    
    def get_vram_usage(self, buffers):
        """
        Calculate and display VRAM usage.

        Args:
            context: The OpenCL context.
            buffers: A list of OpenCL buffers allocated by your program.
        """
        
        # Get total VRAM available on the device
        total_vram = self.best_device.get_info(cl.device_info.GLOBAL_MEM_SIZE)  # In bytes

        # Calculate VRAM used by your code (sum of buffer sizes)
        used_by_code = sum(buf.size for buf in buffers)  # In bytes

        # Calculate percentages
        used_by_code_percent = (used_by_code / total_vram) * 100 if total_vram > 0 else 0

        # Print VRAM usage
        print(f"Total VRAM: {total_vram / (1024 ** 2):.2f} MB")
        print(f"CappuSim used VRAM: {used_by_code / (1024 ** 2):.2f} MB ({used_by_code_percent:.2f}%)")
    
    def importSTL(self, filename):
        pass