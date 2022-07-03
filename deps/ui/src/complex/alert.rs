use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    basic::Button,
    impl_view, view,
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
        self.message = message.to_string();
        self.label.set_text(message);
    }
}

impl ViewCallbacks for Alert {
    fn setup(&mut self) {
        self.set_color(Color::WHITE)
            .make_layout(|l| l.width(200).height(80).center());

        (self.label, self.ok_button) = (self.add_view(), self.add_view());

        self.label.set_text(self.message.clone());

        self.label.make_layout(|l| {
            l.width(200).height(20);
            l.center_hor();
            l.top().offset(10);
        });

        self.ok_button
            .set_color(Color::LIGHT_GRAY)
            .set_text("OK")
            .make_layout(|l| {
                l.width(50).height(20);
                l.center_hor();
                l.bottom().offset(10);
            });
        self.ok_button
            .on_tap
            .set(self, |this, _| this.remove_from_superview());
    }
}
