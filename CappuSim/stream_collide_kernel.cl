__kernel void lbm_collide_and_stream(
    __global float* f, __global float* rho, __global float* u,
    __constant float* c, __constant float* w, float omega,
    int Q, int D, int Nx, int Ny, int Nz
) {
    int i = get_global_id(0);
    int j = get_global_id(1);
    int k = get_global_id(2);

    if (i >= Nx || j >= Ny || k >= Nz) return;

    // Compute density and velocity
    float rho_ij = 0.0f;
    float u_ij[3] = {0.0f, 0.0f, 0.0f};

    for (int q = 0; q < Q; q++) {
        rho_ij += f[i * Ny * Nz * Q + j * Nz * Q + k * Q + q];
        for (int d = 0; d < D; d++) {
            u_ij[d] += f[i * Ny * Nz * Q + j * Nz * Q + k * Q + q] * c[q * D + d];
        }
    }
    for (int d = 0; d < D; d++) u_ij[d] /= rho_ij;

    // Precompute u.u
    float uu = 0.0f;
    for (int d = 0; d < D; d++) {
        uu += u_ij[d] * u_ij[d];
    }

    // Collision and streaming
    for (int q = 0; q < Q; q++) {
        // Compute c_q.u
        float cu = c[q * D + 0] * u_ij[0] + c[q * D + 1] * u_ij[1];
        if (D == 3) cu += c[q * D + 2] * u_ij[2];

        // Compute f_eq
        float feq = w[q] * rho_ij * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * uu);

        // Streaming
        int new_i = (i + (int)c[q * D + 0] + Nx) % Nx;
        int new_j = (j + (int)c[q * D + 1] + Ny) % Ny;
        int new_k = (k + (int)c[q * D + 2] + Nz) % Nz;

        f[new_i * Ny * Nz * Q + new_j * Nz * Q + new_k * Q + q] = f[i * Ny * Nz * Q + j * Nz * Q + k * Q + q] - omega * (f[i * Ny * Nz * Q + j * Nz * Q + k * Q + q] - feq);
    }

    // Save macroscopic quantities
    rho[i * Ny * Nz + j * Nz + k] = rho_ij;
    for (int d = 0; d < D; d++) {
        u[i * Ny * Nz * 3 + j * Nz * 3 + k * 3 + d] = u_ij[d];
    }
}
