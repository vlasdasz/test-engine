use std::default::default;

use crate::{IntoF32, Point, Size};

#[derive(Copy, Clone, Default, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size:   Size,
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

    pub fn square(&self) -> Size {
        let side = if self.size.height < self.size.width {
            self.size.height
        } else {
            self.size.width
        };
        (side, side).into()
    }
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
            origin: default(),
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
            origin: default(),
            size:   (tup.0, tup.1).into(),
        }
    }
}
