#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Velocity { x, y, z }
    }

    pub fn zero() -> Self {
        Velocity { x: 0.0, y: 0.0, z: 0.0 }
    }
}
