// src/core/kernels.rs

pub const LBM_KERNEL: &str = r#"
// Define the number of directions based on the model
#if defined(D2Q9)
#define NUM_DIRECTIONS 9
#elif defined(D3Q7)
#define NUM_DIRECTIONS 7
#elif defined(D3Q15)
#define NUM_DIRECTIONS 15
#elif defined(D3Q19)
#define NUM_DIRECTIONS 19
#elif defined(D3Q27)
#define NUM_DIRECTIONS 27
#else
#error "No valid model defined (D2Q9, D3Q7, D3Q15, D3Q19, D3Q27)"
#endif

// Velocity vectors (D2Q9, D3Q7, D3Q15, D3Q19, D3Q27)
constant int c[NUM_DIRECTIONS][3] = {
#ifdef D2Q9
    {0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0}, {0, -1, 0},
    {1, 1, 0}, {-1, -1, 0}, {1, -1, 0}, {-1, 1, 0}
#elif defined(D3Q7)
    {0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0},
    {0, -1, 0}, {0, 0, 1}, {0, 0, -1}
#elif defined(D3Q15)
    {0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0},
    {0, -1, 0}, {0, 0, 1}, {0, 0, -1}, {1, 1, 1},
    {-1, -1, -1}, {1, 1, -1}, {-1, -1, 1}, {1, -1, 1},
    {-1, 1, -1}, {-1, 1, 1}, {1, -1, -1}
#elif defined(D3Q19)
    {0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0},
    {0, -1, 0}, {0, 0, 1}, {0, 0, -1}, {1, 1, 0},
    {-1, -1, 0}, {1, 0, 1}, {-1, 0, -1}, {0, 1, 1},
    {0, -1, -1}, {1, -1, 0}, {-1, 1, 0}, {1, 0, -1},
    {-1, 0, 1}, {0, 1, -1}, {0, -1, 1}
#elif defined(D3Q27)
    {0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0},
    {0, -1, 0}, {0, 0, 1}, {0, 0, -1}, {1, 1, 0},
    {-1, -1, 0}, {1, 0, 1}, {-1, 0, -1}, {0, 1, 1},
    {0, -1, -1}, {1, -1, 0}, {-1, 1, 0}, {1, 0, -1},
    {-1, 0, 1}, {0, 1, -1}, {0, -1, 1}, {1, 1, 1},
    {-1, -1, -1}, {1, 1, -1}, {-1, -1, 1}, {1, -1, 1},
    {-1, 1, -1}, {-1, 1, 1}, {1, -1, -1}
#endif
};

// Weights for the velocity sets
constant float w[NUM_DIRECTIONS] = {
#ifdef defined(D2Q9)
    4.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0
#elif defined(D3Q7)
    1.0 / 4.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0
#elif defined(D3Q15)
    2.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
    1.0 / 9.0, 1.0 / 9.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0,
    1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0
#elif defined(D3Q19)
    1.0 / 3.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0,
    1.0 / 18.0, 1.0 / 18.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0
#elif defined(D3Q27)
    8.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0,
    2.0 / 27.0, 2.0 / 27.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0,
    1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0,
    1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 216.0,
    1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0,
    1.0 / 216.0, 1.0 / 216.0
#endif
};

// Streaming kernel
__kernel void streaming_kernel(
    __global float* f,          // Input distribution functions
    __global float* f_new,      // Output distribution functions
    int nx,                    // Grid size in x
    int ny,                    // Grid size in y
    int nz                     // Grid size in z
) {
    // Get global IDs (node coordinates in the 3D grid)
    int x = get_global_id(0);
    int y = get_global_id(1);
    int z = get_global_id(2);

    // Check if the thread is within the grid boundaries
    if (x >= nx || y >= ny || z >= nz) return;

    // Calculate the linear index of the current node
    int idx = x + y * nx + z * nx * ny;

    // Loop over the directions
    for (int i = 0; i < NUM_DIRECTIONS; i++) {
        // Calculate the coordinates of the neighboring node
        int x_nbr = x + c[i][0];
        int y_nbr = y + c[i][1];
        int z_nbr = z + c[i][2];

        // Apply periodic boundary conditions
        x_nbr = (x_nbr + nx) % nx;
        y_nbr = (y_nbr + ny) % ny;
        z_nbr = (z_nbr + nz) % nz;

        // Calculate the linear index of the neighboring node
        int idx_nbr = x_nbr + y_nbr * nx + z_nbr * nx * ny;

        // Perform the streaming of the distribution function
        f_new[idx_nbr * NUM_DIRECTIONS + i] = f[idx * NUM_DIRECTIONS + i];
    }
}

// Collision kernel (BGK model)
__kernel void collision_kernel(
    __global float* f,          // Input/output distribution functions
    int nx,                    // Grid size in x
    int ny,                    // Grid size in y
    int nz,                    // Grid size in z
    int num_directions,        // Number of directions (9, 7, 15, 19, 27)
    float omega                // Relaxation parameter (tau = 1/omega)
) {
    // Get global IDs (node coordinates in the 3D grid)
    int x = get_global_id(0);
    int y = get_global_id(1);
    int z = get_global_id(2);

    // Check if the thread is within the grid boundaries
    if (x >= nx || y >= ny || z >= nz) return;

    // Calculate the linear index of the current node
    int idx = x + y * nx + z * nx * ny;

    // Calculate density and velocity (momentum)
    float rho = 0.0;
    float ux = 0.0, uy = 0.0, uz = 0.0;

    for (int i = 0; i < NUM_DIRECTIONS; i++) {
        rho += f[idx * NUM_DIRECTIONS + i];
        ux += c[i][0] * f[idx * NUM_DIRECTIONS + i];
        uy += c[i][1] * f[idx * NUM_DIRECTIONS + i];
        uz += c[i][2] * f[idx * NUM_DIRECTIONS + i];
    }

    // Normalize velocity
    ux /= rho;
    uy /= rho;
    uz /= rho;

    // Calculate the equilibrium distribution function and apply the BGK model
    for (int i = 0; i < NUM_DIRECTIONS; i++) {
        float cu = c[i][0] * ux + c[i][1] * uy + c[i][2] * uz;
        float u2 = ux * ux + uy * uy + uz * uz;
        float feq = w[i] * rho * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2);

        f[idx * num_directions + i] = f[idx * num_directions + i] - omega * (f[idx * num_directions + i] - feq);
    }
}
    "#;