#![allow(clippy::mismatched_target_os)]

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, ops::DerefMut, rc::Rc};

use gl_image::Image;
use gl_wrapper::GLWrapper;
use gm::{
    flat::{PointsPath, Rect, Size},
    Color,
};
use rtools::{address::Address, Rglica, ToRglica};
use ui::{
    complex::{initialize_path_data, DrawMode, PathData},
    UIDrawer, View, ViewData, ViewFrame, ViewSubviews,
};

use crate::assets::Assets;

type RoundStorage = HashMap<u64, (PathData, Size)>;

pub struct TEUIDrawer {
    pub assets:      Rc<Assets>,
    pub window_size: Size,

    round_storage: RefCell<RoundStorage>,

    scale:        f32,
    screen_scale: f32,
}

impl TEUIDrawer {
    pub fn new(assets: Rc<Assets>) -> TEUIDrawer {
        TEUIDrawer {
            assets,
            window_size: Default::default(),
            round_storage: Default::default(),
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

impl TEUIDrawer {
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

        let needs_stensil = view.corner_radius() > 0.0;

        if needs_stensil {
            self.set_viewport(view.frame());

            GLWrapper::start_stensil();

            let mut storage = self.round_storage.borrow_mut();
            let path = self.rounded_path_for_view(view, &mut storage);
            self.draw_path(path, view.absolute_frame(), DrawMode::Fill.into());

            GLWrapper::draw_stensiled();
        }

        self.fill(view.absolute_frame(), view.color());

        if let Some(image) = view.image().get() {
            self.draw_image(image, view.absolute_frame(), view.color(), false);
        }

        if view.border_color().is_visible() {
            if needs_stensil {
                self.draw_round_border(view);
            } else {
                self.outline(view.absolute_frame(), view.border_color());
            }
        }

        for path in view.paths() {
            self.draw_path(path, view.absolute_frame(), None);
        }

        for view in view.subviews_mut() {
            self.draw(view.deref_mut())
        }

        GLWrapper::disable_stensil();
    }
}

impl TEUIDrawer {
    fn set_viewport(&self, rect: impl Borrow<Rect>) {
        GLWrapper::set_ui_viewport(
            self.window_size.height,
            self.screen_scale,
            rect.borrow() * self.scale,
        );
    }
}

impl TEUIDrawer {
    fn rounded_path_for_view<'a>(&'a self, view: &dyn View, storage: &'a mut RoundStorage) -> &'a PathData {
        if storage.get(&view.address()).is_some() {
            let (path, prev_size) = storage.get_mut(&view.address()).unwrap();
            if *prev_size == view.frame().size {
                return path;
            }
            *path = make_round_border(view);
            *prev_size = view.frame().size;
            return path;
        }

        let path = make_round_border(view);
        storage.insert(view.address(), (path, view.frame().size));
        &storage.get(&view.address()).unwrap().0
    }
}

impl UIDrawer for TEUIDrawer {
    fn reset_viewport(&self) {
        GLWrapper::set_ui_viewport(self.window_size.height, self.screen_scale, self.window_size);
    }

    fn fill(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable().set_color(color);
        self.assets.buffers.full.draw();
    }

    fn outline(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        self.assets.shaders.ui.enable().set_color(color);
        self.assets.buffers.full_outline.draw();
    }

    fn draw_image(&self, image: &Image, rect: &Rect, color: &Color, raw_frame: bool) {
        // debug_assert!(rect.size.is_valid());
        // debug_assert!(image.is_valid());

        if image.is_monochrome() {
            self.assets.shaders.ui_monochrome.enable().set_color(color)
        } else {
            self.assets.shaders.ui_texture.enable()
        }
        .set_flipped(image.flipped)
        .set_flipped_y(image.flipped_y);

        if raw_frame {
            GLWrapper::set_viewport(*rect);
        } else {
            self.set_viewport(&image.size.fit_in(rect));
        }
        image.bind();
        self.assets.buffers.full_image.draw();
    }

    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>) {
        if rect.size.is_invalid() {
            return;
        }
        self.set_viewport(rect);
        self.assets
            .shaders
            .ui_path
            .enable()
            .set_color(&path.color)
            .set_size(rect.size);
        if let Some(mode) = custom_mode {
            path.buffer.draw_with_mode(mode.to_gl())
        } else {
            path.buffer.draw();
        }
    }

    fn draw_round_border(&self, view: &mut dyn View) {
        let mut storage = self.round_storage.borrow_mut();
        let path = self.rounded_path_for_view(view, &mut storage);
        self.draw_path(path, view.absolute_frame(), None);
    }

    fn rglica(&self) -> Rglica<dyn UIDrawer> {
        (self as &dyn UIDrawer).to_rglica()
    }
}

fn make_round_border(view: &dyn View) -> PathData {
    initialize_path_data(
        PointsPath::rounded_rect(view.frame().size, view.corner_radius(), 5),
        view.border_color(),
        DrawMode::Outline,
    )
}
