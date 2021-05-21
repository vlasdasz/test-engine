use crate::gm::{Point, Size};

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const DEFAULT: Rect = Rect::new();

    pub const fn new() -> Rect {
        Rect {
            origin: Point::new(),
            size: Size::new(),
        }
    }

    pub const fn from_size(size: &Size) -> Rect {
        Rect {
            origin: Point::new(),
            size: *size,
        }
    }

    pub const fn make(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }
}

impl Rect {
    pub fn max_x(&self) -> f32 {
        self.origin.x + self.size.width
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.y >= self.origin.y
            && point.x <= self.origin.x + self.size.width
            && point.y <= self.origin.y + self.size.height
    }
}
