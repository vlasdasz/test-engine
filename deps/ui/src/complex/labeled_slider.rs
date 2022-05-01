use rtools::{Event, IntoF32, Rglica, ToRglica};

use crate::{
    complex::Slider,
    impl_view, view,
    view::{ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct LabeledSlider {
    label:  Rglica<Label>,
    slider: Rglica<Slider>,

    pub on_change: Event<f32>,
}

impl_view!(LabeledSlider);

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
        let frames = self.place().frames_for_ratio([1, 5]);

        self.label = self.add_view_with_frame(frames[0]);
        self.slider = self.add_view_with_frame(frames[1]);

        self.slider.on_change.set(self, |s, a| s.on_change(a));
    }

    fn layout(&mut self) {
        self.place().all_vertically_with_ratio([1, 5]);
    }
}
