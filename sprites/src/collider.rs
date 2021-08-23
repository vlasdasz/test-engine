use crate::{Sprite, SpriteBase};
use rapier2d::prelude::ColliderHandle;
use tools::new;

pub struct Collider {
    base: SpriteBase,
    handle: ColliderHandle,
}

impl Collider {
    pub fn make(base: SpriteBase, handle: ColliderHandle) -> Self {
        Self { base, handle }
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
