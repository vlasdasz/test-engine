use crate::te::Assets;
use crate::gm::{Size, Rect, Color};
use crate::ui::View;
use crate::gl_wrapper::GLWrapper;
use std::borrow::{BorrowMut, Borrow};
use crate::utils::Shared;

pub struct TEUIDrawer {
    assets: Assets,
    window_size: Size
}

impl TEUIDrawer {
    pub fn new(assets: Assets) -> TEUIDrawer {
        TEUIDrawer { assets, window_size: Size::new() }
    }
    pub fn set_size(&mut self, size: &Size) {
        self.window_size = *size;
    }
}

impl TEUIDrawer {
    pub fn draw_view(&self, view: Shared<View>) {
        {
            let mut borrowed_mut = view.try_borrow_mut().unwrap();
            borrowed_mut.calculate_absolute_frame();
            self.draw_rect(borrowed_mut.absolute_frame(), &borrowed_mut.color);
        }

        let borrowed = view.try_borrow().unwrap();

        for view in borrowed.subviews() {
            self.draw_view(view.clone());
        }
    }
}

impl TEUIDrawer {
    fn set_viewport(&self, rect: &Rect) {
        GLWrapper::set_viewport(self.window_size.height, 1.0, rect);
    }
}

impl TEUIDrawer {
    fn fill_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen.draw();
    }
    fn draw_rect(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable();
        self.assets.shaders.ui.set_color(color);
        self.assets.buffers.fullscreen_outline.draw();
    }
}