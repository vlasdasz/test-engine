use gm::Color;
use refs::Weak;
use ui::{view, Container, Event, SubView, ViewData, ViewFrame, ViewSetup};

#[view]
pub struct Switch {
    center: SubView<Container>,
    on:     bool,

    pub selected: Event<bool>,
}

impl Switch {
    pub fn set_on(&mut self, on: bool) {
        const MARGIN: f32 = 5.0;
        self.center.place.clear().w(self.width() / 2.0 - MARGIN * 2.0).tb(MARGIN);
        if on {
            self.center.place.r(MARGIN);
            self.set_color(Color::GREEN);
        } else {
            self.center.place.l(MARGIN);
            self.set_color(Color::CLEAR);
        }
    }
}

impl ViewSetup for Switch {
    fn setup(mut self: Weak<Self>) {
        self.center.set_color(Color::BLUE);
        self.set_on(false);
        self.on_touch_began.sub(move || self.selected.trigger(self.on));
    }
}
