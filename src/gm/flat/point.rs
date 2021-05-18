use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

impl Point {
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
    pub fn normalized(&self) -> Point {
        self.with_length(1.0)
    }
    pub fn normalize(&mut self) {
        self.set_length(1.0)
    }
}

impl Point {
    pub fn with_length(&self, l: f32) -> Point {
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

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}
