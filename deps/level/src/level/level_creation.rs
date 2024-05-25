use gm::flat::{Point, Rect, Shape};
use refs::Weak;

use crate::{Level, Object, Sprite};

pub trait LevelCreation {
    fn add_sprite<S: 'static + Sprite>(&mut self, _: Shape, _: impl Into<Point>) -> Weak<S>;
    fn add_rect(&mut self, rect: impl Into<Rect>) -> Weak<Object>;
}

impl<T: ?Sized + Level> LevelCreation for T {
    fn add_sprite<S: 'static + Sprite>(&mut self, shape: Shape, position: impl Into<Point>) -> Weak<S> {
        let sprite = S::make(shape, position.into());
        let result = sprite.weak();
        self.sprites.push(sprite);
        result
    }

    fn add_rect(&mut self, rect: impl Into<Rect>) -> Weak<Object> {
        let rect = rect.into();
        self.add_sprite::<Object>(Shape::Rect(rect.size), rect.origin)
    }
}
