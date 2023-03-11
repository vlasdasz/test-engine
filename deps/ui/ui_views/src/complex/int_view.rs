use gl_image::ToImage;
use refs::Weak;
use ui::{view, Event, SubView, ViewData, ViewSetup};

use crate::{Button, Label};

#[view]
pub struct IntView {
    value: f32,
    label: SubView<Label>,
    up:    SubView<Button>,
    down:  SubView<Button>,

    pub on_change: Event<f32>,
    pub step:      f32,
}

impl IntView {
    pub fn set_images(&mut self, up: impl ToImage, down: impl ToImage) {
        self.up.set_image(up);
        self.down.set_image(down);
    }
}

impl ViewSetup for IntView {
    fn setup(mut self: Weak<Self>) {
        self.value = 1.0;
        self.step = 1.0;

        self.place.all_ver();

        self.label.set_text("1.0");

        self.up.on_tap.sub(move || {
            self.value += self.step;
            let val = self.value;
            self.on_change.trigger(val);
            self.label.set_text(format!("{val:.1}"));
        });

        self.down.on_tap.sub(move || {
            self.value -= self.step;
            let val = self.value;
            self.on_change.trigger(val);
            self.label.set_text(format!("{val:.1}"));
        });
    }
}
