use std::any::Any;

use gm::Point;
use rapier2d::{
    dynamics::RigidBody,
    na::Vector2,
    prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyHandle},
};
use rtools::{as_any::AsAny, Rglica, ToRglica};

use crate::{control::Control, Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Body {
    sprite: SpriteBase,
    handle: RigidBodyHandle,
}

impl Body {
    pub fn make(sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Rglica<Self> {
        let level_base = level.level_mut();

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .restitution(0.7)
            .build();

        let body_handle = level_base.sets.rigid_body.insert(rigid_body);
        level_base
            .sets
            .collider
            .insert_with_parent(collider, body_handle, &mut level_base.sets.rigid_body);

        #[allow(clippy::drop_ref)]
        drop(level_base);

        let boxed = Box::new(Self {
            sprite: sprite.with_level(level),
            handle: body_handle,
        });

        let body = boxed.to_rglica();

        level.level_mut().sprites.push(boxed);

        body
    }

    fn body(&self) -> &RigidBody {
        &self.level().rigid_bodies()[self.handle]
    }

    fn body_mut(&mut self) -> &mut RigidBody {
        let handle = self.handle;
        &mut self.level_mut().rigid_bodies_mut()[handle]
    }

    pub fn lock_rotations(&mut self) {
        self.body_mut().lock_rotations(true, true);
    }
}

impl Sprite for Body {
    fn position(&self) -> Point {
        (self.body().translation().x, self.body().translation().y).into()
    }

    fn rotation(&self) -> f32 {
        self.body().rotation().angle()
    }

    fn rigid_body_handle(&self) -> Option<RigidBodyHandle> {
        self.handle.into()
    }

    fn sprite(&self) -> &SpriteBase {
        &self.sprite
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}

impl AsAny for Body {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Body {
    fn _gravity(&self) -> Point {
        self.level().gravity()
    }
}

impl Control for Body {
    fn jump(&mut self) {
        self.add_impulse((0, -100).into());
    }

    fn go_left(&mut self) {
        self.add_impulse((-100, 0).into());
    }

    fn go_right(&mut self) {
        self.add_impulse((100, 0).into());
    }

    fn go_down(&mut self) {
        self.add_impulse((0, 100).into());
    }

    fn add_impulse(&mut self, impulse: Point) {
        self.body_mut()
            .apply_force([impulse.x * 1000.0, impulse.y * -1000.0].into(), true)
    }
}
