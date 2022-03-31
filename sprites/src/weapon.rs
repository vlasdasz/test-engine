use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::Point;
use rtools::ToRglica;

use crate::{Control, Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Weapon {
    sprite:           SpriteBase,
    pub bullet_image: Option<Image>,
}

impl Weapon {
    pub fn new(level: &mut (impl Level + 'static)) -> Self {
        let mut sprite: SpriteBase = (0, 0, 2365.0 / 1000.0, 854.0 / 1000.0).into();
        sprite.level = level.level().to_rglica();
        Self {
            sprite,
            bullet_image: None,
        }
    }

    pub fn shoot_at(&mut self, pos: Point) {
        let mut body = self.level_mut().add_body((pos.x, pos.y, 0.8, 0.15).into());
        body.set_rotation(self.rotation());
        let mut impulse = pos - self.position();
        impulse /= 10;
        body.add_impulse(impulse);
        if let Some(image) = &self.bullet_image {
            body.set_image(image.clone())
        }
    }
}

impl Sprite for Weapon {
    fn sprite(&self) -> &SpriteBase {
        &self.sprite
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}

impl Deref for Weapon {
    type Target = SpriteBase;
    fn deref(&self) -> &SpriteBase {
        &self.sprite
    }
}

impl DerefMut for Weapon {
    fn deref_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}
