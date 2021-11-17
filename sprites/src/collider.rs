use std::any::Any;

use serde::{Deserialize, Serialize};
use tools::as_any::AsAny;

use crate::{Sprite, SpriteBase};

#[derive(Serialize, Deserialize)]
pub struct Collider {
    base: SpriteBase,
}

#[typetag::serde(name = "Collider")]
impl Sprite for Collider {
    fn sprite(&self) -> &SpriteBase {
        &self.base
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.base
    }
}

impl AsAny for Collider {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<SpriteBase> for Collider {
    fn from(base: SpriteBase) -> Self {
        Self { base }
    }
}
