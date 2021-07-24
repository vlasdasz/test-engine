use crate::gm::{IntoF32, Point, Size};
use tools::{new, New};

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const DEFAULT: Rect = Rect {
        origin: Point { x: 0.0, y: 0.0 },
        size: Size {
            width: 0.0,
            height: 0.0,
        },
    };

    pub fn make<X: IntoF32, Y: IntoF32, W: IntoF32, H: IntoF32>(
        x: X,
        y: Y,
        width: W,
        height: H,
    ) -> Rect {
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

    pub fn x(&self) -> f32 {
        self.origin.x
    }

    pub fn y(&self) -> f32 {
        self.origin.y
    }

    pub fn width(&self) -> f32 {
        self.size.width
    }

    pub fn height(&self) -> f32 {
        self.size.height
    }
}

impl New for Rect {
    fn new() -> Rect {
        Rect {
            origin: new(),
            size: new(),
        }
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            origin: new(),
            size,
        }
    }
}
