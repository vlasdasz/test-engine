use gm::flat::{Point, Shape};
use refs::Own;

use crate::{Sprite, SpriteData};

pub struct Object {
    sprite: SpriteData,
}

impl Sprite for Object {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        Own::new(Self {
            sprite: SpriteData::make(shape, position),
        })
    }
}
