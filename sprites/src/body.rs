use crate::{Sprite, SpriteBase};
use gm::Point;
use tools::Rglica;

pub struct Body {
    base: SpriteBase,
    body: Rglica<rapier2d::prelude::RigidBody>,
}

impl Body {
    pub fn make(base: SpriteBase, rigid_body: &mut rapier2d::prelude::RigidBody) -> Self {
        Self {
            base,
            body: Rglica::from_ref(rigid_body),
        }
    }
}

impl Sprite for Body {
    fn position(&self) -> Point {
        (self.body.translation().x, self.body.translation().y).into()
    }

    fn rotation(&self) -> f32 {
        self.body.rotation().angle()
    }

    fn sprite(&self) -> &SpriteBase {
        &self.base
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.base
    }
}
