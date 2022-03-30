use rapier2d::{
    na::Vector2,
    prelude::{ColliderBuilder, ColliderHandle},
};
use rtools::{Rglica, ToRglica};

use crate::{Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Wall {
    base:   SpriteBase,
    handle: ColliderHandle,
}

impl Wall {
    pub fn make(mut sprite: SpriteBase, level: &mut (impl Level + 'static)) -> Rglica<Self> {
        sprite.level = Rglica::from_ref(level.level());
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let handle = level.level_mut().sets.collider.insert(collider);
        let boxed = Box::new(Wall { base: sprite, handle });
        let wall = boxed.to_rglica();
        level.level_mut().sprites.push(boxed);
        wall
    }
}

impl Sprite for Wall {
    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.handle.into()
    }

    fn sprite(&self) -> &SpriteBase {
        &self.base
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.base
    }
}
