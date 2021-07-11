use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Rect, Size};
use crate::image::Image;
use crate::te::Assets;
use crate::tools::platform::Platform;
use crate::ui::view::View;
use crate::ui::ImageView;
use std::rc::Rc;
use tools::refs::Shared;

pub struct UIDrawer {
    pub assets: Rc<Assets>,
    pub window_size: Size,
}

impl UIDrawer {
    pub fn new(assets: Rc<Assets>) -> UIDrawer {
        UIDrawer {
            assets,
            window_size: Size::new(),
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        self.window_size = size.clone();
    }
}

impl UIDrawer {
    pub fn draw_view(&self, view: Shared<dyn View>) {
        if let Some(image_view) = view.borrow_mut().as_any().downcast_ref::<ImageView>() {
            self.draw_image_in_rect(
                &image_view.image,
                image_view.absolute_frame(),
                &image_view.color(),
            );
        }

        self.fill_rect(view.borrow().absolute_frame(), &view.borrow().color());

        self.draw_rect(view.borrow().absolute_frame(), &Color::TURQUOISE);

        for view in view.borrow_mut().subviews_mut() {
            self.draw_view(view.clone())
        }
    }
}

impl UIDrawer {
    fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = if Platform::MAC { 2.0 } else { 1.0 };
        GLWrapper::set_viewport(self.window_size.height, &SCALE, rect);
    }
}

impl UIDrawer {
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
        if image.invalid() {
            return;
        }

        if rect.size.is_negative() {
            return;
        }

        if image.is_monochrome() {
            self.assets.shaders.ui_monochrome.enable();
            self.assets.shaders.ui_monochrome.set_color(color);
        } else {
            self.assets.shaders.ui_texture.enable();
        }

        self.set_viewport(rect);
        image.bind();
        self.assets.buffers.fullscreen_image.draw();
    }
}
