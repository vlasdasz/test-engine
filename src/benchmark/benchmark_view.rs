use std::string::String;

use test_engine::{
    rtools::{Boxed, Random, Rglica, ToRglica},
    ui::{
        basic::{Button, TextField},
        layout::Anchor,
        view, SubView, View, ViewBase, ViewCallbacks, ViewLayout, ViewSubviews,
    },
    Screen,
};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    f1: SubView<TextField>,
    f2: SubView<TextField>,

    button: SubView<Button>,

    back: SubView<Button>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.f1.place().size(200, 40).center();
        self.f2.place().size(200, 40).center_hor().anchor(self.f1, Anchor::Bot, 20);
        self.button.place().size(100, 40).center_hor().anchor(self.f2, Anchor::Bot, 20);

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
