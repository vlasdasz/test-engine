use gm::IntoF32;
use refs::Weak;
use ui::{view, Sub, ViewSetup};
use vents::Event;

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::{Label, Slider};

#[view]
pub struct LabeledSlider {
    label:  Sub<Label>,
    slider: Sub<Slider>,

    pub on_change: Event<f32>,
}

impl LabeledSlider {
    pub fn set_start(&mut self, start: impl IntoF32) -> &mut Self {
        self.slider.set_min(start);
        self
    }

    pub fn set_finish(&mut self, finish: impl IntoF32) -> &mut Self {
        self.slider.set_max(finish);
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
