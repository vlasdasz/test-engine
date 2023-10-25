use test_engine::{gm::Color, view};
use test_game_lib::benchmark::ui_debug_view::UIDebugView;
use ui::{
    refs::{Own, Weak},
    SubView, ViewController, ViewFrame, ViewSetup,
};
use ui_views::{link_button, Button};

use crate::ui_test::CollectionTestView;

#[view]
pub struct UITestView {
    push_pop:   SubView<Button>,
    collection: SubView<Button>,
    debug:      SubView<Button>,
    nothing:    SubView<Button>,
}

impl ViewSetup for UITestView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();

        self.push_pop.set_text("Push Pop");
        self.push_pop.set_text_color(Color::RED);
        link_button!(self, push_pop, on_push_pop);

        self.collection.set_text_color(Color::GREEN);
        self.collection.set_text("Collection");
        self.collection
            .on_tap
            .sub(move || self.navigation().push(Own::<CollectionTestView>::default()));

        self.debug.set_text_color(Color::BLUE);
        self.debug.set_text("Debug");
        self.debug
            .on_tap
            .sub(move || self.navigation().push(Own::<UIDebugView>::default()));

        self.nothing.set_text_color(Color::WHITE);
        self.nothing.set_text("Nothing");
    }
}

impl UITestView {
    fn on_push_pop(&mut self) {
        dbg!(self.frame());
    }
}
