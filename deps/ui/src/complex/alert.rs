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
pub struct Alert {
    label:     SubView<Label>,
    ok_button: SubView<Button>,
    message:   String,
}

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
            .size(200, 80)
            .center();

        self.label = self.make_this(|this, v: &mut Label| {
            v.set_text(this.message.clone())
                .place()
                .left()
                .right()
                .val(10)
                .top()
                .val(10)
                .height(20);
        });

        self.ok_button = self.make_this(|this, v: &mut Button| {
            v.set_text("OK")
                .set_border_color(Color::GRAY)
                .set_text_color(Color::BLUE)
                .place()
                .size(202, 20)
                .center_hor()
                .bottom()
                .val(-1);

            v.on_tap.set(this, |this, _| this.remove_from_superview());
        });
    }
}
