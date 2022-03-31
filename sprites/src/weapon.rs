use std::{
    fmt::{Debug},
    ops::{Deref, DerefMut},
};

use rtools::ToRglica;

use crate::{Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Weapon {
    sprite: SpriteBase,
}

impl Weapon {
    pub fn new(level: &mut (impl Level + 'static)) -> Self {
        let mut sprite: SpriteBase = (0, 0, 2365.0 / 1000.0, 854.0 / 1000.0).into();
        sprite.level = level.level().to_rglica();
        Self { sprite }
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
