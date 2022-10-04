use gm::flat::{Point, Shape};
use rapier2d::na::Vector2;
use refs::{Strong, Weak};
use rtools::IntoF32;

use crate::{sprite::SpriteTemplates, Level, Sprite, SpriteData, ToCollider};

pub struct Wall {
    data: Strong<SpriteData>,
}

impl Wall {
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
}

impl Sprite for Wall {
    fn data(&self) -> &SpriteData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.data
    }

    fn make(shape: Shape, position: Point, mut level: Weak<dyn Level>) -> Strong<Self> {
        let collider = shape
            .to_collider()
            .translation(Vector2::new(position.x, position.y))
            .restitution(1.0)
            .build();
        let mut sprite = SpriteData::make(shape, position, level);
        sprite.collider_handle = level.base_mut().sets.collider.insert(collider).into();
        Strong::new(Wall { data: sprite })
    }
}
