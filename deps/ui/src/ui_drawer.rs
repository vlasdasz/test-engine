use std::ops::DerefMut;

use gl_wrapper::GLWrapper;
use gm::{flat::Rect, Color};

use crate::{
    view::{ViewAnimation, ViewData, ViewSubviews},
    DrawMode, PathData, UIManager, View,
};

pub trait UIDrawer {
    fn fill(&self, rect: &Rect, color: &Color);
    fn outline(&self, rect: &Rect, color: &Color);
    fn draw_path(&self, path: &PathData, rect: &Rect, custom_mode: Option<DrawMode>);
    fn draw(&self, view: &mut dyn View);

    fn update(&self, view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.update();
        view.commit_animations();
        for view in view.subviews_mut() {
            self.update((*view).deref_mut());
        }
    }

    fn reset_viewport(&self) {
        GLWrapper::set_viewport((
            0,
            0,
            UIManager::window_size().width * UIManager::display_scale(),
            UIManager::window_size().height * UIManager::display_scale(),
        ));
    }
}
