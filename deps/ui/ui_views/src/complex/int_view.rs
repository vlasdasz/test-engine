use refs::Weak;
use rtools::IntoF32;
use ui::{view, Event, SubView, ViewData, ViewSetup};

use crate as ui_views;
use crate::{Button, Label, UIImages};

#[view]
pub struct IntView {
    value: f32,
    label: SubView<Label>,

    #[link = up_tap]
    up: SubView<Button>,

    #[link = down_tap]
    down: SubView<Button>,

    on_change_event: Event<f32>,
    pub step:        f32,
}

impl ViewSetup for IntView {
    fn setup(mut self: Weak<Self>) {
        self.value = 1.0;
        self.step = 1.0;

        self.place().all_ver();
        self.label.set_text("1.0");
        self.up.set_image(UIImages::up());
        self.down.set_image(UIImages::down());
    }
}

impl IntView {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, val: impl IntoF32) -> &mut Self {
        let val = val.into_f32();
        self.value = val;
        self.label.set_text(format!("{val:.1}"));
        self.on_change_event.trigger(val);
        self
    }

    pub fn set_step(&mut self, step: impl IntoF32) -> &mut Self {
        self.step = step.into_f32();
        self
    }

    fn up_tap(mut self: Weak<Self>) {
        let val = self.value + self.step;
        self.set_value(val);
    }

    fn down_tap(mut self: Weak<Self>) {
        let val = self.value - self.step;
        self.set_value(val);
    }

    pub fn on_change(&self, action: impl FnMut(f32) + 'static) -> &Self {
        self.on_change_event.val(action);
        self
    }
}
