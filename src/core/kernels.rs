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

__kernel void streaming_kernel(
    __global float* f,
    __global float* f_new,
    __global int* flags
) {
    int n = get_global_id(0);
    if (n >= N) return;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    for (int q = 0; q < Q; q++) {
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        int xn = x + dx;
        int yn = y + dy;
        int zn = z + dz;

        // Handle boundary conditions
        if (xn < 0 || xn >= NX || yn < 0 || yn >= NY || zn < 0 || zn >= NZ) {
            // Bounce-back boundary
            f_new[n * Q + opposite[q]] = f[n * Q + q];
        } else {
            // Normal streaming
            int neighbor = xn + yn*NX + zn*NX*NY;
            if (flags[neighbor] == FLAG_SOLID) {
                // Bounce-back from solid neighbor
                f_new[n * Q + opposite[q]] = f[n * Q + q];
            } else {
                f_new[neighbor * Q + q] = f[n * Q + q];
            }
        }
    }
}

__kernel void collision_kernel(__global float* f, __global float* rho, __global int* flags, __global float* u, float omega) { 
    // Get global IDs (node coordinates in the 1D grid)
    int n = get_global_id(0);

    // Check if the thread is within the grid boundaries
    if (n >= N) return;

    // Check if the current node is a boundary node (you need to define isBoundaryNode based on your flags array)
    int flag = flags[n]; // Get the boundary type (1 for bounce-back, 2 for equilibrium, etc.)
    bool isBoundaryNode = flag != 0; // If flag is 0, it's a fluid node (no boundary)

    // Calculate density and velocity (momentum)
    float local_rho = 0.0f;
    float ux = 0.0f;
    float uy = 0.0f;
    float uz = 0.0f;

    for (int q = 0; q < Q; q++) {
        local_rho += f[n * Q + q];
        ux += c[q][0] * f[n * Q + q];
        uy += c[q][1] * f[n * Q + q];
        uz += c[q][2] * f[n * Q + q];
    }

    // Normalize velocity
    if (local_rho >= 1.0e-10) {
        ux /= local_rho;
        uy /= local_rho;
        uz /= local_rho;
    } else {
        ux = 0.0;
        uy = 0.0;
        uz = 0.0;
    }

    // Calculate the equilibrium distribution function
    for (int q = 0; q < Q; q++) {
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
        float u2 = ux * ux + uy * uy + uz * uz;
        float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);

        if (!isBoundaryNode) {
            // For fluid nodes, apply the BGK model
            f[n * Q + q] = f[n * Q + q] * (1.0f - omega) + feq * omega;
        } else {
            // Apply boundary condition based on the flag value
            if (flag == 1) {
                // Bounce-back boundary: reverse the distribution
                int opposite_q = (q % 2 == 0) ? (q - 1) : (q + 1); // opposite direction rule
                f[n * Q + q] = f[n * Q + opposite_q];
            } else if (flag == 2) {
                // Equilibrium boundary: set f = f_eq
                f[n * Q + q] = feq;
            }
        }
    }
    
    // Store the computed density and velocity
    rho[n] = local_rho;
    u[n * 3] = ux;
    u[n * 3 + 1] = uy;
    u[n * 3 + 2] = uz;
}

__kernel void copy(__global float* f, __global float* f_new) {
    int n = get_global_id(0);
    f[n] = f_new[n];
}

"#;