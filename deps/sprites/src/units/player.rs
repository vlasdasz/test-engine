use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rtools::Rglica;

use crate::{Level, Sprite, SpriteData, Unit, Weapon};

pub struct Player {
    unit:       Unit,
    pub weapon: Weapon,
}

impl Sprite for Player {
    fn update(&mut self) {
        let cursor = self.level().cursor_position();
        self.weapon.rotation = self.position().angle_to(cursor);
        self.weapon.position = self.unit.position();
        self.weapon.velocity = self.velocity();

        if self.image.is_ok() {
            self.image().flipped = cursor.x < self.position().x;
        }
        if self.weapon.image.is_ok() {
            self.weapon.image().flipped_y = cursor.x < self.position().x;
        }
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

    fn data(&self) -> &SpriteData {
        self.unit.data()
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        self.unit.data_mut()
    }

    fn make(shape: Shape, position: Point, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Player {
            unit:   *Unit::make(shape, position, level),
            weapon: *Weapon::make(shape, position, level),
        })
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
