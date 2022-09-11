use std::borrow::Borrow;

use test_engine::{
    main_view::{HasLevel, MainView},
    rtools::{Rglica, ToRglica},
    sprites::Player,
    ui::{basic::Button, view, Label, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout, ViewSubviews},
    ui_layer::UILayer,
    Level,
};

use crate::{benchmark::benchmark_level::BenchmarkLevel, TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    level:         BenchmarkLevel,
    bullets_label: Rglica<Label>,

    back: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.level.setup();

        self.back = self.add_view();
        self.back
            .set_text("Back")
            .place()
            .width(120)
            .height(20)
            .bottom()
            .val(20)
            .center_hor();

        self.back.on_tap.set(self, |this, _| {
            this.ui.set_view::<TestGameView>();
        });

        self.bullets_label = self.add_view();
        self.bullets_label.set_frame((120, 20));
    }

    fn update(&mut self) {
        self.bullets_label
            .set_text(format!("Bullets: {}", self.level.bullets_count));
    }
}

impl MainView for BenchmarkView {
    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}

impl HasLevel for BenchmarkView {
    fn player(&self) -> Rglica<Player> {
        self.level.player
    }

    fn level(&self) -> Rglica<dyn Level> {
        (self.level.borrow() as &dyn Level).to_rglica()
    }
}
