use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::Deref,
};

use gl_image::draw_image;
use gl_wrapper::{buffers::Buffers, GLWrapper};
use gm::{
    axis::Axis,
    flat::{PointsPath, Rect, Size},
    Color,
};
use ui::{
    refs::Address, DrawMode, PathData, UIDrawer, UIManager, UIShaders, View, ViewData, ViewFrame,
    ViewSubviews,
};
use ui_views::{initialize_path_data, DrawingView, ImageView};

type RoundStorage = HashMap<usize, (PathData, Size)>;

#[derive(Default)]
pub struct TEUIDrawer {
    root_frame:    Rect,
    round_storage: RefCell<RoundStorage>,
}

impl TEUIDrawer {
    pub fn set_viewport(&self, rect: &Rect) {
        GLWrapper::set_viewport(UIManager::rescale_frame(rect));
    }
}

impl TEUIDrawer {
    fn rounded_path_for_view<'a>(view: &dyn View, storage: &'a mut RoundStorage) -> &'a PathData {
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

    fn draw_round_border(&self, view: &dyn View, priority: usize) {
        let mut storage = self.round_storage.borrow_mut();
        let path = Self::rounded_path_for_view(view, &mut storage);
        self.draw_path(path, view.absolute_frame(), None, priority);
    }
}

impl UIDrawer for TEUIDrawer {
    fn fill(&self, rect: &Rect, color: &Color, priority: usize) {
        self.set_viewport(rect);
        UIShaders::view().enable().set_color(color).set_priority(priority);
        Buffers::get().full.draw();
    }

    fn outline(&self, rect: &Rect, color: &Color, priority: usize) {
        self.set_viewport(rect);
        UIShaders::view().enable().set_color(color).set_priority(priority);
        Buffers::get().full_outline.draw();
    }

    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>, priority: usize) {
        if rect.size.is_invalid() {
            return;
        }
        self.set_viewport(rect);
        UIShaders::path()
            .enable()
            .set_color(&path.color)
            .set_size(rect.size)
            .set_priority(priority);
        if let Some(mode) = custom_mode {
            path.buffer.draw_with_mode(mode.to_gl())
        } else {
            path.buffer.draw();
        }
    }

    fn draw(&self, view: &dyn View) {
        if view.is_hidden {
            return;
        }

        if view.frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label,
                view.frame()
            );
            return;
        }

        let needs_stensil = view.corner_radius() > 0.0;

        if needs_stensil {
            self.set_viewport(view.frame());

            GLWrapper::start_stensil();

            let mut storage = self.round_storage.borrow_mut();
            let path = Self::rounded_path_for_view(view, &mut storage);
            self.draw_path(
                path,
                view.absolute_frame(),
                DrawMode::Fill.into(),
                view.base().priority,
            );

            GLWrapper::draw_stensiled();
        }

        self.fill(view.absolute_frame(), view.color(), view.priority);

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if let Some(image) = image_view.image.get() {
                let frame = &image.size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                draw_image(
                    image,
                    &UIManager::rescale_frame(frame),
                    &image_view.tint_color,
                    view.priority,
                    false,
                );
            }
        }

        if view.border_color().is_visible() {
            if needs_stensil {
                self.draw_round_border(view, view.priority);
            } else {
                self.outline(view.absolute_frame(), view.border_color(), view.priority);
            }
        }

        if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for path in drawing_view.paths() {
                self.draw_path(path, view.absolute_frame(), None, view.priority);
            }
        }

        //MARK - Debug frames
        self.outline(view.absolute_frame(), &Color::TURQUOISE, view.priority);

        for view in view.subviews() {
            if view.dont_hide || view.absolute_frame().intersects(&self.root_frame) {
                self.draw(view.deref())
            }
        }

        GLWrapper::disable_stensil();
    }

    fn set_root_frame(&mut self, frame: Rect) {
        self.root_frame = frame;
    }
}

impl Debug for TEUIDrawer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        "TEUIDrawer".fmt(f)
    }
}

fn make_round_border(view: &dyn View) -> PathData {
    initialize_path_data(
        PointsPath::rounded_rect(view.frame().size, view.corner_radius(), 5),
        view.border_color(),
        DrawMode::Outline,
    )
}
