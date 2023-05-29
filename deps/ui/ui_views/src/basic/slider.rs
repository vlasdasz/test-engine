use gm::Color;
use refs::Weak;
use ui::{view, Event, SubView, Touch, ViewFrame, ViewSetup, ViewTouch};

use crate::CircleView;

#[view]
pub struct Slider {
    circle:    SubView<CircleView>,
    raw_value: f32,

    pub on_change: Event<f32>,

    pub start:  f32,
    pub finish: f32,
}

impl ViewSetup for Slider {
    fn setup(mut self: Weak<Self>) {
        self.start = 0.0;
        self.finish = 1.0;

        self.enable_touch();

        self.on_touch.val(move |touch| {
            self.on_touch(&touch);
        });

        let radius = self.width() / 2.0;

        self.circle.set_radius(radius).set_color(Color::BLUE);
    }
}

impl Slider {
    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = half_circle.clamp(self.frame().height() - half_circle, touch.position.y);

        self.circle.set_y(y_pos - half_circle);
        self.raw_value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);

        let span = self.finish - self.start;

        self.on_change.trigger(self.start + span * self.raw_value);
    }
}
