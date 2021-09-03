use crate::{Level, Sprite, SpriteBase};
use gm::Point;
use rapier2d::dynamics::RigidBody;
use rapier2d::prelude::RigidBodyHandle;
use tools::Rglica;

pub struct Body {
    base: SpriteBase,
    handle: RigidBodyHandle,
    level: Rglica<dyn Level>,
}

impl Body {
    pub fn make(
        base: SpriteBase,
        handle: RigidBodyHandle,
        level: &mut (impl Level + 'static),
    ) -> Self {
        Self {
            base,
            handle,
            level: Rglica::from_ref(level),
        }
    }

    fn body(&self) -> &RigidBody {
        &self.level.rigid_bodies()[self.handle]
    }
}

impl Sprite for Body {
    fn position(&self) -> Point {
        (self.body().translation().x, self.body().translation().y).into()
    }

    fn rotation(&self) -> f32 {
        self.body().rotation().angle()
    }

    fn sprite(&self) -> &SpriteBase {
        &self.base
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.base
    }
}
