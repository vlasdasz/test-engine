use std::slice::from_raw_parts;

use crate::{
    flat::{Point, ProcessPoints, Size},
    ToF32,
};

#[derive(Clone, Debug)]
pub enum Shape {
    Rect(Size),
    Circle(f32),
    Triangle(Point, Point, Point),
    Polygon(Vec<Point>, bool),
}

impl Shape {
    pub fn rect(width: impl ToF32, height: impl ToF32) -> Self {
        Self::Rect((width, height).into())
    }

    pub fn triangle(a: impl Into<Point>, b: impl Into<Point>, c: impl Into<Point>) -> Self {
        Self::Triangle(a.into(), b.into(), c.into())
    }
}

impl Shape {
    pub fn size(&self) -> Size {
        match self {
            Self::Rect(size) => *size,
            Self::Circle(r) => (*r, *r).into(),
            Self::Triangle(a, b, c) => vec![*a, *b, *c].size() / 2.0,
            Self::Polygon(points, _) => points.size() / 2.0,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Rect(size) => size.width,
            Self::Circle(r) => *r,
            Self::Triangle(a, b, c) => vec![*a, *b, *c].width() / 2.0,
            Self::Polygon(points, _) => points.width() / 2.0,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Self::Rect(size) => size.height,
            Self::Circle(r) => *r,
            Self::Triangle(a, b, c) => vec![*a, *b, *c].height() / 2.0,
            Self::Polygon(points, _) => points.height() / 2.0,
        }
    }

    pub fn is_polygon(&self) -> bool {
        matches!(self, Shape::Polygon(..) | Shape::Triangle(..))
    }

    pub fn points(&self) -> &[Point] {
        match self {
            Shape::Polygon(points, _) => points,
            Shape::Triangle(a, _, _) => unsafe { from_raw_parts(a, 3) },
            _ => unreachable!("No points"),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Rect(Size::default())
    }
}

#[cfg(test)]
mod test {
    use crate::flat::Shape;

    #[test]
    fn triangle_to_points() {
        let tr = Shape::triangle((1, 2), (3, 4), (5, 6));
        assert_eq!(tr.points(), [(1, 2).into(), (3, 4).into(), (5, 6).into()]);
    }
}
