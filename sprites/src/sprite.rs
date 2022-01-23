use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{Color, Point, Size};
use rtools::{as_any::AsAny, Address, IntoF32, Rglica, ToRglica};

use crate::Level;

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

    fn color(&self) -> Color {
        self.sprite().color
    }

    fn image(&self) -> &Option<Image> {
        &self.sprite().image
    }

    fn set_image(&mut self, image: Image) {
        self.sprite_mut().image = image.into()
    }

    fn remove(&mut self) {
        let address = self.to_rglica().address();
        self.level_mut().remove_sprite(address);
    }

    fn level(&self) -> &dyn Level {
        self.sprite().level.deref()
    }

    fn level_mut(&mut self) -> &mut dyn Level {
        self.sprite_mut().level.deref_mut()
    }

    fn sprite(&self) -> &SpriteBase;
    fn sprite_mut(&mut self) -> &mut SpriteBase;
}

#[derive(Default, Debug)]
pub struct SpriteBase {
    position: Point,
    size:     Size,
    rotation: f32,
    level:    Rglica<dyn Level>,

    pub color: Color,
    pub image: Option<Image>,
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

impl<W: IntoF32, H: IntoF32> From<(W, H)> for SpriteBase {
    fn from(data: (W, H)) -> Self {
        Self {
            size: (data.0.into_f32(), data.1.into_f32()).into(),
            color: Color::random(),
            ..Default::default()
        }
    }
}
