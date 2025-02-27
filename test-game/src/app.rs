use test_engine::{
    App,
    refs::Own,
    ui::{Button, Label, Setup, View},
};

use crate::interface::test_game_view::{_BUTTON, TestGameView};

pub struct TestGameApp;

impl App for TestGameApp {
    fn new() -> Self
    where Self: Sized {
        Self
    }

    fn setup(&self) {
        dbg!("setup");
        _BUTTON.apply_to_all::<Button>();
        _BUTTON.apply_to_all::<Label>();
    }

    fn make_root_view(&self) -> Own<dyn View> {
        TestGameView::new()
    }
}
