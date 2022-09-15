use std::ops::DerefMut;

use gm::{
    flat::{Rect, Size},
    Color,
};
use rtools::Rglica;

use crate::{
    complex::{DrawMode, PathData},
    View,
};

static mut DRAWER: Option<Box<dyn UIDrawer>> = Option::None;

pub trait UIDrawer {
    fn reset_viewport(&self);
    fn fill(&self, rect: &Rect, color: &Color);
    fn outline(&self, rect: &Rect, color: &Color);
    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>);
    fn draw_round_border(&self, view: &mut dyn View);
    fn set_screen_scale(&mut self, scale: f32);
    fn set_scale(&mut self, scale: f32);
    fn set_size(&mut self, size: Size);
    fn update(&self, view: &mut dyn View);
    fn draw(&self, view: &mut dyn View);
    fn rglica(&self) -> Rglica<dyn UIDrawer>;
    fn window_size(&self) -> &Size;
}

pub fn set_ui_drawer(drawer: Box<dyn UIDrawer>) {
    unsafe { DRAWER = drawer.into() }
}

pub fn get_ui_drawer() -> &'static mut dyn UIDrawer {
    unsafe { DRAWER.as_mut().unwrap().deref_mut() }
}
