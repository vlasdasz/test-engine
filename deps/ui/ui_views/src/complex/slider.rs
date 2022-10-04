use gm::Color;
use refs::ToWeak;
use rtools::{math::clamped_by, Event};
use smart_default::SmartDefault;
use ui::{view, SubView, Touch, ViewCallbacks, ViewFrame, ViewTouch};

use crate::CircleView;

#[view]
#[derive(SmartDefault)]
pub struct Slider {
    circle:    SubView<CircleView>,
    raw_value: f32,

    pub on_change: Event<f32>,

    #[default = 0.0]
    pub start:  f32,
    #[default = 1.0]
    pub finish: f32,
}

impl ViewCallbacks for Slider {
    fn setup(&mut self) {
        self.enable_touch();
        let mut this = self.weak();
        self.on_touch.sub(move |touch| {
            this.on_touch(&touch);
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
        let y_pos = clamped_by(half_circle, self.frame().height() - half_circle, touch.position.y);

        self.circle.set_y(y_pos - half_circle);
        self.raw_value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);

        let span = self.finish - self.start;

        self.on_change.trigger(self.start + span * self.raw_value);
    }
}
