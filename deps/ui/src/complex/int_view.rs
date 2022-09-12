use gl_image::Image;
use rtools::{data_manager::Handle, Event, Rglica, ToRglica};
use smart_default::SmartDefault;

use crate::{
    basic::Button,
    view,
    view::{ViewData, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(SmartDefault, Debug)]
pub struct IntView {
    #[default = 1.0]
    value: f32,
    label: Rglica<Label>,
    up:    Rglica<Button>,
    down:  Rglica<Button>,

    pub on_change: Event<f32>,
    #[default = 1.0]
    pub step:      f32,
}

impl IntView {
    pub fn set_images(&mut self, up: Handle<Image>, down: Handle<Image>) {
        self.up.set_image(up);
        self.down.set_image(down);
    }
}

impl ViewCallbacks for IntView {
    fn setup(&mut self) {
        self.place().all_ver();

        (self.label, self.up, self.down) = (self.add_view(), self.add_view(), self.add_view());

        self.label.set_text("1.0");

        self.up.on_tap.set(self, |this, _| {
            this.value += this.step;
            this.on_change.trigger(this.value);
            this.label.set_text(format!("{:.1}", this.value));
        });

        self.down.on_tap.set(self, |this, _| {
            this.value -= this.step;
            this.on_change.trigger(this.value);
            this.label.set_text(format!("{:.1}", this.value));
        });
    }
}
