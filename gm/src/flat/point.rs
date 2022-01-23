use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use rtools::IntoF32;
use serde::{Deserialize, Serialize};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Default, Debug, Deserialize, Serialize)]
pub struct PointBase<T> {
    pub x: T,
    pub y: T,
}

pub type Point = PointBase<f32>;

impl Point {
    pub const DEFAULT: Point = Point { x: 0.0, y: 0.0 };

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Point {
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn invert_x(&mut self) {
        self.x = -self.x
    }

    pub fn invert_y(&mut self) {
        self.y = -self.y
    }
}

impl Point {
    pub fn normalized(self) -> Point {
        self.with_length(1.0)
    }

    pub fn normalize(&mut self) {
        self.set_length(1.0)
    }
}

impl Point {
    pub fn with_length(self, l: f32) -> Point {
        let ratio = l / self.length();
        Point {
            x: self.x * ratio,
            y: self.y * ratio,
        }
    }

    pub fn set_length(&mut self, l: f32) {
        let ratio = l / self.length();
        self.x *= ratio;
        self.y *= ratio;
    }

    pub fn trim(&mut self, max_length: f32) {
        if self.length() < max_length {
            return;
        }
        self.set_length(max_length)
    }

    pub fn trimmed(mut self, max_length: f32) -> Point {
        self.trim(max_length);
        self
    }
}

impl PointBase<i32> {
    pub fn is_negative(&self) -> bool {
        self.x < 0 || self.y < 0
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl<T: Add<Output = T> + Copy> Add for &PointBase<T> {
    type Output = PointBase<T>;
    fn add(self, rhs: &Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &PointBase<T> {
    type Output = PointBase<T>;
    fn sub(self, rhs: &Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: IntoF32> Mul<T> for Point {
    type Output = Point;
    fn mul(self, rhs: T) -> Point {
        (self.x * rhs.into_f32(), self.y * rhs.into_f32()).into()
    }
}

impl<X: IntoF32, Y: IntoF32> From<(X, Y)> for Point {
    fn from(tup: (X, Y)) -> Self {
        Self {
            x: tup.0.into_f32(),
            y: tup.1.into_f32(),
        }
    }
}
