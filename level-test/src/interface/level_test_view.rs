use test_engine::{
    level::LevelManager,
    ui::{view, ViewSetup},
};

use crate::levels::FrictionLevel;

#[view]
pub struct LevelTestView {}

impl ViewSetup for LevelTestView {
    fn setup(self: test_engine::refs::Weak<Self>) {
        LevelManager::set_level(FrictionLevel::default());
    }
}
