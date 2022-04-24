use gm::Color;
use rtools::Rglica;

use crate::{
    basic::Button,
    placer::Anchor,
    view::ViewSetters,
    view_base::{add_view, ViewBase},
    Label, View,
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

impl View for Alert {
    fn setup(&mut self) {
        self.set_frame((280, 140)).set_color(Color::WHITE);

        (self.label, self.ok_button) = (add_view(self), add_view(self));

        self.label.set_text(self.message.clone());

        self.ok_button.set_color(Color::LIGHT_GRAY);
        self.ok_button.set_text("OK");
        self.ok_button.frame_mut().size = (100, 50).into();

        // let this = self.to_rglica();
        //  self.ok_button
        //      .on_tap
        //      .subscribe_with(self.to_rglica().clone(), move |_, this|
        // this.remove_from_superview());
    }

    fn layout(&mut self) {
        self.place().center();
        self.label.place().center_hor();
        self.label.frame_mut().origin.y = 5.0;
        self.ok_button
            .place()
            .anchor(self.label, Anchor::Bot, Anchor::Center, 20);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
