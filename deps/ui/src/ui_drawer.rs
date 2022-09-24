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
    fn update(&self, view: &mut dyn View);
    fn draw(&self, view: &mut dyn View);
    fn rglica(&self) -> Rglica<dyn UIDrawer>;
    fn window_size(&self) -> &Size;

    fn touch_disabled(&mut self) -> &mut bool;

    fn open_keyboard(&mut self) -> &mut bool;
    fn close_keyboard(&mut self) -> &mut bool;

    fn root_view(&mut self) -> &mut dyn View;
    fn next_view(&mut self) -> &mut Option<Box<dyn View>>;

    fn animations(&mut self) -> &mut Vec<UIAnimation>;

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

    fn set_scheduled(&mut self) {
        let Some(mut view) = self.next_view().take() else {
            return;
        };
        self.root_view().remove_all_subviews();
        view.frame = self.root_view().frame;
        view.place.as_background();
        self.root_view().add_subview(view);
    }

    fn set_view(&mut self, view: Box<dyn View>) {
        self.next_view().replace(view);
    }
}

pub fn set_ui_drawer(drawer: Box<dyn UIDrawer>) {
    unsafe { DRAWER = drawer.into() }
}

pub fn get_ui_drawer() -> &'static mut dyn UIDrawer {
    unsafe { DRAWER.as_mut().unwrap().deref_mut() }
}
