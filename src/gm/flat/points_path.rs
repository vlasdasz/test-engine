use crate::gm::Point;
use std::f32::consts::PI;
use tools::New;

#[derive(Debug)]
pub struct PointsPath {
    pub points: Vec<Point>,
}

impl PointsPath {
    pub fn circle_with(center: Point, radius: f32, precision: u32) -> Self {
        let mut path = PointsPath::new();
        let angle_step = PI * 2.0 / precision as f32;
        for i in 0..precision {
            path.add_point(point_on_circle(radius, angle_step * i as f32, &center));
        }
        return path;
    }
}

impl PointsPath {
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn clear(&mut self) {
        self.points.clear()
    }
}

impl New for PointsPath {
    fn new() -> Self {
        Self { points: vec![] }
    }
}

fn point_on_circle(radius: f32, angle: f32, center: &Point) -> Point {
    Point::make(
        (radius / 2.0) * angle.cos() + center.x,
        (radius / 2.0) * angle.sin() + center.y,
    )
}
