use std::{ops::DerefMut, rc::Rc};

use gl_image::Image;
use gl_wrapper::GLWrapper;
use gm::{Color, Rect, Size};
use ui::{complex::PathData, View};

use crate::assets::Assets;

#[derive(Default)]
pub struct UIDrawer {
    pub assets:      Rc<Assets>,
    pub window_size: Size,
    scale:           f32,
}

impl UIDrawer {
    pub fn new(assets: Rc<Assets>) -> UIDrawer {
        UIDrawer {
            assets,
            window_size: Default::default(),
            scale: 1.0,
        }
    }

    #[cfg(windows)]
    pub fn set_scale(&mut self, _scale: f32) {
        self.scale = 1.0
    }

    #[cfg(unix)]
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    pub fn set_size(&mut self, size: Size) {
        self.window_size = size
    }
}

impl UIDrawer {
    pub fn draw(&self, view: &mut dyn View) {

        view.update();

        if let Some(image) = view.image() {
            self.draw_image_in_rect(&image, view.absolute_frame(), view.color());
        }

        self.fill_rect(view.absolute_frame(), view.color());

        self.draw_rect(view.absolute_frame(), &Color::TURQUOISE);

        if let Some(paths) = view.paths() {
            for path in paths {
                self.draw_path_in_rect(path, view.absolute_frame());
            }
        }

        for view in view.subviews_mut() {
            self.draw(view.deref_mut())
        }
    }
}

impl UIDrawer {
    pub fn reset_viewport(&self) {
        self.set_viewport(&self.window_size.into());
    }

    fn set_viewport(&self, rect: &Rect) {
        GLWrapper::set_viewport(self.window_size.height, &self.scale, rect);
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

    fn draw_image_in_rect(&self, image: &Image, rect: &Rect, color: &Color) {
        if image.is_invalid() {
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

impl UIDrawer {
    pub fn draw_path_in_rect(&self, path: &PathData, rect: &Rect) {
        self.set_viewport(rect);
        self.assets.shaders.ui_path.enable();
        self.assets.shaders.ui_path.set_color(&path.color);
        self.assets.shaders.ui_path.set_size(&rect.size);
        path.buffer.draw();
    }
}
