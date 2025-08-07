// ============================================================
// FP32 - FULL PRECISION MODE
// ============================================================
#ifdef USE_FP32
__kernel void stream_collide_kernel(
    __global float* f,        // Distribution function (input/output, ping-pong)
    __global float* f_new,    // Output buffer (ping-pong)
    __global float* rho,      // Density array (output)
    __global float* u,        // Velocity array (output)
    __global uchar* flags,    // Flag array: FLUID, SOLID, EQ
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

    float inv_rho = (local_rho > FLOAT_EPSILON) ? FLOAT_ONE / local_rho : 0.0f;
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
        local_rho = rho[n];
        u2 = ux * ux + uy * uy + uz * uz;
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            write_buf[q * N + n] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
        }
    } else {
        // Standard BGK collision for fluid cells
        rho[n] = local_rho;
        
        int offset = n * 3;
        u[offset + 0] = ux;
        u[offset + 1] = uy;
        u[offset + 2] = uz;
        
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            float feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + 
                FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
            write_buf[q * N + n] = (1.0f - omega) * f_pop[q] + omega * feq;
        }
    }
}

// ============================================================
// FP16S - STORAGE MODE (FP16 storage, FP32 computation)
// ============================================================
#elif defined(USE_FP16S)
__kernel void stream_collide_kernel(
    __global half* f,         // FP16 distribution function (input/output, ping-pong)
    __global half* f_new,     // FP16 output buffer (ping-pong)
    __global float* rho,      // Density array (output)
    __global float* u,        // Velocity array (output)
    __global uchar* flags,    // Flag array: FLUID, SOLID, EQ
    float omega,              // Relaxation parameter
    int timestep              // Current time step
) {
    int n = get_global_id(0);
    if (n >= N) return;
    if (flags[n] == FLAG_SOLID) return;

    // Determine which buffer to read from and write to based on timestep
    __global half* read_buf_fp16 = (timestep % 2 == 0) ? f : f_new;
    __global half* write_buf_fp16 = (timestep % 2 == 0) ? f_new : f;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    float f_pop[Q];
    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;

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
            f_pop[q] = vload_half(opposite[q] * N + n, read_buf_fp16);
        } else {
            f_pop[q] = vload_half(q * N + np, read_buf_fp16);
        }

        // Accumulate for macroscopic variables
        local_rho += f_pop[q];
        ux += c[q][0] * f_pop[q];
        uy += c[q][1] * f_pop[q];
        uz += c[q][2] * f_pop[q];
    }

    float inv_rho = (local_rho > FLOAT_EPSILON) ? FLOAT_ONE / local_rho : 0.0f;
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
        local_rho = rho[n];
        u2 = ux * ux + uy * uy + uz * uz;
        
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
            vstore_half(feq, q * N + n, write_buf_fp16);
        }
    } else {
        // Standard BGK collision for fluid cells
        rho[n] = local_rho;
        
        // Compute offset apenas uma vez
        int offset = n * 3;
        u[offset + 0] = ux;
        u[offset + 1] = uy;
        u[offset + 2] = uz;
        
        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            float feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + 
                FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
            float f_new_val = (1.0f - omega) * f_pop[q] + omega * feq;
            vstore_half(f_new_val, q * N + n, write_buf_fp16);
        }
    }
}

// ============================================================
// FP16C - COMPUTE MODE (Full FP16 pipeline)
// ============================================================
#elif defined(USE_FP16C)
__kernel void stream_collide_kernel(
    __global half* f,         // FP16 distribution function (input/output, ping-pong)
    __global half* f_new,     // FP16 output buffer (ping-pong)
    __global float* rho,      // Density array (output) - keep in FP32
    __global float* u,        // Velocity array (output) - keep in FP32
    __global uchar* flags,    // Flag array: FLUID, SOLID, EQ
    float omega,              // Relaxation parameter
    int timestep              // Current time step
) {
    int n = get_global_id(0);
    if (n >= N) return;
    if (flags[n] == FLAG_SOLID) return;

    // Determine which buffer to read from and write to based on timestep
    __global half* read_buf = (timestep % 2 == 0) ? f : f_new;
    __global half* write_buf = (timestep % 2 == 0) ? f_new : f;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);
    
    half omega_h = (half)omega;

    // Accumulate macroscopic variables in float for higher accuracy
    half f_pop[Q];
    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;

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
            f_pop[q] = read_buf[opposite[q] * N + n];
        } else {
            f_pop[q] = read_buf[q * N + np];
        }

        // Accumulate for macroscopic variables (in float)
        float f_pop_f = (float)f_pop[q];
        local_rho += f_pop_f;
        ux += (float)c[q][0] * f_pop_f;
        uy += (float)c[q][1] * f_pop_f;
        uz += (float)c[q][2] * f_pop_f;
    }

    float inv_rho = (local_rho > FLOAT_EPSILON) ? FLOAT_ONE / local_rho : 0.0f;
    ux *= inv_rho;
    uy *= inv_rho;
    uz *= inv_rho;

    float u2 = ux * ux + uy * uy + uz * uz;

    // --- Collision ---
    if (flags[n] == FLAG_EQ) {
        // Use prescribed velocity and density from host (convert to float for computation)
        ux = u[n * 3 + 0];
        uy = u[n * 3 + 1];
        uz = u[n * 3 + 2];
        local_rho = rho[n];
        u2 = ux * ux + uy * uy + uz * uz;
        for (int q = 0; q < Q; q++) {
            float cu = (float)c[q][0] * ux + (float)c[q][1] * uy + (float)c[q][2] * uz;
            float feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
            write_buf[q * N + n] = (half)feq;
        }
    } else {
        // Standard BGK collision for fluid cells
        rho[n] = local_rho;  // Output as float
        u[n * 3 + 0] = ux;
        u[n * 3 + 1] = uy;
        u[n * 3 + 2] = uz;
        for (int q = 0; q < Q; q++) {
            float cu = (float)c[q][0] * ux + (float)c[q][1] * uy + (float)c[q][2] * uz;
            float feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
            float f_new_val = (1.0f - omega) * (float)f_pop[q] + omega * feq;
            write_buf[q * N + n] = (half)f_new_val;
        }
    }
}

#endif