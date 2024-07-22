use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, na::Vector2, prelude::RigidBodyBuilder};
use refs::Own;

use crate::{control::Control, LevelManager, Sprite, SpriteData, ToCollider};

pub struct Body {
    rigid_handle:    RigidBodyHandle,
    collider_handle: ColliderHandle,
    sprite:          SpriteData,

    pub jump_force: f32,
}

impl Body {
    pub fn velocity(&self) -> Point {
        let vel = self.rigid_body().linvel();
        (vel.x, vel.y).into()
    }

    pub fn set_velocity(&mut self, vel: Point) -> &mut Self {
        self.rigid_body_mut().set_linvel([vel.x, vel.y].into(), true);
        self
    }
}

impl Sprite for Body {
    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let collider = shape.make_collider().build();

        let (rigid_handle, collider_handle) =
            LevelManager::level_weak().physics.sets.insert(rigid_body, collider);

        let sprite = SpriteData::make(shape, position);

        Own::new(Self {
            rigid_handle,
            collider_handle,
            sprite,
            jump_force: 0.1,
        })
    }

    fn rigid_handle(&self) -> Option<RigidBodyHandle> {
        self.rigid_handle.into()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.collider_handle.into()
    }
}

impl Control for Body {
    fn jump(&mut self) {
        self.add_impulse((0, self.jump_force).into());
    }

    fn go_left(&mut self) {
        self.add_impulse((-self.jump_force, 0).into());
    }

    fn go_right(&mut self) {
        self.add_impulse((self.jump_force, 0).into());
    }

    fn go_down(&mut self) {
        self.add_impulse((0, -self.jump_force).into());
    }

    fn add_impulse(&mut self, impulse: Point) {
        self.rigid_body_mut()
            .apply_impulse([impulse.x * 100.0, impulse.y * 100.0].into(), true);
    }
}

impl Deref for Body {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.sprite
    }
}

impl DerefMut for Body {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }
}
