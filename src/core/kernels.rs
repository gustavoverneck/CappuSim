/// This module contains OpenCL kernels for implementing the Lattice Boltzmann Method (LBM).
/// The kernels include functionality for streaming, collision, equilibrium computation, 
/// and copying data between distribution function arrays.
///
/// # Constants
/// - `c`: Velocity vectors for different lattice configurations (D2Q9, D3Q7, D3Q15, D3Q19, D3Q27).
/// - `opposite`: Opposite direction indices for bounce-back boundary conditions.
/// - `w`: Weight factors for equilibrium distribution computation.
///
/// # Kernels
///
/// ## `streaming_kernel`
/// Performs the streaming step of the LBM. It streams the distribution function values
/// to neighboring nodes based on velocity vectors. Handles boundary conditions using
/// bounce-back for solid nodes.
///
/// ### Parameters:
/// - `f`: Input distribution function array.
/// - `f_new`: Output distribution function array after streaming.
/// - `flags`: Flags array indicating boundary conditions.
///
/// ## `collision_kernel`
/// Performs the collision step of the LBM. It computes the local density and velocity,
/// applies boundary conditions, and relaxes the distribution function towards equilibrium.
///
/// ### Parameters:
/// - `f`: Input distribution function array.
/// - `rho`: Output density array.
/// - `flags`: Flags array indicating boundary conditions.
/// - `u`: Output velocity array.
/// - `omega`: Relaxation parameter.
///
/// ## `copy`
/// Copies data from the new distribution function array to the original array.
///
/// ### Parameters:
/// - `f`: Original distribution function array.
/// - `f_new`: New distribution function array to copy from.
///
/// ## `equilibrium`
/// Computes the equilibrium distribution function for each node based on local density
/// and velocity.
///
/// ### Parameters:
/// - `f`: Distribution function array.
/// - `rho`: Density array.
/// - `flags`: Flags array indicating boundary conditions.
/// - `u`: Velocity array.
pub const LBM_KERNEL: &str = r#"
// Velocity vectors (D2Q9, D3Q7, D3Q15, D3Q19, D3Q27)
constant int c[Q][3] = {
#if defined(D2Q9)
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

constant int opposite[Q] = {
#if defined(D2Q9)
    0, 2, 1, 4, 3, 6, 5, 8, 7
#elif defined(D3Q7)
    0, 2, 1, 4, 3, 6, 5
#elif defined(D3Q15)
    0, 2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13
#elif defined(D3Q19)
    0, 2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17
#elif defined(D3Q27)
    0, 2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 20, 19, 22, 21, 24, 23, 26, 25
#endif
};

constant float w[Q] = {
#if defined(D2Q9)
    4.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0,
    1.0/36.0, 1.0/36.0, 1.0/36.0, 1.0/36.0
#elif defined(D3Q7)
    1.0/4.0, 1.0/8.0, 1.0/8.0, 1.0/8.0, 1.0/8.0, 1.0/8.0, 1.0/8.0
#elif defined(D3Q15)
    2.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0, 1.0/9.0,
    1.0/9.0, 1.0/9.0, 1.0/72.0, 1.0/72.0, 1.0/72.0,
    1.0/72.0, 1.0/72.0, 1.0/72.0, 1.0/72.0, 1.0/72.0
#elif defined(D3Q19)
    1.0/3.0, 1.0/18.0, 1.0/18.0, 1.0/18.0, 1.0/18.0,
    1.0/18.0, 1.0/18.0, 1.0/36.0, 1.0/36.0, 1.0/36.0,
    1.0/36.0, 1.0/36.0, 1.0/36.0, 1.0/36.0, 1.0/36.0,
    1.0/36.0, 1.0/36.0, 1.0/36.0, 1.0/36.0
#elif defined(D3Q27)
    8.0/27.0, 2.0/27.0, 2.0/27.0, 2.0/27.0, 2.0/27.0,
    2.0/27.0, 2.0/27.0, 1.0/54.0, 1.0/54.0, 1.0/54.0,
    1.0/54.0, 1.0/54.0, 1.0/54.0, 1.0/54.0, 1.0/54.0,
    1.0/54.0, 1.0/54.0, 1.0/54.0, 1.0/54.0, 1.0/216.0,
    1.0/216.0, 1.0/216.0, 1.0/216.0, 1.0/216.0, 1.0/216.0,
    1.0/216.0, 1.0/216.0
#endif
};

// ----------------------------------------------------------------------------------------------------------------------

__kernel void streaming_kernel(
    __global float* f,        // Input distribution function
    __global float* f_new,    // Output distribution function after streaming
    __global int* flags       // Flags array to indicate boundary conditions
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Exit if the thread is out of bounds

    // Compute the 3D coordinates (x, y, z) of the current node index n
    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    // Loop over all velocity directions
    for (int q = 0; q < Q; q++) {
        // Get the velocity vector components for the current direction
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        // Compute the coordinates of the neighboring node
        int xn = x + dx;
        int yn = y + dy;
        int zn = z + dz;

        // Check if the neighboring node is outside the domain boundaries
        bool is_boundary = (xn < 0) | (xn >= NX) | (yn < 0) | (yn >= NY) | (zn < 0) | (zn >= NZ);
        int neighbor = zn * (NX * NY) + yn * NX + xn; // Linear index of the neighbor

        if (is_boundary || flags[neighbor] == FLAG_SOLID) {
            // If the neighbor is a boundary or solid node, apply bounce-back
            f_new[n * Q + opposite[q]] = f[n * Q + q];
        } else {
            // Otherwise, stream the distribution function to the neighbor
            f_new[neighbor * Q + q] = f[n * Q + q];
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------

__kernel void collision_kernel(
    __global float* f,        // Input distribution function
    __global float* rho,      // Density array
    __global int* flags,      // Flags array to indicate boundary conditions
    __global float* u,        // Velocity array
    float omega               // Relaxation parameter
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Exit if the thread is out of bounds

    // Initialize local variables for density and velocity components
    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;

    // Compute local density and velocity components
    for (int q = 0; q < Q; q++) {
        float f_q = f[n * Q + q]; // Distribution function for direction q
        local_rho += f_q;         // Accumulate density
        ux += c[q][0] * f_q;      // Accumulate x-velocity
        uy += c[q][1] * f_q;      // Accumulate y-velocity
        uz += c[q][2] * f_q;      // Accumulate z-velocity
    }

    // Normalize velocity components if density is non-zero
    if (local_rho > 1e-6f) {
        ux /= local_rho;
        uy /= local_rho;
        uz /= local_rho;
    } else {
        // Set velocity to zero if density is too small
        ux = 0.0f;
        uy = 0.0f;
        uz = 0.0f;
    }

    // Store computed density and velocity in the output arrays
    rho[n] = local_rho;
    u[n * 3] = ux;
    u[n * 3 + 1] = uy;
    u[n * 3 + 2] = uz;

    // Retrieve the flag for the current node
    int flag = flags[n];

    // Handle boundary conditions based on the flag
    if (flag != FLAG_FLUID) {
        if (flag == FLAG_SOLID) {
            // Bounce-back condition for solid nodes
            for (int q = 0; q < Q; q++) {
                f[n * Q + q] = f[n * Q + opposite[q]];
            }
        } else if (flag == FLAG_EQ) {
            // Equilibrium condition for specific nodes
            for (int q = 0; q < Q; q++) {
                float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz; // Dot product of velocity and direction
                float u2 = ux * ux + uy * uy + uz * uz;                // Squared velocity magnitude
                f[n * Q + q] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
            }
        }
        return; // Exit after handling boundary conditions
    }

    // Perform collision step for fluid nodes
    for (int q = 0; q < Q; q++) {
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz; // Dot product of velocity and direction
        float u2 = ux * ux + uy * uy + uz * uz;                // Squared velocity magnitude
        float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2); // Equilibrium distribution
        f[n * Q + q] = f[n * Q + q] * (1.0f - omega) + feq * omega; // Relaxation towards equilibrium
    }
}

// ----------------------------------------------------------------------------------------------------------------------

// Kernel to copy data from the new distribution function array to the original array
__kernel void copy(__global float* f, __global float* f_new) {
    // Get the global ID of the current thread
    int n = get_global_id(0);

    // Check if the thread is within the grid boundaries
    if (n >= N) return;

    // Copy the value from the new distribution function array to the original array
    f[n] = f_new[n];
}

// ----------------------------------------------------------------------------------------------------------------------

__kernel void equilibrium(
    __global float* f,        // Distribution function array
    __global float* rho,      // Density array
    __global int* flags,      // Flags array to indicate boundary conditions
    __global float* u         // Velocity array
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Exit if the thread is out of bounds

    // Retrieve velocity components for the current node
    float ux = u[n * 3];
    float uy = u[n * 3 + 1];
    float uz = u[n * 3 + 2];
    
    // Retrieve the local density for the current node
    float local_rho = rho[n];

    // Loop over all velocity directions
    for (int q = 0; q < Q; q++) {
        // Compute the dot product of velocity and direction vector
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;

        // Compute the squared velocity magnitude
        float u2 = ux * ux + uy * uy + uz * uz;

        // Compute the equilibrium distribution function for the current direction
        f[n * Q + q] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
    }
}
// ----------------------------------------------------------------------------------------------------------------------
"#;