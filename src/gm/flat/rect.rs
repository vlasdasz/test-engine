use crate::gm::{Point, Size};

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Size
}

impl Rect {
    pub fn new() -> Rect {
        Rect{ origin: Point::new(), size: Size::new() }
    }
    pub const fn from_size(size: &Size) -> Rect {
        Rect { origin: Point::new(), size: *size }
    }
    pub const fn make(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect { origin: Point { x, y }, size: Size { width, height } }
    }
}