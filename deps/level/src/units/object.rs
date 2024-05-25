use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use refs::Own;

use crate::{Sprite, SpriteData};

pub struct Object {
    sprite: SpriteData,
}

impl Deref for Object {
    type Target = SpriteData;

    fn deref(&self) -> &Self::Target {
        &self.sprite
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sprite
    }
}

impl Sprite for Object {
    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        Own::new(Self {
            sprite: SpriteData::make(shape, position),
        })
    }
}
