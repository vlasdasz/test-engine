use test_engine::{
    rtools::{Boxed, Rglica, ToRglica},
    ui::{
        basic::Button, view, Label, SubView, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout,
        ViewSubviews,
    },
    Screen,
};

use crate::test_game::{TestGameLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    bullets_label: SubView<Label>,

    back: SubView<Button>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.back
            .set_text("Back")
            .place()
            .width(120)
            .height(20)
            .bottom()
            .val(20)
            .center_hor();

        self.back.on_tap.sub(|_| {
            Screen::current().ui.set_level(TestGameLevel::boxed());
            Screen::current().ui.set_view(TestGameView::boxed());
        });

        self.bullets_label.set_frame((120, 20));
    }
}
