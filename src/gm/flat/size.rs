
#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

impl Size {

    pub fn new() -> Size {
        Size { width: 0.0, height: 0.0 }
    }

    pub fn is_negative(&self) -> bool {
        self.width < 0.0 || self.height < 0.0
    }
}