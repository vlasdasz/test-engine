use std::ops::{Deref, DerefMut};

use gm::{
    flat::{Point, Shape},
    ToF32,
};
use rapier2d::{geometry::ColliderHandle, na::Vector2};
use refs::Own;

use crate::{LevelManager, Sprite, SpriteData, SpriteTemplates, ToCollider};

pub struct Wall {
    collider_handle: ColliderHandle,
    sprite:          SpriteData,
}

impl Wall {
    pub fn set_x(&mut self, x: impl ToF32) {
        let mut pos = self.position();
        pos.x = x.to_f32();
        self.set_position(pos);
    }

    pub fn set_y(&mut self, y: impl ToF32) {
        let mut pos = self.position();
        pos.y = y.to_f32();
        self.set_position(pos);
    }
}

impl Sprite for Wall {
    fn make(shape: Shape, position: Point) -> Own<Self> {
        let collider = shape
            .make_collider()
            .translation(Vector2::new(position.x, position.y))
            .restitution(1.0)
            .build();

        let sprite = SpriteData::make(shape, position);
        let collider_handle = LevelManager::level_mut().sets.colliders.insert(collider);

        Own::new(Wall {
            collider_handle,
            sprite,
        })
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.collider_handle.into()
    }
}

impl Deref for Wall {
    type Target = SpriteData;

    fn deref(&self) -> &Self::Target {
        &self.sprite
    }
}

impl DerefMut for Wall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sprite
    }
}
