#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub w: f64, // Scalar part
    pub x: f64, // Vector part (i)
    pub y: f64, // Vector part (j)
    pub z: f64, // Vector part (k)
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }
}