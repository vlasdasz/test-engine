use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{geometry::ColliderHandle, na::Vector2};
use refs::Own;

use crate::{LevelManager, Sprite, SpriteData, ToCollider};

pub struct Wall {
    collider_handle: ColliderHandle,
    sprite:          SpriteData,
}

impl Sprite for Wall {
    fn make(shape: Shape, position: Point) -> Own<Self> {
        let collider = shape
            .make_collider()
            .translation(Vector2::new(position.x, position.y))
            .restitution(1.0)
            .build();

        let sprite = SpriteData::make(shape, position);
        let collider_handle = LevelManager::level_weak().sets.colliders.insert(collider);

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
