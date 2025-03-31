// Kernel to swap data from the new distribution function array to the original array
__kernel void swap(__global float* f, __global float* f_new) {
    // Get the global ID of the current thread
    int n = get_global_id(0);

    // Prevent out-of-bounds access
    if (n >= N * Q) return;

    // Copy the value from the new distribution function array to the original array
    f[n] = f_new[n];
}
