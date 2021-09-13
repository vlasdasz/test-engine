use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use proc_macro::New;

pub trait IntoF32 {
    fn into_f32(self) -> f32;
}

impl IntoF32 for i32 {
    fn into_f32(self) -> f32 { self as f32 }
}

impl IntoF32 for f32 {
    fn into_f32(self) -> f32 { self }
}

impl IntoF32 for f64 {
    fn into_f32(self) -> f32 { self as f32 }
}

#[derive(Copy, Clone, Debug, New)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const DEFAULT: Point = Point { x: 0.0, y: 0.0 };

    pub fn angle(&self) -> f32 { self.y.atan2(self.x) }
    pub fn is_zero(&self) -> bool { self.x == 0.0 && self.y == 0.0 }
    pub fn length(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
}

impl Point {
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
    pub fn invert_x(&mut self) { self.x = -self.x }
    pub fn invert_y(&mut self) { self.y = -self.y }
}

impl Point {
    pub fn normalized(self) -> Point { self.with_length(1.0) }
    pub fn normalize(&mut self) { self.set_length(1.0) }
}

impl Point {
    pub fn with_length(self, l: f32) -> Point {
        let ratio = l / self.length();
        return Point {
            x: self.x * ratio,
            y: self.y * ratio,
        };
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

impl Point {
    pub fn to_string(&self) -> String {
        String::new() + "x: " + &self.x.to_string() + " y: " + &self.y.to_string()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point { (self.x + rhs.x, self.y + rhs.y).into() }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point { (self.x - rhs.x, self.y - rhs.y).into() }
}

impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Point { (self.x * rhs, self.y * rhs).into() }
}

impl<X: IntoF32, Y: IntoF32> From<(X, Y)> for Point {
    fn from(tup: (X, Y)) -> Self {
        Self {
            x: tup.0.into_f32(),
            y: tup.1.into_f32(),
        }
    }
}
