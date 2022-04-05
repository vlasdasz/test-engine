use gm::Rect;
use rapier2d::{na::Vector2, prelude::ColliderBuilder};
use rtools::{IntoF32, Rglica};

use crate::{Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Wall {
    data: SpriteData,
}

impl Wall {
    pub fn set_x(&mut self, x: impl IntoF32) {
        let mut pos = self.position();
        pos.x = x.into_f32();
        self.set_position(pos);
    }

    pub fn set_y(&mut self, y: impl IntoF32) {
        let mut pos = self.position();
        pos.y = y.into_f32();
        self.set_position(pos);
    }
}

impl Sprite for Wall {
    fn data(&self) -> &SpriteData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.data
    }

    fn make(rect: Rect, mut level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        let mut sprite = SpriteData::from(rect);
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        sprite.collider_handle = level.base_mut().sets.collider.insert(collider).into();
        Box::new(Wall {
            data: sprite.with_level(level),
        })
    }
}
