use gm::Color;
use rtools::{Event, Rglica, ToRglica};

use crate::{
    view, view::ViewSubviews, Label, SubView, Touch, View, ViewBase, ViewCallbacks, ViewLayout, ViewTouch,
};

#[view]
#[derive(Default)]
pub struct Button {
    label: SubView<Label>,

    pub on_tap: Event,
}

impl Button {
    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        self.label.place().as_background();
        self.label.set_text(text);
        self
    }

    pub fn set_text_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.label.set_text_color(color);
        self
    }
}

impl ViewCallbacks for Button {
    fn setup(&mut self) {
        self.enable_touch()
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.on_tap.trigger(())
        }
    }
}
