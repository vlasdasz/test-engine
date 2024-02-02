use gm::flat::Size;
use refs::Weak;
use ui::{view, SubView, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSetup};

use crate::Slider;

#[view]
pub struct ScrollView {
    slider:           SubView<Slider>,
    pub content_size: Size,
}

impl ViewSetup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.dont_hide = true;
        self.slider.set_range(-1000, 0);
        self.slider.place().w(40).r(0);
        self.slider.on_change.val(move |val| {
            self.content_offset.y = val;
        });

        UIManager::on_scroll(self, move |scroll| {
            self.content_offset.y += scroll.y;
        });
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = self.content_offset;
        self.slider.frame_mut().origin.y = -co.y;
        let range = self.content_size.height - self.height();
        self.slider.frame_mut().size.height = self.height();
        self.slider.set_range(-range, 0);
        let hidden = self.height() >= self.content_size.height;
        self.slider.set_hidden(hidden);
    }

    fn content_size(&self) -> &Size {
        &self.content_size
    }
}
