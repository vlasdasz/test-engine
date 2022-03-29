use derive_more::{Deref, DerefMut};

use crate::{Body, Level, SpriteBase};

#[derive(Deref, DerefMut)]
pub struct Player {
    body: Body,
}

impl Player {
    pub fn make(sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Self {
        Player {
            body: Body::make(sprite, level),
        }
    }
}
