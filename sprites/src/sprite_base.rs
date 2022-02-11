use std::any::Any;

use gl_image::Image;
use gm::{Color, Point, Size};
use rtools::{as_any::AsAny, IntoF32, Rglica};

use crate::{Level, Sprite};

#[derive(Default)]
pub struct SpriteBase {
    pub(crate) position:    Point,
    pub(crate) size:        Size,
    pub(crate) rotation:    f32,
    pub(crate) level:       Rglica<dyn Level>,
    pub(crate) is_selected: bool,

    pub color: Color,
    pub image: Option<Image>,
}

impl SpriteBase {
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.position.x - self.size.width
            && point.x <= self.position.x + self.size.width
            && point.y >= self.position.y - self.size.height
            && point.y <= self.position.y + self.size.height
    }
}

impl Sprite for SpriteBase {
    fn sprite(&self) -> &SpriteBase {
        self
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        self
    }
}

impl AsAny for SpriteBase {
    fn as_any(&self) -> &dyn Any {
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

impl From<(Point, Size)> for SpriteBase {
    fn from(data: (Point, Size)) -> Self {
        Self {
            position: data.0,
            size: data.1,
            color: Color::random(),
            ..Default::default()
        }
    }
}
