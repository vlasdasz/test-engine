use rtools::Rglica;
use test_engine::{
    game_view::GameView,
    ui::{
        view_base::{add_view, ViewBase},
        Label, View,
    },
    Level,
};

use crate::benchmark::benchmark_level::BenchmarkLevel;

#[derive(Debug, Default)]
pub struct BenchmarkView {
    view:          ViewBase,
    level:         BenchmarkLevel,
    bullets_label: Rglica<Label>,
}

impl View for BenchmarkView {
    fn setup(&mut self) {
        self.level.setup();

        self.bullets_label = add_view(self);
    }

    fn layout(&mut self) {
        self.place().as_background();
        self.bullets_label.place().top_right(10);
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
}
