use refs::Weak;
use ui::{view, Event, SubView, ViewSetup};

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

    pub on_change: Event<f32>,
    pub step:      f32,
}

impl ViewSetup for IntView {
    fn setup(mut self: Weak<Self>) {
        self.value = 1.0;
        self.step = 1.0;

        self.place.all_ver();
        self.label.set_text("1.0");
        self.up.set_image(UIImages::up());
        self.down.set_image(UIImages::down());
    }
}

impl IntView {
    fn up_tap(mut self: Weak<Self>) {
        self.value += self.step;
        let val = self.value;
        self.on_change.trigger(val);
        self.label.set_text(format!("{val:.1}"));
    }

    fn down_tap(mut self: Weak<Self>) {
        self.value -= self.step;
        let val = self.value;
        self.on_change.trigger(val);
        self.label.set_text(format!("{val:.1}"));
    }
}
