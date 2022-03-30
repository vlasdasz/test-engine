use std::ops::{Deref, DerefMut};

use gm::Point;
use rtools::ToRglica;

use crate::{Body, Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Player {
    body:       Body,
    pub weapon: SpriteBase,
}

impl Player {
    pub fn make(sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Self {
        let mut body = Body::make(sprite, level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);

        let mut weapon: SpriteBase = (0, 0, 5, 0.28).into();
        weapon.level = level.level().to_rglica();

        Player { body, weapon }
    }
}

impl Sprite for Player {
    fn update(&mut self) {
        self.body.update();
        self.weapon.position = self.body.position();
    }

    fn position(&self) -> Point {
        self.body.position()
    }

    fn rotation(&self) -> f32 {
        self.body.rotation()
    }

    fn draw(&self) {
        self.body.draw();
        self.weapon.draw();
    }

    fn sprite(&self) -> &SpriteBase {
        self.body.sprite()
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        self.body.sprite_mut()
    }
}

impl Deref for Player {
    type Target = Body;
    fn deref(&self) -> &Body {
        &self.body
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}
