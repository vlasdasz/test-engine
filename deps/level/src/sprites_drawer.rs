use std::ops::DerefMut;

use gm::flat::{Point, Size};

use crate::Sprite;

static mut DRAWER: Option<Box<dyn SpritesDrawer>> = None;

pub trait SpritesDrawer {
    fn scale(&self) -> f32;
    fn set_scale(&mut self, scale: f32);
    fn resolution(&self) -> Size<u32>;
    fn set_resolution(&mut self, size: Size<u32>);
    fn set_camera_rotation(&self, angle: f32);
    fn camera_position(&self) -> Point;
    fn set_camera_position(&mut self, pos: Point);
    fn draw(&self, sprite: &dyn Sprite);
}

pub fn set_sprites_drawer(drawer: Box<dyn SpritesDrawer>) {
    unsafe { DRAWER = drawer.into() }
}

pub fn get_sprites_drawer() -> &'static mut dyn SpritesDrawer {
    unsafe { DRAWER.as_mut().unwrap().deref_mut() }
}
