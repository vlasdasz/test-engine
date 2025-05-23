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
        _BUTTON.apply_globally::<Button>();
        _BUTTON.apply_globally::<Label>();
    }

    fn make_root_view(&self) -> Own<dyn View> {
        TestGameView::new()
    }
}
