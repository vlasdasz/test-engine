use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Rect, Size};
use crate::image::Image;
use crate::te::Assets;
use crate::tools::platform::Platform;
use crate::ui::view::View;
use crate::ui::ImageView;
use std::ops::DerefMut;

pub struct TEUIDrawer {
    pub assets: Assets,
    pub window_size: Size,
}

impl TEUIDrawer {
    pub fn new(assets: Assets) -> TEUIDrawer {
        TEUIDrawer {
            assets,
            window_size: Size::new(),
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        self.window_size = size.clone();
    }
}

impl TEUIDrawer {
    pub fn draw_view(&self, view: &mut dyn View) {
        view.view_mut().calculate_absolute_frame();

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            self.draw_image_in_rect(
                &image_view.image,
                image_view.absolute_frame(),
                &image_view.color,
            );
        } else {
            self.draw_rect(view.view().absolute_frame(), &view.view().color);
        }

        for view in view.view().subviews() {
            self.draw_view(view.try_borrow_mut().unwrap().deref_mut());
        }
    }
}

impl TEUIDrawer {
    fn set_viewport(&self, rect: &Rect) {
        const SCALE: f32 = if Platform::MAC { 2.0 } else { 1.0 };
        GLWrapper::set_viewport(self.window_size.height, &SCALE, rect);
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
