#ifdef USE_FP32
// ============================================================
// FP32 - FULL PRECISION MODE
// ============================================================
__kernel void equilibrium(
    __global float* f,        // Distribution function array
    __global float* rho,      // Density array
    __global float* u         // Velocity array
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Prevent out-of-bounds access

    // Retrieve velocity components for the current node
    int offset = n * 3;
    float ux = u[offset];
    float uy = u[offset + 1];
    float uz = u[offset + 2];
    
    // Compute the squared velocity magnitude
    float u2 = ux * ux + uy * uy + uz * uz;

    // Retrieve the local density for the current node
    float local_rho = rho[n];

    // Loop over all velocity directions
    for (int q = 0; q < Q; q++) {
        // Compute the dot product of velocity and direction vector
        float cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;

        // Compute the equilibrium distribution function for the current direction
        f[q * N + n] = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + 
                       FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
    }
}

#elif defined(USE_FP16S)
// ============================================================
// FP16S - STORAGE MODE (FP16 storage, FP32 computation)
// ============================================================
__kernel void equilibrium(
    __global half* f,         // Distribution function array (FP16)
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

        // Compute the equilibrium distribution function in FP32
        float feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + 
                    FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
            
        // Store result in FP16 format
        vstore_half(feq, q * N + n, f);
    }
}

#elif defined(USE_FP16C)
// ============================================================
// FP16C - COMPUTE MODE (Full FP16 pipeline)
// ============================================================
__kernel void equilibrium(
    __global half* f,         // Distribution function array (FP16)
    __global float* rho,      // Density array
    __global float* u         // Velocity array
) {
    // Get the global ID of the current thread
    int n = get_global_id(0);
    if (n >= N) return; // Prevent out-of-bounds access

    // Retrieve velocity components and convert to half
    half ux = (half)u[n * 3];
    half uy = (half)u[n * 3 + 1];
    half uz = (half)u[n * 3 + 2];
    
    // Compute the squared velocity magnitude in half precision
    half u2 = ux * ux + uy * uy + uz * uz;

    // Retrieve the local density and convert to half
    half local_rho = (half)rho[n];

    // Loop over all velocity directions
    for (int q = 0; q < Q; q++) {
        half cu = c[q][0] * ux + c[q][1] * uy + c[q][2] * uz;

        // Compute the equilibrium distribution function in half precision
        half feq = local_rho * w[q] * (FLOAT_ONE + FLOAT_THREE * cu + 
                  FLOAT_FOUR_POINT_FIVE * cu * cu - FLOAT_ONE_POINT_FIVE * u2);
        
        f[q * N + n] = feq;
    }
}
#endif