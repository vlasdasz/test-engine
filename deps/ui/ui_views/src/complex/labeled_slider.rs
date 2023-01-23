use refs::Weak;
use rtools::IntoF32;
use ui::{view, Event, SubView, ViewSetup};

use crate::{Label, Slider};

#[view]
#[derive(Default)]
pub struct LabeledSlider {
    label:  SubView<Label>,
    slider: SubView<Slider>,

    pub on_change: Event<f32>,
}

impl LabeledSlider {
    pub fn set_start(&mut self, start: impl IntoF32) -> &mut Self {
        self.slider.start = start.into_f32();
        self
    }

    pub fn set_finish(&mut self, finish: impl IntoF32) -> &mut Self {
        self.slider.finish = finish.into_f32();
        self
    }

    fn on_change(&mut self, val: f32) {
        self.label.set_text(format!("{val:.2}"));
        self.on_change.trigger(val);
    }
}

impl ViewSetup for LabeledSlider {
    fn setup(mut self: Weak<Self>) {
        self.slider.on_change.val(move |a| self.on_change(a));
    }
}
