use gm::Color;
use rtools::{math::clamped_by, Event, Rglica, ToRglica};
use smart_default::SmartDefault;

use crate::{
    basic::CircleView,
    view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks, ViewTouch,
};

#[view]
#[derive(SmartDefault)]
pub struct Slider {
    circle:    Rglica<CircleView>,
    raw_value: f32,

    pub on_change: Event<f32>,

    #[default = 0.0]
    pub start:  f32,
    #[default = 1.0]
    pub finish: f32,
}

impl ViewCallbacks for Slider {
    fn setup(&mut self) {
        let radius = self.width() / 2.0;
        self.circle = self.add_view();
        self.circle.set_radius(radius).set_color(Color::BLUE);

        self.on_touch().set(self, |this, touch| {
            if touch.is_ended() {
                return;
            }

            let half_circle = this.circle.frame().height() / 2.0;
            let y_pos = clamped_by(half_circle, this.frame().height() - half_circle, touch.position.y);

            this.circle.set_y(y_pos - half_circle);
            this.raw_value = 1.0 - (y_pos - half_circle) / (this.height() - half_circle * 2.0);

            let span = this.finish - this.start;

            this.on_change.trigger(this.start + span * this.raw_value);
        });
    }
}
