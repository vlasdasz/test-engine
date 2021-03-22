use crate::te::Assets;
use crate::gm::{ Rect, Color };
use crate::gl_wrapper::GLWrapper;

pub struct TEUIDrawer<'a> {
    gl:     &'a GLWrapper,
    assets: &'a Assets
}

impl<'a> TEUIDrawer<'a> {
    pub fn new(gl: &'a GLWrapper, assets: &'a Assets) -> TEUIDrawer<'a> {
        TEUIDrawer { gl, assets }
    }
}

impl TEUIDrawer<'_> {
    fn set_viewport(&self, rect: &Rect) {
        self.gl.set_viewport(rect)
    }
}

impl TEUIDrawer<'_> {
    pub fn fill_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen.draw();
    }
    pub fn draw_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen_outline.draw();
    }
}