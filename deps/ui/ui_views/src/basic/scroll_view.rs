use gm::flat::Size;
use refs::Weak;
use ui::{view, Sub, UIManager, ViewCallbacks, ViewData, ViewFrame, ViewSetup, ViewSubviews};
use vents::Event;
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::Slider;

#[view]
pub struct ScrollView {
    slider:           Sub<Slider>,
    pub content_size: Size,
    pub on_scroll:    Event<f32>,
}

impl ScrollView {
    pub fn remove_all_subviews(&mut self) {
        let slider_addr = self.slider.addr();

        for mut view in self.subviews_mut() {
            if view.addr() == slider_addr {
                continue;
            }

            view.remove_from_superview();
        }
    }
}

impl ViewSetup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.dont_hide = true;
        self.slider.place().w(40).r(0);
        self.slider.on_change.val(move |val| {
            let val = 1.0 - val;
            let range = self.content_size.height - self.height();
            self.content_offset.y = -range * val;
            self.on_scroll.trigger(self.content_offset.y);
        });

        UIManager::on_scroll(self, move |scroll| {
            self.on_scroll(scroll.y);
        });

        self.size_changed().sub(move || {
            self.on_scroll(0.0);
        })
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = self.content_offset;
        self.slider.set_y(-co.y);
        let height = self.height();
        self.slider.set_height(height);
        let hidden = self.height() >= self.content_size.height;
        self.slider.set_hidden(hidden);
    }

    fn content_size(&self) -> &Size {
        &self.content_size
    }
}

impl ScrollView {
    fn on_scroll(mut self: Weak<Self>, scroll: f32) {
        if self.height() >= self.content_size.height {
            return;
        }
        self.content_offset.y += scroll;
        let range = self.content_size.height - self.height();
        self.content_offset.y = self.content_offset.y.clamp(-range, 0.0);
        let slider_val = -self.content_offset.y / range;
        self.slider.set_value_without_event(1.0 - slider_val);

        self.on_scroll.trigger(self.content_offset.y);
    }
}
