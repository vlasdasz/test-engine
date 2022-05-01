use rtools::{Event, Rglica, ToRglica};

use crate::{
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks, ViewTouch,
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
        }
        self.label.set_text(text);
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

    fn layout(&mut self) {
        if self.label.is_ok() {
            self.label.place().as_background()
        }
    }
}
