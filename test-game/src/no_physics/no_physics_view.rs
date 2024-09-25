use test_engine::{
    level::LevelManager,
    refs::Weak,
    ui::{view, Setup, TransitionButton, ViewData},
};

use crate::{interface::test_game_view::TestGameView, no_physics::NoPhysicsLevel};

#[view]
pub struct NoPhysicsView {
    #[init]
    back: TransitionButton<Self, TestGameView>,
}

impl Setup for NoPhysicsView {
    fn setup(mut self: Weak<Self>) {
        LevelManager::set_level(NoPhysicsLevel::default());

        self.back.set_text("Back");
        self.back.place().size(100, 50).t(100).l(20);
    }
}
