use std::fmt::Display;

use refs::Weak;
use ui_proc::view;
use window::image::{Image, ToImage};

use crate::{NineSegmentImageView, ViewData, ViewFrame, ViewSubviews};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

pub enum ImageMode {}

#[view]
pub struct ImageView {
    image: Weak<Image>,

    nine_segment: Weak<NineSegmentImageView>,

    pub flip_x: bool,
    pub flip_y: bool,
}

impl ImageView {
    pub fn image(&self) -> Weak<Image> {
        self.image
    }

    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image = image.to_image();
        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        if !self.nine_segment.was_initialized() {
            self.nine_segment = self.add_view();
            self.nine_segment.place().back();
            self.nine_segment
                .subviews_mut()
                .iter_mut()
                .for_each(|v| v.base_view_mut().z_position = self.z_position());
        }

        self.nine_segment.set_image(name);

        self
    }
}
