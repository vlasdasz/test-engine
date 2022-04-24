#![allow(clippy::mismatched_target_os)]

use std::{borrow::Borrow, ops::DerefMut, rc::Rc};

use gl_image::Image;
use gl_wrapper::GLWrapper;
use gm::{
    flat::{Rect, Size},
    Color,
};
use ui::{complex::PathData, View, ViewData, ViewFrame, ViewSubviews};

use crate::assets::Assets;

pub struct UIDrawer {
    pub assets:      Rc<Assets>,
    pub window_size: Size,

    scale:        f32,
    screen_scale: f32,
}

impl UIDrawer {
    pub fn new(assets: Rc<Assets>) -> UIDrawer {
        UIDrawer {
            assets,
            window_size: Default::default(),
            scale: 1.0,
            screen_scale: 1.0,
        }
    }

    #[cfg(any(windows, linux))]
    pub fn set_screen_scale(&mut self, _scale: f32) {
        self.screen_scale = 1.0
    }

    #[cfg(macos)]
    pub fn set_screen_scale(&mut self, scale: f32) {
        self.screen_scale = scale
    }

    #[cfg(mobile)]
    pub fn set_screen_scale(&mut self, scale: f32) {
        self.screen_scale = scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    pub fn set_size(&mut self, size: Size) {
        self.window_size = size
    }
}

impl UIDrawer {
    pub fn update(&self, view: &mut dyn View) {
        view.update();
        for view in view.subviews_mut() {
            self.update(view.deref_mut());
        }
    }

    pub fn draw(&self, view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }

        if let Some(image) = view.image().get() {
            self.draw_image_in_rect(image, view.absolute_frame(), view.color());
        }

        self.fill_rect(view.absolute_frame(), view.color());

        self.draw_rect(view.absolute_frame(), Color::TURQUOISE);

        for path in view.paths() {
            self.draw_path_in_rect(path, view.absolute_frame());
        }

        for view in view.subviews_mut() {
            self.draw(view.deref_mut())
        }
    }
}

impl UIDrawer {
    pub fn reset_viewport(&self) {
        GLWrapper::set_viewport(
            self.window_size.height,
            self.screen_scale,
            &self.window_size.into(),
        );
    }

    fn set_viewport(&self, rect: impl Borrow<Rect>) {
        GLWrapper::set_viewport(
            self.window_size.height,
            self.screen_scale,
            rect.borrow() * self.scale,
        );
    }
}

impl UIDrawer {
    fn fill_rect(&self, rect: &Rect, color: Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable().set_color(color);
        self.assets.buffers.fullscreen.draw();
    }

    fn draw_rect(&self, rect: &Rect, color: Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable().set_color(color);
        self.assets.buffers.fullscreen_outline.draw();
    }

    fn draw_image_in_rect(&self, image: &Image, rect: &Rect, color: Color) {
        debug_assert!(rect.size.is_valid());
        debug_assert!(image.is_valid());

        if image.is_monochrome() {
            self.assets.shaders.ui_monochrome.enable().set_color(color);
        } else {
            self.assets.shaders.ui_texture.enable();
        }

        self.set_viewport(&image.size.fit_in(rect));
        image.bind();
        self.assets.buffers.fullscreen_image.draw();
    }
}

impl UIDrawer {
    pub fn draw_path_in_rect(&self, path: &PathData, rect: &Rect) {
        debug_assert!(rect.size.is_valid());
        self.set_viewport(rect);
        self.assets
            .shaders
            .ui_path
            .enable()
            .set_color(path.color)
            .set_size(rect.size);
        path.buffer.draw();
    }
}
