use gm::Color;
use rtools::{math::clamped_by, Event, Rglica, ToRglica};

use crate::{
    basic::CircleView,
    view,
    view::{ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks, ViewTouch,
};

#[view]
#[derive(Debug)]
pub struct Slider {
    circle:    Rglica<CircleView>,
    raw_value: f32,

    pub on_change: Event<f32>,

    pub start:  f32,
    pub finish: f32,
}

impl ViewCallbacks for Slider {
    fn setup(&mut self) {
        let radius = self.width() / 2.0;
        let circle = CircleView::with_radius(radius);
        self.circle = circle.to_rglica();
        self.add_subview(circle);

        self.circle.set_color(Color::BLUE);

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

impl Default for Slider {
    fn default() -> Self {
        Self {
            view:      Default::default(),
            circle:    Default::default(),
            raw_value: Default::default(),
            on_change: Default::default(),

            start:  0.0,
            finish: 1.0,
        }
    }
}
