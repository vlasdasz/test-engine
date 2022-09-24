#![allow(clippy::mismatched_target_os)]

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, ops::DerefMut};

use gl_image::draw_image;
use gl_wrapper::{buffers::Buffers, GLWrapper};
use gm::{
    flat::{PointsPath, Rect, Size},
    Color,
};
use rtools::{address::Address, Rglica, ToRglica};
use smart_default::SmartDefault;
use ui::{
    BaseView, DrawMode, PathData, UIAnimation, UIDrawer, View, ViewAnimation, ViewData, ViewFrame,
    ViewSubviews,
};
use ui_views::initialize_path_data;

use crate::assets::Assets;

type RoundStorage = HashMap<u64, (PathData, Size)>;

#[derive(SmartDefault)]
pub struct TEUIDrawer {
    round_storage: RefCell<RoundStorage>,

    #[default = 1.0]
    scale:        f32,
    #[default = 1.0]
    screen_scale: f32,

    #[default(Box::<BaseView>::default())]
    root_view: Box<dyn View>,

    next_view: Option<Box<dyn View>>,

    views_to_remove: Vec<Rglica<dyn View>>,

    animations: Vec<UIAnimation>,

    open_keyboard:  bool,
    close_keyboard: bool,
    touch_disabled: bool,
}

impl TEUIDrawer {
    pub fn convert_viewport(&self, rect: impl Borrow<Rect>) -> Rect {
        let scale = self.screen_scale;
        let rect = rect.borrow() * self.scale;

        (
            rect.origin.x * scale,
            (self.window_size().height - rect.origin.y - rect.size.height) * scale,
            rect.size.width * scale,
            rect.size.height * scale,
        )
            .into()
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
    fn next_view(&mut self) -> &mut Option<Box<dyn View>> {
        &mut self.next_view
    }

    fn animations(&mut self) -> &mut Vec<UIAnimation> {
        &mut self.animations
    }

    fn reset_viewport(&self) {
        GLWrapper::set_viewport((
            0,
            0,
            self.window_size().width * self.screen_scale,
            self.window_size().height * self.screen_scale,
        ));
    }

    fn fill(&self, rect: &Rect, color: &Color) {
        GLWrapper::set_viewport(self.convert_viewport(rect));
        Assets::get().shaders.ui.enable().set_color(color);
        Buffers::get().full.draw();
    }

    fn outline(&self, rect: &Rect, color: &Color) {
        GLWrapper::set_viewport(self.convert_viewport(rect));
        Assets::get().shaders.ui.enable().set_color(color);
        Buffers::get().full_outline.draw();
    }

    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>) {
        if rect.size.is_invalid() {
            return;
        }
        GLWrapper::set_viewport(self.convert_viewport(rect));
        Assets::get()
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

    #[cfg(any(windows, linux))]
    fn set_screen_scale(&mut self, _scale: f32) {
        self.screen_scale = 1.0
    }

    #[cfg(macos)]
    fn set_screen_scale(&mut self, scale: f32) {
        self.screen_scale = scale
    }

    #[cfg(mobile)]
    fn set_screen_scale(&mut self, scale: f32) {
        self.screen_scale = scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    fn update(&self, view: &mut dyn View) {
        view.update();
        view.commit_animations();
        for view in view.subviews_mut() {
            self.update(view.deref_mut());
        }
    }

    fn draw(&self, view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }

        let needs_stensil = view.corner_radius() > 0.0;

        if needs_stensil {
            GLWrapper::set_viewport(self.convert_viewport(view.frame()));

            GLWrapper::start_stensil();

            let mut storage = self.round_storage.borrow_mut();
            let path = self.rounded_path_for_view(view, &mut storage);
            self.draw_path(path, view.absolute_frame(), DrawMode::Fill.into());

            GLWrapper::draw_stensiled();
        }

        self.fill(view.absolute_frame(), view.color());

        if let Some(image) = view.image().get() {
            let frame = &image.size.fit_in(view.absolute_frame());
            draw_image(image, &self.convert_viewport(frame), view.color());
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

        //MARK - Debug frames
        self.outline(view.absolute_frame(), &Color::TURQUOISE);

        for view in view.subviews_mut() {
            self.draw(view.deref_mut())
        }

        GLWrapper::disable_stensil();
    }

    fn rglica(&self) -> Rglica<dyn UIDrawer> {
        (self as &dyn UIDrawer).to_rglica()
    }

    fn window_size(&self) -> &Size {
        &self.root_view.frame().size
    }

    fn views_to_remove(&mut self) -> &mut Vec<Rglica<dyn View>> {
        &mut self.views_to_remove
    }

    fn root_view(&mut self) -> &mut dyn View {
        self.root_view.deref_mut()
    }

    fn open_keyboard(&mut self) -> &mut bool {
        &mut self.open_keyboard
    }

    fn close_keyboard(&mut self) -> &mut bool {
        &mut self.close_keyboard
    }

    fn touch_disabled(&mut self) -> &mut bool {
        &mut self.touch_disabled
    }
}

fn make_round_border(view: &dyn View) -> PathData {
    initialize_path_data(
        PointsPath::rounded_rect(view.frame().size, view.corner_radius(), 5),
        view.border_color(),
        DrawMode::Outline,
    )
}
