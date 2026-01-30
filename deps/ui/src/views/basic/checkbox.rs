use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{
    Container, Setup,
    view::{ViewData, ViewTouch},
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct CheckBox {
    on: bool,

    pub selected: Event<bool>,

    #[init]
    dot: Container,
}

impl CheckBox {
    pub fn on(&self) -> bool {
        self.on
    }

    pub fn on_change<Ret>(
        self: Weak<Self>,
        mut callback: impl FnMut(bool) -> Ret + Send + 'static,
    ) -> Weak<Self> {
        self.selected.val(move |val| {
            callback(val);
        });
        self
    }

    pub fn set_on(&mut self, on: bool) {
        self.dot.set_hidden(!on);
        self.on = on;
    }
}

impl Setup for CheckBox {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();

        self.set_on(false);

        self.set_color((39, 53, 73));
        self.set_border_width(5).set_corner_radius(10);
        self.set_border_color((70, 78, 97));

        self.dot.set_color((88, 148, 242)).set_corner_radius(8);
        self.dot.place().center().relative_size(self, 0.45);

        self.touch().up_inside.sub(move || {
            let on = !self.on;
            self.set_on(on);
            self.selected.trigger(on);
        });
    }
}
