use gm::Point;
use rapier2d::{
    na::Vector2,
    prelude::{Collider, ColliderBuilder, ColliderHandle},
};
use rtools::{IntoF32, Rglica, ToRglica};

use crate::{Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Wall {
    base:   SpriteBase,
    handle: ColliderHandle,
}

impl Wall {
    pub fn make(mut sprite: SpriteBase, mut level: Rglica<dyn Level>) -> Rglica<Self> {
        sprite.level = level;
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let handle = level.base_mut().sets.collider.insert(collider);
        let boxed = Box::new(Wall { base: sprite, handle });
        let wall = boxed.to_rglica();
        level.base_mut().sprites.push(boxed);
        wall
    }

    pub fn collider(&self) -> &Collider {
        &self.level().colliders()[self.handle]
    }

    pub fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.handle;
        &mut self.level_mut().colliders_mut()[handle]
    }

    pub fn set_x(&mut self, x: impl IntoF32) {
        let mut pos = self.position();
        pos.x = x.into_f32();
        self.set_position(pos);
    }

    pub fn set_y(&mut self, y: impl IntoF32) {
        let mut pos = self.position();
        pos.y = y.into_f32();
        self.set_position(pos);
    }

    pub fn set_position(&mut self, pos: Point) {
        self.collider_mut().set_position([pos.x, pos.y].into())
    }
}

impl Sprite for Wall {
    fn update(&mut self) {
        let mut this = self.to_rglica();
        this.sprite_mut().position = self.position();
        this.sprite_mut().rotation = self.rotation();
    }

    fn position(&self) -> Point {
        let pos = self.collider().position().translation;
        (pos.x, pos.y).into()
    }

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
