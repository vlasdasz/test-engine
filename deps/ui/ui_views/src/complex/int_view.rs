use gl_image::Image;
use refs::Weak;
use rtools::data_manager::Handle;
use smart_default::SmartDefault;
use ui::{view, Event, SubView, ViewData, ViewSetup};

use crate::{Button, Label};

#[view]
#[derive(SmartDefault)]
pub struct IntView {
    #[default = 1.0]
    value: f32,
    label: SubView<Label>,
    up:    SubView<Button>,
    down:  SubView<Button>,

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

impl ViewSetup for IntView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();

        self.label.set_text("1.0");

        self.up.on_tap.sub(move |_| {
            self.value += self.step;
            let val = self.value;
            self.on_change.trigger(val);
            self.label.set_text(format!("{val:.1}"));
        });

        self.down.on_tap.sub(move |_| {
            self.value -= self.step;
            let val = self.value;
            self.on_change.trigger(val);
            self.label.set_text(format!("{val:.1}"));
        });
    }
}
