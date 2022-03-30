use gm::{Point, Size};

use crate::Sprite;

pub trait SpritesDrawer {
    fn scale(&self) -> f32;
    fn set_scale(&mut self, scale: f32);
    fn resolution(&self) -> Size;
    fn set_resolution(&mut self, size: Size);
    fn set_camera_rotation(&self, angle: f32);
    fn set_camera_position(&self, pos: Point);
    fn draw(&self, sprite: &dyn Sprite);
}
