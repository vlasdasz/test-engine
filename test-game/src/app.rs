#![allow(dead_code)]

use std::fs::read_to_string;

use test_engine::{
    App,
    filesystem::Paths,
    net::RestAPI,
    refs::Own,
    ui::{Button, Label, Setup, Size, View},
};

use crate::interface::test_game_view::{BUTTON, TestGameView};

pub struct TestGameApp;

impl App for TestGameApp {
    fn new() -> Box<Self>
    where Self: Sized {
        Box::new(Self)
    }

    fn before_launch(&self) {
        RestAPI::init("https://jsonplaceholder.typicode.com/");

        BUTTON.apply_globally::<Button>();
        BUTTON.apply_globally::<Label>();
    }

    fn make_root_view(&self) -> Own<dyn View> {
        TestGameView::new()
    }

    fn initial_size(&self) -> Size {
        (2400, 2000).into()
    }

    fn config_yaml(&self) -> Option<String> {
        Paths::git_root()
            .ok()
            .and_then(|root| read_to_string(root.join("secrets/decrypted/test-game.yml")).ok())
    }
}
