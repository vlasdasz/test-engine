use std::string::String;

use test_engine::{
    rtools::{Boxed, Random, Rglica, ToRglica},
    ui::{
        basic::{Button, TextField},
        view, SubView, View, ViewBase, ViewCallbacks, ViewLayout, ViewSubviews,
    },
    Screen,
};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    field1: SubView<TextField>,
    field2: SubView<TextField>,

    button: SubView<Button>,

    back: SubView<Button>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.field1.place().size(200, 40).center();
        self.field2.place().size(200, 40).center_hor().top().val(200);

        self.button.place().size(100, 40).center_hor().top().val(50);
        self.button.on_tap.set(self, |this, _| {
            this.button.set_text(String::random());
        });

        self.back
            .set_text("Back")
            .place()
            .size(120, 20)
            .bottom()
            .val(20)
            .center_hor();

        self.back.on_tap.sub(|_| {
            Screen::current().ui.set_level(TestGameLevel::boxed());
            Screen::current().ui.set_view(TestGameView::boxed());
        });
    }
}
