use gm::volume::GyroData;

use crate::Level;

pub trait LevelSetup {
    fn needs_physics(&self) -> bool;

    fn setup(&mut self);

    fn update(&mut self);

    fn on_key_pressed(&mut self, _: char);

    fn on_gyro_changed(&mut self, _: GyroData);
}

impl<T: Level + 'static> LevelSetup for T {
    default fn needs_physics(&self) -> bool {
        false
    }

    default fn setup(&mut self) {}

    default fn update(&mut self) {}

    default fn on_key_pressed(&mut self, _: char) {}

    default fn on_gyro_changed(&mut self, _: GyroData) {}
}

pub trait LevelInternal {
    fn __internal_setup(&self);
    fn __internal_update(&self, frame_time: f32);
}
