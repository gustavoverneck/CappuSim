__kernel void collision_kernel(
    __global float* f,        // Distribution function (input/output)
    __global float* f_new,    // Output distribution function after streaming
    __global float* rho,      // Prescribed density (used for EQ), also output
    __global int* flags,      // Flag array: FLUID, SOLID, EQ
    __global float* u,        // Prescribed velocity (used for EQ), also output
    float omega,               // Relaxation parameter
    int timestep               // Current Time step
) {
    int n = get_global_id(0);
    if (n >= N) return;

    int flag = flags[n];

    if (flag == FLAG_SOLID) return;

    // Determine which buffer to read from and write to based on timestep
    __global float* write_buf = (timestep % 2 == 0) ? f : f_new;

    float local_rho = 0.0f;
    float ux = 0.0f, uy = 0.0f, uz = 0.0f;

    // Accumulate density and momentum from distributions
    for (int q = 0; q < Q; q++) {
        float fq = write_buf[q * N + n];
        local_rho += fq;
        ux += c[q][0] * fq;
        uy += c[q][1] * fq;
        uz += c[q][2] * fq;
    }

    float inv_rho = (local_rho > 1e-10f) ? 1.0f / local_rho : 0.0f;
    ux *= inv_rho;
    uy *= inv_rho;
    uz *= inv_rho;

    float u2;

    if (flag == FLAG_EQ) {
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
        // Standard collision (BGK) for fluid cells
        rho[n] = local_rho;
        u[n * 3 + 0] = ux;
        u[n * 3 + 1] = uy;
        u[n * 3 + 2] = uz;

        u2 = ux * ux + uy * uy + uz * uz;

        for (int q = 0; q < Q; q++) {
            float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;
            float feq = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
            write_buf[q * N + n] = (1.0f - omega) * write_buf[q * N + n] + omega * feq;
        }
    }
}