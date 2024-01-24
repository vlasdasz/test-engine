use std::f32::consts::PI;

use rtools::IntoF32;

use crate::flat::{Point, Rect};

pub type Points = Vec<Point>;

pub struct PointsPath {}

impl PointsPath {
    pub fn circle_with(center: Point, radius: f32, precision: u16) -> Points {
        let mut path = vec![];
        let angle_step = PI * 2.0 / f32::from(precision);
        for i in 0..precision {
            path.push(point_on_circle(radius, angle_step * f32::from(i), center));
        }
        path
    }

    pub fn rounded_rect(rect: impl Into<Rect>, radius: impl IntoF32, precision: u16) -> Points {
        let mut path = vec![];
        let rect = rect.into();
        let radius = radius.into_f32();

        let a = (rect.x() + radius, rect.y() + radius);
        let b = (rect.max_x() - radius, rect.y() + radius);
        let c = (rect.max_x() - radius, rect.max_y() - radius);
        let d = (rect.x() + radius, rect.max_y() - radius);

        let angle_step = PI * 0.5 / f32::from(precision);

        for i in 0..precision {
            path.push(point_on_circle(radius, -3.0 + angle_step * f32::from(i), a));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, -1.5 + angle_step * f32::from(i), b));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, angle_step * f32::from(i), c));
        }

        for i in 0..precision {
            path.push(point_on_circle(radius, 1.5 + angle_step * f32::from(i), d));
        }

        path
    }
}

pub fn point_on_circle(radius: f32, angle: f32, center: impl Into<Point>) -> Point {
    let center = center.into();
    (radius * angle.cos() + center.x, radius * angle.sin() + center.y).into()
}
