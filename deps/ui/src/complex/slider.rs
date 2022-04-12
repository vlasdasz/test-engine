use gm::Color;
use rtools::{math::clamped_by, Event, IntoF32, Rglica};

use crate::{
    basic::CircleView,
    view_base::{add_boxed, ViewBase},
    Touch, View,
};

#[derive(Default, Debug)]
pub struct Slider {
    base:          ViewBase,
    circle:        Rglica<CircleView>,
    value:         f32,
    multiplier:    f32,
    pub on_change: Event<f32>,
}

impl Slider {
    pub fn set_multiplier(&mut self, multiplier: impl IntoF32) {
        self.multiplier = multiplier.into_f32()
    }
}

impl View for Slider {
    fn setup(&mut self) {
        self.multiplier = 1.0;
        dbg!(self.frame());

        let radius = self.width();
        self.circle = add_boxed(self, CircleView::with_radius(radius));

        self.circle.set_color(Color::BLUE);

        self.enable_touch();
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = clamped_by(
            half_circle,
            self.frame().height() - half_circle,
            touch.position.y,
        );

        self.circle.frame_mut().origin.y = y_pos - half_circle;

        let value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);
        let value = value * self.multiplier;
        self.value = value;
        self.on_change.trigger(value);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
