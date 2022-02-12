use std::ops::{Deref, DerefMut};

use gl_image::Image;
use gm::{Color, Point, Size};
use rtools::as_any::AsAny;

use crate::{Level, SpriteBase};

pub trait Sprite: AsAny {
    fn size(&self) -> Size {
        self.sprite().size
    }

    fn position(&self) -> Point {
        self.sprite().position
    }

    fn rotation(&self) -> f32 {
        self.sprite().rotation
    }

    fn contains(&self, point: Point) -> bool {
        let pos = self.position();
        let size = self.size();
        point.x >= pos.x - size.width
            && point.x <= pos.x + size.width
            && point.y >= pos.y - size.height
            && point.y <= pos.y + size.height
    }

    fn color(&self) -> Color {
        self.sprite().color
    }

    fn image(&self) -> &Option<Image> {
        &self.sprite().image
    }

    fn set_image(&mut self, image: Image) {
        self.sprite_mut().image = image.into()
    }

    fn is_selected(&self) -> bool {
        self.sprite().is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.sprite_mut().is_selected = selected
    }

    fn remove(&mut self) {}

    fn level(&self) -> &dyn Level {
        self.sprite().level.deref()
    }

    fn level_mut(&mut self) -> &mut dyn Level {
        self.sprite_mut().level.deref_mut()
    }

    fn sprite(&self) -> &SpriteBase;
    fn sprite_mut(&mut self) -> &mut SpriteBase;
}
