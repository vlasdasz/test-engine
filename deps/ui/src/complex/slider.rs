use gm::Color;
use rtools::{math::clamped_by, Event, Rglica, ToRglica};

use crate::{basic::CircleView, view::ViewTemplates, view_base::ViewBase, Touch, View};

#[derive(Debug)]
pub struct Slider {
    base:      ViewBase,
    circle:    Rglica<CircleView>,
    raw_value: f32,

    pub on_change: Event<f32>,

    pub start:  f32,
    pub finish: f32,
}

impl View for Slider {
    fn setup(&mut self) {
        let radius = self.width();
        let circle = CircleView::with_radius(radius);
        self.circle = circle.to_rglica();
        self.add_boxed(circle);

        self.circle.set_color(Color::BLUE);

        self.enable_touch();
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = clamped_by(half_circle, self.frame().height() - half_circle, touch.position.y);

        self.circle.frame_mut().origin.y = y_pos - half_circle;
        self.raw_value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);

        let span = self.finish - self.start;

        self.on_change.trigger(self.start + span * self.raw_value);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            base:      Default::default(),
            circle:    Default::default(),
            raw_value: Default::default(),
            on_change: Default::default(),

            start:  0.0,
            finish: 1.0,
        }
    }
}
