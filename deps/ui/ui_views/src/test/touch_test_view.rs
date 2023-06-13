use gm::Color;
use refs::Weak;
use ui::{view, SubView, ViewData, ViewSetup};

use crate::{link_button, Button, _ui_link_button};

#[view]
pub struct TouchTestView {
    button: SubView<Button>,
}

impl TouchTestView {
    fn tap(&mut self) {
        self.set_color(Color::random());
    }
}

impl ViewSetup for TouchTestView {
    fn setup(mut self: Weak<Self>) {
        self.place.size(100, 100).center_ver().r(0);
        self.set_color(Color::random());
        self.button.place.back();
        _ui_link_button!(self, button, tap);
    }
}
