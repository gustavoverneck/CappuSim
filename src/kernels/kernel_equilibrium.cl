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
    for (int q = 0; q < Q; q++) {
        // Compute the dot product of velocity and direction vector
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;

        // Compute the equilibrium distribution function for the current direction
        f[q * N + n] = local_rho * w[q] * (1.0f + 3.0f * cu + 4.5f * cu * cu - 1.5f * u2);
    }
}