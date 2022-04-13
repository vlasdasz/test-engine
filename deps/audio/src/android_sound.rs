#![cfg(target_os = "android")]

use std::path::Path;

use rtools::data_manager::LoadFromPath;

pub struct Sound;

impl Sound {
    pub fn play(&mut self) {}
}

impl LoadFromPath for Sound {
    fn load(_path: &Path) -> Self {
        Sound
    }
}
