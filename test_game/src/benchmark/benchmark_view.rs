use test_engine::{
    game_view::GameView,
    rtools::{Boxed, Rglica},
    ui::{basic::Button, placer::Anchor, view_base::ViewBase, Label, View, ViewTemplates},
    ui_layer::UILayer,
    Level,
};

use crate::{benchmark::benchmark_level::BenchmarkLevel, TestGameView};

#[derive(Debug, Default)]
pub struct BenchmarkView {
    view:          ViewBase,
    level:         BenchmarkLevel,
    bullets_label: Rglica<Label>,

    to_test: Rglica<Button>,

    ui: Rglica<UILayer>,
}

impl View for BenchmarkView {
    fn setup(&mut self) {
        self.level.setup();

        self.to_test = self.add_view();
        self.to_test.set_text("Test");
        self.to_test.frame_mut().size = (120, 20).into();
        self.to_test.on_tap.set(self, move |_, this| {
            this.ui.set_view(TestGameView::boxed());
        });

        self.bullets_label = self.add_view();
    }

    fn layout(&mut self) {
        self.place().as_background();
        self.to_test.place().bottom_center(20);
        self.bullets_label
            .place()
            .anchor(self.to_test, Anchor::Top, Anchor::Center, 20);
    }

    fn update(&mut self) {
        self.bullets_label
            .set_text(format!("Bullets: {}", self.level.bullets_count));
    }

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}

impl GameView for BenchmarkView {
    fn level(&self) -> &dyn Level {
        &self.level
    }

    fn level_mut(&mut self) -> &mut dyn Level {
        &mut self.level
    }

    fn set_ui(&mut self, ui: Rglica<UILayer>) {
        self.ui = ui
    }
}
