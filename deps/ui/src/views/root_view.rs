use gm::color::Color;
use refs::Weak;
use ui_proc::view;
use window::image::ToImage;

use crate::{ImageView, UIManager, View, ViewData, ViewSubviews, view::Setup};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct RootView {
    background: Weak<ImageView>,
}

impl RootView {
    pub fn setup_root(&mut self) {
        let image = ImageView::new();
        self.background = self.__add_subview_internal(image, true).downcast_view::<ImageView>().unwrap();
        self.background.place().back();
    }

    pub fn clear_root(&mut self) {
        UIManager::get()
            .deleted_views
            .lock()
            .unwrap()
            .extend(self.base_view_mut().subviews.drain(1..));
    }

    pub fn set_color(mut self: Weak<Self>, color: impl Into<Color>) -> Weak<Self> {
        self.background.set_color(color);
        self
    }

    pub fn set_image(mut self: Weak<Self>, image: impl ToImage) -> Weak<Self> {
        self.background.set_image(image);
        self
    }
}
