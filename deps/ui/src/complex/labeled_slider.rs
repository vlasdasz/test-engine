use rtools::{Event, IntoF32, Rglica, ToRglica};

use crate::{complex::Slider, view, view::ViewSubviews, Label, SubView, View, ViewBase, ViewCallbacks};

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
        self.label.set_text(format!("{:.2}", val));
        self.on_change.trigger(val);
    }
}

impl ViewCallbacks for LabeledSlider {
    fn setup(&mut self) {
        self.slider.on_change.set(self, |s, a| s.on_change(a));
    }
}
