use gm::{Point, Size};

use crate::Sprite;

pub trait SpritesDrawer {
    fn set_scale(&self, scale: f32);
    fn set_resolution(&self, size: &Size);
    fn set_camera_rotation(&self, angle: f32);
    fn set_camera_position(&self, pos: Point);
    fn draw(&self, sprite: &dyn Sprite);
}

#[derive(Default)]
pub struct DummyDrawer {}

impl SpritesDrawer for DummyDrawer {
    fn set_scale(&self, _scale: f32) {
        todo!()
    }
    fn set_resolution(&self, _size: &Size) {
        todo!()
    }
    fn set_camera_rotation(&self, _angle: f32) {
        todo!()
    }
    fn set_camera_position(&self, _pos: Point) {
        todo!()
    }
    fn draw(&self, _sprite: &dyn Sprite) {
        todo!()
    }
}