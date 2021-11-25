use gm::Color;
use tools::{math::clamped_by, Event, Rglica, ToRglica};

use crate::{basic::Circle, init_view_on, View, ViewBase};

#[derive(Default)]
pub struct Slider {
    base:           ViewBase,
    circle:         Rglica<Circle>,
    value:          f32,
    pub multiplier: f32,
    pub on_change:  Event<f32>,
}

impl Slider {
    fn setup_touch(&mut self) {
        self.enable_touch();

        let mut this = self.to_rglica();
        self.on_touch().subscribe(move |touch| {
            if touch.is_ended() {
                return;
            }

            let half_circle = this.circle.frame().height() / 2.0;
            let y_pos = clamped_by(
                half_circle,
                this.frame().height() - half_circle,
                touch.position.y,
            );

            this.circle.frame_mut().origin.y = y_pos - half_circle;

            let value = 1.0 - (y_pos - half_circle) / (this.frame().height() - half_circle * 2.0);
            let value = value * this.multiplier;
            this.value = value;
            this.on_change.trigger(value);
        });
    }
}

impl View for Slider {
    fn setup(&mut self) {
        self.multiplier = 1.0;
        self.circle = init_view_on(self);
        let mut circle = self.circle.clone();
        circle.set_frame(self.frame().square().into());
        circle.set_color(Color::BLUE);

        self.setup_touch();
    }

    fn layout(&mut self) {
        self.circle.frame_mut().size = (self.frame().width(), self.frame().width()).into();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
