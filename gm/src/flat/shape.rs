use crate::flat::{Point, ProcessPoints, Size};

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rect(Size),
    Circle(f32),
    Triangle(Point, Point, Point),
}

impl Shape {
    pub fn triangle(a: impl Into<Point>, b: impl Into<Point>, c: impl Into<Point>) -> Self {
        Self::Triangle(a.into(), b.into(), c.into())
    }
}

impl Shape {
    pub fn size(&self) -> Size {
        match self {
            Self::Rect(size) => *size,
            Self::Circle(r) => (*r, *r).into(),
            Self::Triangle(a, b, c) => vec![a, b, c].size() / 2.0,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Rect(size) => size.width,
            Self::Circle(r) => *r,
            Self::Triangle(a, b, c) => vec![a, b, c].width() / 2.0,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Self::Rect(size) => size.height,
            Self::Circle(r) => *r,
            Self::Triangle(a, b, c) => vec![a, b, c].height() / 2.0,
        }
    }
}

impl<T: Into<Size>> From<T> for Shape {
    fn from(val: T) -> Self {
        Shape::Rect(val.into())
    }
}

impl From<f32> for Shape {
    fn from(val: f32) -> Self {
        Shape::Circle(val)
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Rect(Default::default())
    }
}
