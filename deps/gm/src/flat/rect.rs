use std::ops::Mul;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::{
    axis::Axis,
    flat::{Point, Size},
    num::into_f32::ToF32,
};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize, Zeroable, Pod)]
pub struct Rect {
    pub origin: Point,
    pub size:   Size,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        (x, y, w, h).into()
    }

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

    pub fn intersects(&self, rect: &Rect) -> bool {
        let x1 = self.x();
        let y1 = self.y();
        let x2 = x1 + self.width();
        let y2 = y1 + self.height();

        let x3 = rect.x();
        let y3 = rect.y();
        let x4 = x3 + rect.width();
        let y4 = y3 + rect.height();

        x1 < x4 && x2 > x3 && y1 < y4 && y2 > y3
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

    pub fn short_display(&self) -> String {
        format!(
            "x: {} y: {} w: {} h: {}",
            self.origin.x, self.origin.y, self.size.width, self.size.height
        )
    }

    pub fn fit_aspect_ratio(&self, size: Size) -> Rect {
        let scale_x = self.size.width / size.width;
        let scale_y = self.size.height / size.height;
        let scale = scale_x.min(scale_y);

        let new_width = size.width * scale;
        let new_height = size.height * scale;

        let x = self.origin.x + (self.size.width - new_width) / 2.0;
        let y = self.origin.y + (self.size.height - new_height) / 2.0;

        Rect {
            origin: Point { x, y },
            size:   Size {
                width:  new_width,
                height: new_height,
            },
        }
    }

    pub fn fill_aspect_ratio(&self, target_size: Size) -> Rect {
        let scale_x = self.size.width / target_size.width;
        let scale_y = self.size.height / target_size.height;
        let scale = scale_x.max(scale_y);

        let new_width = target_size.width * scale;
        let new_height = target_size.height * scale;

        let x = self.origin.x + (self.size.width - new_width) / 2.0;
        let y = self.origin.y + (self.size.height - new_height) / 2.0;

        Rect {
            origin: Point { x, y },
            size:   Size {
                width:  new_width,
                height: new_height,
            },
        }
    }
}

impl Rect {
    pub fn center(&self) -> Point {
        (self.x() + self.width() / 2.0, self.y() + self.height() / 2.0).into()
    }

    pub fn set_center(&mut self, center: impl Into<Point>) {
        let center = center.into();
        self.origin.x = center.x - self.width() / 2.0;
        self.origin.y = center.y - self.height() / 2.0;
    }

    pub fn with_zero_origin(&self) -> Rect {
        (0, 0, self.size.width, self.size.height).into()
    }

    pub fn to_borders(self, width: impl ToF32) -> [Rect; 4] {
        let width = width.to_f32();

        [
            (self.x(), self.y(), self.width(), width).into(),
            (self.x() + self.width() - width, self.y(), width, self.height()).into(),
            (self.x(), self.y() + self.height() - width, self.width(), width).into(),
            (self.x(), self.y(), width, self.height()).into(),
        ]
    }
}

impl Rect {
    pub fn position<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.origin.x,
            Axis::Y => self.origin.y,
        }
    }

    pub fn other_position<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.origin.y,
            Axis::Y => self.origin.x,
        }
    }

    pub fn length<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.size.width,
            Axis::Y => self.size.height,
        }
    }

    pub fn other_length<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.size.height,
            Axis::Y => self.size.width,
        }
    }

    pub fn set_position<const AXIS: Axis>(&mut self, pos: impl ToF32) {
        match AXIS {
            Axis::X => self.origin.x = pos.to_f32(),
            Axis::Y => self.origin.y = pos.to_f32(),
        }
    }

    pub fn set_other_position<const AXIS: Axis>(&mut self, pos: impl ToF32) {
        match AXIS {
            Axis::X => self.origin.y = pos.to_f32(),
            Axis::Y => self.origin.x = pos.to_f32(),
        }
    }

    pub fn set_length<const AXIS: Axis>(&mut self, length: impl ToF32) {
        match AXIS {
            Axis::X => self.size.width = length.to_f32(),
            Axis::Y => self.size.height = length.to_f32(),
        }
    }

    pub fn set_other_length<const AXIS: Axis>(&mut self, length: impl ToF32) {
        match AXIS {
            Axis::X => self.size.height = length.to_f32(),
            Axis::Y => self.size.width = length.to_f32(),
        }
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size,
        }
    }
}

impl<X, Y, W, H> From<(X, Y, W, H)> for Rect
where
    X: ToF32,
    Y: ToF32,
    W: ToF32,
    H: ToF32,
{
    fn from(tup: (X, Y, W, H)) -> Self {
        Self {
            origin: Point {
                x: tup.0.to_f32(),
                y: tup.1.to_f32(),
            },
            size:   Size {
                width:  tup.2.to_f32(),
                height: tup.3.to_f32(),
            },
        }
    }
}

impl<T: ToF32> Mul<T> for &Rect {
    type Output = Rect;
    fn mul(self, rhs: T) -> Rect {
        let mul = rhs.to_f32();
        (
            self.origin.x * mul,
            self.origin.y * mul,
            self.size.width * mul,
            self.size.height * mul,
        )
            .into()
    }
}

impl<T: ToF32> Mul<T> for Rect {
    type Output = Rect;
    fn mul(self, rhs: T) -> Rect {
        (&self).mul(rhs)
    }
}
