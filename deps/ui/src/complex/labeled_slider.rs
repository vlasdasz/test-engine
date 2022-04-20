use rtools::{Event, IntoF32, Rglica};

use crate::{
    complex::Slider,
    view_base::{add_view_with_frame, ViewBase},
    Label, View,
};

#[derive(Default, Debug)]
pub struct LabeledSlider {
    base:   ViewBase,
    label:  Rglica<Label>,
    slider: Rglica<Slider>,

    pub on_change: Event<f32>,
}

impl LabeledSlider {
    pub fn set_start(&mut self, start: impl IntoF32) {
        self.slider.start = start.into_f32()
    }

    pub fn set_finish(&mut self, finish: impl IntoF32) {
        self.slider.finish = finish.into_f32()
    }

    fn on_change(&mut self, val: f32) {
        self.label.set_text(format!("{:.2}", val));
        self.on_change.trigger(val);
    }
}

impl View for LabeledSlider {
    fn setup(&mut self) {
        let frames = self.place().frames_for_ratio([1, 5]);

        self.label = add_view_with_frame(self, frames[0]);
        self.slider = add_view_with_frame(self, frames[1]);

        self.slider.on_change.set(self, move |a, s| s.on_change(a));
    }

    fn layout(&mut self) {
        self.place().all_vertically_with_ratio([1, 5]);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
