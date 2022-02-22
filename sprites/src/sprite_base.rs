use gl_image::Image;
use gm::{Color, Point, Rect, Size};
use rtools::{IntoF32, Rglica, ToRglica};

use crate::{Level, LevelBase};

#[derive(Default, Debug)]
pub struct SpriteBase {
    pub(crate) position:    Point,
    pub(crate) size:        Size,
    pub(crate) rotation:    f32,
    pub(crate) level:       Rglica<LevelBase>,
    pub(crate) is_selected: bool,

    pub color: Color,
    pub image: Option<Image>,
}

impl SpriteBase {
    pub(crate) fn with_level<T: Level + ?Sized>(mut self, level: &T) -> Self {
        self.level = level.level().to_rglica();
        self
    }
}

impl<X: IntoF32, Y: IntoF32, W: IntoF32, H: IntoF32> From<(X, Y, W, H)> for SpriteBase {
    fn from(data: (X, Y, W, H)) -> Self {
        Self {
            position: (data.0.into_f32(), data.1.into_f32()).into(),
            size: (data.2.into_f32(), data.3.into_f32()).into(),
            color: Color::random(),
            ..Default::default()
        }
    }
}

impl From<Rect> for SpriteBase {
    fn from(rect: Rect) -> Self {
        Self {
            position: rect.origin,
            size: rect.size,
            color: Color::random(),
            ..Default::default()
        }
    }
}
