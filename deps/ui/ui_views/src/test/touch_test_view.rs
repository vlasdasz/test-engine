use gm::Color;
use refs::Weak;
use ui::{view, SubView, ViewData, ViewSetup};

use crate::{self as ui_views, Button};

#[view]
pub struct TouchTestView {
    #[link = tap]
    button: SubView<Button>,
}

impl TouchTestView {
    fn tap(&mut self) {
        self.set_color(Color::random());
    }
}

impl ViewSetup for TouchTestView {
    fn setup(mut self: Weak<Self>) {
        self.place().size(100, 100).center_y().r(0);
        self.set_color(Color::random());
        self.button.place().back();
    }
}
