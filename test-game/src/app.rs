#![allow(dead_code)]

use test_engine::{
    App,
    refs::Own,
    ui::{Button, Label, Setup, Size, View},
};

use crate::interface::test_game_view::{_BUTTON, TestGameView};

pub struct TestGameApp;

impl App for TestGameApp {
    fn new() -> Box<Self>
    where Self: Sized {
        Box::new(Self)
    }

    fn before_launch(&self) {
        _BUTTON.apply_globally::<Button>();
        _BUTTON.apply_globally::<Label>();
    }

    fn make_root_view(&self) -> Own<dyn View> {
        TestGameView::new()
    }

    fn initial_size(&self) -> Size {
        (2400, 2000).into()
    }
}
