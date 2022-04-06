use std::f32::consts::PI;

use crate::flat::Point;

#[derive(Default, Debug)]
pub struct PointsPath {
    pub points: Vec<Point>,
}

impl PointsPath {
    pub fn circle_with(center: Point, radius: f32) -> Self {
        let mut path = PointsPath::default();
        let precision = 50;
        let angle_step = PI * 2.0 / precision as f32;
        for i in 0..precision {
            path.add_point(point_on_circle(radius, angle_step * i as f32, &center));
        }
        path
    }
}

impl PointsPath {
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn add_points(&mut self, points: impl IntoIterator<Item = Point>) {
        self.points.extend(points)
    }

    pub fn clear(&mut self) {
        self.points.clear()
    }
}

fn point_on_circle(radius: f32, angle: f32, center: &Point) -> Point {
    (
        (radius / 2.0) * angle.cos() + center.x,
        (radius / 2.0) * angle.sin() + center.y,
    )
        .into()
}

impl<T: Into<Point> + Copy> From<Vec<T>> for PointsPath {
    fn from(vec: Vec<T>) -> Self {
        Self {
            points: vec.iter().map(|a| (*a).into()).collect(),
        }
    }
}
