use crate::{Sprite, SpriteBase};
use tools::Rglica;

pub struct Collider {
    base: SpriteBase,
    _collider: Rglica<rapier2d::prelude::Collider>,
}

impl Collider {
    pub fn make(base: SpriteBase, collider: &mut rapier2d::prelude::Collider) -> Self {
        Self {
            base,
            _collider: Rglica::from_ref(collider),
        }
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
