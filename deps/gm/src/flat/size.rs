use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::{Div, Mul},
};

use rtools::IntoF32;
use serde::{Deserialize, Serialize};

use crate::{
    axis::Axis,
    flat::{Point, Rect},
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizeBase<T> {
    pub width:  T,
    pub height: T,
}

pub type Size = SizeBase<f32>;
pub type IntSize = SizeBase<u32>;

impl Size {
    pub fn diagonal(&self) -> f32 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    pub fn square(&self) -> f32 {
        self.width * self.height
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_negative(&self) -> bool {
        self.width < 0.0 || self.height < 0.0
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.width / 2.0,
            y: self.height / 2.0,
        }
    }

    pub fn side<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }

    pub fn other_size<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.height,
            Axis::Y => self.width,
        }
    }

    pub fn fit_height(&self, height: impl IntoF32) -> Size {
        let ratio = height.into_f32() / self.height;
        *self * ratio
    }

    pub fn fit_width(&self, width: impl IntoF32) -> Size {
        let ratio = width.into_f32() / self.width;
        *self * ratio
    }

    pub fn fit_in_rect<const AXIS: Axis>(&self, rect: impl Borrow<Rect>) -> Rect {
        let rect = rect.borrow();
        let ratio = rect.length::<AXIS>() / self.side::<AXIS>();
        let size = *self * ratio;
        let pos = rect.other_position::<AXIS>() + rect.other_length::<AXIS>() / 2.0
            - size.other_size::<AXIS>() / 2.0;

        let mut result: Rect = (size.width, size.height).into();

        result.set_position::<AXIS>(rect.position::<AXIS>());
        result.set_other_position::<AXIS>(pos);

        result
    }
}

impl<W: ~const IntoF32, H: ~const IntoF32> From<(W, H)> for Size {
    fn from(tup: (W, H)) -> Self {
        Self {
            width:  tup.0.into_f32(),
            height: tup.1.into_f32(),
        }
    }
}

impl<T: IntoF32> Mul<T> for Size {
    type Output = Size;
    fn mul(self, rhs: T) -> Self::Output {
        (self.width * rhs.into_f32(), self.height * rhs.into_f32()).into()
    }
}

impl<T: IntoF32> Div<T> for Size {
    type Output = Size;
    fn div(self, rhs: T) -> Self::Output {
        (self.width / rhs.into_f32(), self.height / rhs.into_f32()).into()
    }
}

impl ToString for Size {
    fn to_string(&self) -> String {
        format!("width: {:.2}, height: {:.2}", self.width, self.height)
    }
}

impl Hash for Size {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.to_bits().hash(state);
        self.height.to_bits().hash(state);
    }
}

impl From<IntSize> for Size {
    fn from(value: IntSize) -> Self {
        Self {
            width:  value.width as _,
            height: value.height as _,
        }
    }
}

impl From<(u32, u32)> for IntSize {
    fn from(tup: (u32, u32)) -> Self {
        Self {
            width:  tup.0,
            height: tup.1,
        }
    }
}
