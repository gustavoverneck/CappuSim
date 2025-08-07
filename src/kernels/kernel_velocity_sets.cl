// Define precision type for calculations
#ifdef USE_FP16C
    #pragma OPENCL EXTENSION cl_khr_fp16 : enable
    #define FLOAT_TYPE half
    #define FLOAT_CONST(x) x##h  // Example: FLOAT_CONST(1.0) becomes 1.0h
#else
    #define FLOAT_TYPE float
    #define FLOAT_CONST(x) x##f  // Example: FLOAT_CONST(1.0) becomes 1.0f
#endif

// Velocity vectors (same for all modes)
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

// Weights with the appropriate type automatically
constant FLOAT_TYPE w[Q] = {
#if defined(D2Q9)
    FLOAT_CONST(4.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(9.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(9.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0)
#elif defined(D3Q7)
    FLOAT_CONST(1.0)/FLOAT_CONST(4.0), FLOAT_CONST(1.0)/FLOAT_CONST(8.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(8.0), FLOAT_CONST(1.0)/FLOAT_CONST(8.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(8.0), FLOAT_CONST(1.0)/FLOAT_CONST(8.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(8.0)
#elif defined(D3Q15)
    FLOAT_CONST(2.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(9.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(9.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(9.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(9.0), FLOAT_CONST(1.0)/FLOAT_CONST(72.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(72.0), FLOAT_CONST(1.0)/FLOAT_CONST(72.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(72.0), FLOAT_CONST(1.0)/FLOAT_CONST(72.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(72.0), FLOAT_CONST(1.0)/FLOAT_CONST(72.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(72.0)
#elif defined(D3Q19)
    FLOAT_CONST(1.0)/FLOAT_CONST(3.0), FLOAT_CONST(1.0)/FLOAT_CONST(18.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(18.0), FLOAT_CONST(1.0)/FLOAT_CONST(18.0), 
    FLOAT_CONST(1.0)/FLOAT_CONST(18.0), FLOAT_CONST(1.0)/FLOAT_CONST(18.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(18.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0), FLOAT_CONST(1.0)/FLOAT_CONST(36.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(36.0)
#elif defined(D3Q27)
    FLOAT_CONST(8.0)/FLOAT_CONST(27.0), FLOAT_CONST(2.0)/FLOAT_CONST(27.0), 
    FLOAT_CONST(2.0)/FLOAT_CONST(27.0), FLOAT_CONST(2.0)/FLOAT_CONST(27.0), 
    FLOAT_CONST(2.0)/FLOAT_CONST(27.0), FLOAT_CONST(2.0)/FLOAT_CONST(27.0),
    FLOAT_CONST(2.0)/FLOAT_CONST(27.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(54.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(54.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(54.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(54.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(54.0), FLOAT_CONST(1.0)/FLOAT_CONST(54.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(216.0), FLOAT_CONST(1.0)/FLOAT_CONST(216.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(216.0), FLOAT_CONST(1.0)/FLOAT_CONST(216.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(216.0), FLOAT_CONST(1.0)/FLOAT_CONST(216.0),
    FLOAT_CONST(1.0)/FLOAT_CONST(216.0), FLOAT_CONST(1.0)/FLOAT_CONST(216.0)
#endif
};

// Additional pre-defined constants for efficient calculation
constant FLOAT_TYPE FLOAT_ONE = FLOAT_CONST(1.0);
constant FLOAT_TYPE FLOAT_THREE = FLOAT_CONST(3.0);
constant FLOAT_TYPE FLOAT_FOUR_POINT_FIVE = FLOAT_CONST(4.5);
constant FLOAT_TYPE FLOAT_ONE_POINT_FIVE = FLOAT_CONST(1.5);
constant FLOAT_TYPE FLOAT_EPSILON = FLOAT_CONST(1e-10);