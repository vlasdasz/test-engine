use gm::ToF32;
use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{Sub, ViewSetup};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Label, Slider};

#[view]
pub struct LabeledSlider {
    label:  Sub<Label>,
    slider: Sub<Slider>,

    pub on_change: Event<f32>,
}

impl LabeledSlider {
    pub fn set_start(&mut self, start: impl ToF32) -> &mut Self {
        self.slider.set_min(start);
        self
    }

    pub fn set_finish(&mut self, finish: impl ToF32) -> &mut Self {
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
