use test_engine::{
    App,
    refs::Own,
    ui::{Setup, View},
};

use crate::benchmark_view::BenchmarkView;

pub struct BenchmarkApp;

impl App for BenchmarkApp {
    fn new() -> Self
    where Self: Sized {
        Self
    }

    fn setup(&self) {
        dbg!("Setup");
    }

    fn make_root_view(&self) -> Own<dyn View> {
        BenchmarkView::new()
    }
}
