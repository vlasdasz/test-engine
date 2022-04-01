use std::ops::{Deref, DerefMut};

use gm::Point;

use crate::{Body, Level, Sprite, SpriteBase, Weapon};

#[derive(Debug)]
pub struct Player {
    body:       Body,
    pub weapon: Weapon,
}

impl Player {
    pub fn make(sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Self {
        let mut body = Body::make(sprite, level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);

        Player {
            body,
            weapon: Weapon::new(level),
        }
    }
}

impl Sprite for Player {
    fn update(&mut self) {
        let cursor = self.level().cursor_position();
        self.body.update();
        self.weapon.rotation = self.position().angle_to(cursor);
        self.weapon.position = self.body.position();
        self.image_mut().unwrap().flipped = cursor.x < self.position().x;
        self.weapon.image_mut().unwrap().flipped_y = cursor.x < self.position().x;
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
