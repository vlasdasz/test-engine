use crate::gm::{IntoF32, Point, Size};

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

    pub fn make<T: IntoF32>(x: T, y: T, width: T, height: T) -> Rect {
        Rect {
            origin: Point::make(x, y),
            size: Size::make(width, height),
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

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            origin: Point::new(),
            size,
        }
    }
}
