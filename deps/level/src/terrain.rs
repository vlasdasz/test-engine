use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{na::Vector2, prelude::RigidBodyBuilder};
use refs::{Own, Weak};

use crate::{Level, Sprite, SpriteData, ToCollider};

pub struct Terrain {
    sprite: Own<SpriteData>,
}

impl Sprite for Terrain {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point, mut level: Weak<dyn Level>) -> Own<Self>
    where Self: Sized {
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let collider = shape.make_collider().build();

        let level_base = level.base_mut();

        let rigid_handle = level_base.sets.rigid_body.insert(rigid_body);

        let collider_handle = level_base.sets.collider.insert_with_parent(
            collider,
            rigid_handle,
            &mut level_base.sets.rigid_body,
        );

        let mut sprite = SpriteData::make(shape, position, level);

        sprite.collider_handle = collider_handle.into();
        sprite.rigid_handle = rigid_handle.into();
        sprite.level = level;

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
