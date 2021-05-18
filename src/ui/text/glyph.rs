use crate::gm::{Point, Size};
use crate::image::Image;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub ch: char,
    pub image: Image,
    pub advance: u32,
    pub bearing: Point,
}

impl Glyph {
    pub fn new(ch: char, image: Image, advance: u32, bearing: Point) -> Glyph {
        Glyph {
            ch,
            image,
            advance: advance / 2,
            bearing: Point {
                x: bearing.x / 2.0,
                y: bearing.y / 2.0,
            },
        }
    }

    pub fn size(&self) -> Size {
        Size {
            width: self.image.size.width / 2.0,
            height: self.image.size.height / 2.0,
        }
    }

    pub fn y_max(&self) -> f32 {
        self.bearing.y
    }

    pub fn y_min(&self) -> f32 {
        self.bearing.y - self.image.size.height
    }
}
