use crate::flat::Size;

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rect(Size),
    Circle(f32),
}

impl Shape {
    pub fn size(&self) -> Size {
        match self {
            Self::Rect(size) => *size,
            Self::Circle(r) => (*r, *r).into(),
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Rect(size) => size.width,
            Self::Circle(r) => *r,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Self::Rect(size) => size.height,
            Self::Circle(r) => *r,
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
