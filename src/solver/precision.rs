#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrecisionMode {
    FP32,     // Full precision
    FP16S,    // FP16 Storage, FP32 Compute
    FP16C,    // FP16 Compute
}

impl PrecisionMode {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "FP32" => Ok(PrecisionMode::FP32),
            "FP16S" => Ok(PrecisionMode::FP16S),
            "FP16C" => Ok(PrecisionMode::FP16C),
            _ => Err(format!("Invalid precision mode: {}. Use FP32, FP16S, or FP16C", s)),
        }
    }

    pub fn memory_factor(&self) -> f32 {
        match self {
            PrecisionMode::FP32 => 1.0,
            PrecisionMode::FP16S => 0.6,  // ~60%
            PrecisionMode::FP16C => 0.5,  // ~50%
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PrecisionMode::FP32 => "Full FP32 precision (maximum accuracy)",
            PrecisionMode::FP16S => "FP16 storage, FP32 compute (balanced)",
            PrecisionMode::FP16C => "FP16 compute (maximum performance)",
        }
    }
}