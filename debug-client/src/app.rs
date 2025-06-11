use test_engine::{
    App,
    refs::Own,
    ui::{Setup, View},
};

use crate::interface::main_view::MainView;

pub struct DebugApp;

impl App for DebugApp {
    fn new() -> Self
    where Self: Sized {
        Self
    }

    fn make_root_view(&self) -> Own<dyn View> {
        MainView::new()
    }
}
