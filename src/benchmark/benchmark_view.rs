use std::string::String;

use test_engine::{
    rtools::{Boxed, Random},
    ui::{layout::Anchor, SubView},
    view, Screen,
};
use ui::{ViewCallbacks, ViewLayout};
use ui_views::{Button, LabeledTextField};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    login:    SubView<LabeledTextField>,
    password: SubView<LabeledTextField>,

    button: SubView<Button>,

    back: SubView<Button>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.login.place().size(200, 80).center_hor();
        self.login.place().anchor(self.password, Anchor::Bot, 20);
        self.login.set_title("Login:");

        self.password.place().size(200, 40).center();
        self.password.set_title("Password:");

        self.button.place().size(100, 40).center_hor();
        self.button.place().anchor(self.login, Anchor::Bot, 20);

        self.button.on_tap.set(self, |this, _| {
            this.button.set_text(String::random());
        });

        self.back.set_text("Back").place().size(120, 20).b(20).center_hor();

        self.back.on_tap.sub(|_| {
            Screen::current().ui.set_level(TestGameLevel::boxed());
            Screen::current().ui.set_view(TestGameView::boxed());
        });
    }
}
