use gm::Color;
use refs::{Weak, weak_from_ref};
use ui_proc::view;
use vents::Event;

use crate::{
    Anchor, Container, HasTitle, InputView, Setup,
    view::{ViewData, ViewTouch},
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct Switch {
    on: bool,

    off_color: Color,

    pub selected: Event<bool>,

    #[init]
    center: Container,
}

impl Switch {
    pub fn on(&self) -> bool {
        self.on
    }

    pub fn set_on(&mut self, on: bool) {
        const MARGIN: f32 = 5.0;
        self.on = on;
        self.center
            .place()
            .clear()
            .relative(Anchor::Width, weak_from_ref(self), 0.4)
            .tb(MARGIN);
        if on {
            self.center.place().r(MARGIN);
            self.set_color(Color::GREEN);
        } else {
            self.center.place().l(MARGIN);
            self.set_color(self.off_color);
        }
    }

    pub fn set_off_color(&mut self, color: Color) -> &mut Self {
        self.off_color = color;
        self
    }
}

impl Setup for Switch {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.center.set_color(Color::BLUE);
        self.set_on(false);
        self.touch().began.sub(move || {
            let on = !self.on;
            self.set_on(on);
            self.selected.trigger(on);
        });
    }
}

impl HasTitle for Switch {
    fn title(&self) -> &str {
        todo!()
    }

    fn set_title(&mut self, _title: &str) {
        todo!()
    }
}

impl InputView for Switch {
    fn set_text(&mut self, text: &str) {
        match text {
            "0" => self.set_on(false),
            "1" => self.set_on(true),
            _ => panic!(),
        }
    }

    fn text(&self) -> String {
        if self.on { "1" } else { "0" }.to_string()
    }

    fn enable_editing(&mut self) {
        self.enable_touch();
    }

    fn disable_editing(&mut self) {
        self.disable_touch();
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self)
    }
}
