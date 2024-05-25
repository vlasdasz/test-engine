use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{na::Vector2, prelude::RigidBodyBuilder};
use refs::Own;

use crate::{LevelManager, Sprite, SpriteData, ToCollider};

pub struct Terrain {
    sprite: SpriteData,
}

impl Sprite for Terrain {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let collider = shape.make_collider().build();

        let level = LevelManager::level_mut().deref_mut();

        let rigid_handle = level.sets.rigid_bodies.insert(rigid_body);

        let collider_handle =
            level
                .sets
                .colliders
                .insert_with_parent(collider, rigid_handle, &mut level.sets.rigid_bodies);

        let mut sprite = SpriteData::make(shape, position);

        sprite.collider_handle = collider_handle.into();
        sprite.rigid_handle = rigid_handle.into();

        Own::new(Self { sprite })
    }
}

impl Deref for Terrain {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.sprite
    }
}

impl DerefMut for Terrain {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }
}
