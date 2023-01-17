use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{na::Vector2, prelude::RigidBodyBuilder};
use refs::{Strong, Weak};

use crate::{control::Control, Level, Sprite, SpriteData, ToCollider};

pub struct Body {
    sprite: Strong<SpriteData>,
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

    pub fn lock_rotations(&mut self) -> &mut Self {
        self.rigid_body_mut().lock_rotations(true, true);
        self
    }

    pub fn unlock_rotation(&mut self) -> &mut Self {
        self.rigid_body_mut().lock_rotations(false, true);
        self
    }
}

impl Sprite for Body {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point, mut level: Weak<dyn Level>) -> Strong<Self>
    where Self: Sized {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let collider = shape.to_collider().build();

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
        sprite.level = level.rglica();

        Strong::new(Self { sprite })
    }
}

const FORCE: f32 = 10.0;
impl Control for Body {
    fn jump(&mut self) {
        self.add_impulse((0, FORCE).into());
    }

    fn go_left(&mut self) {
        self.add_impulse((-FORCE, 0).into());
    }

    fn go_right(&mut self) {
        self.add_impulse((FORCE, 0).into());
    }

    fn go_down(&mut self) {
        self.add_impulse((0, -FORCE).into());
    }

    fn add_impulse(&mut self, impulse: Point) {
        self.rigid_body_mut()
            .apply_impulse([impulse.x * 100.0, impulse.y * 100.0].into(), true)
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
