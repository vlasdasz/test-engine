use std::ops::{Deref, DerefMut};

use gl_image::Image;
use gm::Point;

use crate::{Level, Sprite, SpriteBase, Unit, Weapon};

#[derive(Debug)]
pub struct Player {
    unit:       Unit,
    pub weapon: Weapon,
}

impl Player {
    pub fn make(image: Image, level: &mut (impl Level + 'static)) -> Self {
        Player {
            unit:   Unit::make(image, level),
            weapon: Weapon::new(level),
        }
    }
}

impl Sprite for Player {
    fn update(&mut self) {
        let cursor = self.level().cursor_position();
        self.unit.update();
        self.weapon.rotation = self.position().angle_to(cursor);
        self.weapon.position = self.unit.position();
        self.weapon.velocity = self.velocity();

        self.image_mut().unwrap().flipped = cursor.x < self.position().x;
        self.weapon.image_mut().unwrap().flipped_y = cursor.x < self.position().x;
    }

    fn position(&self) -> Point {
        self.unit.position()
    }

    fn rotation(&self) -> f32 {
        self.unit.rotation()
    }

    fn draw(&self) {
        self.unit.draw();
        self.weapon.draw();
    }

    fn sprite(&self) -> &SpriteBase {
        self.unit.sprite()
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        self.unit.sprite_mut()
    }
}

impl Deref for Player {
    type Target = Unit;
    fn deref(&self) -> &Unit {
        &self.unit
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Unit {
        &mut self.unit
    }
}
