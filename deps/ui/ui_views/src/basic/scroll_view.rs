use gm::flat::Size;
use refs::Weak;
use ui::{view, SubView, ViewCallbacks, ViewFrame, ViewSetup};

use crate::Slider;

#[view]
pub struct ScrollView {
    slider:           SubView<Slider>,
    pub content_size: Size,
}

impl ViewSetup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.slider.set_range(-1000, 0);
        self.slider.place.w(50).r(0);
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = self.content_offset;
        self.slider.frame_mut().origin.y = -co.y;
        let range = self.content_size.height - self.height();
        self.slider.frame_mut().size.height = self.height();
        self.slider.set_range(-range, 0);
        self.content_offset.y = self.slider.value;
        self.slider.is_hidden = dbg!(self.height()) >= dbg!(self.content_size.height);
        dbg!(self.slider.is_hidden);
    }

    fn content_size(&self) -> &Size {
        &self.content_size
    }
}
