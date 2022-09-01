use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    basic::Button,
    impl_view, view,
    view::{ViewData, ViewSubviews},
    Label, View, ViewBase, ViewCallbacks, ViewLayout,
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
            .set_corner_radius(10)
            .set_border_color(Color::BLACK)
            .place()
            .width(100)
            .height(80)
            .center();

        self.label = self.make_this(|this, v: &mut Label| {
            v.set_text(this.message.clone())
                .place()
                .left()
                .right()
                .offset(10)
                .top()
                .offset(10)
                .height(20);
        });

        self.ok_button = self.make_this(|this, v: &mut Button| {
            v.set_text("OK")
                .set_border_color(Color::GRAY)
                .set_text_color(Color::BLUE)
                .place()
                .width(202)
                .height(20)
                .center_hor()
                .bottom()
                .offset(-1);

            v.on_tap.set(this, |this, _| this.remove_from_superview());
        });
    }
}
