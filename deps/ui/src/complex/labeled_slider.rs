use rtools::{Event, Rglica, ToRglica};

use crate::{
    complex::Slider,
    view_base::{add_view, ViewBase},
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
    pub fn set_multiplier(&mut self, multiplier: f32) {
        self.slider.set_multiplier(multiplier)
    }

    fn on_change(&mut self, val: f32) {
        self.label.set_text(val);
        self.on_change.trigger(val);
    }
}

impl View for LabeledSlider {
    fn setup(&mut self) {
        self.label = add_view(self);
        self.slider = add_view(self);

        let mut this = self.to_rglica();
        self.slider.on_change.subscribe(move |a| this.on_change(a));
    }

    fn layout(&mut self) {
        self.place().all_vertically_with_ratio([1, 5])
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
