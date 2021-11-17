use std::any::Any;

use gm::Point;
use rapier2d::{dynamics::RigidBody, na::Vector2, prelude::RigidBodyHandle};
use serde::{Deserialize, Serialize};
use tools::{as_any::AsAny, Rglica};

use crate::{rigid_handle::RigidHandle, Control, Level, Sprite, SpriteBase};

#[derive(Deserialize, Serialize)]
pub struct Body {
    base:   SpriteBase,
    #[serde(skip)]
    handle: RigidHandle,
    #[serde(skip)]
    level:  Rglica<dyn Level>,
}

impl Body {
    pub fn make(
        base: SpriteBase,
        handle: RigidBodyHandle,
        level: &mut (impl Level + 'static),
    ) -> Self {
        Self {
            base,
            handle: handle.into(),
            level: Rglica::from_ref(level),
        }
    }

    fn body(&self) -> &RigidBody {
        &self.level.rigid_bodies()[self.handle.handle]
    }

    fn body_mut(&mut self) -> &mut RigidBody {
        &mut self.level.rigid_bodies_mut()[self.handle.handle]
    }

    pub fn lock_rotations(&mut self) {
        self.body_mut().lock_rotations(true, true);
    }
}

#[typetag::serde(name = "Body")]
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

impl AsAny for Body {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Control for Body {
    fn jump(&mut self) {
        self.body_mut().set_linvel([0.0, 50.0].into(), true)
    }

    fn go_left(&mut self) {
        self.body_mut().set_linvel([-50.0, 0.0].into(), true)
    }

    fn go_right(&mut self) {
        self.body_mut().set_linvel([50.0, 0.0].into(), true)
    }

    fn add_impulse(&mut self, impulse: &Point) {
        self.body_mut()
            .apply_force(Vector2::new(impulse.x * 1000.0, impulse.y * -1000.0), true)
    }
}
