use gm::{Point, Size};
use image::Image;

#[derive(Debug, Copy, Clone)]
pub struct Glyph {
    pub ch: char,
    pub size: Size,
    pub image: Image,
    pub advance: u32,
    pub bearing: Point,
}

impl Glyph {
    pub fn new(ch: char, image: Image, advance: u32, bearing: Point) -> Glyph {
        Glyph {
            ch,
            size: Size {
                width: image.size.width / 2.0,
                height: image.size.height / 2.0,
            },
            image,
            advance: advance / 2,
            bearing: Point {
                x: bearing.x / 2.0,
                y: bearing.y / 2.0,
            },
        }
    }

    pub fn y_max(&self) -> f32 {
        self.bearing.y
    }

    pub fn y_min(&self) -> f32 {
        self.bearing.y - self.image.size.height
    }
}
