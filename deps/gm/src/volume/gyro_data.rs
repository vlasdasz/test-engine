#[derive(Debug, Copy, Clone)]
pub struct GyroData {
    pub pitch: f32,
    pub roll:  f32,
    pub yaw:   f32,
}

impl ToString for GyroData {
    fn to_string(&self) -> String {
        format!("Pitch: {}, Roll: {}, Yaw: {}", self.pitch, self.roll, self.yaw)
    }
}
