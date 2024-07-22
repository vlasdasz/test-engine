use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{geometry::ColliderHandle, na::Vector2};
use refs::Own;

use crate::{LevelManager, Sprite, SpriteData, ToCollider};

pub struct Sensor {
    collider_handle: ColliderHandle,
    sprite:          SpriteData,
}

impl Sprite for Sensor {
    fn make(shape: Shape, position: Point) -> Own<Self> {
        let collider = shape
            .make_collider()
            .sensor(true)
            .translation(Vector2::new(position.x, position.y))
            .build();

        let sprite = SpriteData::make(shape, position);
        let collider_handle = LevelManager::level_weak().physics.sets.colliders.insert(collider);

        let mut new = Own::new(Self {
            collider_handle,
            sprite,
        });

        new.enable_collision_detection();

        new
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.collider_handle.into()
    }
}

impl Deref for Sensor {
    type Target = SpriteData;

    fn deref(&self) -> &Self::Target {
        &self.sprite
    }
}

impl DerefMut for Sensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sprite
    }
}
