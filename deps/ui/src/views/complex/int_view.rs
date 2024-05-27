use gm::ToF32;
use refs::{weak_from_ref, Weak};
use vents::Event;

use crate::{view::ViewData, Button, InputView, Label, UIImages, ViewSetup, ViewTouch};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use ui_proc::view;

use crate::Sub;

#[view]
pub struct IntView {
    value: f32,
    label: Sub<Label>,

    #[link = up_tap]
    up: Sub<Button>,

    #[link = down_tap]
    down: Sub<Button>,

    on_change_event: Event<f32>,
    pub step:        f32,
}

impl ViewSetup for IntView {
    fn setup(mut self: Weak<Self>) {
        self.value = 1.0;
        self.step = 1.0;

        self.place().all_ver();
        self.label.text = "1.0".into();
        self.up.set_image(UIImages::up());
        self.down.set_image(UIImages::down());
    }
}

impl IntView {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, val: impl ToF32) -> &mut Self {
        let val = val.to_f32();
        self.value = val;
        self.label.text = format!("{val:.1}");
        self.on_change_event.trigger(val);
        self
    }

    pub fn set_step(&mut self, step: impl ToF32) -> &mut Self {
        self.step = step.to_f32();
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

impl InputView for IntView {
    fn set_title(&mut self, _title: &str) {
        unimplemented!()
    }

    fn text(&self) -> &str {
        self.label.text()
    }

    fn enable_editing(&mut self) {
        self.up.enable_touch();
        self.down.enable_touch();
    }

    fn disable_editing(&mut self) {
        self.up.disable_touch();
        self.down.disable_touch();
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self as _)
    }
}
