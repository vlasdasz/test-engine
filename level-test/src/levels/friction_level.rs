use test_engine::level::{level, LevelSetup};

#[level]
#[derive(Default)]
pub struct FrictionLevel {}

impl LevelSetup for FrictionLevel {
    fn setup(&mut self) {}
}
