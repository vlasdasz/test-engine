
#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new() -> Vector3 { Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
}