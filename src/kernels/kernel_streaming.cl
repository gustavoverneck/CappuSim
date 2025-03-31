__kernel void streaming_kernel(
    __global float* f,        // Input distribution function
    __global float* f_new,    // Output distribution function after streaming
    __global int* flags       // Flags: FLUID, SOLID, EQ
) {
    int n = get_global_id(0); 
    if (n >= NX * NY * NZ) return;

    if (flags[n] == FLAG_SOLID) return;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    for (int q = 0; q < Q; q++) {
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        int xn = (x + dx + NX) % NX;
        int yn = (y + dy + NY) % NY;
        int zn = (z + dz + NZ) % NZ;

        int nn = zn * (NX * NY) + yn * NX + xn;

        int neighbor_flag = flags[nn];

        if (neighbor_flag == FLAG_SOLID) {
            f_new[n * Q + opposite[q]] = f[n * Q + q];
        }
        else if (neighbor_flag == FLAG_FLUID || neighbor_flag == FLAG_EQ) {
            f_new[nn * Q + q] = f[n * Q + q];
        }
    }
}