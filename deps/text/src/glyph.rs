use gl_image::{GlImage, ToImage};
use gm::flat::{Point, Size};
use refs::Weak;

#[derive(Clone, Debug)]
pub struct Glyph {
    pub ch:      char,
    pub size:    Size,
    pub image:   Weak<GlImage>,
    pub advance: f32,
    pub bearing: Point,
}

impl Glyph {
    pub fn new(ch: char, image: Weak<GlImage>, advance: f32, bearing: Point) -> Glyph {
        Glyph {
            ch,
            size: image.size / 2.0,
            image,
            advance: advance / 2.0,
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
        self.bearing.y - self.image.to_image().size.height
    }
}
