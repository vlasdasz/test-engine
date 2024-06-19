use gm::flat::{Point, Rect, Shape};
use refs::{Own, Weak};

use crate::{Level, Object, Sprite};

pub trait LevelCreation {
    fn add_sprite<S: 'static + Sprite>(&mut self, sprite: Own<S>) -> Weak<S>;
    fn make_sprite<S: 'static + Sprite>(&mut self, _: Shape, _: impl Into<Point>) -> Weak<S>;
    fn make_rect(&mut self, rect: impl Into<Rect>) -> Weak<Object>;
}

impl<T: ?Sized + Level> LevelCreation for T {
    fn add_sprite<S: 'static + Sprite>(&mut self, sprite: Own<S>) -> Weak<S> {
        let weak = sprite.weak();
        self.sprites.push(sprite);
        weak
    }

    fn make_sprite<S: 'static + Sprite>(&mut self, shape: Shape, position: impl Into<Point>) -> Weak<S> {
        self.add_sprite(S::make(shape, position.into()))
    }

    fn make_rect(&mut self, rect: impl Into<Rect>) -> Weak<Object> {
        let rect = rect.into();
        self.make_sprite::<Object>(Shape::Rect(rect.size), rect.origin)
    }
}
