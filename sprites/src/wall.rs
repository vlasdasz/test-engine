use gm::{Point, Rect};
use rapier2d::{
    na::Vector2,
    prelude::{Collider, ColliderBuilder, ColliderHandle},
};
use rtools::{IntoF32, Rglica};

use crate::{Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Wall {
    base:   SpriteData,
    handle: ColliderHandle,
}

impl Wall {
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
    fn position(&self) -> Point {
        let pos = self.collider().position().translation;
        (pos.x, pos.y).into()
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.handle.into()
    }

    fn data(&self) -> &SpriteData {
        &self.base
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.base
    }

    fn make(rect: Rect, mut level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        let mut sprite = SpriteData::from(rect);
        sprite.level = level;
        let collider = ColliderBuilder::cuboid(sprite.size.width, sprite.size.height)
            .translation(Vector2::new(sprite.position.x, sprite.position.y))
            .build();
        let handle = level.base_mut().sets.collider.insert(collider);
        Box::new(Wall { base: sprite, handle })
    }
}
