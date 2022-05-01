use gm::Color;
use rtools::Rglica;

use crate::{
    basic::Button,
    placer::Anchor,
    view::{ViewData, ViewFrame, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks,
};

#[derive(Default, Debug)]
pub struct Alert {
    base:      ViewBase,
    label:     Rglica<Label>,
    ok_button: Rglica<Button>,
    message:   String,
}

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

        // let this = self.to_rglica();
        //  self.ok_button
        //      .on_tap
        //      .subscribe_with(self.to_rglica().clone(), |_, this|
        // this.remove_from_superview());
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

impl View for Alert {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
