use gm::Color;
use tools::{Event, Rglica};

use crate::{basic::Circle, make_view_on, View, ViewBase};

#[derive(Default)]
pub struct Slider {
    base:      ViewBase,
    circle:    Rglica<Circle>,
    value:     f32,
    on_change: Event<f32>,
}

impl Slider {
    fn setup_touch(&mut self) {
        self.circle.enable_touch();

        let mut this = Rglica::from_ref(self);
        self.circle.on_touch().subscribe(move |touch| {
            if touch.is_ended() {
                return;
            }
            let value = dbg!(1.0 - dbg!(touch.position.y) - this.frame().size.height);
            this.value = value;
            this.on_change.trigger(value);
        });
    }

    fn set_slider_position(&mut self) {
        let size = self.circle.frame().size.width;

        let frame = (0, self.frame().size.height * (1.0 - self.value), size, size).into();

        self.circle.set_frame(frame);
    }
}

impl View for Slider {
    fn setup(&mut self) {
        self.circle = make_view_on(self);
        let mut circle = self.circle.clone();
        circle.set_frame(self.frame().square().into());
        circle.set_color(Color::BLUE);

        self.setup_touch();
    }

    fn layout(&mut self) {
        self.set_slider_position()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
