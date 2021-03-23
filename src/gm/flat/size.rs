
#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

impl Size {
    pub fn new() -> Size {
        Size { width: 0.0, height: 0.0 }
    }
}