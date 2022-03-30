use derive_more::{Deref, DerefMut};

use crate::{Body, Level, SpriteBase};

#[derive(Deref, DerefMut)]
pub struct Player {
    body: Body,
}

impl Player {
    pub fn make(sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Self {
        let mut body = Body::make(sprite, level);

        body.lock_rotations();
        body.collider_mut().set_restitution(0.0);

        Player { body }
    }
}
