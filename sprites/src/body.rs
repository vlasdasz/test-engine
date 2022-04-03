use gm::Point;
use rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderHandle},
    na::Vector2,
    prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyHandle},
};
use rtools::{Rglica, ToRglica};

use crate::{control::Control, Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Body {
    sprite:          SpriteBase,
    rigid_handle:    RigidBodyHandle,
    collider_handle: ColliderHandle,
}

impl Body {
    pub fn make(sprite: SpriteBase, mut level: Rglica<dyn Level>) -> Self {
        let level_base = level.base_mut();

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .restitution(0.7)
            .build();

        let rigid_handle = level_base.sets.rigid_body.insert(rigid_body);

        let collider_handle = level_base.sets.collider.insert_with_parent(
            collider,
            rigid_handle,
            &mut level_base.sets.rigid_body,
        );

        Self {
            sprite: sprite.with_level(level),
            rigid_handle,
            collider_handle,
        }
    }

    pub fn body(&self) -> &RigidBody {
        &self.level().rigid_bodies()[self.rigid_handle]
    }

    pub fn body_mut(&mut self) -> &mut RigidBody {
        let handle = self.rigid_handle;
        &mut self.level_mut().rigid_bodies_mut()[handle]
    }

    pub fn collider(&self) -> &Collider {
        &self.level().colliders()[self.collider_handle]
    }

    pub fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.collider_handle;
        &mut self.level_mut().colliders_mut()[handle]
    }

    pub fn velocity(&self) -> Point {
        let vel = self.body().linvel();
        (vel.x, vel.y).into()
    }

    pub fn set_velocity(&mut self, vel: Point) {
        self.body_mut().set_linvel([vel.x, vel.y].into(), true)
    }

    pub fn lock_rotations(&mut self) {
        self.body_mut().lock_rotations(true, true);
    }
}

impl Sprite for Body {
    fn update(&mut self) {
        let mut this = self.to_rglica();
        this.sprite_mut().position = self.position();
        this.sprite_mut().rotation = self.rotation();
    }

    fn position(&self) -> Point {
        (self.body().translation().x, self.body().translation().y).into()
    }

    fn rotation(&self) -> f32 {
        self.body().rotation().angle()
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.sprite.rotation = rotation;
        self.body_mut().set_rotation(rotation, true);
    }

    fn rigid_body_handle(&self) -> Option<RigidBodyHandle> {
        self.rigid_handle.into()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.collider_handle.into()
    }

    fn sprite(&self) -> &SpriteBase {
        &self.sprite
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}

impl Body {
    fn _gravity(&self) -> Point {
        self.level().gravity()
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
        self.body_mut()
            .apply_force([impulse.x * 1000.0, impulse.y * 1000.0].into(), true)
    }
}
