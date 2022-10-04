use gl_image::Image;
use refs::ToWeak;
use rtools::{data_manager::Handle};
use smart_default::SmartDefault;
use ui::{view, SubView, ViewCallbacks, ViewData, Event};

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

impl ViewCallbacks for IntView {
    fn setup(&mut self) {
        self.place.all_ver();

        self.label.set_text("1.0");

        let mut this = self.weak();
        self.up.on_tap.sub(move |_| {
            this.value += this.step;
            let val = this.value;
            this.on_change.trigger(val);
            this.label.set_text(format!("{:.1}", val));
        });

        self.down.on_tap.sub(move |_| {
            this.value -= this.step;
            let val = this.value;
            this.on_change.trigger(val);
            this.label.set_text(format!("{:.1}", val));
        });
    }
}
