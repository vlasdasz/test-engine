use test_engine::{
    rtools::{Rglica, ToRglica},
    ui::{
        basic::Button, view, Label, SubView, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout,
        ViewSubviews,
    },
    ui_layer::UILayer,
    Level,
};

use crate::{benchmark::benchmark_level::BenchmarkLevel, test_game::TestGameView};

#[view]
#[derive(Default)]
pub struct BenchmarkView {
    level:         BenchmarkLevel,
    bullets_label: SubView<Label>,

    back: SubView<Button>,

    ui: Rglica<UILayer>,
}

impl ViewCallbacks for BenchmarkView {
    fn setup(&mut self) {
        self.level.setup();

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

        self.bullets_label.set_frame((120, 20));
    }

    fn update(&mut self) {
        self.bullets_label
            .set_text(format!("Bullets: {}", self.level.bullets_count));
    }
}
