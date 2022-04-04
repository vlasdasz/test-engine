use gm::{Point, Rect};
use rapier2d::{
    na::Vector2,
    prelude::{Collider, ColliderBuilder},
};
use rtools::{IntoF32, Rglica};

use crate::{Level, Sprite, SpriteData};

#[derive(Debug)]
pub struct Wall {
    data: SpriteData,
}

impl Wall {
    pub fn collider(&self) -> &Collider {
        &self.level().colliders()[self.data.collider_handle.unwrap()]
    }

    pub fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.data.collider_handle.unwrap();
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

    fn data(&self) -> &SpriteData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.data
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
        sprite.collider_handle = level.base_mut().sets.collider.insert(collider).into();
        Box::new(Wall { data: sprite })
    }
}
