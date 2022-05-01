use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    basic::Button,
    impl_view,
    placer::Anchor,
    view,
    view::{ViewData, ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct Alert {
    label:     Rglica<Label>,
    ok_button: Rglica<Button>,
    message:   String,
}

impl_view!(Alert);

impl Alert {
    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string()
    }
}

impl ViewCallbacks for Alert {
    fn setup(&mut self) {
        self.set_frame((280, 140)).set_color(Color::WHITE);

        (self.label, self.ok_button) = (self.add_view(), self.add_view());

        self.label.set_text(self.message.clone());

        self.ok_button.set_color(Color::LIGHT_GRAY);
        self.ok_button.set_text("OK");
        self.ok_button.set_frame((100, 50));
    }

    fn layout(&mut self) {
        self.place().center();
        self.label.place().center_hor();
        self.label.set_y(5);
        self.ok_button
            .place()
            .anchor(self.label, Anchor::Bot, Anchor::Center, 20);
    }
}
