use crate::{Sprite, SpriteBase};
use rapier2d::prelude::ColliderHandle;

pub struct Collider {
    base: SpriteBase,
    _handle: ColliderHandle,
}

impl Collider {
    pub fn make(base: SpriteBase, _handle: ColliderHandle) -> Self {
        Self { base, _handle }
    }
}

impl Sprite for Collider {
    fn sprite(&self) -> &SpriteBase {
        &self.base
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.base
    }
}
