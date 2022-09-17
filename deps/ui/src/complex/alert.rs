use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    basic::Button,
    view,
    view::{ViewData, ViewSubviews},
    Label, SubView, View, ViewBase, ViewCallbacks, ViewLayout,
};

#[view]
#[derive(Default)]
pub struct AlertView {
    label:     SubView<Label>,
    ok_button: SubView<Button>,
    message:   String,
}

impl AlertView {
    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
        self.label.set_text(message);
    }
}

impl ViewCallbacks for AlertView {
    fn setup(&mut self) {
        self.place().size(200, 80).center();
        self.set_color(Color::WHITE)
            .set_corner_radius(10)
            .set_border_color(Color::BLACK);

        self.label.place().lrt(10).h(20);
        self.label.set_text(self.message.clone());

        self.ok_button.place().size(202, 20).center_hor().b(-1);
        self.ok_button
            .set_text("OK")
            .set_border_color(Color::GRAY)
            .set_text_color(Color::BLUE);
        self.ok_button.on_tap.set(self, |this, _| this.remove_from_superview());
    }
}
