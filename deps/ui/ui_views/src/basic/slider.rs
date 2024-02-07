use gm::{converter::Converter, Color};
use refs::Weak;
use rtools::IntoF32;
use ui::{view, Event, SubView, Touch, ViewCallbacks, ViewFrame, ViewSetup, ViewTouch};
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::CircleView;

#[view]
pub struct Slider {
    circle:    SubView<CircleView>,
    raw_value: f32,

    converter: Converter,

    pub on_change: Event<f32>,

    pub value: f32,
}

impl Slider {
    pub fn set_range(&mut self, min: impl IntoF32, max: impl IntoF32) -> &mut Self {
        self.set_min(min).set_max(max)
    }

    pub fn set_min(&mut self, min: impl IntoF32) -> &mut Self {
        self.converter.set_min(min);
        self
    }

    pub fn set_max(&mut self, max: impl IntoF32) -> &mut Self {
        self.converter.set_max(max);
        self
    }
}

impl ViewSetup for Slider {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch.all.val(move |touch| {
            self.on_touch(&touch);
        });

        self.circle.set_color(Color::BLUE);
    }
}

impl ViewCallbacks for Slider {
    fn update(&mut self) {
        let radius = self.width() / 2.0;
        self.circle.set_radius(radius);
    }
}

impl Slider {
    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = touch.position.y.clamp(half_circle, self.height() - half_circle);

        self.circle.set_y(y_pos - half_circle);
        self.raw_value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);

        let val = self.converter.convert(self.raw_value);
        self.value = val;
        self.on_change.trigger(val);
    }
}
