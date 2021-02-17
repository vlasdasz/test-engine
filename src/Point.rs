pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {

    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn kok(&self) -> f32 {
        self.x * self.y
    }
}