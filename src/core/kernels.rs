// src/core/kernels.rs

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

// Weights for the velocity sets
constant float w[Q] = {
#if defined(D2Q9)
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

__kernel void streaming_kernel(
    __global float* f, __global float* f_new) {
    // Get global IDs (node coordinates in the 3D grid)
    int x = get_global_id(0);
    int y = get_global_id(1);
    int z = get_global_id(2);

    // Check if the thread is within the grid boundaries
    if (x >= NX || y >= NY || z >= NZ) return;

    // Calculate the linear index of the current node
    int idx = x + NX * (y + NY * z);

    // Loop over the directions
    for (int i = 0; i < Q; i++) {
        // Calculate the coordinates of the neighboring node
        int x_nbr = x + c[i][0];
        int y_nbr = y + c[i][1];
        int z_nbr = z + c[i][2];

        // Apply periodic boundary conditions
        x_nbr = (x_nbr + NX) % NX;  // Wrap around x within grid bounds
        y_nbr = (y_nbr + NY) % NY;  // Wrap around y within grid bounds
        z_nbr = (z_nbr + NZ) % NZ;  // Wrap around z within grid bounds

        // Calculate the linear index of the neighboring node
        int idx_nbr = x_nbr + y_nbr * NX + z_nbr * NX * NY;

        // Perform the streaming of the distribution function
        f_new[idx_nbr * Q + i] = f[idx * Q + i];
    }

}

// Collision kernel (BGK model)
__kernel void collision_kernel(__global float* f, __global float* rho, __global float* u, float omega) {
    // Get global IDs (node coordinates in the 3D grid)
    int x = get_global_id(0);
    int y = get_global_id(1);
    int z = get_global_id(2);

    // Check if the thread is within the grid boundaries
    if (x >= NX || y >= NY || z >= NZ) return;

    // Calculate the linear index of the current node
    int idx = x + y * NX + z * NX * NY;

    // Calculate density and velocity (momentum)
    float local_rho = 0.0;
    float ux = 0.0, uy = 0.0, uz = 0.0;

    for (int i = 0; i < Q; i++) {
        local_rho += f[idx * Q + i];
        ux += c[i][0] * f[idx * Q + i];
        uy += c[i][1] * f[idx * Q + i];
        uz += c[i][2] * f[idx * Q + i];
    }

    // Normalize velocity
    if (local_rho > 1.0e-15) {
        ux /= local_rho;
        uy /= local_rho;
        uz /= local_rho;
    } else {
        ux = 0.0;
        uy = 0.0;
        uz = 0.0;
    }

    // Calculate the equilibrium distribution function and apply the BGK model
    for (int i = 0; i < Q; i++) {
        float cu = c[i][0] * ux + c[i][1] * uy + c[i][2] * uz;
        float u2 = ux * ux + uy * uy + uz * uz;
        float feq = w[i] * local_rho * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2);

        f[idx * Q + i] = f[idx * Q + i] - omega * (f[idx * Q + i] - feq);
    }

    // Store the computed density and velocity
    rho[idx] = local_rho;
    u[idx * 3 + 0] = ux;
    u[idx * 3 + 1] = uy;
    u[idx * 3 + 2] = uz;
}
"#;