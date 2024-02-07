use gm::Color;
use ui::{view, SubView, ViewData, ViewSetup};
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

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
    fn setup(mut self: refs::Weak<Self>) {
        self.place().size(100, 100).center_y().r(0);
        self.set_color(Color::random());
        self.button.place().back();
    }
}
