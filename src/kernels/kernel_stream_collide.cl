__kernel void stream_collide_kernel(
    __global float* f,        // Distribution function (input/output, ping-pong)
    __global float* f_new,    // Output buffer (ping-pong)
    __global float* rho,      // Density array (output)
    __global float* u,        // Velocity array (output)
    __global uchar* flags,      // Flag array: FLUID, SOLID, EQ
    float omega,              // Relaxation parameter
    int timestep              // Current time step
) {
    int n = get_global_id(0);
    if (n >= N) return;

    if (flags[n] == FLAG_SOLID) return;

    // Determine which buffer to read from and write to based on timestep
    __global float* read_buf = (timestep % 2 == 0) ? f : f_new;
    __global float* write_buf = (timestep % 2 == 0) ? f_new : f;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;
    float f_pop[Q];

    // --- Streaming (pull) ---
    for (int q = 0; q < Q; q++) {
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        int xp = (x - dx + NX) % NX;
        int yp = (y - dy + NY) % NY;
        int zp = (z - dz + NZ) % NZ;

        int np = zp * (NX * NY) + yp * NX + xp;
        uchar neighbor_flag = flags[np];

        if (neighbor_flag == FLAG_SOLID) {
            // Bounce-back
            f_pop[q] = read_buf[opposite[q] * N + n];
        } else {
            f_pop[q] = read_buf[q * N + np];
        }

        // Accumulate for macroscopic variables
        local_rho += f_pop[q];
        ux += c[q][0] * f_pop[q];
        uy += c[q][1] * f_pop[q];
        uz += c[q][2] * f_pop[q];
    }

    float inv_rho = (local_rho > 1e-10f) ? 1.0f / local_rho : 0.0f;
    ux *= inv_rho;
    uy *= inv_rho;
    uz *= inv_rho;

    float u2 = ux * ux + uy * uy + uz * uz;

    // --- Collision ---
    if (flags[n] == FLAG_EQ) {
        // Use prescribed velocity and density from host
        ux = u[n * 3 + 0];
        uy = u[n * 3 + 1];
        uz = u[n * 3 + 2];
        local_rho = rho[n];  // or fixed value like 1.0f
        u2 = ux * ux + uy * uy + uz * uz;
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            write_buf[q * N + n] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
        }
    } else {
        // Standard BGK collision for fluid cells
        rho[n] = local_rho;
        u[n * 3 + 0] = ux;
        u[n * 3 + 1] = uy;
        u[n * 3 + 2] = uz;
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
            write_buf[q * N + n] = (1.0f - omega) * f_pop[q] + omega * feq;
        }
    }
}