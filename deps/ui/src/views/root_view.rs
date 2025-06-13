use gm::{
    Platform,
    color::Color,
    flat::{Point, Size},
};
use refs::Weak;
use ui_proc::view;
use window::image::ToImage;

use crate::{ImageView, UIManager, View, ViewData, ViewFrame, ViewSubviews, view::Setup};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct RootView {
    inner_position: Point,
    size:           Size,

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

    pub fn resize_root(mut self: Weak<Self>, inner_position: Point, size: Size, scale: f32) {
        self.inner_position = inner_position;
        self.size = size;

        self.set_size(size.width * (1.0 / scale), size.height * (1.0 / scale));

        if Platform::IOS {
            self.set_position(inner_position);
        }
    }

    pub fn rescale_root(self: Weak<Self>, scale: f32) {
        self.resize_root(self.inner_position, self.size, scale);
    }
}
