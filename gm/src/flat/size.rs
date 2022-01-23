use std::ops::Mul;

use rtools::IntoF32;
use serde::{Deserialize, Serialize};

use crate::Point;

#[derive(Copy, Clone, Default, Debug, Deserialize, Serialize)]
pub struct Size {
    pub width:  f32,
    pub height: f32,
}

impl Size {
    pub fn diagonal(&self) -> f32 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    pub fn square(side: f32) -> Size {
        (side, side).into()
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
}

impl<W: IntoF32, H: IntoF32> From<(W, H)> for Size {
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
