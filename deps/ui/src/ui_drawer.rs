use std::ops::DerefMut;

use gm::{
    flat::{Rect, Size},
    Color,
};
use rtools::{address::Address, Rglica};

use crate::{view::ViewSubviews, DrawMode, PathData, UIAnimation, View};

static mut DRAWER: Option<Box<dyn UIDrawer>> = Option::None;

pub trait UIDrawer {
    fn reset_viewport(&self);
    fn fill(&self, rect: &Rect, color: &Color);
    fn outline(&self, rect: &Rect, color: &Color);
    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>);
    fn draw_round_border(&self, view: &mut dyn View);
    fn set_screen_scale(&mut self, scale: f32);
    fn set_scale(&mut self, scale: f32);
    fn set_size(&mut self, size: Size);
    fn update(&self, view: &mut dyn View);
    fn draw(&self, view: &mut dyn View);
    fn rglica(&self) -> Rglica<dyn UIDrawer>;
    fn window_size(&self) -> &Size;

    fn root_view(&mut self) -> &mut Box<dyn View>;

    fn next_view(&mut self) -> Option<Box<dyn View>>;
    fn set_next_view(&mut self, view: Box<dyn View>);

    fn animations(&mut self) -> &mut Vec<UIAnimation>;

    fn replace_view(&mut self, view: Rglica<dyn View>);
    fn view_to_replace(&self) -> Rglica<dyn View>;

    fn replace_scheduled(&mut self) {
        let mut view = self.view_to_replace();
        if view.is_null() {
            return;
        }

        let mut superview = view.superview();

        let index = view.subviews().iter().position(|sub| view.address() == sub.address()).unwrap();
        view.superview = Default::default();
        let view = superview.remove_subview_at(index);

        *self.root_view() = view;
    }

    fn views_to_remove(&mut self) -> &mut Vec<Rglica<dyn View>>;

    fn schedule_remove(&mut self, view: Rglica<dyn View>) {
        self.views_to_remove().push(view)
    }

    fn remove_scheduled(&mut self) {
        if self.views_to_remove().is_empty() {
            return;
        }
        let to_remove = self.views_to_remove().drain(..);
        for view in to_remove {
            let index = view
                .superview()
                .subviews()
                .iter()
                .position(|sub| view.address() == sub.address())
                .unwrap();
            view.superview().remove_subview_at(index);
        }
    }
}

pub fn set_ui_drawer(drawer: Box<dyn UIDrawer>) {
    unsafe { DRAWER = drawer.into() }
}

pub fn get_ui_drawer() -> &'static mut dyn UIDrawer {
    unsafe { DRAWER.as_mut().unwrap().deref_mut() }
}
