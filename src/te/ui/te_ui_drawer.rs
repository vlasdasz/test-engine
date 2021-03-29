use crate::te::Assets;
use crate::gm::{Size, Rect, Color};
use crate::ui::ViewBase;
use crate::gl_wrapper::GLWrapper;
use crate::utils::{Shared, Platform};
use crate::image::Image;

pub struct TEUIDrawer {
    pub assets: Assets,
    pub window_size: Size
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

    pub fn draw_view(&self, view: Shared<ViewBase>) {
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
        const SCALE: f32 = if Platform::MAC { 2.0 } else { 1.0 };
        GLWrapper::set_viewport(self.window_size.height, SCALE, rect);
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

    pub fn draw_image_in_rect(&self, image: &Image, rect: &Rect, color: &Color) {
        if rect.size.is_negative() {
            return
        }

        if image.is_monochrome() {
            self.assets.shaders.ui_monochrome.enable();
            self.assets.shaders.ui_monochrome.set_color(color);
        }
        else {
            self.assets.shaders.ui_texture.enable();
        }

        self.set_viewport(rect);
        image.bind();
        self.assets.buffers.fullscreen_image.draw();
    }
}