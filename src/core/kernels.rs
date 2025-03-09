// src/core/kernels.rs

pub const STREAMING: &str = r#"
                __kernel void add(__global float* buffer, float scalar) {
                    uint idx = get_global_id(0);
                    buffer[idx] += scalar;
                }
                "#;

pub const COLLISION: &str = r#"kernel_collision"#;

pub const BOUNDARY_CONDITIONS: &str = r#"kernel_boundary_conditions"#;
