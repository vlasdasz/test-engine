use crate::gm::{Color, Point, Size};

pub struct Sprite {
    pub position: Point,
    pub size: Size,
    pub rotation: f32,
    pub color: Color,
}

impl From<Size> for Sprite {
    fn from(size: Size) -> Self {
        Self {
            position: Point::new(),
            size,
            rotation: 0.0,
            color: Color::random().clone(),
        }
    }
}
