__kernel void streaming_kernel(
    __global float* f,        // Input distribution function
    __global float* f_new,    // Output distribution function after streaming
    __global int* flags,       // Flags: FLUID, SOLID, EQ
    int timestep               // Current Time step
) {
    int n = get_global_id(0); 
    if (n >= NX * NY * NZ) return;

    if (flags[n] == FLAG_SOLID) return;

    // Determine which buffer to read from and write to based on timestep
    __global float* read_buf = (timestep % 2 == 0) ? f : f_new;
    __global float* write_buf = (timestep % 2 == 0) ? f_new : f;

    int x = n % NX;
    int y = (n / NX) % NY;
    int z = n / (NX * NY);

    // Pull scheme: Each node pulls from its neighbors instead of pushing to them
    for (int q = 0; q < Q; q++) {
        int dx = c[q][0];
        int dy = c[q][1];
        int dz = c[q][2];

        // Compute neighbor in the opposite direction (pull from)
        int xp = (x - dx + NX) % NX;
        int yp = (y - dy + NY) % NY;
        int zp = (z - dz + NZ) % NZ;

        int np = zp * (NX * NY) + yp * NX + xp;
        int neighbor_flag = flags[np];

        if (neighbor_flag == FLAG_SOLID) {
            // For solid neighbors, bounce back
            write_buf[q * (NX * NY * NZ) + n] = read_buf[opposite[q] * (NX * NY * NZ) + n];
        } else if (neighbor_flag == FLAG_FLUID || neighbor_flag == FLAG_EQ) {
            // Pull the distribution from the neighbor
            write_buf[q * (NX * NY * NZ) + n] = read_buf[q * (NX * NY * NZ) + np];
        }
    }
}