use std::any::Any;

use serde::{Deserialize, Serialize};
use tools::{as_any::AsAny, Rglica};

use crate::{rigid_handle::RigidHandle, Level, Sprite, SpriteBase};

#[derive(Serialize, Deserialize)]
pub enum Rigid {
    Body {
        base:   SpriteBase,
        #[serde(skip)]
        handle: RigidHandle,
        #[serde(skip)]
        level:  Rglica<dyn Level>,
    },
    Collider {
        base: SpriteBase,
    },
}

impl Sprite for Rigid {
    fn sprite(&self) -> &SpriteBase {
        match self {
            Rigid::Body {
                base,
                handle: _,
                level: _,
            } => base,
            Rigid::Collider { base } => base,
        }
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        match self {
            Rigid::Body {
                base,
                handle: _,
                level: _,
            } => base,
            Rigid::Collider { base } => base,
        }
    }
}

impl AsAny for Rigid {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
