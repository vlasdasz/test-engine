use std::{cell::RefCell, collections::HashMap, ops::DerefMut};

use gl_image::draw_image;
use gl_wrapper::{buffers::Buffers, GLWrapper};
use gm::{
    flat::{PointsPath, Rect, Size},
    Color,
};
use ui::{refs::Address, DrawMode, PathData, UIDrawer, UIManager, View, ViewData, ViewFrame, ViewSubviews};
use ui_views::initialize_path_data;

use crate::assets::Assets;

type RoundStorage = HashMap<usize, (PathData, Size)>;

#[derive(Default)]
pub struct TEUIDrawer {
    round_storage: RefCell<RoundStorage>,
}

impl TEUIDrawer {
    pub fn set_viewport(&self, rect: &Rect) {
        GLWrapper::set_viewport(UIManager::rescale_frame(rect));
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

    fn draw_round_border(&self, view: &mut dyn View) {
        let mut storage = self.round_storage.borrow_mut();
        let path = self.rounded_path_for_view(view, &mut storage);
        self.draw_path(path, view.absolute_frame(), None);
    }
}

impl UIDrawer for TEUIDrawer {
    fn fill(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        Assets::get().shaders.ui.enable().set_color(color);
        Buffers::get().full.draw();
    }

    fn outline(&self, rect: &Rect, color: &Color) {
        self.set_viewport(rect);
        Assets::get().shaders.ui.enable().set_color(color);
        Buffers::get().full_outline.draw();
    }

    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>) {
        if rect.size.is_invalid() {
            return;
        }
        self.set_viewport(rect);
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

    fn draw(&self, view: &mut dyn View) {
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
            let frame = &image.size.fit_in(view.absolute_frame());
            draw_image(image, &UIManager::rescale_frame(frame), view.color());
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
}

fn make_round_border(view: &dyn View) -> PathData {
    initialize_path_data(
        PointsPath::rounded_rect(view.frame().size, view.corner_radius(), 5),
        view.border_color(),
        DrawMode::Outline,
    )
}
