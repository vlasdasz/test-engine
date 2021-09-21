use proc_macro::New;
use tools::new;

use crate::{IntoF32, Point, Size};

#[derive(Copy, Clone, Debug, New)]
pub struct Rect {
    pub origin: Point,
    pub size:   Size,
}

impl Rect {
    pub const DEFAULT: Rect = Rect {
        origin: Point { x: 0.0, y: 0.0 },
        size:   Size {
            width:  0.0,
            height: 0.0,
        },
    };
}

impl Rect {
    pub fn max_x(&self) -> f32 { self.origin.x + self.size.width }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.y >= self.origin.y
            && point.x <= self.origin.x + self.size.width
            && point.y <= self.origin.y + self.size.height
    }

    pub fn x(&self) -> f32 { self.origin.x }

    pub fn y(&self) -> f32 { self.origin.y }

    pub fn width(&self) -> f32 { self.size.width }

    pub fn height(&self) -> f32 { self.size.height }
}

impl Rect {
    pub fn set_center(&mut self, center: Point) {
        self.origin.x = center.x - self.size.width / 2.0;
        self.origin.y = center.y - self.size.height / 2.0;
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            origin: tools::new(),
            size,
        }
    }
}

impl<X: IntoF32, Y: IntoF32, W: IntoF32, H: IntoF32> From<(X, Y, W, H)> for Rect {
    fn from(tup: (X, Y, W, H)) -> Self {
        Self {
            origin: (tup.0, tup.1).into(),
            size:   (tup.2, tup.3).into(),
        }
    }
}

impl<W: IntoF32, H: IntoF32> From<(W, H)> for Rect {
    fn from(tup: (W, H)) -> Self {
        Self {
            origin: tools::new(),
            size:   (tup.0, tup.1).into(),
        }
    }
}

impl From<()> for Rect {
    fn from(_: ()) -> Self { new() }
}
