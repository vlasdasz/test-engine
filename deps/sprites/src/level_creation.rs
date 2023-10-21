use gm::flat::{Point, Rect, Shape};
use refs::{weak_from_ref, Weak};

use crate::{Level, Sprite, SpriteData};

pub trait LevelCreation {
    fn add_sprite<S: 'static + Sprite>(&self, _: impl Into<Shape>, _: impl Into<Point>) -> Weak<S>;
    fn add_rect(&self, rect: impl Into<Rect>) -> Weak<SpriteData>;
}

impl<T: ?Sized + Level> LevelCreation for T {
    fn add_sprite<S: 'static + Sprite>(
        &self,
        shape: impl Into<Shape>,
        position: impl Into<Point>,
    ) -> Weak<S> {
        let sprite = S::make(shape.into(), position.into(), weak_from_ref::<dyn Level>(self));
        let result = sprite.weak();
        self.base_mut().sprites.push(sprite);
        result
    }

    fn add_rect(&self, rect: impl Into<Rect>) -> Weak<SpriteData> {
        let rect = rect.into();
        self.add_sprite::<SpriteData>(rect.size, rect.origin)
    }
}
