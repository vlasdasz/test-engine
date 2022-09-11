use std::ops::Mul;

use rtools::IntoF32;
use serde::{Deserialize, Serialize};

use crate::flat::{Point, Size};

#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size:   Size,
}

impl Rect {
    pub fn max_x(&self) -> f32 {
        self.x() + self.width()
    }

    pub fn max_y(&self) -> f32 {
        self.y() + self.height()
    }

    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        point.x >= self.x()
            && point.y >= self.y()
            && point.x <= self.x() + self.width()
            && point.y <= self.y() + self.height()
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

    pub fn square(&self) -> Size {
        let side = if self.height() < self.width() {
            self.height()
        } else {
            self.width()
        };
        (side, side).into()
    }
}

impl Rect {
    pub fn center(&self) -> Point {
        (self.x() + self.width() / 2.0, self.y() + self.height() / 2.0).into()
    }

    pub fn set_center(&mut self, center: Point) {
        self.origin.x = center.x - self.width() / 2.0;
        self.origin.y = center.y - self.height() / 2.0;
    }

    pub fn with_zero_origin(&self) -> Rect {
        (0, 0, self.size.width, self.size.height).into()
    }
}

impl const From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size,
        }
    }
}

impl<X, Y, W, H> const From<(X, Y, W, H)> for Rect
where
    X: ~const IntoF32,
    Y: ~const IntoF32,
    W: ~const IntoF32,
    H: ~const IntoF32,
{
    fn from(tup: (X, Y, W, H)) -> Self {
        Self {
            origin: Point {
                x: tup.0.into_f32(),
                y: tup.1.into_f32(),
            },
            size:   Size {
                width:  tup.2.into_f32(),
                height: tup.3.into_f32(),
            },
        }
    }
}

impl<W: ~const IntoF32, H: ~const IntoF32> const From<(W, H)> for Rect {
    fn from(tup: (W, H)) -> Self {
        Self {
            origin: Point { x: 0.0, y: 0.0 },
            size:   (tup.0, tup.1).into(),
        }
    }
}

impl<X: ~const IntoF32, Y: ~const IntoF32> const From<(X, Y, Size)> for Rect {
    fn from(tup: (X, Y, Size)) -> Self {
        Self {
            origin: (tup.0, tup.1).into(),
            size:   tup.2,
        }
    }
}

impl<T: IntoF32> Mul<T> for &Rect {
    type Output = Rect;
    fn mul(self, rhs: T) -> Rect {
        let mul = rhs.into_f32();
        (
            self.origin.x * mul,
            self.origin.y * mul,
            self.size.width * mul,
            self.size.height * mul,
        )
            .into()
    }
}
