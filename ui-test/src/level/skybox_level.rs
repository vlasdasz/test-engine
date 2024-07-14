use test_engine::{
    level::{level, LevelSetup},
    ui::Image,
    DataManager,
};

#[level]
#[derive(Default)]
pub struct SkyboxLevel {}

impl LevelSetup for SkyboxLevel {
    fn setup(&mut self) {
        self.background = Image::get("sky.png");
    }
}
