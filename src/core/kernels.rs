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
    __global int* flags       // Flags: FLUID, SOLID, EQ
) {
    // Get the global ID of the current thread
    int n = get_global_id(0); 
    if (n >= NX * NY * NZ) return; // Prevent out-of-bounds access

    // Skip processing for solid cells
    if (flags[n] == FLAG_SOLID) return;

    // Compute the 3D coordinates of the current cell
    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    // Loop over all velocity directions
    #pragma unroll
    for (int q = 0; q < Q; q++) {
        // Get the velocity vector components for the current direction
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        // Compute the coordinates of the neighboring cell, applying periodic boundary conditions
        int xn = (x + dx + NX) % NX;
        int yn = (y + dy + NY) % NY;
        int zn = (z + dz + NZ) % NZ;

        // Compute the linear index of the neighboring cell
        int nn = zn * (NX * NY) + yn * NX + xn;

        // Retrieve the flag of the neighboring cell
        int neighbor_flag = flags[nn];

        if (neighbor_flag == FLAG_SOLID) {
            // Bounce-back condition: reflect to the opposite direction within the current cell
            f_new[n * Q + opposite[q]] = f[n * Q + q];
        }
        else if (neighbor_flag == FLAG_FLUID || neighbor_flag == FLAG_EQ) {
            // Propagate to the neighboring cell (includes safe periodicity)
            f_new[nn * Q + q] = f[n * Q + q];
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------

__kernel void collision_kernel(
    __global float* f,        // Input distribution function array
    __global float* rho,      // Output density array
    __global int* flags,      // Flags array indicating boundary conditions
    __global float* u,        // Output velocity array
    float omega               // Relaxation parameter
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Prevent out-of-bounds access

    // Retrieve the flag for the current cell
    int flag = flags[n];

    // Skip collision for solid cells
    if (flag == FLAG_SOLID) return;

    // Compute local density and velocity
    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;

    // Loop over all velocity directions to calculate density and velocity
    #pragma unroll
    for (int q = 0; q < Q; q++) {
        float fq = f[n * Q + q];
        local_rho += fq;                  // Accumulate density
        ux += c[q][0] * fq;               // Accumulate x-velocity
        uy += c[q][1] * fq;               // Accumulate y-velocity
        uz += c[q][2] * fq;               // Accumulate z-velocity
    }

    // Normalize velocity by density, avoiding division by zero
    float inv_rho = (local_rho > 1e-10f) ? 1.0f / local_rho : 0.0f;
    ux *= inv_rho;
    uy *= inv_rho;
    uz *= inv_rho;

    // Store macroscopic variables (density and velocity)
    rho[n] = local_rho;
    u[n * 3 + 0] = ux;
    u[n * 3 + 1] = uy;
    u[n * 3 + 2] = uz;

    float u2 = ux * ux + uy * uy + uz * uz;                // Squared velocity magnitude

    if (flag == FLAG_EQ) {
        #pragma unroll    
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz; // Dot product of velocity and direction vector
            f[n * Q + q] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2); // Equilibrium distribution
        }  
    } else {
        // Perform standard BGK collision for fluid cells
        #pragma unroll
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz; // Dot product of velocity and direction vector
            float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2); // Equilibrium distribution
            f[n * Q + q] = (1.0f - omega) * f[n * Q + q] + omega * feq; // Relaxation towards equilibrium
        }
    }
}


// ----------------------------------------------------------------------------------------------------------------------

// Kernel to swap data from the new distribution function array to the original array
__kernel void swap(__global float* f, __global float* f_new) {
    // Get the global ID of the current thread
    int n = get_global_id(0);

    // Prevent out-of-bounds access
    if (n >= N * Q) return;

    // Copy the value from the new distribution function array to the original array
    f[n] = f_new[n];
}

// ----------------------------------------------------------------------------------------------------------------------

__kernel void equilibrium(
    __global float* f,        // Distribution function array
    __global float* rho,      // Density array
    __global float* u         // Velocity array
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Prevent out-of-bounds access

    // Retrieve velocity components for the current node
    float ux = u[n * 3];
    float uy = u[n * 3 + 1];
    float uz = u[n * 3 + 2];
    
    // Compute the squared velocity magnitude
    float u2 = ux * ux + uy * uy + uz * uz;

    // Retrieve the local density for the current node
    float local_rho = rho[n];

    // Loop over all velocity directions
    #pragma unroll
    for (int q = 0; q < Q; q++) {
        // Compute the dot product of velocity and direction vector
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;

        // Compute the equilibrium distribution function for the current direction
        f[n * Q + q] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
    }
}
// ----------------------------------------------------------------------------------------------------------------------
"#;