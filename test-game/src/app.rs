use test_engine::{
    App,
    refs::Own,
    ui::{Setup, View},
};

use crate::interface::test_game_view::TestGameView;

pub struct TestGameApp;

impl App for TestGameApp {
    fn setup(&self) {
        dbg!("setup");
    }

    fn make_root_view(&self) -> Own<dyn View> {
        TestGameView::new()
    }
}
