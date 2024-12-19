use test_engine::{
    DataManager,
    level::{LevelSetup, level},
    ui::Image,
};

#[level]
#[derive(Default)]
pub struct SkyboxLevel {}

impl LevelSetup for SkyboxLevel {
    fn setup(&mut self) {
        self.background = Image::get("sky.png");
    }
}
