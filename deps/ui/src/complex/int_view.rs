use std::ops::AddAssign;

use gl_image::Image;
use rtools::{data_manager::Handle, Event, Rglica};

use crate::{
    basic::Button,
    impl_view, view,
    view::{ViewData, ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct IntView {
    value:         i64,
    label:         Rglica<Label>,
    up:            Rglica<Button>,
    down:          Rglica<Button>,
    pub on_change: Event<i64>,
}

impl_view!(IntView);

impl IntView {
    pub fn set_images(&mut self, up: Handle<Image>, down: Handle<Image>) {
        self.up.set_image(up);
        self.down.set_image(down);
    }
}

impl ViewCallbacks for IntView {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.up = self.add_view();
        self.down = self.add_view();

        self.up.on_tap.set(self, |this, _| {
            this.value.add_assign(1);
            let val = this.value;
            this.on_change.trigger(val);
        });

        self.down.on_tap.set(self, |this, _| {
            this.value.add_assign(-1);
            let val = this.value;
            this.on_change.trigger(val);
        });
    }

    fn layout(&mut self) {
        self.place().all_vertically();
        self.label.set_text(&self.value.to_string());
    }
}
