use gm::Color;
use rtools::{Event, Rglica, ToRglica};

use crate::{
    impl_view, view, view::ViewSubviews, Label, View, ViewBase, ViewCallbacks, ViewLayout, ViewTouch,
};

#[view]
#[derive(Default, Debug)]
pub struct Button {
    label: Rglica<Label>,

    pub on_tap: Event,
}
impl_view!(Button);

impl Button {
    pub fn set_text(&mut self, text: impl ToString) -> &mut Self {
        if self.label.is_null() {
            self.label = self.add_view();
            self.label.place().as_background();
        }
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
        self.on_touch().set(self, |this, touch| {
            if touch.is_began() {
                this.on_tap.trigger(())
            }
        });
    }
}
