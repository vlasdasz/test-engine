use gm::Color;
use refs::{weak_from_ref, Weak};
use ui::{layout::Anchor, view, Container, Event, SubView, ViewData, ViewSetup, ViewTouch};

#[view]
pub struct Switch {
    center: SubView<Container>,
    on:     bool,

    pub selected: Event<bool>,
}

impl Switch {
    pub fn on(&self) -> bool {
        self.on
    }

    pub fn set_on(&mut self, on: bool) {
        const MARGIN: f32 = 5.0;
        self.on = on;
        self.center
            .place
            .clear()
            .relative(Anchor::Width, 0.4, weak_from_ref(self))
            .tb(MARGIN);
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
        self.enable_touch();
        self.center.set_color(Color::BLUE);
        self.set_on(false);
        self.touch.began.sub(move || {
            let on = !self.on;
            self.set_on(on);
            self.selected.trigger(on);
        });
    }
}
